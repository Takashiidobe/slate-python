//! An implementation of Text I/O as defined by PEP 3116 - "New I/O"
//!
//! Classes defined here: TextIOBase, IncrementalNewlineDecoder, TextIOWrapper.
//!
//! Written by Amaury Forgeot d'Arc and Antoine Pitrou
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
//! Fast inlined version of PyType_IS_GC()
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
macro_rules! CHECK_ATTACHED_INT {
    ($self:expr) => {{
        if (unsafe { (*$self).ok }) <= 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"I/O operation on uninitialized object".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        } else {
            if (unsafe { (*$self).detached }) != 0 {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"underlying buffer has been detached".as_ptr(),
                    )
                };
                return 1i32.wrapping_neg();
            }
        }
    }};
}

macro_rules! CHECK_ATTACHED {
    ($self:expr) => {{
        CHECK_INITIALIZED__3!($self);
        if (unsafe { (*$self).detached }) != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"underlying buffer has been detached".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }};
}

/// This macro takes some shortcuts to make the common case faster.
macro_rules! CHECK_CLOSED__4 {
    ($self:expr) => {{
        loop {
            let mut r: i32 = 0;
            if Py_IS_TYPE__2!($self, unsafe { (*(*$self).state).PyTextIOWrapper_Type }) != 0 {
                if !unsafe { (*$self).raw }.is_null() {
                    r = unsafe { _PyFileIO_closed((*$self).raw) };
                } else {
                    let _res: *mut _object = _io_TextIOWrapper_closed_get_impl($self);
                    if _res.is_null() {
                        return core::ptr::null_mut::<_object>();
                    }
                    r = unsafe { PyObject_IsTrue(_res) };
                    Py_DECREF!(_res);
                    if r < 0 {
                        return core::ptr::null_mut::<_object>();
                    }
                }
                if r > 0 {
                    unsafe {
                        PyErr_SetString(PyExc_ValueError, c"I/O operation on closed file.".as_ptr())
                    };
                    return core::ptr::null_mut::<_object>();
                }
            } else {
                if unsafe {
                    _PyIOBase_check_closed(
                        ($self) as *mut _object,
                        _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
                    )
                }
                .is_null()
                {
                    return core::ptr::null_mut::<_object>();
                }
            }
            if true {
                break;
            }
        }
    }};
}

macro_rules! CHECK_INITIALIZED_DECODER {
    ($self:expr) => {{
        if unsafe { (*$self).errors }.is_null() {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"IncrementalNewlineDecoder.__init__() not called".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }};
}

macro_rules! CHECK_INITIALIZED__3 {
    ($self:expr) => {{
        if (unsafe { (*$self).ok }) <= 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"I/O operation on uninitialized object".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }};
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

macro_rules! PyDict_GET_SIZE {
    ($op:expr) => {
        PyDict_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyList_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyList_Type) })
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

macro_rules! PyList_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
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

macro_rules! PyUnicode_2BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u16, PyUnicode_DATA!($op))
    };
}

macro_rules! PyUnicode_4BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u32, PyUnicode_DATA!($op))
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

