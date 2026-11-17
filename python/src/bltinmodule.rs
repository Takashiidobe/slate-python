//! Built-in functions
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
//! These definitions must match corresponding definitions in graminit.h.
//! Public interface
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
// Smallest negative value of type Py_ssize_t.
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
macro_rules! ADD_TO_ALL {
    () => {};
}

macro_rules! EVAL_CALL_STAT_INC {
    () => {};
}

macro_rules! PyBool_Check {
    ($x:expr) => {
        Py_IS_TYPE!($x, unsafe { core::ptr::addr_of_mut!(PyBool_Type) })
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

macro_rules! PyCell_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyCell_Type) })
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

macro_rules! PyFloat_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyFloat_Type) })
    };
}

macro_rules! PyFunction_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyFunction_Type) })
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

/// Use macros for a bunch of old variants
macro_rules! PyRun_String {
    ($str:expr, $s:expr, $g:expr, $l:expr) => {
        unsafe {
            PyRun_StringFlags(
                $str,
                $s,
                $g,
                $l,
                core::ptr::null_mut::<__SlateRecord_compile_h_30>(),
            )
        }
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

macro_rules! PyType_Check {
    ($op:expr) => {
        PyType_Check(_PyObject_CAST!($op))
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

macro_rules! PyUnicode_READ_CHAR__2 {
    ($unicode:expr, $index:expr) => {
        PyUnicode_READ_CHAR(_PyObject_CAST!($unicode), ($index) as i64)
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

macro_rules! Py_SETREF__3 {
    ($dst:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _object = core::ptr::addr_of_mut!($dst);
            let _tmp_old_dst: *mut _object = unsafe { *_tmp_dst_ptr };
            unsafe {
                *_tmp_dst_ptr = core::ptr::null_mut::<_object>();
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

macro_rules! SETBUILTIN__2 {
    ($NAME:expr, $OBJECT:expr, $dict:expr) => {{
        if (unsafe { PyDict_SetItemString($dict, ($NAME) as *const i8, $OBJECT) }) < 0 {
            return core::ptr::null_mut::<_object>();
        }
        ADD_TO_ALL!();
    }};
}

macro_rules! SETBUILTIN {
    ($NAME:expr, $OBJECT:expr, $dict:expr) => {{
        if (unsafe { PyDict_SetItemString($dict, ($NAME) as *const i8, ($OBJECT) as *mut _object) })
            < 0
        {
            return core::ptr::null_mut::<_object>();
        }
        ADD_TO_ALL!();
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

macro_rules! _filterobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_bltinmodule_c_497
    };
}

macro_rules! _mapobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_bltinmodule_c_1361
    };
}

macro_rules! _zipobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_bltinmodule_c_3089
    };
}

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut PyBool_Type: _typeobject;
    static mut PyType_Type: _typeobject;
    static mut PyBaseObject_Type: _typeobject;
    static mut PySuper_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyByteArray_Type: _typeobject;
    static mut _PyByteArray_empty_string: [i8; 0];
    static mut PyBytes_Type: _typeobject;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_StopIteration: *mut _object;
    static mut PyExc_EOFError: *mut _object;
    static mut PyExc_KeyboardInterrupt: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyComplex_Type: _typeobject;
    static mut PyRange_Type: _typeobject;
    static mut PyMemoryView_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyList_Type: _typeobject;
    static mut PyDict_Type: _typeobject;
    static mut PyEnum_Type: _typeobject;
    static mut PyReversed_Type: _typeobject;
    static mut PySet_Type: _typeobject;
    static mut PyFrozenSet_Type: _typeobject;
    static mut PyFunction_Type: _typeobject;
    static mut PyClassMethod_Type: _typeobject;
    static mut PyStaticMethod_Type: _typeobject;
    static mut PyCode_Type: _typeobject;
    static mut _Py_EllipsisObject: _object;
    static mut PySlice_Type: _typeobject;
    static mut PyCell_Type: _typeobject;
    static mut PyProperty_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn exp(arg0: f64) -> f64;
    fn fabs(arg0: f64) -> f64;
    fn fileno(arg0: *mut slate_support::FILE) -> i32;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyObject_Repr(arg0: *mut _object) -> *mut _object;
    fn PyObject_Str(arg0: *mut _object) -> *mut _object;
    fn PyObject_ASCII(arg0: *mut _object) -> *mut _object;
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_GetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GetOptionalAttr(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn PyObject_SetAttr(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyObject_DelAttr(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_Hash(arg0: *mut _object) -> i64;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyCallable_Check(arg0: *mut _object) -> i32;
    fn PyObject_Dir(arg0: *mut _object) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyObject_GC_Del(arg0: *mut ());
    fn PyUnicode_FromOrdinal(arg0: i32) -> *mut _object;
    fn PyUnicode_Decode(
        arg0: *const i8,
        arg1: i64,
        arg2: *const i8,
        arg3: *const i8,
    ) -> *mut _object;
    fn PyUnicode_AsEncodedString(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
    ) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_FSDecoder(arg0: *mut _object, arg1: *mut ()) -> i32;
    fn PyUnicode_AsUTF8(arg0: *mut _object) -> *const i8;
    fn PyErr_SetNone(arg0: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyErr_CheckSignals() -> i32;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsLongAndOverflow(arg0: *mut _object, arg1: *mut i32) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyLong_AsDouble(arg0: *mut _object) -> f64;
    fn PyLong_FromVoidPtr(arg0: *mut ()) -> *mut _object;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyComplex_FromDoubles(arg0: f64, arg1: f64) -> *mut _object;
    fn PyComplex_AsCComplex(arg0: *mut _object) -> __SlateRecord_complexobject_h_8;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Size(arg0: *mut _object) -> i64;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyList_SetSlice(arg0: *mut _object, arg1: i64, arg2: i64, arg3: *mut _object) -> i32;
    fn PyList_AsTuple(arg0: *mut _object) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyDict_Pop(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyModule_GetDict(arg0: *mut _object) -> *mut _object;
    fn PyFile_GetLine(arg0: *mut _object, arg1: i32) -> *mut _object;
    fn PyFile_WriteObject(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyFile_WriteString(arg0: *const i8, arg1: *mut _object) -> i32;
    fn PyCallIter_New(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
    fn PyArg_UnpackTuple(arg0: *mut _object, arg1: *const i8, arg2: i64, arg3: i64, ...) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyRun_StringFlags(
        arg0: *const i8,
        arg1: i32,
        arg2: *mut _object,
        arg3: *mut _object,
        arg4: *mut __SlateRecord_compile_h_30,
    ) -> *mut _object;
    fn Py_CompileStringObject(
        arg0: *const i8,
        arg1: *mut _object,
        arg2: i32,
        arg3: *mut __SlateRecord_compile_h_30,
        arg4: i32,
    ) -> *mut _object;
    fn PyOS_Readline(
        arg0: *mut slate_support::FILE,
        arg1: *mut slate_support::FILE,
        arg2: *const i8,
    ) -> *mut i8;
    fn PyEval_EvalCode(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyEval_EvalCodeEx(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: *const *mut _object,
        arg4: i32,
        arg5: *const *mut _object,
        arg6: i32,
        arg7: *const *mut _object,
        arg8: i32,
        arg9: *mut _object,
        arg10: *mut _object,
    ) -> *mut _object;
    fn PyEval_GetGlobals() -> *mut _object;
    fn PyEval_MergeCompilerFlags(arg0: *mut __SlateRecord_compile_h_30) -> i32;
    fn PySys_Audit(arg0: *const i8, arg1: *const i8, ...) -> i32;
    fn PyImport_ImportModuleLevelObject(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: *mut _object,
        arg4: i32,
    ) -> *mut _object;
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
    fn PyObject_Size(arg0: *mut _object) -> i64;
    fn PyObject_Format(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetAIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Check(arg0: *mut _object) -> i32;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyNumber_Add(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Divmod(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Power(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyNumber_Absolute(arg0: *mut _object) -> *mut _object;
    fn PyNumber_ToBase(arg0: *mut _object, arg1: i32) -> *mut _object;
    fn PySequence_List(arg0: *mut _object) -> *mut _object;
    fn PyMapping_Check(arg0: *mut _object) -> i32;
    fn PyMapping_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyObject_IsInstance(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyObject_IsSubclass(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn _PyStack_AsDict(arg0: *const *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_VectorcallDict(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyArena_New() -> *mut _arena;
    fn _PyArena_Free(arg0: *mut _arena);
    fn PyAST_mod2obj(arg0: *mut _mod) -> *mut _object;
    fn PyAST_obj2mod(arg0: *mut _object, arg1: *mut _arena, arg2: i32) -> *mut _mod;
    fn PyAST_Check(arg0: *mut _object) -> i32;
    fn _PyAST_Validate(arg0: *mut _mod) -> i32;
    fn _Py_SourceAsString(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        arg3: *mut __SlateRecord_compile_h_30,
        arg4: *mut *mut _object,
    ) -> *const i8;
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
    fn _PyErr_NoMemory(arg0: *mut _ts) -> *mut _object;
    fn _PyInterpreterState_GetConfig(arg0: *mut _is) -> *const PyConfig;
    fn _PyEval_Vector(
        arg0: *mut _ts,
        arg1: *mut __SlateRecord_funcobject_h_62,
        arg2: *mut _object,
        arg3: *const *mut _object,
        arg4: u64,
        arg5: *mut _object,
    ) -> *mut _object;
    fn _PyEval_GetFrame() -> *mut _PyInterpreterFrame;
    fn _PyEval_GetGlobalsFromRunningMain(arg0: *mut _ts) -> *mut _object;
    fn _PyEval_EnsureBuiltins(arg0: *mut _ts, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn _PyEval_GetFrameLocals() -> *mut _object;
    fn _PyAST_Compile(
        arg0: *mut _mod,
        arg1: *mut _object,
        arg2: *mut __SlateRecord_compile_h_30,
        arg3: i32,
        arg4: *mut _arena,
    ) -> *mut PyCodeObject;
    fn _PyCompile_AstPreprocess(
        arg0: *mut _mod,
        arg1: *mut _object,
        arg2: *mut __SlateRecord_compile_h_30,
        arg3: i32,
        arg4: *mut _arena,
        arg5: i32,
    ) -> i32;
    fn _PyFile_Flush(arg0: *mut _object) -> i32;
    fn _PyFloat_ExactDealloc(arg0: *mut _object);
    fn _PyArg_NoKwnames(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_NoKeywords(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
    fn _PyModule_CreateInitialized(arg0: *mut PyModuleDef, arg1: i32) -> *mut _object;
    fn _PyArg_UnpackStack(
        arg0: *const *mut _object,
        arg1: i64,
        arg2: *const i8,
        arg3: i64,
        arg4: i64,
        ...
    ) -> i32;
    fn _PyArg_BadArgument(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut _object);
    fn _PyArg_ParseStackAndKeywords(
        arg0: *const *mut _object,
        arg1: i64,
        arg2: *mut _object,
        arg3: *mut _PyArg_Parser,
        ...
    ) -> i32;
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
    fn _PyType_CalculateMetaclass(arg0: *mut _typeobject, arg1: *mut _object) -> *mut _typeobject;
    fn _PyObject_MaybeCallSpecialNoArgs(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyObject_MaybeCallSpecialOneArg(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn _PySys_GetRequiredAttr(arg0: *mut _object) -> *mut _object;
    fn _PySys_GetRequiredAttrString(arg0: *const i8) -> *mut _object;
    fn _PyTuple_FromArray(arg0: *const *mut _object, arg1: i64) -> *mut _object;
    fn PyAnextAwaitable_New(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
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
struct _frame {}

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
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
// Rich comparison opcodes
fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
}

fn PyType_Check(op: *mut _object) -> i32 {
    PyType_FastSubclass!(Py_TYPE!(op), 2147483648u64)
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits143U0,
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
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits201U0,
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
struct __SlateRecord_complexobject_h_8 {
    real: f64,
    imag: f64,
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
struct _arena {}

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

fn PyCode_GetNumFree(op: *mut PyCodeObject) -> i64 {
    (unsafe { (*op).co_nfreevars }) as i64
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_cellobject_h_14 {
    ob_base: _object,
    ob_ref: *mut _object,
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
union __SlateRecord_pycore_ast_h_165 {
    Module: __SlateRecord_pycore_ast_h_166,
    Interactive: __SlateRecord_pycore_ast_h_171,
    Expression: __SlateRecord_pycore_ast_h_175,
    FunctionType: __SlateRecord_pycore_ast_h_179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_166 {
    body: *mut __SlateRecord_pycore_ast_h_67,
    type_ignores: *mut __SlateRecord_pycore_ast_h_147,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_171 {
    body: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_175 {
    body: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_179 {
    argtypes: *mut __SlateRecord_pycore_ast_h_74,
    returns: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_198 {
    FunctionDef: __SlateRecord_pycore_ast_h_199,
    AsyncFunctionDef: __SlateRecord_pycore_ast_h_209,
    ClassDef: __SlateRecord_pycore_ast_h_219,
    Return: __SlateRecord_pycore_ast_h_228,
    Delete: __SlateRecord_pycore_ast_h_232,
    Assign: __SlateRecord_pycore_ast_h_236,
    TypeAlias: __SlateRecord_pycore_ast_h_242,
    AugAssign: __SlateRecord_pycore_ast_h_248,
    AnnAssign: __SlateRecord_pycore_ast_h_254,
    For: __SlateRecord_pycore_ast_h_261,
    AsyncFor: __SlateRecord_pycore_ast_h_269,
    While: __SlateRecord_pycore_ast_h_277,
    If: __SlateRecord_pycore_ast_h_283,
    With: __SlateRecord_pycore_ast_h_289,
    AsyncWith: __SlateRecord_pycore_ast_h_295,
    Match: __SlateRecord_pycore_ast_h_301,
    Raise: __SlateRecord_pycore_ast_h_306,
    Try: __SlateRecord_pycore_ast_h_311,
    TryStar: __SlateRecord_pycore_ast_h_318,
    Assert: __SlateRecord_pycore_ast_h_325,
    Import: __SlateRecord_pycore_ast_h_330,
    ImportFrom: __SlateRecord_pycore_ast_h_334,
    Global: __SlateRecord_pycore_ast_h_340,
    Nonlocal: __SlateRecord_pycore_ast_h_344,
    Expr: __SlateRecord_pycore_ast_h_348,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_199 {
    name: *mut _object,
    args: *mut _arguments,
    body: *mut __SlateRecord_pycore_ast_h_67,
    decorator_list: *mut __SlateRecord_pycore_ast_h_74,
    returns: *mut _expr,
    type_comment: *mut _object,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_209 {
    name: *mut _object,
    args: *mut _arguments,
    body: *mut __SlateRecord_pycore_ast_h_67,
    decorator_list: *mut __SlateRecord_pycore_ast_h_74,
    returns: *mut _expr,
    type_comment: *mut _object,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_219 {
    name: *mut _object,
    bases: *mut __SlateRecord_pycore_ast_h_74,
    keywords: *mut __SlateRecord_pycore_ast_h_111,
    body: *mut __SlateRecord_pycore_ast_h_67,
    decorator_list: *mut __SlateRecord_pycore_ast_h_74,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_228 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_232 {
    targets: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_236 {
    targets: *mut __SlateRecord_pycore_ast_h_74,
    value: *mut _expr,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_242 {
    name: *mut _expr,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_248 {
    target: *mut _expr,
    op: u32,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_254 {
    target: *mut _expr,
    annotation: *mut _expr,
    value: *mut _expr,
    simple: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_261 {
    target: *mut _expr,
    iter: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_269 {
    target: *mut _expr,
    iter: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_277 {
    test: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_283 {
    test: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_289 {
    items: *mut __SlateRecord_pycore_ast_h_125,
    body: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_295 {
    items: *mut __SlateRecord_pycore_ast_h_125,
    body: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_301 {
    subject: *mut _expr,
    cases: *mut __SlateRecord_pycore_ast_h_132,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_306 {
    exc: *mut _expr,
    cause: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_311 {
    body: *mut __SlateRecord_pycore_ast_h_67,
    handlers: *mut __SlateRecord_pycore_ast_h_89,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    finalbody: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_318 {
    body: *mut __SlateRecord_pycore_ast_h_67,
    handlers: *mut __SlateRecord_pycore_ast_h_89,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    finalbody: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_325 {
    test: *mut _expr,
    msg: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_330 {
    names: *mut __SlateRecord_pycore_ast_h_118,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_334 {
    module: *mut _object,
    names: *mut __SlateRecord_pycore_ast_h_118,
    level: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_340 {
    names: *mut __SlateRecord_pycore_asdl_h_40,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_344 {
    names: *mut __SlateRecord_pycore_asdl_h_40,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_348 {
    value: *mut _expr,
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
union __SlateRecord_pycore_ast_h_538 {
    ExceptHandler: __SlateRecord_pycore_ast_h_539,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_539 {
    r#type: *mut _expr,
    name: *mut _object,
    body: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_607 {
    MatchValue: __SlateRecord_pycore_ast_h_608,
    MatchSingleton: __SlateRecord_pycore_ast_h_612,
    MatchSequence: __SlateRecord_pycore_ast_h_616,
    MatchMapping: __SlateRecord_pycore_ast_h_620,
    MatchClass: __SlateRecord_pycore_ast_h_626,
    MatchStar: __SlateRecord_pycore_ast_h_633,
    MatchAs: __SlateRecord_pycore_ast_h_637,
    MatchOr: __SlateRecord_pycore_ast_h_642,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_608 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_612 {
    value: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_616 {
    patterns: *mut __SlateRecord_pycore_ast_h_140,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_620 {
    keys: *mut __SlateRecord_pycore_ast_h_74,
    patterns: *mut __SlateRecord_pycore_ast_h_140,
    rest: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_626 {
    cls: *mut _expr,
    patterns: *mut __SlateRecord_pycore_ast_h_140,
    kwd_attrs: *mut __SlateRecord_pycore_asdl_h_40,
    kwd_patterns: *mut __SlateRecord_pycore_ast_h_140,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_633 {
    name: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_637 {
    pattern: *mut _pattern,
    name: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_642 {
    patterns: *mut __SlateRecord_pycore_ast_h_140,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_656 {
    TypeIgnore: __SlateRecord_pycore_ast_h_657,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_657 {
    lineno: i32,
    tag: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_668 {
    TypeVar: __SlateRecord_pycore_ast_h_669,
    ParamSpec: __SlateRecord_pycore_ast_h_675,
    TypeVarTuple: __SlateRecord_pycore_ast_h_680,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_669 {
    name: *mut _object,
    bound: *mut _expr,
    default_value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_675 {
    name: *mut _object,
    default_value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_680 {
    name: *mut _object,
    default_value: *mut _expr,
}

// bpo-39562: CO_FUTURE_ and PyCF_ constants must be kept unique.
// PyCF_ constants can use bits from 0x0100 to 0x10000.
// CO_FUTURE_ constants use bits starting at 0x20000.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_compile_h_30 {
    cf_flags: i32,
    cf_feature_version: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _inittab {
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
}

fn _PyVectorcall_NARGS(n: u64) -> i64 {
    (n & !(_Py_STATIC_CAST!(u64, 1i32)
        << 8u64
            .wrapping_mul(core::mem::size_of::<u64>() as u64)
            .wrapping_sub(1))) as i64
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

#[unsafe(no_mangle)]
static mut PyFilter_Type: _typeobject = {
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
    __t0.tp_name = (b"filter\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_bltinmodule_c_497>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(filter_dealloc);
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
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(filter_doc.0) as *const i8 };
    __t0.tp_traverse = Some(filter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(filter_next);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(filter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
        >(PyType_GenericAlloc as *const ())
    };
    __t0.tp_new = Some(filter_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_GC_Del as *const (),
        )
    };
    __t0.tp_vectorcall = Some(filter_vectorcall);
    __t0
};

#[unsafe(no_mangle)]
static mut PyMap_Type: _typeobject = {
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
    __t0.tp_name = (b"map\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_bltinmodule_c_1361>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(map_dealloc);
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
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(map_doc.0) as *const i8 };
    __t0.tp_traverse = Some(map_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(map_next);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(map_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
        >(PyType_GenericAlloc as *const ())
    };
    __t0.tp_new = Some(map_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_GC_Del as *const (),
        )
    };
    __t0.tp_vectorcall = Some(map_vectorcall);
    __t0
};

#[unsafe(no_mangle)]
static mut PyZip_Type: _typeobject = {
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
    __t0.tp_name = (b"zip\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_bltinmodule_c_3089>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(zip_dealloc);
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
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(zip_doc.0) as *const i8 };
    __t0.tp_traverse = Some(zip_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(zip_next);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(zip_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
        >(PyType_GenericAlloc as *const ())
    };
    __t0.tp_new = Some(zip_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_GC_Del as *const (),
        )
    };
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_asdl_h_40 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _object; 1],
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
struct __SlateRecord_pycore_ast_h_67 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _stmt; 1],
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
struct __SlateRecord_pycore_ast_h_89 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _excepthandler; 1],
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
struct __SlateRecord_pycore_ast_h_118 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _alias; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_125 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _withitem; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_132 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _match_case; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_140 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _pattern; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_147 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _type_ignore; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_155 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _type_param; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct code_arena_st {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _mod {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_165,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _stmt {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_198,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
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
struct _excepthandler {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_538,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
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
struct _alias {
    name: *mut _object,
    asname: *mut _object,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _withitem {
    context_expr: *mut _expr,
    optional_vars: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _match_case {
    pattern: *mut _pattern,
    guard: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pattern {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_607,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _type_ignore {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_656,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _type_param {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_668,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_15 {
    value_and_backoff: u16,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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
struct _Py_unicode_state {
    fs_codec: _Py_unicode_fs_codec,
    ucnhash_capi: *mut __SlateRecord_pycore_ucnhash_h_29,
    ids: _Py_unicode_ids,
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
struct __SlateRecord_bltinmodule_c_h_52 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_279 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_439 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_520 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_884 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_962 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_1108 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_h_1177 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
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
    pub struct __SlateBits143U0 {
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
fn _PyGCHead_SET_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    next: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    unsafe {
        (*gc)._gc_next = next as u64;
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

// Fast inlined version of PyType_IS_GC()
fn _PyTuple_Recycle(op: *mut _object) {
    _PyTuple_RESET_HASH_CACHE!(op);
    if _PyObject_GC_IS_TRACKED!(op) == 0 {
        _PyObject_GC_TRACK__2!(op);
    }
}

fn PyCell_GetRef(cell: *mut __SlateRecord_cellobject_h_14) -> *mut _object {
    let res: *mut _object = Py_XNewRef!(unsafe { (*cell).ob_ref });
    res
}

static builtin___import____doc__: __SlateAlign16<[i8; 892]> = __SlateAlign16([
    95i8, 95i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 95i8, 95i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8,
    32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8,
    102i8, 114i8, 111i8, 109i8, 108i8, 105i8, 115i8, 116i8, 61i8, 40i8, 41i8, 44i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 108i8, 101i8, 118i8, 101i8, 108i8,
    61i8, 48i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 73i8, 109i8, 112i8, 111i8, 114i8, 116i8, 32i8,
    97i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 46i8, 10i8, 10i8, 66i8, 101i8, 99i8,
    97i8, 117i8, 115i8, 101i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 102i8, 117i8, 110i8, 99i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 109i8, 101i8, 97i8, 110i8, 116i8, 32i8,
    102i8, 111i8, 114i8, 32i8, 117i8, 115i8, 101i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 10i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8,
    114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 97i8, 110i8, 100i8, 32i8, 110i8, 111i8, 116i8, 32i8,
    102i8, 111i8, 114i8, 32i8, 103i8, 101i8, 110i8, 101i8, 114i8, 97i8, 108i8, 32i8, 117i8, 115i8,
    101i8, 44i8, 32i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8, 98i8, 101i8, 116i8, 116i8, 101i8,
    114i8, 32i8, 116i8, 111i8, 32i8, 117i8, 115i8, 101i8, 10i8, 105i8, 109i8, 112i8, 111i8, 114i8,
    116i8, 108i8, 105i8, 98i8, 46i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 95i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 40i8, 41i8, 32i8, 116i8, 111i8, 32i8, 112i8, 114i8, 111i8, 103i8,
    114i8, 97i8, 109i8, 109i8, 97i8, 116i8, 105i8, 99i8, 97i8, 108i8, 108i8, 121i8, 32i8, 105i8,
    109i8, 112i8, 111i8, 114i8, 116i8, 32i8, 97i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8,
    32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 111i8,
    110i8, 108i8, 121i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 100i8, 101i8,
    116i8, 101i8, 114i8, 109i8, 105i8, 110i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8,
    110i8, 116i8, 101i8, 120i8, 116i8, 59i8, 10i8, 116i8, 104i8, 101i8, 121i8, 32i8, 97i8, 114i8,
    101i8, 32i8, 110i8, 111i8, 116i8, 32i8, 109i8, 111i8, 100i8, 105i8, 102i8, 105i8, 101i8, 100i8,
    46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 32i8, 97i8,
    114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 117i8, 110i8, 117i8,
    115i8, 101i8, 100i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    108i8, 105i8, 115i8, 116i8, 10i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8, 32i8, 98i8, 101i8,
    32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 111i8, 102i8, 32i8, 110i8, 97i8, 109i8,
    101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 101i8, 109i8, 117i8, 108i8, 97i8, 116i8, 101i8, 32i8,
    96i8, 96i8, 102i8, 114i8, 111i8, 109i8, 32i8, 110i8, 97i8, 109i8, 101i8, 32i8, 105i8, 109i8,
    112i8, 111i8, 114i8, 116i8, 32i8, 46i8, 46i8, 46i8, 96i8, 96i8, 44i8, 32i8, 111i8, 114i8, 32i8,
    97i8, 110i8, 10i8, 101i8, 109i8, 112i8, 116i8, 121i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8,
    116i8, 111i8, 32i8, 101i8, 109i8, 117i8, 108i8, 97i8, 116i8, 101i8, 32i8, 96i8, 96i8, 105i8,
    109i8, 112i8, 111i8, 114i8, 116i8, 32i8, 110i8, 97i8, 109i8, 101i8, 96i8, 96i8, 46i8, 10i8,
    87i8, 104i8, 101i8, 110i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 105i8, 110i8, 103i8,
    32i8, 97i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 97i8, 32i8, 112i8, 97i8, 99i8, 107i8, 97i8, 103i8, 101i8, 44i8, 32i8, 110i8, 111i8,
    116i8, 101i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 95i8, 95i8, 105i8, 109i8, 112i8, 111i8,
    114i8, 116i8, 95i8, 95i8, 40i8, 39i8, 65i8, 46i8, 66i8, 39i8, 44i8, 32i8, 46i8, 46i8, 46i8,
    41i8, 10i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 112i8, 97i8, 99i8, 107i8,
    97i8, 103i8, 101i8, 32i8, 65i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 102i8, 114i8, 111i8,
    109i8, 108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8,
    44i8, 32i8, 98i8, 117i8, 116i8, 32i8, 105i8, 116i8, 115i8, 32i8, 115i8, 117i8, 98i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 66i8, 32i8, 119i8, 104i8, 101i8, 110i8, 10i8, 102i8,
    114i8, 111i8, 109i8, 108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 108i8,
    101i8, 118i8, 101i8, 108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    105i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 100i8, 101i8, 116i8,
    101i8, 114i8, 109i8, 105i8, 110i8, 101i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8,
    114i8, 10i8, 116i8, 111i8, 32i8, 112i8, 101i8, 114i8, 102i8, 111i8, 114i8, 109i8, 32i8, 97i8,
    98i8, 115i8, 111i8, 108i8, 117i8, 116i8, 101i8, 32i8, 111i8, 114i8, 32i8, 114i8, 101i8, 108i8,
    97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 115i8, 58i8,
    32i8, 48i8, 32i8, 105i8, 115i8, 32i8, 97i8, 98i8, 115i8, 111i8, 108i8, 117i8, 116i8, 101i8,
    44i8, 32i8, 119i8, 104i8, 105i8, 108i8, 101i8, 32i8, 97i8, 32i8, 112i8, 111i8, 115i8, 105i8,
    116i8, 105i8, 118i8, 101i8, 10i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 105i8, 115i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8,
    32i8, 112i8, 97i8, 114i8, 101i8, 110i8, 116i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8,
    111i8, 114i8, 105i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 115i8, 101i8, 97i8, 114i8, 99i8,
    104i8, 32i8, 114i8, 101i8, 108i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 116i8, 111i8, 32i8,
    116i8, 104i8, 101i8, 10i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin___import__")]
extern "C-unwind" fn builtin___import__(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut level: i32 = unsafe { core::mem::zeroed() };
    let mut fromlist: *mut _object = unsafe { core::mem::zeroed() };
    let mut locals: *mut _object = unsafe { core::mem::zeroed() };
    let mut globals: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v12: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 5]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_14: *mut [*mut _object; 5] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 5];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v12 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v12 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v12).wrapping_sub(1);
            globals = core::ptr::null_mut::<_object>();
            locals = core::ptr::null_mut::<_object>();
            fromlist = core::ptr::null_mut::<_object>();
            level = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__65),
                1i32,
                5i32,
                0i32,
                0i32,
                (*__slate_slot_14).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    name = *args.wrapping_offset(0 as isize);
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                            globals = *args.wrapping_offset(1 as isize);
                            let __v5: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v5;
                            if __v5 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *args.wrapping_offset(2 as isize) }.is_null() {
                            locals = *args.wrapping_offset(2 as isize);
                            let __v3: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v3;
                            if __v3 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *args.wrapping_offset(3 as isize) }.is_null() {
                            fromlist = *args.wrapping_offset(3 as isize);
                            let __v1: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v1;
                            if __v1 == 0 {
                                break '__join_1;
                            }
                        }
                        level = PyLong_AsInt(*args.wrapping_offset(4 as isize));
                        if level == 1i32.wrapping_neg() {
                            __v0 = !PyErr_Occurred().is_null();
                        } else {
                            __v0 = false;
                        }
                        if __v0 {
                            break '__join_0;
                        }
                    }
                }
                return_value =
                    builtin___import___impl(module, name, globals, locals, fromlist, level);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__52: __SlateRecord_bltinmodule_c_h_52 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_52 = unsafe { core::mem::zeroed() };
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
                    ._py_name
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
                    ._py_globals
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
                    ._py_locals
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
                    ._py_fromlist
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
                    ._py_level
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__64: __SlateAlign16<[*const i8; 6]> = __SlateAlign16([
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"globals\0".as_ptr() as *mut i8) as *const i8,
    (b"locals\0".as_ptr() as *mut i8) as *const i8,
    (b"fromlist\0".as_ptr() as *mut i8) as *const i8,
    (b"level\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__65: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__64.0) as *const *const i8 };
    __t0.fname = (b"__import__\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__52.ob_base.ob_base) };
    __t0
};

static builtin_abs__doc__: __SlateAlign16<[i8; 66]> = __SlateAlign16([
    97i8, 98i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 120i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 98i8, 115i8, 111i8, 108i8, 117i8, 116i8, 101i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8,
    114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 0i8,
]);

static builtin_all__doc__: __SlateAlign16<[i8; 135]> = __SlateAlign16([
    97i8, 108i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8,
    102i8, 32i8, 98i8, 111i8, 111i8, 108i8, 40i8, 120i8, 41i8, 32i8, 105i8, 115i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 108i8, 108i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 115i8, 32i8, 120i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8,
    115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 46i8, 0i8,
]);

static builtin_any__doc__: __SlateAlign16<[i8; 129]> = __SlateAlign16([
    97i8, 110i8, 121i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8,
    102i8, 32i8, 98i8, 111i8, 111i8, 108i8, 40i8, 120i8, 41i8, 32i8, 105i8, 115i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 110i8, 121i8, 32i8, 120i8, 32i8,
    105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8,
    44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8,
    46i8, 0i8,
]);

static builtin_ascii__doc__: __SlateAlign16<[i8; 329]> = __SlateAlign16([
    97i8, 115i8, 99i8, 105i8, 105i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 45i8, 111i8,
    110i8, 108i8, 121i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 65i8, 115i8, 32i8, 114i8, 101i8, 112i8, 114i8, 40i8,
    41i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 115i8, 116i8,
    114i8, 105i8, 110i8, 103i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 105i8, 110i8,
    103i8, 32i8, 97i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 10i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    44i8, 32i8, 98i8, 117i8, 116i8, 32i8, 101i8, 115i8, 99i8, 97i8, 112i8, 101i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 110i8, 111i8, 110i8, 45i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8, 99i8, 104i8,
    97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 101i8, 100i8, 32i8, 98i8, 121i8, 10i8, 114i8, 101i8, 112i8, 114i8, 40i8, 41i8, 32i8,
    117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 92i8, 92i8, 120i8, 44i8, 32i8, 92i8, 92i8, 117i8,
    32i8, 111i8, 114i8, 32i8, 92i8, 92i8, 85i8, 32i8, 101i8, 115i8, 99i8, 97i8, 112i8, 101i8,
    115i8, 46i8, 32i8, 84i8, 104i8, 105i8, 115i8, 32i8, 103i8, 101i8, 110i8, 101i8, 114i8, 97i8,
    116i8, 101i8, 115i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 115i8,
    105i8, 109i8, 105i8, 108i8, 97i8, 114i8, 10i8, 116i8, 111i8, 32i8, 116i8, 104i8, 97i8, 116i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 114i8,
    101i8, 112i8, 114i8, 40i8, 41i8, 32i8, 105i8, 110i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 50i8, 46i8, 0i8,
]);

static builtin_bin__doc__: __SlateAlign16<[i8; 127]> = __SlateAlign16([
    98i8, 105i8, 110i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 110i8,
    117i8, 109i8, 98i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 105i8, 110i8, 97i8,
    114i8, 121i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    103i8, 101i8, 114i8, 46i8, 10i8, 10i8, 32i8, 32i8, 32i8, 62i8, 62i8, 62i8, 32i8, 98i8, 105i8,
    110i8, 40i8, 50i8, 55i8, 57i8, 54i8, 50i8, 48i8, 50i8, 41i8, 10i8, 32i8, 32i8, 32i8, 39i8,
    48i8, 98i8, 49i8, 48i8, 49i8, 48i8, 49i8, 48i8, 49i8, 48i8, 49i8, 48i8, 49i8, 48i8, 49i8, 48i8,
    49i8, 48i8, 49i8, 48i8, 49i8, 48i8, 49i8, 48i8, 39i8, 0i8,
]);

static builtin_callable__doc__: __SlateAlign16<[i8; 186]> = __SlateAlign16([
    99i8, 97i8, 108i8, 108i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8,
    114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 105i8,
    115i8, 32i8, 99i8, 97i8, 108i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8, 40i8, 105i8, 46i8,
    101i8, 46i8, 44i8, 32i8, 115i8, 111i8, 109i8, 101i8, 32i8, 107i8, 105i8, 110i8, 100i8, 32i8,
    111i8, 102i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 41i8, 46i8, 10i8,
    10i8, 78i8, 111i8, 116i8, 101i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 108i8, 97i8,
    115i8, 115i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 97i8,
    98i8, 108i8, 101i8, 44i8, 32i8, 97i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 105i8, 110i8,
    115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 99i8, 108i8, 97i8,
    115i8, 115i8, 101i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 10i8, 95i8, 95i8,
    99i8, 97i8, 108i8, 108i8, 95i8, 95i8, 40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8,
    100i8, 46i8, 0i8,
]);

static builtin_format__doc__: __SlateAlign16<[i8; 362]> = __SlateAlign16([
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    44i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 102i8, 111i8, 114i8, 109i8, 97i8,
    116i8, 95i8, 115i8, 112i8, 101i8, 99i8, 61i8, 39i8, 39i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 121i8, 112i8, 101i8,
    40i8, 118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 46i8, 95i8, 95i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 95i8, 95i8, 40i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 95i8, 115i8, 112i8, 101i8, 99i8, 41i8, 10i8, 10i8, 77i8, 97i8,
    110i8, 121i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8, 45i8, 105i8, 110i8, 32i8, 116i8, 121i8,
    112i8, 101i8, 115i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 115i8, 112i8, 101i8, 99i8, 32i8, 97i8, 99i8,
    99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8,
    10i8, 70i8, 111i8, 114i8, 109i8, 97i8, 116i8, 32i8, 83i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 99i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 77i8, 105i8, 110i8, 105i8, 45i8, 108i8,
    97i8, 110i8, 103i8, 117i8, 97i8, 103i8, 101i8, 46i8, 32i8, 83i8, 101i8, 101i8, 32i8, 104i8,
    101i8, 108i8, 112i8, 40i8, 39i8, 70i8, 79i8, 82i8, 77i8, 65i8, 84i8, 84i8, 73i8, 78i8, 71i8,
    39i8, 41i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 121i8, 112i8, 101i8, 40i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 41i8, 32i8, 100i8, 111i8, 101i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8,
    115i8, 117i8, 112i8, 112i8, 108i8, 121i8, 32i8, 97i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8,
    100i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 95i8, 95i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 95i8, 95i8, 10i8, 97i8, 110i8, 100i8, 32i8, 102i8, 111i8, 114i8, 109i8, 97i8,
    116i8, 95i8, 115i8, 112i8, 101i8, 99i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8,
    121i8, 44i8, 32i8, 116i8, 104i8, 101i8, 110i8, 32i8, 115i8, 116i8, 114i8, 40i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 41i8, 32i8, 105i8, 115i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    101i8, 100i8, 46i8, 10i8, 83i8, 101i8, 101i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 104i8,
    101i8, 108i8, 112i8, 40i8, 39i8, 83i8, 80i8, 69i8, 67i8, 73i8, 65i8, 76i8, 77i8, 69i8, 84i8,
    72i8, 79i8, 68i8, 83i8, 39i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_format")]
extern "C-unwind" fn builtin_format(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut format_spec: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            format_spec = core::ptr::null_mut::<_object>();
            if _PyArg_CheckPositional!(c"format".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
                value = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                        _PyArg_BadArgument(
                            c"format".as_ptr(),
                            c"argument 2".as_ptr(),
                            c"str".as_ptr(),
                            *args.wrapping_offset(1 as isize),
                        );
                        break '__join_0;
                    } else {
                        format_spec = *args.wrapping_offset(1 as isize);
                    }
                }
                return_value = builtin_format_impl(module, value, format_spec);
            }
        }
        return return_value;
    }
}

static builtin_chr__doc__: __SlateAlign16<[i8; 100]> = __SlateAlign16([
    99i8, 104i8, 114i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 105i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 97i8, 32i8, 85i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 115i8, 116i8, 114i8,
    105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 111i8, 110i8, 101i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 111i8, 114i8,
    100i8, 105i8, 110i8, 97i8, 108i8, 32i8, 105i8, 59i8, 32i8, 48i8, 32i8, 60i8, 61i8, 32i8, 105i8,
    32i8, 60i8, 61i8, 32i8, 48i8, 120i8, 49i8, 48i8, 102i8, 102i8, 102i8, 102i8, 46i8, 0i8,
]);

static builtin_compile__doc__: __SlateAlign16<[i8; 826]> = __SlateAlign16([
    99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 44i8, 32i8,
    102i8, 105i8, 108i8, 101i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 109i8, 111i8, 100i8, 101i8,
    44i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 61i8, 48i8, 44i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 100i8, 111i8, 110i8, 116i8, 95i8, 105i8, 110i8, 104i8, 101i8, 114i8,
    105i8, 116i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 44i8, 32i8, 111i8, 112i8, 116i8, 105i8,
    109i8, 105i8, 122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 42i8, 44i8, 32i8, 95i8, 102i8, 101i8,
    97i8, 116i8, 117i8, 114i8, 101i8, 95i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 61i8,
    45i8, 49i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 109i8, 112i8, 105i8, 108i8, 101i8,
    32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 97i8,
    32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 116i8,
    104i8, 97i8, 116i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 101i8, 120i8, 101i8,
    99i8, 117i8, 116i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 101i8, 120i8, 101i8, 99i8, 40i8,
    41i8, 32i8, 111i8, 114i8, 32i8, 101i8, 118i8, 97i8, 108i8, 40i8, 41i8, 46i8, 10i8, 10i8, 84i8,
    104i8, 101i8, 32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 32i8, 99i8, 111i8, 100i8, 101i8,
    32i8, 109i8, 97i8, 121i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8,
    32i8, 97i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    111i8, 114i8, 10i8, 101i8, 120i8, 112i8, 114i8, 101i8, 115i8, 115i8, 105i8, 111i8, 110i8, 46i8,
    10i8, 84i8, 104i8, 101i8, 32i8, 102i8, 105i8, 108i8, 101i8, 110i8, 97i8, 109i8, 101i8, 32i8,
    119i8, 105i8, 108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 114i8, 117i8, 110i8, 45i8, 116i8, 105i8, 109i8, 101i8, 32i8, 101i8, 114i8,
    114i8, 111i8, 114i8, 32i8, 109i8, 101i8, 115i8, 115i8, 97i8, 103i8, 101i8, 115i8, 46i8, 10i8,
    84i8, 104i8, 101i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8,
    98i8, 101i8, 32i8, 39i8, 101i8, 120i8, 101i8, 99i8, 39i8, 32i8, 116i8, 111i8, 32i8, 99i8,
    111i8, 109i8, 112i8, 105i8, 108i8, 101i8, 32i8, 97i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 39i8, 115i8, 105i8, 110i8, 103i8, 108i8, 101i8, 39i8, 32i8, 116i8, 111i8,
    32i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 101i8, 32i8, 97i8, 10i8, 115i8, 105i8, 110i8,
    103i8, 108i8, 101i8, 32i8, 40i8, 105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8, 105i8,
    118i8, 101i8, 41i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 44i8,
    32i8, 111i8, 114i8, 32i8, 39i8, 101i8, 118i8, 97i8, 108i8, 39i8, 32i8, 116i8, 111i8, 32i8,
    99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 101i8, 32i8, 97i8, 110i8, 32i8, 101i8, 120i8, 112i8,
    114i8, 101i8, 115i8, 115i8, 105i8, 111i8, 110i8, 46i8, 10i8, 84i8, 104i8, 101i8, 32i8, 102i8,
    108i8, 97i8, 103i8, 115i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 44i8,
    32i8, 105i8, 102i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 44i8, 32i8, 99i8,
    111i8, 110i8, 116i8, 114i8, 111i8, 108i8, 115i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8,
    102i8, 117i8, 116i8, 117i8, 114i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8, 101i8,
    110i8, 116i8, 115i8, 10i8, 105i8, 110i8, 102i8, 108i8, 117i8, 101i8, 110i8, 99i8, 101i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 100i8, 101i8, 46i8,
    10i8, 84i8, 104i8, 101i8, 32i8, 100i8, 111i8, 110i8, 116i8, 95i8, 105i8, 110i8, 104i8, 101i8,
    114i8, 105i8, 116i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 44i8, 32i8,
    105i8, 102i8, 32i8, 116i8, 114i8, 117i8, 101i8, 44i8, 32i8, 115i8, 116i8, 111i8, 112i8, 115i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 97i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 105i8, 110i8, 104i8, 101i8, 114i8, 105i8, 116i8, 105i8, 110i8, 103i8, 10i8,
    116i8, 104i8, 101i8, 32i8, 101i8, 102i8, 102i8, 101i8, 99i8, 116i8, 115i8, 32i8, 111i8, 102i8,
    32i8, 97i8, 110i8, 121i8, 32i8, 102i8, 117i8, 116i8, 117i8, 114i8, 101i8, 32i8, 115i8, 116i8,
    97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 110i8, 32i8, 101i8, 102i8,
    102i8, 101i8, 99i8, 116i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8,
    100i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 105i8, 110i8, 103i8, 10i8, 99i8, 111i8, 109i8,
    112i8, 105i8, 108i8, 101i8, 59i8, 32i8, 105i8, 102i8, 32i8, 97i8, 98i8, 115i8, 101i8, 110i8,
    116i8, 32i8, 111i8, 114i8, 32i8, 102i8, 97i8, 108i8, 115i8, 101i8, 32i8, 116i8, 104i8, 101i8,
    115i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8,
    100i8, 111i8, 32i8, 105i8, 110i8, 102i8, 108i8, 117i8, 101i8, 110i8, 99i8, 101i8, 32i8, 116i8,
    104i8, 101i8, 10i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    44i8, 32i8, 105i8, 110i8, 32i8, 97i8, 100i8, 100i8, 105i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    116i8, 111i8, 32i8, 97i8, 110i8, 121i8, 32i8, 102i8, 101i8, 97i8, 116i8, 117i8, 114i8, 101i8,
    115i8, 32i8, 101i8, 120i8, 112i8, 108i8, 105i8, 99i8, 105i8, 116i8, 108i8, 121i8, 32i8, 115i8,
    112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_compile")]
extern "C-unwind" fn builtin_compile(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut mode_length: i64 = unsafe { core::mem::zeroed() };
    let mut feature_version: i32 = unsafe { core::mem::zeroed() };
    let mut optimize: i32 = unsafe { core::mem::zeroed() };
    let mut dont_inherit: i32 = unsafe { core::mem::zeroed() };
    let mut flags: i32 = unsafe { core::mem::zeroed() };
    let mut mode: *const i8 = unsafe { core::mem::zeroed() };
    let mut filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut source: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v17: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 7]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_19: *mut [*mut _object; 7] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 7];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v17 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v17 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v17).wrapping_sub(3);
            filename = core::ptr::null_mut::<_object>();
            flags = 0;
            dont_inherit = 0;
            optimize = 1i32.wrapping_neg();
            feature_version = 1i32.wrapping_neg();
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__292),
                3i32,
                6i32,
                0i32,
                0i32,
                (*__slate_slot_19).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                source = *args.wrapping_offset(0 as isize);
                if PyUnicode_FSDecoder(
                    *args.wrapping_offset(1 as isize),
                    core::ptr::addr_of_mut!(filename) as *mut (),
                ) != 0
                {
                    if PyUnicode_Check!(unsafe { *args.wrapping_offset(2 as isize) }) == 0 {
                        _PyArg_BadArgument(
                            c"compile".as_ptr(),
                            c"argument 'mode'".as_ptr(),
                            c"str".as_ptr(),
                            *args.wrapping_offset(2 as isize),
                        );
                    } else {
                        mode = PyUnicode_AsUTF8AndSize(
                            *args.wrapping_offset(2 as isize),
                            core::ptr::addr_of_mut!(mode_length),
                        );
                        if !mode.is_null() {
                            if (strlen(mode as *const core::ffi::c_char) as u64)
                                != (mode_length as u64)
                            {
                                PyErr_SetString(
                                    PyExc_ValueError,
                                    c"embedded null character".as_ptr(),
                                );
                            } else {
                                '__join_6: {
                                    if noptargs != 0 {
                                        if !unsafe { *args.wrapping_offset(3 as isize) }.is_null() {
                                            flags = PyLong_AsInt(*args.wrapping_offset(3 as isize));
                                            if flags == 1i32.wrapping_neg() {
                                                __v8 = !PyErr_Occurred().is_null();
                                            } else {
                                                __v8 = false;
                                            }
                                            if __v8 {
                                                break '__join_0;
                                            } else {
                                                let __v6: i64 = noptargs.wrapping_sub(1);
                                                noptargs = __v6;
                                                if __v6 == 0 {
                                                    break '__join_6;
                                                }
                                            }
                                        }
                                        if !unsafe { *args.wrapping_offset(4 as isize) }.is_null() {
                                            dont_inherit =
                                                PyObject_IsTrue(*args.wrapping_offset(4 as isize));
                                            if dont_inherit < 0 {
                                                break '__join_0;
                                            } else {
                                                let __v4: i64 = noptargs.wrapping_sub(1);
                                                noptargs = __v4;
                                                if __v4 == 0 {
                                                    break '__join_6;
                                                }
                                            }
                                        }
                                        if !unsafe { *args.wrapping_offset(5 as isize) }.is_null() {
                                            optimize =
                                                PyLong_AsInt(*args.wrapping_offset(5 as isize));
                                            if optimize == 1i32.wrapping_neg() {
                                                __v3 = !PyErr_Occurred().is_null();
                                            } else {
                                                __v3 = false;
                                            }
                                            if __v3 {
                                                break '__join_0;
                                            } else {
                                                noptargs = noptargs.wrapping_sub(1);
                                            }
                                        }
                                    }
                                }
                                if noptargs != 0 {
                                    feature_version =
                                        PyLong_AsInt(*args.wrapping_offset(6 as isize));
                                    if feature_version == 1i32.wrapping_neg() {
                                        __v0 = !PyErr_Occurred().is_null();
                                    } else {
                                        __v0 = false;
                                    }
                                    if __v0 {
                                        break '__join_0;
                                    }
                                }
                                return_value = builtin_compile_impl(
                                    module,
                                    source,
                                    filename,
                                    mode,
                                    flags,
                                    dont_inherit,
                                    optimize,
                                    feature_version,
                                );
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(filename); // Cleanup for filename
        return return_value;
    }
}

static mut _kwtuple__279: __SlateRecord_bltinmodule_c_h_279 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_279 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 7i64,
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
                    ._py_filename
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
                    ._py_mode
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
                    ._py_flags
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
                    ._py_dont_inherit
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
                    ._py_optimize
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
                    ._py__feature_version
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__291: __SlateAlign16<[*const i8; 8]> = __SlateAlign16([
    (b"source\0".as_ptr() as *mut i8) as *const i8,
    (b"filename\0".as_ptr() as *mut i8) as *const i8,
    (b"mode\0".as_ptr() as *mut i8) as *const i8,
    (b"flags\0".as_ptr() as *mut i8) as *const i8,
    (b"dont_inherit\0".as_ptr() as *mut i8) as *const i8,
    (b"optimize\0".as_ptr() as *mut i8) as *const i8,
    (b"_feature_version\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__292: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__291.0) as *const *const i8 };
    __t0.fname = (b"compile\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__279.ob_base.ob_base) };
    __t0
};

static builtin_divmod__doc__: __SlateAlign16<[i8; 89]> = __SlateAlign16([
    100i8, 105i8, 118i8, 109i8, 111i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    44i8, 32i8, 120i8, 44i8, 32i8, 121i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 117i8, 112i8,
    108i8, 101i8, 32i8, 40i8, 120i8, 47i8, 47i8, 121i8, 44i8, 32i8, 120i8, 37i8, 121i8, 41i8, 46i8,
    32i8, 32i8, 73i8, 110i8, 118i8, 97i8, 114i8, 105i8, 97i8, 110i8, 116i8, 58i8, 32i8, 100i8,
    105i8, 118i8, 42i8, 121i8, 32i8, 43i8, 32i8, 109i8, 111i8, 100i8, 32i8, 61i8, 61i8, 32i8,
    120i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_divmod")]
extern "C-unwind" fn builtin_divmod(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"divmod".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            let x: *mut _object = *args.wrapping_offset(0 as isize);
            let y: *mut _object = *args.wrapping_offset(1 as isize);
            return_value = builtin_divmod_impl(module, x, y);
        }
        return return_value;
    }
}

static builtin_eval__doc__: __SlateAlign16<[i8; 383]> = __SlateAlign16([
    101i8, 118i8, 97i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 103i8, 108i8, 111i8,
    98i8, 97i8, 108i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 108i8, 111i8, 99i8,
    97i8, 108i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 69i8,
    118i8, 97i8, 108i8, 117i8, 97i8, 116i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8,
    118i8, 101i8, 110i8, 32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 32i8, 105i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 32i8, 111i8, 102i8,
    32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 108i8,
    111i8, 99i8, 97i8, 108i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 115i8, 111i8,
    117i8, 114i8, 99i8, 101i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 97i8, 32i8,
    115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8,
    101i8, 110i8, 116i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 101i8, 120i8, 112i8, 114i8, 101i8, 115i8, 115i8, 105i8, 111i8, 110i8, 10i8, 111i8,
    114i8, 32i8, 97i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 97i8, 115i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8,
    98i8, 121i8, 32i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 101i8, 40i8, 41i8, 46i8, 10i8,
    84i8, 104i8, 101i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 109i8, 117i8,
    115i8, 116i8, 32i8, 98i8, 101i8, 32i8, 97i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 97i8, 114i8, 121i8, 32i8, 97i8, 110i8, 100i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8,
    115i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 97i8, 110i8, 121i8, 32i8, 109i8, 97i8,
    112i8, 112i8, 105i8, 110i8, 103i8, 44i8, 10i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8,
    114i8, 101i8, 110i8, 116i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 46i8, 10i8, 73i8, 102i8, 32i8,
    111i8, 110i8, 108i8, 121i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 105i8,
    115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8,
    115i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8,
    105i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_eval")]
extern "C-unwind" fn builtin_eval(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut locals: *mut _object = unsafe { core::mem::zeroed() };
    let mut globals: *mut _object = unsafe { core::mem::zeroed() };
    let mut source: *mut _object = unsafe { core::mem::zeroed() };
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
        noptargs = nargs.wrapping_add(__v5).wrapping_sub(1);
        globals = core::ptr::addr_of_mut!(_Py_NoneStruct);
        locals = core::ptr::addr_of_mut!(_Py_NoneStruct);
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__452),
            1i32,
            3i32,
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
                source = *args.wrapping_offset(0 as isize);
                if noptargs != 0 {
                    if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                        globals = *args.wrapping_offset(1 as isize);
                        let __v0: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v0;
                        if __v0 == 0 {
                            break '__join_1;
                        }
                    }
                    locals = *args.wrapping_offset(2 as isize);
                }
            }
            return_value = builtin_eval_impl(module, source, globals, locals);
        }
        return return_value;
    }
}

static mut _kwtuple__439: __SlateRecord_bltinmodule_c_h_439 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_439 = unsafe { core::mem::zeroed() };
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
                    ._py_globals
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
                    ._py_locals
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__451: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"globals\0".as_ptr() as *mut i8) as *const i8,
    (b"locals\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__452: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__451.0) as *const *const i8 };
    __t0.fname = (b"eval\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__439.ob_base.ob_base) };
    __t0
};

static builtin_exec__doc__: __SlateAlign16<[i8; 538]> = __SlateAlign16([
    101i8, 120i8, 101i8, 99i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 103i8, 108i8, 111i8,
    98i8, 97i8, 108i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 108i8, 111i8, 99i8,
    97i8, 108i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 42i8, 44i8, 32i8, 99i8, 108i8,
    111i8, 115i8, 117i8, 114i8, 101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 69i8, 120i8, 101i8, 99i8, 117i8, 116i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 32i8, 105i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 32i8,
    111i8, 102i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8,
    115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8,
    97i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8,
    115i8, 101i8, 110i8, 116i8, 105i8, 110i8, 103i8, 32i8, 111i8, 110i8, 101i8, 32i8, 111i8, 114i8,
    32i8, 109i8, 111i8, 114i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 115i8,
    116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 10i8, 111i8, 114i8, 32i8, 97i8,
    32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 97i8,
    115i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8,
    99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 101i8, 40i8, 41i8, 46i8, 10i8, 84i8, 104i8, 101i8,
    32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8,
    98i8, 101i8, 32i8, 97i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8,
    121i8, 32i8, 97i8, 110i8, 100i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 32i8, 99i8,
    97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 97i8, 110i8, 121i8, 32i8, 109i8, 97i8, 112i8, 112i8,
    105i8, 110i8, 103i8, 44i8, 10i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 105i8, 110i8,
    103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8,
    110i8, 116i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 46i8, 10i8, 73i8, 102i8, 32i8, 111i8, 110i8,
    108i8, 121i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 32i8, 105i8, 115i8, 32i8,
    103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 32i8,
    100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 105i8, 116i8,
    46i8, 10i8, 84i8, 104i8, 101i8, 32i8, 99i8, 108i8, 111i8, 115i8, 117i8, 114i8, 101i8, 32i8,
    109i8, 117i8, 115i8, 116i8, 32i8, 98i8, 101i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8, 108i8,
    101i8, 32i8, 111i8, 102i8, 32i8, 99i8, 101i8, 108i8, 108i8, 118i8, 97i8, 114i8, 115i8, 44i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 99i8, 97i8, 110i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8,
    98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 10i8, 119i8, 104i8, 101i8, 110i8, 32i8, 115i8,
    111i8, 117i8, 114i8, 99i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 32i8, 99i8, 111i8, 100i8,
    101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 114i8, 101i8, 113i8, 117i8, 105i8,
    114i8, 105i8, 110i8, 103i8, 32i8, 101i8, 120i8, 97i8, 99i8, 116i8, 108i8, 121i8, 32i8, 116i8,
    104i8, 97i8, 116i8, 32i8, 109i8, 97i8, 110i8, 121i8, 32i8, 99i8, 101i8, 108i8, 108i8, 118i8,
    97i8, 114i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_exec")]
extern "C-unwind" fn builtin_exec(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut closure: *mut _object = unsafe { core::mem::zeroed() };
    let mut locals: *mut _object = unsafe { core::mem::zeroed() };
    let mut globals: *mut _object = unsafe { core::mem::zeroed() };
    let mut source: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v8: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 4]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_10: *mut [*mut _object; 4] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 4];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v8 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v8 = 0;
        }
        noptargs = nargs.wrapping_add(__v8).wrapping_sub(1);
        globals = core::ptr::addr_of_mut!(_Py_NoneStruct);
        locals = core::ptr::addr_of_mut!(_Py_NoneStruct);
        closure = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__533),
            1i32,
            3i32,
            0i32,
            0i32,
            (*__slate_slot_10).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            '__join_3: {
                source = *args.wrapping_offset(0 as isize);
                if noptargs != 0 {
                    if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                        globals = *args.wrapping_offset(1 as isize);
                        let __v2: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v2;
                        if __v2 == 0 {
                            break '__join_3;
                        }
                    }
                    if !unsafe { *args.wrapping_offset(2 as isize) }.is_null() {
                        locals = *args.wrapping_offset(2 as isize);
                        noptargs = noptargs.wrapping_sub(1);
                    }
                }
            }
            if noptargs != 0 {
                closure = *args.wrapping_offset(3 as isize);
            }
            return_value = builtin_exec_impl(module, source, globals, locals, closure);
        }
        return return_value;
    }
}

static mut _kwtuple__520: __SlateRecord_bltinmodule_c_h_520 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_520 = unsafe { core::mem::zeroed() };
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
                    ._py_globals
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
                    ._py_locals
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
                    ._py_closure
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__532: __SlateAlign16<[*const i8; 5]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"globals\0".as_ptr() as *mut i8) as *const i8,
    (b"locals\0".as_ptr() as *mut i8) as *const i8,
    (b"closure\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__533: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__532.0) as *const *const i8 };
    __t0.fname = (b"exec\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__520.ob_base.ob_base) };
    __t0
};

static builtin_globals__doc__: __SlateAlign16<[i8; 200]> = __SlateAlign16([
    103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    114i8, 121i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 99i8,
    111i8, 112i8, 101i8, 39i8, 115i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 32i8, 118i8,
    97i8, 114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 115i8, 46i8, 10i8, 10i8, 78i8, 79i8, 84i8, 69i8,
    58i8, 32i8, 85i8, 112i8, 100i8, 97i8, 116i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 116i8,
    104i8, 105i8, 115i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8,
    32i8, 42i8, 119i8, 105i8, 108i8, 108i8, 42i8, 32i8, 97i8, 102i8, 102i8, 101i8, 99i8, 116i8,
    32i8, 110i8, 97i8, 109i8, 101i8, 32i8, 108i8, 111i8, 111i8, 107i8, 117i8, 112i8, 115i8, 32i8,
    105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 10i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8,
    32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 32i8, 115i8, 99i8, 111i8, 112i8, 101i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 118i8, 105i8, 99i8, 101i8, 45i8, 118i8, 101i8, 114i8, 115i8, 97i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_globals")]
extern "C-unwind" fn builtin_globals(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    builtin_globals_impl(module)
}

static builtin_hasattr__doc__: __SlateAlign16<[i8; 172]> = __SlateAlign16([
    104i8, 97i8, 115i8, 97i8, 116i8, 116i8, 114i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 119i8,
    104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 32i8, 104i8, 97i8, 115i8, 32i8, 97i8, 110i8, 32i8, 97i8, 116i8, 116i8,
    114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 110i8, 97i8, 109i8, 101i8, 46i8, 10i8,
    10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8, 100i8, 111i8, 110i8, 101i8, 32i8,
    98i8, 121i8, 32i8, 99i8, 97i8, 108i8, 108i8, 105i8, 110i8, 103i8, 32i8, 103i8, 101i8, 116i8,
    97i8, 116i8, 116i8, 114i8, 40i8, 111i8, 98i8, 106i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8,
    41i8, 32i8, 97i8, 110i8, 100i8, 32i8, 99i8, 97i8, 116i8, 99i8, 104i8, 105i8, 110i8, 103i8,
    32i8, 65i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 69i8, 114i8, 114i8, 111i8,
    114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_hasattr")]
extern "C-unwind" fn builtin_hasattr(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"hasattr".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            let obj: *mut _object = *args.wrapping_offset(0 as isize);
            let name: *mut _object = *args.wrapping_offset(1 as isize);
            return_value = builtin_hasattr_impl(module, obj, name);
        }
        return return_value;
    }
}

static builtin_id__doc__: __SlateAlign16<[i8; 174]> = __SlateAlign16([
    105i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 111i8, 98i8,
    106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 100i8, 101i8, 110i8, 116i8, 105i8, 116i8, 121i8,
    32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8,
    10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8, 103i8, 117i8, 97i8, 114i8,
    97i8, 110i8, 116i8, 101i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 117i8,
    110i8, 105i8, 113i8, 117i8, 101i8, 32i8, 97i8, 109i8, 111i8, 110i8, 103i8, 32i8, 115i8, 105i8,
    109i8, 117i8, 108i8, 116i8, 97i8, 110i8, 101i8, 111i8, 117i8, 115i8, 108i8, 121i8, 32i8, 101i8,
    120i8, 105i8, 115i8, 116i8, 105i8, 110i8, 103i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    115i8, 46i8, 10i8, 40i8, 67i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 117i8, 115i8,
    101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 39i8,
    115i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 100i8, 100i8, 114i8, 101i8,
    115i8, 115i8, 46i8, 41i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_id")]
extern "C-unwind" fn builtin_id(self_: *mut _object, v: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = builtin_id_impl(self_ as *mut PyModuleDef, v);
    return_value
}

static builtin_setattr__doc__: __SlateAlign16<[i8; 160]> = __SlateAlign16([
    115i8, 101i8, 116i8, 97i8, 116i8, 116i8, 114i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8,
    101i8, 116i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8,
    97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 32i8, 111i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8,
    102i8, 105i8, 101i8, 100i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 10i8, 10i8, 115i8,
    101i8, 116i8, 97i8, 116i8, 116i8, 114i8, 40i8, 120i8, 44i8, 32i8, 39i8, 121i8, 39i8, 44i8,
    32i8, 118i8, 41i8, 32i8, 105i8, 115i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8,
    101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 96i8, 96i8, 120i8, 46i8, 121i8, 32i8, 61i8,
    32i8, 118i8, 96i8, 96i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_setattr")]
extern "C-unwind" fn builtin_setattr(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"setattr".as_ptr() as *mut i8, nargs, 3i32, 3i32) {
            let obj: *mut _object = *args.wrapping_offset(0 as isize);
            let name: *mut _object = *args.wrapping_offset(1 as isize);
            let value: *mut _object = *args.wrapping_offset(2 as isize);
            return_value = builtin_setattr_impl(module, obj, name, value);
        }
        return return_value;
    }
}

use slate_support::isatty;
use slate_support::memcpy;
use slate_support::strcmp;
use slate_support::strlen;

static builtin_delattr__doc__: __SlateAlign16<[i8; 132]> = __SlateAlign16([
    100i8, 101i8, 108i8, 97i8, 116i8, 116i8, 114i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 68i8, 101i8, 108i8, 101i8, 116i8, 101i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 97i8, 116i8, 116i8, 114i8,
    105i8, 98i8, 117i8, 116i8, 101i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8,
    10i8, 10i8, 100i8, 101i8, 108i8, 97i8, 116i8, 116i8, 114i8, 40i8, 120i8, 44i8, 32i8, 39i8,
    121i8, 39i8, 41i8, 32i8, 105i8, 115i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8,
    101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 96i8, 96i8, 100i8, 101i8, 108i8, 32i8, 120i8,
    46i8, 121i8, 96i8, 96i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_delattr")]
extern "C-unwind" fn builtin_delattr(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"delattr".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            let obj: *mut _object = *args.wrapping_offset(0 as isize);
            let name: *mut _object = *args.wrapping_offset(1 as isize);
            return_value = builtin_delattr_impl(module, obj, name);
        }
        return return_value;
    }
}

static builtin_hash__doc__: __SlateAlign16<[i8; 325]> = __SlateAlign16([
    104i8, 97i8, 115i8, 104i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    111i8, 98i8, 106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8,
    114i8, 32i8, 104i8, 97i8, 115i8, 104i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 84i8, 119i8, 111i8, 32i8, 111i8, 98i8,
    106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 111i8, 109i8,
    112i8, 97i8, 114i8, 101i8, 32i8, 101i8, 113i8, 117i8, 97i8, 108i8, 32i8, 109i8, 117i8, 115i8,
    116i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 104i8, 97i8, 118i8, 101i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 104i8, 97i8, 115i8, 104i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 44i8, 32i8, 98i8, 117i8, 116i8, 10i8, 116i8, 104i8, 101i8, 32i8, 114i8,
    101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8,
    110i8, 101i8, 99i8, 101i8, 115i8, 115i8, 97i8, 114i8, 105i8, 108i8, 121i8, 32i8, 116i8, 114i8,
    117i8, 101i8, 46i8, 32i8, 32i8, 72i8, 97i8, 115i8, 104i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 115i8, 32i8, 109i8, 97i8, 121i8, 32i8, 100i8, 105i8, 102i8, 102i8, 101i8, 114i8, 32i8,
    98i8, 101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 10i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8,
    32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 101i8, 115i8, 46i8, 32i8, 32i8, 78i8,
    111i8, 116i8, 32i8, 97i8, 108i8, 108i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8,
    32i8, 97i8, 114i8, 101i8, 32i8, 104i8, 97i8, 115i8, 104i8, 97i8, 98i8, 108i8, 101i8, 59i8,
    32i8, 99i8, 97i8, 108i8, 108i8, 105i8, 110i8, 103i8, 32i8, 104i8, 97i8, 115i8, 104i8, 40i8,
    41i8, 32i8, 111i8, 110i8, 32i8, 97i8, 110i8, 10i8, 117i8, 110i8, 104i8, 97i8, 115i8, 104i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 114i8, 97i8,
    105i8, 115i8, 101i8, 115i8, 32i8, 84i8, 121i8, 112i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8,
    46i8, 0i8,
]);

static builtin_hex__doc__: __SlateAlign16<[i8; 117]> = __SlateAlign16([
    104i8, 101i8, 120i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 110i8,
    117i8, 109i8, 98i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 104i8, 101i8, 120i8, 97i8,
    100i8, 101i8, 99i8, 105i8, 109i8, 97i8, 108i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8,
    101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8,
    32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 46i8, 10i8, 10i8, 32i8, 32i8, 32i8,
    62i8, 62i8, 62i8, 32i8, 104i8, 101i8, 120i8, 40i8, 49i8, 50i8, 54i8, 52i8, 56i8, 52i8, 51i8,
    48i8, 41i8, 10i8, 32i8, 32i8, 32i8, 39i8, 48i8, 120i8, 99i8, 48i8, 102i8, 102i8, 101i8, 101i8,
    39i8, 0i8,
]);

static builtin_aiter__doc__: __SlateAlign16<[i8; 91]> = __SlateAlign16([
    97i8, 105i8, 116i8, 101i8, 114i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 97i8, 115i8, 121i8, 110i8, 99i8, 95i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 97i8, 110i8, 32i8, 65i8, 115i8, 121i8, 110i8, 99i8, 73i8, 116i8, 101i8, 114i8,
    97i8, 116i8, 111i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 110i8, 32i8, 65i8, 115i8,
    121i8, 110i8, 99i8, 73i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 98i8,
    106i8, 101i8, 99i8, 116i8, 46i8, 0i8,
]);

static builtin_anext__doc__: __SlateAlign16<[i8; 218]> = __SlateAlign16([
    97i8, 110i8, 101i8, 120i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 97i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 44i8, 32i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 61i8, 60i8, 117i8, 110i8, 114i8, 101i8, 112i8, 114i8, 101i8,
    115i8, 101i8, 110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 62i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    110i8, 101i8, 120i8, 116i8, 32i8, 105i8, 116i8, 101i8, 109i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 115i8, 121i8, 110i8, 99i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 116i8, 111i8, 114i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 100i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 115i8, 121i8, 110i8, 99i8, 32i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8,
    104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 44i8, 10i8, 105i8, 116i8, 32i8, 105i8, 115i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 115i8, 116i8,
    101i8, 97i8, 100i8, 32i8, 111i8, 102i8, 32i8, 114i8, 97i8, 105i8, 115i8, 105i8, 110i8, 103i8,
    32i8, 83i8, 116i8, 111i8, 112i8, 65i8, 115i8, 121i8, 110i8, 99i8, 73i8, 116i8, 101i8, 114i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_anext")]
extern "C-unwind" fn builtin_anext(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut default_value: *mut _object = unsafe { core::mem::zeroed() };
    let mut aiterator: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        default_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"anext".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
            aiterator = *args.wrapping_offset(0 as isize);
            if nargs >= 2 {
                default_value = *args.wrapping_offset(1 as isize);
            }
            return_value = builtin_anext_impl(module, aiterator, default_value);
        }
        return return_value;
    }
}

static builtin_len__doc__: __SlateAlign16<[i8; 68]> = __SlateAlign16([
    108i8, 101i8, 110i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 111i8,
    98i8, 106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8,
    111i8, 102i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 105i8, 110i8, 32i8, 97i8, 32i8,
    99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 101i8, 114i8, 46i8, 0i8,
]);

static builtin_locals__doc__: __SlateAlign16<[i8; 288]> = __SlateAlign16([
    108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 97i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8,
    99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 99i8, 111i8, 112i8, 101i8,
    39i8, 115i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8,
    98i8, 108i8, 101i8, 115i8, 46i8, 10i8, 10i8, 78i8, 79i8, 84i8, 69i8, 58i8, 32i8, 87i8, 104i8,
    101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 111i8, 114i8, 32i8, 110i8, 111i8, 116i8, 32i8, 117i8,
    112i8, 100i8, 97i8, 116i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 105i8, 115i8,
    32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8, 119i8, 105i8,
    108i8, 108i8, 32i8, 97i8, 102i8, 102i8, 101i8, 99i8, 116i8, 32i8, 110i8, 97i8, 109i8, 101i8,
    10i8, 108i8, 111i8, 111i8, 107i8, 117i8, 112i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 32i8, 115i8, 99i8, 111i8, 112i8, 101i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 118i8, 105i8, 99i8, 101i8, 45i8, 118i8, 101i8, 114i8, 115i8, 97i8,
    32i8, 105i8, 115i8, 32i8, 42i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 10i8, 100i8, 101i8, 112i8, 101i8, 110i8, 100i8, 101i8, 110i8,
    116i8, 42i8, 32i8, 97i8, 110i8, 100i8, 32i8, 110i8, 111i8, 116i8, 32i8, 99i8, 111i8, 118i8,
    101i8, 114i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 97i8, 110i8, 121i8, 32i8, 98i8, 97i8,
    99i8, 107i8, 119i8, 97i8, 114i8, 100i8, 115i8, 32i8, 99i8, 111i8, 109i8, 112i8, 97i8, 116i8,
    105i8, 98i8, 105i8, 108i8, 105i8, 116i8, 121i8, 10i8, 103i8, 117i8, 97i8, 114i8, 97i8, 110i8,
    116i8, 101i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_locals")]
extern "C-unwind" fn builtin_locals(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    builtin_locals_impl(module)
}

static builtin_oct__doc__: __SlateAlign16<[i8; 110]> = __SlateAlign16([
    111i8, 99i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 110i8,
    117i8, 109i8, 98i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 99i8, 116i8, 97i8,
    108i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8,
    101i8, 114i8, 46i8, 10i8, 10i8, 32i8, 32i8, 32i8, 62i8, 62i8, 62i8, 32i8, 111i8, 99i8, 116i8,
    40i8, 51i8, 52i8, 50i8, 51i8, 57i8, 49i8, 41i8, 10i8, 32i8, 32i8, 32i8, 39i8, 48i8, 111i8,
    49i8, 50i8, 51i8, 52i8, 53i8, 54i8, 55i8, 39i8, 0i8,
]);

static builtin_ord__doc__: __SlateAlign16<[i8; 256]> = __SlateAlign16([
    111i8, 114i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 99i8,
    104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8,
    114i8, 100i8, 105i8, 110i8, 97i8, 108i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8,
    102i8, 32i8, 97i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 46i8, 10i8,
    10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 97i8, 32i8, 111i8, 110i8, 101i8, 45i8, 99i8, 104i8,
    97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8,
    44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 85i8,
    110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 99i8, 111i8, 100i8, 101i8, 10i8, 112i8, 111i8,
    105i8, 110i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 104i8,
    97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8,
    32i8, 97i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 97i8, 114i8, 114i8, 97i8, 121i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    32i8, 111i8, 102i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 49i8, 44i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 116i8, 115i8, 10i8, 115i8, 105i8, 110i8,
    103i8, 108i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8,
    46i8, 0i8,
]);

static builtin_pow__doc__: __SlateAlign16<[i8; 232]> = __SlateAlign16([
    112i8, 111i8, 119i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    44i8, 32i8, 98i8, 97i8, 115i8, 101i8, 44i8, 32i8, 101i8, 120i8, 112i8, 44i8, 32i8, 109i8,
    111i8, 100i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 69i8, 113i8,
    117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 98i8, 97i8,
    115i8, 101i8, 42i8, 42i8, 101i8, 120i8, 112i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 50i8,
    32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 111i8, 114i8, 32i8,
    98i8, 97i8, 115i8, 101i8, 42i8, 42i8, 101i8, 120i8, 112i8, 32i8, 37i8, 32i8, 109i8, 111i8,
    100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 51i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 10i8, 10i8, 83i8, 111i8, 109i8, 101i8, 32i8, 116i8, 121i8, 112i8,
    101i8, 115i8, 44i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 97i8, 115i8, 32i8, 105i8, 110i8,
    116i8, 115i8, 44i8, 32i8, 97i8, 114i8, 101i8, 32i8, 97i8, 98i8, 108i8, 101i8, 32i8, 116i8,
    111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 97i8, 32i8, 109i8, 111i8, 114i8, 101i8, 32i8, 101i8,
    102i8, 102i8, 105i8, 99i8, 105i8, 101i8, 110i8, 116i8, 32i8, 97i8, 108i8, 103i8, 111i8, 114i8,
    105i8, 116i8, 104i8, 109i8, 10i8, 119i8, 104i8, 101i8, 110i8, 32i8, 105i8, 110i8, 118i8, 111i8,
    107i8, 101i8, 100i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    116i8, 104i8, 114i8, 101i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8,
    32i8, 102i8, 111i8, 114i8, 109i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_pow")]
extern "C-unwind" fn builtin_pow(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    let mut exp: *mut _object = unsafe { core::mem::zeroed() };
    let mut base: *mut _object = unsafe { core::mem::zeroed() };
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
        r#mod = core::ptr::addr_of_mut!(_Py_NoneStruct);
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__897),
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
            base = *args.wrapping_offset(0 as isize);
            exp = *args.wrapping_offset(1 as isize);
            if noptargs != 0 {
                r#mod = *args.wrapping_offset(2 as isize);
            }
            return_value = builtin_pow_impl(module, base, exp, r#mod);
        }
        return return_value;
    }
}

static mut _kwtuple__884: __SlateRecord_bltinmodule_c_h_884 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_884 = unsafe { core::mem::zeroed() };
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
                    ._py_base
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
                    ._py_exp
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
                    ._py_mod
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__896: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"base\0".as_ptr() as *mut i8) as *const i8,
    (b"exp\0".as_ptr() as *mut i8) as *const i8,
    (b"mod\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__897: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__896.0) as *const *const i8 };
    __t0.fname = (b"pow\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__884.ob_base.ob_base) };
    __t0
};

static builtin_print__doc__: __SlateAlign16<[i8; 385]> = __SlateAlign16([
    112i8, 114i8, 105i8, 110i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 42i8, 97i8, 114i8, 103i8, 115i8, 44i8, 32i8, 115i8, 101i8, 112i8, 61i8,
    39i8, 32i8, 39i8, 44i8, 32i8, 101i8, 110i8, 100i8, 61i8, 39i8, 92i8, 110i8, 39i8, 44i8, 32i8,
    102i8, 105i8, 108i8, 101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 102i8, 108i8, 117i8,
    115i8, 104i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8,
    114i8, 105i8, 110i8, 116i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8,
    44i8, 32i8, 111i8, 114i8, 32i8, 116i8, 111i8, 32i8, 115i8, 121i8, 115i8, 46i8, 115i8, 116i8,
    100i8, 111i8, 117i8, 116i8, 32i8, 98i8, 121i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 101i8, 112i8, 10i8, 32i8, 32i8, 32i8, 32i8, 115i8,
    116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 115i8, 101i8, 114i8, 116i8, 101i8,
    100i8, 32i8, 98i8, 101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 115i8, 44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 97i8, 32i8,
    115i8, 112i8, 97i8, 99i8, 101i8, 46i8, 10i8, 32i8, 32i8, 101i8, 110i8, 100i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 97i8, 112i8, 112i8, 101i8, 110i8,
    100i8, 101i8, 100i8, 32i8, 97i8, 102i8, 116i8, 101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    108i8, 97i8, 115i8, 116i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8,
    101i8, 46i8, 10i8, 32i8, 32i8, 102i8, 105i8, 108i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 97i8,
    32i8, 102i8, 105i8, 108i8, 101i8, 45i8, 108i8, 105i8, 107i8, 101i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 32i8, 40i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 41i8, 59i8, 32i8,
    100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 121i8, 115i8, 46i8,
    115i8, 116i8, 100i8, 111i8, 117i8, 116i8, 46i8, 10i8, 32i8, 32i8, 102i8, 108i8, 117i8, 115i8,
    104i8, 10i8, 32i8, 32i8, 32i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8,
    116i8, 111i8, 32i8, 102i8, 111i8, 114i8, 99i8, 105i8, 98i8, 108i8, 121i8, 32i8, 102i8, 108i8,
    117i8, 115i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_print")]
extern "C-unwind" fn builtin_print(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut flush: i32 = unsafe { core::mem::zeroed() };
    let mut file: *mut _object = unsafe { core::mem::zeroed() };
    let mut end: *mut _object = unsafe { core::mem::zeroed() };
    let mut sep: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v12: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 4]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_15: *mut [*mut _object; 4] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 4];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v12 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v12 = 0;
        }
        '__join_0: {
            noptargs = 0i64.wrapping_add(__v12).wrapping_sub(0);
            sep = core::ptr::addr_of_mut!(_Py_NoneStruct);
            end = core::ptr::addr_of_mut!(_Py_NoneStruct);
            file = core::ptr::addr_of_mut!(_Py_NoneStruct);
            flush = 0;
            fastargs = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__975),
                0i32,
                0i32,
                0i32,
                1i32,
                (*__slate_slot_15).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !fastargs.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *fastargs.wrapping_offset(0 as isize) }.is_null() {
                            sep = *fastargs.wrapping_offset(0 as isize);
                            let __v4: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v4;
                            if __v4 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *fastargs.wrapping_offset(1 as isize) }.is_null() {
                            end = *fastargs.wrapping_offset(1 as isize);
                            let __v2: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v2;
                            if __v2 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *fastargs.wrapping_offset(2 as isize) }.is_null() {
                            file = *fastargs.wrapping_offset(2 as isize);
                            let __v0: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v0;
                            if __v0 == 0 {
                                break '__join_1;
                            }
                        }
                        flush = PyObject_IsTrue(*fastargs.wrapping_offset(3 as isize));
                        if flush < 0 {
                            break '__join_0;
                        }
                    }
                }
                let __clinic_args: *const *mut _object = args;
                let args_length: i64 = nargs;
                return_value =
                    builtin_print_impl(module, __clinic_args, args_length, sep, end, file, flush);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__962: __SlateRecord_bltinmodule_c_h_962 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_962 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 4i64,
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
                    ._py_end
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
                    ._py_file
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
                    ._py_flush
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__974: __SlateAlign16<[*const i8; 5]> = __SlateAlign16([
    (b"sep\0".as_ptr() as *mut i8) as *const i8,
    (b"end\0".as_ptr() as *mut i8) as *const i8,
    (b"file\0".as_ptr() as *mut i8) as *const i8,
    (b"flush\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__975: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__974.0) as *const *const i8 };
    __t0.fname = (b"print\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__962.ob_base.ob_base) };
    __t0
};

static builtin_input__doc__: __SlateAlign16<[i8; 338]> = __SlateAlign16([
    105i8, 110i8, 112i8, 117i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 112i8, 114i8, 111i8, 109i8, 112i8, 116i8, 61i8, 39i8, 39i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 97i8, 100i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    110i8, 103i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 115i8, 116i8, 97i8, 110i8, 100i8, 97i8,
    114i8, 100i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8,
    32i8, 116i8, 114i8, 97i8, 105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 110i8, 101i8, 119i8, 108i8,
    105i8, 110i8, 101i8, 32i8, 105i8, 115i8, 32i8, 115i8, 116i8, 114i8, 105i8, 112i8, 112i8, 101i8,
    100i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 112i8, 114i8, 111i8, 109i8, 112i8, 116i8,
    32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 105i8, 102i8, 32i8, 103i8, 105i8,
    118i8, 101i8, 110i8, 44i8, 32i8, 105i8, 115i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 101i8,
    100i8, 32i8, 116i8, 111i8, 32i8, 115i8, 116i8, 97i8, 110i8, 100i8, 97i8, 114i8, 100i8, 32i8,
    111i8, 117i8, 116i8, 112i8, 117i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 111i8, 117i8,
    116i8, 32i8, 97i8, 10i8, 116i8, 114i8, 97i8, 105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 110i8,
    101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8, 32i8,
    114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 46i8,
    10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 115i8, 101i8, 114i8, 32i8,
    104i8, 105i8, 116i8, 115i8, 32i8, 69i8, 79i8, 70i8, 32i8, 40i8, 42i8, 110i8, 105i8, 120i8,
    58i8, 32i8, 67i8, 116i8, 114i8, 108i8, 45i8, 68i8, 44i8, 32i8, 87i8, 105i8, 110i8, 100i8,
    111i8, 119i8, 115i8, 58i8, 32i8, 67i8, 116i8, 114i8, 108i8, 45i8, 90i8, 43i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 41i8, 44i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 10i8, 69i8,
    79i8, 70i8, 69i8, 114i8, 114i8, 111i8, 114i8, 46i8, 10i8, 79i8, 110i8, 32i8, 42i8, 110i8,
    105i8, 120i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8, 115i8, 44i8, 32i8, 114i8, 101i8,
    97i8, 100i8, 108i8, 105i8, 110i8, 101i8, 32i8, 105i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8,
    32i8, 105i8, 102i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_input")]
extern "C-unwind" fn builtin_input(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut prompt: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        prompt = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"input".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                prompt = *args.wrapping_offset(0 as isize);
            }
            return_value = builtin_input_impl(module, prompt);
        }
        return return_value;
    }
}

static builtin_repr__doc__: __SlateAlign16<[i8; 157]> = __SlateAlign16([
    114i8, 101i8, 112i8, 114i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    111i8, 98i8, 106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 110i8, 111i8, 110i8, 105i8,
    99i8, 97i8, 108i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 114i8, 101i8, 112i8,
    114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 10i8, 10i8,
    70i8, 111i8, 114i8, 32i8, 109i8, 97i8, 110i8, 121i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 44i8, 32i8, 105i8, 110i8, 99i8, 108i8, 117i8,
    100i8, 105i8, 110i8, 103i8, 32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 98i8, 117i8, 105i8, 108i8,
    116i8, 105i8, 110i8, 115i8, 44i8, 32i8, 101i8, 118i8, 97i8, 108i8, 40i8, 114i8, 101i8, 112i8,
    114i8, 40i8, 111i8, 98i8, 106i8, 41i8, 41i8, 32i8, 61i8, 61i8, 32i8, 111i8, 98i8, 106i8, 46i8,
    0i8,
]);

static builtin_round__doc__: __SlateAlign16<[i8; 248]> = __SlateAlign16([
    114i8, 111i8, 117i8, 110i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 44i8, 32i8, 110i8, 100i8,
    105i8, 103i8, 105i8, 116i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 111i8, 117i8, 110i8, 100i8, 32i8, 97i8, 32i8, 110i8, 117i8, 109i8, 98i8,
    101i8, 114i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8,
    112i8, 114i8, 101i8, 99i8, 105i8, 115i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 32i8, 100i8,
    101i8, 99i8, 105i8, 109i8, 97i8, 108i8, 32i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 46i8,
    10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 103i8, 101i8, 114i8, 32i8, 105i8, 102i8, 32i8, 110i8, 100i8, 105i8, 103i8, 105i8, 116i8,
    115i8, 32i8, 105i8, 115i8, 32i8, 111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 111i8,
    114i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 10i8, 79i8, 116i8, 104i8, 101i8, 114i8, 119i8,
    105i8, 115i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 104i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 116i8, 121i8, 112i8, 101i8, 32i8, 97i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 46i8, 32i8, 32i8, 110i8,
    100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 10i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8,
    110i8, 101i8, 103i8, 97i8, 116i8, 105i8, 118i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_round")]
extern "C-unwind" fn builtin_round(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut ndigits: *mut _object = unsafe { core::mem::zeroed() };
    let mut number: *mut _object = unsafe { core::mem::zeroed() };
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
        ndigits = core::ptr::addr_of_mut!(_Py_NoneStruct);
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__1121),
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
            number = *args.wrapping_offset(0 as isize);
            if noptargs != 0 {
                ndigits = *args.wrapping_offset(1 as isize);
            }
            return_value = builtin_round_impl(module, number, ndigits);
        }
        return return_value;
    }
}

static mut _kwtuple__1108: __SlateRecord_bltinmodule_c_h_1108 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_1108 = unsafe { core::mem::zeroed() };
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
                    ._py_number
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
                    ._py_ndigits
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1120: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"number\0".as_ptr() as *mut i8) as *const i8,
    (b"ndigits\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1121: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1120.0) as *const *const i8 };
    __t0.fname = (b"round\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1108.ob_base.ob_base) };
    __t0
};

static builtin_sum__doc__: __SlateAlign16<[i8; 268]> = __SlateAlign16([
    115i8, 117i8, 109i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 115i8, 116i8,
    97i8, 114i8, 116i8, 61i8, 48i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 117i8, 109i8, 32i8, 111i8, 102i8, 32i8,
    97i8, 32i8, 39i8, 115i8, 116i8, 97i8, 114i8, 116i8, 39i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 32i8, 40i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 48i8, 41i8,
    32i8, 112i8, 108i8, 117i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    98i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 115i8,
    10i8, 10i8, 87i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8,
    44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8,
    116i8, 97i8, 114i8, 116i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 10i8, 84i8, 104i8,
    105i8, 115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8,
    32i8, 105i8, 110i8, 116i8, 101i8, 110i8, 100i8, 101i8, 100i8, 32i8, 115i8, 112i8, 101i8, 99i8,
    105i8, 102i8, 105i8, 99i8, 97i8, 108i8, 108i8, 121i8, 32i8, 102i8, 111i8, 114i8, 32i8, 117i8,
    115i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 110i8, 117i8, 109i8, 101i8, 114i8, 105i8,
    99i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 97i8, 110i8, 100i8, 10i8, 109i8,
    97i8, 121i8, 32i8, 114i8, 101i8, 106i8, 101i8, 99i8, 116i8, 32i8, 110i8, 111i8, 110i8, 45i8,
    110i8, 117i8, 109i8, 101i8, 114i8, 105i8, 99i8, 32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_sum")]
extern "C-unwind" fn builtin_sum(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut start: *mut _object = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
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
        start = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__1190),
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
            iterable = *args.wrapping_offset(0 as isize);
            if noptargs != 0 {
                start = *args.wrapping_offset(1 as isize);
            }
            return_value = builtin_sum_impl(module, iterable, start);
        }
        return return_value;
    }
}

static mut _kwtuple__1177: __SlateRecord_bltinmodule_c_h_1177 = {
    let mut __t0: __SlateRecord_bltinmodule_c_h_1177 = unsafe { core::mem::zeroed() };
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
                ._py_start
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__1189: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"start\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1190: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1189.0) as *const *const i8 };
    __t0.fname = (b"sum\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1177.ob_base.ob_base) };
    __t0
};

static builtin_isinstance__doc__: __SlateAlign16<[i8; 294]> = __SlateAlign16([
    105i8, 115i8, 105i8, 110i8, 115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 99i8, 108i8, 97i8,
    115i8, 115i8, 95i8, 111i8, 114i8, 95i8, 116i8, 117i8, 112i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 119i8,
    104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 32i8, 105i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 115i8, 116i8, 97i8,
    110i8, 99i8, 101i8, 32i8, 111i8, 102i8, 32i8, 97i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8,
    32i8, 111i8, 114i8, 32i8, 111i8, 102i8, 32i8, 97i8, 32i8, 115i8, 117i8, 98i8, 99i8, 108i8,
    97i8, 115i8, 115i8, 32i8, 116i8, 104i8, 101i8, 114i8, 101i8, 111i8, 102i8, 46i8, 10i8, 10i8,
    65i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 44i8, 32i8, 97i8, 115i8, 32i8, 105i8, 110i8,
    32i8, 96i8, 96i8, 105i8, 115i8, 105i8, 110i8, 115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 40i8,
    120i8, 44i8, 32i8, 40i8, 65i8, 44i8, 32i8, 66i8, 44i8, 32i8, 46i8, 46i8, 46i8, 41i8, 41i8,
    96i8, 96i8, 44i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 103i8, 105i8, 118i8,
    101i8, 110i8, 32i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 10i8, 116i8, 97i8, 114i8, 103i8,
    101i8, 116i8, 32i8, 116i8, 111i8, 32i8, 99i8, 104i8, 101i8, 99i8, 107i8, 32i8, 97i8, 103i8,
    97i8, 105i8, 110i8, 115i8, 116i8, 46i8, 32i8, 32i8, 84i8, 104i8, 105i8, 115i8, 32i8, 105i8,
    115i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8, 32i8, 116i8,
    111i8, 32i8, 96i8, 96i8, 105i8, 115i8, 105i8, 110i8, 115i8, 116i8, 97i8, 110i8, 99i8, 101i8,
    40i8, 120i8, 44i8, 32i8, 65i8, 41i8, 32i8, 111i8, 114i8, 10i8, 105i8, 115i8, 105i8, 110i8,
    115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 40i8, 120i8, 44i8, 32i8, 66i8, 41i8, 32i8, 111i8,
    114i8, 32i8, 46i8, 46i8, 46i8, 96i8, 96i8, 32i8, 101i8, 116i8, 99i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_isinstance")]
extern "C-unwind" fn builtin_isinstance(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"isinstance".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            let obj: *mut _object = *args.wrapping_offset(0 as isize);
            let class_or_tuple: *mut _object = *args.wrapping_offset(1 as isize);
            return_value = builtin_isinstance_impl(module, obj, class_or_tuple);
        }
        return return_value;
    }
}

static builtin_issubclass__doc__: __SlateAlign16<[i8; 286]> = __SlateAlign16([
    105i8, 115i8, 115i8, 117i8, 98i8, 99i8, 108i8, 97i8, 115i8, 115i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 99i8, 108i8, 115i8, 44i8, 32i8, 99i8, 108i8, 97i8,
    115i8, 115i8, 95i8, 111i8, 114i8, 95i8, 116i8, 117i8, 112i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 119i8,
    104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 39i8, 99i8, 108i8, 115i8, 39i8, 32i8, 105i8,
    115i8, 32i8, 100i8, 101i8, 114i8, 105i8, 118i8, 101i8, 100i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 97i8, 110i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8,
    32i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 97i8, 109i8,
    101i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 46i8, 10i8, 10i8, 65i8, 32i8, 116i8, 117i8,
    112i8, 108i8, 101i8, 44i8, 32i8, 97i8, 115i8, 32i8, 105i8, 110i8, 32i8, 96i8, 96i8, 105i8,
    115i8, 115i8, 117i8, 98i8, 99i8, 108i8, 97i8, 115i8, 115i8, 40i8, 120i8, 44i8, 32i8, 40i8,
    65i8, 44i8, 32i8, 66i8, 44i8, 32i8, 46i8, 46i8, 46i8, 41i8, 41i8, 96i8, 96i8, 44i8, 32i8,
    109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 97i8,
    115i8, 32i8, 116i8, 104i8, 101i8, 10i8, 116i8, 97i8, 114i8, 103i8, 101i8, 116i8, 32i8, 116i8,
    111i8, 32i8, 99i8, 104i8, 101i8, 99i8, 107i8, 32i8, 97i8, 103i8, 97i8, 105i8, 110i8, 115i8,
    116i8, 46i8, 32i8, 32i8, 84i8, 104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8, 101i8, 113i8,
    117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 96i8, 96i8,
    105i8, 115i8, 115i8, 117i8, 98i8, 99i8, 108i8, 97i8, 115i8, 115i8, 40i8, 120i8, 44i8, 32i8,
    65i8, 41i8, 32i8, 111i8, 114i8, 10i8, 105i8, 115i8, 115i8, 117i8, 98i8, 99i8, 108i8, 97i8,
    115i8, 115i8, 40i8, 120i8, 44i8, 32i8, 66i8, 41i8, 32i8, 111i8, 114i8, 32i8, 46i8, 46i8, 46i8,
    96i8, 96i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_issubclass")]
extern "C-unwind" fn builtin_issubclass(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"issubclass".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            let cls: *mut _object = *args.wrapping_offset(0 as isize);
            let class_or_tuple: *mut _object = *args.wrapping_offset(1 as isize);
            return_value = builtin_issubclass_impl(module, cls, class_or_tuple);
        }
        return return_value;
    }
}

// _PyAST_Validate()
// _PyObject_CallNoArgs()
// _PyEval_Vector()
// _PyAST_Compile()
// _PyFile_Flush
// _PyFloat_ExactDealloc()
// _PyInterpreterState_GetConfig()
// _PyLong_CompactValue
// _PyArg_NoKwnames()
// _Py_AddToAllObjects()
// _PyErr_NoMemory()
// _PyThreadState_GET()
// _Py_SourceAsString()
// _PySys_GetRequiredAttr()
// _PyTuple_FromArray()
// PyCell_GetRef()
// isatty()
fn update_bases(bases: *mut _object, args: *const *mut _object, nargs: i64) -> *mut _object {
    let mut new_bases: *mut _object = unsafe { core::mem::zeroed() };
    let mut new_base: *mut _object = unsafe { core::mem::zeroed() };
    let mut meth: *mut _object = unsafe { core::mem::zeroed() };
    let mut base: *mut _object = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        new_bases = core::ptr::null_mut::<_object>();
        i = 0;
        '__join_0: {
            '__join_15: {
                '__join_12: {
                    '__join_7: {
                        while i < nargs {
                            base = *args.wrapping_offset(i as isize);
                            if PyType_Check!(base) != 0 {
                                if !new_bases.is_null() {
                                    // If we already have made a replacement, then we append every normal base,
                                    // otherwise just skip it.
                                    if PyList_Append(new_bases, base) < 0 {
                                        break '__join_0;
                                    }
                                }
                            } else {
                                if PyObject_GetOptionalAttr(
                                    base,
                                    core::ptr::addr_of_mut!(
                                        _PyRuntime
                                            .static_objects
                                            .singletons
                                            .strings
                                            .identifiers
                                            ._py___mro_entries__
                                            ._ascii
                                            .ob_base
                                    ),
                                    core::ptr::addr_of_mut!(meth),
                                ) < 0
                                {
                                    break '__join_0;
                                } else {
                                    if meth.is_null() {
                                        if !new_bases.is_null() {
                                            if PyList_Append(new_bases, base) < 0 {
                                                break '__join_0;
                                            }
                                        }
                                    } else {
                                        new_base = PyObject_CallOneArg(meth, bases);
                                        Py_DECREF!(meth);
                                        if new_base.is_null() {
                                            break '__join_0;
                                        } else {
                                            if PyTuple_Check!(new_base) == 0 {
                                                break '__join_15;
                                            } else {
                                                if new_bases.is_null() {
                                                    // If this is a first successful replacement, create new_bases list and
                                                    // copy previously encountered bases.
                                                    let __v4: *mut _object = PyList_New(i);
                                                    new_bases = __v4;
                                                    if __v4.is_null() {
                                                        break '__join_12;
                                                    } else {
                                                        j = 0;
                                                        while j < i {
                                                            base =
                                                                *args.wrapping_offset(j as isize);
                                                            PyList_SET_ITEM!(
                                                                new_bases,
                                                                j,
                                                                Py_NewRef!(base)
                                                            );
                                                            j = j.wrapping_add(1);
                                                        }
                                                    }
                                                }
                                                j = PyList_GET_SIZE!(new_bases);
                                                if PyList_SetSlice(new_bases, j, j, new_base) < 0 {
                                                    break '__join_7;
                                                } else {
                                                    Py_DECREF!(new_base);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        if new_bases.is_null() {
                            return bases;
                        } else {
                            let result: *mut _object = PyList_AsTuple(new_bases);
                            Py_DECREF!(new_bases);
                            return result;
                        }
                    }
                    Py_DECREF!(new_base);
                    break '__join_0;
                }
                Py_DECREF!(new_base);
                break '__join_0;
            }
            PyErr_SetString(
                PyExc_TypeError,
                c"__mro_entries__ must return a tuple".as_ptr(),
            );
            Py_DECREF!(new_base);
        }
        Py_XDECREF!(new_bases);
        return core::ptr::null_mut::<_object>();
    }
}

/// AC: cannot convert yet, waiting for *args support
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin___build_class__")]
extern "C-unwind" fn builtin___build_class__(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut cell_cls: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut margs: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(margs) as *mut [*mut _object; 3];
    let mut pargs: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(pargs) as *mut [*mut _object; 2];
    // initialize to prevent gcc warning
    let mut isclass: i32 = unsafe { core::mem::zeroed() };
    let mut bases: *mut _object = unsafe { core::mem::zeroed() };
    let mut mkw: *mut _object = unsafe { core::mem::zeroed() };
    let mut orig_bases: *mut _object = unsafe { core::mem::zeroed() };
    let mut meta: *mut _object = unsafe { core::mem::zeroed() };
    let mut ns: *mut _object = unsafe { core::mem::zeroed() };
    let mut cell: *mut _object = unsafe { core::mem::zeroed() };
    let mut cls: *mut _object = unsafe { core::mem::zeroed() };
    let mut prep: *mut _object = unsafe { core::mem::zeroed() };
    let mut winner: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    let mut func: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        cls = core::ptr::null_mut::<_object>();
        cell = core::ptr::null_mut::<_object>();
        ns = core::ptr::null_mut::<_object>();
        meta = core::ptr::null_mut::<_object>();
        orig_bases = core::ptr::null_mut::<_object>();
        mkw = core::ptr::null_mut::<_object>();
        bases = core::ptr::null_mut::<_object>();
        isclass = 0;
        if nargs < 2 {
            PyErr_SetString(
                PyExc_TypeError,
                c"__build_class__: not enough arguments".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            func = *args.wrapping_offset(0 as isize); // Better be callable
            if PyFunction_Check!(func) == 0 {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"__build_class__: func must be a function".as_ptr(),
                );
                return core::ptr::null_mut::<_object>();
            } else {
                name = *args.wrapping_offset(1 as isize);
                if PyUnicode_Check!(name) == 0 {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"__build_class__: name is not a string".as_ptr(),
                    );
                    return core::ptr::null_mut::<_object>();
                } else {
                    orig_bases =
                        _PyTuple_FromArray(args.wrapping_offset(2 as isize), nargs.wrapping_sub(2));
                    if orig_bases.is_null() {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        bases = update_bases(
                            orig_bases,
                            args.wrapping_offset(2 as isize),
                            nargs.wrapping_sub(2),
                        );
                        if bases.is_null() {
                            Py_DECREF!(orig_bases);
                            return core::ptr::null_mut::<_object>();
                        } else {
                            '__join_2: {
                                if kwnames.is_null() {
                                    meta = core::ptr::null_mut::<_object>();
                                    mkw = core::ptr::null_mut::<_object>();
                                } else {
                                    mkw = _PyStack_AsDict(
                                        args.wrapping_offset(nargs as isize),
                                        kwnames,
                                    );
                                    if mkw.is_null() {
                                        break '__join_2;
                                    } else {
                                        if PyDict_Pop(
                                            mkw,
                                            core::ptr::addr_of_mut!(
                                                _PyRuntime
                                                    .static_objects
                                                    .singletons
                                                    .strings
                                                    .identifiers
                                                    ._py_metaclass
                                                    ._ascii
                                                    .ob_base
                                            ),
                                            core::ptr::addr_of_mut!(meta),
                                        ) < 0
                                        {
                                            break '__join_2;
                                        } else {
                                            if !meta.is_null() {
                                                // metaclass is explicitly given, check if it's indeed a class
                                                isclass = PyType_Check!(meta);
                                            }
                                        }
                                    }
                                }
                                if meta.is_null() {
                                    // if there are no bases, use type:
                                    if PyTuple_GET_SIZE!(bases) == 0 {
                                        meta = core::ptr::addr_of_mut!(PyType_Type) as *mut _object;
                                    } else {
                                        let base0: *mut _object = PyTuple_GET_ITEM!(bases, 0i32);
                                        meta = Py_TYPE!(base0) as *mut _object;
                                    }
                                    // else get the type of the first base
                                    Py_INCREF!(meta);
                                    isclass = 1; // meta is really a class
                                }
                                if isclass != 0 {
                                    // meta is really a class, so check for a more derived
                                    // metaclass, or possible metaclass conflicts:
                                    winner =
                                        _PyType_CalculateMetaclass(meta as *mut _typeobject, bases)
                                            as *mut _object;
                                    if winner.is_null() {
                                        break '__join_2;
                                    } else {
                                        if winner != meta {
                                            Py_SETREF!(meta, Py_NewRef!(winner));
                                        }
                                    }
                                }
                                // else: meta is not a class, so we cannot do the metaclass
                                // calculation, so we will use the explicitly given object as it is
                                if PyObject_GetOptionalAttr(
                                    meta,
                                    core::ptr::addr_of_mut!(
                                        _PyRuntime
                                            .static_objects
                                            .singletons
                                            .strings
                                            .identifiers
                                            ._py___prepare__
                                            ._ascii
                                            .ob_base
                                    ),
                                    core::ptr::addr_of_mut!(prep),
                                ) < 0
                                {
                                    ns = core::ptr::null_mut::<_object>();
                                } else {
                                    if prep.is_null() {
                                        ns = PyDict_New();
                                    } else {
                                        core::ptr::write(__slate_slot_7, [name, bases]);
                                        ns = PyObject_VectorcallDict(
                                            prep,
                                            (*__slate_slot_7).as_mut_ptr() as *const *mut _object,
                                            2u64,
                                            mkw,
                                        );
                                        Py_DECREF!(prep);
                                    }
                                }
                                if !ns.is_null() {
                                    if PyMapping_Check(ns) == 0 {
                                        PyErr_Format(PyExc_TypeError, c"%.200s.__prepare__() must return a mapping, not %.200s".as_ptr(), if isclass != 0 { (*(meta as *mut _typeobject)).tp_name } else { c"<metaclass>".as_ptr() }, (*Py_TYPE!(ns)).tp_name);
                                    } else {
                                        let tstate: *mut _ts = _PyThreadState_GET();
                                        EVAL_CALL_STAT_INC!();
                                        cell = _PyEval_Vector(
                                            tstate,
                                            func as *mut __SlateRecord_funcobject_h_62,
                                            ns,
                                            core::ptr::null::<*mut _object>(),
                                            0u64,
                                            core::ptr::null_mut::<_object>(),
                                        );
                                        if !cell.is_null() {
                                            if bases != orig_bases {
                                                if PyMapping_SetItemString(
                                                    ns,
                                                    c"__orig_bases__".as_ptr(),
                                                    orig_bases,
                                                ) < 0
                                                {
                                                    break '__join_2;
                                                }
                                            }
                                            core::ptr::write(__slate_slot_5, [name, bases, ns]);
                                            cls = PyObject_VectorcallDict(
                                                meta,
                                                (*__slate_slot_5).as_mut_ptr()
                                                    as *const *mut _object,
                                                3u64,
                                                mkw,
                                            );
                                            if !cls.is_null() {
                                                __v4 = PyType_Check!(cls) != 0;
                                            } else {
                                                __v4 = false;
                                            }
                                            if __v4 {
                                                __v3 = PyCell_Check!(cell) != 0;
                                            } else {
                                                __v3 = false;
                                            }
                                            if __v3 {
                                                cell_cls = PyCell_GetRef(
                                                    cell as *mut __SlateRecord_cellobject_h_14,
                                                );
                                                if cell_cls != cls {
                                                    if cell_cls.is_null() {
                                                        let msg: *const i8 = c"__class__ not set defining %.200R as %.200R. Was __classcell__ propagated to type.__new__?".as_ptr();
                                                        PyErr_Format(
                                                            PyExc_RuntimeError,
                                                            msg,
                                                            name,
                                                            cls,
                                                        );
                                                    } else {
                                                        let msg: *const i8 = c"__class__ set to %.200R defining %.200R as %.200R".as_ptr();
                                                        PyErr_Format(
                                                            PyExc_TypeError,
                                                            msg,
                                                            cell_cls,
                                                            name,
                                                            cls,
                                                        );
                                                    }
                                                    Py_XDECREF!(cell_cls);
                                                    Py_SETREF__3!(cls);
                                                } else {
                                                    Py_DECREF!(cell_cls);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Py_XDECREF!(cell);
                            Py_XDECREF!(ns);
                            Py_XDECREF!(meta);
                            Py_XDECREF!(mkw);
                            if bases != orig_bases {
                                Py_DECREF!(orig_bases);
                            }
                            Py_DECREF!(bases);
                            return cls;
                        }
                    }
                }
            }
        }
    }
}

static build_class_doc: __SlateAlign16<[i8; 124]> = __SlateAlign16([
    95i8, 95i8, 98i8, 117i8, 105i8, 108i8, 100i8, 95i8, 99i8, 108i8, 97i8, 115i8, 115i8, 95i8,
    95i8, 40i8, 102i8, 117i8, 110i8, 99i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 47i8,
    44i8, 32i8, 42i8, 98i8, 97i8, 115i8, 101i8, 115i8, 44i8, 32i8, 91i8, 109i8, 101i8, 116i8, 97i8,
    99i8, 108i8, 97i8, 115i8, 115i8, 93i8, 44i8, 32i8, 42i8, 42i8, 107i8, 119i8, 100i8, 115i8,
    41i8, 32i8, 45i8, 62i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 10i8, 10i8, 73i8, 110i8, 116i8,
    101i8, 114i8, 110i8, 97i8, 108i8, 32i8, 104i8, 101i8, 108i8, 112i8, 101i8, 114i8, 32i8, 102i8,
    117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 98i8,
    121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 32i8, 115i8, 116i8,
    97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 46i8, 0i8,
]);

// [clinic input]
// __import__ as builtin___import__
//
//     name: object
//     globals: object(c_default="NULL") = None
//     locals: object(c_default="NULL") = None
//     fromlist: object(c_default="NULL") = ()
//     level: int = 0
//
// Import a module.
//
// Because this function is meant for use by the Python
// interpreter and not for general use, it is better to use
// importlib.import_module() to programmatically import a module.
//
// The globals argument is only used to determine the context;
// they are not modified.  The locals argument is unused.  The fromlist
// should be a list of names to emulate ``from name import ...``, or an
// empty list to emulate ``import name``.
// When importing a module from a package, note that __import__('A.B', ...)
// returns package A when fromlist is empty, but its submodule B when
// fromlist is not empty.  The level argument is used to determine whether
// to perform absolute or relative imports: 0 is absolute, while a positive
// number is the number of parent directories to search relative to the
// current module.
// [clinic start generated code]
/// [clinic end generated code: output=4febeda88a0cd245 input=e3096a230383f72d]
fn builtin___import___impl(
    module: *mut _object,
    name: *mut _object,
    globals: *mut _object,
    locals: *mut _object,
    fromlist: *mut _object,
    level: i32,
) -> *mut _object {
    unsafe { PyImport_ImportModuleLevelObject(name, globals, locals, fromlist, level) }
}

// [clinic input]
// abs as builtin_abs
//
//     x: object
//     /
//
// Return the absolute value of the argument.
// [clinic start generated code]
/// [clinic end generated code: output=b1b433b9e51356f5 input=bed4ca14e29c20d1]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_abs")]
extern "C-unwind" fn builtin_abs(module: *mut _object, x: *mut _object) -> *mut _object {
    unsafe { PyNumber_Absolute(x) }
}

// [clinic input]
// all as builtin_all
//
//     iterable: object
//     /
//
// Return True if bool(x) is True for all values x in the iterable.
//
// If the iterable is empty, return True.
// [clinic start generated code]
/// [clinic end generated code: output=ca2a7127276f79b3 input=1a7c5d1bc3438a21]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_all")]
extern "C-unwind" fn builtin_all(module: *mut _object, iterable: *mut _object) -> *mut _object {
    let mut cmp: i32 = 0;
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let iternext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(it)).tp_iternext };
    loop {
        let item: *mut _object = unsafe { iternext.unwrap()(it) };
        if item.is_null() {
            break;
        }
        cmp = unsafe { PyObject_IsTrue(item) };
        Py_DECREF!(item);
        if cmp < 0 {
            Py_DECREF!(it);
            return core::ptr::null_mut::<_object>();
        }
        if cmp == 0 {
            Py_DECREF!(it);
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
    }
    Py_DECREF!(it);
    if !unsafe { PyErr_Occurred() }.is_null() {
        if (unsafe { PyErr_ExceptionMatches(PyExc_StopIteration) }) != 0 {
            unsafe { PyErr_Clear() };
        } else {
            return core::ptr::null_mut::<_object>();
        }
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

// [clinic input]
// any as builtin_any
//
//     iterable: object
//     /
//
// Return True if bool(x) is True for any x in the iterable.
//
// If the iterable is empty, return False.
// [clinic start generated code]
/// [clinic end generated code: output=fa65684748caa60e input=41d7451c23384f24]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_any")]
extern "C-unwind" fn builtin_any(module: *mut _object, iterable: *mut _object) -> *mut _object {
    let mut cmp: i32 = 0;
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let iternext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(it)).tp_iternext };
    loop {
        let item: *mut _object = unsafe { iternext.unwrap()(it) };
        if item.is_null() {
            break;
        }
        cmp = unsafe { PyObject_IsTrue(item) };
        Py_DECREF!(item);
        if cmp < 0 {
            Py_DECREF!(it);
            return core::ptr::null_mut::<_object>();
        }
        if cmp > 0 {
            Py_DECREF!(it);
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
        }
    }
    Py_DECREF!(it);
    if !unsafe { PyErr_Occurred() }.is_null() {
        if (unsafe { PyErr_ExceptionMatches(PyExc_StopIteration) }) != 0 {
            unsafe { PyErr_Clear() };
        } else {
            return core::ptr::null_mut::<_object>();
        }
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
}

// [clinic input]
// ascii as builtin_ascii
//
//     obj: object
//     /
//
// Return an ASCII-only representation of an object.
//
// As repr(), return a string containing a printable representation of an
// object, but escape the non-ASCII characters in the string returned by
// repr() using \\x, \\u or \\U escapes. This generates a string similar
// to that returned by repr() in Python 2.
// [clinic start generated code]
/// [clinic end generated code: output=6d37b3f0984c7eb9 input=4c62732e1b3a3cc9]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_ascii")]
extern "C-unwind" fn builtin_ascii(module: *mut _object, obj: *mut _object) -> *mut _object {
    unsafe { PyObject_ASCII(obj) }
}

// [clinic input]
// bin as builtin_bin
//
//     number: object
//     /
//
// Return the binary representation of an integer.
//
//    >>> bin(2796202)
//    '0b1010101010101010101010'
// [clinic start generated code]
/// [clinic end generated code: output=b6fc4ad5e649f4f7 input=53f8a0264bacaf90]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_bin")]
extern "C-unwind" fn builtin_bin(module: *mut _object, number: *mut _object) -> *mut _object {
    unsafe { PyNumber_ToBase(number, 2i32) }
}

// [clinic input]
// callable as builtin_callable
//
//     obj: object
//     /
//
// Return whether the object is callable (i.e., some kind of function).
//
// Note that classes are callable, as are instances of classes with a
// __call__() method.
// [clinic start generated code]
/// [clinic end generated code: output=2b095d59d934cb7e input=1423bab99cc41f58]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_callable")]
extern "C-unwind" fn builtin_callable(module: *mut _object, obj: *mut _object) -> *mut _object {
    unsafe { PyBool_FromLong(PyCallable_Check(obj) as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_breakpoint")]
extern "C-unwind" fn builtin_breakpoint(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    keywords__0: *mut _object,
) -> *mut _object {
    let hook: *mut _object = unsafe { _PySys_GetRequiredAttrString(c"breakpointhook".as_ptr()) };
    if hook.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PySys_Audit(c"builtins.breakpoint".as_ptr(), c"O".as_ptr(), hook) }) < 0 {
        Py_DECREF!(hook);
        return core::ptr::null_mut::<_object>();
    }
    let retval: *mut _object =
        unsafe { PyObject_Vectorcall(hook, args, nargs as u64, keywords__0) };
    Py_DECREF!(hook);
    retval
}

static breakpoint_doc: __SlateAlign16<[i8; 196]> = __SlateAlign16([
    98i8, 114i8, 101i8, 97i8, 107i8, 112i8, 111i8, 105i8, 110i8, 116i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 42i8, 97i8, 114i8, 103i8, 115i8,
    44i8, 32i8, 42i8, 42i8, 107i8, 119i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 97i8,
    108i8, 108i8, 32i8, 115i8, 121i8, 115i8, 46i8, 98i8, 114i8, 101i8, 97i8, 107i8, 112i8, 111i8,
    105i8, 110i8, 116i8, 104i8, 111i8, 111i8, 107i8, 40i8, 42i8, 97i8, 114i8, 103i8, 115i8, 44i8,
    32i8, 42i8, 42i8, 107i8, 119i8, 115i8, 41i8, 46i8, 32i8, 32i8, 115i8, 121i8, 115i8, 46i8, 98i8,
    114i8, 101i8, 97i8, 107i8, 112i8, 111i8, 105i8, 110i8, 116i8, 104i8, 111i8, 111i8, 107i8, 40i8,
    41i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 97i8, 99i8, 99i8, 101i8, 112i8, 116i8, 10i8,
    119i8, 104i8, 97i8, 116i8, 101i8, 118i8, 101i8, 114i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 112i8, 97i8, 115i8, 115i8, 101i8,
    100i8, 46i8, 10i8, 10i8, 66i8, 121i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    44i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 100i8, 114i8, 111i8, 112i8, 115i8, 32i8, 121i8,
    111i8, 117i8, 32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 100i8,
    98i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 103i8, 101i8, 114i8, 46i8, 0i8,
]);

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_497 {
    ob_base: _object,
    func: *mut _object,
    it: *mut _object,
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.filter_new")]
extern "C-unwind" fn filter_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut func: *mut _object = unsafe { core::mem::zeroed() };
    let mut seq: *mut _object = unsafe { core::mem::zeroed() };
    let __v0: bool = (r#type == unsafe { core::ptr::addr_of_mut!(PyFilter_Type) }
        || (unsafe { (*r#type).tp_init }) == unsafe { PyFilter_Type.tp_init })
        && !_PyArg_NoKeywords!(c"filter".as_ptr() as *mut i8, kwds);
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe {
        PyArg_UnpackTuple(
            args,
            c"filter".as_ptr(),
            2i64,
            2i64,
            core::ptr::addr_of_mut!(func),
            core::ptr::addr_of_mut!(seq),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(seq) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create filterobject structure
    let lz: *mut __SlateRecord_bltinmodule_c_497 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_bltinmodule_c_497;
    if lz.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).func = Py_NewRef!(func);
    }
    unsafe {
        (*lz).it = it;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.filter_vectorcall")]
extern "C-unwind" fn filter_vectorcall(
    r#type: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let tp: *mut _typeobject = _PyType_CAST!(r#type);
    let __v0: bool = tp == unsafe { core::ptr::addr_of_mut!(PyFilter_Type) }
        && !_PyArg_NoKwnames!(c"filter".as_ptr() as *mut i8, kwnames);
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    if !_PyArg_CheckPositional!(c"filter".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
        return core::ptr::null_mut::<_object>();
    }
    let it: *mut _object = unsafe { PyObject_GetIter(*args.wrapping_offset(1 as isize)) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let lz: *mut __SlateRecord_bltinmodule_c_497 =
        (unsafe { unsafe { (*tp).tp_alloc }.unwrap()(tp, 0i64) })
            as *mut __SlateRecord_bltinmodule_c_497;
    if lz.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).func = Py_NewRef!(unsafe { *args.wrapping_offset(0 as isize) });
    }
    unsafe {
        (*lz).it = it;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.filter_dealloc")]
extern "C-unwind" fn filter_dealloc(self_: *mut _object) {
    let lz: *mut __SlateRecord_bltinmodule_c_497 = _filterobject_CAST!(self_);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).func });
    Py_XDECREF!(unsafe { (*lz).it });
    unsafe { unsafe { (*Py_TYPE__2!(lz)).tp_free }.unwrap()(lz as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.filter_traverse")]
extern "C-unwind" fn filter_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_bltinmodule_c_497 = _filterobject_CAST!(self_);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).func }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.filter_next")]
extern "C-unwind" fn filter_next(self_: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_bltinmodule_c_497 = _filterobject_CAST!(self_);
    let it: *mut _object = unsafe { (*lz).it };
    let mut ok: i64 = 0;
    let checktrue: i32 = ((unsafe { (*lz).func })
        == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        || (unsafe { (*lz).func })
            == ((unsafe { core::ptr::addr_of_mut!(PyBool_Type) }) as *mut _object))
        as i32;
    let iternext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(it)).tp_iternext };
    loop {
        let item: *mut _object = unsafe { iternext.unwrap()(it) };
        if item.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if checktrue != 0 {
            ok = (unsafe { PyObject_IsTrue(item) }) as i64;
        } else {
            let good: *mut _object = unsafe { PyObject_CallOneArg((*lz).func, item) };
            if good.is_null() {
                Py_DECREF!(item);
                return core::ptr::null_mut::<_object>();
            }
            ok = (unsafe { PyObject_IsTrue(good) }) as i64;
            Py_DECREF!(good);
        }
        if ok > 0 {
            return item;
        }
        Py_DECREF!(item);
        if ok < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.filter_reduce")]
extern "C-unwind" fn filter_reduce(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let lz: *mut __SlateRecord_bltinmodule_c_497 = _filterobject_CAST!(self_);
    unsafe { Py_BuildValue(c"O(OO)".as_ptr(), Py_TYPE__2!(lz), (*lz).func, (*lz).it) }
}

static reduce_doc: __SlateAlign16<[i8; 39]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8,
    110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

/// sentinel
static mut filter_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(filter_reduce);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static filter_doc: __SlateAlign16<[i8; 174]> = __SlateAlign16([
    102i8, 105i8, 108i8, 116i8, 101i8, 114i8, 40i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 110i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 121i8, 105i8, 101i8, 108i8,
    100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 111i8, 115i8, 101i8, 32i8, 105i8, 116i8, 101i8,
    109i8, 115i8, 32i8, 111i8, 102i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 10i8, 102i8, 117i8, 110i8,
    99i8, 116i8, 105i8, 111i8, 110i8, 40i8, 105i8, 116i8, 101i8, 109i8, 41i8, 32i8, 105i8, 115i8,
    32i8, 116i8, 114i8, 117i8, 101i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 102i8, 117i8, 110i8,
    99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8,
    101i8, 109i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 10i8, 97i8, 114i8, 101i8, 32i8, 116i8,
    114i8, 117i8, 101i8, 46i8, 0i8,
]);

// [clinic input]
// format as builtin_format
//
//     value: object
//     format_spec: unicode(c_default="NULL") = ''
//     /
//
// Return type(value).__format__(value, format_spec)
//
// Many built-in types implement format_spec according to the
// Format Specification Mini-language. See help('FORMATTING').
//
// If type(value) does not supply a method named __format__
// and format_spec is empty, then str(value) is returned.
// See also help('SPECIALMETHODS').
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
/// [clinic end generated code: output=2f40bdfa4954b077 input=45ef3934b86d5624]
fn builtin_format_impl(
    module: *mut _object,
    value: *mut _object,
    format_spec: *mut _object,
) -> *mut _object {
    unsafe { PyObject_Format(value, format_spec) }
}

// [clinic input]
// chr as builtin_chr
//
//     i: object
//     /
//
// Return a Unicode string of one character with ordinal i; 0 <= i <= 0x10ffff.
// [clinic start generated code]
/// [clinic end generated code: output=d34f25b8035a9b10 input=f919867f0ba2f496]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_chr")]
extern "C-unwind" fn builtin_chr(module: *mut _object, i: *mut _object) -> *mut _object {
    let mut overflow: i32 = 0;
    let mut v: i64 = unsafe { PyLong_AsLongAndOverflow(i, core::ptr::addr_of_mut!(overflow)) };
    let __v0: bool = v == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if overflow != 0 {
        v = (if overflow < 0 {
            2147483647i32.wrapping_neg().wrapping_sub(1)
        } else {
            2147483647i32
        }) as i64;
    // Allow PyUnicode_FromOrdinal() to raise an exception
    } else {
        if v < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64) {
            v = 2147483647i32.wrapping_neg().wrapping_sub(1) as i64;
        } else {
            if v > 2147483647 {
                v = 2147483647;
            }
        }
    }
    unsafe { PyUnicode_FromOrdinal(v as i32) }
}

// [clinic input]
// compile as builtin_compile
//
//     source: object
//     filename: unicode_fs_decoded
//     mode: str
//     flags: int = 0
//     dont_inherit: bool = False
//     optimize: int = -1
//     *
//     _feature_version as feature_version: int = -1
//
// Compile source into a code object that can be executed by exec() or eval().
//
// The source code may represent a Python module, statement or
// expression.
// The filename will be used for run-time error messages.
// The mode must be 'exec' to compile a module, 'single' to compile a
// single (interactive) statement, or 'eval' to compile an expression.
// The flags argument, if present, controls which future statements
// influence the compilation of the code.
// The dont_inherit argument, if true, stops the compilation inheriting
// the effects of any future statements in effect in the code calling
// compile; if absent or false these statements do influence the
// compilation, in addition to any features explicitly specified.
// [clinic start generated code]
/// [clinic end generated code: output=b0c09c84f116d3d7 input=d69ec2180ff24031]
fn builtin_compile_impl(
    module: *mut _object,
    source: *mut _object,
    filename: *mut _object,
    mode: *const i8,
    flags: i32,
    dont_inherit: i32,
    optimize: i32,
    feature_version: i32,
) -> *mut _object {
    let mut __slate_compound_14: __SlateRecord_compile_h_30 = unsafe { core::mem::zeroed() }; // unoptiomized AST
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _mod = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut r#mod_1: *mut _mod = unsafe { core::mem::zeroed() };
    let mut arena: *mut _arena = unsafe { core::mem::zeroed() };
    let mut msg: *const i8 = unsafe { core::mem::zeroed() };
    let mut cf: core::mem::MaybeUninit<__SlateRecord_compile_h_30> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut __SlateRecord_compile_h_30 =
        core::ptr::addr_of_mut!(cf) as *mut __SlateRecord_compile_h_30;
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut start: core::mem::MaybeUninit<__SlateAlign16<[i32; 4]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [i32; 4] = core::ptr::addr_of_mut!(start) as *mut [i32; 4];
    let mut is_ast: i32 = unsafe { core::mem::zeroed() };
    let mut compile_mode: i32 = unsafe { core::mem::zeroed() };
    let mut str: *const i8 = unsafe { core::mem::zeroed() };
    let mut source_copy: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        compile_mode = 1i32.wrapping_neg();
        core::ptr::write(__slate_slot_9, [257i32, 258i32, 256i32, 345i32]);
        core::ptr::write(__slate_slot_7, *{
            __slate_compound_14 = __SlateRecord_compile_h_30 {
                cf_flags: 0,
                cf_feature_version: 14,
            };
            core::ptr::addr_of_mut!(__slate_compound_14)
        });
        (*__slate_slot_7).cf_flags = flags | 256;
        if feature_version >= 0 && flags & 1024 != 0 {
            (*__slate_slot_7).cf_feature_version = feature_version;
        }
        '__join_0: {
            '__join_1: {
                if flags
                    & !(131072
                        | 262144
                        | 524288
                        | 1048576
                        | 2097152
                        | 4194304
                        | 8388608
                        | 16777216
                        | 16
                        | (1024 | 8192 | 4096 | 512 | 16384 | (32768 | 1024)))
                    != 0
                {
                    PyErr_SetString(PyExc_ValueError, c"compile(): unrecognised flags".as_ptr());
                } else {
                    // XXX Warn if (supplied_flags & PyCF_MASK_OBSOLETE) != 0?
                    if optimize < 1i32.wrapping_neg() || optimize > 2 {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"compile(): invalid optimize value".as_ptr(),
                        );
                    } else {
                        if dont_inherit == 0 {
                            PyEval_MergeCompilerFlags(core::ptr::addr_of_mut!(*__slate_slot_7));
                        }
                        if (strcmp(
                            mode as *const core::ffi::c_char,
                            c"exec".as_ptr() as *const core::ffi::c_char,
                        ) as i32)
                            == 0
                        {
                            compile_mode = 0;
                        } else {
                            if (strcmp(
                                mode as *const core::ffi::c_char,
                                c"eval".as_ptr() as *const core::ffi::c_char,
                            ) as i32)
                                == 0
                            {
                                compile_mode = 1;
                            } else {
                                if (strcmp(
                                    mode as *const core::ffi::c_char,
                                    c"single".as_ptr() as *const core::ffi::c_char,
                                ) as i32)
                                    == 0
                                {
                                    compile_mode = 2;
                                } else {
                                    if (strcmp(
                                        mode as *const core::ffi::c_char,
                                        c"func_type".as_ptr() as *const core::ffi::c_char,
                                    ) as i32)
                                        == 0
                                    {
                                        if flags & 1024 == 0 {
                                            PyErr_SetString(PyExc_ValueError, c"compile() mode 'func_type' requires flag PyCF_ONLY_AST".as_ptr());
                                            break '__join_1;
                                        } else {
                                            compile_mode = 3;
                                        }
                                    } else {
                                        if flags & 1024 != 0 {
                                            msg = c"compile() mode must be 'exec', 'eval', 'single' or 'func_type'".as_ptr();
                                        } else {
                                            msg = c"compile() mode must be 'exec', 'eval' or 'single'".as_ptr();
                                        }
                                        PyErr_SetString(PyExc_ValueError, msg);
                                        break '__join_1;
                                    }
                                }
                            }
                        }
                        is_ast = PyAST_Check(source);
                        if is_ast != 1i32.wrapping_neg() {
                            if is_ast != 0 {
                                arena = _PyArena_New();
                                if !arena.is_null() {
                                    if flags & 1024 != 0 {
                                        r#mod = PyAST_obj2mod(source, arena, compile_mode);
                                        if r#mod.is_null() {
                                            __v1 = true;
                                        } else {
                                            __v1 = _PyAST_Validate(r#mod) == 0;
                                        }
                                        if __v1 {
                                            _PyArena_Free(arena);
                                            break '__join_1;
                                        } else {
                                            let syntax_check_only: i32 =
                                                (flags & (32768 | 1024) == 1024) as i32;
                                            if _PyCompile_AstPreprocess(
                                                r#mod,
                                                filename,
                                                core::ptr::addr_of_mut!(*__slate_slot_7),
                                                optimize,
                                                arena,
                                                syntax_check_only,
                                            ) < 0
                                            {
                                                _PyArena_Free(arena);
                                                break '__join_1;
                                            } else {
                                                result = PyAST_mod2obj(r#mod);
                                            }
                                        }
                                    } else {
                                        r#mod_1 = PyAST_obj2mod(source, arena, compile_mode);
                                        if r#mod_1.is_null() {
                                            __v3 = true;
                                        } else {
                                            __v3 = _PyAST_Validate(r#mod_1) == 0;
                                        }
                                        if __v3 {
                                            _PyArena_Free(arena);
                                            break '__join_1;
                                        } else {
                                            result = _PyAST_Compile(
                                                r#mod_1,
                                                filename,
                                                core::ptr::addr_of_mut!(*__slate_slot_7),
                                                optimize,
                                                arena,
                                            )
                                                as *mut _object;
                                        }
                                    }
                                    _PyArena_Free(arena);
                                    break '__join_0;
                                }
                            } else {
                                str = _Py_SourceAsString(
                                    source,
                                    c"compile".as_ptr(),
                                    c"string, bytes or AST".as_ptr(),
                                    core::ptr::addr_of_mut!(*__slate_slot_7),
                                    core::ptr::addr_of_mut!(source_copy),
                                );
                                if !str.is_null() {
                                    result = Py_CompileStringObject(
                                        str,
                                        filename,
                                        (*__slate_slot_9)[compile_mode as usize],
                                        core::ptr::addr_of_mut!(*__slate_slot_7),
                                        optimize,
                                    );
                                    Py_XDECREF!(source_copy);
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
            }
            result = core::ptr::null_mut::<_object>();
        }
        return result;
    }
}

/// AC: cannot convert yet, as needs PEP 457 group support in inspect
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_dir")]
extern "C-unwind" fn builtin_dir(self_: *mut _object, args: *mut _object) -> *mut _object {
    let mut arg: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyArg_UnpackTuple(
            args,
            c"dir".as_ptr(),
            0i64,
            1i64,
            core::ptr::addr_of_mut!(arg),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyObject_Dir(arg) }
}

static dir_doc: __SlateAlign16<[i8; 624]> = __SlateAlign16([
    100i8, 105i8, 114i8, 40i8, 91i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 93i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 111i8, 102i8, 32i8, 115i8, 116i8, 114i8,
    105i8, 110i8, 103i8, 115i8, 10i8, 10i8, 73i8, 102i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8,
    100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 111i8, 117i8, 116i8, 32i8, 97i8, 110i8, 32i8, 97i8,
    114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 97i8, 109i8, 101i8, 115i8, 32i8, 105i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8,
    99i8, 111i8, 112i8, 101i8, 46i8, 10i8, 69i8, 108i8, 115i8, 101i8, 44i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 97i8, 108i8, 112i8, 104i8, 97i8, 98i8,
    101i8, 116i8, 105i8, 122i8, 101i8, 100i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 111i8, 102i8,
    32i8, 110i8, 97i8, 109i8, 101i8, 115i8, 32i8, 99i8, 111i8, 109i8, 112i8, 114i8, 105i8, 115i8,
    105i8, 110i8, 103i8, 32i8, 40i8, 115i8, 111i8, 109i8, 101i8, 32i8, 111i8, 102i8, 41i8, 32i8,
    116i8, 104i8, 101i8, 10i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 111i8, 102i8,
    32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 32i8, 114i8, 101i8,
    97i8, 99i8, 104i8, 97i8, 98i8, 108i8, 101i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 105i8,
    116i8, 46i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 32i8, 115i8, 117i8, 112i8, 112i8, 108i8, 105i8, 101i8, 115i8, 32i8, 97i8, 32i8,
    109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 95i8,
    95i8, 100i8, 105i8, 114i8, 95i8, 95i8, 44i8, 32i8, 105i8, 116i8, 32i8, 119i8, 105i8, 108i8,
    108i8, 32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 59i8, 10i8, 111i8, 116i8, 104i8,
    101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 32i8, 100i8, 105i8, 114i8, 40i8, 41i8, 32i8, 108i8, 111i8, 103i8,
    105i8, 99i8, 32i8, 105i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 58i8, 10i8, 32i8, 32i8, 102i8, 111i8,
    114i8, 32i8, 97i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 58i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 39i8, 115i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8,
    46i8, 10i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 58i8, 32i8, 32i8, 105i8, 116i8, 115i8, 32i8,
    97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 44i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 118i8, 101i8, 108i8, 121i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 111i8, 102i8, 32i8, 105i8, 116i8, 115i8, 32i8, 98i8, 97i8, 115i8,
    101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 110i8, 121i8, 32i8,
    111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 58i8, 32i8,
    105i8, 116i8, 115i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8,
    44i8, 32i8, 105i8, 116i8, 115i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 39i8, 115i8, 32i8,
    97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 44i8, 32i8, 97i8, 110i8,
    100i8, 10i8, 32i8, 32i8, 32i8, 32i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 118i8,
    101i8, 108i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8,
    117i8, 116i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 105i8, 116i8, 115i8, 32i8, 99i8, 108i8,
    97i8, 115i8, 115i8, 39i8, 115i8, 32i8, 98i8, 97i8, 115i8, 101i8, 32i8, 99i8, 108i8, 97i8,
    115i8, 115i8, 101i8, 115i8, 46i8, 0i8,
]);

// [clinic input]
// divmod as builtin_divmod
//
//     x: object
//     y: object
//     /
//
// Return the tuple (x//y, x%y).  Invariant: div*y + mod == x.
// [clinic start generated code]
/// [clinic end generated code: output=b06d8a5f6e0c745e input=175ad9c84ff41a85]
fn builtin_divmod_impl(module: *mut _object, x: *mut _object, y: *mut _object) -> *mut _object {
    unsafe { PyNumber_Divmod(x, y) }
}

// [clinic input]
// eval as builtin_eval
//
//     source: object
//     /
//     globals: object = None
//     locals: object = None
//
// Evaluate the given source in the context of globals and locals.
//
// The source may be a string representing a Python expression
// or a code object as returned by compile().
// The globals must be a dictionary and locals can be any mapping,
// defaulting to the current globals and locals.
// If only globals is given, locals defaults to it.
// [clinic start generated code]
/// [clinic end generated code: output=0a0824aa70093116 input=7c7bce5299a89062]
fn builtin_eval_impl(
    module: *mut _object,
    source: *mut _object,
    mut globals: *mut _object,
    mut locals: *mut _object,
) -> *mut _object {
    let mut __slate_compound_10: __SlateRecord_compile_h_30 = unsafe { core::mem::zeroed() };
    let mut cf: core::mem::MaybeUninit<__SlateRecord_compile_h_30> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_compile_h_30 =
        core::ptr::addr_of_mut!(cf) as *mut __SlateRecord_compile_h_30;
    let mut fromframe: i32 = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut str: *const i8 = unsafe { core::mem::zeroed() };
    let mut source_copy: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    unsafe {
        tstate = _PyThreadState_GET();
        result = core::ptr::null_mut::<_object>();
        if locals != core::ptr::addr_of_mut!(_Py_NoneStruct) {
            __v5 = PyMapping_Check(locals) == 0;
        } else {
            __v5 = false;
        }
        if __v5 {
            PyErr_SetString(PyExc_TypeError, c"locals must be a mapping".as_ptr());
            return core::ptr::null_mut::<_object>();
        } else {
            if globals != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                __v4 = PyDict_Check!(globals) == 0;
            } else {
                __v4 = false;
            }
            if __v4 {
                PyErr_SetString(
                    PyExc_TypeError,
                    (if PyMapping_Check(globals) != 0 {
                        c"globals must be a real dict; try eval(expr, {}, mapping)".as_ptr()
                            as *mut i8
                    } else {
                        c"globals must be a dict".as_ptr() as *mut i8
                    }) as *const i8,
                );
                return core::ptr::null_mut::<_object>();
            } else {
                fromframe = 0;
                if globals != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    Py_INCREF!(globals);
                } else {
                    if !_PyEval_GetFrame().is_null() {
                        fromframe = 1;
                        globals = PyEval_GetGlobals();
                        Py_INCREF!(globals);
                    } else {
                        globals = _PyEval_GetGlobalsFromRunningMain(tstate);
                        if globals.is_null() {
                            if _PyErr_Occurred(tstate).is_null() {
                                PyErr_SetString(PyExc_TypeError, c"eval must be given globals and locals when called without a frame".as_ptr());
                            }
                            return core::ptr::null_mut::<_object>();
                        } else {
                            Py_INCREF!(globals);
                        }
                    }
                }
                if locals != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    Py_INCREF!(locals);
                } else {
                    if fromframe != 0 {
                        locals = _PyEval_GetFrameLocals();
                        if locals.is_null() {
                            Py_DECREF!(globals);
                            return core::ptr::null_mut::<_object>();
                        }
                    } else {
                        locals = Py_NewRef!(globals);
                    }
                }
                if _PyEval_EnsureBuiltins(tstate, globals, core::ptr::null_mut::<*mut _object>())
                    >= 0
                {
                    if PyCode_Check!(source) != 0 {
                        if PySys_Audit(c"exec".as_ptr(), c"O".as_ptr(), source) >= 0 {
                            if PyCode_GetNumFree(source as *mut PyCodeObject) > 0 {
                                PyErr_SetString(
                                    PyExc_TypeError,
                                    c"code object passed to eval() may not contain free variables"
                                        .as_ptr(),
                                );
                            } else {
                                result = PyEval_EvalCode(source, globals, locals);
                            }
                        }
                    } else {
                        core::ptr::write(__slate_slot_2, *{
                            __slate_compound_10 = __SlateRecord_compile_h_30 {
                                cf_flags: 0,
                                cf_feature_version: 14,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_10)
                        });
                        (*__slate_slot_2).cf_flags = 256;
                        str = _Py_SourceAsString(
                            source,
                            c"eval".as_ptr(),
                            c"string, bytes or code".as_ptr(),
                            core::ptr::addr_of_mut!(*__slate_slot_2),
                            core::ptr::addr_of_mut!(source_copy),
                        );
                        if !str.is_null() {
                            while (*str as i32) == 32 || (*str as i32) == 9 {
                                let __v1: *const i8 = str;
                                str = __v1.wrapping_offset(1 as isize);
                            }
                            PyEval_MergeCompilerFlags(core::ptr::addr_of_mut!(*__slate_slot_2));
                            result = PyRun_StringFlags(
                                str,
                                258i32,
                                globals,
                                locals,
                                core::ptr::addr_of_mut!(*__slate_slot_2),
                            );
                            Py_XDECREF!(source_copy);
                        }
                    }
                }
                Py_XDECREF!(globals);
                Py_XDECREF!(locals);
                return result;
            }
        }
    }
}

// [clinic input]
// exec as builtin_exec
//
//     source: object
//     /
//     globals: object = None
//     locals: object = None
//     *
//     closure: object(c_default="NULL") = None
//
// Execute the given source in the context of globals and locals.
//
// The source may be a string representing one or more Python statements
// or a code object as returned by compile().
// The globals must be a dictionary and locals can be any mapping,
// defaulting to the current globals and locals.
// If only globals is given, locals defaults to it.
// The closure must be a tuple of cellvars, and can only be used
// when source is a code object requiring exactly that many cellvars.
// [clinic start generated code]
/// [clinic end generated code: output=7579eb4e7646743d input=25e989b6d87a3a21]
fn builtin_exec_impl(
    module: *mut _object,
    source: *mut _object,
    mut globals: *mut _object,
    mut locals: *mut _object,
    mut closure: *mut _object,
) -> *mut _object {
    let mut __slate_compound_14: __SlateRecord_compile_h_30 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut closure_is_ok: i32 = unsafe { core::mem::zeroed() };
    let mut num_free: i64 = unsafe { core::mem::zeroed() };
    let mut cf: core::mem::MaybeUninit<__SlateRecord_compile_h_30> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut __SlateRecord_compile_h_30 =
        core::ptr::addr_of_mut!(cf) as *mut __SlateRecord_compile_h_30;
    let mut str: *const i8 = unsafe { core::mem::zeroed() };
    let mut source_copy: *mut _object = unsafe { core::mem::zeroed() };
    let mut fromframe: i32 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            tstate = _PyThreadState_GET();
            fromframe = 0;
            if globals != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                Py_INCREF!(globals);
            } else {
                if !_PyEval_GetFrame().is_null() {
                    fromframe = 1;
                    globals = PyEval_GetGlobals();
                    Py_INCREF!(globals);
                } else {
                    globals = _PyEval_GetGlobalsFromRunningMain(tstate);
                    if globals.is_null() {
                        if _PyErr_Occurred(tstate).is_null() {
                            PyErr_SetString(
                                PyExc_SystemError,
                                c"globals and locals cannot be NULL".as_ptr(),
                            );
                            break '__join_0;
                        } else {
                            break '__join_0;
                        }
                    } else {
                        Py_INCREF!(globals);
                    }
                }
            }
            if locals != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                Py_INCREF!(locals);
            } else {
                if fromframe != 0 {
                    locals = _PyEval_GetFrameLocals();
                    if locals.is_null() {
                        Py_DECREF!(globals);
                        return core::ptr::null_mut::<_object>();
                    }
                } else {
                    locals = Py_NewRef!(globals);
                }
            }
            if PyDict_Check!(globals) == 0 {
                PyErr_Format(
                    PyExc_TypeError,
                    c"exec() globals must be a dict, not %.100s".as_ptr(),
                    (*Py_TYPE!(globals)).tp_name,
                );
            } else {
                if PyMapping_Check(locals) == 0 {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"locals must be a mapping or None, not %.100s".as_ptr(),
                        (*Py_TYPE!(locals)).tp_name,
                    );
                } else {
                    if _PyEval_EnsureBuiltins(
                        tstate,
                        globals,
                        core::ptr::null_mut::<*mut _object>(),
                    ) >= 0
                    {
                        if closure == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            closure = core::ptr::null_mut::<_object>();
                        }
                        if PyCode_Check!(source) != 0 {
                            num_free = PyCode_GetNumFree(source as *mut PyCodeObject);
                            if num_free == 0 {
                                if !closure.is_null() {
                                    PyErr_SetString(
                                        PyExc_TypeError,
                                        c"cannot use a closure with this code object".as_ptr(),
                                    );
                                    break '__join_0;
                                }
                            } else {
                                if !closure.is_null() {
                                    __v5 = PyTuple_CheckExact!(closure) != 0;
                                } else {
                                    __v5 = false;
                                }
                                if __v5 {
                                    __v4 = PyTuple_GET_SIZE!(closure) == num_free;
                                } else {
                                    __v4 = false;
                                }
                                '__join_10: {
                                    closure_is_ok = __v4 as i32;
                                    if closure_is_ok != 0 {
                                        i = 0;
                                        loop {
                                            if i < num_free {
                                                let cell: *mut _object =
                                                    PyTuple_GET_ITEM!(closure, i);
                                                if PyCell_Check!(cell) == 0 {
                                                    break;
                                                } else {
                                                    i = i.wrapping_add(1);
                                                }
                                            } else {
                                                break '__join_10;
                                            }
                                        }
                                        closure_is_ok = 0;
                                    }
                                }
                                if closure_is_ok == 0 {
                                    PyErr_Format(
                                        PyExc_TypeError,
                                        c"code object requires a closure of exactly length %zd"
                                            .as_ptr(),
                                        num_free,
                                    );
                                    break '__join_0;
                                }
                            }
                            if PySys_Audit(c"exec".as_ptr(), c"O".as_ptr(), source) < 0 {
                                break '__join_0;
                            } else {
                                if closure.is_null() {
                                    v = PyEval_EvalCode(source, globals, locals);
                                } else {
                                    v = PyEval_EvalCodeEx(
                                        source,
                                        globals,
                                        locals,
                                        core::ptr::null::<*mut _object>(),
                                        0i32,
                                        core::ptr::null::<*mut _object>(),
                                        0i32,
                                        core::ptr::null::<*mut _object>(),
                                        0i32,
                                        core::ptr::null_mut::<_object>(),
                                        closure,
                                    );
                                }
                            }
                        } else {
                            if !closure.is_null() {
                                PyErr_SetString(
                                    PyExc_TypeError,
                                    c"closure can only be used when source is a code object"
                                        .as_ptr(),
                                );
                                break '__join_0;
                            } else {
                                core::ptr::write(__slate_slot_8, *{
                                    __slate_compound_14 = __SlateRecord_compile_h_30 {
                                        cf_flags: 0,
                                        cf_feature_version: 14,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_14)
                                });
                                (*__slate_slot_8).cf_flags = 256;
                                str = _Py_SourceAsString(
                                    source,
                                    c"exec".as_ptr(),
                                    c"string, bytes or code".as_ptr(),
                                    core::ptr::addr_of_mut!(*__slate_slot_8),
                                    core::ptr::addr_of_mut!(source_copy),
                                );
                                if str.is_null() {
                                    break '__join_0;
                                } else {
                                    if PyEval_MergeCompilerFlags(core::ptr::addr_of_mut!(
                                        *__slate_slot_8
                                    )) != 0
                                    {
                                        v = PyRun_StringFlags(
                                            str,
                                            257i32,
                                            globals,
                                            locals,
                                            core::ptr::addr_of_mut!(*__slate_slot_8),
                                        );
                                    } else {
                                        v = PyRun_String!(str, 257i32, globals, locals);
                                    }
                                    Py_XDECREF!(source_copy);
                                }
                            }
                        }
                        if !v.is_null() {
                            Py_DECREF!(globals);
                            Py_DECREF!(locals);
                            Py_DECREF!(v);
                            return core::ptr::addr_of_mut!(_Py_NoneStruct);
                        }
                    }
                }
            }
        }
        Py_XDECREF!(globals);
        Py_XDECREF!(locals);
        return core::ptr::null_mut::<_object>();
    }
}

/// AC: cannot convert yet, as needs PEP 457 group support in inspect
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_getattr")]
extern "C-unwind" fn builtin_getattr(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if !_PyArg_CheckPositional!(c"getattr".as_ptr() as *mut i8, nargs, 2i32, 3i32) {
        return core::ptr::null_mut::<_object>();
    }
    let v: *mut _object = unsafe { *args.wrapping_offset(0 as isize) };
    let name: *mut _object = unsafe { *args.wrapping_offset(1 as isize) };
    if nargs > 2 {
        if (unsafe { PyObject_GetOptionalAttr(v, name, core::ptr::addr_of_mut!(result)) }) == 0 {
            let dflt: *mut _object = unsafe { *args.wrapping_offset(2 as isize) };
            return Py_NewRef!(dflt);
        }
    } else {
        result = unsafe { PyObject_GetAttr(v, name) };
    }
    result
}

static getattr_doc: __SlateAlign16<[i8; 252]> = __SlateAlign16([
    103i8, 101i8, 116i8, 97i8, 116i8, 116i8, 114i8, 40i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 91i8, 44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 10i8, 10i8,
    71i8, 101i8, 116i8, 32i8, 97i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 97i8, 116i8,
    116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8,
    110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 103i8, 101i8, 116i8,
    97i8, 116i8, 116i8, 114i8, 40i8, 120i8, 44i8, 32i8, 39i8, 121i8, 39i8, 41i8, 32i8, 105i8,
    115i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8, 32i8, 116i8,
    111i8, 32i8, 120i8, 46i8, 121i8, 46i8, 10i8, 87i8, 104i8, 101i8, 110i8, 32i8, 97i8, 32i8,
    100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8, 105i8,
    116i8, 32i8, 105i8, 115i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8,
    119i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8,
    98i8, 117i8, 116i8, 101i8, 10i8, 100i8, 111i8, 101i8, 115i8, 110i8, 39i8, 116i8, 32i8, 101i8,
    120i8, 105i8, 115i8, 116i8, 59i8, 32i8, 119i8, 105i8, 116i8, 104i8, 111i8, 117i8, 116i8, 32i8,
    105i8, 116i8, 44i8, 32i8, 97i8, 110i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 100i8, 32i8, 105i8,
    110i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 97i8, 115i8, 101i8, 46i8, 0i8,
]);

// [clinic input]
// globals as builtin_globals
//
// Return the dictionary containing the current scope's global variables.
//
// NOTE: Updates to this dictionary *will* affect name lookups in the
// current global scope and vice-versa.
// [clinic start generated code]
/// [clinic end generated code: output=e5dd1527067b94d2 input=6d725a9b48d1eaeb]
fn builtin_globals_impl(module: *mut _object) -> *mut _object {
    let mut globals: *mut _object = unsafe { core::mem::zeroed() };
    if !unsafe { _PyEval_GetFrame() }.is_null() {
        globals = unsafe { PyEval_GetGlobals() };
        return Py_NewRef!(globals);
    }
    let tstate: *mut _ts = _PyThreadState_GET();
    globals = unsafe { _PyEval_GetGlobalsFromRunningMain(tstate) };
    if globals.is_null() {
        if !_PyErr_Occurred(tstate).is_null() {
            return core::ptr::null_mut::<_object>();
        }
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(globals)
}

// [clinic input]
// hasattr as builtin_hasattr
//
//     obj: object
//     name: object
//     /
//
// Return whether the object has an attribute with the given name.
//
// This is done by calling getattr(obj, name) and catching AttributeError.
// [clinic start generated code]
/// [clinic end generated code: output=a7aff2090a4151e5 input=0faec9787d979542]
fn builtin_hasattr_impl(
    module: *mut _object,
    obj: *mut _object,
    name: *mut _object,
) -> *mut _object {
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyObject_GetOptionalAttr(obj, name, core::ptr::addr_of_mut!(v)) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if v.is_null() {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    Py_DECREF!(v);
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

// AC: gdb's integration with CPython relies on builtin_id having
// the *exact* parameter names of "self" and "v", so we ensure we
// preserve those name rather than using the AC defaults.
//
// [clinic input]
// id as builtin_id
//
//     self: self(type="PyModuleDef *")
//     obj as v: object
//     /
//
// Return the identity of an object.
//
// This is guaranteed to be unique among simultaneously existing objects.
// (CPython uses the object's memory address.)
// [clinic start generated code]
/// [clinic end generated code: output=4908a6782ed343e9 input=5a534136419631f4]
fn builtin_id_impl(self_: *mut PyModuleDef, v: *mut _object) -> *mut _object {
    let id: *mut _object = unsafe { PyLong_FromVoidPtr(v as *mut ()) };
    let __v0: bool =
        !id.is_null() && (unsafe { PySys_Audit(c"builtins.id".as_ptr(), c"O".as_ptr(), id) }) < 0;
    if __v0 {
        Py_DECREF!(id);
        return core::ptr::null_mut::<_object>();
    }
    id
}

// map object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_1361 {
    ob_base: _object,
    iters: *mut _object,
    func: *mut _object,
    strict: i32,
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_new")]
extern "C-unwind" fn map_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut numargs: i64 = 0;
    let mut strict: i32 = 0;
    if !kwds.is_null() {
        let empty: *mut _object = unsafe { PyTuple_New(0i64) };
        if empty.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let parsed: i32 = unsafe {
            PyArg_ParseTupleAndKeywords(
                empty,
                kwds,
                c"|$p:map".as_ptr(),
                (core::ptr::addr_of_mut!(kwlist__1378.0) as *mut *mut i8) as *const *mut i8,
                core::ptr::addr_of_mut!(strict),
            )
        };
        Py_DECREF!(empty);
        if parsed == 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    numargs = unsafe { PyTuple_Size(args) };
    if numargs < 2 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"map() must have at least two arguments.".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let iters: *mut _object = unsafe { PyTuple_New(numargs.wrapping_sub(1)) };
    if iters.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    for i in 1..numargs {
        // Get iterator.
        let it: *mut _object = unsafe { PyObject_GetIter(PyTuple_GET_ITEM!(args, i)) };
        if it.is_null() {
            Py_DECREF!(iters);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(iters, i.wrapping_sub(1), it);
    }
    // create mapobject structure
    let lz: *mut __SlateRecord_bltinmodule_c_1361 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_bltinmodule_c_1361;
    if lz.is_null() {
        Py_DECREF!(iters);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).iters = iters;
    }
    let func: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
    unsafe {
        (*lz).func = Py_NewRef!(func);
    }
    unsafe {
        (*lz).strict = strict;
    }
    lz as *mut _object
}

static mut kwlist__1378: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"strict\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_vectorcall")]
extern "C-unwind" fn map_vectorcall(
    r#type: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let tp: *mut _typeobject = _PyType_CAST!(r#type);
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    let __v0: bool = !kwnames.is_null() && PyTuple_GET_SIZE!(kwnames) != 0;
    if __v0 {
        // Fallback to map_new()
        let tstate: *mut _ts = _PyThreadState_GET();
        return unsafe { _PyObject_MakeTpCall(tstate, r#type, args, nargs, kwnames) };
    }
    if nargs < 2 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"map() must have at least two arguments.".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let iters: *mut _object = unsafe { PyTuple_New(nargs.wrapping_sub(1)) };
    if iters.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut i: i32 = 1;
    while (i as i64) < nargs {
        let it: *mut _object = unsafe { PyObject_GetIter(*args.wrapping_offset(i as isize)) };
        if it.is_null() {
            Py_DECREF!(iters);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM!(iters, i.wrapping_sub(1), it);
        i = i.wrapping_add(1);
    }
    let lz: *mut __SlateRecord_bltinmodule_c_1361 =
        (unsafe { unsafe { (*tp).tp_alloc }.unwrap()(tp, 0i64) })
            as *mut __SlateRecord_bltinmodule_c_1361;
    if lz.is_null() {
        Py_DECREF!(iters);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).iters = iters;
    }
    unsafe {
        (*lz).func = Py_NewRef!(unsafe { *args.wrapping_offset(0 as isize) });
    }
    unsafe {
        (*lz).strict = 0;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_dealloc")]
extern "C-unwind" fn map_dealloc(self_: *mut _object) {
    let lz: *mut __SlateRecord_bltinmodule_c_1361 = _mapobject_CAST!(self_);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).iters });
    Py_XDECREF!(unsafe { (*lz).func });
    unsafe { unsafe { (*Py_TYPE__2!(lz)).tp_free }.unwrap()(lz as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_traverse")]
extern "C-unwind" fn map_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_bltinmodule_c_1361 = _mapobject_CAST!(self_);
    Py_VISIT!(unsafe { (*lz).iters }, visit, arg);
    Py_VISIT!(unsafe { (*lz).func }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_next")]
extern "C-unwind" fn map_next(self_: *mut _object) -> *mut _object {
    let mut val: *mut _object = unsafe { core::mem::zeroed() };
    let mut val_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut niters: i64 = unsafe { core::mem::zeroed() };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut stack: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut small_stack: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 5]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_19: *mut [*mut _object; 5] =
        core::ptr::addr_of_mut!(small_stack) as *mut [*mut _object; 5];
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut lz: *mut __SlateRecord_bltinmodule_c_1361 = unsafe { core::mem::zeroed() };
    unsafe {
        lz = _mapobject_CAST!(self_);
        result = core::ptr::null_mut::<_object>();
        tstate = _PyThreadState_GET();
        niters = PyTuple_GET_SIZE!(unsafe { (*lz).iters });
        if niters <= (Py_ARRAY_LENGTH__2!() as i64) {
            stack = (*__slate_slot_19).as_mut_ptr() as *mut *mut _object;
        } else {
            stack = PyMem_Malloc(
                (niters as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
            ) as *mut *mut _object;
            if stack.is_null() {
                _PyErr_NoMemory(tstate);
                return core::ptr::null_mut::<_object>();
            }
        }
        nargs = 0;
        i = 0;
        '__join_5: {
            '__join_26: {
                while i < niters {
                    let it: *mut _object = PyTuple_GET_ITEM!(unsafe { (*lz).iters }, i);
                    val_1 = unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it);
                    if val_1.is_null() {
                        break '__join_26;
                    } else {
                        *stack.wrapping_offset(i as isize) = val_1;
                        nargs = nargs.wrapping_add(1);
                        i = i.wrapping_add(1);
                    }
                }
                result = _PyObject_VectorcallTstate(
                    tstate,
                    (*lz).func,
                    stack as *const *mut _object,
                    nargs as u64,
                    core::ptr::null_mut::<_object>(),
                );
                break '__join_5;
            }
            '__join_6: {
                if (*lz).strict != 0 {
                    if !PyErr_Occurred().is_null() {
                        if PyErr_ExceptionMatches(PyExc_StopIteration) == 0 {
                            // next() on argument i raised an exception (not StopIteration)
                            break '__join_6;
                        } else {
                            PyErr_Clear();
                        }
                    }
                    if i != 0 {
                        // ValueError: map() argument 2 is shorter than argument 1
                        //
                        // ValueError: map() argument 3 is shorter than arguments 1-2
                        let plural: *const i8 = (if i == 1 {
                            c" ".as_ptr() as *mut i8
                        } else {
                            c"s 1-".as_ptr() as *mut i8
                        }) as *const i8;
                        PyErr_Format(
                            PyExc_ValueError,
                            c"map() argument %zd is shorter than argument%s%zd".as_ptr(),
                            i.wrapping_add(1),
                            plural,
                            i,
                        );
                    } else {
                        i = 1;
                        '__join_14: {
                            '__join_11: {
                                while i < niters {
                                    let it: *mut _object =
                                        PyTuple_GET_ITEM!(unsafe { (*lz).iters }, i);
                                    val = unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it);
                                    if !val.is_null() {
                                        break '__join_14;
                                    } else {
                                        if !PyErr_Occurred().is_null() {
                                            if PyErr_ExceptionMatches(PyExc_StopIteration) == 0 {
                                                break '__join_11;
                                            } else {
                                                PyErr_Clear();
                                            }
                                        }
                                        // Argument i is exhausted. So far so good...
                                        i = i.wrapping_add(1);
                                    }
                                }
                                // All arguments are exhausted. Success!
                                break '__join_6;
                            }
                            // next() on argument i raised an exception (not StopIteration)
                            break '__join_6;
                        }
                        Py_DECREF!(val);
                        let plural: *const i8 = (if i == 1 {
                            c" ".as_ptr() as *mut i8
                        } else {
                            c"s 1-".as_ptr() as *mut i8
                        }) as *const i8;
                        PyErr_Format(
                            PyExc_ValueError,
                            c"map() argument %zd is longer than argument%s%zd".as_ptr(),
                            i.wrapping_add(1),
                            plural,
                            i,
                        );
                    }
                }
            }
        }
        i = 0;
        while i < nargs {
            Py_DECREF!(unsafe { *stack.wrapping_offset(i as isize) });
            i = i.wrapping_add(1);
        }
        if stack != ((*__slate_slot_19).as_mut_ptr() as *mut *mut _object) {
            PyMem_Free(stack as *mut ());
        }
        return result;
    }
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_reduce")]
extern "C-unwind" fn map_reduce(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let lz: *mut __SlateRecord_bltinmodule_c_1361 = _mapobject_CAST!(self_);
    let numargs: i64 = PyTuple_GET_SIZE!(unsafe { (*lz).iters });
    let args: *mut _object = unsafe { PyTuple_New(numargs.wrapping_add(1)) };
    if args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    PyTuple_SET_ITEM!(args, 0i32, Py_NewRef!(unsafe { (*lz).func }));
    for i in 0..numargs {
        let it: *mut _object = PyTuple_GET_ITEM!(unsafe { (*lz).iters }, i);
        PyTuple_SET_ITEM__2!(args, i.wrapping_add(1), Py_NewRef!(it));
    }
    if (unsafe { (*lz).strict }) != 0 {
        return unsafe {
            Py_BuildValue(
                c"ONO".as_ptr(),
                Py_TYPE__2!(lz),
                args,
                _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
            )
        };
    }
    unsafe { Py_BuildValue(c"ON".as_ptr(), Py_TYPE__2!(lz), args) }
}

static setstate_doc: __SlateAlign16<[i8; 38]> = __SlateAlign16([
    83i8, 101i8, 116i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8, 32i8, 117i8, 110i8,
    112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.map_setstate")]
extern "C-unwind" fn map_setstate(self_: *mut _object, state: *mut _object) -> *mut _object {
    let strict: i32 = unsafe { PyObject_IsTrue(state) };
    if strict < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let lz: *mut __SlateRecord_bltinmodule_c_1361 = _mapobject_CAST!(self_);
    unsafe {
        (*lz).strict = strict;
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

/// sentinel
static mut map_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(map_reduce);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(map_setstate);
        __t1.ml_flags = 8i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(setstate_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

static map_doc: __SlateAlign16<[i8; 289]> = __SlateAlign16([
    109i8, 97i8, 112i8, 40i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 42i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 115i8, 44i8, 32i8, 115i8, 116i8, 114i8,
    105i8, 99i8, 116i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    77i8, 97i8, 107i8, 101i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8,
    111i8, 114i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 111i8, 109i8, 112i8, 117i8, 116i8,
    101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 10i8, 101i8, 97i8, 99i8, 104i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 115i8, 46i8, 32i8, 32i8, 83i8, 116i8, 111i8, 112i8, 115i8, 32i8, 119i8, 104i8, 101i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 104i8, 111i8, 114i8, 116i8, 101i8, 115i8, 116i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8,
    120i8, 104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8,
    115i8, 116i8, 114i8, 105i8, 99i8, 116i8, 32i8, 105i8, 115i8, 32i8, 116i8, 114i8, 117i8, 101i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 111i8, 110i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 115i8,
    32i8, 101i8, 120i8, 104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 32i8, 98i8, 101i8, 102i8,
    111i8, 114i8, 101i8, 32i8, 116i8, 104i8, 101i8, 10i8, 111i8, 116i8, 104i8, 101i8, 114i8, 115i8,
    44i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 32i8, 97i8, 32i8, 86i8, 97i8, 108i8, 117i8,
    101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 46i8, 0i8,
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
/// AC: cannot convert yet, as needs PEP 457 group support in inspect
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_next")]
extern "C-unwind" fn builtin_next(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    if !_PyArg_CheckPositional!(c"next".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
        return core::ptr::null_mut::<_object>();
    }
    let it: *mut _object = unsafe { *args.wrapping_offset(0 as isize) };
    if (unsafe { PyIter_Check(it) }) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"'%.200s' object is not an iterator".as_ptr(),
                (*Py_TYPE!(it)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let res: *mut _object = unsafe { unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it) };
    if !res.is_null() {
        return res;
    } else {
        if nargs > 1 {
            let def: *mut _object = unsafe { *args.wrapping_offset(1 as isize) };
            if !unsafe { PyErr_Occurred() }.is_null() {
                if (unsafe { PyErr_ExceptionMatches(PyExc_StopIteration) }) == 0 {
                    return core::ptr::null_mut::<_object>();
                }
                unsafe { PyErr_Clear() };
            }
            return Py_NewRef!(def);
        } else {
            if !unsafe { PyErr_Occurred() }.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                unsafe { PyErr_SetNone(PyExc_StopIteration) };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

static next_doc: __SlateAlign16<[i8; 168]> = __SlateAlign16([
    110i8, 101i8, 120i8, 116i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 91i8,
    44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 93i8, 41i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 101i8, 120i8, 116i8,
    32i8, 105i8, 116i8, 101i8, 109i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 46i8, 32i8, 32i8, 73i8, 102i8,
    32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 103i8, 105i8,
    118i8, 101i8, 110i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 10i8, 105i8, 116i8,
    101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8, 104i8, 97i8,
    117i8, 115i8, 116i8, 101i8, 100i8, 44i8, 32i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 115i8, 116i8, 101i8, 97i8,
    100i8, 32i8, 111i8, 102i8, 32i8, 114i8, 97i8, 105i8, 115i8, 105i8, 110i8, 103i8, 32i8, 83i8,
    116i8, 111i8, 112i8, 73i8, 116i8, 101i8, 114i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

// [clinic input]
// setattr as builtin_setattr
//
//     obj: object
//     name: object
//     value: object
//     /
//
// Sets the named attribute on the given object to the specified value.
//
// setattr(x, 'y', v) is equivalent to ``x.y = v``
// [clinic start generated code]
/// [clinic end generated code: output=dc2ce1d1add9acb4 input=5e26417f2e8598d4]
fn builtin_setattr_impl(
    module: *mut _object,
    obj: *mut _object,
    name: *mut _object,
    value: *mut _object,
) -> *mut _object {
    if (unsafe { PyObject_SetAttr(obj, name, value) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// delattr as builtin_delattr
//
//     obj: object
//     name: object
//     /
//
// Deletes the named attribute from the given object.
//
// delattr(x, 'y') is equivalent to ``del x.y``
// [clinic start generated code]
/// [clinic end generated code: output=85134bc58dff79fa input=164865623abe7216]
fn builtin_delattr_impl(
    module: *mut _object,
    obj: *mut _object,
    name: *mut _object,
) -> *mut _object {
    if (unsafe { PyObject_DelAttr(obj, name) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// hash as builtin_hash
//
//     obj: object
//     /
//
// Return the integer hash value for the given object.
//
// Two objects that compare equal must also have the same hash value, but
// the reverse is not necessarily true.  Hash values may differ between
// Python processes.  Not all objects are hashable; calling hash() on an
// unhashable object raises TypeError.
// [clinic start generated code]
/// [clinic end generated code: output=237668e9d7688db7 input=70a242ff65f6717c]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_hash")]
extern "C-unwind" fn builtin_hash(module: *mut _object, obj: *mut _object) -> *mut _object {
    let x: i64 = unsafe { PyObject_Hash(obj) };
    if x == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t(x) }
}

// [clinic input]
// hex as builtin_hex
//
//     number: object
//     /
//
// Return the hexadecimal representation of an integer.
//
//    >>> hex(12648430)
//    '0xc0ffee'
// [clinic start generated code]
/// [clinic end generated code: output=e46b612169099408 input=e645aff5fc7d540e]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_hex")]
extern "C-unwind" fn builtin_hex(module: *mut _object, number: *mut _object) -> *mut _object {
    unsafe { PyNumber_ToBase(number, 16i32) }
}

/// AC: cannot convert yet, as needs PEP 457 group support in inspect
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_iter")]
extern "C-unwind" fn builtin_iter(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    if !_PyArg_CheckPositional!(c"iter".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
        return core::ptr::null_mut::<_object>();
    }
    let v: *mut _object = unsafe { *args.wrapping_offset(0 as isize) };
    if nargs == 1 {
        return unsafe { PyObject_GetIter(v) };
    }
    if (unsafe { PyCallable_Check(v) }) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"iter(v, w): v must be callable".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let sentinel: *mut _object = unsafe { *args.wrapping_offset(1 as isize) };
    unsafe { PyCallIter_New(v, sentinel) }
}

static iter_doc: __SlateAlign16<[i8; 252]> = __SlateAlign16([
    105i8, 116i8, 101i8, 114i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 41i8,
    32i8, 45i8, 62i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 10i8, 105i8,
    116i8, 101i8, 114i8, 40i8, 99i8, 97i8, 108i8, 108i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8,
    115i8, 101i8, 110i8, 116i8, 105i8, 110i8, 101i8, 108i8, 41i8, 32i8, 45i8, 62i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 10i8, 10i8, 71i8, 101i8, 116i8, 32i8, 97i8,
    110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 102i8, 114i8, 111i8,
    109i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 32i8, 32i8, 73i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 102i8, 111i8,
    114i8, 109i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 32i8, 109i8, 117i8, 115i8, 116i8, 10i8, 115i8, 117i8, 112i8, 112i8, 108i8, 121i8,
    32i8, 105i8, 116i8, 115i8, 32i8, 111i8, 119i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    116i8, 111i8, 114i8, 44i8, 32i8, 111i8, 114i8, 32i8, 98i8, 101i8, 32i8, 97i8, 32i8, 115i8,
    101i8, 113i8, 117i8, 101i8, 110i8, 99i8, 101i8, 46i8, 10i8, 73i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 32i8, 102i8, 111i8, 114i8, 109i8, 44i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 117i8, 110i8, 116i8, 105i8,
    108i8, 32i8, 105i8, 116i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 116i8,
    104i8, 101i8, 10i8, 115i8, 101i8, 110i8, 116i8, 105i8, 110i8, 101i8, 108i8, 46i8, 0i8,
]);

// [clinic input]
// aiter as builtin_aiter
//
//     async_iterable: object
//     /
//
// Return an AsyncIterator for an AsyncIterable object.
// [clinic start generated code]
/// [clinic end generated code: output=1bae108d86f7960e input=473993d0cacc7d23]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_aiter")]
extern "C-unwind" fn builtin_aiter(
    module: *mut _object,
    async_iterable: *mut _object,
) -> *mut _object {
    unsafe { PyObject_GetAIter(async_iterable) }
}

// [clinic input]
// anext as builtin_anext
//
//     aiterator: object
//     default: object = NULL
//     /
//
// Return the next item from the async iterator.
//
// If default is given and the async iterator is exhausted,
// it is returned instead of raising StopAsyncIteration.
// [clinic start generated code]
/// [clinic end generated code: output=f02c060c163a81fa input=2900e4a370d39550]
fn builtin_anext_impl(
    module: *mut _object,
    aiterator: *mut _object,
    default_value: *mut _object,
) -> *mut _object {
    let t: *mut _typeobject = Py_TYPE!(aiterator);
    if unsafe { (*t).tp_as_async }.is_null() || (unsafe { (*(*t).tp_as_async).am_anext }) == None {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"'%.200s' object is not an async iterator".as_ptr(),
                (*t).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let awaitable: *mut _object =
        unsafe { unsafe { (*(*t).tp_as_async).am_anext }.unwrap()(aiterator) };
    if awaitable.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if default_value.is_null() {
        return awaitable;
    }
    let new_awaitable: *mut _object = unsafe { PyAnextAwaitable_New(awaitable, default_value) };
    Py_DECREF!(awaitable);
    new_awaitable
}

// [clinic input]
// len as builtin_len
//
//     obj: object
//     /
//
// Return the number of items in a container.
// [clinic start generated code]
/// [clinic end generated code: output=fa7a270d314dfb6c input=bc55598da9e9c9b5]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_len")]
extern "C-unwind" fn builtin_len(module: *mut _object, obj: *mut _object) -> *mut _object {
    let res: i64 = unsafe { PyObject_Size(obj) };
    if res < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t(res) }
}

// [clinic input]
// locals as builtin_locals
//
// Return a dictionary containing the current scope's local variables.
//
// NOTE: Whether or not updates to this dictionary will affect name
// lookups in the local scope and vice-versa is *implementation
// dependent* and not covered by any backwards compatibility
// guarantees.
// [clinic start generated code]
/// [clinic end generated code: output=b46c94015ce11448 input=989cc75c22167c42]
fn builtin_locals_impl(module: *mut _object) -> *mut _object {
    let mut locals: *mut _object = unsafe { core::mem::zeroed() };
    if !unsafe { _PyEval_GetFrame() }.is_null() {
        locals = unsafe { _PyEval_GetFrameLocals() };
        return locals;
    }
    let tstate: *mut _ts = _PyThreadState_GET();
    locals = unsafe { _PyEval_GetGlobalsFromRunningMain(tstate) };
    if locals.is_null() {
        if !_PyErr_Occurred(tstate).is_null() {
            return core::ptr::null_mut::<_object>();
        }
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(locals)
}

fn min_max(
    mut args: *const *mut _object,
    mut nargs: i64,
    kwnames: *mut _object,
    op: i32,
) -> *mut _object {
    let mut cmp: i32 = unsafe { core::mem::zeroed() };
    // False iff nargs == 1
    let mut positional: i32 = unsafe { core::mem::zeroed() };
    let mut __v6: bool = unsafe { core::mem::zeroed() };
    let mut _parser: *mut _PyArg_Parser = unsafe { core::mem::zeroed() };
    let mut name: *const i8 = unsafe { core::mem::zeroed() };
    let mut defaultval: *mut _object = unsafe { core::mem::zeroed() };
    let mut keyfunc: *mut _object = unsafe { core::mem::zeroed() };
    let mut maxval: *mut _object = unsafe { core::mem::zeroed() };
    let mut maxitem: *mut _object = unsafe { core::mem::zeroed() };
    let mut val: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        it = core::ptr::null_mut::<_object>();
        keyfunc = core::ptr::null_mut::<_object>();
        defaultval = core::ptr::null_mut::<_object>();
        name = (if op == 0 {
            c"min".as_ptr() as *mut i8
        } else {
            c"max".as_ptr() as *mut i8
        }) as *const i8;
        _parser = if op == 0 {
            core::ptr::addr_of_mut!(_parser_min)
        } else {
            core::ptr::addr_of_mut!(_parser_max)
        };
        if nargs == 0 {
            PyErr_Format(
                PyExc_TypeError,
                c"%s expected at least 1 argument, got 0".as_ptr(),
                name,
            );
            return core::ptr::null_mut::<_object>();
        } else {
            if !kwnames.is_null() {
                __v6 = _PyArg_ParseStackAndKeywords(
                    args.wrapping_offset(nargs as isize),
                    0i64,
                    kwnames,
                    _parser,
                    core::ptr::addr_of_mut!(keyfunc),
                    core::ptr::addr_of_mut!(defaultval),
                ) == 0;
            } else {
                __v6 = false;
            }
            if __v6 {
                return core::ptr::null_mut::<_object>();
            } else {
                positional = (nargs > 1) as i32;
                if positional != 0 && !defaultval.is_null() {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"Cannot specify a default for %s() with multiple positional arguments"
                            .as_ptr(),
                        name,
                    );
                    return core::ptr::null_mut::<_object>();
                } else {
                    if positional == 0 {
                        it = PyObject_GetIter(*args.wrapping_offset(0 as isize));
                        if it.is_null() {
                            return core::ptr::null_mut::<_object>();
                        }
                    }
                    if keyfunc == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                        keyfunc = core::ptr::null_mut::<_object>();
                    }
                    maxitem = core::ptr::null_mut::<_object>(); // the result
                    maxval = core::ptr::null_mut::<_object>(); // the value associated with the result
                    '__join_8: {
                        '__join_0: {
                            '__join_1: {
                                '__join_2: {
                                    loop {
                                        if true {
                                            if it.is_null() {
                                                let __v4: i64 = nargs;
                                                nargs = __v4.wrapping_sub(1);
                                                if __v4 <= 0 {
                                                    break '__join_8;
                                                } else {
                                                    let __v2: *const *mut _object = args;
                                                    args = __v2.wrapping_offset(1 as isize);
                                                    item = *__v2;
                                                    Py_INCREF!(item);
                                                }
                                            } else {
                                                item = PyIter_Next(it);
                                                if item.is_null() {
                                                    break;
                                                }
                                            }
                                            // get the value from the key function
                                            if !keyfunc.is_null() {
                                                val = PyObject_CallOneArg(keyfunc, item);
                                                if val.is_null() {
                                                    break '__join_1;
                                                }
                                            } else {
                                                val = Py_NewRef!(item);
                                            }
                                            // no key function; the value is the item
                                            // maximum value and item are unset; set them
                                            if maxval.is_null() {
                                                maxitem = item;
                                                maxval = val;
                                            } else {
                                                cmp = PyObject_RichCompareBool(val, maxval, op);
                                                if cmp < 0 {
                                                    break '__join_2;
                                                } else {
                                                    if cmp > 0 {
                                                        Py_DECREF!(maxval);
                                                        Py_DECREF!(maxitem);
                                                        maxval = val;
                                                        maxitem = item;
                                                    } else {
                                                        Py_DECREF!(item);
                                                        Py_DECREF!(val);
                                                    }
                                                }
                                            }
                                        // maximum value and item are set; update them as necessary
                                        } else {
                                            break '__join_8;
                                        }
                                    }
                                    if !PyErr_Occurred().is_null() {
                                        break '__join_0;
                                    } else {
                                        break '__join_8;
                                    }
                                }
                                Py_DECREF!(val);
                            }
                            Py_DECREF!(item);
                        }
                        Py_XDECREF!(maxval);
                        Py_XDECREF!(maxitem);
                        Py_XDECREF!(it);
                        return core::ptr::null_mut::<_object>();
                    }
                    if maxval.is_null() {
                        if !defaultval.is_null() {
                            maxitem = Py_NewRef!(defaultval);
                        } else {
                            PyErr_Format(
                                PyExc_ValueError,
                                c"%s() iterable argument is empty".as_ptr(),
                                name,
                            );
                        }
                    } else {
                        Py_DECREF!(maxval);
                    }
                    Py_XDECREF!(it);
                    return maxitem;
                }
            }
        }
    }
}

static keywords: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"key\0".as_ptr() as *mut i8) as *const i8,
    (b"default\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser_min: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.format = (b"|$OO:min\0".as_ptr() as *mut i8) as *const i8;
    __t0.keywords = unsafe { core::ptr::addr_of!(keywords.0) as *const *const i8 };
    __t0.fname = core::ptr::null::<i8>();
    __t0
};

static mut _parser_max: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.format = (b"|$OO:max\0".as_ptr() as *mut i8) as *const i8;
    __t0.keywords = unsafe { core::ptr::addr_of!(keywords.0) as *const *const i8 };
    __t0.fname = core::ptr::null::<i8>();
    __t0
};

/// AC: cannot convert yet, waiting for *args support
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_min")]
extern "C-unwind" fn builtin_min(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    min_max(args, nargs, kwnames, 0i32)
}

static min_doc: __SlateAlign16<[i8; 326]> = __SlateAlign16([
    109i8, 105i8, 110i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8,
    42i8, 91i8, 44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 61i8, 111i8, 98i8,
    106i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 102i8, 117i8, 110i8, 99i8, 93i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 10i8, 109i8, 105i8, 110i8, 40i8, 97i8,
    114i8, 103i8, 49i8, 44i8, 32i8, 97i8, 114i8, 103i8, 50i8, 44i8, 32i8, 42i8, 97i8, 114i8, 103i8,
    115i8, 44i8, 32i8, 42i8, 91i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 102i8, 117i8, 110i8,
    99i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 10i8, 10i8, 87i8,
    105i8, 116i8, 104i8, 32i8, 97i8, 32i8, 115i8, 105i8, 110i8, 103i8, 108i8, 101i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 116i8, 115i8,
    32i8, 115i8, 109i8, 97i8, 108i8, 108i8, 101i8, 115i8, 116i8, 32i8, 105i8, 116i8, 101i8, 109i8,
    46i8, 32i8, 84i8, 104i8, 101i8, 10i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8,
    107i8, 101i8, 121i8, 119i8, 111i8, 114i8, 100i8, 45i8, 111i8, 110i8, 108i8, 121i8, 32i8, 97i8,
    114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 101i8, 115i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    116i8, 111i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 102i8, 10i8, 116i8,
    104i8, 101i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 101i8, 100i8, 32i8, 105i8, 116i8,
    101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8,
    121i8, 46i8, 10i8, 87i8, 105i8, 116i8, 104i8, 32i8, 116i8, 119i8, 111i8, 32i8, 111i8, 114i8,
    32i8, 109i8, 111i8, 114i8, 101i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8,
    97i8, 108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 44i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 109i8, 97i8,
    108i8, 108i8, 101i8, 115i8, 116i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8,
    46i8, 0i8,
]);

/// AC: cannot convert yet, waiting for *args support
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_max")]
extern "C-unwind" fn builtin_max(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    min_max(args, nargs, kwnames, 4i32)
}

static max_doc: __SlateAlign16<[i8; 324]> = __SlateAlign16([
    109i8, 97i8, 120i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8,
    42i8, 91i8, 44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 61i8, 111i8, 98i8,
    106i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 102i8, 117i8, 110i8, 99i8, 93i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 10i8, 109i8, 97i8, 120i8, 40i8, 97i8,
    114i8, 103i8, 49i8, 44i8, 32i8, 97i8, 114i8, 103i8, 50i8, 44i8, 32i8, 42i8, 97i8, 114i8, 103i8,
    115i8, 44i8, 32i8, 42i8, 91i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 102i8, 117i8, 110i8,
    99i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 10i8, 10i8, 87i8,
    105i8, 116i8, 104i8, 32i8, 97i8, 32i8, 115i8, 105i8, 110i8, 103i8, 108i8, 101i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 116i8, 115i8,
    32i8, 98i8, 105i8, 103i8, 103i8, 101i8, 115i8, 116i8, 32i8, 105i8, 116i8, 101i8, 109i8, 46i8,
    32i8, 84i8, 104i8, 101i8, 10i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 107i8,
    101i8, 121i8, 119i8, 111i8, 114i8, 100i8, 45i8, 111i8, 110i8, 108i8, 121i8, 32i8, 97i8, 114i8,
    103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8,
    101i8, 115i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 116i8,
    111i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 102i8, 10i8, 116i8, 104i8,
    101i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 101i8, 100i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8,
    46i8, 10i8, 87i8, 105i8, 116i8, 104i8, 32i8, 116i8, 119i8, 111i8, 32i8, 111i8, 114i8, 32i8,
    109i8, 111i8, 114i8, 101i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 44i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 97i8, 114i8, 103i8,
    101i8, 115i8, 116i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 0i8,
]);

// [clinic input]
// oct as builtin_oct
//
//     number: object
//     /
//
// Return the octal representation of an integer.
//
//    >>> oct(342391)
//    '0o1234567'
// [clinic start generated code]
/// [clinic end generated code: output=40a34656b6875352 input=ad6b274af4016c72]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_oct")]
extern "C-unwind" fn builtin_oct(module: *mut _object, number: *mut _object) -> *mut _object {
    unsafe { PyNumber_ToBase(number, 8i32) }
}

// [clinic input]
// ord as builtin_ord
//
//     character as c: object
//     /
//
// Return the ordinal value of a character.
//
// If the argument is a one-character string, return the Unicode code
// point of that character.
//
// If the argument is a bytes or bytearray object of length 1, return its
// single byte value.
// [clinic start generated code]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_ord")]
extern "C-unwind" fn builtin_ord(module: *mut _object, c: *mut _object) -> *mut _object {
    let mut ord: i64 = 0;
    let mut size: i64 = 0;
    if PyBytes_Check!(c) != 0 {
        size = PyBytes_GET_SIZE!(c);
        if size == 1 {
            ord = ((unsafe { *PyBytes_AS_STRING!(c) }) as u8) as i64;
            return unsafe { PyLong_FromLong(ord) };
        }
    } else {
        if PyUnicode_Check!(c) != 0 {
            size = PyUnicode_GET_LENGTH!(c);
            if size == 1 {
                ord = PyUnicode_READ_CHAR__2!(c, 0i32) as i64;
                return unsafe { PyLong_FromLong(ord) };
            }
        } else {
            if PyByteArray_Check!(c) != 0 {
                // XXX Hopefully this is temporary
                size = PyByteArray_GET_SIZE!(c);
                if size == 1 {
                    ord = ((unsafe { *PyByteArray_AS_STRING__2!(c) }) as u8) as i64;
                    return unsafe { PyLong_FromLong(ord) };
                }
            } else {
                unsafe {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"ord() expected string of length 1, but %.200s found".as_ptr(),
                        (*Py_TYPE!(c)).tp_name,
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"ord() expected a character, but string of length %zd found".as_ptr(),
            size,
        )
    };
    core::ptr::null_mut::<_object>()
}

// [clinic input]
// pow as builtin_pow
//
//     base: object
//     exp: object
//     mod: object = None
//
// Equivalent to base**exp with 2 arguments or base**exp % mod with 3 arguments
//
// Some types, such as ints, are able to use a more efficient algorithm
// when invoked using the three argument form.
// [clinic start generated code]
// [clinic end generated code: output=4fa5e87a323bae71 input=98d38480432e1177]
/// [clinic end generated code: output=3ca1538221bbf15f input=6133ded72c7db82e]
fn builtin_pow_impl(
    module: *mut _object,
    base: *mut _object,
    exp: *mut _object,
    r#mod: *mut _object,
) -> *mut _object {
    unsafe { PyNumber_Power(base, exp, r#mod) }
}

// [clinic input]
// print as builtin_print
//
//     *args: array
//     sep: object(c_default="Py_None") = ' '
//         string inserted between values, default a space.
//     end: object(c_default="Py_None") = '\n'
//         string appended after the last value, default a newline.
//     file: object = None
//         a file-like object (stream); defaults to the current sys.stdout.
//     flush: bool = False
//         whether to forcibly flush the stream.
//
// Prints the values to a stream, or to sys.stdout by default.
//
// [clinic start generated code]
/// [clinic end generated code: output=3cb7e5b66f1a8547 input=66ea4de1605a2437]
fn builtin_print_impl(
    module: *mut _object,
    args: *const *mut _object,
    args_length: i64,
    mut sep: *mut _object,
    mut end: *mut _object,
    mut file: *mut _object,
    flush: i32,
) -> *mut _object {
    let mut i: i32 = 0;
    let mut err: i32 = 0;
    if file == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        file = unsafe {
            _PySys_GetRequiredAttr(core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_stdout
                    ._ascii
                    .ob_base
            ))
        };
        if file.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        // sys.stdout may be None when FILE* stdout isn't connected
        if file == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            Py_DECREF!(file);
            return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
        }
    } else {
        Py_INCREF!(file);
    }
    if sep == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        sep = core::ptr::null_mut::<_object>();
    } else {
        let __v0: bool = !sep.is_null() && PyUnicode_Check!(sep) == 0;
        if __v0 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"sep must be None or a string, not %.200s".as_ptr(),
                    (*Py_TYPE!(sep)).tp_name,
                )
            };
            Py_DECREF!(file);
            return core::ptr::null_mut::<_object>();
        }
    }
    if end == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        end = core::ptr::null_mut::<_object>();
    } else {
        let __v1: bool = !end.is_null() && PyUnicode_Check!(end) == 0;
        if __v1 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"end must be None or a string, not %.200s".as_ptr(),
                    (*Py_TYPE!(end)).tp_name,
                )
            };
            Py_DECREF!(file);
            return core::ptr::null_mut::<_object>();
        }
    }
    while (i as i64) < args_length {
        if i > 0 {
            if sep.is_null() {
                err = unsafe { PyFile_WriteString(c" ".as_ptr(), file) };
            } else {
                err = unsafe { PyFile_WriteObject(sep, file, 1i32) };
            }
            if err != 0 {
                Py_DECREF!(file);
                return core::ptr::null_mut::<_object>();
            }
        }
        err = unsafe { PyFile_WriteObject(*args.wrapping_offset(i as isize), file, 1i32) };
        if err != 0 {
            Py_DECREF!(file);
            return core::ptr::null_mut::<_object>();
        }
        i = i.wrapping_add(1);
    }
    if end.is_null() {
        err = unsafe { PyFile_WriteString(c"\n".as_ptr(), file) };
    } else {
        err = unsafe { PyFile_WriteObject(end, file, 1i32) };
    }
    if err != 0 {
        Py_DECREF!(file);
        return core::ptr::null_mut::<_object>();
    }
    if flush != 0 {
        if (unsafe { _PyFile_Flush(file) }) < 0 {
            Py_DECREF!(file);
            return core::ptr::null_mut::<_object>();
        }
    }
    Py_DECREF!(file);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// input as builtin_input
//
//     prompt: object(c_default="NULL") = ""
//     /
//
// Read a string from standard input.  The trailing newline is stripped.
//
// The prompt string, if given, is printed to standard output without a
// trailing newline before reading input.
//
// If the user hits EOF (*nix: Ctrl-D, Windows: Ctrl-Z+Return), raise
// EOFError.
// On *nix systems, readline is used if available.
// [clinic start generated code]
/// [clinic end generated code: output=83db5a191e7a0d60 input=ebb939c954639427]
fn builtin_input_impl(module: *mut _object, prompt: *mut _object) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut stringpo: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdout_errors_str: *const i8 = unsafe { core::mem::zeroed() };
    // We have a prompt, encode it as stdout would
    let mut stdout_encoding_str: *const i8 = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut len: u64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdin_errors_str: *const i8 = unsafe { core::mem::zeroed() };
    let mut stdin_encoding_str: *const i8 = unsafe { core::mem::zeroed() };
    let mut stdout_errors: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdout_encoding: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdin_errors: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdin_encoding: *mut _object = unsafe { core::mem::zeroed() };
    let mut s: *mut i8 = unsafe { core::mem::zeroed() };
    let mut promptstr: *const i8 = unsafe { core::mem::zeroed() };
    let mut po: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v20: bool = unsafe { core::mem::zeroed() };
    let mut __v21: bool = unsafe { core::mem::zeroed() };
    let mut __v22: bool = unsafe { core::mem::zeroed() };
    let mut __v23: bool = unsafe { core::mem::zeroed() };
    let mut tty: i32 = unsafe { core::mem::zeroed() };
    let mut fd: i64 = unsafe { core::mem::zeroed() };
    let mut tmp: *mut _object = unsafe { core::mem::zeroed() };
    let mut ferr: *mut _object = unsafe { core::mem::zeroed() };
    let mut fout: *mut _object = unsafe { core::mem::zeroed() };
    let mut fin: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            fin = core::ptr::null_mut::<_object>();
            fout = core::ptr::null_mut::<_object>();
            ferr = core::ptr::null_mut::<_object>();
            // Check that stdin/out/err are intact
            fin = _PySys_GetRequiredAttr(core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_stdin
                    ._ascii
                    .ob_base
            ));
            if !fin.is_null() {
                if fin == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    PyErr_SetString(PyExc_RuntimeError, c"lost sys.stdin".as_ptr());
                } else {
                    fout = _PySys_GetRequiredAttr(core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_stdout
                            ._ascii
                            .ob_base
                    ));
                    if !fout.is_null() {
                        if fout == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            PyErr_SetString(PyExc_RuntimeError, c"lost sys.stdout".as_ptr());
                        } else {
                            ferr = _PySys_GetRequiredAttr(core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_stderr
                                    ._ascii
                                    .ob_base
                            ));
                            if !ferr.is_null() {
                                if ferr == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                    PyErr_SetString(
                                        PyExc_RuntimeError,
                                        c"lost sys.stderr".as_ptr(),
                                    );
                                } else {
                                    if PySys_Audit(
                                        c"builtins.input".as_ptr(),
                                        c"O".as_ptr(),
                                        if !prompt.is_null() {
                                            prompt
                                        } else {
                                            core::ptr::addr_of_mut!(_Py_NoneStruct)
                                        },
                                    ) >= 0
                                    {
                                        // First of all, flush stderr
                                        if _PyFile_Flush(ferr) < 0 {
                                            PyErr_Clear();
                                        }
                                        // We should only use (GNU) readline if Python's sys.stdin and
                                        // sys.stdout are the same as C's stdin and stdout, because we
                                        // need to pass it those.
                                        tmp = PyObject_CallMethodNoArgs(
                                            fin,
                                            core::ptr::addr_of_mut!(
                                                _PyRuntime
                                                    .static_objects
                                                    .singletons
                                                    .strings
                                                    .identifiers
                                                    ._py_fileno
                                                    ._ascii
                                                    .ob_base
                                            ),
                                        );
                                        if tmp.is_null() {
                                            PyErr_Clear();
                                            tty = 0;
                                        } else {
                                            fd = PyLong_AsLong(tmp);
                                            Py_DECREF!(tmp);
                                            if fd < 0 {
                                                __v23 = !PyErr_Occurred().is_null();
                                            } else {
                                                __v23 = false;
                                            }
                                            if __v23 {
                                                break '__join_0;
                                            } else {
                                                if fd == (fileno(slate_support::stdin()) as i64) {
                                                    __v22 =
                                                        (isatty((fd as i32) as i32) as i32) != 0;
                                                } else {
                                                    __v22 = false;
                                                }
                                                tty = __v22 as i32;
                                            }
                                        }
                                        if tty != 0 {
                                            tmp = PyObject_CallMethodNoArgs(
                                                fout,
                                                core::ptr::addr_of_mut!(
                                                    _PyRuntime
                                                        .static_objects
                                                        .singletons
                                                        .strings
                                                        .identifiers
                                                        ._py_fileno
                                                        ._ascii
                                                        .ob_base
                                                ),
                                            );
                                            if tmp.is_null() {
                                                PyErr_Clear();
                                                tty = 0;
                                            } else {
                                                fd = PyLong_AsLong(tmp);
                                                Py_DECREF!(tmp);
                                                if fd < 0 {
                                                    __v21 = !PyErr_Occurred().is_null();
                                                } else {
                                                    __v21 = false;
                                                }
                                                if __v21 {
                                                    break '__join_0;
                                                } else {
                                                    if fd
                                                        == (fileno(slate_support::stdout()) as i64)
                                                    {
                                                        __v20 = (isatty((fd as i32) as i32) as i32)
                                                            != 0;
                                                    } else {
                                                        __v20 = false;
                                                    }
                                                    tty = __v20 as i32;
                                                }
                                            }
                                        }
                                        // If we're interactive, use (GNU) readline
                                        if tty != 0 {
                                            '__join_7: {
                                                po = core::ptr::null_mut::<_object>();
                                                s = core::ptr::null_mut::<i8>();
                                                stdin_encoding = core::ptr::null_mut::<_object>();
                                                stdin_errors = core::ptr::null_mut::<_object>();
                                                stdout_encoding = core::ptr::null_mut::<_object>();
                                                stdout_errors = core::ptr::null_mut::<_object>();
                                                // stdin is a text stream, so it must have an encoding.
                                                stdin_encoding = PyObject_GetAttr(
                                                    fin,
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
                                                if stdin_encoding.is_null() {
                                                    tty = 0;
                                                } else {
                                                    stdin_errors = PyObject_GetAttr(
                                                        fin,
                                                        core::ptr::addr_of_mut!(
                                                            _PyRuntime
                                                                .static_objects
                                                                .singletons
                                                                .strings
                                                                .identifiers
                                                                ._py_errors
                                                                ._ascii
                                                                .ob_base
                                                        ),
                                                    );
                                                    if stdin_errors.is_null() {
                                                        tty = 0;
                                                    } else {
                                                        if PyUnicode_Check!(stdin_encoding) == 0 {
                                                            __v8 = true;
                                                        } else {
                                                            __v8 =
                                                                PyUnicode_Check!(stdin_errors) == 0;
                                                        }
                                                        if __v8 {
                                                            tty = 0;
                                                        } else {
                                                            stdin_encoding_str =
                                                                PyUnicode_AsUTF8(stdin_encoding);
                                                            if !stdin_encoding_str.is_null() {
                                                                stdin_errors_str =
                                                                    PyUnicode_AsUTF8(stdin_errors);
                                                                if !stdin_errors_str.is_null() {
                                                                    if _PyFile_Flush(fout) < 0 {
                                                                        PyErr_Clear();
                                                                    }
                                                                    if !prompt.is_null() {
                                                                        stdout_encoding = PyObject_GetAttr(fout, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_encoding._ascii.ob_base));
                                                                        if stdout_encoding.is_null()
                                                                        {
                                                                            tty = 0;
                                                                            break '__join_7;
                                                                        } else {
                                                                            stdout_errors = PyObject_GetAttr(fout, core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.identifiers._py_errors._ascii.ob_base));
                                                                            if stdout_errors
                                                                                .is_null()
                                                                            {
                                                                                tty = 0;
                                                                                break '__join_7;
                                                                            } else {
                                                                                if PyUnicode_Check!(
                                                                                    stdout_encoding
                                                                                ) == 0
                                                                                {
                                                                                    __v4 = true;
                                                                                } else {
                                                                                    __v4 = PyUnicode_Check!(stdout_errors) == 0;
                                                                                }
                                                                                if __v4 {
                                                                                    tty = 0;
                                                                                    break '__join_7;
                                                                                } else {
                                                                                    stdout_encoding_str = PyUnicode_AsUTF8(stdout_encoding);
                                                                                    if stdout_encoding_str.is_null() {
break '__join_7;
} else {
stdout_errors_str = PyUnicode_AsUTF8(stdout_errors);
if stdout_errors_str.is_null() {
break '__join_7;
} else {
stringpo = PyObject_Str(prompt);
if stringpo.is_null() {
break '__join_7;
} else {
po = PyUnicode_AsEncodedString(stringpo, stdout_encoding_str, stdout_errors_str);
Py_CLEAR__2!(stdout_encoding);
Py_CLEAR__2!(stdout_errors);
Py_CLEAR__2!(stringpo);
if po.is_null() {
break '__join_7;
} else {
promptstr = PyBytes_AS_STRING!(po) as *const i8;
if ((strlen(promptstr as *const core::ffi::c_char) as u64) as i64) != PyBytes_GET_SIZE!(po) {
PyErr_SetString(PyExc_ValueError, c"input: prompt string cannot contain null characters".as_ptr());
break '__join_7;
}
}
}
}
}
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        po = core::ptr::null_mut::<
                                                                            _object,
                                                                        >(
                                                                        );
                                                                        promptstr = c"".as_ptr();
                                                                    }
                                                                    s = PyOS_Readline(
                                                                        slate_support::stdin(),
                                                                        slate_support::stdout(),
                                                                        promptstr,
                                                                    );
                                                                    if s.is_null() {
                                                                        PyErr_CheckSignals();
                                                                        if PyErr_Occurred()
                                                                            .is_null()
                                                                        {
                                                                            PyErr_SetNone(PyExc_KeyboardInterrupt);
                                                                        }
                                                                    } else {
                                                                        len = strlen((s as *const i8) as *const core::ffi::c_char) as u64;
                                                                        if len == 0 {
                                                                            PyErr_SetNone(
                                                                                PyExc_EOFError,
                                                                            );
                                                                            result =
                                                                                core::ptr::null_mut::<
                                                                                    _object,
                                                                                >(
                                                                                );
                                                                        } else {
                                                                            if len > 9223372036854775807 {
PyErr_SetString(PyExc_OverflowError, c"input: input too long".as_ptr());
result = core::ptr::null_mut::<_object>();
} else {
len = len.wrapping_sub(1); // strip trailing '\n'
if len != 0 && (*s.wrapping_offset(len.wrapping_sub(1) as isize) as i32) == 13 {
len = len.wrapping_sub(1);
}
// strip trailing '\r'
result = PyUnicode_Decode(s as *const i8, len as i64, stdin_encoding_str, stdin_errors_str);
}
                                                                        }
                                                                        Py_DECREF!(stdin_encoding);
                                                                        Py_DECREF!(stdin_errors);
                                                                        Py_XDECREF!(po);
                                                                        PyMem_Free(s as *mut ());
                                                                        if !result.is_null() {
                                                                            if PySys_Audit(c"builtins.input/result".as_ptr(), c"O".as_ptr(), result) < 0 {
break '__join_0;
}
                                                                        }
                                                                        Py_DECREF!(fin);
                                                                        Py_DECREF!(fout);
                                                                        Py_DECREF!(ferr);
                                                                        return result;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Py_XDECREF!(stdin_encoding);
                                            Py_XDECREF!(stdout_encoding);
                                            Py_XDECREF!(stdin_errors);
                                            Py_XDECREF!(stdout_errors);
                                            Py_XDECREF!(po);
                                            if tty != 0 {
                                                break '__join_0;
                                            } else {
                                                PyErr_Clear();
                                            }
                                        }
                                        // Fallback if we're not interactive
                                        if !prompt.is_null() {
                                            if PyFile_WriteObject(prompt, fout, 1i32) != 0 {
                                                break '__join_0;
                                            }
                                        }
                                        if _PyFile_Flush(fout) < 0 {
                                            PyErr_Clear();
                                        }
                                        tmp = PyFile_GetLine(fin, 1i32.wrapping_neg());
                                        Py_DECREF!(fin);
                                        Py_DECREF!(fout);
                                        Py_DECREF!(ferr);
                                        return tmp;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(fin);
        Py_XDECREF!(fout);
        Py_XDECREF!(ferr);
        return core::ptr::null_mut::<_object>();
    }
}

// [clinic input]
// repr as builtin_repr
//
//     obj: object
//     /
//
// Return the canonical string representation of the object.
//
// For many object types, including most builtins, eval(repr(obj)) == obj.
// [clinic start generated code]
/// [clinic end generated code: output=7ed3778c44fd0194 input=1c9e6d66d3e3be04]
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_repr")]
extern "C-unwind" fn builtin_repr(module: *mut _object, obj: *mut _object) -> *mut _object {
    unsafe { PyObject_Repr(obj) }
}

// [clinic input]
// round as builtin_round
//
//     number: object
//     ndigits: object = None
//
// Round a number to a given precision in decimal digits.
//
// The return value is an integer if ndigits is omitted or None.
// Otherwise the return value has the same type as the number.  ndigits
// may be negative.
// [clinic start generated code]
/// [clinic end generated code: output=ff0d9dd176c02ede input=bdcb7c67bf4a4320]
fn builtin_round_impl(
    module: *mut _object,
    number: *mut _object,
    ndigits: *mut _object,
) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if ndigits == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        result = unsafe {
            _PyObject_MaybeCallSpecialNoArgs(
                number,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py___round__
                        ._ascii
                        .ob_base
                ),
            )
        };
    } else {
        result = unsafe {
            _PyObject_MaybeCallSpecialOneArg(
                number,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py___round__
                        ._ascii
                        .ob_base
                ),
                ndigits,
            )
        };
    }
    let __v0: bool = result.is_null() && unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"type %.100s doesn't define __round__ method".as_ptr(),
                (*Py_TYPE!(number)).tp_name,
            )
        };
    }
    result
}

// AC: we need to keep the kwds dict intact to easily call into the
// list.sort method, which isn't currently supported in AC. So we just use
// the initially generated signature with a custom implementation.
//
// [disabled clinic input]
// sorted as builtin_sorted
//
//     iterable as seq: object
//     key as keyfunc: object = None
//     reverse: object = False
//
// Return a new list containing all items from the iterable in ascending order.
//
// A custom key function can be supplied to customize the sort order, and
// the reverse flag can be set to request the result in descending order.
// [end disabled clinic input]
static builtin_sorted__doc__: __SlateAlign16<[i8; 281]> = __SlateAlign16([
    115i8, 111i8, 114i8, 116i8, 101i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    42i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 114i8,
    101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 110i8,
    101i8, 119i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8,
    110i8, 105i8, 110i8, 103i8, 32i8, 97i8, 108i8, 108i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 110i8, 32i8, 97i8, 115i8, 99i8, 101i8, 110i8, 100i8,
    105i8, 110i8, 103i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 46i8, 10i8, 10i8, 65i8, 32i8,
    99i8, 117i8, 115i8, 116i8, 111i8, 109i8, 32i8, 107i8, 101i8, 121i8, 32i8, 102i8, 117i8, 110i8,
    99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 115i8,
    117i8, 112i8, 112i8, 108i8, 105i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 99i8, 117i8, 115i8,
    116i8, 111i8, 109i8, 105i8, 122i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 111i8, 114i8,
    116i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8,
    104i8, 101i8, 10i8, 114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 32i8, 102i8, 108i8, 97i8,
    103i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8,
    111i8, 32i8, 114i8, 101i8, 113i8, 117i8, 101i8, 115i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 32i8, 105i8, 110i8, 32i8, 100i8, 101i8, 115i8, 99i8,
    101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_sorted")]
extern "C-unwind" fn builtin_sorted(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut seq: *mut _object = unsafe { core::mem::zeroed() };
    // Keyword arguments are passed through list.sort() which will check
    // them.
    if (unsafe {
        _PyArg_UnpackStack(
            args,
            nargs,
            c"sorted".as_ptr(),
            1i64,
            1i64,
            core::ptr::addr_of_mut!(seq),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let newlist: *mut _object = unsafe { PySequence_List(seq) };
    if newlist.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let callable: *mut _object = unsafe {
        PyObject_GetAttr(
            newlist,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_sort
                    ._ascii
                    .ob_base
            ),
        )
    };
    if callable.is_null() {
        Py_DECREF!(newlist);
        return core::ptr::null_mut::<_object>();
    }
    let v: *mut _object = unsafe {
        PyObject_Vectorcall(
            callable,
            args.wrapping_offset(1 as isize),
            nargs.wrapping_sub(1) as u64,
            kwnames,
        )
    };
    Py_DECREF!(callable);
    if v.is_null() {
        Py_DECREF!(newlist);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(v);
    newlist
}

/// AC: cannot convert yet, as needs PEP 457 group support in inspect
#[unsafe(link_section = ".text.slate_distinct.bltinmodule.builtin_vars")]
extern "C-unwind" fn builtin_vars(self_: *mut _object, args: *mut _object) -> *mut _object {
    let mut v: *mut _object = core::ptr::null_mut::<_object>();
    let mut d: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_UnpackTuple(
            args,
            c"vars".as_ptr(),
            0i64,
            1i64,
            core::ptr::addr_of_mut!(v),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if v.is_null() {
        if !unsafe { _PyEval_GetFrame() }.is_null() {
            d = unsafe { _PyEval_GetFrameLocals() };
        } else {
            let tstate: *mut _ts = _PyThreadState_GET();
            d = unsafe { _PyEval_GetGlobalsFromRunningMain(tstate) };
            if d.is_null() {
                if _PyErr_Occurred(tstate).is_null() {
                    d = unsafe { _PyEval_GetFrameLocals() };
                }
            } else {
                Py_INCREF!(d);
            }
        }
    } else {
        if (unsafe {
            PyObject_GetOptionalAttr(
                v,
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
                core::ptr::addr_of_mut!(d),
            )
        }) == 0
        {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"vars() argument must have __dict__ attribute".as_ptr(),
                )
            };
        }
    }
    d
}

static vars_doc: __SlateAlign16<[i8; 122]> = __SlateAlign16([
    118i8, 97i8, 114i8, 115i8, 40i8, 91i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 93i8, 41i8,
    32i8, 45i8, 62i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8,
    10i8, 10i8, 87i8, 105i8, 116i8, 104i8, 111i8, 117i8, 116i8, 32i8, 97i8, 114i8, 103i8, 117i8,
    109i8, 101i8, 110i8, 116i8, 115i8, 44i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8,
    101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 40i8,
    41i8, 46i8, 10i8, 87i8, 105i8, 116i8, 104i8, 32i8, 97i8, 110i8, 32i8, 97i8, 114i8, 103i8,
    117i8, 109i8, 101i8, 110i8, 116i8, 44i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8,
    101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8,
    95i8, 95i8, 100i8, 105i8, 99i8, 116i8, 95i8, 95i8, 46i8, 0i8,
]);

// Improved Kahan–Babuška algorithm by Arnold Neumaier
// Neumaier, A. (1974), Rundungsfehleranalyse einiger Verfahren
// zur Summation endlicher Summen.  Z. angew. Math. Mech.,
// 54: 39-51. https://doi.org/10.1002/zamm.19740540106
// https://en.wikipedia.org/wiki/Kahan_summation_algorithm#Further_enhancements
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_2744 {
    /// high-order bits for a running sum
    hi: f64,
    /// a running compensation for lost low-order bits
    lo: f64,
}

fn cs_from_double(x: f64) -> __SlateRecord_bltinmodule_c_2744 {
    let mut __slate_compound_0: __SlateRecord_bltinmodule_c_2744 = unsafe { core::mem::zeroed() };
    return unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_bltinmodule_c_2744 = core::mem::zeroed();
                __t0.hi = x;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
}

fn cs_add(mut total: __SlateRecord_bltinmodule_c_2744, x: f64) -> __SlateRecord_bltinmodule_c_2744 {
    let mut __slate_compound_4: __SlateRecord_bltinmodule_c_2744 = unsafe { core::mem::zeroed() };
    let t: f64 = total.hi + x;
    if (unsafe { fabs(total.hi) }) >= unsafe { fabs(x) } {
        total.lo += total.hi - t + x;
    } else {
        total.lo += x - t + total.hi;
    }
    return unsafe {
        *{
            __slate_compound_4 = __SlateRecord_bltinmodule_c_2744 {
                hi: t,
                lo: total.lo,
            };
            core::ptr::addr_of_mut!(__slate_compound_4)
        }
    };
}

fn cs_to_double(mut total: __SlateRecord_bltinmodule_c_2744) -> f64 {
    // Avoid losing the sign on a negative result,
    // and don't let adding the compensation convert
    // an infinite or overflowed sum to a NaN.
    if total.lo != 0.0f64 && total.lo.is_finite() {
        return total.hi + total.lo;
    }
    total.hi
}

// [clinic input]
// sum as builtin_sum
//
//     iterable: object
//     /
//     start: object(c_default="NULL") = 0
//
// Return the sum of a 'start' value (default: 0) plus an iterable of numbers
//
// When the iterable is empty, return the start value.
// This function is intended specifically for use with numeric values and
// may reject non-numeric types.
// [clinic start generated code]
/// [clinic end generated code: output=df758cec7d1d302f input=83d1bc7fd7f7ad3b]
fn builtin_sum_impl(
    module: *mut _object,
    iterable: *mut _object,
    mut result: *mut _object,
) -> *mut _object {
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let iter: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if iter.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if result.is_null() {
        result = unsafe { PyLong_FromLong(0i64) };
        if result.is_null() {
            Py_DECREF!(iter);
            return core::ptr::null_mut::<_object>();
        }
    } else {
        // reject string values for 'start' parameter
        if PyUnicode_Check!(result) != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"sum() can't sum strings [use ''.join(seq) instead]".as_ptr(),
                )
            };
            Py_DECREF!(iter);
            return core::ptr::null_mut::<_object>();
        }
        if PyBytes_Check!(result) != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"sum() can't sum bytes [use b''.join(seq) instead]".as_ptr(),
                )
            };
            Py_DECREF!(iter);
            return core::ptr::null_mut::<_object>();
        }
        if PyByteArray_Check!(result) != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"sum() can't sum bytearray [use b''.join(seq) instead]".as_ptr(),
                )
            };
            Py_DECREF!(iter);
            return core::ptr::null_mut::<_object>();
        }
        Py_INCREF!(result);
    }
    // Fast addition by keeping temporary sums in C instead of new Python objects.
    // Assumes all inputs are the same type.  If the assumption fails, default
    // to the more general routine.
    if PyLong_CheckExact!(result) != 0 {
        let mut overflow: i32 = 0;
        let mut i_result: i64 =
            unsafe { PyLong_AsLongAndOverflow(result, core::ptr::addr_of_mut!(overflow)) };
        // If this already overflowed, don't even enter the loop.
        if overflow == 0 {
            Py_SETREF__3!(result);
        }
        while result.is_null() {
            '__slate_continue_4: {
                item = unsafe { PyIter_Next(iter) };
                if item.is_null() {
                    Py_DECREF!(iter);
                    if !unsafe { PyErr_Occurred() }.is_null() {
                        return core::ptr::null_mut::<_object>();
                    }
                    return unsafe { PyLong_FromSsize_t(i_result) };
                }
                let __v1: bool = PyLong_CheckExact!(item) != 0 || PyBool_Check!(item) != 0;
                if __v1 {
                    let mut b: i64 = 0;
                    overflow = 0;
                    // Single digits are common, fast, and cannot overflow on unpacking.
                    if _PyLong_IsCompact(item as *const _longobject) != 0 {
                        b = _PyLong_CompactValue(item as *const _longobject);
                    } else {
                        b = unsafe {
                            PyLong_AsLongAndOverflow(item, core::ptr::addr_of_mut!(overflow))
                        };
                    }
                    if overflow == 0
                        && (if i_result >= 0 {
                            (b <= 9223372036854775807i64.wrapping_sub(i_result)) as i32
                        } else {
                            (b >= 9223372036854775807i64
                                .wrapping_neg()
                                .wrapping_sub(1)
                                .wrapping_sub(i_result)) as i32
                        }) != 0
                    {
                        i_result = i_result.wrapping_add(b);
                        Py_DECREF!(item);
                        break '__slate_continue_4;
                    }
                }
                // Either overflowed or is not an int. Restore real objects and process normally
                result = unsafe { PyLong_FromSsize_t(i_result) };
                if result.is_null() {
                    Py_DECREF!(item);
                    Py_DECREF!(iter);
                    return core::ptr::null_mut::<_object>();
                }
                temp = unsafe { PyNumber_Add(result, item) };
                Py_DECREF!(result);
                Py_DECREF!(item);
                result = temp;
                if result.is_null() {
                    Py_DECREF!(iter);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
    if PyFloat_CheckExact!(result) != 0 {
        let mut re_sum: __SlateRecord_bltinmodule_c_2744 =
            cs_from_double(PyFloat_AS_DOUBLE!(result));
        Py_SETREF__3!(result);
        while result.is_null() {
            '__slate_continue_6: {
                item = unsafe { PyIter_Next(iter) };
                if item.is_null() {
                    Py_DECREF!(iter);
                    if !unsafe { PyErr_Occurred() }.is_null() {
                        return core::ptr::null_mut::<_object>();
                    }
                    return unsafe { PyFloat_FromDouble(cs_to_double(re_sum)) };
                }
                if PyFloat_CheckExact!(item) != 0 {
                    re_sum = cs_add(re_sum, PyFloat_AS_DOUBLE!(item));
                    _Py_DECREF_SPECIALIZED(item, unsafe {
                        core::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(*mut _object)>,
                        >(_PyFloat_ExactDealloc as *const ())
                    });
                } else {
                    if PyLong_Check!(item) != 0 {
                        let value: f64 = unsafe { PyLong_AsDouble(item) };
                        let __v7: bool = value != -1.0f64 || unsafe { PyErr_Occurred() }.is_null();
                        if __v7 {
                            re_sum = cs_add(re_sum, value);
                            Py_DECREF!(item);
                            break '__slate_continue_6;
                        } else {
                            Py_DECREF!(item);
                            Py_DECREF!(iter);
                            return core::ptr::null_mut::<_object>();
                        }
                    }
                    result = unsafe { PyFloat_FromDouble(cs_to_double(re_sum)) };
                    if result.is_null() {
                        Py_DECREF!(item);
                        Py_DECREF!(iter);
                        return core::ptr::null_mut::<_object>();
                    }
                    temp = unsafe { PyNumber_Add(result, item) };
                    Py_DECREF!(result);
                    Py_DECREF!(item);
                    result = temp;
                    if result.is_null() {
                        Py_DECREF!(iter);
                        return core::ptr::null_mut::<_object>();
                    }
                }
            }
        }
    }
    if PyComplex_CheckExact!(result) != 0 {
        let mut z: __SlateRecord_complexobject_h_8 = unsafe { PyComplex_AsCComplex(result) };
        let mut re_sum: __SlateRecord_bltinmodule_c_2744 = cs_from_double(z.real);
        let mut im_sum: __SlateRecord_bltinmodule_c_2744 = cs_from_double(z.imag);
        Py_SETREF__3!(result);
        while result.is_null() {
            '__slate_continue_9: {
                item = unsafe { PyIter_Next(iter) };
                if item.is_null() {
                    Py_DECREF!(iter);
                    if !unsafe { PyErr_Occurred() }.is_null() {
                        return core::ptr::null_mut::<_object>();
                    }
                    return unsafe {
                        PyComplex_FromDoubles(cs_to_double(re_sum), cs_to_double(im_sum))
                    };
                }
                if PyComplex_CheckExact!(item) != 0 {
                    z = unsafe { PyComplex_AsCComplex(item) };
                    re_sum = cs_add(re_sum, z.real);
                    im_sum = cs_add(im_sum, z.imag);
                    Py_DECREF!(item);
                } else {
                    if PyLong_Check!(item) != 0 {
                        let value: f64 = unsafe { PyLong_AsDouble(item) };
                        let __v10: bool = value != -1.0f64 || unsafe { PyErr_Occurred() }.is_null();
                        if __v10 {
                            re_sum = cs_add(re_sum, value);
                            Py_DECREF!(item);
                            break '__slate_continue_9;
                        } else {
                            Py_DECREF!(item);
                            Py_DECREF!(iter);
                            return core::ptr::null_mut::<_object>();
                        }
                    }
                    if PyFloat_Check!(item) != 0 {
                        let value: f64 = PyFloat_AS_DOUBLE!(item);
                        re_sum = cs_add(re_sum, value);
                        _Py_DECREF_SPECIALIZED(item, unsafe {
                            core::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut _object)>,
                            >(_PyFloat_ExactDealloc as *const ())
                        });
                    } else {
                        result = unsafe {
                            PyComplex_FromDoubles(cs_to_double(re_sum), cs_to_double(im_sum))
                        };
                        if result.is_null() {
                            Py_DECREF!(item);
                            Py_DECREF!(iter);
                            return core::ptr::null_mut::<_object>();
                        }
                        temp = unsafe { PyNumber_Add(result, item) };
                        Py_DECREF!(result);
                        Py_DECREF!(item);
                        result = temp;
                        if result.is_null() {
                            Py_DECREF!(iter);
                            return core::ptr::null_mut::<_object>();
                        }
                    }
                }
            }
        }
    }
    loop {
        item = unsafe { PyIter_Next(iter) };
        if item.is_null() {
            // error, or end-of-sequence
            if !unsafe { PyErr_Occurred() }.is_null() {
                Py_SETREF__3!(result);
            }
            break;
        }
        // It's tempting to use PyNumber_InPlaceAdd instead of
        // PyNumber_Add here, to avoid quadratic running time
        // when doing 'sum(list_of_lists, [])'.  However, this
        // would produce a change in behaviour: a snippet like
        //
        //   empty = []
        //   sum([[x] for x in range(10)], empty)
        //
        // would change the value of empty. In fact, using
        // in-place addition rather that binary addition for
        // any of the steps introduces subtle behavior changes:
        //
        // https://bugs.python.org/issue18305
        temp = unsafe { PyNumber_Add(result, item) };
        Py_DECREF!(result);
        Py_DECREF!(item);
        result = temp;
        if result.is_null() {
            break;
        }
    }
    Py_DECREF!(iter);
    result
}

// [clinic input]
// isinstance as builtin_isinstance
//
//     obj: object
//     class_or_tuple: object
//     /
//
// Return whether an object is an instance of a class or of a subclass thereof.
//
// A tuple, as in ``isinstance(x, (A, B, ...))``, may be given as the
// target to check against.  This is equivalent to ``isinstance(x, A) or
// isinstance(x, B) or ...`` etc.
// [clinic start generated code]
/// [clinic end generated code: output=6faf01472c13b003 input=477d04768621c24b]
fn builtin_isinstance_impl(
    module: *mut _object,
    obj: *mut _object,
    class_or_tuple: *mut _object,
) -> *mut _object {
    let retval: i32 = unsafe { PyObject_IsInstance(obj, class_or_tuple) };
    if retval < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong(retval as i64) }
}

// [clinic input]
// issubclass as builtin_issubclass
//
//     cls: object
//     class_or_tuple: object
//     /
//
// Return whether 'cls' is derived from another class or is the same class.
//
// A tuple, as in ``issubclass(x, (A, B, ...))``, may be given as the
// target to check against.  This is equivalent to ``issubclass(x, A) or
// issubclass(x, B) or ...``.
// [clinic start generated code]
/// [clinic end generated code: output=358412410cd7a250 input=a91ce96345a6705d]
fn builtin_issubclass_impl(
    module: *mut _object,
    cls: *mut _object,
    class_or_tuple: *mut _object,
) -> *mut _object {
    let retval: i32 = unsafe { PyObject_IsSubclass(cls, class_or_tuple) };
    if retval < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong(retval as i64) }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bltinmodule_c_3089 {
    ob_base: _object,
    tuplesize: i64,
    /// tuple of iterators
    ittuple: *mut _object,
    result: *mut _object,
    strict: i32,
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.zip_new")]
extern "C-unwind" fn zip_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut i: i64 = 0;
    let mut tuplesize: i64 = 0;
    let mut strict: i32 = 0;
    if !kwds.is_null() {
        let empty: *mut _object = unsafe { PyTuple_New(0i64) };
        if empty.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let parsed: i32 = unsafe {
            PyArg_ParseTupleAndKeywords(
                empty,
                kwds,
                c"|$p:zip".as_ptr(),
                (core::ptr::addr_of_mut!(kwlist__3108.0) as *mut *mut i8) as *const *mut i8,
                core::ptr::addr_of_mut!(strict),
            )
        };
        Py_DECREF!(empty);
        if parsed == 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    // args must be a tuple
    tuplesize = PyTuple_GET_SIZE!(args);
    // obtain iterators
    // tuple of iterators
    let ittuple: *mut _object = unsafe { PyTuple_New(tuplesize) };
    if ittuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    while i < tuplesize {
        let item: *mut _object = PyTuple_GET_ITEM!(args, i);
        let it: *mut _object = unsafe { PyObject_GetIter(item) };
        if it.is_null() {
            Py_DECREF!(ittuple);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(ittuple, i, it);
        i = i.wrapping_add(1);
    }
    // create a result holder
    let result: *mut _object = unsafe { PyTuple_New(tuplesize) };
    if result.is_null() {
        Py_DECREF!(ittuple);
        return core::ptr::null_mut::<_object>();
    }
    i = 0;
    while i < tuplesize {
        PyTuple_SET_ITEM__2!(
            result,
            i,
            Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
        );
        i = i.wrapping_add(1);
    }
    // create zipobject structure
    let lz: *mut __SlateRecord_bltinmodule_c_3089 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_bltinmodule_c_3089;
    if lz.is_null() {
        Py_DECREF!(ittuple);
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).ittuple = ittuple;
    }
    unsafe {
        (*lz).tuplesize = tuplesize;
    }
    unsafe {
        (*lz).result = result;
    }
    unsafe {
        (*lz).strict = strict;
    }
    lz as *mut _object
}

static mut kwlist__3108: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"strict\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.zip_dealloc")]
extern "C-unwind" fn zip_dealloc(self_: *mut _object) {
    let lz: *mut __SlateRecord_bltinmodule_c_3089 = _zipobject_CAST!(self_);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).ittuple });
    Py_XDECREF!(unsafe { (*lz).result });
    unsafe { unsafe { (*Py_TYPE__2!(lz)).tp_free }.unwrap()(lz as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.zip_traverse")]
extern "C-unwind" fn zip_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_bltinmodule_c_3089 = _zipobject_CAST!(self_);
    Py_VISIT!(unsafe { (*lz).ittuple }, visit, arg);
    Py_VISIT!(unsafe { (*lz).result }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.zip_next")]
extern "C-unwind" fn zip_next(self_: *mut _object) -> *mut _object {
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut tuplesize: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut lz: *mut __SlateRecord_bltinmodule_c_3089 = unsafe { core::mem::zeroed() };
    unsafe {
        lz = _zipobject_CAST!(self_);
        tuplesize = (*lz).tuplesize;
        result = (*lz).result;
        if tuplesize == 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_15: {
                '__join_16: {
                    if _PyObject_IsUniquelyReferenced(result) != 0 {
                        Py_INCREF!(result);
                        i = 0;
                        '__join_21: {
                            while i < tuplesize {
                                it = PyTuple_GET_ITEM!(unsafe { (*lz).ittuple }, i);
                                item = unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it);
                                if item.is_null() {
                                    break '__join_21;
                                } else {
                                    let olditem: *mut _object = PyTuple_GET_ITEM!(result, i);
                                    PyTuple_SET_ITEM__2!(result, i, item);
                                    Py_DECREF!(olditem);
                                    i = i.wrapping_add(1);
                                }
                            }
                            // bpo-42536: The GC may have untracked this result tuple. Since we're
                            //
                            // recycling it, make sure it's tracked again:
                            _PyTuple_Recycle(result);
                            break '__join_16;
                        }
                        Py_DECREF!(result);
                        if (*lz).strict != 0 {
                            break '__join_15;
                        } else {
                            return core::ptr::null_mut::<_object>();
                        }
                    } else {
                        result = PyTuple_New(tuplesize);
                        if result.is_null() {
                            return core::ptr::null_mut::<_object>();
                        } else {
                            i = 0;
                            loop {
                                if i < tuplesize {
                                    it = PyTuple_GET_ITEM!(unsafe { (*lz).ittuple }, i);
                                    item = unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it);
                                    if item.is_null() {
                                        break;
                                    } else {
                                        PyTuple_SET_ITEM__2!(result, i, item);
                                        i = i.wrapping_add(1);
                                    }
                                } else {
                                    break '__join_16;
                                }
                            }
                            Py_DECREF!(result);
                            if (*lz).strict != 0 {
                                break '__join_15;
                            } else {
                                return core::ptr::null_mut::<_object>();
                            }
                        }
                    }
                }
                return result;
            }
            if !PyErr_Occurred().is_null() {
                if PyErr_ExceptionMatches(PyExc_StopIteration) == 0 {
                    // next() on argument i raised an exception (not StopIteration)
                    return core::ptr::null_mut::<_object>();
                } else {
                    PyErr_Clear();
                }
            }
            if i != 0 {
                // ValueError: zip() argument 2 is shorter than argument 1
                //
                // ValueError: zip() argument 3 is shorter than arguments 1-2
                let plural: *const i8 = (if i == 1 {
                    c" ".as_ptr() as *mut i8
                } else {
                    c"s 1-".as_ptr() as *mut i8
                }) as *const i8;
                return PyErr_Format(
                    PyExc_ValueError,
                    c"zip() argument %zd is shorter than argument%s%zd".as_ptr(),
                    i.wrapping_add(1),
                    plural,
                    i,
                );
            } else {
                i = 1;
                while i < tuplesize {
                    it = PyTuple_GET_ITEM!(unsafe { (*lz).ittuple }, i);
                    item = unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it);
                    if !item.is_null() {
                        Py_DECREF!(item);
                        let plural: *const i8 = (if i == 1 {
                            c" ".as_ptr() as *mut i8
                        } else {
                            c"s 1-".as_ptr() as *mut i8
                        }) as *const i8;
                        return PyErr_Format(
                            PyExc_ValueError,
                            c"zip() argument %zd is longer than argument%s%zd".as_ptr(),
                            i.wrapping_add(1),
                            plural,
                            i,
                        );
                    } else {
                        if !PyErr_Occurred().is_null() {
                            if PyErr_ExceptionMatches(PyExc_StopIteration) == 0 {
                                // next() on argument i raised an exception (not StopIteration)
                                return core::ptr::null_mut::<_object>();
                            } else {
                                PyErr_Clear();
                            }
                        }
                        // Argument i is exhausted. So far so good...
                        i = i.wrapping_add(1);
                    }
                }
                // All arguments are exhausted. Success!
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.zip_reduce")]
extern "C-unwind" fn zip_reduce(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let lz: *mut __SlateRecord_bltinmodule_c_3089 = _zipobject_CAST!(self_);
    // Just recreate the zip with the internal iterator tuple
    if (unsafe { (*lz).strict }) != 0 {
        return unsafe {
            PyTuple_Pack(
                3i64,
                Py_TYPE__2!(lz),
                (*lz).ittuple,
                _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
            )
        };
    }
    unsafe { PyTuple_Pack(2i64, Py_TYPE__2!(lz), (*lz).ittuple) }
}

#[unsafe(link_section = ".text.slate_distinct.bltinmodule.zip_setstate")]
extern "C-unwind" fn zip_setstate(self_: *mut _object, state: *mut _object) -> *mut _object {
    let strict: i32 = unsafe { PyObject_IsTrue(state) };
    if strict < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let lz: *mut __SlateRecord_bltinmodule_c_3089 = _zipobject_CAST!(self_);
    unsafe {
        (*lz).strict = strict;
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

/// sentinel
static mut zip_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(zip_reduce);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(zip_setstate);
        __t1.ml_flags = 8i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(setstate_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2
    },
]);

static zip_doc: __SlateAlign16<[i8; 476]> = __SlateAlign16([
    122i8, 105i8, 112i8, 40i8, 42i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 115i8,
    44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 99i8, 116i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 122i8, 105i8, 112i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 121i8, 105i8, 101i8, 108i8, 100i8, 115i8, 32i8, 110i8,
    45i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8,
    44i8, 32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 110i8, 32i8, 105i8, 115i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 10i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 115i8, 32i8, 112i8, 97i8, 115i8, 115i8, 101i8,
    100i8, 32i8, 97i8, 115i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 116i8, 111i8,
    32i8, 122i8, 105i8, 112i8, 40i8, 41i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 105i8, 45i8,
    116i8, 104i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 10i8, 105i8, 110i8, 32i8,
    101i8, 118i8, 101i8, 114i8, 121i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 99i8, 111i8,
    109i8, 101i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8,
    45i8, 116i8, 104i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8,
    114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 122i8, 105i8, 112i8,
    40i8, 41i8, 46i8, 32i8, 32i8, 84i8, 104i8, 105i8, 115i8, 10i8, 99i8, 111i8, 110i8, 116i8,
    105i8, 110i8, 117i8, 101i8, 115i8, 32i8, 117i8, 110i8, 116i8, 105i8, 108i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 104i8, 111i8, 114i8, 116i8, 101i8, 115i8, 116i8, 32i8, 97i8, 114i8, 103i8,
    117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8, 104i8, 97i8, 117i8,
    115i8, 116i8, 101i8, 100i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    99i8, 116i8, 32i8, 105i8, 115i8, 32i8, 116i8, 114i8, 117i8, 101i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 111i8, 110i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8,
    103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8, 104i8,
    97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8, 32i8,
    116i8, 104i8, 101i8, 10i8, 111i8, 116i8, 104i8, 101i8, 114i8, 115i8, 44i8, 32i8, 114i8, 97i8,
    105i8, 115i8, 101i8, 32i8, 97i8, 32i8, 86i8, 97i8, 108i8, 117i8, 101i8, 69i8, 114i8, 114i8,
    111i8, 114i8, 46i8, 10i8, 10i8, 32i8, 32i8, 32i8, 62i8, 62i8, 62i8, 32i8, 108i8, 105i8, 115i8,
    116i8, 40i8, 122i8, 105i8, 112i8, 40i8, 39i8, 97i8, 98i8, 99i8, 100i8, 101i8, 102i8, 103i8,
    39i8, 44i8, 32i8, 114i8, 97i8, 110i8, 103i8, 101i8, 40i8, 51i8, 41i8, 44i8, 32i8, 114i8, 97i8,
    110i8, 103i8, 101i8, 40i8, 52i8, 41i8, 41i8, 41i8, 10i8, 32i8, 32i8, 32i8, 91i8, 40i8, 39i8,
    97i8, 39i8, 44i8, 32i8, 48i8, 44i8, 32i8, 48i8, 41i8, 44i8, 32i8, 40i8, 39i8, 98i8, 39i8, 44i8,
    32i8, 49i8, 44i8, 32i8, 49i8, 41i8, 44i8, 32i8, 40i8, 39i8, 99i8, 39i8, 44i8, 32i8, 50i8, 44i8,
    32i8, 50i8, 41i8, 93i8, 0i8,
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
static mut builtin_methods: __SlateAlign16<[PyMethodDef; 45]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__build_class__\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin___build_class__))
            })
        };
        __t0.ml_flags = 130i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(build_class_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__import__\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin___import__))
            })
        };
        __t1.ml_flags = 130i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(builtin___import____doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"abs\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(builtin_abs);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(builtin_abs__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"all\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(builtin_all);
        __t3.ml_flags = 8i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(builtin_all__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"any\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(builtin_any);
        __t4.ml_flags = 8i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(builtin_any__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"ascii\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(builtin_ascii);
        __t5.ml_flags = 8i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(builtin_ascii__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"bin\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(builtin_bin);
        __t6.ml_flags = 8i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(builtin_bin__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"breakpoint\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_breakpoint))
            })
        };
        __t7.ml_flags = 130i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(breakpoint_doc.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"callable\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(builtin_callable);
        __t8.ml_flags = 8i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(builtin_callable__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"chr\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(builtin_chr);
        __t9.ml_flags = 8i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(builtin_chr__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"compile\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_compile))
            })
        };
        __t10.ml_flags = 130i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(builtin_compile__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"delattr\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_delattr))
            })
        };
        __t11.ml_flags = 128i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(builtin_delattr__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"dir\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(builtin_dir);
        __t12.ml_flags = 1i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(dir_doc.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"divmod\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(builtin_divmod))
            })
        };
        __t13.ml_flags = 128i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(builtin_divmod__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"eval\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(builtin_eval))
            })
        };
        __t14.ml_flags = 130i32;
        __t14.ml_doc = unsafe { core::ptr::addr_of!(builtin_eval__doc__.0) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"exec\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_exec))
            })
        };
        __t15.ml_flags = 130i32;
        __t15.ml_doc = unsafe { core::ptr::addr_of!(builtin_exec__doc__.0) as *const i8 };
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"format\0".as_ptr() as *mut i8) as *const i8;
        __t16.ml_meth = unsafe {
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
                >(Some(builtin_format))
            })
        };
        __t16.ml_flags = 128i32;
        __t16.ml_doc = unsafe { core::ptr::addr_of!(builtin_format__doc__.0) as *const i8 };
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"getattr\0".as_ptr() as *mut i8) as *const i8;
        __t17.ml_meth = unsafe {
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
                >(Some(builtin_getattr))
            })
        };
        __t17.ml_flags = 128i32;
        __t17.ml_doc = unsafe { core::ptr::addr_of!(getattr_doc.0) as *const i8 };
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = (b"globals\0".as_ptr() as *mut i8) as *const i8;
        __t18.ml_meth = Some(builtin_globals);
        __t18.ml_flags = 4i32;
        __t18.ml_doc = unsafe { core::ptr::addr_of!(builtin_globals__doc__.0) as *const i8 };
        __t18
    },
    {
        let mut __t19: PyMethodDef = unsafe { core::mem::zeroed() };
        __t19.ml_name = (b"hasattr\0".as_ptr() as *mut i8) as *const i8;
        __t19.ml_meth = unsafe {
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
                >(Some(builtin_hasattr))
            })
        };
        __t19.ml_flags = 128i32;
        __t19.ml_doc = unsafe { core::ptr::addr_of!(builtin_hasattr__doc__.0) as *const i8 };
        __t19
    },
    {
        let mut __t20: PyMethodDef = unsafe { core::mem::zeroed() };
        __t20.ml_name = (b"hash\0".as_ptr() as *mut i8) as *const i8;
        __t20.ml_meth = Some(builtin_hash);
        __t20.ml_flags = 8i32;
        __t20.ml_doc = unsafe { core::ptr::addr_of!(builtin_hash__doc__.0) as *const i8 };
        __t20
    },
    {
        let mut __t21: PyMethodDef = unsafe { core::mem::zeroed() };
        __t21.ml_name = (b"hex\0".as_ptr() as *mut i8) as *const i8;
        __t21.ml_meth = Some(builtin_hex);
        __t21.ml_flags = 8i32;
        __t21.ml_doc = unsafe { core::ptr::addr_of!(builtin_hex__doc__.0) as *const i8 };
        __t21
    },
    {
        let mut __t22: PyMethodDef = unsafe { core::mem::zeroed() };
        __t22.ml_name = (b"id\0".as_ptr() as *mut i8) as *const i8;
        __t22.ml_meth = Some(builtin_id);
        __t22.ml_flags = 8i32;
        __t22.ml_doc = unsafe { core::ptr::addr_of!(builtin_id__doc__.0) as *const i8 };
        __t22
    },
    {
        let mut __t23: PyMethodDef = unsafe { core::mem::zeroed() };
        __t23.ml_name = (b"input\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_input))
            })
        };
        __t23.ml_flags = 128i32;
        __t23.ml_doc = unsafe { core::ptr::addr_of!(builtin_input__doc__.0) as *const i8 };
        __t23
    },
    {
        let mut __t24: PyMethodDef = unsafe { core::mem::zeroed() };
        __t24.ml_name = (b"isinstance\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_isinstance))
            })
        };
        __t24.ml_flags = 128i32;
        __t24.ml_doc = unsafe { core::ptr::addr_of!(builtin_isinstance__doc__.0) as *const i8 };
        __t24
    },
    {
        let mut __t25: PyMethodDef = unsafe { core::mem::zeroed() };
        __t25.ml_name = (b"issubclass\0".as_ptr() as *mut i8) as *const i8;
        __t25.ml_meth = unsafe {
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
                >(Some(builtin_issubclass))
            })
        };
        __t25.ml_flags = 128i32;
        __t25.ml_doc = unsafe { core::ptr::addr_of!(builtin_issubclass__doc__.0) as *const i8 };
        __t25
    },
    {
        let mut __t26: PyMethodDef = unsafe { core::mem::zeroed() };
        __t26.ml_name = (b"iter\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_iter))
            })
        };
        __t26.ml_flags = 128i32;
        __t26.ml_doc = unsafe { core::ptr::addr_of!(iter_doc.0) as *const i8 };
        __t26
    },
    {
        let mut __t27: PyMethodDef = unsafe { core::mem::zeroed() };
        __t27.ml_name = (b"aiter\0".as_ptr() as *mut i8) as *const i8;
        __t27.ml_meth = Some(builtin_aiter);
        __t27.ml_flags = 8i32;
        __t27.ml_doc = unsafe { core::ptr::addr_of!(builtin_aiter__doc__.0) as *const i8 };
        __t27
    },
    {
        let mut __t28: PyMethodDef = unsafe { core::mem::zeroed() };
        __t28.ml_name = (b"len\0".as_ptr() as *mut i8) as *const i8;
        __t28.ml_meth = Some(builtin_len);
        __t28.ml_flags = 8i32;
        __t28.ml_doc = unsafe { core::ptr::addr_of!(builtin_len__doc__.0) as *const i8 };
        __t28
    },
    {
        let mut __t29: PyMethodDef = unsafe { core::mem::zeroed() };
        __t29.ml_name = (b"locals\0".as_ptr() as *mut i8) as *const i8;
        __t29.ml_meth = Some(builtin_locals);
        __t29.ml_flags = 4i32;
        __t29.ml_doc = unsafe { core::ptr::addr_of!(builtin_locals__doc__.0) as *const i8 };
        __t29
    },
    {
        let mut __t30: PyMethodDef = unsafe { core::mem::zeroed() };
        __t30.ml_name = (b"max\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(builtin_max))
            })
        };
        __t30.ml_flags = 130i32;
        __t30.ml_doc = unsafe { core::ptr::addr_of!(max_doc.0) as *const i8 };
        __t30
    },
    {
        let mut __t31: PyMethodDef = unsafe { core::mem::zeroed() };
        __t31.ml_name = (b"min\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(builtin_min))
            })
        };
        __t31.ml_flags = 130i32;
        __t31.ml_doc = unsafe { core::ptr::addr_of!(min_doc.0) as *const i8 };
        __t31
    },
    {
        let mut __t32: PyMethodDef = unsafe { core::mem::zeroed() };
        __t32.ml_name = (b"next\0".as_ptr() as *mut i8) as *const i8;
        __t32.ml_meth = unsafe {
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
                >(Some(builtin_next))
            })
        };
        __t32.ml_flags = 128i32;
        __t32.ml_doc = unsafe { core::ptr::addr_of!(next_doc.0) as *const i8 };
        __t32
    },
    {
        let mut __t33: PyMethodDef = unsafe { core::mem::zeroed() };
        __t33.ml_name = (b"anext\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(builtin_anext))
            })
        };
        __t33.ml_flags = 128i32;
        __t33.ml_doc = unsafe { core::ptr::addr_of!(builtin_anext__doc__.0) as *const i8 };
        __t33
    },
    {
        let mut __t34: PyMethodDef = unsafe { core::mem::zeroed() };
        __t34.ml_name = (b"oct\0".as_ptr() as *mut i8) as *const i8;
        __t34.ml_meth = Some(builtin_oct);
        __t34.ml_flags = 8i32;
        __t34.ml_doc = unsafe { core::ptr::addr_of!(builtin_oct__doc__.0) as *const i8 };
        __t34
    },
    {
        let mut __t35: PyMethodDef = unsafe { core::mem::zeroed() };
        __t35.ml_name = (b"ord\0".as_ptr() as *mut i8) as *const i8;
        __t35.ml_meth = Some(builtin_ord);
        __t35.ml_flags = 8i32;
        __t35.ml_doc = unsafe { core::ptr::addr_of!(builtin_ord__doc__.0) as *const i8 };
        __t35
    },
    {
        let mut __t36: PyMethodDef = unsafe { core::mem::zeroed() };
        __t36.ml_name = (b"pow\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_pow))
            })
        };
        __t36.ml_flags = 130i32;
        __t36.ml_doc = unsafe { core::ptr::addr_of!(builtin_pow__doc__.0) as *const i8 };
        __t36
    },
    {
        let mut __t37: PyMethodDef = unsafe { core::mem::zeroed() };
        __t37.ml_name = (b"print\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(builtin_print))
            })
        };
        __t37.ml_flags = 130i32;
        __t37.ml_doc = unsafe { core::ptr::addr_of!(builtin_print__doc__.0) as *const i8 };
        __t37
    },
    {
        let mut __t38: PyMethodDef = unsafe { core::mem::zeroed() };
        __t38.ml_name = (b"repr\0".as_ptr() as *mut i8) as *const i8;
        __t38.ml_meth = Some(builtin_repr);
        __t38.ml_flags = 8i32;
        __t38.ml_doc = unsafe { core::ptr::addr_of!(builtin_repr__doc__.0) as *const i8 };
        __t38
    },
    {
        let mut __t39: PyMethodDef = unsafe { core::mem::zeroed() };
        __t39.ml_name = (b"round\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_round))
            })
        };
        __t39.ml_flags = 130i32;
        __t39.ml_doc = unsafe { core::ptr::addr_of!(builtin_round__doc__.0) as *const i8 };
        __t39
    },
    {
        let mut __t40: PyMethodDef = unsafe { core::mem::zeroed() };
        __t40.ml_name = (b"setattr\0".as_ptr() as *mut i8) as *const i8;
        __t40.ml_meth = unsafe {
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
                >(Some(builtin_setattr))
            })
        };
        __t40.ml_flags = 128i32;
        __t40.ml_doc = unsafe { core::ptr::addr_of!(builtin_setattr__doc__.0) as *const i8 };
        __t40
    },
    {
        let mut __t41: PyMethodDef = unsafe { core::mem::zeroed() };
        __t41.ml_name = (b"sorted\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(builtin_sorted))
            })
        };
        __t41.ml_flags = 130i32;
        __t41.ml_doc = unsafe { core::ptr::addr_of!(builtin_sorted__doc__.0) as *const i8 };
        __t41
    },
    {
        let mut __t42: PyMethodDef = unsafe { core::mem::zeroed() };
        __t42.ml_name = (b"sum\0".as_ptr() as *mut i8) as *const i8;
        __t42.ml_meth = unsafe {
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
                >(Some(builtin_sum))
            })
        };
        __t42.ml_flags = 130i32;
        __t42.ml_doc = unsafe { core::ptr::addr_of!(builtin_sum__doc__.0) as *const i8 };
        __t42
    },
    {
        let mut __t43: PyMethodDef = unsafe { core::mem::zeroed() };
        __t43.ml_name = (b"vars\0".as_ptr() as *mut i8) as *const i8;
        __t43.ml_meth = Some(builtin_vars);
        __t43.ml_flags = 1i32;
        __t43.ml_doc = unsafe { core::ptr::addr_of!(vars_doc.0) as *const i8 };
        __t43
    },
    {
        let mut __t44: PyMethodDef = unsafe { core::mem::zeroed() };
        __t44.ml_name = core::ptr::null::<i8>();
        __t44.ml_meth = None;
        __t44
    },
]);

static builtin_doc: __SlateAlign16<[i8; 427]> = __SlateAlign16([
    66i8, 117i8, 105i8, 108i8, 116i8, 45i8, 105i8, 110i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 115i8, 44i8, 32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 44i8, 32i8, 101i8,
    120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 115i8, 44i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8,
    46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 101i8, 115i8, 32i8, 100i8, 105i8, 114i8, 101i8,
    99i8, 116i8, 32i8, 97i8, 99i8, 99i8, 101i8, 115i8, 115i8, 32i8, 116i8, 111i8, 32i8, 97i8,
    108i8, 108i8, 32i8, 39i8, 98i8, 117i8, 105i8, 108i8, 116i8, 45i8, 105i8, 110i8, 39i8, 10i8,
    105i8, 100i8, 101i8, 110i8, 116i8, 105i8, 102i8, 105i8, 101i8, 114i8, 115i8, 32i8, 111i8,
    102i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 59i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    101i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 44i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8,
    105i8, 110i8, 115i8, 46i8, 108i8, 101i8, 110i8, 32i8, 105i8, 115i8, 10i8, 116i8, 104i8, 101i8,
    32i8, 102i8, 117i8, 108i8, 108i8, 32i8, 110i8, 97i8, 109i8, 101i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8, 45i8, 105i8, 110i8, 32i8,
    102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 108i8, 101i8, 110i8, 40i8, 41i8,
    46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 110i8, 111i8, 114i8, 109i8, 97i8, 108i8,
    108i8, 121i8, 32i8, 97i8, 99i8, 99i8, 101i8, 115i8, 115i8, 101i8, 100i8, 32i8, 101i8, 120i8,
    112i8, 108i8, 105i8, 99i8, 105i8, 116i8, 108i8, 121i8, 32i8, 98i8, 121i8, 32i8, 109i8, 111i8,
    115i8, 116i8, 10i8, 97i8, 112i8, 112i8, 108i8, 105i8, 99i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    115i8, 44i8, 32i8, 98i8, 117i8, 116i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 117i8,
    115i8, 101i8, 102i8, 117i8, 108i8, 32i8, 105i8, 110i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8,
    101i8, 10i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 110i8, 97i8, 109i8, 101i8,
    32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8, 45i8, 105i8, 110i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 98i8, 117i8, 116i8, 32i8, 105i8, 110i8,
    10i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 105i8,
    108i8, 116i8, 45i8, 105i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8,
    110i8, 97i8, 109i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 110i8,
    101i8, 101i8, 100i8, 101i8, 100i8, 46i8, 0i8,
]);

/// multiple "initialization" just copies the module dict.
static mut builtinsmodule: PyModuleDef = PyModuleDef {
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
    m_name: (b"builtins\0".as_ptr() as *mut i8) as *const i8,
    m_doc: unsafe { core::ptr::addr_of!(builtin_doc.0) as *const i8 },
    m_size: 1i32.wrapping_neg() as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(builtin_methods.0) as *mut PyMethodDef },
    m_slots: core::ptr::null_mut::<PyModuleDef_Slot>(),
    m_traverse: None,
    m_clear: None,
    m_free: None,
};

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBuiltin_Init(interp: *mut _is) -> *mut _object {
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    let config: *const PyConfig = unsafe { _PyInterpreterState_GetConfig(interp) };
    let r#mod: *mut _object =
        unsafe { _PyModule_CreateInitialized(core::ptr::addr_of_mut!(builtinsmodule), 1013i32) };
    if r#mod.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    dict = unsafe { PyModule_GetDict(r#mod) };
    SETBUILTIN__2!(
        c"None".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
        dict
    );
    SETBUILTIN__2!(
        c"Ellipsis".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(_Py_EllipsisObject) },
        dict
    );
    SETBUILTIN__2!(
        c"NotImplemented".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) },
        dict
    );
    SETBUILTIN__2!(
        c"False".as_ptr() as *mut i8,
        _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) }),
        dict
    );
    SETBUILTIN__2!(
        c"True".as_ptr() as *mut i8,
        _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) }),
        dict
    );
    SETBUILTIN!(
        c"bool".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyBool_Type) },
        dict
    );
    SETBUILTIN!(
        c"memoryview".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyMemoryView_Type) },
        dict
    );
    SETBUILTIN!(
        c"bytearray".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyByteArray_Type) },
        dict
    );
    SETBUILTIN!(
        c"bytes".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyBytes_Type) },
        dict
    );
    SETBUILTIN!(
        c"classmethod".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyClassMethod_Type) },
        dict
    );
    SETBUILTIN!(
        c"complex".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyComplex_Type) },
        dict
    );
    SETBUILTIN!(
        c"dict".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyDict_Type) },
        dict
    );
    SETBUILTIN!(
        c"enumerate".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyEnum_Type) },
        dict
    );
    SETBUILTIN!(
        c"filter".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyFilter_Type) },
        dict
    );
    SETBUILTIN!(
        c"float".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyFloat_Type) },
        dict
    );
    SETBUILTIN!(
        c"frozenset".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyFrozenSet_Type) },
        dict
    );
    SETBUILTIN!(
        c"property".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyProperty_Type) },
        dict
    );
    SETBUILTIN!(
        c"int".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyLong_Type) },
        dict
    );
    SETBUILTIN!(
        c"list".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyList_Type) },
        dict
    );
    SETBUILTIN!(
        c"map".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyMap_Type) },
        dict
    );
    SETBUILTIN!(
        c"object".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyBaseObject_Type) },
        dict
    );
    SETBUILTIN!(
        c"range".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyRange_Type) },
        dict
    );
    SETBUILTIN!(
        c"reversed".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyReversed_Type) },
        dict
    );
    SETBUILTIN!(
        c"set".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PySet_Type) },
        dict
    );
    SETBUILTIN!(
        c"slice".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PySlice_Type) },
        dict
    );
    SETBUILTIN!(
        c"staticmethod".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyStaticMethod_Type) },
        dict
    );
    SETBUILTIN!(
        c"str".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyUnicode_Type) },
        dict
    );
    SETBUILTIN!(
        c"super".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PySuper_Type) },
        dict
    );
    SETBUILTIN!(
        c"tuple".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        dict
    );
    SETBUILTIN!(
        c"type".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyType_Type) },
        dict
    );
    SETBUILTIN!(
        c"zip".as_ptr() as *mut i8,
        unsafe { core::ptr::addr_of_mut!(PyZip_Type) },
        dict
    );
    let debug: *mut _object =
        unsafe { PyBool_FromLong(((*config).optimization_level == 0) as i64) };
    if (unsafe { PyDict_SetItemString(dict, c"__debug__".as_ptr(), debug) }) < 0 {
        Py_DECREF!(debug);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(debug);
    r#mod
}