macro_rules! PyUnicode_MAX_CHAR_VALUE {
    ($op:expr) => {
        PyUnicode_MAX_CHAR_VALUE(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_READ_CHAR {
    ($unicode:expr, $index:expr) => {
        PyUnicode_READ_CHAR(_PyObject_CAST!($unicode), $index)
    };
}

macro_rules! PyUnicode_READ__3 {
    ($kind:expr, $data:expr, $index:expr) => {
        PyUnicode_READ(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_STATIC_CAST!(*const (), $data),
            ($index) as i64,
        )
    };
}

macro_rules! PyUnicode_READ__4 {
    ($kind:expr, $data:expr, $index:expr) => {
        PyUnicode_READ(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_STATIC_CAST!(*const (), $data),
            $index,
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

macro_rules! PyUnicode_WRITE__3 {
    ($kind:expr, $data:expr, $index:expr, $value:expr) => {
        PyUnicode_WRITE(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_CAST!(*mut (), $data),
            ($index) as i64,
            _Py_STATIC_CAST!(u32, $value),
        );
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

/// Maximum value between x and y
macro_rules! Py_MAX {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $x } else { $y }
    };
}

macro_rules! Py_NewRef {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST!($obj))
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

macro_rules! Py_XSETREF__2 {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _object = core::ptr::addr_of_mut!($dst);
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

macro_rules! _PyObject_GC_UNTRACK__2 {
    ($op:expr) => {
        _PyObject_GC_UNTRACK(_PyObject_CAST__2!($op));
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

macro_rules! nldecoder_object_CAST {
    ($op:expr) => {
        ($op) as *mut nldecoder_object
    };
}

macro_rules! textio_CAST {
    ($op:expr) => {
        ($op) as *mut textio
    };
}

unsafe extern "C" {
    static mut _Py_NoneStruct: _object;
    static mut PyExc_AttributeError: *mut _object;
    static mut PyExc_OSError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_BlockingIOError: *mut _object;
    static mut PyExc_EncodingWarning: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyList_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    static mut _PyIO_Module: PyModuleDef;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_GetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GetOptionalAttr(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn PyObject_HasAttrWithError(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn Py_ReprEnter(arg0: *mut _object) -> i32;
    fn Py_ReprLeave(arg0: *mut _object);
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn PyType_GetModuleByDef(arg0: *mut _typeobject, arg1: *mut PyModuleDef) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyBytes_AsString(arg0: *mut _object) -> *mut i8;
    fn PyBytes_Concat(arg0: *mut *mut _object, arg1: *mut _object);
    fn PyBytes_AsStringAndSize(arg0: *mut _object, arg1: *mut *mut i8, arg2: *mut i64) -> i32;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_Substring(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyUnicode_GetLength(arg0: *mut _object) -> i64;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_Concat(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyUnicode_AppendAndDel(arg0: *mut *mut _object, arg1: *mut _object);
    fn PyUnicode_Join(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyUnicode_FindChar(arg0: *mut _object, arg1: u32, arg2: i64, arg3: i64, arg4: i32) -> i64;
    fn PyUnicode_New(arg0: i64, arg1: u32) -> *mut _object;
    fn PyUnicode_FromKindAndData(arg0: i32, arg1: *const (), arg2: i64) -> *mut _object;
    fn PyUnicode_AsUTF8(arg0: *mut _object) -> *const i8;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_ChainExceptions1(arg0: *mut _object);
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn _PyLong_FromByteArray(arg0: *const u8, arg1: u64, arg2: i32, arg3: i32) -> *mut _object;
    fn _PyLong_AsByteArray(
        arg0: *mut _longobject,
        arg1: *mut u8,
        arg2: u64,
        arg3: i32,
        arg4: i32,
        arg5: i32,
    ) -> i32;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyErr_WarnEx(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn PyCodec_LookupError(arg0: *const i8) -> *mut _object;
    fn PyArg_ParseTuple(arg0: *mut _object, arg1: *const i8, ...) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyObject_CallFunctionObjArgs(arg0: *mut _object, ...) -> *mut _object;
    fn PyObject_CallMethodObjArgs(arg0: *mut _object, arg1: *mut _object, ...) -> *mut _object;
    fn PyObject_VectorcallMethod(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PyNumber_Long(arg0: *mut _object) -> *mut _object;
    fn _PyObject_CallMethod(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        ...
    ) -> *mut _object;
    fn _PyCodec_LookupTextEncoding(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn _PyCodecInfo_GetIncrementalDecoder(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn _PyCodecInfo_GetIncrementalEncoder(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn _Py_GetLocaleEncodingObject() -> *mut _object;
    fn _PyFile_Flush(arg0: *mut _object) -> i32;
    fn _PyInterpreterState_GetConfig(arg0: *mut _is) -> *const PyConfig;
    fn _PyUnicode_AsUTF8String(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn _PyUnicode_EncodeUTF32(arg0: *mut _object, arg1: *const i8, arg2: i32) -> *mut _object;
    fn _PyUnicode_EncodeUTF16(arg0: *mut _object, arg1: *const i8, arg2: i32) -> *mut _object;
    fn _PyUnicode_AsLatin1String(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn _PyUnicode_AsASCIIString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn _PyUnicode_EqualToASCIIString(arg0: *mut _object, arg1: *const i8) -> i32;
    fn _PyUnicode_AsUTF8NoNUL(arg0: *mut _object) -> *const i8;
    fn _PyIOBase_check_closed(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyIOBase_finalize(arg0: *mut _object) -> i32;
    fn _PyFileIO_closed(arg0: *mut _object) -> i32;
    fn _PyIO_trap_eintr() -> i32;
    fn _PyIOBase_cannot_pickle(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _Py_convert_optional_to_ssize_t(arg0: *mut _object, arg1: *mut ()) -> i32;
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
// Set if the type object is immutable: type attributes cannot be set nor deleted
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
// Rich comparison opcodes
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
struct _dictkeysobject {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits150U0,
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
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
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
struct _dictvalues {}

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

fn PyUnicode_WRITE(kind: i32, data: *mut (), index: i64, value: u32) {
    if kind == 1 {
        unsafe {
            *_Py_STATIC_CAST!(*mut u8, data).wrapping_offset(index as isize) =
                _Py_STATIC_CAST!(u8, value);
        }
    } else {
        if kind == 2 {
            unsafe {
                *_Py_STATIC_CAST!(*mut u16, data).wrapping_offset(index as isize) =
                    _Py_STATIC_CAST!(u16, value);
            }
        } else {
            unsafe {
                *_Py_STATIC_CAST!(*mut u32, data).wrapping_offset(index as isize) = value;
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

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

fn PyUnicode_READ_CHAR(unicode: *mut _object, index: i64) -> u32 {
    let mut kind: i32 = 0;
    // Tolerate reading the NUL character at str[len(str)]
    kind = PyUnicode_KIND!(unicode);
    if kind == 1 {
        return (unsafe { *PyUnicode_1BYTE_DATA!(unicode).wrapping_offset(index as isize) }) as u32;
    }
    if kind == 2 {
        return (unsafe { *PyUnicode_2BYTE_DATA!(unicode).wrapping_offset(index as isize) }) as u32;
    }
    unsafe { *PyUnicode_4BYTE_DATA!(unicode).wrapping_offset(index as isize) }
}

fn PyUnicode_MAX_CHAR_VALUE(op: *mut _object) -> u32 {
    let mut kind: i32 = 0;
    if PyUnicode_IS_ASCII!(op) != 0 {
        return 127;
    }
    kind = PyUnicode_KIND!(op);
    if kind == 1 {
        return 255;
    }
    if kind == 2 {
        return 65535;
    }
    1114111
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
    __slate_bits_0: __slate_bits::__SlateBits207U0,
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_hashtable_h_19 {
    head: *mut _Py_slist_item_s,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct _Py_GlobalMonitors {
    tools: [u8; 16],
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
struct __SlateRecord_textio_c_h_324 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_textio_c_h_396 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_textio_c_h_564 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_textio_c_h_707 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 5],
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
    pub struct __SlateBits150U0 {
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
    pub struct __SlateBits207U0 {
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
    pub struct __SlateBits1315U0 {
        #[bits(1)]
        pub pendingcr: u32,
        #[bits(1)]
        pub translate: u32,
        #[bits(3)]
        pub seennl: u32,
        #[bits(3, access = na)]
        pub __slate_pad_3: u8,
    }
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

fn _PyType_GetModuleState(r#type: *mut _typeobject) -> *mut () {
    let r#mod: *mut __SlateRecord_pycore_moduleobject_h_30 =
        (unsafe { (*(r#type as *mut _heaptypeobject)).ht_module })
            as *mut __SlateRecord_pycore_moduleobject_h_30;
    unsafe { (*r#mod).md_state }
}

#[unsafe(no_mangle)]
static mut nldecoder_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io.IncrementalNewlineDecoder\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<nldecoder_object>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(nldecoder_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

#[unsafe(no_mangle)]
static mut textiobase_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io._TextIOBase\0".as_ptr() as *mut i8) as *const i8;
    __t0.flags = 1280u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(textiobase_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

#[unsafe(no_mangle)]
static mut textiowrapper_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io.TextIOWrapper\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<textio>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(textiowrapper_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct _io_state {
    initialized: i32,
    unsupported_operation: *mut _object,
    PyIOBase_Type: *mut _typeobject,
    PyIncrementalNewlineDecoder_Type: *mut _typeobject,
    PyRawIOBase_Type: *mut _typeobject,
    PyBufferedIOBase_Type: *mut _typeobject,
    PyBufferedRWPair_Type: *mut _typeobject,
    PyBufferedRandom_Type: *mut _typeobject,
    PyBufferedReader_Type: *mut _typeobject,
    PyBufferedWriter_Type: *mut _typeobject,
    PyBytesIOBuffer_Type: *mut _typeobject,
    PyBytesIO_Type: *mut _typeobject,
    PyFileIO_Type: *mut _typeobject,
    PyStringIO_Type: *mut _typeobject,
    PyTextIOBase_Type: *mut _typeobject,
    PyTextIOWrapper_Type: *mut _typeobject,
}

fn get_io_state(module: *mut _object) -> *mut _io_state {
    let state: *mut () = _PyModule_GetState(module);
    state as *mut _io_state
}

fn get_io_state_by_cls(cls: *mut _typeobject) -> *mut _io_state {
    let state: *mut () = _PyType_GetModuleState(cls);
    state as *mut _io_state
}

fn find_io_state_by_def(r#type: *mut _typeobject) -> *mut _io_state {
    let r#mod: *mut _object =
        unsafe { PyType_GetModuleByDef(r#type, core::ptr::addr_of_mut!(_PyIO_Module)) };
    get_io_state(r#mod)
}

static _io__TextIOBase_detach__doc__: __SlateAlign16<[i8; 171]> = __SlateAlign16([
    100i8, 101i8, 116i8, 97i8, 99i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 101i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8,
    103i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 84i8, 101i8, 120i8, 116i8, 73i8, 79i8, 66i8, 97i8, 115i8, 101i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 116i8,
    46i8, 10i8, 10i8, 65i8, 102i8, 116i8, 101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 117i8,
    110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8, 98i8, 117i8, 102i8, 102i8,
    101i8, 114i8, 32i8, 104i8, 97i8, 115i8, 32i8, 98i8, 101i8, 101i8, 110i8, 32i8, 100i8, 101i8,
    116i8, 97i8, 99i8, 104i8, 101i8, 100i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 84i8, 101i8,
    120i8, 116i8, 73i8, 79i8, 32i8, 105i8, 115i8, 32i8, 105i8, 110i8, 10i8, 97i8, 110i8, 32i8,
    117i8, 110i8, 117i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_detach")]
extern "C-unwind" fn _io__TextIOBase_detach(
    self_: *mut _object,
    cls: *mut _typeobject,
    args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let __v0: bool = nargs != 0 || !kwnames.is_null() && PyTuple_GET_SIZE!(kwnames) != 0;
    if __v0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"detach() takes no arguments".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    _io__TextIOBase_detach_impl(self_, cls)
}

static _io__TextIOBase_read__doc__: __SlateAlign16<[i8; 193]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 105i8,
    122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 97i8, 100i8, 32i8, 97i8, 116i8, 32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 115i8, 105i8,
    122i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8,
    102i8, 114i8, 111i8, 109i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 46i8, 10i8, 10i8,
    82i8, 101i8, 97i8, 100i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 117i8, 110i8, 100i8, 101i8,
    114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8,
    117i8, 110i8, 116i8, 105i8, 108i8, 32i8, 119i8, 101i8, 32i8, 104i8, 97i8, 118i8, 101i8, 32i8,
    115i8, 105i8, 122i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8,
    115i8, 32i8, 111i8, 114i8, 32i8, 119i8, 101i8, 32i8, 104i8, 105i8, 116i8, 10i8, 69i8, 79i8,
    70i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 105i8, 115i8,
    32i8, 110i8, 101i8, 103i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 111i8, 114i8, 32i8, 111i8,
    109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 44i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 117i8,
    110i8, 116i8, 105i8, 108i8, 32i8, 69i8, 79i8, 70i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_read")]
extern "C-unwind" fn _io__TextIOBase_read(
    self_: *mut _object,
    cls: *mut _typeobject,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut size: i32 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            size = 1i32.wrapping_neg();
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__65),
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
                if nargs >= 1 {
                    size = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                    if size == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = _io__TextIOBase_read_impl(self_, cls, size);
            }
        }
        return return_value;
    }
}

static _keywords__64: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__65: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__64.0) as *const *const i8 };
    __t0.fname = (b"read\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io__TextIOBase_readline__doc__: __SlateAlign16<[i8; 170]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 108i8, 105i8, 110i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 115i8, 105i8, 122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 82i8, 101i8, 97i8, 100i8, 32i8, 117i8, 110i8, 116i8, 105i8, 108i8, 32i8,
    110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 32i8, 111i8, 114i8, 32i8, 69i8, 79i8, 70i8,
    46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 101i8,
    109i8, 112i8, 116i8, 121i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 105i8, 102i8,
    32i8, 69i8, 79i8, 70i8, 32i8, 105i8, 115i8, 32i8, 104i8, 105i8, 116i8, 32i8, 105i8, 109i8,
    109i8, 101i8, 100i8, 105i8, 97i8, 116i8, 101i8, 108i8, 121i8, 46i8, 10i8, 73i8, 102i8, 32i8,
    115i8, 105i8, 122i8, 101i8, 32i8, 105i8, 115i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 101i8, 100i8, 44i8, 32i8, 97i8, 116i8, 32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 115i8,
    105i8, 122i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8,
    32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 114i8, 101i8, 97i8, 100i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_readline")]
extern "C-unwind" fn _io__TextIOBase_readline(
    self_: *mut _object,
    cls: *mut _typeobject,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut size: i32 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            size = 1i32.wrapping_neg();
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__120),
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
                if nargs >= 1 {
                    size = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                    if size == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = _io__TextIOBase_readline_impl(self_, cls, size);
            }
        }
        return return_value;
    }
}

static _keywords__119: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__120: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__119.0) as *const *const i8 };
    __t0.fname = (b"readline\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io__TextIOBase_write__doc__: __SlateAlign16<[i8; 143]> = __SlateAlign16([
    119i8, 114i8, 105i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 87i8, 114i8, 105i8, 116i8, 101i8, 32i8,
    115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 115i8, 32i8, 116i8, 111i8, 32i8, 115i8, 116i8,
    114i8, 101i8, 97i8, 109i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8,
    99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 119i8, 114i8, 105i8,
    116i8, 116i8, 101i8, 110i8, 10i8, 40i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 105i8, 115i8,
    32i8, 97i8, 108i8, 119i8, 97i8, 121i8, 115i8, 32i8, 101i8, 113i8, 117i8, 97i8, 108i8, 32i8,
    116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 41i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_write")]
extern "C-unwind" fn _io__TextIOBase_write(
    self_: *mut _object,
    cls: *mut _typeobject,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut s_length: i64 = unsafe { core::mem::zeroed() };
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__175),
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
            if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                _PyArg_BadArgument(
                    c"write".as_ptr(),
                    c"argument 1".as_ptr(),
                    c"str".as_ptr(),
                    *args.wrapping_offset(0 as isize),
                );
            } else {
                s = PyUnicode_AsUTF8AndSize(
                    *args.wrapping_offset(0 as isize),
                    core::ptr::addr_of_mut!(s_length),
                );
                if !s.is_null() {
                    if (strlen(s as *const core::ffi::c_char) as u64) != (s_length as u64) {
                        PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                    } else {
                        return_value = _io__TextIOBase_write_impl(self_, cls, s);
                    }
                }
            }
        }
        return return_value;
    }
}

static _keywords__174: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__175: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__174.0) as *const *const i8 };
    __t0.fname = (b"write\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io__TextIOBase_encoding__doc__: __SlateAlign16<[i8; 58]> = __SlateAlign16([
    69i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 116i8, 101i8, 120i8, 116i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 46i8,
    10i8, 10i8, 83i8, 117i8, 98i8, 99i8, 108i8, 97i8, 115i8, 115i8, 101i8, 115i8, 32i8, 115i8,
    104i8, 111i8, 117i8, 108i8, 100i8, 32i8, 111i8, 118i8, 101i8, 114i8, 114i8, 105i8, 100i8,
    101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_encoding_get")]
extern "C-unwind" fn _io__TextIOBase_encoding_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    _io__TextIOBase_encoding_get_impl(self_)
}

static _io__TextIOBase_newlines__doc__: __SlateAlign16<[i8; 122]> = __SlateAlign16([
    76i8, 105i8, 110i8, 101i8, 32i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 115i8, 32i8, 116i8,
    114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 100i8, 32i8, 115i8, 111i8, 32i8, 102i8,
    97i8, 114i8, 46i8, 10i8, 10i8, 79i8, 110i8, 108i8, 121i8, 32i8, 108i8, 105i8, 110i8, 101i8,
    32i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 115i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8,
    108i8, 97i8, 116i8, 101i8, 100i8, 32i8, 100i8, 117i8, 114i8, 105i8, 110i8, 103i8, 32i8, 114i8,
    101i8, 97i8, 100i8, 105i8, 110i8, 103i8, 32i8, 97i8, 114i8, 101i8, 32i8, 99i8, 111i8, 110i8,
    115i8, 105i8, 100i8, 101i8, 114i8, 101i8, 100i8, 46i8, 10i8, 10i8, 83i8, 117i8, 98i8, 99i8,
    108i8, 97i8, 115i8, 115i8, 101i8, 115i8, 32i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8, 32i8,
    111i8, 118i8, 101i8, 114i8, 114i8, 105i8, 100i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_newlines_get")]
extern "C-unwind" fn _io__TextIOBase_newlines_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    _io__TextIOBase_newlines_get_impl(self_)
}

static _io__TextIOBase_errors__doc__: __SlateAlign16<[i8; 74]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 32i8, 115i8, 101i8, 116i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 99i8,
    111i8, 100i8, 101i8, 114i8, 32i8, 111i8, 114i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 101i8,
    114i8, 46i8, 10i8, 10i8, 83i8, 117i8, 98i8, 99i8, 108i8, 97i8, 115i8, 115i8, 101i8, 115i8,
    32i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8, 32i8, 111i8, 118i8, 101i8, 114i8, 114i8, 105i8,
    100i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io__TextIOBase_errors_get")]
extern "C-unwind" fn _io__TextIOBase_errors_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    _io__TextIOBase_errors_get_impl(self_)
}

static _io_IncrementalNewlineDecoder___init____doc__: __SlateAlign16<[i8; 485]> = __SlateAlign16([
    73i8, 110i8, 99i8, 114i8, 101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 108i8, 78i8, 101i8, 119i8,
    108i8, 105i8, 110i8, 101i8, 68i8, 101i8, 99i8, 111i8, 100i8, 101i8, 114i8, 40i8, 100i8, 101i8,
    99i8, 111i8, 100i8, 101i8, 114i8, 44i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8,
    116i8, 101i8, 44i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 61i8, 39i8, 115i8, 116i8,
    114i8, 105i8, 99i8, 116i8, 39i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 100i8, 101i8,
    99i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 114i8, 101i8,
    97i8, 100i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 105i8,
    110i8, 32i8, 117i8, 110i8, 105i8, 118i8, 101i8, 114i8, 115i8, 97i8, 108i8, 32i8, 110i8, 101i8,
    119i8, 108i8, 105i8, 110i8, 101i8, 115i8, 32i8, 109i8, 111i8, 100i8, 101i8, 46i8, 10i8, 10i8,
    73i8, 116i8, 32i8, 119i8, 114i8, 97i8, 112i8, 115i8, 32i8, 97i8, 110i8, 111i8, 116i8, 104i8,
    101i8, 114i8, 32i8, 105i8, 110i8, 99i8, 114i8, 101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 108i8,
    32i8, 100i8, 101i8, 99i8, 111i8, 100i8, 101i8, 114i8, 44i8, 32i8, 116i8, 114i8, 97i8, 110i8,
    115i8, 108i8, 97i8, 116i8, 105i8, 110i8, 103i8, 32i8, 92i8, 114i8, 92i8, 110i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 92i8, 114i8, 32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 92i8, 110i8, 46i8,
    10i8, 73i8, 116i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 114i8, 101i8, 99i8, 111i8, 114i8,
    100i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 32i8, 111i8,
    102i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 115i8, 32i8, 101i8, 110i8, 99i8,
    111i8, 117i8, 110i8, 116i8, 101i8, 114i8, 101i8, 100i8, 46i8, 32i8, 32i8, 87i8, 104i8, 101i8,
    110i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 10i8, 116i8, 114i8,
    97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 44i8,
    32i8, 105i8, 116i8, 32i8, 101i8, 110i8, 115i8, 117i8, 114i8, 101i8, 115i8, 32i8, 116i8, 104i8,
    97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8,
    32i8, 115i8, 101i8, 113i8, 117i8, 101i8, 110i8, 99i8, 101i8, 32i8, 105i8, 115i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 10i8, 111i8, 110i8, 101i8,
    32i8, 112i8, 105i8, 101i8, 99i8, 101i8, 46i8, 32i8, 87i8, 104i8, 101i8, 110i8, 32i8, 117i8,
    115i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 100i8, 101i8, 99i8, 111i8, 100i8,
    101i8, 114i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 105i8, 116i8, 32i8, 101i8, 120i8,
    112i8, 101i8, 99i8, 116i8, 115i8, 32i8, 117i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8,
    115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 115i8, 32i8, 97i8, 115i8, 10i8, 100i8, 101i8, 99i8,
    111i8, 100i8, 101i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 115i8, 32i8, 110i8, 101i8, 119i8,
    108i8, 105i8, 110i8, 101i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 111i8, 117i8, 116i8, 32i8,
    102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 105i8, 110i8, 118i8, 111i8, 107i8, 105i8, 110i8,
    103i8, 32i8, 97i8, 110i8, 32i8, 101i8, 120i8, 116i8, 101i8, 114i8, 110i8, 97i8, 108i8, 10i8,
    100i8, 101i8, 99i8, 111i8, 100i8, 101i8, 114i8, 46i8, 0i8,
]);

use slate_support::memchr;
use slate_support::memcpy;
use slate_support::strcmp;
use slate_support::strlen;

#[unsafe(link_section = ".text.slate_distinct.textio._io_IncrementalNewlineDecoder___init__")]
extern "C-unwind" fn _io_IncrementalNewlineDecoder___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut errors: *mut _object = unsafe { core::mem::zeroed() };
    let mut translate: i32 = unsafe { core::mem::zeroed() };
    let mut decoder: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v3: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v3 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v3 = 0;
        }
        noptargs = nargs.wrapping_add(__v3).wrapping_sub(2);
        errors = core::ptr::null_mut::<_object>();
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__337),
            2i32,
            3i32,
            0i32,
            0i32,
            (*__slate_slot_7).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            decoder = *fastargs.wrapping_offset(0 as isize);
            translate = PyObject_IsTrue(*fastargs.wrapping_offset(1 as isize));
            if translate >= 0 {
                if noptargs != 0 {
                    errors = *fastargs.wrapping_offset(2 as isize);
                }
                return_value = _io_IncrementalNewlineDecoder___init___impl(
                    self_ as *mut nldecoder_object,
                    decoder,
                    translate,
                    errors,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__324: __SlateRecord_textio_c_h_324 = {
    let mut __t0: __SlateRecord_textio_c_h_324 = unsafe { core::mem::zeroed() };
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
                    ._py_decoder
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
                    ._py_translate
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

static _keywords__336: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"decoder\0".as_ptr() as *mut i8) as *const i8,
    (b"translate\0".as_ptr() as *mut i8) as *const i8,
    (b"errors\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__337: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__336.0) as *const *const i8 };
    __t0.fname = (b"IncrementalNewlineDecoder\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__324.ob_base.ob_base) };
    __t0
};

static _io_IncrementalNewlineDecoder_decode__doc__: __SlateAlign16<[i8; 42]> = __SlateAlign16([
    100i8, 101i8, 99i8, 111i8, 100i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 44i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 44i8, 32i8, 102i8, 105i8, 110i8, 97i8,
    108i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_IncrementalNewlineDecoder_decode")]
extern "C-unwind" fn _io_IncrementalNewlineDecoder_decode(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut r#final: i32 = unsafe { core::mem::zeroed() };
    let mut input: *mut _object = unsafe { core::mem::zeroed() };
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
        '__join_0: {
            noptargs = nargs.wrapping_add(__v2).wrapping_sub(1);
            r#final = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__409),
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
                input = *args.wrapping_offset(0 as isize);
                if noptargs != 0 {
                    r#final = PyObject_IsTrue(*args.wrapping_offset(1 as isize));
                    if r#final < 0 {
                        break '__join_0;
                    }
                }
                return_value = _io_IncrementalNewlineDecoder_decode_impl(
                    self_ as *mut nldecoder_object,
                    input,
                    r#final,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__396: __SlateRecord_textio_c_h_396 = {
    let mut __t0: __SlateRecord_textio_c_h_396 = unsafe { core::mem::zeroed() };
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
                    ._py_input
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
                    ._py_final
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__408: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"input\0".as_ptr() as *mut i8) as *const i8,
    (b"final\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__409: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__408.0) as *const *const i8 };
    __t0.fname = (b"decode\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__396.ob_base.ob_base) };
    __t0
};

static _io_IncrementalNewlineDecoder_getstate__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    103i8, 101i8, 116i8, 115i8, 116i8, 97i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_IncrementalNewlineDecoder_getstate")]
extern "C-unwind" fn _io_IncrementalNewlineDecoder_getstate(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_IncrementalNewlineDecoder_getstate_impl(self_ as *mut nldecoder_object);
    return_value
}

static _io_IncrementalNewlineDecoder_setstate__doc__: __SlateAlign16<[i8; 31]> = __SlateAlign16([
    115i8, 101i8, 116i8, 115i8, 116i8, 97i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_IncrementalNewlineDecoder_setstate")]
extern "C-unwind" fn _io_IncrementalNewlineDecoder_setstate(
    self_: *mut _object,
    state: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value =
        _io_IncrementalNewlineDecoder_setstate_impl(self_ as *mut nldecoder_object, state);
    return_value
}

static _io_IncrementalNewlineDecoder_reset__doc__: __SlateAlign16<[i8; 21]> = __SlateAlign16([
    114i8, 101i8, 115i8, 101i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_IncrementalNewlineDecoder_reset")]
extern "C-unwind" fn _io_IncrementalNewlineDecoder_reset(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_IncrementalNewlineDecoder_reset_impl(self_ as *mut nldecoder_object);
    return_value
}

static _io_TextIOWrapper___init____doc__: __SlateAlign16<[i8; 1456]> = __SlateAlign16([
    84i8, 101i8, 120i8, 116i8, 73i8, 79i8, 87i8, 114i8, 97i8, 112i8, 112i8, 101i8, 114i8, 40i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 44i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8,
    110i8, 103i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8,
    115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8,
    101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 108i8, 105i8, 110i8, 101i8, 95i8, 98i8, 117i8, 102i8,
    102i8, 101i8, 114i8, 105i8, 110i8, 103i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 44i8, 32i8,
    119i8, 114i8, 105i8, 116i8, 101i8, 95i8, 116i8, 104i8, 114i8, 111i8, 117i8, 103i8, 104i8, 61i8,
    70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 97i8, 110i8, 100i8, 32i8, 108i8, 105i8, 110i8, 101i8,
    32i8, 98i8, 97i8, 115i8, 101i8, 100i8, 32i8, 108i8, 97i8, 121i8, 101i8, 114i8, 32i8, 111i8,
    118i8, 101i8, 114i8, 32i8, 97i8, 32i8, 66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8,
    73i8, 79i8, 66i8, 97i8, 115i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8, 32i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 46i8, 10i8, 10i8, 101i8, 110i8, 99i8, 111i8, 100i8,
    105i8, 110i8, 103i8, 32i8, 103i8, 105i8, 118i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    110i8, 97i8, 109i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 110i8,
    99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8,
    98i8, 101i8, 10i8, 100i8, 101i8, 99i8, 111i8, 100i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8,
    101i8, 110i8, 99i8, 111i8, 100i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 46i8, 32i8,
    73i8, 116i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 46i8, 103i8, 101i8, 116i8, 101i8, 110i8, 99i8,
    111i8, 100i8, 105i8, 110i8, 103i8, 40i8, 41i8, 46i8, 10i8, 10i8, 101i8, 114i8, 114i8, 111i8,
    114i8, 115i8, 32i8, 100i8, 101i8, 116i8, 101i8, 114i8, 109i8, 105i8, 110i8, 101i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 99i8, 116i8, 110i8, 101i8, 115i8, 115i8,
    32i8, 111i8, 102i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 100i8, 101i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 32i8, 40i8, 115i8,
    101i8, 101i8, 10i8, 104i8, 101i8, 108i8, 112i8, 40i8, 99i8, 111i8, 100i8, 101i8, 99i8, 115i8,
    46i8, 67i8, 111i8, 100i8, 101i8, 99i8, 41i8, 32i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 100i8, 111i8, 99i8, 117i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 99i8, 111i8, 100i8, 101i8, 99i8, 115i8, 46i8, 114i8, 101i8,
    103i8, 105i8, 115i8, 116i8, 101i8, 114i8, 41i8, 32i8, 97i8, 110i8, 100i8, 10i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 34i8, 115i8, 116i8, 114i8,
    105i8, 99i8, 116i8, 34i8, 46i8, 10i8, 10i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8,
    32i8, 99i8, 111i8, 110i8, 116i8, 114i8, 111i8, 108i8, 115i8, 32i8, 104i8, 111i8, 119i8, 32i8,
    108i8, 105i8, 110i8, 101i8, 32i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 115i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 104i8, 97i8, 110i8, 100i8, 108i8, 101i8, 100i8, 46i8, 32i8, 73i8, 116i8,
    32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 39i8,
    39i8, 44i8, 10i8, 39i8, 92i8, 110i8, 39i8, 44i8, 32i8, 39i8, 92i8, 114i8, 39i8, 44i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 39i8, 92i8, 114i8, 92i8, 110i8, 39i8, 46i8, 32i8, 32i8, 73i8, 116i8,
    32i8, 119i8, 111i8, 114i8, 107i8, 115i8, 32i8, 97i8, 115i8, 32i8, 102i8, 111i8, 108i8, 108i8,
    111i8, 119i8, 115i8, 58i8, 10i8, 10i8, 42i8, 32i8, 79i8, 110i8, 32i8, 105i8, 110i8, 112i8,
    117i8, 116i8, 44i8, 32i8, 105i8, 102i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 117i8, 110i8, 105i8, 118i8,
    101i8, 114i8, 115i8, 97i8, 108i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 115i8,
    32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 105i8, 115i8, 10i8, 32i8, 32i8, 101i8, 110i8, 97i8,
    98i8, 108i8, 101i8, 100i8, 46i8, 32i8, 76i8, 105i8, 110i8, 101i8, 115i8, 32i8, 105i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 99i8, 97i8, 110i8,
    32i8, 101i8, 110i8, 100i8, 32i8, 105i8, 110i8, 32i8, 39i8, 92i8, 110i8, 39i8, 44i8, 32i8, 39i8,
    92i8, 114i8, 39i8, 44i8, 32i8, 111i8, 114i8, 32i8, 39i8, 92i8, 114i8, 92i8, 110i8, 39i8, 44i8,
    32i8, 97i8, 110i8, 100i8, 10i8, 32i8, 32i8, 116i8, 104i8, 101i8, 115i8, 101i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 100i8, 32i8,
    105i8, 110i8, 116i8, 111i8, 32i8, 39i8, 92i8, 110i8, 39i8, 32i8, 98i8, 101i8, 102i8, 111i8,
    114i8, 101i8, 32i8, 98i8, 101i8, 105i8, 110i8, 103i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 10i8, 32i8, 32i8, 99i8,
    97i8, 108i8, 108i8, 101i8, 114i8, 46i8, 32i8, 73i8, 102i8, 32i8, 105i8, 116i8, 32i8, 105i8,
    115i8, 32i8, 39i8, 39i8, 44i8, 32i8, 117i8, 110i8, 105i8, 118i8, 101i8, 114i8, 115i8, 97i8,
    108i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 32i8, 109i8, 111i8, 100i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 44i8, 32i8, 98i8,
    117i8, 116i8, 32i8, 108i8, 105i8, 110i8, 101i8, 10i8, 32i8, 32i8, 101i8, 110i8, 100i8, 105i8,
    110i8, 103i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8,
    101i8, 114i8, 32i8, 117i8, 110i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8,
    100i8, 46i8, 32i8, 73i8, 102i8, 32i8, 105i8, 116i8, 32i8, 104i8, 97i8, 115i8, 32i8, 97i8,
    110i8, 121i8, 32i8, 111i8, 102i8, 10i8, 32i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 116i8,
    104i8, 101i8, 114i8, 32i8, 108i8, 101i8, 103i8, 97i8, 108i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 115i8, 44i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 108i8, 105i8, 110i8, 101i8,
    115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8, 116i8, 101i8, 114i8,
    109i8, 105i8, 110i8, 97i8, 116i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 10i8, 32i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8,
    103i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 110i8,
    101i8, 32i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 115i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    99i8, 97i8, 108i8, 108i8, 101i8, 114i8, 32i8, 117i8, 110i8, 116i8, 114i8, 97i8, 110i8, 115i8,
    108i8, 97i8, 116i8, 101i8, 100i8, 46i8, 10i8, 10i8, 42i8, 32i8, 79i8, 110i8, 32i8, 111i8,
    117i8, 116i8, 112i8, 117i8, 116i8, 44i8, 32i8, 105i8, 102i8, 32i8, 110i8, 101i8, 119i8, 108i8,
    105i8, 110i8, 101i8, 32i8, 105i8, 115i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 97i8,
    110i8, 121i8, 32i8, 39i8, 92i8, 110i8, 39i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8,
    101i8, 114i8, 115i8, 32i8, 119i8, 114i8, 105i8, 116i8, 116i8, 101i8, 110i8, 32i8, 97i8, 114i8,
    101i8, 10i8, 32i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 100i8,
    32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8,
    32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 108i8, 105i8, 110i8, 101i8, 32i8,
    115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 44i8, 32i8, 111i8, 115i8, 46i8,
    108i8, 105i8, 110i8, 101i8, 115i8, 101i8, 112i8, 46i8, 32i8, 73i8, 102i8, 10i8, 32i8, 32i8,
    110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 32i8, 105i8, 115i8, 32i8, 39i8, 39i8, 32i8,
    111i8, 114i8, 32i8, 39i8, 92i8, 110i8, 39i8, 44i8, 32i8, 110i8, 111i8, 32i8, 116i8, 114i8,
    97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 97i8, 107i8, 101i8,
    115i8, 32i8, 112i8, 108i8, 97i8, 99i8, 101i8, 46i8, 32i8, 73i8, 102i8, 32i8, 110i8, 101i8,
    119i8, 108i8, 105i8, 110i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 110i8, 121i8, 10i8, 32i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8,
    108i8, 101i8, 103i8, 97i8, 108i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 44i8, 32i8,
    97i8, 110i8, 121i8, 32i8, 39i8, 92i8, 110i8, 39i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8,
    116i8, 101i8, 114i8, 115i8, 32i8, 119i8, 114i8, 105i8, 116i8, 116i8, 101i8, 110i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 100i8, 10i8,
    32i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 108i8,
    105i8, 110i8, 101i8, 95i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 105i8, 110i8, 103i8, 32i8,
    105i8, 115i8, 32i8, 84i8, 114i8, 117i8, 101i8, 44i8, 32i8, 97i8, 32i8, 99i8, 97i8, 108i8,
    108i8, 32i8, 116i8, 111i8, 32i8, 102i8, 108i8, 117i8, 115i8, 104i8, 32i8, 105i8, 115i8, 32i8,
    105i8, 109i8, 112i8, 108i8, 105i8, 101i8, 100i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 97i8,
    32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 116i8, 111i8, 10i8, 119i8, 114i8, 105i8, 116i8, 101i8,
    32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 115i8, 32i8, 97i8, 32i8, 110i8, 101i8,
    119i8, 108i8, 105i8, 110i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8,
    114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper___init__")]
extern "C-unwind" fn _io_TextIOWrapper___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut newline_length: i64 = unsafe { core::mem::zeroed() };
    let mut encoding_length: i64 = unsafe { core::mem::zeroed() };
    let mut write_through: i32 = unsafe { core::mem::zeroed() };
    let mut line_buffering: i32 = unsafe { core::mem::zeroed() };
    let mut newline: *const i8 = unsafe { core::mem::zeroed() };
    let mut errors: *mut _object = unsafe { core::mem::zeroed() };
    let mut encoding: *const i8 = unsafe { core::mem::zeroed() };
    let mut buffer: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v16: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 6]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_20: *mut [*mut _object; 6] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 6];
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v16 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v16 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v16).wrapping_sub(1);
            encoding = core::ptr::null::<i8>();
            errors = core::ptr::addr_of_mut!(_Py_NoneStruct);
            newline = core::ptr::null::<i8>();
            line_buffering = 0;
            write_through = 0;
            fastargs = _PyArg_UnpackKeywords__2!(
                (*(args as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object,
                nargs,
                kwargs,
                core::ptr::null_mut::<()>(),
                core::ptr::addr_of_mut!(_parser__577),
                1i32,
                6i32,
                0i32,
                0i32,
                (*__slate_slot_20).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !fastargs.is_null() {
                '__join_1: {
                    buffer = *fastargs.wrapping_offset(0 as isize);
                    if noptargs != 0 {
                        if !unsafe { *fastargs.wrapping_offset(1 as isize) }.is_null() {
                            if *fastargs.wrapping_offset(1 as isize)
                                == core::ptr::addr_of_mut!(_Py_NoneStruct)
                            {
                                encoding = core::ptr::null::<i8>();
                            } else {
                                if PyUnicode_Check!(unsafe {
                                    *fastargs.wrapping_offset(1 as isize)
                                }) != 0
                                {
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
                                        }
                                    }
                                } else {
                                    _PyArg_BadArgument(
                                        c"TextIOWrapper".as_ptr(),
                                        c"argument 'encoding'".as_ptr(),
                                        c"str or None".as_ptr(),
                                        *fastargs.wrapping_offset(1 as isize),
                                    );
                                    break '__join_0;
                                }
                            }
                            let __v7: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v7;
                            if __v7 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *fastargs.wrapping_offset(2 as isize) }.is_null() {
                            errors = *fastargs.wrapping_offset(2 as isize);
                            let __v5: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v5;
                            if __v5 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *fastargs.wrapping_offset(3 as isize) }.is_null() {
                            if *fastargs.wrapping_offset(3 as isize)
                                == core::ptr::addr_of_mut!(_Py_NoneStruct)
                            {
                                newline = core::ptr::null::<i8>();
                            } else {
                                if PyUnicode_Check!(unsafe {
                                    *fastargs.wrapping_offset(3 as isize)
                                }) != 0
                                {
                                    newline = PyUnicode_AsUTF8AndSize(
                                        *fastargs.wrapping_offset(3 as isize),
                                        core::ptr::addr_of_mut!(newline_length),
                                    );
                                    if newline.is_null() {
                                        break '__join_0;
                                    } else {
                                        if (strlen(newline as *const core::ffi::c_char) as u64)
                                            != (newline_length as u64)
                                        {
                                            PyErr_SetString(
                                                PyExc_ValueError,
                                                c"embedded null character".as_ptr(),
                                            );
                                            break '__join_0;
                                        }
                                    }
                                } else {
                                    _PyArg_BadArgument(
                                        c"TextIOWrapper".as_ptr(),
                                        c"argument 'newline'".as_ptr(),
                                        c"str or None".as_ptr(),
                                        *fastargs.wrapping_offset(3 as isize),
                                    );
                                    break '__join_0;
                                }
                            }
                            let __v2: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v2;
                            if __v2 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *fastargs.wrapping_offset(4 as isize) }.is_null() {
                            line_buffering = PyObject_IsTrue(*fastargs.wrapping_offset(4 as isize));
                            if line_buffering < 0 {
                                break '__join_0;
                            } else {
                                let __v0: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v0;
                                if __v0 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        write_through = PyObject_IsTrue(*fastargs.wrapping_offset(5 as isize));
                        if write_through < 0 {
                            break '__join_0;
                        }
                    }
                }
                return_value = _io_TextIOWrapper___init___impl(
                    self_ as *mut textio,
                    buffer,
                    encoding,
                    errors,
                    newline,
                    line_buffering,
                    write_through,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__564: __SlateRecord_textio_c_h_564 = {
    let mut __t0: __SlateRecord_textio_c_h_564 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 6i64,
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
                    ._py_buffer
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
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_newline
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
                    ._py_line_buffering
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
                    ._py_write_through
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__576: __SlateAlign16<[*const i8; 7]> = __SlateAlign16([
    (b"buffer\0".as_ptr() as *mut i8) as *const i8,
    (b"encoding\0".as_ptr() as *mut i8) as *const i8,
    (b"errors\0".as_ptr() as *mut i8) as *const i8,
    (b"newline\0".as_ptr() as *mut i8) as *const i8,
    (b"line_buffering\0".as_ptr() as *mut i8) as *const i8,
    (b"write_through\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__577: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__576.0) as *const *const i8 };
    __t0.fname = (b"TextIOWrapper\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__564.ob_base.ob_base) };
    __t0
};

static _io_TextIOWrapper_reconfigure__doc__: __SlateAlign16<[i8; 215]> = __SlateAlign16([
    114i8, 101i8, 99i8, 111i8, 110i8, 102i8, 105i8, 103i8, 117i8, 114i8, 101i8, 40i8, 36i8, 115i8,
    101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 44i8, 32i8, 42i8, 44i8, 32i8, 101i8, 110i8, 99i8, 111i8,
    100i8, 105i8, 110i8, 103i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 101i8, 114i8, 114i8,
    111i8, 114i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 110i8, 101i8, 119i8, 108i8,
    105i8, 110i8, 101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 108i8, 105i8, 110i8, 101i8, 95i8, 98i8, 117i8, 102i8,
    102i8, 101i8, 114i8, 105i8, 110i8, 103i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 119i8,
    114i8, 105i8, 116i8, 101i8, 95i8, 116i8, 104i8, 114i8, 111i8, 117i8, 103i8, 104i8, 61i8, 78i8,
    111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 99i8, 111i8, 110i8,
    102i8, 105i8, 103i8, 117i8, 114i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 101i8, 120i8,
    116i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8,
    110i8, 101i8, 119i8, 32i8, 112i8, 97i8, 114i8, 97i8, 109i8, 101i8, 116i8, 101i8, 114i8, 115i8,
    46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 100i8,
    111i8, 101i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 109i8, 112i8, 108i8, 105i8, 99i8, 105i8,
    116i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 102i8, 108i8, 117i8, 115i8, 104i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_reconfigure")]
extern "C-unwind" fn _io_TextIOWrapper_reconfigure(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut write_through_obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut line_buffering_obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut newline_obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut errors: *mut _object = unsafe { core::mem::zeroed() };
    let mut encoding: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v13: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 5]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_15: *mut [*mut _object; 5] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 5];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v13 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v13 = 0;
        }
        noptargs = nargs.wrapping_add(__v13).wrapping_sub(0);
        encoding = core::ptr::addr_of_mut!(_Py_NoneStruct);
        errors = core::ptr::addr_of_mut!(_Py_NoneStruct);
        newline_obj = core::ptr::null_mut::<_object>();
        line_buffering_obj = core::ptr::addr_of_mut!(_Py_NoneStruct);
        write_through_obj = core::ptr::addr_of_mut!(_Py_NoneStruct);
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__720),
            0i32,
            0i32,
            0i32,
            0i32,
            (*__slate_slot_15).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            '__join_1: {
                if noptargs != 0 {
                    if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                        encoding = *args.wrapping_offset(0 as isize);
                        let __v6: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v6;
                        if __v6 == 0 {
                            break '__join_1;
                        }
                    }
                    if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                        errors = *args.wrapping_offset(1 as isize);
                        let __v4: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v4;
                        if __v4 == 0 {
                            break '__join_1;
                        }
                    }
                    if !unsafe { *args.wrapping_offset(2 as isize) }.is_null() {
                        newline_obj = *args.wrapping_offset(2 as isize);
                        let __v2: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v2;
                        if __v2 == 0 {
                            break '__join_1;
                        }
                    }
                    if !unsafe { *args.wrapping_offset(3 as isize) }.is_null() {
                        line_buffering_obj = *args.wrapping_offset(3 as isize);
                        let __v0: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v0;
                        if __v0 == 0 {
                            break '__join_1;
                        }
                    }
                    write_through_obj = *args.wrapping_offset(4 as isize);
                }
            }
            return_value = _io_TextIOWrapper_reconfigure_impl(
                self_ as *mut textio,
                encoding,
                errors,
                newline_obj,
                line_buffering_obj,
                write_through_obj,
            );
        }
        return return_value;
    }
}

static mut _kwtuple__707: __SlateRecord_textio_c_h_707 = {
    let mut __t0: __SlateRecord_textio_c_h_707 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 5i64,
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
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_newline
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
                    ._py_line_buffering
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
                    ._py_write_through
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__719: __SlateAlign16<[*const i8; 6]> = __SlateAlign16([
    (b"encoding\0".as_ptr() as *mut i8) as *const i8,
    (b"errors\0".as_ptr() as *mut i8) as *const i8,
    (b"newline\0".as_ptr() as *mut i8) as *const i8,
    (b"line_buffering\0".as_ptr() as *mut i8) as *const i8,
    (b"write_through\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__720: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__719.0) as *const *const i8 };
    __t0.fname = (b"reconfigure\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__707.ob_base.ob_base) };
    __t0
};

static _io_TextIOWrapper_detach__doc__: __SlateAlign16<[i8; 22]> = __SlateAlign16([
    100i8, 101i8, 116i8, 97i8, 99i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_detach")]
extern "C-unwind" fn _io_TextIOWrapper_detach(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_detach_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_write__doc__: __SlateAlign16<[i8; 27]> = __SlateAlign16([
    119i8, 114i8, 105i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 116i8,
    101i8, 120i8, 116i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_write")]
extern "C-unwind" fn _io_TextIOWrapper_write(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"write".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            let text: *mut _object = arg;
            return_value = _io_TextIOWrapper_write_impl(self_ as *mut textio, text);
        }
        return return_value;
    }
}

static _io_TextIOWrapper_read__doc__: __SlateAlign16<[i8; 29]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 105i8,
    122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_read")]
extern "C-unwind" fn _io_TextIOWrapper_read(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            n = 1i32.wrapping_neg() as i64;
            if _PyArg_CheckPositional!(c"read".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
                if nargs >= 1 {
                    if _Py_convert_optional_to_ssize_t(
                        *args.wrapping_offset(0 as isize),
                        core::ptr::addr_of_mut!(n) as *mut (),
                    ) == 0
                    {
                        break '__join_0;
                    }
                }
                return_value = _io_TextIOWrapper_read_impl(self_ as *mut textio, n);
            }
        }
        return return_value;
    }
}

static _io_TextIOWrapper_readline__doc__: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 108i8, 105i8, 110i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 115i8, 105i8, 122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_readline")]
extern "C-unwind" fn _io_TextIOWrapper_readline(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut size: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            size = 1i32.wrapping_neg() as i64;
            if _PyArg_CheckPositional!(c"readline".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
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
                        size = ival;
                    }
                }
                return_value = _io_TextIOWrapper_readline_impl(self_ as *mut textio, size);
            }
        }
        return return_value;
    }
}

static _io_TextIOWrapper_seek__doc__: __SlateAlign16<[i8; 652]> = __SlateAlign16([
    115i8, 101i8, 101i8, 107i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 99i8, 111i8,
    111i8, 107i8, 105i8, 101i8, 44i8, 32i8, 119i8, 104i8, 101i8, 110i8, 99i8, 101i8, 61i8, 111i8,
    115i8, 46i8, 83i8, 69i8, 69i8, 75i8, 95i8, 83i8, 69i8, 84i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8,
    114i8, 101i8, 97i8, 109i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 44i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 110i8, 101i8, 119i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 112i8,
    111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8, 32i8, 32i8, 99i8, 111i8,
    111i8, 107i8, 105i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 90i8, 101i8, 114i8, 111i8, 32i8,
    111i8, 114i8, 32i8, 97i8, 110i8, 32i8, 111i8, 112i8, 97i8, 113i8, 117i8, 101i8, 32i8, 110i8,
    117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8,
    32i8, 98i8, 121i8, 32i8, 116i8, 101i8, 108i8, 108i8, 40i8, 41i8, 46i8, 10i8, 32i8, 32i8, 119i8,
    104i8, 101i8, 110i8, 99i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8,
    114i8, 101i8, 108i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 116i8, 111i8, 32i8, 115i8, 101i8, 101i8, 107i8, 32i8, 102i8, 114i8,
    111i8, 109i8, 46i8, 10i8, 10i8, 70i8, 111i8, 117i8, 114i8, 32i8, 111i8, 112i8, 101i8, 114i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 115i8, 117i8, 112i8,
    112i8, 111i8, 114i8, 116i8, 101i8, 100i8, 44i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8,
    98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 111i8, 108i8, 108i8, 111i8, 119i8, 105i8,
    110i8, 103i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 10i8, 99i8, 111i8,
    109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 58i8, 10i8, 10i8, 45i8,
    32i8, 115i8, 101i8, 101i8, 107i8, 40i8, 48i8, 44i8, 32i8, 83i8, 69i8, 69i8, 75i8, 95i8, 83i8,
    69i8, 84i8, 41i8, 58i8, 32i8, 82i8, 101i8, 119i8, 105i8, 110i8, 100i8, 32i8, 116i8, 111i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 111i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 46i8, 10i8, 45i8, 32i8,
    115i8, 101i8, 101i8, 107i8, 40i8, 99i8, 111i8, 111i8, 107i8, 105i8, 101i8, 44i8, 32i8, 83i8,
    69i8, 69i8, 75i8, 95i8, 83i8, 69i8, 84i8, 41i8, 58i8, 32i8, 82i8, 101i8, 115i8, 116i8, 111i8,
    114i8, 101i8, 32i8, 97i8, 32i8, 112i8, 114i8, 101i8, 118i8, 105i8, 111i8, 117i8, 115i8, 32i8,
    112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 59i8, 10i8, 32i8, 32i8, 39i8, 99i8,
    111i8, 111i8, 107i8, 105i8, 101i8, 39i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 98i8, 101i8,
    32i8, 97i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 101i8, 108i8, 108i8, 40i8, 41i8,
    46i8, 10i8, 45i8, 32i8, 115i8, 101i8, 101i8, 107i8, 40i8, 48i8, 44i8, 32i8, 83i8, 69i8, 69i8,
    75i8, 95i8, 69i8, 78i8, 68i8, 41i8, 58i8, 32i8, 70i8, 97i8, 115i8, 116i8, 45i8, 102i8, 111i8,
    114i8, 119i8, 97i8, 114i8, 100i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8,
    110i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 101i8,
    97i8, 109i8, 46i8, 10i8, 45i8, 32i8, 115i8, 101i8, 101i8, 107i8, 40i8, 48i8, 44i8, 32i8, 83i8,
    69i8, 69i8, 75i8, 95i8, 67i8, 85i8, 82i8, 41i8, 58i8, 32i8, 76i8, 101i8, 97i8, 118i8, 101i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8,
    116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 46i8, 10i8, 10i8, 65i8,
    110i8, 121i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 32i8, 99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 105i8, 110i8, 118i8, 97i8, 108i8, 105i8, 100i8,
    44i8, 10i8, 97i8, 110i8, 100i8, 32i8, 109i8, 97i8, 121i8, 32i8, 114i8, 97i8, 105i8, 115i8,
    101i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_seek")]
extern "C-unwind" fn _io_TextIOWrapper_seek(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut whence: i32 = unsafe { core::mem::zeroed() };
    let mut cookieObj: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            whence = 0;
            if _PyArg_CheckPositional!(c"seek".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
                cookieObj = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    whence = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                    if whence == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = _io_TextIOWrapper_seek_impl(self_ as *mut textio, cookieObj, whence);
            }
        }
        return return_value;
    }
}

static _io_TextIOWrapper_tell__doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    116i8, 101i8, 108i8, 108i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 97i8, 115i8, 32i8, 97i8, 110i8, 32i8, 111i8, 112i8, 97i8, 113i8,
    117i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 46i8, 10i8, 10i8, 84i8, 104i8,
    101i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 101i8, 108i8, 108i8, 40i8, 41i8, 32i8, 99i8, 97i8, 110i8,
    32i8, 98i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 97i8, 115i8, 32i8, 105i8,
    110i8, 112i8, 117i8, 116i8, 32i8, 116i8, 111i8, 32i8, 115i8, 101i8, 101i8, 107i8, 40i8, 41i8,
    44i8, 32i8, 116i8, 111i8, 10i8, 114i8, 101i8, 115i8, 116i8, 111i8, 114i8, 101i8, 32i8, 97i8,
    32i8, 112i8, 114i8, 101i8, 118i8, 105i8, 111i8, 117i8, 115i8, 32i8, 115i8, 116i8, 114i8, 101i8,
    97i8, 109i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_tell")]
extern "C-unwind" fn _io_TextIOWrapper_tell(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_tell_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_truncate__doc__: __SlateAlign16<[i8; 34]> = __SlateAlign16([
    116i8, 114i8, 117i8, 110i8, 99i8, 97i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 112i8, 111i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_truncate")]
extern "C-unwind" fn _io_TextIOWrapper_truncate(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut pos: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        pos = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"truncate".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                pos = *args.wrapping_offset(0 as isize);
            }
            return_value = _io_TextIOWrapper_truncate_impl(self_ as *mut textio, pos);
        }
        return return_value;
    }
}

static _io_TextIOWrapper_fileno__doc__: __SlateAlign16<[i8; 22]> = __SlateAlign16([
    102i8, 105i8, 108i8, 101i8, 110i8, 111i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_fileno")]
extern "C-unwind" fn _io_TextIOWrapper_fileno(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_fileno_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_seekable__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    115i8, 101i8, 101i8, 107i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_seekable")]
extern "C-unwind" fn _io_TextIOWrapper_seekable(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_seekable_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_readable__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_readable")]
extern "C-unwind" fn _io_TextIOWrapper_readable(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_readable_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_writable__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    119i8, 114i8, 105i8, 116i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_writable")]
extern "C-unwind" fn _io_TextIOWrapper_writable(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_writable_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_isatty__doc__: __SlateAlign16<[i8; 22]> = __SlateAlign16([
    105i8, 115i8, 97i8, 116i8, 116i8, 121i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_isatty")]
extern "C-unwind" fn _io_TextIOWrapper_isatty(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_isatty_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_flush__doc__: __SlateAlign16<[i8; 21]> = __SlateAlign16([
    102i8, 108i8, 117i8, 115i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_flush")]
extern "C-unwind" fn _io_TextIOWrapper_flush(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_flush_impl(self_ as *mut textio);
    return_value
}

static _io_TextIOWrapper_close__doc__: __SlateAlign16<[i8; 21]> = __SlateAlign16([
    99i8, 108i8, 111i8, 115i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_close")]
extern "C-unwind" fn _io_TextIOWrapper_close(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_close_impl(self_ as *mut textio);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_name_get")]
extern "C-unwind" fn _io_TextIOWrapper_name_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_name_get_impl(self_ as *mut textio);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_closed_get")]
extern "C-unwind" fn _io_TextIOWrapper_closed_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_closed_get_impl(self_ as *mut textio);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_newlines_get")]
extern "C-unwind" fn _io_TextIOWrapper_newlines_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_newlines_get_impl(self_ as *mut textio);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper_errors_get")]
extern "C-unwind" fn _io_TextIOWrapper_errors_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper_errors_get_impl(self_ as *mut textio);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper__CHUNK_SIZE_get")]
extern "C-unwind" fn _io_TextIOWrapper__CHUNK_SIZE_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io_TextIOWrapper__CHUNK_SIZE_get_impl(self_ as *mut textio);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.textio._io_TextIOWrapper__CHUNK_SIZE_set")]
extern "C-unwind" fn _io_TextIOWrapper__CHUNK_SIZE_set(
    self_: *mut _object,
    value: *mut _object,
    _unused_context: *mut (),
) -> i32 {
    let mut return_value: i32 = 0;
    if value.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_AttributeError,
                c"attribute '_CHUNK_SIZE' of '%.100s' objects cannot be deleted".as_ptr(),
                (*Py_TYPE!(self_)).tp_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    return_value = _io_TextIOWrapper__CHUNK_SIZE_set_impl(self_ as *mut textio, value);
    return_value
}

// _PyObject_CallMethod()
// _PyCodecInfo_GetIncrementalDecoder()
// _Py_GetLocaleEncoding()
// PyInterpreterState.fs_codec
// _PyLong_GetZero()
// _PyObject_GC_UNTRACK()
// _PyErr_ChainExceptions1()
// _PyInterpreterState_GET()
// _PyUnicode_AsASCIIString()
// FT_CLEAR_WEAKREFS()
// [clinic input]
// module _io
// class _io.IncrementalNewlineDecoder "nldecoder_object *" "clinic_state()->PyIncrementalNewlineDecoder_Type"
// class _io.TextIOWrapper "textio *" "clinic_state()->TextIOWrapper_Type"
// class _io._TextIOBase "PyObject *" "&PyTextIOBase_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=8b7f24fa13bfdd7f]
// TextIOBase
static textiobase_doc: __SlateAlign16<[i8; 177]> = __SlateAlign16([
    66i8, 97i8, 115i8, 101i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 116i8, 101i8, 120i8, 116i8, 32i8, 73i8, 47i8, 79i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8,
    115i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8,
    101i8, 115i8, 32i8, 97i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 108i8, 105i8, 110i8, 101i8, 32i8, 98i8, 97i8, 115i8, 101i8,
    100i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 102i8, 97i8, 99i8, 101i8, 32i8, 116i8, 111i8,
    32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 10i8, 73i8, 47i8, 79i8, 46i8, 32i8, 84i8, 104i8,
    101i8, 114i8, 101i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 32i8, 114i8, 101i8, 97i8, 100i8,
    105i8, 110i8, 116i8, 111i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 32i8, 98i8, 101i8,
    99i8, 97i8, 117i8, 115i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 39i8, 115i8,
    32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 115i8, 116i8, 114i8,
    105i8, 110i8, 103i8, 115i8, 10i8, 97i8, 114i8, 101i8, 32i8, 105i8, 109i8, 109i8, 117i8, 116i8,
    97i8, 98i8, 108i8, 101i8, 46i8, 10i8, 0i8,
]);

fn _unsupported(state: *mut _io_state, message: *const i8) -> *mut _object {
    unsafe { PyErr_SetString((*state).unsupported_operation, message) };
    core::ptr::null_mut::<_object>()
}

// [clinic input]
// _io._TextIOBase.detach
//     cls: defining_class
//     /
//
// Separate the underlying buffer from the TextIOBase and return it.
//
// After the underlying buffer has been detached, the TextIO is in
// an unusable state.
// [clinic start generated code]
/// [clinic end generated code: output=50915f40c609eaa4 input=8099c088abcb87d8]
fn _io__TextIOBase_detach_impl(self_: *mut _object, cls: *mut _typeobject) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    _unsupported(state, c"detach".as_ptr())
}

// [clinic input]
// _io._TextIOBase.read
//     cls: defining_class
//     size: int(unused=True) = -1
//     /
//
// Read at most size characters from stream.
//
// Read from underlying buffer until we have size characters or we hit
// EOF.  If size is negative or omitted, read until EOF.
// [clinic start generated code]
/// [clinic end generated code: output=51a5178a309ce647 input=c9fd4cc1cf1b4614]
fn _io__TextIOBase_read_impl(
    self_: *mut _object,
    cls: *mut _typeobject,
    _unused_size: i32,
) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    _unsupported(state, c"read".as_ptr())
}

// [clinic input]
// _io._TextIOBase.readline
//     cls: defining_class
//     size: int(unused=True) = -1
//     /
//
// Read until newline or EOF.
//
// Return an empty string if EOF is hit immediately.
// If size is specified, at most size characters will be read.
// [clinic start generated code]
/// [clinic end generated code: output=3f47d7966d6d074e input=42eafec94107fa27]
fn _io__TextIOBase_readline_impl(
    self_: *mut _object,
    cls: *mut _typeobject,
    _unused_size: i32,
) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    _unsupported(state, c"readline".as_ptr())
}

// [clinic input]
// _io._TextIOBase.write
//     cls: defining_class
//     s: str(unused=True)
//     /
//
// Write string s to stream.
//
// Return the number of characters written
// (which is always equal to the length of the string).
// [clinic start generated code]
/// [clinic end generated code: output=18b28231460275de input=e9cabaa5f6732b07]
fn _io__TextIOBase_write_impl(
    self_: *mut _object,
    cls: *mut _typeobject,
    _unused_s: *const i8,
) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    _unsupported(state, c"write".as_ptr())
}

// [clinic input]
// @getter
// _io._TextIOBase.encoding
//
// Encoding of the text stream.
//
// Subclasses should override.
// [clinic start generated code]
/// [clinic end generated code: output=e0f5d8f548b92432 input=4736d7621dd38f43]
fn _io__TextIOBase_encoding_get_impl(self_: *mut _object) -> *mut _object {
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// @getter
// _io._TextIOBase.newlines
//
// Line endings translated so far.
//
// Only line endings translated during reading are considered.
//
// Subclasses should override.
// [clinic start generated code]
/// [clinic end generated code: output=46ec147fb9f00c2a input=a5b196d076af1164]
fn _io__TextIOBase_newlines_get_impl(self_: *mut _object) -> *mut _object {
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// @getter
// _io._TextIOBase.errors
//
// The error setting of the decoder or encoder.
//
// Subclasses should override.
// [clinic start generated code]
/// [clinic end generated code: output=c6623d6addcd087d input=974aa52d1db93a82]
fn _io__TextIOBase_errors_get_impl(self_: *mut _object) -> *mut _object {
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

static mut textiobase_methods: __SlateAlign16<[PyMethodDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"detach\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _typeobject,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_io__TextIOBase_detach))
            })
        };
        __t0.ml_flags = 642i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_io__TextIOBase_detach__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"read\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _typeobject,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_io__TextIOBase_read))
            })
        };
        __t1.ml_flags = 642i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(_io__TextIOBase_read__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"readline\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _typeobject,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_io__TextIOBase_readline))
            })
        };
        __t2.ml_flags = 642i32;
        __t2.ml_doc =
            unsafe { core::ptr::addr_of!(_io__TextIOBase_readline__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"write\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _typeobject,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_io__TextIOBase_write))
            })
        };
        __t3.ml_flags = 642i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(_io__TextIOBase_write__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = core::ptr::null::<i8>();
        __t4.ml_meth = None;
        __t4
    },
]);

static mut textiobase_getset: __SlateAlign16<[PyGetSetDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"encoding\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(_io__TextIOBase_encoding_get);
        __t0.set = None;
        __t0.doc = unsafe { core::ptr::addr_of!(_io__TextIOBase_encoding__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"newlines\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(_io__TextIOBase_newlines_get);
        __t1.set = None;
        __t1.doc = unsafe { core::ptr::addr_of!(_io__TextIOBase_newlines__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"errors\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(_io__TextIOBase_errors_get);
        __t2.set = None;
        __t2.doc = unsafe { core::ptr::addr_of!(_io__TextIOBase_errors__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut textiobase_slots: __SlateAlign16<[__SlateRecord_object_h_377; 4]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 56i32;
        __t0.pfunc = (unsafe { core::ptr::addr_of!(textiobase_doc.0) as *const i8 }) as *mut ();
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 64i32;
        __t1.pfunc = (unsafe { core::ptr::addr_of_mut!(textiobase_methods.0) as *mut PyMethodDef })
            as *mut ();
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 73i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of_mut!(textiobase_getset.0) as *mut PyGetSetDef })
            as *mut ();
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 0i32;
        __t3.pfunc = core::ptr::null_mut::<()>();
        __t3
    },
]);

// IncrementalNewlineDecoder
// Do not set Py_TPFLAGS_HAVE_GC so that tp_traverse and tp_clear are inherited
#[repr(C)]
#[derive(Clone, Copy)]
struct nldecoder_object {
    ob_base: _object,
    decoder: *mut _object,
    errors: *mut _object,
    __slate_bits_0: __slate_bits::__SlateBits1315U0,
}

// [clinic input]
// _io.IncrementalNewlineDecoder.__init__
//     decoder: object
//     translate: bool
//     errors: object(c_default="NULL") = "strict"
//
// Codec used when reading a file in universal newlines mode.
//
// It wraps another incremental decoder, translating \r\n and \r into \n.
// It also records the types of newlines encountered.  When used with
// translate=False, it ensures that the newline sequence is returned in
// one piece. When used with decoder=None, it expects unicode strings as
// decode input and translates newlines without first invoking an external
// decoder.
// [clinic start generated code]
/// [clinic end generated code: output=fbd04d443e764ec2 input=ed547aa257616b0e]
fn _io_IncrementalNewlineDecoder___init___impl(
    self_: *mut nldecoder_object,
    decoder: *mut _object,
    translate: i32,
    mut errors: *mut _object,
) -> i32 {
    if errors.is_null() {
        errors = unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_strict
                    ._ascii
                    .ob_base
            )
        };
    } else {
        errors = Py_NewRef!(errors);
    }
    Py_XSETREF!((*self_).errors, errors);
    Py_XSETREF!((*self_).decoder, Py_NewRef!(decoder));
    unsafe {
        let __t0 = (if translate != 0 { 1i32 } else { 0i32 }) as u32;
        (*self_).__slate_bits_0.__set_translate(__t0);
    }
    unsafe {
        (*self_).__slate_bits_0.__set_seennl(0u32);
    }
    unsafe {
        (*self_).__slate_bits_0.__set_pendingcr(0u32);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.textio.incrementalnewlinedecoder_traverse")]
extern "C-unwind" fn incrementalnewlinedecoder_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut nldecoder_object = nldecoder_object_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(self_ as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*self_).decoder }, visit, arg);
    Py_VISIT!(unsafe { (*self_).errors }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.textio.incrementalnewlinedecoder_clear")]
extern "C-unwind" fn incrementalnewlinedecoder_clear(op: *mut _object) -> i32 {
    let self_: *mut nldecoder_object = nldecoder_object_CAST!(op);
    Py_CLEAR!((*self_).decoder);
    Py_CLEAR!((*self_).errors);
    0
}

#[unsafe(link_section = ".text.slate_distinct.textio.incrementalnewlinedecoder_dealloc")]
extern "C-unwind" fn incrementalnewlinedecoder_dealloc(op: *mut _object) {
    let self_: *mut nldecoder_object = nldecoder_object_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(self_);
    _PyObject_GC_UNTRACK__2!(self_);
    incrementalnewlinedecoder_clear(op);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(self_ as *mut ()) };
    Py_DECREF__2!(tp);
}

fn check_decoded(decoded: *mut _object) -> i32 {
    if decoded.is_null() {
        return 1i32.wrapping_neg();
    }
    if PyUnicode_Check!(decoded) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"decoder should return a string result, not '%.200s'".as_ptr(),
                (*Py_TYPE!(decoded)).tp_name,
            )
        };
        Py_DECREF!(decoded);
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyIncrementalNewlineDecoder_decode(
    myself: *mut _object,
    input: *mut _object,
    r#final: i32,
) -> *mut _object {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut c_1: u32 = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut c_2: u32 = unsafe { core::mem::zeroed() };
    let mut out: i64 = unsafe { core::mem::zeroed() };
    let mut r#in: i64 = unsafe { core::mem::zeroed() };
    let mut in_str: *const () = unsafe { core::mem::zeroed() };
    let mut kind: i32 = unsafe { core::mem::zeroed() };
    let mut translated: *mut () = unsafe { core::mem::zeroed() };
    let mut kind_1: i32 = unsafe { core::mem::zeroed() };
    let mut only_lf: i32 = unsafe { core::mem::zeroed() };
    let mut seennl: i32 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    // Record which newlines are read and do newline translation if desired,
    // all in one pass.
    let mut in_str_1: *const () = unsafe { core::mem::zeroed() };
    let mut modified: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v58: bool = unsafe { core::mem::zeroed() };
    let mut modified_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut self_: *mut nldecoder_object = unsafe { core::mem::zeroed() };
    let mut output_len: i64 = unsafe { core::mem::zeroed() };
    let mut output: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        self_ = nldecoder_object_CAST!(myself);
        CHECK_INITIALIZED_DECODER!(self_);
        // decode input (with the eventual \r from a previous pass)
        if (*self_).decoder != core::ptr::addr_of_mut!(_Py_NoneStruct) {
            output = PyObject_CallMethodObjArgs(
                (*self_).decoder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_decode
                        ._ascii
                        .ob_base
                ),
                input,
                if r#final != 0 {
                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct))
                } else {
                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct))
                },
                core::ptr::null_mut::<()>(),
            );
        } else {
            output = Py_NewRef!(input);
        }
        if check_decoded(output) < 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                output_len = PyUnicode_GET_LENGTH!(output);
                if ((*self_).__slate_bits_0.__get_pendingcr() as i32) != 0
                    && (r#final != 0 || output_len > 0)
                {
                    modified_1 = PyUnicode_New(
                        output_len.wrapping_add(1),
                        PyUnicode_MAX_CHAR_VALUE!(output),
                    );
                    if modified_1.is_null() {
                        break '__join_0;
                    } else {
                        // Prefix output with CR
                        let kind: i32 = PyUnicode_KIND!(modified_1);
                        let out: *mut i8 = PyUnicode_DATA!(modified_1) as *mut i8;
                        PyUnicode_WRITE__3!(kind, out, 0i32, 13i32);
                        core::ptr::copy_nonoverlapping(
                            ((PyUnicode_DATA!(output) as *const ()) as *const core::ffi::c_void)
                                as *const u8,
                            ((out.wrapping_offset(kind as isize) as *mut ())
                                as *mut core::ffi::c_void) as *mut u8,
                            (((kind as i64).wrapping_mul(output_len) as u64) as usize) as usize,
                        );
                        Py_SETREF!(output, modified_1);
                        (*self_).__slate_bits_0.__set_pendingcr(0u32);
                        output_len = output_len.wrapping_add(1);
                    }
                }
                // retain last \r even when not translating data:
                // then readline() is sure to get \r\n in one pass
                if r#final == 0 {
                    if output_len > 0 {
                        __v58 = PyUnicode_READ_CHAR!(output, output_len.wrapping_sub(1)) == 13;
                    } else {
                        __v58 = false;
                    }
                    if __v58 {
                        modified = PyUnicode_Substring(output, 0i64, output_len.wrapping_sub(1));
                        if modified.is_null() {
                            break '__join_0;
                        } else {
                            Py_SETREF!(output, modified);
                            (*self_).__slate_bits_0.__set_pendingcr(1u32);
                        }
                    }
                }
                seennl = (*self_).__slate_bits_0.__get_seennl() as i32;
                only_lf = 0;
                in_str_1 = PyUnicode_DATA!(output) as *const ();
                len = PyUnicode_GET_LENGTH!(output);
                kind_1 = PyUnicode_KIND!(output);
                if len == 0 {
                    return output;
                } else {
                    // If, up to now, newlines are consistently \n, do a quick check
                    // for the \r *byte* with the libc's optimized memchr.
                    if seennl == 2 || seennl == 0 {
                        only_lf = ((memchr(
                            in_str_1 as *const core::ffi::c_void,
                            13i32 as i32,
                            ((kind_1 as i64).wrapping_mul(len) as u64) as usize,
                        ) as *mut ()) as *const ())
                            .is_null() as i32;
                    }
                    if only_lf != 0 {
                        '__join_2: {
                            // If not already seen, quick scan for a possible "\n" character.
                            // (there's nothing else to be done, even when in translation mode)
                            if seennl == 0
                                && !((memchr(
                                    in_str_1 as *const core::ffi::c_void,
                                    10i32 as i32,
                                    ((kind_1 as i64).wrapping_mul(len) as u64) as usize,
                                ) as *mut ()) as *const ())
                                    .is_null()
                            {
                                if kind_1 == 1 {
                                    seennl |= 2;
                                } else {
                                    i = 0;
                                    loop {
                                        // Fast loop for non-control characters
                                        while PyUnicode_READ!(kind_1, in_str_1, i) > 10 {
                                            i = i.wrapping_add(1);
                                        }
                                        let __v8: i64 = i;
                                        i = __v8.wrapping_add(1);
                                        let c: u32 = PyUnicode_READ(
                                            _Py_STATIC_CAST!(i32, kind_1),
                                            _Py_STATIC_CAST__2!(in_str_1),
                                            __v8,
                                        );
                                        if c == 10 {
                                            break;
                                        } else {
                                            if i >= len {
                                                break '__join_2;
                                            }
                                        }
                                    }
                                    seennl |= 2;
                                }
                            }
                        }
                    // Finished: we have scanned for newlines, and none of them
                    // need translating
                    } else {
                        if ((*self_).__slate_bits_0.__get_translate() as i32) == 0 {
                            i_1 = 0;
                            // We have already seen all newline types, no need to scan again
                            if seennl != 1 | 2 | 4 {
                                loop {
                                    // Fast loop for non-control characters
                                    while PyUnicode_READ!(kind_1, in_str_1, i_1) > 13 {
                                        i_1 = i_1.wrapping_add(1);
                                    }
                                    let __v22: i64 = i_1;
                                    i_1 = __v22.wrapping_add(1);
                                    c_1 = PyUnicode_READ(
                                        _Py_STATIC_CAST!(i32, kind_1),
                                        _Py_STATIC_CAST__2!(in_str_1),
                                        __v22,
                                    );
                                    if c_1 == 10 {
                                        seennl |= 2;
                                    } else {
                                        if c_1 == 13 {
                                            if PyUnicode_READ!(kind_1, in_str_1, i_1) == 10 {
                                                seennl |= 4;
                                                i_1 = i_1.wrapping_add(1);
                                            } else {
                                                seennl |= 1;
                                            }
                                        }
                                    }
                                    if i_1 >= len {
                                        break;
                                    } else {
                                        if seennl == 1 | 2 | 4 {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            kind = PyUnicode_KIND!(output);
                            in_str = PyUnicode_DATA!(output) as *const ();
                            // XXX: Previous in-place translation here is disabled as
                            // resizing is not possible anymore
                            //
                            // We could try to optimize this so that we only do a copy
                            // when there is something to translate. On the other hand,
                            // we already know there is a \r byte, so chances are high
                            // that something needs to be done.
                            translated = PyMem_Malloc((kind as i64).wrapping_mul(len) as u64);
                            if translated.is_null() {
                                PyErr_NoMemory();
                                break '__join_0;
                            } else {
                                out = 0;
                                r#in = 0;
                                loop {
                                    // Fast loop for non-control characters
                                    loop {
                                        let __v43: i64 = r#in;
                                        r#in = __v43.wrapping_add(1);
                                        let __v41: u32 = PyUnicode_READ(
                                            _Py_STATIC_CAST!(i32, kind),
                                            _Py_STATIC_CAST__2!(in_str),
                                            __v43,
                                        );
                                        c_2 = __v41;
                                        if __v41 > 13 {
                                            let __v45: i64 = out;
                                            out = __v45.wrapping_add(1);
                                            PyUnicode_WRITE(
                                                _Py_STATIC_CAST!(i32, kind),
                                                _Py_CAST__2!(translated),
                                                __v45,
                                                _Py_STATIC_CAST!(u32, c_2),
                                            );
                                        } else {
                                            break;
                                        }
                                    }
                                    if c_2 == 10 {
                                        let __v40: i64 = out;
                                        out = __v40.wrapping_add(1);
                                        PyUnicode_WRITE(
                                            _Py_STATIC_CAST!(i32, kind),
                                            _Py_CAST__2!(translated),
                                            __v40,
                                            _Py_STATIC_CAST!(u32, c_2),
                                        );
                                        seennl |= 2;
                                    } else {
                                        if c_2 == 13 {
                                            if PyUnicode_READ!(kind, in_str, r#in) == 10 {
                                                r#in = r#in.wrapping_add(1);
                                                seennl |= 4;
                                            } else {
                                                seennl |= 1;
                                            }
                                            let __v30: i64 = out;
                                            out = __v30.wrapping_add(1);
                                            PyUnicode_WRITE(
                                                _Py_STATIC_CAST!(i32, kind),
                                                _Py_CAST__2!(translated),
                                                __v30,
                                                _Py_STATIC_CAST!(u32, 10i32),
                                            );
                                        } else {
                                            if r#in > len {
                                                break;
                                            } else {
                                                let __v28: i64 = out;
                                                out = __v28.wrapping_add(1);
                                                PyUnicode_WRITE(
                                                    _Py_STATIC_CAST!(i32, kind),
                                                    _Py_CAST__2!(translated),
                                                    __v28,
                                                    _Py_STATIC_CAST!(u32, c_2),
                                                );
                                            }
                                        }
                                    }
                                }
                                Py_DECREF!(output);
                                output =
                                    PyUnicode_FromKindAndData(kind, translated as *const (), out);
                                PyMem_Free(translated);
                                if output.is_null() {
                                    return core::ptr::null_mut::<_object>();
                                }
                            }
                        }
                    }
                    let __v1: u32 = (*self_).__slate_bits_0.__get_seennl();
                    (*self_)
                        .__slate_bits_0
                        .__set_seennl(((__v1 as i32) | seennl) as u32);
                    return output;
                }
            }
            Py_DECREF!(output);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// [clinic input]
// @critical_section
// _io.IncrementalNewlineDecoder.decode
//     input: object
//     final: bool = False
// [clinic start generated code]
/// [clinic end generated code: output=0d486755bb37a66e input=9475d16a73168504]
fn _io_IncrementalNewlineDecoder_decode_impl(
    self_: *mut nldecoder_object,
    input: *mut _object,
    r#final: i32,
) -> *mut _object {
    _PyIncrementalNewlineDecoder_decode(self_ as *mut _object, input, r#final)
}

// [clinic input]
// @critical_section
// _io.IncrementalNewlineDecoder.getstate
// [clinic start generated code]
/// [clinic end generated code: output=f0d2c9c136f4e0d0 input=dc3e1f27aa850f12]
fn _io_IncrementalNewlineDecoder_getstate_impl(self_: *mut nldecoder_object) -> *mut _object {
    let mut buffer: *mut _object = unsafe { core::mem::zeroed() };
    let mut flag: u64 = 0;
    CHECK_INITIALIZED_DECODER!(self_);
    if (unsafe { (*self_).decoder }) != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        let state: *mut _object = PyObject_CallMethodNoArgs(unsafe { (*self_).decoder }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_getstate
                    ._ascii
                    .ob_base
            )
        });
        if state.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if PyTuple_Check!(state) == 0 {
            unsafe { PyErr_SetString(PyExc_TypeError, c"illegal decoder state".as_ptr()) };
            Py_DECREF!(state);
            return core::ptr::null_mut::<_object>();
        }
        if (unsafe {
            PyArg_ParseTuple(
                state,
                c"OK;illegal decoder state".as_ptr(),
                core::ptr::addr_of_mut!(buffer),
                core::ptr::addr_of_mut!(flag),
            )
        }) == 0
        {
            Py_DECREF!(state);
            return core::ptr::null_mut::<_object>();
        }
        Py_INCREF!(buffer);
        Py_DECREF!(state);
    } else {
        buffer = unsafe { Py_GetConstant(8u32) };
        flag = 0;
    }
    flag <<= 1i32;
    if ((unsafe { (*self_).__slate_bits_0.__get_pendingcr() }) as i32) != 0 {
        flag |= 1;
    }
    unsafe { Py_BuildValue(c"NK".as_ptr(), buffer, flag) }
}

// [clinic input]
// @critical_section
// _io.IncrementalNewlineDecoder.setstate
//     state: object
//     /
// [clinic start generated code]
/// [clinic end generated code: output=09135cb6e78a1dc8 input=275fd3982d2b08cb]
fn _io_IncrementalNewlineDecoder_setstate_impl(
    self_: *mut nldecoder_object,
    state: *mut _object,
) -> *mut _object {
    let mut buffer: *mut _object = unsafe { core::mem::zeroed() };
    let mut flag: u64 = 0;
    CHECK_INITIALIZED_DECODER!(self_);
    if PyTuple_Check!(state) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"state argument must be a tuple".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe {
        PyArg_ParseTuple(
            state,
            c"OK;setstate(): illegal state argument".as_ptr(),
            core::ptr::addr_of_mut!(buffer),
            core::ptr::addr_of_mut!(flag),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        let __t0 = ((flag & 1) as i32) as u32;
        (*self_).__slate_bits_0.__set_pendingcr(__t0);
    }
    flag >>= 1i32;
    if (unsafe { (*self_).decoder }) != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return unsafe {
            _PyObject_CallMethod(
                (*self_).decoder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_setstate
                        ._ascii
                        .ob_base
                ),
                c"((OK))".as_ptr(),
                buffer,
                flag,
            )
        };
    } else {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
}

// [clinic input]
// @critical_section
// _io.IncrementalNewlineDecoder.reset
// [clinic start generated code]
/// [clinic end generated code: output=32fa40c7462aa8ff input=31bd8ae4e36cec83]
fn _io_IncrementalNewlineDecoder_reset_impl(self_: *mut nldecoder_object) -> *mut _object {
    CHECK_INITIALIZED_DECODER!(self_);
    unsafe {
        (*self_).__slate_bits_0.__set_seennl(0u32);
    }
    unsafe {
        (*self_).__slate_bits_0.__set_pendingcr(0u32);
    }
    if (unsafe { (*self_).decoder }) != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return PyObject_CallMethodNoArgs(unsafe { (*self_).decoder }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_reset
                    ._ascii
                    .ob_base
            )
        });
    } else {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
}

#[unsafe(link_section = ".text.slate_distinct.textio.incrementalnewlinedecoder_newlines_get")]
extern "C-unwind" fn incrementalnewlinedecoder_newlines_get(
    op: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let self_: *mut nldecoder_object = nldecoder_object_CAST!(op);
    CHECK_INITIALIZED_DECODER!(self_);
    match (unsafe { (*self_).__slate_bits_0.__get_seennl() }) as i32 {
        1 => {
            return unsafe { PyUnicode_FromString(c"\r".as_ptr()) };
        }
        2 => {
            return unsafe { PyUnicode_FromString(c"\n".as_ptr()) };
        }
        4 => {
            return unsafe { PyUnicode_FromString(c"\r\n".as_ptr()) };
        }
        3 => {
            return unsafe {
                Py_BuildValue(
                    c"ss".as_ptr(),
                    c"\r".as_ptr() as *mut i8,
                    c"\n".as_ptr() as *mut i8,
                )
            };
        }
        5 => {
            return unsafe {
                Py_BuildValue(
                    c"ss".as_ptr(),
                    c"\r".as_ptr() as *mut i8,
                    c"\r\n".as_ptr() as *mut i8,
                )
            };
        }
        6 => {
            return unsafe {
                Py_BuildValue(
                    c"ss".as_ptr(),
                    c"\n".as_ptr() as *mut i8,
                    c"\r\n".as_ptr() as *mut i8,
                )
            };
        }
        7 => {
            return unsafe {
                Py_BuildValue(
                    c"sss".as_ptr(),
                    c"\r".as_ptr() as *mut i8,
                    c"\n".as_ptr() as *mut i8,
                    c"\r\n".as_ptr() as *mut i8,
                )
            };
        }
        _ => {
            return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
        }
    }
    unsafe { core::mem::zeroed() }
}

// TextIOWrapper
#[repr(C)]
#[derive(Clone, Copy)]
struct textio {
    ob_base: _object,
    /// initialized?
    ok: i32,
    detached: i32,
    chunk_size: i64,
    buffer: *mut _object,
    encoding: *mut _object,
    encoder: *mut _object,
    decoder: *mut _object,
    readnl: *mut _object,
    errors: *mut _object,
    /// ASCII-encoded; NULL stands for \n
    writenl: *const i8,
    line_buffering: i8,
    write_through: i8,
    readuniversal: i8,
    readtranslate: i8,
    writetranslate: i8,
    seekable: i8,
    has_read1: i8,
    telling: i8,
    finalizing: i8,
    /// Specialized encoding func (see below)
    encodefunc: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    /// Whether or not it's the start of the stream
    encoding_start_of_stream: i8,
    /// Reads and writes are internally buffered in order to speed things up.
    /// However, any read will first flush the write buffer if itsn't empty.
    ///
    /// Please also note that text to be written is first encoded before being
    /// buffered. This is necessary so that encoding errors are immediately
    /// reported to the caller, but it unfortunately means that the
    /// IncrementalEncoder (whose encode() method is always written in Python)
    /// becomes a bottleneck for small writes.
    /// buffer for text returned from decoder
    decoded_chars: *mut _object,
    /// offset into _decoded_chars for read()
    decoded_chars_used: i64,
    /// data waiting to be written.
    ///
    /// ascii unicode, bytes, or list of them.
    pending_bytes: *mut _object,
    pending_bytes_count: i64,
    /// snapshot is either NULL, or a tuple (dec_flags, next_input) where
    /// dec_flags is the second (integer) item of the decoder state and
    /// next_input is the chunk of input bytes that comes next after the
    /// snapshot point.  We use this to reconstruct decoder states in tell().
    snapshot: *mut _object,
    /// Bytes-to-characters ratio for the current chunk. Serves as input for
    /// the heuristic in tell().
    b2cratio: f64,
    /// Cache raw object if it's a FileIO object
    raw: *mut _object,
    weakreflist: *mut _object,
    dict: *mut _object,
    state: *mut _io_state,
}

// A couple of specialized cases in order to bypass the slow incremental
// encoding methods for the most popular encodings.
#[unsafe(link_section = ".text.slate_distinct.textio.ascii_encode")]
extern "C-unwind" fn ascii_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_AsASCIIString(text, PyUnicode_AsUTF8((*self_).errors)) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf16be_encode")]
extern "C-unwind" fn utf16be_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_EncodeUTF16(text, PyUnicode_AsUTF8((*self_).errors), 1i32) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf16le_encode")]
extern "C-unwind" fn utf16le_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_EncodeUTF16(text, PyUnicode_AsUTF8((*self_).errors), 1i32.wrapping_neg()) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf16_encode")]
extern "C-unwind" fn utf16_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    if (unsafe { (*self_).encoding_start_of_stream }) == 0 {
        // Skip the BOM and use native byte ordering
        return utf16le_encode(op, text);
    }
    unsafe { _PyUnicode_EncodeUTF16(text, PyUnicode_AsUTF8((*self_).errors), 0i32) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf32be_encode")]
extern "C-unwind" fn utf32be_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_EncodeUTF32(text, PyUnicode_AsUTF8((*self_).errors), 1i32) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf32le_encode")]
extern "C-unwind" fn utf32le_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_EncodeUTF32(text, PyUnicode_AsUTF8((*self_).errors), 1i32.wrapping_neg()) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf32_encode")]
extern "C-unwind" fn utf32_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    if (unsafe { (*self_).encoding_start_of_stream }) == 0 {
        // Skip the BOM and use native byte ordering
        return utf32le_encode(op, text);
    }
    unsafe { _PyUnicode_EncodeUTF32(text, PyUnicode_AsUTF8((*self_).errors), 0i32) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.utf8_encode")]
extern "C-unwind" fn utf8_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_AsUTF8String(text, PyUnicode_AsUTF8((*self_).errors)) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.latin1_encode")]
extern "C-unwind" fn latin1_encode(op: *mut _object, text: *mut _object) -> *mut _object {
    let self_: *mut textio = textio_CAST!(op);
    unsafe { _PyUnicode_AsLatin1String(text, PyUnicode_AsUTF8((*self_).errors)) }
}

/// Return true when encoding can be skipped when text is ascii.
fn is_asciicompat_encoding(
    f: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
) -> i32 {
    (f == Some(ascii_encode) || f == Some(latin1_encode) || f == Some(utf8_encode)) as i32
}

// Map normalized encoding names onto the specialized encoding funcs
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_textio_c_829 {
    name: *const i8,
    encodefunc: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
}

static encodefuncs: __SlateAlign16<[__SlateRecord_textio_c_829; 10]> = __SlateAlign16([
    __SlateRecord_textio_c_829 {
        name: (b"ascii\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(ascii_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"iso8859-1\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(latin1_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-8\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf8_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-16-be\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf16be_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-16-le\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf16le_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-16\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf16_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-32-be\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf32be_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-32-le\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf32le_encode),
    },
    __SlateRecord_textio_c_829 {
        name: (b"utf-32\0".as_ptr() as *mut i8) as *const i8,
        encodefunc: Some(utf32_encode),
    },
    __SlateRecord_textio_c_829 {
        name: core::ptr::null::<i8>(),
        encodefunc: None,
    },
]);

fn validate_newline(newline: *const i8) -> i32 {
    if !newline.is_null()
        && ((unsafe { *newline.wrapping_offset(0 as isize) }) as i32) != 0
        && !(((unsafe { *newline.wrapping_offset(0 as isize) }) as i32) == 10
            && ((unsafe { *newline.wrapping_offset(1 as isize) }) as i32) == 0)
        && !(((unsafe { *newline.wrapping_offset(0 as isize) }) as i32) == 13
            && ((unsafe { *newline.wrapping_offset(1 as isize) }) as i32) == 0)
        && !(((unsafe { *newline.wrapping_offset(0 as isize) }) as i32) == 13
            && ((unsafe { *newline.wrapping_offset(1 as isize) }) as i32) == 10
            && ((unsafe { *newline.wrapping_offset(2 as isize) }) as i32) == 0)
    {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"illegal newline value: %s".as_ptr(),
                newline,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

fn set_newline(self_: *mut textio, newline: *const i8) -> i32 {
    let old: *mut _object = unsafe { (*self_).readnl };
    if newline.is_null() {
        unsafe {
            (*self_).readnl = core::ptr::null_mut::<_object>();
        }
    } else {
        unsafe {
            (*self_).readnl = PyUnicode_FromString(newline);
        }
        if unsafe { (*self_).readnl }.is_null() {
            unsafe {
                (*self_).readnl = old;
            }
            return 1i32.wrapping_neg();
        }
    }
    unsafe {
        (*self_).readuniversal =
            (newline.is_null() || (*newline.wrapping_offset(0 as isize) as i32) == 0) as i8;
    }
    unsafe {
        (*self_).readtranslate = newline.is_null() as i8;
    }
    unsafe {
        (*self_).writetranslate =
            (newline.is_null() || (*newline.wrapping_offset(0 as isize) as i32) != 0) as i8;
    }
    if (unsafe { (*self_).readuniversal }) == 0 && !unsafe { (*self_).readnl }.is_null() {
        // validate_newline() accepts only ASCII newlines.
        unsafe {
            (*self_).writenl = PyUnicode_1BYTE_DATA!(unsafe { (*self_).readnl }) as *const i8;
        }
        if (unsafe {
            strcmp(
                (*self_).writenl as *const core::ffi::c_char,
                c"\n".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            unsafe {
                (*self_).writenl = core::ptr::null::<i8>();
            }
        }
    } else {
        unsafe {
            (*self_).writenl = core::ptr::null::<i8>();
        }
    }
    Py_XDECREF!(old);
    0
}

fn _textiowrapper_set_decoder(
    self_: *mut textio,
    codec_info: *mut _object,
    errors: *const i8,
) -> i32 {
    let mut r: i32 = 0;
    let res: *mut _object = PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_readable
                ._ascii
                .ob_base
        )
    });
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    r = unsafe { PyObject_IsTrue(res) };
    Py_DECREF!(res);
    if r == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    if r != 1 {
        return 0;
    }
    Py_CLEAR!((*self_).decoder);
    unsafe {
        (*self_).decoder = _PyCodecInfo_GetIncrementalDecoder(codec_info, errors);
    }
    if unsafe { (*self_).decoder }.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*self_).readuniversal }) != 0 {
        let state: *mut _io_state = unsafe { (*self_).state };
        let incrementalDecoder: *mut _object = unsafe {
            PyObject_CallFunctionObjArgs(
                (*state).PyIncrementalNewlineDecoder_Type as *mut _object,
                (*self_).decoder,
                if (*self_).readtranslate != 0 {
                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct))
                } else {
                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct))
                },
                core::ptr::null_mut::<()>(),
            )
        };
        if incrementalDecoder.is_null() {
            return 1i32.wrapping_neg();
        }
        Py_XSETREF!((*self_).decoder, incrementalDecoder);
    }
    0
}

fn _textiowrapper_decode(
    state: *mut _io_state,
    decoder: *mut _object,
    bytes: *mut _object,
    eof: i32,
) -> *mut _object {
    let mut chars: *mut _object = unsafe { core::mem::zeroed() };
    if Py_IS_TYPE!(decoder, unsafe {
        (*state).PyIncrementalNewlineDecoder_Type
    }) != 0
    {
        chars = _PyIncrementalNewlineDecoder_decode(decoder, bytes, eof);
    } else {
        chars = unsafe {
            PyObject_CallMethodObjArgs(
                decoder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_decode
                        ._ascii
                        .ob_base
                ),
                bytes,
                if eof != 0 {
                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct))
                } else {
                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct))
                },
                core::ptr::null_mut::<()>(),
            )
        };
    }
    if check_decoded(chars) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    // check_decoded already decreases refcount
    chars
}

fn _textiowrapper_set_encoder(
    self_: *mut textio,
    codec_info: *mut _object,
    errors: *const i8,
) -> i32 {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut r: i32 = 0;
    res = PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_writable
                ._ascii
                .ob_base
        )
    });
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    r = unsafe { PyObject_IsTrue(res) };
    Py_DECREF!(res);
    if r == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    if r != 1 {
        return 0;
    }
    Py_CLEAR!((*self_).encoder);
    unsafe {
        (*self_).encodefunc = None;
    }
    unsafe {
        (*self_).encoder = _PyCodecInfo_GetIncrementalEncoder(codec_info, errors);
    }
    if unsafe { (*self_).encoder }.is_null() {
        return 1i32.wrapping_neg();
    }
    // Get the normalized named of the codec
    if (unsafe {
        PyObject_GetOptionalAttr(
            codec_info,
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
            core::ptr::addr_of_mut!(res),
        )
    }) < 0
    {
        return 1i32.wrapping_neg();
    }
    let __v1: bool = !res.is_null() && PyUnicode_Check!(res) != 0;
    if __v1 {
        let mut e: *const __SlateRecord_textio_c_829 =
            unsafe { core::ptr::addr_of!(encodefuncs.0) as *const __SlateRecord_textio_c_829 };
        while !unsafe { (*e).name }.is_null() {
            if (unsafe { _PyUnicode_EqualToASCIIString(res, (*e).name) }) != 0 {
                unsafe {
                    (*self_).encodefunc = (*e).encodefunc;
                }
                break;
            }
            let __v3: *const __SlateRecord_textio_c_829 = e;
            e = __v3.wrapping_offset(1 as isize);
        }
    }
    Py_XDECREF!(res);
    0
}

fn _textiowrapper_fix_encoder_state(self_: *mut textio) -> i32 {
    if (unsafe { (*self_).seekable }) == 0 || unsafe { (*self_).encoder }.is_null() {
        return 0;
    }
    unsafe {
        (*self_).encoding_start_of_stream = 1;
    }
    let cookieObj: *mut _object = PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_tell
                ._ascii
                .ob_base
        )
    });
    if cookieObj.is_null() {
        return 1i32.wrapping_neg();
    }
    let cmp: i32 = unsafe { PyObject_RichCompareBool(cookieObj, _PyLong_GetZero(), 2i32) };
    Py_DECREF!(cookieObj);
    if cmp < 0 {
        return 1i32.wrapping_neg();
    }
    if cmp == 0 {
        unsafe {
            (*self_).encoding_start_of_stream = 0;
        }
        let res: *mut _object = PyObject_CallMethodOneArg(
            unsafe { (*self_).encoder },
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_setstate
                        ._ascii
                        .ob_base
                )
            },
            _PyLong_GetZero(),
        );
        if res.is_null() {
            return 1i32.wrapping_neg();
        }
        Py_DECREF!(res);
    }
    0
}

fn io_check_errors(errors: *mut _object) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    // In release mode, only check in development mode (-X dev)
    if (unsafe { (*_PyInterpreterState_GetConfig(interp)).dev_mode }) == 0 {
        return 0;
    }
    // Avoid calling PyCodec_LookupError() before the codec registry is ready:
    // before_PyUnicode_InitEncodings() is called.
    if unsafe { (*interp).unicode.fs_codec.encoding }.is_null() {
        return 0;
    }
    let name: *const i8 = unsafe { _PyUnicode_AsUTF8NoNUL(errors) };
    if name.is_null() {
        return 1i32.wrapping_neg();
    }
    let handler: *mut _object = unsafe { PyCodec_LookupError(name) };
    if !handler.is_null() {
        Py_DECREF!(handler);
        return 0;
    }
    1i32.wrapping_neg()
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.__init__
//     buffer: object
//     encoding: str(accept={str, NoneType}) = None
//     errors: object = None
//     newline: str(accept={str, NoneType}) = None
//     line_buffering: bool = False
//     write_through: bool = False
//
// Character and line based layer over a BufferedIOBase object, buffer.
//
// encoding gives the name of the encoding that the stream will be
// decoded or encoded with. It defaults to locale.getencoding().
//
// errors determines the strictness of encoding and decoding (see
// help(codecs.Codec) or the documentation for codecs.register) and
// defaults to "strict".
//
// newline controls how line endings are handled. It can be None, '',
// '\n', '\r', and '\r\n'.  It works as follows:
//
// * On input, if newline is None, universal newlines mode is
//   enabled. Lines in the input can end in '\n', '\r', or '\r\n', and
//   these are translated into '\n' before being returned to the
//   caller. If it is '', universal newline mode is enabled, but line
//   endings are returned to the caller untranslated. If it has any of
//   the other legal values, input lines are only terminated by the given
//   string, and the line ending is returned to the caller untranslated.
//
// * On output, if newline is None, any '\n' characters written are
//   translated to the system default line separator, os.linesep. If
//   newline is '' or '\n', no translation takes place. If newline is any
//   of the other legal values, any '\n' characters written are translated
//   to the given string.
//
// If line_buffering is True, a call to flush is implied when a call to
// write contains a newline character.
// [clinic start generated code]
/// [clinic end generated code: output=72267c0c01032ed2 input=0f077220214c40a4]
fn _io_TextIOWrapper___init___impl(
    self_: *mut textio,
    buffer: *mut _object,
    mut encoding: *const i8,
    mut errors: *mut _object,
    newline: *const i8,
    line_buffering: i32,
    write_through: i32,
) -> i32 {
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    // Build the decoder object
    let mut state: *mut _io_state = unsafe { core::mem::zeroed() };
    let mut errors_str: *const i8 = unsafe { core::mem::zeroed() };
    let mut r: i32 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut codec_info: *mut _object = unsafe { core::mem::zeroed() };
    let mut raw: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        codec_info = core::ptr::null_mut::<_object>();
        (*self_).ok = 0;
        (*self_).detached = 0;
        if encoding.is_null() {
            let interp: *mut _is = _PyInterpreterState_GET();
            if (*_PyInterpreterState_GetConfig(interp)).warn_default_encoding != 0 {
                if PyErr_WarnEx(
                    PyExc_EncodingWarning,
                    c"'encoding' argument not specified".as_ptr(),
                    1i64,
                ) != 0
                {
                    return 1i32.wrapping_neg();
                }
            }
        }
        if errors == core::ptr::addr_of_mut!(_Py_NoneStruct) {
            errors = core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_strict
                    ._ascii
                    .ob_base
            );
        } else {
            if PyUnicode_Check!(errors) == 0 {
                // Check 'errors' argument here because Argument Clinic doesn't support
                //
                // 'str(accept={str, NoneType})' converter.
                PyErr_Format(
                    PyExc_TypeError,
                    c"TextIOWrapper() argument 'errors' must be str or None, not %.50s".as_ptr(),
                    (*Py_TYPE!(errors)).tp_name,
                );
                return 1i32.wrapping_neg();
            } else {
                if io_check_errors(errors) != 0 {
                    return 1i32.wrapping_neg();
                }
            }
        }
        errors_str = _PyUnicode_AsUTF8NoNUL(errors);
        if errors_str.is_null() {
            return 1i32.wrapping_neg();
        } else {
            if validate_newline(newline) < 0 {
                return 1i32.wrapping_neg();
            } else {
                '__join_0: {
                    Py_CLEAR!((*self_).buffer);
                    Py_CLEAR!((*self_).encoding);
                    Py_CLEAR!((*self_).encoder);
                    Py_CLEAR!((*self_).decoder);
                    Py_CLEAR!((*self_).readnl);
                    Py_CLEAR!((*self_).decoded_chars);
                    Py_CLEAR!((*self_).pending_bytes);
                    Py_CLEAR!((*self_).snapshot);
                    Py_CLEAR!((*self_).errors);
                    Py_CLEAR!((*self_).raw);
                    (*self_).decoded_chars_used = 0;
                    (*self_).pending_bytes_count = 0;
                    (*self_).encodefunc = None;
                    (*self_).b2cratio = 0.0f64;
                    if encoding.is_null() && _PyRuntime.preconfig.utf8_mode != 0 {
                        (*self_).encoding = core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .literals
                                ._py_utf_8
                                ._ascii
                                .ob_base
                        );
                    } else {
                        if encoding.is_null()
                            || (strcmp(
                                encoding as *const core::ffi::c_char,
                                c"locale".as_ptr() as *const core::ffi::c_char,
                            ) as i32)
                                == 0
                        {
                            (*self_).encoding = _Py_GetLocaleEncodingObject();
                            if unsafe { (*self_).encoding }.is_null() {
                                break '__join_0;
                            }
                        }
                    }
                    if !unsafe { (*self_).encoding }.is_null() {
                        encoding = PyUnicode_AsUTF8((*self_).encoding);
                        if encoding.is_null() {
                            break '__join_0;
                        }
                    } else {
                        if !encoding.is_null() {
                            (*self_).encoding = PyUnicode_FromString(encoding);
                            if unsafe { (*self_).encoding }.is_null() {
                                break '__join_0;
                            }
                        } else {
                            PyErr_SetString(
                                PyExc_OSError,
                                c"could not determine default encoding".as_ptr(),
                            );
                            break '__join_0;
                        }
                    }
                    // Check we have been asked for a real text encoding
                    codec_info = _PyCodec_LookupTextEncoding(encoding, core::ptr::null::<i8>());
                    if codec_info.is_null() {
                        Py_CLEAR!((*self_).encoding);
                    } else {
                        // XXX: Failures beyond this point have the potential to leak elements
                        // of the partially constructed object (like self->encoding)
                        (*self_).errors = Py_NewRef!(errors);
                        (*self_).chunk_size = 8192;
                        (*self_).line_buffering = line_buffering as i8;
                        (*self_).write_through = write_through as i8;
                        if set_newline(self_, newline) >= 0 {
                            (*self_).buffer = Py_NewRef!(buffer);
                            state = find_io_state_by_def(Py_TYPE__2!(self_));
                            (*self_).state = state;
                            if _textiowrapper_set_decoder(self_, codec_info, errors_str) == 0 {
                                // Build the encoder object
                                if _textiowrapper_set_encoder(self_, codec_info, errors_str) == 0 {
                                    // Finished sorting out the codec details
                                    Py_CLEAR__2!(codec_info);
                                    if Py_IS_TYPE!(buffer, unsafe {
                                        (*state).PyBufferedReader_Type
                                    }) != 0
                                    {
                                        __v2 = true;
                                    } else {
                                        __v2 = Py_IS_TYPE!(buffer, unsafe {
                                            (*state).PyBufferedWriter_Type
                                        }) != 0;
                                    }
                                    if __v2 {
                                        __v1 = true;
                                    } else {
                                        __v1 = Py_IS_TYPE!(buffer, unsafe {
                                            (*state).PyBufferedRandom_Type
                                        }) != 0;
                                    }
                                    if __v1 {
                                        if PyObject_GetOptionalAttr(
                                            buffer,
                                            core::ptr::addr_of_mut!(
                                                _PyRuntime
                                                    .static_objects
                                                    .singletons
                                                    .strings
                                                    .identifiers
                                                    ._py_raw
                                                    ._ascii
                                                    .ob_base
                                            ),
                                            core::ptr::addr_of_mut!(raw),
                                        ) < 0
                                        {
                                            break '__join_0;
                                        } else {
                                            // Cache the raw FileIO object to speed up 'closed' checks
                                            if !raw.is_null() {
                                                if Py_IS_TYPE!(raw, unsafe {
                                                    (*state).PyFileIO_Type
                                                }) != 0
                                                {
                                                    (*self_).raw = raw;
                                                } else {
                                                    Py_DECREF!(raw);
                                                }
                                            }
                                        }
                                    }
                                    res = PyObject_CallMethodNoArgs(
                                        buffer,
                                        core::ptr::addr_of_mut!(
                                            _PyRuntime
                                                .static_objects
                                                .singletons
                                                .strings
                                                .identifiers
                                                ._py_seekable
                                                ._ascii
                                                .ob_base
                                        ),
                                    );
                                    if !res.is_null() {
                                        r = PyObject_IsTrue(res);
                                        Py_DECREF!(res);
                                        if r >= 0 {
                                            let __v0: i8 = r as i8;
                                            (*self_).telling = __v0;
                                            (*self_).seekable = __v0;
                                            r = PyObject_HasAttrWithError(
                                                buffer,
                                                core::ptr::addr_of_mut!(
                                                    _PyRuntime
                                                        .static_objects
                                                        .singletons
                                                        .strings
                                                        .identifiers
                                                        ._py_read1
                                                        ._ascii
                                                        .ob_base
                                                ),
                                            );
                                            if r >= 0 {
                                                (*self_).has_read1 = r as i8;
                                                (*self_).encoding_start_of_stream = 0;
                                                if _textiowrapper_fix_encoder_state(self_) >= 0 {
                                                    (*self_).ok = 1;
                                                    return 0;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Py_XDECREF!(codec_info);
                return 1i32.wrapping_neg();
            }
        }
    }
}

/// Return *default_value* if ob is None, 0 if ob is false, 1 if ob is true,
/// -1 on error.
fn convert_optional_bool(obj: *mut _object, default_value: i32) -> i32 {
    let mut v: i64 = 0;
    if obj == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        v = default_value as i64;
    } else {
        v = unsafe { PyLong_AsLong(obj) };
        let __v0: bool =
            v == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return 1i32.wrapping_neg();
        }
    }
    (v != 0) as i32
}

fn textiowrapper_change_encoding(
    self_: *mut textio,
    mut encoding: *mut _object,
    mut errors: *mut _object,
    newline_changed: i32,
) -> i32 {
    // Use existing settings where new settings are not specified
    if encoding == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        && errors == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        && newline_changed == 0
    {
        return 0; // no change
    }
    if encoding == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        encoding = unsafe { (*self_).encoding };
        if errors == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            errors = unsafe { (*self_).errors };
        }
        Py_INCREF!(encoding);
    } else {
        if (unsafe { _PyUnicode_EqualToASCIIString(encoding, c"locale".as_ptr()) }) != 0 {
            encoding = unsafe { _Py_GetLocaleEncodingObject() };
            if encoding.is_null() {
                return 1i32.wrapping_neg();
            }
        } else {
            Py_INCREF!(encoding);
        }
        if errors == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            errors = unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_strict
                        ._ascii
                        .ob_base
                )
            };
        }
    }
    Py_INCREF!(errors);
    let c_encoding: *const i8 = unsafe { PyUnicode_AsUTF8(encoding) };
    if c_encoding.is_null() {
        Py_DECREF!(encoding);
        Py_DECREF!(errors);
        return 1i32.wrapping_neg();
    }
    let c_errors: *const i8 = unsafe { PyUnicode_AsUTF8(errors) };
    if c_errors.is_null() {
        Py_DECREF!(encoding);
        Py_DECREF!(errors);
        return 1i32.wrapping_neg();
    }
    // Create new encoder & decoder
    let codec_info: *mut _object =
        unsafe { _PyCodec_LookupTextEncoding(c_encoding, core::ptr::null::<i8>()) };
    if codec_info.is_null() {
        Py_DECREF!(encoding);
        Py_DECREF!(errors);
        return 1i32.wrapping_neg();
    }
    let __v0: bool = _textiowrapper_set_decoder(self_, codec_info, c_errors) != 0
        || _textiowrapper_set_encoder(self_, codec_info, c_errors) != 0;
    if __v0 {
        Py_DECREF!(codec_info);
        Py_DECREF!(encoding);
        Py_DECREF!(errors);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(codec_info);
    Py_SETREF__2!((*self_).encoding, encoding);
    Py_SETREF__2!((*self_).errors, errors);
    _textiowrapper_fix_encoder_state(self_)
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.reconfigure
//     *
//     encoding: object = None
//     errors: object = None
//     newline as newline_obj: object(c_default="NULL") = None
//     line_buffering as line_buffering_obj: object = None
//     write_through as write_through_obj: object = None
//
// Reconfigure the text stream with new parameters.
//
// This also does an implicit stream flush.
//
// [clinic start generated code]
/// [clinic end generated code: output=52b812ff4b3d4b0f input=dc3bd35ebda702a7]
fn _io_TextIOWrapper_reconfigure_impl(
    self_: *mut textio,
    encoding: *mut _object,
    errors: *mut _object,
    newline_obj: *mut _object,
    line_buffering_obj: *mut _object,
    write_through_obj: *mut _object,
) -> *mut _object {
    let mut line_buffering: i32 = 0;
    let mut write_through: i32 = 0;
    let mut newline: *const i8 = core::ptr::null::<i8>();
    let __v0: bool = encoding != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        && PyUnicode_Check!(encoding) == 0;
    if __v0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"reconfigure() argument 'encoding' must be str or None, not %s".as_ptr(),
                (*Py_TYPE!(encoding)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let __v1: bool = errors != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        && PyUnicode_Check!(errors) == 0;
    if __v1 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"reconfigure() argument 'errors' must be str or None, not %s".as_ptr(),
                (*Py_TYPE!(errors)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let __v2: bool = !newline_obj.is_null()
        && newline_obj != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        && PyUnicode_Check!(newline_obj) == 0;
    if __v2 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"reconfigure() argument 'newline' must be str or None, not %s".as_ptr(),
                (*Py_TYPE!(newline_obj)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // Check if something is in the read buffer
    if !unsafe { (*self_).decoded_chars }.is_null() {
        if encoding != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
            || errors != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
            || !newline_obj.is_null()
        {
            _unsupported(
                unsafe { (*self_).state },
                c"It is not possible to set the encoding or newline of stream after the first read"
                    .as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        }
    }
    if !newline_obj.is_null() && newline_obj != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        newline = unsafe { PyUnicode_AsUTF8(newline_obj) };
        let __v3: bool = newline.is_null() || validate_newline(newline) < 0;
        if __v3 {
            return core::ptr::null_mut::<_object>();
        }
    }
    line_buffering = convert_optional_bool(
        line_buffering_obj,
        (unsafe { (*self_).line_buffering }) as i32,
    );
    if line_buffering < 0 {
        return core::ptr::null_mut::<_object>();
    }
    write_through = convert_optional_bool(
        write_through_obj,
        (unsafe { (*self_).write_through }) as i32,
    );
    if write_through < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { _PyFile_Flush(self_ as *mut _object) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).b2cratio = 0i32 as f64;
    }
    let __v4: bool = !newline_obj.is_null() && set_newline(self_, newline) < 0;
    if __v4 {
        return core::ptr::null_mut::<_object>();
    }
    if textiowrapper_change_encoding(self_, encoding, errors, !newline_obj.is_null() as i32) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).line_buffering = line_buffering as i8;
    }
    unsafe {
        (*self_).write_through = write_through as i8;
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(link_section = ".text.slate_distinct.textio.textiowrapper_clear")]
extern "C-unwind" fn textiowrapper_clear(op: *mut _object) -> i32 {
    let self_: *mut textio = textio_CAST!(op);
    unsafe {
        (*self_).ok = 0;
    }
    Py_CLEAR!((*self_).buffer);
    Py_CLEAR!((*self_).encoding);
    Py_CLEAR!((*self_).encoder);
    Py_CLEAR!((*self_).decoder);
    Py_CLEAR!((*self_).readnl);
    Py_CLEAR!((*self_).decoded_chars);
    Py_CLEAR!((*self_).pending_bytes);
    Py_CLEAR!((*self_).snapshot);
    Py_CLEAR!((*self_).errors);
    Py_CLEAR!((*self_).raw);
    Py_CLEAR!((*self_).dict);
    0
}

#[unsafe(link_section = ".text.slate_distinct.textio.textiowrapper_dealloc")]
extern "C-unwind" fn textiowrapper_dealloc(op: *mut _object) {
    let self_: *mut textio = textio_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(self_);
    unsafe {
        (*self_).finalizing = 1;
    }
    if (unsafe { _PyIOBase_finalize(op) }) < 0 {
        return;
    }
    unsafe {
        (*self_).ok = 0;
    }
    _PyObject_GC_UNTRACK__2!(self_);
    FT_CLEAR_WEAKREFS!(op, unsafe { (*self_).weakreflist });
    textiowrapper_clear(op);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(self_ as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.textio.textiowrapper_traverse")]
extern "C-unwind" fn textiowrapper_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut textio = textio_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(self_ as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*self_).buffer }, visit, arg);
    Py_VISIT!(unsafe { (*self_).encoding }, visit, arg);
    Py_VISIT!(unsafe { (*self_).encoder }, visit, arg);
    Py_VISIT!(unsafe { (*self_).decoder }, visit, arg);
    Py_VISIT!(unsafe { (*self_).readnl }, visit, arg);
    Py_VISIT!(unsafe { (*self_).decoded_chars }, visit, arg);
    Py_VISIT!(unsafe { (*self_).pending_bytes }, visit, arg);
    Py_VISIT!(unsafe { (*self_).snapshot }, visit, arg);
    Py_VISIT!(unsafe { (*self_).errors }, visit, arg);
    Py_VISIT!(unsafe { (*self_).raw }, visit, arg);
    Py_VISIT!(unsafe { (*self_).dict }, visit, arg);
    0
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.detach
// [clinic start generated code]
/// [clinic end generated code: output=7ba3715cd032d5f2 input=c908a3b4ef203b0f]
fn _io_TextIOWrapper_detach_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    if (unsafe { _PyFile_Flush(self_ as *mut _object) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let buffer: *mut _object = unsafe { (*self_).buffer };
    unsafe {
        (*self_).buffer = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).detached = 1;
    }
    buffer
}

/// Flush the internal write buffer. This doesn't explicitly flush the
/// underlying buffered object, though.
fn _textiowrapper_writeflush(self_: *mut textio) -> i32 {
    if unsafe { (*self_).pending_bytes }.is_null() {
        return 0;
    }
    let pending: *mut _object = unsafe { (*self_).pending_bytes };
    let mut b: *mut _object = unsafe { core::mem::zeroed() };
    if PyBytes_Check!(pending) != 0 {
        b = Py_NewRef!(pending);
    } else {
        if PyUnicode_Check!(pending) != 0 {
            b = unsafe {
                PyBytes_FromStringAndSize(
                    PyUnicode_DATA!(pending) as *const i8,
                    PyUnicode_GET_LENGTH!(pending),
                )
            };
            if b.is_null() {
                return 1i32.wrapping_neg();
            }
        } else {
            b = unsafe {
                PyBytes_FromStringAndSize(core::ptr::null::<i8>(), (*self_).pending_bytes_count)
            };
            if b.is_null() {
                return 1i32.wrapping_neg();
            }
            let buf: *mut i8 = unsafe { PyBytes_AsString(b) };
            let mut pos: i64 = 0;
            let mut i: i64 = 0;
            while i < PyList_GET_SIZE!(pending) {
                let obj: *mut _object = PyList_GET_ITEM!(pending, i);
                let mut src: *mut i8 = unsafe { core::mem::zeroed() };
                let mut len: i64 = 0;
                if PyUnicode_Check!(obj) != 0 {
                    src = PyUnicode_DATA!(obj) as *mut i8;
                    len = PyUnicode_GET_LENGTH!(obj);
                } else {
                    if (unsafe {
                        PyBytes_AsStringAndSize(
                            obj,
                            core::ptr::addr_of_mut!(src),
                            core::ptr::addr_of_mut!(len),
                        )
                    }) < 0
                    {
                        Py_DECREF!(b);
                        return 1i32.wrapping_neg();
                    }
                }
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((src as *const ()) as *const core::ffi::c_void) as *const u8,
                        ((buf.wrapping_offset(pos as isize) as *mut ()) as *mut core::ffi::c_void)
                            as *mut u8,
                        ((len as u64) as usize) as usize,
                    )
                };
                pos = pos.wrapping_add(len);
                i = i.wrapping_add(1);
            }
        }
    }
    unsafe {
        (*self_).pending_bytes_count = 0;
    }
    unsafe {
        (*self_).pending_bytes = core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(pending);
    let mut ret: *mut _object = unsafe { core::mem::zeroed() };
    loop {
        ret = PyObject_CallMethodOneArg(
            unsafe { (*self_).buffer },
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
            b,
        );
        let __v6: bool = ret.is_null() && (unsafe { _PyIO_trap_eintr() }) != 0;
        if !__v6 {
            break;
        }
    }
    Py_DECREF!(b);
    // NOTE: We cleared buffer but we don't know how many bytes are actually written
    //
    // when an error occurred.
    if ret.is_null() {
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(ret);
    0
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.write
//     text: unicode
//     /
// [clinic start generated code]
/// [clinic end generated code: output=d2deb0d50771fcec input=73ec95c5c4a3489c]
fn _io_TextIOWrapper_write_impl(self_: *mut textio, mut text: *mut _object) -> *mut _object {
    let mut b: *mut _object = unsafe { core::mem::zeroed() };
    let mut textlen: i64 = 0;
    let mut haslf: i32 = 0;
    let mut needflush: i32 = 0;
    let mut text_needflush: i32 = 0;
    CHECK_ATTACHED!(self_);
    CHECK_CLOSED__4!(self_);
    if unsafe { (*self_).encoder }.is_null() {
        return _unsupported(unsafe { (*self_).state }, c"not writable".as_ptr());
    }
    Py_INCREF!(text);
    textlen = PyUnicode_GET_LENGTH!(text);
    if (unsafe { (*self_).writetranslate }) != 0 && !unsafe { (*self_).writenl }.is_null()
        || (unsafe { (*self_).line_buffering }) != 0
    {
        if (unsafe { PyUnicode_FindChar(text, 10u32, 0i64, PyUnicode_GET_LENGTH!(text), 1i32) })
            != (1i32.wrapping_neg() as i64)
        {
            haslf = 1;
        }
    }
    if haslf != 0
        && (unsafe { (*self_).writetranslate }) != 0
        && !unsafe { (*self_).writenl }.is_null()
    {
        let newtext: *mut _object = unsafe {
            _PyObject_CallMethod(
                text,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_replace
                        ._ascii
                        .ob_base
                ),
                c"ss".as_ptr(),
                c"\n".as_ptr() as *mut i8,
                (*self_).writenl,
            )
        };
        Py_DECREF!(text);
        if newtext.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        text = newtext;
    }
    if (unsafe { (*self_).write_through }) != 0 {
        text_needflush = 1;
    }
    let __v1: bool = (unsafe { (*self_).line_buffering }) != 0
        && (haslf != 0
            || (unsafe {
                PyUnicode_FindChar(text, 13u32, 0i64, PyUnicode_GET_LENGTH!(text), 1i32)
            }) != (1i32.wrapping_neg() as i64));
    if __v1 {
        needflush = 1;
    }
    // XXX What if we were just reading?
    if (unsafe { (*self_).encodefunc }) != None {
        let __v3: bool = PyUnicode_IS_ASCII!(text) != 0
            && PyUnicode_GET_LENGTH!(text) <= unsafe { (*self_).chunk_size };
        let __v4: bool = __v3 && is_asciicompat_encoding(unsafe { (*self_).encodefunc }) != 0;
        if __v4 {
            b = Py_NewRef!(text);
        } else {
            b = unsafe { unsafe { (*self_).encodefunc }.unwrap()(self_ as *mut _object, text) };
        }
        // See bpo-43260
        unsafe {
            (*self_).encoding_start_of_stream = 0;
        }
    } else {
        b = PyObject_CallMethodOneArg(
            unsafe { (*self_).encoder },
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_encode
                        ._ascii
                        .ob_base
                )
            },
            text,
        );
    }
    Py_DECREF!(text);
    if b.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let __v5: bool = b != text && PyBytes_Check!(b) == 0;
    if __v5 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"encoder should return a bytes object, not '%.200s'".as_ptr(),
                (*Py_TYPE!(b)).tp_name,
            )
        };
        Py_DECREF!(b);
        return core::ptr::null_mut::<_object>();
    }
    let bytes_len: i64 = if b == text {
        PyUnicode_GET_LENGTH!(b)
    } else {
        PyBytes_GET_SIZE!(b)
    };
    // We should avoid concatenating huge data.
    //
    // Flush the buffer before adding b to the buffer if b is not small.
    //
    // https://github.com/python/cpython/issues/87426
    if bytes_len >= unsafe { (*self_).chunk_size } {
        // _textiowrapper_writeflush() calls buffer.write().
        //
        // self->pending_bytes can be appended during buffer->write()
        //
        // or other thread.
        //
        // We need to loop until buffer becomes empty.
        //
        // https://github.com/python/cpython/issues/118138
        //
        // https://github.com/python/cpython/issues/119506
        while !unsafe { (*self_).pending_bytes }.is_null() {
            if _textiowrapper_writeflush(self_) < 0 {
                Py_DECREF!(b);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    if bytes_len > 0 {
        if unsafe { (*self_).pending_bytes }.is_null() {
            unsafe {
                (*self_).pending_bytes = b;
            }
        } else {
            if PyList_CheckExact!(unsafe { (*self_).pending_bytes }) == 0 {
                let list: *mut _object = unsafe { PyList_New(2i64) };
                if list.is_null() {
                    Py_DECREF!(b);
                    return core::ptr::null_mut::<_object>();
                }
                // Since Python 3.12, allocating GC object won't trigger GC and release
                //
                // GIL. See https://github.com/python/cpython/issues/97922
                PyList_SET_ITEM__2!(list, 0i32, unsafe { (*self_).pending_bytes });
                PyList_SET_ITEM__2!(list, 1i32, b);
                unsafe {
                    (*self_).pending_bytes = list;
                }
            } else {
                if (unsafe { PyList_Append((*self_).pending_bytes, b) }) < 0 {
                    Py_DECREF!(b);
                    return core::ptr::null_mut::<_object>();
                }
                Py_DECREF!(b);
            }
        }
        let __v8: i64 = unsafe { (*self_).pending_bytes_count };
        unsafe {
            (*self_).pending_bytes_count = __v8.wrapping_add(bytes_len);
        }
    } else {
        Py_DECREF!(b);
    }
    if (unsafe { (*self_).pending_bytes_count }) >= unsafe { (*self_).chunk_size }
        || needflush != 0
        || text_needflush != 0
    {
        if _textiowrapper_writeflush(self_) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    if needflush != 0 {
        if (unsafe { _PyFile_Flush((*self_).buffer) }) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    if !unsafe { (*self_).snapshot }.is_null() {
        textiowrapper_set_decoded_chars(self_, core::ptr::null_mut::<_object>());
        Py_CLEAR!((*self_).snapshot);
    }
    if !unsafe { (*self_).decoder }.is_null() {
        let ret: *mut _object = PyObject_CallMethodNoArgs(unsafe { (*self_).decoder }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_reset
                    ._ascii
                    .ob_base
            )
        });
        if ret.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        Py_DECREF!(ret);
    }
    unsafe { PyLong_FromSsize_t(textlen) }
}

/// Steal a reference to chars and store it in the decoded_char buffer;
fn textiowrapper_set_decoded_chars(self_: *mut textio, chars: *mut _object) {
    Py_XSETREF!((*self_).decoded_chars, chars);
    unsafe {
        (*self_).decoded_chars_used = 0;
    }
}

fn textiowrapper_get_decoded_chars(self_: *mut textio, mut n: i64) -> *mut _object {
    let mut chars: *mut _object = unsafe { core::mem::zeroed() };
    let mut avail: i64 = 0;
    if unsafe { (*self_).decoded_chars }.is_null() {
        return unsafe { Py_GetConstant(7u32) };
    }
    avail = PyUnicode_GET_LENGTH!(unsafe { (*self_).decoded_chars })
        .wrapping_sub(unsafe { (*self_).decoded_chars_used });
    if n < 0 || n > avail {
        n = avail;
    }
    if (unsafe { (*self_).decoded_chars_used }) > 0 || n < avail {
        chars = unsafe {
            PyUnicode_Substring(
                (*self_).decoded_chars,
                (*self_).decoded_chars_used,
                unsafe { (*self_).decoded_chars_used }.wrapping_add(n),
            )
        };
        if chars.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        chars = Py_NewRef!(unsafe { (*self_).decoded_chars });
    }
    let __v1: i64 = unsafe { (*self_).decoded_chars_used };
    unsafe {
        (*self_).decoded_chars_used = __v1.wrapping_add(n);
    }
    chars
}

/// Read and decode the next chunk of data from the BufferedReader.
fn textiowrapper_read_chunk(self_: *mut textio, mut size_hint: i64) -> i32 {
    let mut snapshot: *mut _object = unsafe { core::mem::zeroed() };
    // At the snapshot point, len(dec_buffer) bytes before the read, the
    // next input to be decoded is dec_buffer + input_chunk.
    let mut next_input: *mut _object = unsafe { core::mem::zeroed() };
    // To prepare for tell(), we need to snapshot a point in the file
    // where the decoder's input buffer is empty.
    let mut state: *mut _object = unsafe { core::mem::zeroed() };
    let mut eof: i32 = unsafe { core::mem::zeroed() };
    let mut nchars: i64 = unsafe { core::mem::zeroed() };
    let mut nbytes: i64 = unsafe { core::mem::zeroed() };
    let mut chunk_size: *mut _object = unsafe { core::mem::zeroed() };
    let mut decoded_chars: *mut _object = unsafe { core::mem::zeroed() };
    let mut input_chunk_buf: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(input_chunk_buf) as *mut __SlateRecord_pybuffer_h_33;
    let mut input_chunk: *mut _object = unsafe { core::mem::zeroed() };
    let mut dec_flags: *mut _object = unsafe { core::mem::zeroed() };
    let mut dec_buffer: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        dec_buffer = core::ptr::null_mut::<_object>();
        dec_flags = core::ptr::null_mut::<_object>();
        input_chunk = core::ptr::null_mut::<_object>();
        // The return value is True unless EOF was reached.  The decoded string is
        // placed in self._decoded_chars (replacing its previous value).  The
        // entire input chunk is sent to the decoder, though some of it may remain
        // buffered in the decoder, yet to be converted.
        if unsafe { (*self_).decoder }.is_null() {
            _unsupported((*self_).state, c"not readable".as_ptr());
            return 1i32.wrapping_neg();
        } else {
            if (*self_).telling != 0 {
                state = PyObject_CallMethodNoArgs(
                    (*self_).decoder,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_getstate
                            ._ascii
                            .ob_base
                    ),
                );
                if state.is_null() {
                    return 1i32.wrapping_neg();
                } else {
                    // Given this, we know there was a valid snapshot point
                    // len(dec_buffer) bytes ago with decoder state (b'', dec_flags).
                    if PyTuple_Check!(state) == 0 {
                        PyErr_SetString(PyExc_TypeError, c"illegal decoder state".as_ptr());
                        Py_DECREF!(state);
                        return 1i32.wrapping_neg();
                    } else {
                        if PyArg_ParseTuple(
                            state,
                            c"OO;illegal decoder state".as_ptr(),
                            core::ptr::addr_of_mut!(dec_buffer),
                            core::ptr::addr_of_mut!(dec_flags),
                        ) == 0
                        {
                            Py_DECREF!(state);
                            return 1i32.wrapping_neg();
                        } else {
                            if PyBytes_Check!(dec_buffer) == 0 {
                                PyErr_Format(PyExc_TypeError, c"illegal decoder state: the first item should be a bytes object, not '%.200s'".as_ptr(), (*Py_TYPE!(dec_buffer)).tp_name);
                                Py_DECREF!(state);
                                return 1i32.wrapping_neg();
                            } else {
                                Py_INCREF!(dec_buffer);
                                Py_INCREF!(dec_flags);
                                Py_DECREF!(state);
                            }
                        }
                    }
                }
            }
            // Read a chunk, decode it, and put the result in self._decoded_chars.
            if size_hint > 0 {
                size_hint =
                    (Py_MAX!(unsafe { (*self_).b2cratio }, 1.0f64) * (size_hint as f64)) as i64;
            }
            '__join_0: {
                chunk_size = PyLong_FromSsize_t(Py_MAX!(unsafe { (*self_).chunk_size }, size_hint));
                if !chunk_size.is_null() {
                    input_chunk = PyObject_CallMethodOneArg(
                        (*self_).buffer,
                        if (*self_).has_read1 != 0 {
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_read1
                                    ._ascii
                                    .ob_base
                            )
                        } else {
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_read
                                    ._ascii
                                    .ob_base
                            )
                        },
                        chunk_size,
                    );
                    Py_DECREF!(chunk_size);
                    if !input_chunk.is_null() {
                        if PyObject_GetBuffer(
                            input_chunk,
                            core::ptr::addr_of_mut!(*__slate_slot_8),
                            0i32,
                        ) != 0
                        {
                            PyErr_Format(PyExc_TypeError, c"underlying %s() should have returned a bytes-like object, not '%.200s'".as_ptr(), if (*self_).has_read1 != 0 { c"read1".as_ptr() as *mut i8 } else { c"read".as_ptr() as *mut i8 }, (*Py_TYPE!(input_chunk)).tp_name);
                        } else {
                            nbytes = (*__slate_slot_8).len;
                            eof = (nbytes == 0) as i32;
                            decoded_chars = _textiowrapper_decode(
                                (*self_).state,
                                (*self_).decoder,
                                input_chunk,
                                eof,
                            );
                            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_8));
                            if !decoded_chars.is_null() {
                                textiowrapper_set_decoded_chars(self_, decoded_chars);
                                nchars = PyUnicode_GET_LENGTH!(decoded_chars);
                                if nchars > 0 {
                                    (*self_).b2cratio = (nbytes as f64) / (nchars as f64);
                                } else {
                                    (*self_).b2cratio = 0.0f64;
                                }
                                if nchars > 0 {
                                    eof = 0;
                                }
                                if (*self_).telling != 0 {
                                    next_input = dec_buffer;
                                    PyBytes_Concat(
                                        core::ptr::addr_of_mut!(next_input),
                                        input_chunk,
                                    );
                                    dec_buffer = core::ptr::null_mut::<_object>(); // Reference lost to PyBytes_Concat
                                    if next_input.is_null() {
                                        break '__join_0;
                                    } else {
                                        snapshot =
                                            Py_BuildValue(c"NN".as_ptr(), dec_flags, next_input);
                                        if snapshot.is_null() {
                                            dec_flags = core::ptr::null_mut::<_object>();
                                            break '__join_0;
                                        } else {
                                            Py_XSETREF!((*self_).snapshot, snapshot);
                                        }
                                    }
                                }
                                Py_DECREF!(input_chunk);
                                return (eof == 0) as i32;
                            }
                        }
                    }
                }
            }
            Py_XDECREF!(dec_buffer);
            Py_XDECREF!(dec_flags);
            Py_XDECREF!(input_chunk);
            return 1i32.wrapping_neg();
        }
    }
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.read
//     size as n: Py_ssize_t(accept={int, NoneType}) = -1
//     /
// [clinic start generated code]
/// [clinic end generated code: output=7e651ce6cc6a25a6 input=67d14c5661121377]
fn _io_TextIOWrapper_read_impl(self_: *mut textio, n: i64) -> *mut _object {
    let mut decoded: *mut _object = unsafe { core::mem::zeroed() };
    // Read everything
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut remaining: i64 = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut chunks: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        chunks = core::ptr::null_mut::<_object>();
        CHECK_ATTACHED!(self_);
        CHECK_CLOSED__4!(self_);
        if unsafe { (*self_).decoder }.is_null() {
            return _unsupported((*self_).state, c"not readable".as_ptr());
        } else {
            if _textiowrapper_writeflush(self_) < 0 {
                return core::ptr::null_mut::<_object>();
            } else {
                '__join_0: {
                    if n < 0 {
                        bytes = PyObject_CallMethodNoArgs(
                            (*self_).buffer,
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_read
                                    ._ascii
                                    .ob_base
                            ),
                        );
                        if !bytes.is_null() {
                            if bytes == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                Py_DECREF!(bytes);
                                PyErr_SetString(
                                    PyExc_BlockingIOError,
                                    c"Read returned None.".as_ptr(),
                                );
                                return core::ptr::null_mut::<_object>();
                            } else {
                                let state: *mut _io_state = (*self_).state;
                                if Py_IS_TYPE!(unsafe { (*self_).decoder }, unsafe {
                                    (*state).PyIncrementalNewlineDecoder_Type
                                }) != 0
                                {
                                    decoded = _PyIncrementalNewlineDecoder_decode(
                                        (*self_).decoder,
                                        bytes,
                                        1i32,
                                    );
                                } else {
                                    decoded = PyObject_CallMethodObjArgs(
                                        (*self_).decoder,
                                        core::ptr::addr_of_mut!(
                                            _PyRuntime
                                                .static_objects
                                                .singletons
                                                .strings
                                                .identifiers
                                                ._py_decode
                                                ._ascii
                                                .ob_base
                                        ),
                                        bytes,
                                        _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
                                        core::ptr::null_mut::<()>(),
                                    );
                                }
                                Py_DECREF!(bytes);
                                if check_decoded(decoded) >= 0 {
                                    result = textiowrapper_get_decoded_chars(
                                        self_,
                                        1i32.wrapping_neg() as i64,
                                    );
                                    if result.is_null() {
                                        Py_DECREF!(decoded);
                                        return core::ptr::null_mut::<_object>();
                                    } else {
                                        PyUnicode_AppendAndDel(
                                            core::ptr::addr_of_mut!(result),
                                            decoded,
                                        );
                                        if !result.is_null() {
                                            if !unsafe { (*self_).snapshot }.is_null() {
                                                textiowrapper_set_decoded_chars(
                                                    self_,
                                                    core::ptr::null_mut::<_object>(),
                                                );
                                                Py_CLEAR!((*self_).snapshot);
                                            }
                                            return result;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        res = 1;
                        remaining = n;
                        result = textiowrapper_get_decoded_chars(self_, n);
                        if !result.is_null() {
                            let __v10: i64 = PyUnicode_GET_LENGTH!(result);
                            remaining = remaining.wrapping_sub(__v10);
                            // Keep reading chunks until we have n characters to return
                            while remaining > 0 {
                                res = textiowrapper_read_chunk(self_, remaining);
                                if res < 0 {
                                    // NOTE: PyErr_SetFromErrno() calls PyErr_CheckSignals()
                                    // when EINTR occurs so we needn't do it ourselves.
                                    if _PyIO_trap_eintr() == 0 {
                                        break '__join_0;
                                    }
                                } else {
                                    if res == 0 {
                                        break;
                                    } else {
                                        // EOF
                                        if chunks.is_null() {
                                            chunks = PyList_New(0i64);
                                            if chunks.is_null() {
                                                break '__join_0;
                                            }
                                        }
                                        if PyUnicode_GET_LENGTH!(result) > 0 {
                                            __v7 = PyList_Append(chunks, result) < 0;
                                        } else {
                                            __v7 = false;
                                        }
                                        if __v7 {
                                            break '__join_0;
                                        } else {
                                            Py_DECREF!(result);
                                            result =
                                                textiowrapper_get_decoded_chars(self_, remaining);
                                            if result.is_null() {
                                                break '__join_0;
                                            } else {
                                                let __v6: i64 = PyUnicode_GET_LENGTH!(result);
                                                remaining = remaining.wrapping_sub(__v6);
                                            }
                                        }
                                    }
                                }
                            }
                            if !chunks.is_null() {
                                if !result.is_null() {
                                    __v3 = PyList_Append(chunks, result) < 0;
                                } else {
                                    __v3 = false;
                                }
                                if __v3 {
                                    break '__join_0;
                                } else {
                                    Py_XSETREF__2!(
                                        result,
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
                                            chunks
                                        )
                                    );
                                    if result.is_null() {
                                        break '__join_0;
                                    } else {
                                        Py_CLEAR__2!(chunks);
                                    }
                                }
                            }
                            return result;
                        }
                    }
                }
                Py_XDECREF!(result);
                Py_XDECREF!(chunks);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

/// NOTE: `end` must point to the real end of the Py_UCS4 storage,
/// that is to the NUL character. Otherwise the function will produce
/// incorrect results.
fn find_control_char(kind: i32, mut s: *const i8, end: *const i8, ch: u32) -> *const i8 {
    if kind == 1 {
        return ((unsafe {
            memchr(
                (s as *const ()) as *const core::ffi::c_void,
                ((ch as i8) as i32) as i32,
                ((end.offset_from(s as *const i8) as i64) as u64) as usize,
            ) as *mut ()
        }) as *const ()) as *const i8;
    }
    loop {
        while PyUnicode_READ__3!(kind, s, 0i32) > ch {
            let __v0: *const i8 = s;
            s = __v0.wrapping_offset(kind as isize);
        }
        if PyUnicode_READ__3!(kind, s, 0i32) == ch {
            return s;
        }
        if s == end {
            return core::ptr::null::<i8>();
        }
        let __v3: *const i8 = s;
        s = __v3.wrapping_offset(kind as isize);
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyIO_find_line_ending(
    translated: i32,
    universal: i32,
    readnl: *mut _object,
    kind: i32,
    start: *const i8,
    end: *const i8,
    consumed: *mut i64,
) -> i64 {
    let len: i64 = ((unsafe { end.offset_from(start as *const i8) }) as i64) / (kind as i64);
    if translated != 0 {
        // Newlines are already translated, only search for \n
        let pos: *const i8 = find_control_char(kind, start, end, 10u32);
        if !pos.is_null() {
            return (((unsafe { pos.offset_from(start as *const i8) }) as i64) / (kind as i64))
                .wrapping_add(1);
        } else {
            unsafe {
                *consumed = len;
            }
            return 1i32.wrapping_neg() as i64;
        }
    } else {
        if universal != 0 {
            // Universal newline search. Find any of \r, \r\n, \n
            // The decoder ensures that \r\n are not split in two pieces
            let mut s: *const i8 = start;
            loop {
                let mut ch: u32 = 0;
                // Fast path for non-control chars. The loop always ends
                // since the Unicode string is NUL-terminated.
                while PyUnicode_READ__3!(kind, s, 0i32) > 13 {
                    let __v0: *const i8 = s;
                    s = __v0.wrapping_offset(kind as isize);
                }
                if s >= end {
                    unsafe {
                        *consumed = len;
                    }
                    return 1i32.wrapping_neg() as i64;
                }
                ch = PyUnicode_READ__3!(kind, s, 0i32);
                let __v3: *const i8 = s;
                s = __v3.wrapping_offset(kind as isize);
                if ch == 10 {
                    return ((unsafe { s.offset_from(start as *const i8) }) as i64) / (kind as i64);
                }
                if ch == 13 {
                    if PyUnicode_READ__3!(kind, s, 0i32) == 10 {
                        return (((unsafe { s.offset_from(start as *const i8) }) as i64)
                            / (kind as i64))
                            .wrapping_add(1);
                    } else {
                        return ((unsafe { s.offset_from(start as *const i8) }) as i64)
                            / (kind as i64);
                    }
                }
            }
        } else {
            // Non-universal mode.
            let readnl_len: i64 = PyUnicode_GET_LENGTH!(readnl);
            let nl: *const u8 = PyUnicode_1BYTE_DATA!(readnl) as *const u8;
            // Assume that readnl is an ASCII character.
            if readnl_len == 1 {
                let pos: *const i8 = find_control_char(
                    kind,
                    start,
                    end,
                    (unsafe { *nl.wrapping_offset(0 as isize) }) as u32,
                );
                if !pos.is_null() {
                    return (((unsafe { pos.offset_from(start as *const i8) }) as i64)
                        / (kind as i64))
                        .wrapping_add(1);
                }
                unsafe {
                    *consumed = len;
                }
                return 1i32.wrapping_neg() as i64;
            } else {
                let mut s: *const i8 = start;
                let mut e: *const i8 = end.wrapping_offset(
                    -(readnl_len.wrapping_sub(1).wrapping_mul(kind as i64) as isize),
                );
                if e < s {
                    e = s;
                }
                while s < e {
                    let mut i: i64 = 1;
                    let pos: *const i8 = find_control_char(
                        kind,
                        s,
                        end,
                        (unsafe { *nl.wrapping_offset(0 as isize) }) as u32,
                    );
                    if pos.is_null() || pos >= e {
                        break;
                    }
                    while i < readnl_len {
                        if PyUnicode_READ__4!(kind, pos, i)
                            != (((unsafe { *nl.wrapping_offset(i as isize) }) as i32) as u32)
                        {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    if i == readnl_len {
                        return (((unsafe { pos.offset_from(start as *const i8) }) as i64)
                            / (kind as i64))
                            .wrapping_add(readnl_len);
                    }
                    s = pos.wrapping_offset(kind as isize);
                }
                let pos: *const i8 = find_control_char(
                    kind,
                    e,
                    end,
                    (unsafe { *nl.wrapping_offset(0 as isize) }) as u32,
                );
                if pos.is_null() {
                    unsafe {
                        *consumed = len;
                    }
                } else {
                    unsafe {
                        *consumed = (pos.offset_from(start as *const i8) as i64) / (kind as i64);
                    }
                }
                return 1i32.wrapping_neg() as i64;
            }
        }
    }
    unsafe { core::mem::zeroed() }
}

fn _textiowrapper_readline(self_: *mut textio, limit: i64) -> *mut _object {
    let mut s: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    // No line ending seen yet - put aside current data
    let mut s_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut consumed: i64 = unsafe { core::mem::zeroed() };
    let mut line_len: i64 = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut offset_to_buffer: i64 = unsafe { core::mem::zeroed() };
    let mut chunked: i64 = unsafe { core::mem::zeroed() };
    let mut endpos: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut remaining: *mut _object = unsafe { core::mem::zeroed() };
    let mut chunks: *mut _object = unsafe { core::mem::zeroed() };
    let mut line: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        line = core::ptr::null_mut::<_object>();
        chunks = core::ptr::null_mut::<_object>();
        remaining = core::ptr::null_mut::<_object>();
        CHECK_CLOSED__4!(self_);
        if _textiowrapper_writeflush(self_) < 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            chunked = 0;
            '__join_0: {
                '__join_21: {
                    '__join_41: {
                        '__join_36: {
                            '__join_33: {
                                loop {
                                    if true {
                                        consumed = 0;
                                        // First, get some data if necessary
                                        res = 1;
                                        loop {
                                            if unsafe { (*self_).decoded_chars }.is_null() {
                                                __v8 = true;
                                            } else {
                                                __v8 = PyUnicode_GET_LENGTH!(unsafe {
                                                    (*self_).decoded_chars
                                                }) == 0;
                                            }
                                            if __v8 {
                                                res = textiowrapper_read_chunk(self_, 0i64);
                                                if res < 0 {
                                                    // NOTE: PyErr_SetFromErrno() calls PyErr_CheckSignals()
                                                    // when EINTR occurs so we needn't do it ourselves.
                                                    if _PyIO_trap_eintr() == 0 {
                                                        break '__join_0;
                                                    }
                                                } else {
                                                    if res == 0 {
                                                        break;
                                                    }
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                        if res == 0 {
                                            break '__join_41;
                                        } else {
                                            if remaining.is_null() {
                                                line =
                                                    Py_NewRef!(unsafe { (*self_).decoded_chars });
                                                start = (*self_).decoded_chars_used;
                                                offset_to_buffer = 0;
                                            } else {
                                                line = PyUnicode_Concat(
                                                    remaining,
                                                    (*self_).decoded_chars,
                                                );
                                                start = 0;
                                                offset_to_buffer = PyUnicode_GET_LENGTH!(remaining);
                                                Py_CLEAR__2!(remaining);
                                                if line.is_null() {
                                                    break '__join_0;
                                                }
                                            }
                                            let ptr: *const i8 = PyUnicode_DATA!(line) as *const i8;
                                            line_len = PyUnicode_GET_LENGTH!(line);
                                            let kind: i32 = PyUnicode_KIND!(line);
                                            endpos = _PyIO_find_line_ending(
                                                (*self_).readtranslate as i32,
                                                (*self_).readuniversal as i32,
                                                (*self_).readnl,
                                                kind,
                                                ptr.wrapping_offset(
                                                    (kind as i64).wrapping_mul(start) as isize,
                                                ),
                                                ptr.wrapping_offset(
                                                    (kind as i64).wrapping_mul(line_len) as isize,
                                                ),
                                                core::ptr::addr_of_mut!(consumed),
                                            );
                                            if endpos >= 0 {
                                                break '__join_36;
                                            } else {
                                                // We can put aside up to `endpos`
                                                endpos = consumed.wrapping_add(start);
                                                if limit >= 0
                                                    && endpos
                                                        .wrapping_sub(start)
                                                        .wrapping_add(chunked)
                                                        >= limit
                                                {
                                                    break '__join_33;
                                                } else {
                                                    if endpos > start {
                                                        if chunks.is_null() {
                                                            chunks = PyList_New(0i64);
                                                            if chunks.is_null() {
                                                                break '__join_0;
                                                            }
                                                        }
                                                        s_1 = PyUnicode_Substring(
                                                            line, start, endpos,
                                                        );
                                                        if s_1.is_null() {
                                                            break '__join_0;
                                                        } else {
                                                            if PyList_Append(chunks, s_1) < 0 {
                                                                break;
                                                            } else {
                                                                let __v4: i64 =
                                                                    PyUnicode_GET_LENGTH!(s_1);
                                                                chunked =
                                                                    chunked.wrapping_add(__v4);
                                                                Py_DECREF!(s_1);
                                                            }
                                                        }
                                                    }
                                                    // There may be some remaining bytes we'll have to prepend to the
                                                    // next chunk of data
                                                    if endpos < line_len {
                                                        remaining = PyUnicode_Substring(
                                                            line, endpos, line_len,
                                                        );
                                                        if remaining.is_null() {
                                                            break '__join_0;
                                                        }
                                                    }
                                                    Py_CLEAR__2!(line);
                                                    // We have consumed the buffer
                                                    textiowrapper_set_decoded_chars(
                                                        self_,
                                                        core::ptr::null_mut::<_object>(),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        break '__join_21;
                                    }
                                }
                                Py_DECREF!(s_1);
                                break '__join_0;
                            }
                            // Didn't find line ending, but reached length limit
                            endpos = start.wrapping_add(limit).wrapping_sub(chunked);
                            break '__join_21;
                        }
                        endpos = endpos.wrapping_add(start);
                        if limit >= 0 && endpos.wrapping_sub(start).wrapping_add(chunked) >= limit {
                            endpos = start.wrapping_add(limit).wrapping_sub(chunked);
                            break '__join_21;
                        } else {
                            break '__join_21;
                        }
                    }
                    // end of file
                    textiowrapper_set_decoded_chars(self_, core::ptr::null_mut::<_object>());
                    Py_CLEAR!((*self_).snapshot);
                    offset_to_buffer = 0;
                    endpos = 0;
                    start = 0;
                }
                if !line.is_null() {
                    // Our line ends in the current buffer
                    (*self_).decoded_chars_used = endpos.wrapping_sub(offset_to_buffer);
                    if start > 0 {
                        __v1 = true;
                    } else {
                        __v1 = endpos < PyUnicode_GET_LENGTH!(line);
                    }
                    if __v1 {
                        s = PyUnicode_Substring(line, start, endpos);
                        Py_CLEAR__2!(line);
                        if s.is_null() {
                            break '__join_0;
                        } else {
                            line = s;
                        }
                    }
                }
                if !remaining.is_null() {
                    if chunks.is_null() {
                        chunks = PyList_New(0i64);
                        if chunks.is_null() {
                            break '__join_0;
                        }
                    }
                    if PyList_Append(chunks, remaining) < 0 {
                        break '__join_0;
                    } else {
                        Py_CLEAR__2!(remaining);
                    }
                }
                if !chunks.is_null() {
                    if !line.is_null() {
                        if PyList_Append(chunks, line) < 0 {
                            break '__join_0;
                        } else {
                            Py_DECREF!(line);
                        }
                    }
                    line = PyUnicode_Join(
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
                        chunks,
                    );
                    if line.is_null() {
                        break '__join_0;
                    } else {
                        Py_CLEAR__2!(chunks);
                    }
                }
                if line.is_null() {
                    line = core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .literals
                            ._py_empty
                            ._ascii
                            .ob_base
                    );
                }
                return line;
            }
            Py_XDECREF!(chunks);
            Py_XDECREF!(remaining);
            Py_XDECREF!(line);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.readline
//     size: Py_ssize_t = -1
//     /
// [clinic start generated code]
/// [clinic end generated code: output=344afa98804e8b25 input=b65bab871dc3ddba]
fn _io_TextIOWrapper_readline_impl(self_: *mut textio, size: i64) -> *mut _object {
    CHECK_ATTACHED!(self_);
    _textiowrapper_readline(self_, size)
}

// Seek and Tell
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_textio_c_2395 {
    start_pos: i64,
    dec_flags: i32,
    bytes_to_feed: i32,
    chars_to_skip: i32,
    need_eof: i8,
}

// To speed up cookie packing/unpacking, we store the fields in a temporary
// string and call _PyLong_FromByteArray() or _PyLong_AsByteArray (resp.).
// The following macros define at which offsets in the intermediary byte
// string the various CookieStruct fields will be stored.
// Little-endian mode: the least significant byte of start_pos will
// naturally end up the least significant byte of the cookie.
fn textiowrapper_parse_cookie(
    cookie: &mut __SlateRecord_textio_c_2395,
    cookieObj: *mut _object,
) -> i32 {
    let mut buffer: __SlateAlign16<[u8; 21]> = __SlateAlign16([0u8; 21]);
    let cookieLong: *mut _longobject = (unsafe { PyNumber_Long(cookieObj) }) as *mut _longobject;
    if cookieLong.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        _PyLong_AsByteArray(
            cookieLong,
            buffer.0.as_mut_ptr() as *mut u8,
            core::mem::size_of::<[u8; 21]>() as u64,
            1i32,
            0i32,
            1i32,
        )
    }) < 0
    {
        Py_DECREF__2!(cookieLong);
        return 1i32.wrapping_neg();
    }
    Py_DECREF__2!(cookieLong);
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as *const ())
                as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).start_pos)
                as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i64>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((buffer.0.as_mut_ptr() as *mut u8)
                .wrapping_offset(core::mem::size_of::<i64>() as isize) as *const ())
                as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).dec_flags)
                as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(
                (core::mem::size_of::<i64>() as u64)
                    .wrapping_add(core::mem::size_of::<i32>() as u64) as isize,
            ) as *const ()) as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).bytes_to_feed)
                as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(
                (core::mem::size_of::<i64>() as u64)
                    .wrapping_add(2u64.wrapping_mul(core::mem::size_of::<i32>() as u64))
                    as isize,
            ) as *const ()) as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).chars_to_skip)
                as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(
                (core::mem::size_of::<i64>() as u64)
                    .wrapping_add(3u64.wrapping_mul(core::mem::size_of::<i32>() as u64))
                    as isize,
            ) as *const ()) as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).need_eof)
                as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i8>() as u64) as usize) as usize,
        )
    };
    0
}

fn textiowrapper_build_cookie(cookie: &mut __SlateRecord_textio_c_2395) -> *mut _object {
    let mut buffer: __SlateAlign16<[u8; 21]> = __SlateAlign16([0u8; 21]);
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).start_pos)
                as *const ()) as *const core::ffi::c_void) as *const u8,
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i64>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).dec_flags)
                as *const ()) as *const core::ffi::c_void) as *const u8,
            (((buffer.0.as_mut_ptr() as *mut u8)
                .wrapping_offset(core::mem::size_of::<i64>() as isize) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).bytes_to_feed)
                as *const ()) as *const core::ffi::c_void) as *const u8,
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(
                (core::mem::size_of::<i64>() as u64)
                    .wrapping_add(core::mem::size_of::<i32>() as u64) as isize,
            ) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).chars_to_skip)
                as *const ()) as *const core::ffi::c_void) as *const u8,
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(
                (core::mem::size_of::<i64>() as u64)
                    .wrapping_add(2u64.wrapping_mul(core::mem::size_of::<i32>() as u64))
                    as isize,
            ) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((core::ptr::addr_of_mut!((*(cookie as *mut __SlateRecord_textio_c_2395)).need_eof)
                as *const ()) as *const core::ffi::c_void) as *const u8,
            (((buffer.0.as_mut_ptr() as *mut u8).wrapping_offset(
                (core::mem::size_of::<i64>() as u64)
                    .wrapping_add(3u64.wrapping_mul(core::mem::size_of::<i32>() as u64))
                    as isize,
            ) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<i8>() as u64) as usize) as usize,
        )
    };
    unsafe {
        _PyLong_FromByteArray(
            buffer.0.as_mut_ptr() as *const u8,
            core::mem::size_of::<[u8; 21]>() as u64,
            1i32,
            0i32,
        )
    }
}

fn _textiowrapper_decoder_setstate(
    self_: *mut textio,
    cookie: &__SlateRecord_textio_c_2395,
) -> i32 {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    // When seeking to the start of the stream, we call decoder.reset()
    // rather than decoder.getstate().
    // This is for a few decoders such as utf-16 for which the state value
    // at start is not (b"", 0) but e.g. (b"", 2) (meaning, in the case of
    // utf-16, that we are expecting a BOM).
    if (unsafe { (*(cookie as *const __SlateRecord_textio_c_2395)).start_pos }) == 0
        && (unsafe { (*(cookie as *const __SlateRecord_textio_c_2395)).dec_flags }) == 0
    {
        res = PyObject_CallMethodNoArgs(unsafe { (*self_).decoder }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_reset
                    ._ascii
                    .ob_base
            )
        });
    } else {
        res = unsafe {
            _PyObject_CallMethod(
                (*self_).decoder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_setstate
                        ._ascii
                        .ob_base
                ),
                c"((yi))".as_ptr(),
                c"".as_ptr() as *mut i8,
                (*(cookie as *const __SlateRecord_textio_c_2395)).dec_flags,
            )
        };
    }
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(res);
    0
}

fn _textiowrapper_encoder_reset(self_: *mut textio, start_of_stream: i32) -> i32 {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    if start_of_stream != 0 {
        res = PyObject_CallMethodNoArgs(unsafe { (*self_).encoder }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_reset
                    ._ascii
                    .ob_base
            )
        });
        unsafe {
            (*self_).encoding_start_of_stream = 1;
        }
    } else {
        res = PyObject_CallMethodOneArg(
            unsafe { (*self_).encoder },
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_setstate
                        ._ascii
                        .ob_base
                )
            },
            _PyLong_GetZero(),
        );
        unsafe {
            (*self_).encoding_start_of_stream = 0;
        }
    }
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(res);
    0
}

fn _textiowrapper_encoder_setstate(
    self_: *mut textio,
    cookie: &__SlateRecord_textio_c_2395,
) -> i32 {
    // Same as _textiowrapper_decoder_setstate() above.
    _textiowrapper_encoder_reset(
        self_,
        ((unsafe { (*(cookie as *const __SlateRecord_textio_c_2395)).start_pos }) == 0
            && (unsafe { (*(cookie as *const __SlateRecord_textio_c_2395)).dec_flags }) == 0)
            as i32,
    )
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.seek
//     cookie as cookieObj: object
//       Zero or an opaque number returned by tell().
//     whence: int(c_default='0') = os.SEEK_SET
//       The relative position to seek from.
//     /
//
// Set the stream position, and return the new stream position.
//
// Four operations are supported, given by the following argument
// combinations:
//
// - seek(0, SEEK_SET): Rewind to the start of the stream.
// - seek(cookie, SEEK_SET): Restore a previous position;
//   'cookie' must be a number returned by tell().
// - seek(0, SEEK_END): Fast-forward to the end of the stream.
// - seek(0, SEEK_CUR): Leave the current stream position unchanged.
//
// Any other argument combinations are invalid,
// and may raise exceptions.
// [clinic start generated code]
/// [clinic end generated code: output=0a15679764e2d04d input=4bea78698be23d7e]
fn _io_TextIOWrapper_seek_impl(
    self_: *mut textio,
    mut cookieObj: *mut _object,
    whence: i32,
) -> *mut _object {
    let mut decoded: *mut _object = unsafe { core::mem::zeroed() };
    // Just like _read_chunk, feed the decoder and save a snapshot.
    let mut input_chunk: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    // borrowed reference
    let mut zero: *mut _object = unsafe { core::mem::zeroed() };
    let mut snapshot: *mut _object = unsafe { core::mem::zeroed() };
    let mut cmp: i32 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut cookie: core::mem::MaybeUninit<__SlateRecord_textio_c_2395> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut __SlateRecord_textio_c_2395 =
        core::ptr::addr_of_mut!(cookie) as *mut __SlateRecord_textio_c_2395;
    let mut posobj: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            CHECK_ATTACHED!(self_);
            CHECK_CLOSED__4!(self_);
            Py_INCREF!(cookieObj);
            if (*self_).seekable == 0 {
                _unsupported(
                    (*self_).state,
                    c"underlying stream is not seekable".as_ptr(),
                );
            } else {
                zero = _PyLong_GetZero();
                let __t0: i32 = whence;
                if __t0 == 1 {
                    cmp = PyObject_RichCompareBool(cookieObj, zero, 2i32); // seek relative to current position
                    if cmp < 0 {
                        break '__join_0;
                    } else {
                        if cmp == 0 {
                            _unsupported(
                                (*self_).state,
                                c"can't do nonzero cur-relative seeks".as_ptr(),
                            );
                            break '__join_0;
                        } else {
                            // Seeking to the current position should attempt to
                            // sync the underlying buffer with the current position.
                            Py_DECREF!(cookieObj);
                            cookieObj = PyObject_CallMethodNoArgs(
                                self_ as *mut _object,
                                core::ptr::addr_of_mut!(
                                    _PyRuntime
                                        .static_objects
                                        .singletons
                                        .strings
                                        .identifiers
                                        ._py_tell
                                        ._ascii
                                        .ob_base
                                ),
                            );
                            if cookieObj.is_null() {
                                break '__join_0;
                            }
                        }
                    }
                } else {
                    if __t0 == 2 {
                        cmp = PyObject_RichCompareBool(cookieObj, zero, 2i32); // seek relative to end of file
                        if cmp < 0 {
                            break '__join_0;
                        } else {
                            if cmp == 0 {
                                _unsupported(
                                    (*self_).state,
                                    c"can't do nonzero end-relative seeks".as_ptr(),
                                );
                                break '__join_0;
                            } else {
                                if _PyFile_Flush(self_ as *mut _object) < 0 {
                                    break '__join_0;
                                } else {
                                    textiowrapper_set_decoded_chars(
                                        self_,
                                        core::ptr::null_mut::<_object>(),
                                    );
                                    Py_CLEAR!((*self_).snapshot);
                                    if !unsafe { (*self_).decoder }.is_null() {
                                        res = PyObject_CallMethodNoArgs(
                                            (*self_).decoder,
                                            core::ptr::addr_of_mut!(
                                                _PyRuntime
                                                    .static_objects
                                                    .singletons
                                                    .strings
                                                    .identifiers
                                                    ._py_reset
                                                    ._ascii
                                                    .ob_base
                                            ),
                                        );
                                        if res.is_null() {
                                            break '__join_0;
                                        } else {
                                            Py_DECREF!(res);
                                        }
                                    }
                                    res = _PyObject_CallMethod(
                                        (*self_).buffer,
                                        core::ptr::addr_of_mut!(
                                            _PyRuntime
                                                .static_objects
                                                .singletons
                                                .strings
                                                .identifiers
                                                ._py_seek
                                                ._ascii
                                                .ob_base
                                        ),
                                        c"ii".as_ptr(),
                                        0i32,
                                        2i32,
                                    );
                                    Py_CLEAR__2!(cookieObj);
                                    if res.is_null() {
                                        break '__join_0;
                                    } else {
                                        if !unsafe { (*self_).encoder }.is_null() {
                                            // If seek() == 0, we are at the start of stream, otherwise not
                                            cmp = PyObject_RichCompareBool(res, zero, 2i32);
                                            if cmp < 0 {
                                                __v2 = true;
                                            } else {
                                                __v2 =
                                                    _textiowrapper_encoder_reset(self_, cmp) != 0;
                                            }
                                            if __v2 {
                                                Py_DECREF!(res);
                                                break '__join_0;
                                            }
                                        }
                                        return res;
                                    }
                                }
                            }
                        }
                    } else {
                        if __t0 != 0 {
                            PyErr_Format(
                                PyExc_ValueError,
                                c"invalid whence (%d, should be %d, %d or %d)".as_ptr(),
                                whence,
                                slate_support::SEEK_SET as i32,
                                slate_support::SEEK_CUR as i32,
                                slate_support::SEEK_END as i32,
                            );
                            break '__join_0;
                        }
                    }
                }
                cmp = PyObject_RichCompareBool(cookieObj, zero, 0i32);
                if cmp >= 0 {
                    if cmp == 1 {
                        PyErr_Format(
                            PyExc_ValueError,
                            c"negative seek position %R".as_ptr(),
                            cookieObj,
                        );
                    } else {
                        if _PyFile_Flush(self_ as *mut _object) >= 0 {
                            // The strategy of seek() is to go back to the safe start point
                            // and replay the effect of read(chars_to_skip) from there.
                            if textiowrapper_parse_cookie(
                                &mut (*core::ptr::addr_of_mut!(*__slate_slot_7)),
                                cookieObj,
                            ) >= 0
                            {
                                // Seek back to the safe start point.
                                posobj = PyLong_FromSsize_t((*__slate_slot_7).start_pos);
                                if !posobj.is_null() {
                                    res = PyObject_CallMethodOneArg(
                                        (*self_).buffer,
                                        core::ptr::addr_of_mut!(
                                            _PyRuntime
                                                .static_objects
                                                .singletons
                                                .strings
                                                .identifiers
                                                ._py_seek
                                                ._ascii
                                                .ob_base
                                        ),
                                        posobj,
                                    );
                                    Py_DECREF!(posobj);
                                    if !res.is_null() {
                                        Py_DECREF!(res);
                                        textiowrapper_set_decoded_chars(
                                            self_,
                                            core::ptr::null_mut::<_object>(),
                                        );
                                        Py_CLEAR!((*self_).snapshot);
                                        // Restore the decoder to its state from the safe start point.
                                        if !unsafe { (*self_).decoder }.is_null() {
                                            if _textiowrapper_decoder_setstate(
                                                self_,
                                                &(*core::ptr::addr_of_mut!(*__slate_slot_7)),
                                            ) < 0
                                            {
                                                break '__join_0;
                                            }
                                        }
                                        if (*__slate_slot_7).chars_to_skip != 0 {
                                            input_chunk = _PyObject_CallMethod(
                                                (*self_).buffer,
                                                core::ptr::addr_of_mut!(
                                                    _PyRuntime
                                                        .static_objects
                                                        .singletons
                                                        .strings
                                                        .identifiers
                                                        ._py_read
                                                        ._ascii
                                                        .ob_base
                                                ),
                                                c"i".as_ptr(),
                                                (*__slate_slot_7).bytes_to_feed,
                                            );
                                            if input_chunk.is_null() {
                                                break '__join_0;
                                            } else {
                                                if PyBytes_Check!(input_chunk) == 0 {
                                                    PyErr_Format(PyExc_TypeError, c"underlying read() should have returned a bytes object, not '%.200s'".as_ptr(), (*Py_TYPE!(input_chunk)).tp_name);
                                                    Py_DECREF!(input_chunk);
                                                    break '__join_0;
                                                } else {
                                                    snapshot = Py_BuildValue(
                                                        c"iN".as_ptr(),
                                                        (*__slate_slot_7).dec_flags,
                                                        input_chunk,
                                                    );
                                                    if snapshot.is_null() {
                                                        break '__join_0;
                                                    } else {
                                                        Py_XSETREF!((*self_).snapshot, snapshot);
                                                        decoded = PyObject_CallMethodObjArgs(
                                                            (*self_).decoder,
                                                            core::ptr::addr_of_mut!(
                                                                _PyRuntime
                                                                    .static_objects
                                                                    .singletons
                                                                    .strings
                                                                    .identifiers
                                                                    ._py_decode
                                                                    ._ascii
                                                                    .ob_base
                                                            ),
                                                            input_chunk,
                                                            if (*__slate_slot_7).need_eof != 0 {
                                                                _PyObject_CAST__2!(
                                                                    core::ptr::addr_of_mut!(
                                                                        _Py_TrueStruct
                                                                    )
                                                                )
                                                            } else {
                                                                _PyObject_CAST__2!(
                                                                    core::ptr::addr_of_mut!(
                                                                        _Py_FalseStruct
                                                                    )
                                                                )
                                                            },
                                                            core::ptr::null_mut::<()>(),
                                                        );
                                                        if check_decoded(decoded) < 0 {
                                                            break '__join_0;
                                                        } else {
                                                            textiowrapper_set_decoded_chars(
                                                                self_, decoded,
                                                            );
                                                            // Skip chars_to_skip of the decoded characters.
                                                            if PyUnicode_GetLength(
                                                                (*self_).decoded_chars,
                                                            ) < ((*__slate_slot_7).chars_to_skip
                                                                as i64)
                                                            {
                                                                PyErr_SetString(PyExc_OSError, c"can't restore logical file position".as_ptr());
                                                                break '__join_0;
                                                            } else {
                                                                (*self_).decoded_chars_used =
                                                                    (*__slate_slot_7).chars_to_skip
                                                                        as i64;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            snapshot = Py_BuildValue(
                                                c"iy".as_ptr(),
                                                (*__slate_slot_7).dec_flags,
                                                c"".as_ptr() as *mut i8,
                                            );
                                            if snapshot.is_null() {
                                                break '__join_0;
                                            } else {
                                                Py_XSETREF!((*self_).snapshot, snapshot);
                                            }
                                        }
                                        // Finally, reset the encoder (merely useful for proper BOM handling)
                                        if !unsafe { (*self_).encoder }.is_null() {
                                            if _textiowrapper_encoder_setstate(
                                                self_,
                                                &(*core::ptr::addr_of_mut!(*__slate_slot_7)),
                                            ) < 0
                                            {
                                                break '__join_0;
                                            }
                                        }
                                        return cookieObj;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(cookieObj);
        return core::ptr::null_mut::<_object>();
    }
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.tell
//
// Return the stream position as an opaque number.
//
// The return value of tell() can be given as input to seek(), to
// restore a previous stream position.
// [clinic start generated code]
/// [clinic end generated code: output=4f168c08bf34ad5f input=aeece020f747fd92]
fn _io_TextIOWrapper_tell_impl(self_: *mut textio) -> *mut _object {
    // We didn't get enough decoded data; signal EOF to get more.
    // final =
    let mut decoded: *mut _object = unsafe { core::mem::zeroed() };
    let mut _state: *mut _object = unsafe { core::mem::zeroed() };
    let mut dec_buffer: *mut _object = unsafe { core::mem::zeroed() };
    let mut _decoded: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut _state_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut dec_buffer_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut _decoded_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut dec_flags: i32 = unsafe { core::mem::zeroed() };
    let mut dec_buffer_len: i64 = unsafe { core::mem::zeroed() };
    let mut input_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut input: *const i8 = unsafe { core::mem::zeroed() };
    let mut saved_state: *mut _object = unsafe { core::mem::zeroed() };
    let mut skip_back: i64 = unsafe { core::mem::zeroed() };
    let mut skip_bytes: i64 = unsafe { core::mem::zeroed() };
    let mut chars_decoded: i64 = unsafe { core::mem::zeroed() };
    let mut chars_to_skip: i64 = unsafe { core::mem::zeroed() };
    let mut next_input: *mut _object = unsafe { core::mem::zeroed() };
    let mut cookie: core::mem::MaybeUninit<__SlateRecord_textio_c_2395> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_47: *mut __SlateRecord_textio_c_2395 =
        core::ptr::addr_of_mut!(cookie) as *mut __SlateRecord_textio_c_2395;
    let mut posobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_2: {
            posobj = core::ptr::null_mut::<_object>();
            core::ptr::write(__slate_slot_47, {
                let mut __t0: __SlateRecord_textio_c_2395 = core::mem::zeroed();
                __t0.start_pos = 0i64;
                __t0.dec_flags = 0i32;
                __t0.bytes_to_feed = 0i32;
                __t0.chars_to_skip = 0i32;
                __t0.need_eof = 0i8;
                __t0
            });
            next_input = core::ptr::null_mut::<_object>();
            saved_state = core::ptr::null_mut::<_object>();
            CHECK_ATTACHED!(self_);
            CHECK_CLOSED__4!(self_);
            if (*self_).seekable == 0 {
                _unsupported(
                    (*self_).state,
                    c"underlying stream is not seekable".as_ptr(),
                );
            } else {
                if (*self_).telling == 0 {
                    PyErr_SetString(
                        PyExc_OSError,
                        c"telling position disabled by next() call".as_ptr(),
                    );
                } else {
                    if _textiowrapper_writeflush(self_) < 0 {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        if _PyFile_Flush(self_ as *mut _object) >= 0 {
                            posobj = PyObject_CallMethodNoArgs(
                                (*self_).buffer,
                                core::ptr::addr_of_mut!(
                                    _PyRuntime
                                        .static_objects
                                        .singletons
                                        .strings
                                        .identifiers
                                        ._py_tell
                                        ._ascii
                                        .ob_base
                                ),
                            );
                            if !posobj.is_null() {
                                if unsafe { (*self_).decoder }.is_null()
                                    || unsafe { (*self_).snapshot }.is_null()
                                {
                                    return posobj;
                                } else {
                                    (*__slate_slot_47).start_pos = PyLong_AsLong(posobj);
                                    Py_DECREF!(posobj);
                                    if PyErr_Occurred().is_null() {
                                        // Skip backward to the snapshot point (see _read_chunk).
                                        if PyArg_ParseTuple(
                                            (*self_).snapshot,
                                            c"iO".as_ptr(),
                                            core::ptr::addr_of_mut!((*__slate_slot_47).dec_flags),
                                            core::ptr::addr_of_mut!(next_input),
                                        ) != 0
                                        {
                                            // Own next_input: a reentrant or concurrent seek can drop the snapshot.
                                            Py_INCREF!(next_input);
                                            let __v36: i64 = PyBytes_GET_SIZE!(next_input);
                                            let __v35: i64 = (*__slate_slot_47).start_pos;
                                            (*__slate_slot_47).start_pos =
                                                __v35.wrapping_sub(__v36);
                                            // How many decoded characters have been used up since the snapshot?
                                            if (*self_).decoded_chars_used == 0 {
                                                // We haven't moved from the snapshot point.
                                                Py_DECREF!(next_input);
                                                return textiowrapper_build_cookie(
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_47
                                                    )),
                                                );
                                            } else {
                                                chars_to_skip = (*self_).decoded_chars_used;
                                                // Decoder state will be restored at the end
                                                saved_state = PyObject_CallMethodNoArgs(
                                                    (*self_).decoder,
                                                    core::ptr::addr_of_mut!(
                                                        _PyRuntime
                                                            .static_objects
                                                            .singletons
                                                            .strings
                                                            .identifiers
                                                            ._py_getstate
                                                            ._ascii
                                                            .ob_base
                                                    ),
                                                );
                                                if !saved_state.is_null() {
                                                    // Fast search for an acceptable start point, close to our
                                                    // current pos
                                                    skip_bytes = ((*self_).b2cratio
                                                        * (chars_to_skip as f64))
                                                        as i64;
                                                    skip_back = 1;
                                                    input =
                                                        PyBytes_AS_STRING!(next_input) as *const i8;
                                                    '__join_39: {
                                                        '__join_37: {
                                                            '__join_35: {
                                                                '__join_29: {
                                                                    loop {
                                                                        if skip_bytes > 0 {
                                                                            // Decode up to temptative start point
                                                                            if _textiowrapper_decoder_setstate(self_, &(*core::ptr::addr_of_mut!(*__slate_slot_47))) < 0 {
break '__join_2;
} else {
loop {
_decoded_1 = _PyObject_CallMethod((*self_).decoder, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_decode._ascii.ob_base), c"y#".as_ptr(), input, skip_bytes);
if check_decoded(_decoded_1) < 0 {
break '__join_2;
} else {
chars_decoded = PyUnicode_GET_LENGTH!(_decoded_1);
Py_DECREF!(_decoded_1);
if true {
break;
}
}
}
if chars_decoded <= chars_to_skip {
loop {
_state_1 = PyObject_CallMethodNoArgs((*self_).decoder, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_getstate._ascii.ob_base));
if _state_1.is_null() {
break '__join_2;
} else {
if PyTuple_Check!(_state_1) == 0 {
break '__join_39;
} else {
if PyArg_ParseTuple(_state_1, c"Oi;illegal decoder state".as_ptr(), core::ptr::addr_of_mut!(dec_buffer_1), core::ptr::addr_of_mut!(dec_flags)) == 0 {
break '__join_37;
} else {
if PyBytes_Check!(dec_buffer_1) == 0 {
break '__join_35;
} else {
dec_buffer_len = PyBytes_GET_SIZE!(dec_buffer_1);
Py_DECREF!(_state_1);
if true {
break;
}
}
}
}
}
}
if dec_buffer_len == 0 {
break;
} else {
// Skip back by buffered amount and reset heuristic
skip_bytes = skip_bytes.wrapping_sub(dec_buffer_len);
skip_back = 1;
}
} else {
// We're too far ahead, skip back a bit
skip_bytes = skip_bytes.wrapping_sub(skip_back);
skip_back = skip_back.wrapping_mul(2);
}
}
                                                                        } else {
                                                                            break '__join_29;
                                                                        }
                                                                    }
                                                                    // Before pos and no bytes buffered in decoder => OK
                                                                    (*__slate_slot_47).dec_flags =
                                                                        dec_flags;
                                                                    chars_to_skip = chars_to_skip
                                                                        .wrapping_sub(
                                                                            chars_decoded,
                                                                        );
                                                                }
                                                                if skip_bytes <= 0 {
                                                                    skip_bytes = 0;
                                                                    if _textiowrapper_decoder_setstate(self_, &(*core::ptr::addr_of_mut!(*__slate_slot_47))) < 0 {
break '__join_2;
}
                                                                }
                                                                '__join_5: {
                                                                    // Note our initial start point.
                                                                    let __v22: i64 =
                                                                        (*__slate_slot_47)
                                                                            .start_pos;
                                                                    (*__slate_slot_47).start_pos =
                                                                        __v22.wrapping_add(
                                                                            skip_bytes,
                                                                        );
                                                                    (*__slate_slot_47)
                                                                        .chars_to_skip = Py_SAFE_DOWNCAST!(
                                                                        chars_to_skip,
                                                                        i32
                                                                    );
                                                                    if chars_to_skip != 0 {
                                                                        // We should be close to the desired position.  Now feed the decoder one
                                                                        // byte at a time until we reach the `chars_to_skip` target.
                                                                        // As we go, note the nearest "safe start point" before the current
                                                                        // location (a point where the decoder has nothing buffered, so seek()
                                                                        // can safely start from there and advance to this location).
                                                                        chars_decoded = 0;
                                                                        input = PyBytes_AS_STRING!(
                                                                            next_input
                                                                        )
                                                                            as *const i8;
                                                                        input_end = input
                                                                            .wrapping_offset(
                                                                                PyBytes_GET_SIZE!(
                                                                                    next_input
                                                                                )
                                                                                    as isize,
                                                                            );
                                                                        let __v20: *const i8 =
                                                                            input;
                                                                        input = __v20
                                                                            .wrapping_offset(
                                                                                skip_bytes as isize,
                                                                            );
                                                                        '__join_20: {
                                                                            '__join_18: {
                                                                                '__join_16: {
                                                                                    while input
                                                                                        < input_end
                                                                                    {
                                                                                        loop {
                                                                                            _decoded = _PyObject_CallMethod((*self_).decoder, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_decode._ascii.ob_base), c"y#".as_ptr(), input, 1i64);
                                                                                            if check_decoded(_decoded) < 0 {
break '__join_2;
} else {
n = PyUnicode_GET_LENGTH!(_decoded);
Py_DECREF!(_decoded);
if true {
break;
}
}
                                                                                        }
                                                                                        // We got n chars for 1 byte
                                                                                        chars_decoded = chars_decoded.wrapping_add(n);
                                                                                        let __v14: i32 = (*__slate_slot_47).bytes_to_feed;
                                                                                        (*__slate_slot_47).bytes_to_feed = __v14.wrapping_add(1);
                                                                                        loop {
                                                                                            _state = PyObject_CallMethodNoArgs((*self_).decoder, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_getstate._ascii.ob_base));
                                                                                            if _state.is_null() {
break '__join_2;
} else {
if PyTuple_Check!(_state) == 0 {
break '__join_20;
} else {
if PyArg_ParseTuple(_state, c"Oi;illegal decoder state".as_ptr(), core::ptr::addr_of_mut!(dec_buffer), core::ptr::addr_of_mut!(dec_flags)) == 0 {
break '__join_18;
} else {
if PyBytes_Check!(dec_buffer) == 0 {
break '__join_16;
} else {
dec_buffer_len = PyBytes_GET_SIZE!(dec_buffer);
Py_DECREF!(_state);
if true {
break;
}
}
}
}
}
                                                                                        }
                                                                                        if dec_buffer_len == 0 && chars_decoded <= chars_to_skip {
// Decoder buffer is empty, so this is a safe start point.
let __v10: i64 = (*__slate_slot_47).start_pos;
(*__slate_slot_47).start_pos = __v10.wrapping_add((*__slate_slot_47).bytes_to_feed as i64);
chars_to_skip = chars_to_skip.wrapping_sub(chars_decoded);
(*__slate_slot_47).dec_flags = dec_flags;
(*__slate_slot_47).bytes_to_feed = 0;
chars_decoded = 0;
}
                                                                                        if chars_decoded >= chars_to_skip {
break;
} else {
let __v6: *const i8 = input;
input = __v6.wrapping_offset(1 as isize);
}
                                                                                    }
                                                                                    if input
                                                                                        == input_end
                                                                                    {
                                                                                        decoded = _PyObject_CallMethod((*self_).decoder, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_decode._ascii.ob_base), (b"yO\0".as_ptr() as *mut i8) as *const i8, b"\0".as_ptr() as *mut i8, _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)));
                                                                                        if check_decoded(decoded) < 0 {
break '__join_2;
} else {
let __v3: i64 = PyUnicode_GET_LENGTH!(decoded);
chars_decoded = chars_decoded.wrapping_add(__v3);
Py_DECREF!(decoded);
(*__slate_slot_47).need_eof = 1;
if chars_decoded < chars_to_skip {
PyErr_SetString(PyExc_OSError, c"can't reconstruct logical file position".as_ptr());
break '__join_2;
} else {
break '__join_5;
}
}
                                                                                    } else {
                                                                                        break '__join_5;
                                                                                    }
                                                                                }
                                                                                PyErr_Format(PyExc_TypeError, c"illegal decoder state: the first item should be a bytes object, not '%.200s'".as_ptr(), (*Py_TYPE!(dec_buffer)).tp_name);
                                                                                Py_DECREF!(_state);
                                                                                break '__join_2;
                                                                            }
                                                                            Py_DECREF!(_state);
                                                                            break '__join_2;
                                                                        }
                                                                        PyErr_SetString(PyExc_TypeError, c"illegal decoder state".as_ptr());
                                                                        Py_DECREF!(_state);
                                                                        break '__join_2;
                                                                    }
                                                                }
                                                                Py_XDECREF!(next_input);
                                                                res = PyObject_CallMethodOneArg(
                                                                    (*self_).decoder,
                                                                    core::ptr::addr_of_mut!(
                                                                        _PyRuntime
                                                                            .static_objects
                                                                            .singletons
                                                                            .strings
                                                                            .identifiers
                                                                            ._py_setstate
                                                                            ._ascii
                                                                            .ob_base
                                                                    ),
                                                                    saved_state,
                                                                );
                                                                Py_DECREF!(saved_state);
                                                                if res.is_null() {
                                                                    return core::ptr::null_mut::<
                                                                        _object,
                                                                    >(
                                                                    );
                                                                } else {
                                                                    Py_DECREF!(res);
                                                                    // The returned cookie corresponds to the last safe start point.
                                                                    (*__slate_slot_47)
                                                                        .chars_to_skip = Py_SAFE_DOWNCAST!(
                                                                        chars_to_skip,
                                                                        i32
                                                                    );
                                                                    return textiowrapper_build_cookie(&mut (*core::ptr::addr_of_mut!(*__slate_slot_47)));
                                                                }
                                                            }
                                                            PyErr_Format(PyExc_TypeError, c"illegal decoder state: the first item should be a bytes object, not '%.200s'".as_ptr(), (*Py_TYPE!(dec_buffer_1)).tp_name);
                                                            Py_DECREF!(_state_1);
                                                            break '__join_2;
                                                        }
                                                        Py_DECREF!(_state_1);
                                                        break '__join_2;
                                                    }
                                                    PyErr_SetString(
                                                        PyExc_TypeError,
                                                        c"illegal decoder state".as_ptr(),
                                                    );
                                                    Py_DECREF!(_state_1);
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
        Py_XDECREF!(next_input);
        if !saved_state.is_null() {
            let exc: *mut _object = PyErr_GetRaisedException();
            res = PyObject_CallMethodOneArg(
                (*self_).decoder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_setstate
                        ._ascii
                        .ob_base
                ),
                saved_state,
            );
            _PyErr_ChainExceptions1(exc);
            Py_DECREF!(saved_state);
            Py_XDECREF!(res);
        }
        return core::ptr::null_mut::<_object>();
    }
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.truncate
//     pos: object = None
//     /
// [clinic start generated code]
/// [clinic end generated code: output=90ec2afb9bb7745f input=8bddb320834c93ee]
fn _io_TextIOWrapper_truncate_impl(self_: *mut textio, pos: *mut _object) -> *mut _object {
    CHECK_ATTACHED!(self_);
    if (unsafe { _PyFile_Flush(self_ as *mut _object) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    PyObject_CallMethodOneArg(
        unsafe { (*self_).buffer },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_truncate
                    ._ascii
                    .ob_base
            )
        },
        pos,
    )
}

#[unsafe(link_section = ".text.slate_distinct.textio.textiowrapper_repr")]
extern "C-unwind" fn textiowrapper_repr(op: *mut _object) -> *mut _object {
    let mut type_name: *const i8 = unsafe { core::mem::zeroed() };
    let mut self_: *mut textio = unsafe { core::mem::zeroed() };
    let mut status: i32 = unsafe { core::mem::zeroed() };
    let mut s: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut modeobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut nameobj: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        self_ = textio_CAST!(op);
        type_name = (*Py_TYPE__2!(self_)).tp_name;
        CHECK_INITIALIZED__3!(self_);
        res = PyUnicode_FromFormat(c"<%.100s".as_ptr(), type_name);
        if res.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_2: {
                status = Py_ReprEnter(op);
                if status != 0 {
                    if status > 0 {
                        PyErr_Format(
                            PyExc_RuntimeError,
                            c"reentrant call inside %.100s.__repr__".as_ptr(),
                            type_name,
                        );
                    }
                } else {
                    if PyObject_GetOptionalAttr(
                        op,
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
                        core::ptr::addr_of_mut!(nameobj),
                    ) < 0
                    {
                        if PyErr_ExceptionMatches(PyExc_ValueError) == 0 {
                            break '__join_2;
                        } else {
                            // Ignore ValueError raised if the underlying stream was detached
                            PyErr_Clear();
                        }
                    }
                    if !nameobj.is_null() {
                        s = PyUnicode_FromFormat(c" name=%R".as_ptr(), nameobj);
                        Py_DECREF!(nameobj);
                        if s.is_null() {
                            break '__join_2;
                        } else {
                            PyUnicode_AppendAndDel(core::ptr::addr_of_mut!(res), s);
                            if res.is_null() {
                                break '__join_2;
                            }
                        }
                    }
                    if PyObject_GetOptionalAttr(
                        op,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py_mode
                                ._ascii
                                .ob_base
                        ),
                        core::ptr::addr_of_mut!(modeobj),
                    ) >= 0
                    {
                        if !modeobj.is_null() {
                            s = PyUnicode_FromFormat(c" mode=%R".as_ptr(), modeobj);
                            Py_DECREF!(modeobj);
                            if s.is_null() {
                                break '__join_2;
                            } else {
                                PyUnicode_AppendAndDel(core::ptr::addr_of_mut!(res), s);
                                if res.is_null() {
                                    break '__join_2;
                                }
                            }
                        }
                        s = PyUnicode_FromFormat(
                            c"%U encoding=%R>".as_ptr(),
                            res,
                            (*self_).encoding,
                        );
                        Py_DECREF!(res);
                        if status == 0 {
                            Py_ReprLeave(op);
                        }
                        return s;
                    }
                }
            }
            Py_XDECREF!(res);
            if status == 0 {
                Py_ReprLeave(op);
            }
            return core::ptr::null_mut::<_object>();
        }
    }
}

// Inquiries
// [clinic input]
// @critical_section
// _io.TextIOWrapper.fileno
// [clinic start generated code]
/// [clinic end generated code: output=21490a4c3da13e6c input=515e1196aceb97ab]
fn _io_TextIOWrapper_fileno_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_fileno
                ._ascii
                .ob_base
        )
    })
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.seekable
// [clinic start generated code]
/// [clinic end generated code: output=ab223dbbcffc0f00 input=71c4c092736c549b]
fn _io_TextIOWrapper_seekable_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_seekable
                ._ascii
                .ob_base
        )
    })
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.readable
// [clinic start generated code]
/// [clinic end generated code: output=72ff7ba289a8a91b input=80438d1f01b0a89b]
fn _io_TextIOWrapper_readable_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_readable
                ._ascii
                .ob_base
        )
    })
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.writable
// [clinic start generated code]
/// [clinic end generated code: output=a728c71790d03200 input=9d6c22befb0c340a]
fn _io_TextIOWrapper_writable_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_writable
                ._ascii
                .ob_base
        )
    })
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.isatty
// [clinic start generated code]
/// [clinic end generated code: output=12be1a35bace882e input=7f83ff04d4d1733d]
fn _io_TextIOWrapper_isatty_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_isatty
                ._ascii
                .ob_base
        )
    })
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.flush
// [clinic start generated code]
/// [clinic end generated code: output=59de9165f9c2e4d2 input=3ac3bf521bfed59d]
fn _io_TextIOWrapper_flush_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    CHECK_CLOSED__4!(self_);
    unsafe {
        (*self_).telling = (*self_).seekable;
    }
    if _textiowrapper_writeflush(self_) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_flush
                ._ascii
                .ob_base
        )
    })
}

// [clinic input]
// @critical_section
// _io.TextIOWrapper.close
// [clinic start generated code]
/// [clinic end generated code: output=056ccf8b4876e4f4 input=8e12d7079d5ac5c1]
fn _io_TextIOWrapper_close_impl(self_: *mut textio) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut r: i32 = 0;
    CHECK_ATTACHED!(self_);
    res = _io_TextIOWrapper_closed_get_impl(self_);
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    r = unsafe { PyObject_IsTrue(res) };
    Py_DECREF!(res);
    if r < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if r > 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }; // stream already closed
    }
    if (unsafe { (*self_).detached }) != 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }; // gh-142594 null pointer issue
    } else {
        let mut exc: *mut _object = core::ptr::null_mut::<_object>();
        if (unsafe { (*self_).finalizing }) != 0 {
            res = PyObject_CallMethodOneArg(
                unsafe { (*self_).buffer },
                unsafe {
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py__dealloc_warn
                            ._ascii
                            .ob_base
                    )
                },
                self_ as *mut _object,
            );
            if !res.is_null() {
                Py_DECREF!(res);
            } else {
                unsafe { PyErr_Clear() };
            }
        }
        if (unsafe { _PyFile_Flush(self_ as *mut _object) }) < 0 {
            exc = unsafe { PyErr_GetRaisedException() };
        }
        res = PyObject_CallMethodNoArgs(unsafe { (*self_).buffer }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_close
                    ._ascii
                    .ob_base
            )
        });
        if !exc.is_null() {
            unsafe { _PyErr_ChainExceptions1(exc) };
            Py_CLEAR__2!(res);
        }
        return res;
    }
}

fn textiowrapper_iternext_lock_held(op: *mut _object) -> *mut _object {
    let mut line: *mut _object = unsafe { core::mem::zeroed() };
    let self_: *mut textio = textio_CAST!(op);
    CHECK_ATTACHED!(self_);
    unsafe {
        (*self_).telling = 0;
    }
    if Py_IS_TYPE__2!(self_, unsafe { (*(*self_).state).PyTextIOWrapper_Type }) != 0 {
        // Skip method call overhead for speed
        line = _textiowrapper_readline(self_, 1i32.wrapping_neg() as i64);
    } else {
        line = PyObject_CallMethodNoArgs(op, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_readline
                    ._ascii
                    .ob_base
            )
        });
        let __v0: bool = !line.is_null() && PyUnicode_Check!(line) == 0;
        if __v0 {
            unsafe {
                PyErr_Format(
                    PyExc_OSError,
                    c"readline() should have returned a str object, not '%.200s'".as_ptr(),
                    (*Py_TYPE!(line)).tp_name,
                )
            };
            Py_DECREF!(line);
            return core::ptr::null_mut::<_object>();
        }
    }
    if line.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyUnicode_GET_LENGTH!(line) == 0 {
        // Reached EOF or would have blocked
        Py_DECREF!(line);
        Py_CLEAR!((*self_).snapshot);
        unsafe {
            (*self_).telling = (*self_).seekable;
        }
        return core::ptr::null_mut::<_object>();
    }
    line
}

#[unsafe(link_section = ".text.slate_distinct.textio.textiowrapper_iternext")]
extern "C-unwind" fn textiowrapper_iternext(op: *mut _object) -> *mut _object {
    let result: *mut _object = textiowrapper_iternext_lock_held(op);
    result
}

// [clinic input]
// @critical_section
// @getter
// _io.TextIOWrapper.name
// [clinic start generated code]
/// [clinic end generated code: output=8c2f1d6d8756af40 input=26ecec9b39e30e07]
fn _io_TextIOWrapper_name_get_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    unsafe {
        PyObject_GetAttr(
            (*self_).buffer,
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
        )
    }
}

// [clinic input]
// @critical_section
// @getter
// _io.TextIOWrapper.closed
// [clinic start generated code]
/// [clinic end generated code: output=b49b68f443a85e3c input=7dfcf43f63c7003d]
fn _io_TextIOWrapper_closed_get_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    unsafe {
        PyObject_GetAttr(
            (*self_).buffer,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_closed
                    ._ascii
                    .ob_base
            ),
        )
    }
}

// [clinic input]
// @critical_section
// @getter
// _io.TextIOWrapper.newlines
// [clinic start generated code]
/// [clinic end generated code: output=53aa03ac35573180 input=610df647e514b3e8]
fn _io_TextIOWrapper_newlines_get_impl(self_: *mut textio) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    CHECK_ATTACHED!(self_);
    let __v0: bool = unsafe { (*self_).decoder }.is_null()
        || (unsafe {
            PyObject_GetOptionalAttr(
                (*self_).decoder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_newlines
                        ._ascii
                        .ob_base
                ),
                core::ptr::addr_of_mut!(res),
            )
        }) == 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    res
}

// [clinic input]
// @critical_section
// @getter
// _io.TextIOWrapper.errors
// [clinic start generated code]
/// [clinic end generated code: output=dca3a3ef21b09484 input=b45f983e6d43c4d8]
fn _io_TextIOWrapper_errors_get_impl(self_: *mut textio) -> *mut _object {
    CHECK_INITIALIZED__3!(self_);
    Py_NewRef!(unsafe { (*self_).errors })
}

// [clinic input]
// @critical_section
// @getter
// _io.TextIOWrapper._CHUNK_SIZE
// [clinic start generated code]
/// [clinic end generated code: output=039925cd2df375bc input=e9715b0e06ff0fa6]
fn _io_TextIOWrapper__CHUNK_SIZE_get_impl(self_: *mut textio) -> *mut _object {
    CHECK_ATTACHED!(self_);
    unsafe { PyLong_FromSsize_t((*self_).chunk_size) }
}

// [clinic input]
// @critical_section
// @setter
// _io.TextIOWrapper._CHUNK_SIZE
// [clinic start generated code]
/// [clinic end generated code: output=edb86d2db660a5ab input=32fc99861db02a0a]
fn _io_TextIOWrapper__CHUNK_SIZE_set_impl(self_: *mut textio, value: *mut _object) -> i32 {
    let mut n: i64 = 0;
    CHECK_ATTACHED_INT!(self_);
    n = unsafe { PyNumber_AsSsize_t(value, PyExc_ValueError) };
    let __v0: bool = n == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return 1i32.wrapping_neg();
    }
    if n <= 0 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"a strictly positive integer is required".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*self_).chunk_size = n;
    }
    0
}

static mut incrementalnewlinedecoder_methods: __SlateAlign16<[PyMethodDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"decode\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io_IncrementalNewlineDecoder_decode))
            })
        };
        __t0.ml_flags = 130i32;
        __t0.ml_doc = unsafe {
            core::ptr::addr_of!(_io_IncrementalNewlineDecoder_decode__doc__.0) as *const i8
        };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"getstate\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_io_IncrementalNewlineDecoder_getstate);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe {
            core::ptr::addr_of!(_io_IncrementalNewlineDecoder_getstate__doc__.0) as *const i8
        };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"setstate\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_io_IncrementalNewlineDecoder_setstate);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe {
            core::ptr::addr_of!(_io_IncrementalNewlineDecoder_setstate__doc__.0) as *const i8
        };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"reset\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_io_IncrementalNewlineDecoder_reset);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe {
            core::ptr::addr_of!(_io_IncrementalNewlineDecoder_reset__doc__.0) as *const i8
        };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut incrementalnewlinedecoder_getset: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"newlines\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(incrementalnewlinedecoder_newlines_get);
        __t0.set = None;
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut nldecoder_slots: __SlateAlign16<[__SlateRecord_object_h_377; 8]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(incrementalnewlinedecoder_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 56i32;
        __t1.pfunc = (unsafe {
            core::ptr::addr_of!(_io_IncrementalNewlineDecoder___init____doc__.0) as *const i8
        }) as *mut ();
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 64i32;
        __t2.pfunc = (unsafe {
            core::ptr::addr_of_mut!(incrementalnewlinedecoder_methods.0) as *mut PyMethodDef
        }) as *mut ();
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 73i32;
        __t3.pfunc = (unsafe {
            core::ptr::addr_of_mut!(incrementalnewlinedecoder_getset.0) as *mut PyGetSetDef
        }) as *mut ();
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 71i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                        *mut (),
                    ) -> i32,
                >,
                *mut (),
            >(Some(incrementalnewlinedecoder_traverse))
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 51i32;
        __t5.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(incrementalnewlinedecoder_clear),
            )
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 60i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32,
                >,
                *mut (),
            >(Some(_io_IncrementalNewlineDecoder___init__))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 0i32;
        __t7.pfunc = core::ptr::null_mut::<()>();
        __t7
    },
]);

static mut textiowrapper_methods: __SlateAlign16<[PyMethodDef; 17]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"detach\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_io_TextIOWrapper_detach);
        __t0.ml_flags = 4i32;
        __t0.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_detach__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"reconfigure\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io_TextIOWrapper_reconfigure))
            })
        };
        __t1.ml_flags = 130i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_reconfigure__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"write\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_io_TextIOWrapper_write);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_io_TextIOWrapper_write__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"read\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io_TextIOWrapper_read))
            })
        };
        __t3.ml_flags = 128i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(_io_TextIOWrapper_read__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"readline\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io_TextIOWrapper_readline))
            })
        };
        __t4.ml_flags = 128i32;
        __t4.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_readline__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"flush\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(_io_TextIOWrapper_flush);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(_io_TextIOWrapper_flush__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"close\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(_io_TextIOWrapper_close);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(_io_TextIOWrapper_close__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"fileno\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(_io_TextIOWrapper_fileno);
        __t7.ml_flags = 4i32;
        __t7.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_fileno__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"seekable\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(_io_TextIOWrapper_seekable);
        __t8.ml_flags = 4i32;
        __t8.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_seekable__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"readable\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(_io_TextIOWrapper_readable);
        __t9.ml_flags = 4i32;
        __t9.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_readable__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"writable\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(_io_TextIOWrapper_writable);
        __t10.ml_flags = 4i32;
        __t10.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_writable__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"isatty\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(_io_TextIOWrapper_isatty);
        __t11.ml_flags = 4i32;
        __t11.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_isatty__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"seek\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = unsafe {
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
                >(Some(_io_TextIOWrapper_seek))
            })
        };
        __t12.ml_flags = 128i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(_io_TextIOWrapper_seek__doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"tell\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(_io_TextIOWrapper_tell);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(_io_TextIOWrapper_tell__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"truncate\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = unsafe {
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
                >(Some(_io_TextIOWrapper_truncate))
            })
        };
        __t14.ml_flags = 128i32;
        __t14.ml_doc =
            unsafe { core::ptr::addr_of!(_io_TextIOWrapper_truncate__doc__.0) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"__getstate__\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(_PyIOBase_cannot_pickle as *const ())
        };
        __t15.ml_flags = 4i32;
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = core::ptr::null::<i8>();
        __t16.ml_meth = None;
        __t16
    },
]);

static mut textiowrapper_members: __SlateAlign16<[PyMemberDef; 8]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"encoding\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(textio, encoding) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"buffer\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(textio, buffer) as u64) as i64;
        __t1.flags = 1i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"line_buffering\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 14i32;
        __t2.offset = (core::mem::offset_of!(textio, line_buffering) as u64) as i64;
        __t2.flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"write_through\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 14i32;
        __t3.offset = (core::mem::offset_of!(textio, write_through) as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"_finalizing\0".as_ptr() as *mut i8) as *const i8;
        __t4.r#type = 14i32;
        __t4.offset = (core::mem::offset_of!(textio, finalizing) as u64) as i64;
        __t4.flags = 0i32;
        __t4
    },
    {
        let mut __t5: PyMemberDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t5.r#type = 19i32;
        __t5.offset = (core::mem::offset_of!(textio, weakreflist) as u64) as i64;
        __t5.flags = 1i32;
        __t5
    },
    {
        let mut __t6: PyMemberDef = unsafe { core::mem::zeroed() };
        __t6.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t6.r#type = 19i32;
        __t6.offset = (core::mem::offset_of!(textio, dict) as u64) as i64;
        __t6.flags = 1i32;
        __t6
    },
    {
        let mut __t7: PyMemberDef = unsafe { core::mem::zeroed() };
        __t7.name = core::ptr::null::<i8>();
        __t7
    },
]);

static mut textiowrapper_getset: __SlateAlign16<[PyGetSetDef; 6]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(_io_TextIOWrapper_name_get);
        __t0.set = None;
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"closed\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(_io_TextIOWrapper_closed_get);
        __t1.set = None;
        __t1.doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"newlines\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(_io_TextIOWrapper_newlines_get);
        __t2.set = None;
        __t2.doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"errors\0".as_ptr() as *mut i8) as *const i8;
        __t3.get = Some(_io_TextIOWrapper_errors_get);
        __t3.set = None;
        __t3.doc = core::ptr::null::<i8>();
        __t3
    },
    {
        let mut __t4: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"_CHUNK_SIZE\0".as_ptr() as *mut i8) as *const i8;
        __t4.get = Some(_io_TextIOWrapper__CHUNK_SIZE_get);
        __t4.set = Some(_io_TextIOWrapper__CHUNK_SIZE_set);
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
static mut textiowrapper_slots: __SlateAlign16<[__SlateRecord_object_h_377; 11]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(textiowrapper_dealloc),
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
                >(Some(textiowrapper_repr))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 56i32;
            __t2.pfunc =
                (unsafe { core::ptr::addr_of!(_io_TextIOWrapper___init____doc__.0) as *const i8 })
                    as *mut ();
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
                >(Some(textiowrapper_traverse))
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
                >(Some(textiowrapper_clear))
            };
            __t4
        },
        {
            let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t5.slot = 63i32;
            __t5.pfunc = unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                    *mut (),
                >(Some(textiowrapper_iternext))
            };
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 64i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(textiowrapper_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 72i32;
            __t7.pfunc =
                (unsafe { core::ptr::addr_of_mut!(textiowrapper_members.0) as *mut PyMemberDef })
                    as *mut ();
            __t7
        },
        {
            let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t8.slot = 73i32;
            __t8.pfunc =
                (unsafe { core::ptr::addr_of_mut!(textiowrapper_getset.0) as *mut PyGetSetDef })
                    as *mut ();
            __t8
        },
        {
            let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t9.slot = 60i32;
            __t9.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >,
                    *mut (),
                >(Some(_io_TextIOWrapper___init__))
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
