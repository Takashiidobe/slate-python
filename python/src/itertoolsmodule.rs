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
macro_rules! ADD_TYPE {
    ($module:expr, $type:expr, $spec:expr) => {{
        loop {
            unsafe {
                $type = PyType_FromModuleAndSpec($module, $spec, core::ptr::null_mut::<_object>())
                    as *mut _typeobject;
            }
            if unsafe { $type }.is_null() {
                return 1i32.wrapping_neg();
            }
            if (unsafe { PyModule_AddType($module, $type) }) < 0 {
                return 1i32.wrapping_neg();
            }
            if true {
                break;
            }
        }
    }};
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

macro_rules! PyDict_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyDict_Type) })
    };
}

macro_rules! PyDict_GET_SIZE {
    ($op:expr) => {
        PyDict_GET_SIZE(_PyObject_CAST!($op))
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

macro_rules! PyLong_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 16777216u64)
    };
}

macro_rules! PyMem_New {
    ($type:ty, $n:expr) => {{
        let __v: *mut $type =
            if (($n) as u64) > 9223372036854775807 / (core::mem::size_of::<$type>() as u64) {
                core::ptr::null_mut::<i64>()
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

macro_rules! PyTuple_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
    };
}

macro_rules! PyTuple_SET_ITEM__4 {
    ($op:expr, $index:expr) => {
        PyTuple_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST__4!());
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

macro_rules! Py_CLEAR__28 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut __SlateRecord_itertoolsmodule_c_741 =
                unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut __SlateRecord_itertoolsmodule_c_741 = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<__SlateRecord_itertoolsmodule_c_741>();
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

macro_rules! Py_SETREF__11 {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut __SlateRecord_itertoolsmodule_c_741 =
                unsafe { core::ptr::addr_of_mut!($dst) };
            let _tmp_old_dst: *mut __SlateRecord_itertoolsmodule_c_741 = unsafe { *_tmp_dst_ptr };
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

macro_rules! Py_SET_TYPE__2 {
    ($ob:expr, $type:expr) => {
        Py_SET_TYPE(_PyObject_CAST__2!($ob), $type);
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
macro_rules! _PyObject_CAST__4 {
    () => {
        _Py_CAST__8!()
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
macro_rules! _Py_CAST__8 {
    () => {
        core::ptr::null_mut::<_object>()
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

macro_rules! _grouperobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_607
    };
}

macro_rules! accumulateobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_3092
    };
}

macro_rules! batchedobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_105
    };
}

macro_rules! chainobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_1889
    };
}

macro_rules! clinic_state {
    ($type:expr) => {
        find_state_by_type($type)
    };
}

macro_rules! clinic_state_by_cls {
    ($base_tp:expr) => {
        get_module_state_by_cls($base_tp)
    };
}

macro_rules! combinationsobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_2337
    };
}

macro_rules! compressobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_3239
    };
}

macro_rules! countobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_3492
    };
}

macro_rules! cwrobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_2590
    };
}

macro_rules! cycleobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_1214
    };
}

macro_rules! dropwhileobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_1347
    };
}

macro_rules! filterfalseobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_3368
    };
}

macro_rules! groupbyobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_437
    };
}

macro_rules! isliceobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_1591
    };
}

macro_rules! pairwiseobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_280
    };
}

macro_rules! permutationsobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_2839
    };
}

macro_rules! productobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_2068
    };
}

macro_rules! repeatobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_3735
    };
}

macro_rules! starmapobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_1780
    };
}

macro_rules! takewhileobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_1471
    };
}

macro_rules! teedataobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_741
    };
}

macro_rules! teeobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_751
    };
}

macro_rules! ziplongestobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_itertoolsmodule_c_3868
    };
}

unsafe extern "C" {
    static mut PyBool_Type: _typeobject;
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyExc_StopIteration: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
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
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn _PyType_Name(arg0: *mut _typeobject) -> *const i8;
    fn PyType_GetModuleByDef(arg0: *mut _typeobject, arg1: *mut PyModuleDef) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn _PyObject_GC_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyObject_GC_Track(arg0: *mut ());
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyObject_GC_Del(arg0: *mut ());
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Size(arg0: *mut _object) -> i64;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn _PyTuple_Resize(arg0: *mut *mut _object, arg1: i64) -> i32;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_GetItemRef(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_GetItemWithError(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn Py_GenericAlias(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
    fn PyArg_UnpackTuple(arg0: *mut _object, arg1: *const i8, arg2: i64, arg3: i64, ...) -> i32;
    fn PyModule_AddType(arg0: *mut _object, arg1: *mut _typeobject) -> i32;
    fn PyObject_Call(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyObject_CallFunctionObjArgs(arg0: *mut _object, ...) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyNumber_Check(arg0: *mut _object) -> i32;
    fn PyNumber_Add(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PySequence_Tuple(arg0: *mut _object) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyTuple_FromArray(arg0: *const *mut _object, arg1: i64) -> *mut _object;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
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

fn Py_SET_TYPE(ob: *mut _object, r#type: *mut _typeobject) {
    unsafe {
        (*ob).ob_type = r#type;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_377 {
    slot: i32,
    pfunc: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_385 {
    name: *const i8,
    basicsize: i32,
    itemsize: i32,
    flags: u32,
    slots: *mut __SlateRecord_object_h_377,
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
struct _dictkeysobject {}

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
    __slate_bits_0: __slate_bits::__SlateBits147U0,
}

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF!(obj);
    obj
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
struct _PyExecutorObject {}

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
// METH_CLASS and METH_STATIC are a little different; these control
// the construction of methods for a class.  These cannot be used for
// functions in modules.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits200U0,
}

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
struct code_arena_st {}

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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_crossinterp_h_150 {
    basic: Option<unsafe extern "C-unwind" fn(*mut _ts, *mut _object, *mut _xidata) -> i32>,
    fallback: Option<unsafe extern "C-unwind" fn(*mut _ts, *mut _object, i32, *mut _xidata) -> i32>,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct __SlateRecord_pycore_obmalloc_h_622 {
    tail_hi: i32,
    tail_lo: i32,
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
struct __SlateRecord_siginfo_t_h_124 {
    si_signo: i32,
    si_errno: i32,
    si_code: i32,
    __pad0: i32,
    _sifields: __SlateRecord_siginfo_t_h_124_2,
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
struct __SlateRecord_itertoolsmodule_c_h_49 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_168 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_530 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_604 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_677 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_742 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_820 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_h_921 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
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

fn _PyModule_GetState(r#mod: *mut _object) -> *mut () {
    unsafe { (*(r#mod as *mut __SlateRecord_pycore_moduleobject_h_30)).md_state }
}

fn _PyType_GetModuleState(r#type: *mut _typeobject) -> *mut () {
    let r#mod: *mut __SlateRecord_pycore_moduleobject_h_30 =
        (unsafe { (*(r#type as *mut _heaptypeobject)).ht_module })
            as *mut __SlateRecord_pycore_moduleobject_h_30;
    unsafe { (*r#mod).md_state }
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

// Fast inlined version of PyType_IS_GC()
fn _PyTuple_Recycle(op: *mut _object) {
    _PyTuple_RESET_HASH_CACHE!(op);
    if _PyObject_GC_IS_TRACKED!(op) == 0 {
        _PyObject_GC_TRACK__2!(op);
    }
}

// _PyEval_GetBuiltin()
// Py_BEGIN_CRITICAL_SECTION()
// _PyLong_GetZero()
// _PyModule_GetState()
// _PyType_GetModuleState()
// _PyObject_GC_TRACK()
// _PyTuple_ITEMS()
// offsetof()
// Itertools module written and maintained
// by Raymond D. Hettinger <python@rcn.com>
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_40 {
    accumulate_type: *mut _typeobject,
    batched_type: *mut _typeobject,
    chain_type: *mut _typeobject,
    combinations_type: *mut _typeobject,
    compress_type: *mut _typeobject,
    count_type: *mut _typeobject,
    cwr_type: *mut _typeobject,
    cycle_type: *mut _typeobject,
    dropwhile_type: *mut _typeobject,
    filterfalse_type: *mut _typeobject,
    groupby_type: *mut _typeobject,
    _grouper_type: *mut _typeobject,
    islice_type: *mut _typeobject,
    pairwise_type: *mut _typeobject,
    permutations_type: *mut _typeobject,
    product_type: *mut _typeobject,
    repeat_type: *mut _typeobject,
    starmap_type: *mut _typeobject,
    takewhile_type: *mut _typeobject,
    tee_type: *mut _typeobject,
    teedataobject_type: *mut _typeobject,
    ziplongest_type: *mut _typeobject,
}

fn get_module_state(r#mod: *mut _object) -> *mut __SlateRecord_itertoolsmodule_c_40 {
    let state: *mut () = _PyModule_GetState(r#mod);
    state as *mut __SlateRecord_itertoolsmodule_c_40
}

fn get_module_state_by_cls(cls: *mut _typeobject) -> *mut __SlateRecord_itertoolsmodule_c_40 {
    let state: *mut () = _PyType_GetModuleState(cls);
    state as *mut __SlateRecord_itertoolsmodule_c_40
}

static mut itertoolsmodule: PyModuleDef = PyModuleDef {
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
    m_name: (b"itertools\0".as_ptr() as *mut i8) as *const i8,
    m_doc: unsafe { core::ptr::addr_of!(module_doc.0) as *const i8 },
    m_size: (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_40>() as u64) as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(module_methods.0) as *mut PyMethodDef },
    m_slots: unsafe { core::ptr::addr_of_mut!(itertoolsmodule_slots.0) as *mut PyModuleDef_Slot },
    m_traverse: Some(itertoolsmodule_traverse),
    m_clear: Some(itertoolsmodule_clear),
    m_free: Some(itertoolsmodule_free),
};

fn find_state_by_type(tp: *mut _typeobject) -> *mut __SlateRecord_itertoolsmodule_c_40 {
    let r#mod: *mut _object =
        unsafe { PyType_GetModuleByDef(tp, core::ptr::addr_of_mut!(itertoolsmodule)) };
    get_module_state(r#mod)
}

static batched_new__doc__: __SlateAlign16<[i8; 554]> = __SlateAlign16([
    98i8, 97i8, 116i8, 99i8, 104i8, 101i8, 100i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 44i8, 32i8, 110i8, 44i8, 32i8, 42i8, 44i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    99i8, 116i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 66i8,
    97i8, 116i8, 99i8, 104i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 105i8, 110i8, 116i8, 111i8,
    32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8, 110i8,
    103i8, 116i8, 104i8, 32i8, 110i8, 46i8, 32i8, 84i8, 104i8, 101i8, 32i8, 108i8, 97i8, 115i8,
    116i8, 32i8, 98i8, 97i8, 116i8, 99i8, 104i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8,
    115i8, 104i8, 111i8, 114i8, 116i8, 101i8, 114i8, 32i8, 116i8, 104i8, 97i8, 110i8, 32i8, 110i8,
    46i8, 10i8, 10i8, 76i8, 111i8, 111i8, 112i8, 115i8, 32i8, 111i8, 118i8, 101i8, 114i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 97i8, 99i8, 99i8, 117i8, 109i8,
    117i8, 108i8, 97i8, 116i8, 101i8, 115i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 105i8, 110i8,
    116i8, 111i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8, 10i8, 117i8, 112i8, 32i8, 116i8,
    111i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 110i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8,
    32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 105i8, 115i8, 32i8, 99i8, 111i8, 110i8, 115i8,
    117i8, 109i8, 101i8, 100i8, 32i8, 108i8, 97i8, 122i8, 105i8, 108i8, 121i8, 44i8, 32i8, 106i8,
    117i8, 115i8, 116i8, 32i8, 101i8, 110i8, 111i8, 117i8, 103i8, 104i8, 32i8, 116i8, 111i8, 10i8,
    102i8, 105i8, 108i8, 108i8, 32i8, 97i8, 32i8, 98i8, 97i8, 116i8, 99i8, 104i8, 46i8, 32i8, 32i8,
    84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8,
    121i8, 105i8, 101i8, 108i8, 100i8, 101i8, 100i8, 32i8, 97i8, 115i8, 32i8, 115i8, 111i8, 111i8,
    110i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 98i8, 97i8, 116i8, 99i8, 104i8, 32i8, 105i8, 115i8,
    32i8, 102i8, 117i8, 108i8, 108i8, 10i8, 111i8, 114i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8, 104i8, 97i8, 117i8, 115i8,
    116i8, 101i8, 100i8, 46i8, 10i8, 10i8, 32i8, 32i8, 32i8, 32i8, 62i8, 62i8, 62i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 98i8, 97i8, 116i8, 99i8, 104i8, 32i8, 105i8, 110i8, 32i8, 98i8, 97i8,
    116i8, 99i8, 104i8, 101i8, 100i8, 40i8, 39i8, 65i8, 66i8, 67i8, 68i8, 69i8, 70i8, 71i8, 39i8,
    44i8, 32i8, 51i8, 41i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8, 46i8, 46i8, 46i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 40i8, 98i8, 97i8, 116i8, 99i8, 104i8, 41i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 46i8, 46i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 40i8, 39i8, 65i8,
    39i8, 44i8, 32i8, 39i8, 66i8, 39i8, 44i8, 32i8, 39i8, 67i8, 39i8, 41i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 40i8, 39i8, 68i8, 39i8, 44i8, 32i8, 39i8, 69i8, 39i8, 44i8, 32i8, 39i8, 70i8, 39i8, 41i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 40i8, 39i8, 71i8, 39i8, 44i8, 41i8, 10i8, 10i8, 73i8, 102i8,
    32i8, 34i8, 115i8, 116i8, 114i8, 105i8, 99i8, 116i8, 34i8, 32i8, 105i8, 115i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 44i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 115i8, 32i8, 97i8, 32i8,
    86i8, 97i8, 108i8, 117i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 102i8, 105i8, 110i8, 97i8, 108i8, 32i8, 98i8, 97i8, 116i8, 99i8,
    104i8, 32i8, 105i8, 115i8, 32i8, 115i8, 104i8, 111i8, 114i8, 116i8, 101i8, 114i8, 10i8, 116i8,
    104i8, 97i8, 110i8, 32i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.batched_new")]
extern "C-unwind" fn batched_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut strict: i32 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v6: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_10: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs__0.is_null() {
            __v6 = PyDict_GET_SIZE!(kwargs__0);
        } else {
            __v6 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v6).wrapping_sub(2);
            strict = 0;
            fastargs = _PyArg_UnpackKeywords__2!(
                (*(args as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object,
                nargs,
                kwargs__0,
                core::ptr::null_mut::<()>(),
                core::ptr::addr_of_mut!(_parser__62),
                2i32,
                2i32,
                0i32,
                0i32,
                (*__slate_slot_10).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !fastargs.is_null() {
                iterable = *fastargs.wrapping_offset(0 as isize);
                ival = 1i32.wrapping_neg() as i64;
                iobj = _PyNumber_Index(*fastargs.wrapping_offset(1 as isize));
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
                    n = ival;
                    if noptargs != 0 {
                        strict = PyObject_IsTrue(*fastargs.wrapping_offset(2 as isize));
                        if strict < 0 {
                            break '__join_0;
                        }
                    }
                    return_value = batched_new_impl(r#type, iterable, n, strict);
                }
            }
        }
        return return_value;
    }
}

static mut _kwtuple__49: __SlateRecord_itertoolsmodule_c_h_49 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_49 = unsafe { core::mem::zeroed() };
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
                    ._py_iterable
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.ascii)
                as *mut __SlateRecord_pycore_global_strings_h_815
        }
        .wrapping_offset(110i32 as isize) as *mut _object,
        unsafe {
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
        },
    ];
    __t0
};

static _keywords__61: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"iterable\0".as_ptr() as *mut i8) as *const i8,
    (b"n\0".as_ptr() as *mut i8) as *const i8,
    (b"strict\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__62: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__61.0) as *const *const i8 };
    __t0.fname = (b"batched\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__49.ob_base.ob_base) };
    __t0
};

static pairwise_new__doc__: __SlateAlign16<[i8; 139]> = __SlateAlign16([
    112i8, 97i8, 105i8, 114i8, 119i8, 105i8, 115i8, 101i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8,
    114i8, 32i8, 111i8, 102i8, 32i8, 111i8, 118i8, 101i8, 114i8, 108i8, 97i8, 112i8, 112i8, 105i8,
    110i8, 103i8, 32i8, 112i8, 97i8, 105i8, 114i8, 115i8, 32i8, 116i8, 97i8, 107i8, 101i8, 110i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 112i8, 117i8,
    116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 46i8, 10i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 115i8, 32i8, 45i8, 62i8, 32i8, 40i8, 115i8, 48i8, 44i8, 115i8, 49i8, 41i8,
    44i8, 32i8, 40i8, 115i8, 49i8, 44i8, 115i8, 50i8, 41i8, 44i8, 32i8, 40i8, 115i8, 50i8, 44i8,
    32i8, 115i8, 51i8, 41i8, 44i8, 32i8, 46i8, 46i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.pairwise_new")]
extern "C-unwind" fn pairwise_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).pairwise_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"pairwise".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"pairwise".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                1i32,
                1i32
            ) {
                let iterable: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                return_value = pairwise_new_impl(r#type, iterable);
            }
        }
        return return_value;
    }
}

static itertools_groupby__doc__: __SlateAlign16<[i8; 356]> = __SlateAlign16([
    103i8, 114i8, 111i8, 117i8, 112i8, 98i8, 121i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 109i8, 97i8, 107i8, 101i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8,
    101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 99i8, 111i8, 110i8, 115i8, 101i8, 99i8, 117i8, 116i8,
    105i8, 118i8, 101i8, 32i8, 107i8, 101i8, 121i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 103i8,
    114i8, 111i8, 117i8, 112i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 10i8, 10i8, 32i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 69i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 100i8, 105i8, 118i8, 105i8,
    100i8, 101i8, 32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8, 115i8,
    32i8, 97i8, 99i8, 99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 107i8, 101i8, 121i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8,
    111i8, 110i8, 46i8, 10i8, 32i8, 32i8, 107i8, 101i8, 121i8, 10i8, 32i8, 32i8, 32i8, 32i8, 65i8,
    32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    99i8, 111i8, 109i8, 112i8, 117i8, 116i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    103i8, 114i8, 111i8, 117i8, 112i8, 32i8, 99i8, 97i8, 116i8, 101i8, 103i8, 111i8, 114i8, 121i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 101i8, 108i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 73i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 107i8, 101i8, 121i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 78i8, 111i8, 110i8, 101i8,
    44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    105i8, 116i8, 115i8, 101i8, 108i8, 102i8, 10i8, 32i8, 32i8, 32i8, 32i8, 105i8, 115i8, 32i8,
    117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8,
    105i8, 110i8, 103i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_groupby")]
extern "C-unwind" fn itertools_groupby(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut keyfunc: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs__0.is_null() {
            __v2 = PyDict_GET_SIZE!(kwargs__0);
        } else {
            __v2 = 0;
        }
        noptargs = nargs.wrapping_add(__v2).wrapping_sub(1);
        keyfunc = core::ptr::addr_of_mut!(_Py_NoneStruct);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__181),
            1i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_6).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            it = *fastargs.wrapping_offset(0 as isize);
            if noptargs != 0 {
                keyfunc = *fastargs.wrapping_offset(1 as isize);
            }
            return_value = itertools_groupby_impl(r#type, it, keyfunc);
        }
        return return_value;
    }
}

static mut _kwtuple__168: __SlateRecord_itertoolsmodule_c_h_168 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_168 = unsafe { core::mem::zeroed() };
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
                    ._py_iterable
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
                    ._py_key
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__180: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"iterable\0".as_ptr() as *mut i8) as *const i8,
    (b"key\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__181: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__180.0) as *const *const i8 };
    __t0.fname = (b"groupby\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__168.ob_base.ob_base) };
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools__grouper")]
extern "C-unwind" fn itertools__grouper(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut base_tp: *mut _typeobject = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        base_tp = (*clinic_state!(r#type))._grouper_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"_grouper".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"_grouper".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                2i32,
                2i32
            ) {
                if PyObject_TypeCheck!(PyTuple_GET_ITEM!(args, 0i32), unsafe {
                    (*clinic_state_by_cls!(base_tp)).groupby_type
                }) == 0
                {
                    _PyArg_BadArgument(
                        c"_grouper".as_ptr(),
                        c"argument 1".as_ptr(),
                        (*(*clinic_state_by_cls!(base_tp)).groupby_type).tp_name,
                        PyTuple_GET_ITEM!(args, 0i32),
                    );
                } else {
                    let parent: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                    let tgtkey: *mut _object = PyTuple_GET_ITEM!(args, 1i32);
                    return_value = itertools__grouper_impl(r#type, parent, tgtkey);
                }
            }
        }
        return return_value;
    }
}

static itertools_teedataobject__doc__: __SlateAlign16<[i8; 92]> = __SlateAlign16([
    116i8, 101i8, 101i8, 100i8, 97i8, 116i8, 97i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 40i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 115i8, 44i8, 32i8, 110i8, 101i8, 120i8, 116i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 68i8, 97i8, 116i8, 97i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8,
    101i8, 114i8, 32i8, 99i8, 111i8, 109i8, 109i8, 111i8, 110i8, 32i8, 116i8, 111i8, 32i8, 109i8,
    117i8, 108i8, 116i8, 105i8, 112i8, 108i8, 101i8, 32i8, 116i8, 101i8, 101i8, 32i8, 111i8, 98i8,
    106i8, 101i8, 99i8, 116i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_teedataobject")]
extern "C-unwind" fn itertools_teedataobject(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).teedataobject_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"teedataobject".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"teedataobject".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                3i32,
                3i32
            ) {
                it = PyTuple_GET_ITEM!(args, 0i32);
                if PyList_Check!(PyTuple_GET_ITEM!(args, 1i32)) == 0 {
                    _PyArg_BadArgument(
                        c"teedataobject".as_ptr(),
                        c"argument 2".as_ptr(),
                        c"list".as_ptr(),
                        PyTuple_GET_ITEM!(args, 1i32),
                    );
                } else {
                    let values: *mut _object = PyTuple_GET_ITEM!(args, 1i32);
                    let next: *mut _object = PyTuple_GET_ITEM!(args, 2i32);
                    return_value = itertools_teedataobject_impl(r#type, it, values, next);
                }
            }
        }
        return return_value;
    }
}

static itertools__tee__doc__: __SlateAlign16<[i8; 60]> = __SlateAlign16([
    95i8, 116i8, 101i8, 101i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 73i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8,
    114i8, 32i8, 119i8, 114i8, 97i8, 112i8, 112i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 109i8,
    97i8, 107i8, 101i8, 32i8, 105i8, 116i8, 32i8, 99i8, 111i8, 112i8, 121i8, 97i8, 98i8, 108i8,
    101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools__tee")]
extern "C-unwind" fn itertools__tee(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).tee_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"_tee".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"_tee".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                1i32,
                1i32
            ) {
                let iterable: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                return_value = itertools__tee_impl(r#type, iterable);
            }
        }
        return return_value;
    }
}

static itertools_tee__doc__: __SlateAlign16<[i8; 79]> = __SlateAlign16([
    116i8, 101i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 110i8, 61i8, 50i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8,
    32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 110i8, 32i8, 105i8, 110i8,
    100i8, 101i8, 112i8, 101i8, 110i8, 100i8, 101i8, 110i8, 116i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_tee")]
extern "C-unwind" fn itertools_tee(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            n = 2;
            if _PyArg_CheckPositional!(c"tee".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
                iterable = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
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
                        n = ival;
                    }
                }
                return_value = itertools_tee_impl(module, iterable, n);
            }
        }
        return return_value;
    }
}

static itertools_cycle__doc__: __SlateAlign16<[i8; 119]> = __SlateAlign16([
    99i8, 121i8, 99i8, 108i8, 101i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    117i8, 110i8, 116i8, 105i8, 108i8, 32i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8,
    104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 46i8, 32i8, 84i8, 104i8, 101i8, 110i8, 32i8,
    114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 113i8,
    117i8, 101i8, 110i8, 99i8, 101i8, 32i8, 105i8, 110i8, 100i8, 101i8, 102i8, 105i8, 110i8, 105i8,
    116i8, 101i8, 108i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_cycle")]
extern "C-unwind" fn itertools_cycle(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).cycle_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"cycle".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"cycle".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                1i32,
                1i32
            ) {
                let iterable: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                return_value = itertools_cycle_impl(r#type, iterable);
            }
        }
        return return_value;
    }
}

static itertools_dropwhile__doc__: __SlateAlign16<[i8; 165]> = __SlateAlign16([
    100i8, 114i8, 111i8, 112i8, 119i8, 104i8, 105i8, 108i8, 101i8, 40i8, 112i8, 114i8, 101i8,
    100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 68i8, 114i8, 111i8, 112i8,
    32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 119i8, 104i8, 105i8,
    108i8, 101i8, 32i8, 112i8, 114i8, 101i8, 100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 40i8, 105i8,
    116i8, 101i8, 109i8, 41i8, 32i8, 105i8, 115i8, 32i8, 116i8, 114i8, 117i8, 101i8, 46i8, 10i8,
    10i8, 65i8, 102i8, 116i8, 101i8, 114i8, 119i8, 97i8, 114i8, 100i8, 115i8, 44i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 101i8, 118i8, 101i8, 114i8, 121i8, 32i8, 101i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 117i8, 110i8, 116i8, 105i8, 108i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8,
    101i8, 120i8, 104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_dropwhile")]
extern "C-unwind" fn itertools_dropwhile(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).dropwhile_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"dropwhile".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"dropwhile".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                2i32,
                2i32
            ) {
                let func: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                let seq: *mut _object = PyTuple_GET_ITEM!(args, 1i32);
                return_value = itertools_dropwhile_impl(r#type, func, seq);
            }
        }
        return return_value;
    }
}

static itertools_takewhile__doc__: __SlateAlign16<[i8; 140]> = __SlateAlign16([
    116i8, 97i8, 107i8, 101i8, 119i8, 104i8, 105i8, 108i8, 101i8, 40i8, 112i8, 114i8, 101i8, 100i8,
    105i8, 99i8, 97i8, 116i8, 101i8, 44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 115i8, 117i8, 99i8, 99i8, 101i8, 115i8, 115i8, 105i8, 118i8, 101i8, 32i8, 101i8,
    110i8, 116i8, 114i8, 105i8, 101i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 110i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 115i8, 32i8, 108i8,
    111i8, 110i8, 103i8, 32i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 101i8,
    100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 32i8, 101i8, 118i8, 97i8, 108i8, 117i8, 97i8, 116i8,
    101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 116i8, 114i8, 117i8, 101i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 101i8, 110i8, 116i8, 114i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_takewhile")]
extern "C-unwind" fn itertools_takewhile(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).takewhile_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"takewhile".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"takewhile".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                2i32,
                2i32
            ) {
                let func: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                let seq: *mut _object = PyTuple_GET_ITEM!(args, 1i32);
                return_value = itertools_takewhile_impl(r#type, func, seq);
            }
        }
        return return_value;
    }
}

static itertools_starmap__doc__: __SlateAlign16<[i8; 162]> = __SlateAlign16([
    115i8, 116i8, 97i8, 114i8, 109i8, 97i8, 112i8, 40i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8,
    111i8, 110i8, 44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8,
    110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 119i8, 104i8, 111i8,
    115i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 101i8,
    118i8, 97i8, 108i8, 117i8, 97i8, 116i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8,
    97i8, 110i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 116i8, 117i8,
    112i8, 108i8, 101i8, 32i8, 116i8, 97i8, 107i8, 101i8, 110i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 115i8, 101i8, 113i8,
    117i8, 101i8, 110i8, 99i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_starmap")]
extern "C-unwind" fn itertools_starmap(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).starmap_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"starmap".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"starmap".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                2i32,
                2i32
            ) {
                let func: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                let seq: *mut _object = PyTuple_GET_ITEM!(args, 1i32);
                return_value = itertools_starmap_impl(r#type, func, seq);
            }
        }
        return return_value;
    }
}

static itertools_chain_from_iterable__doc__: __SlateAlign16<[i8; 127]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 95i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 40i8,
    36i8, 116i8, 121i8, 112i8, 101i8, 44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 108i8, 116i8, 101i8, 114i8,
    110i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 99i8, 104i8, 97i8, 105i8, 110i8, 40i8, 41i8,
    32i8, 99i8, 111i8, 110i8, 115i8, 116i8, 114i8, 117i8, 99i8, 116i8, 111i8, 114i8, 32i8, 116i8,
    97i8, 107i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8, 115i8, 105i8, 110i8, 103i8, 108i8, 101i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8,
    109i8, 101i8, 110i8, 116i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 101i8, 118i8, 97i8, 108i8,
    117i8, 97i8, 116i8, 101i8, 115i8, 32i8, 108i8, 97i8, 122i8, 105i8, 108i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_chain_from_iterable")]
extern "C-unwind" fn itertools_chain_from_iterable(
    r#type: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = itertools_chain_from_iterable_impl(r#type as *mut _typeobject, arg);
    return_value
}

static itertools_combinations__doc__: __SlateAlign16<[i8; 165]> = __SlateAlign16([
    99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 40i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 114i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 115i8, 117i8, 99i8, 99i8, 101i8, 115i8,
    115i8, 105i8, 118i8, 101i8, 32i8, 114i8, 45i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8,
    99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 111i8,
    102i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 46i8, 10i8,
    10i8, 99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 40i8,
    114i8, 97i8, 110i8, 103i8, 101i8, 40i8, 52i8, 41i8, 44i8, 32i8, 51i8, 41i8, 32i8, 45i8, 45i8,
    62i8, 32i8, 40i8, 48i8, 44i8, 49i8, 44i8, 50i8, 41i8, 44i8, 32i8, 40i8, 48i8, 44i8, 49i8, 44i8,
    51i8, 41i8, 44i8, 32i8, 40i8, 48i8, 44i8, 50i8, 44i8, 51i8, 41i8, 44i8, 32i8, 40i8, 49i8, 44i8,
    50i8, 44i8, 51i8, 41i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_combinations")]
extern "C-unwind" fn itertools_combinations(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let nargs: i64 = PyTuple_GET_SIZE!(args);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__543),
            2i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_7).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            iterable = *fastargs.wrapping_offset(0 as isize);
            ival = 1i32.wrapping_neg() as i64;
            iobj = _PyNumber_Index(*fastargs.wrapping_offset(1 as isize));
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
                let r: i64 = ival;
                return_value = itertools_combinations_impl(r#type, iterable, r);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__530: __SlateRecord_itertoolsmodule_c_h_530 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_530 = unsafe { core::mem::zeroed() };
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
                    ._py_iterable
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.ascii)
                as *mut __SlateRecord_pycore_global_strings_h_815
        }
        .wrapping_offset(114i32 as isize) as *mut _object,
    ];
    __t0
};

static _keywords__542: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"iterable\0".as_ptr() as *mut i8) as *const i8,
    (b"r\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__543: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__542.0) as *const *const i8 };
    __t0.fname = (b"combinations\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__530.ob_base.ob_base) };
    __t0
};

static itertools_combinations_with_replacement__doc__: __SlateAlign16<[i8; 282]> =
    __SlateAlign16([
        99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 95i8,
        119i8, 105i8, 116i8, 104i8, 95i8, 114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 109i8,
        101i8, 110i8, 116i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8,
        32i8, 114i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
        32i8, 115i8, 117i8, 99i8, 99i8, 101i8, 115i8, 115i8, 105i8, 118i8, 101i8, 32i8, 114i8,
        45i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 99i8, 111i8, 109i8, 98i8, 105i8,
        110i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 111i8, 102i8, 32i8, 101i8, 108i8,
        101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8,
        32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 108i8, 108i8,
        111i8, 119i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 100i8, 105i8, 118i8, 105i8, 100i8,
        117i8, 97i8, 108i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8,
        116i8, 111i8, 32i8, 104i8, 97i8, 118i8, 101i8, 32i8, 115i8, 117i8, 99i8, 99i8, 101i8,
        115i8, 115i8, 105i8, 118i8, 101i8, 32i8, 114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 115i8,
        46i8, 10i8, 10i8, 99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8,
        115i8, 95i8, 119i8, 105i8, 116i8, 104i8, 95i8, 114i8, 101i8, 112i8, 108i8, 97i8, 99i8,
        101i8, 109i8, 101i8, 110i8, 116i8, 40i8, 39i8, 65i8, 66i8, 67i8, 39i8, 44i8, 32i8, 50i8,
        41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 40i8, 39i8, 65i8, 39i8, 44i8, 39i8, 65i8, 39i8, 41i8,
        44i8, 32i8, 40i8, 39i8, 65i8, 39i8, 44i8, 39i8, 66i8, 39i8, 41i8, 44i8, 32i8, 40i8, 39i8,
        65i8, 39i8, 44i8, 39i8, 67i8, 39i8, 41i8, 44i8, 32i8, 40i8, 39i8, 66i8, 39i8, 44i8, 39i8,
        66i8, 39i8, 41i8, 44i8, 32i8, 40i8, 39i8, 66i8, 39i8, 44i8, 39i8, 67i8, 39i8, 41i8, 44i8,
        32i8, 40i8, 39i8, 67i8, 39i8, 44i8, 39i8, 67i8, 39i8, 41i8, 0i8,
    ]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_combinations_with_replacement")]
extern "C-unwind" fn itertools_combinations_with_replacement(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let nargs: i64 = PyTuple_GET_SIZE!(args);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__617),
            2i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_7).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            iterable = *fastargs.wrapping_offset(0 as isize);
            ival = 1i32.wrapping_neg() as i64;
            iobj = _PyNumber_Index(*fastargs.wrapping_offset(1 as isize));
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
                let r: i64 = ival;
                return_value = itertools_combinations_with_replacement_impl(r#type, iterable, r);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__604: __SlateRecord_itertoolsmodule_c_h_604 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_604 = unsafe { core::mem::zeroed() };
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
                    ._py_iterable
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.ascii)
                as *mut __SlateRecord_pycore_global_strings_h_815
        }
        .wrapping_offset(114i32 as isize) as *mut _object,
    ];
    __t0
};

static _keywords__616: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"iterable\0".as_ptr() as *mut i8) as *const i8,
    (b"r\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__617: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__616.0) as *const *const i8 };
    __t0.fname = (b"combinations_with_replacement\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__604.ob_base.ob_base) };
    __t0
};

static itertools_permutations__doc__: __SlateAlign16<[i8; 176]> = __SlateAlign16([
    112i8, 101i8, 114i8, 109i8, 117i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 40i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 114i8, 61i8, 78i8, 111i8, 110i8,
    101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    115i8, 117i8, 99i8, 99i8, 101i8, 115i8, 115i8, 105i8, 118i8, 101i8, 32i8, 114i8, 45i8, 108i8,
    101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 112i8, 101i8, 114i8, 109i8, 117i8, 116i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 115i8, 32i8, 111i8, 102i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8,
    116i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8,
    97i8, 98i8, 108i8, 101i8, 46i8, 10i8, 10i8, 112i8, 101i8, 114i8, 109i8, 117i8, 116i8, 97i8,
    116i8, 105i8, 111i8, 110i8, 115i8, 40i8, 114i8, 97i8, 110i8, 103i8, 101i8, 40i8, 51i8, 41i8,
    44i8, 32i8, 50i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 40i8, 48i8, 44i8, 49i8, 41i8, 44i8, 32i8,
    40i8, 48i8, 44i8, 50i8, 41i8, 44i8, 32i8, 40i8, 49i8, 44i8, 48i8, 41i8, 44i8, 32i8, 40i8, 49i8,
    44i8, 50i8, 41i8, 44i8, 32i8, 40i8, 50i8, 44i8, 48i8, 41i8, 44i8, 32i8, 40i8, 50i8, 44i8, 49i8,
    41i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_permutations")]
extern "C-unwind" fn itertools_permutations(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut robj: *mut _object = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs__0.is_null() {
            __v2 = PyDict_GET_SIZE!(kwargs__0);
        } else {
            __v2 = 0;
        }
        noptargs = nargs.wrapping_add(__v2).wrapping_sub(1);
        robj = core::ptr::addr_of_mut!(_Py_NoneStruct);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__690),
            1i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_6).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            iterable = *fastargs.wrapping_offset(0 as isize);
            if noptargs != 0 {
                robj = *fastargs.wrapping_offset(1 as isize);
            }
            return_value = itertools_permutations_impl(r#type, iterable, robj);
        }
        return return_value;
    }
}

static mut _kwtuple__677: __SlateRecord_itertoolsmodule_c_h_677 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_677 = unsafe { core::mem::zeroed() };
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
                    ._py_iterable
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.ascii)
                as *mut __SlateRecord_pycore_global_strings_h_815
        }
        .wrapping_offset(114i32 as isize) as *mut _object,
    ];
    __t0
};

static _keywords__689: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"iterable\0".as_ptr() as *mut i8) as *const i8,
    (b"r\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__690: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__689.0) as *const *const i8 };
    __t0.fname = (b"permutations\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__677.ob_base.ob_base) };
    __t0
};

static itertools_accumulate__doc__: __SlateAlign16<[i8; 123]> = __SlateAlign16([
    97i8, 99i8, 99i8, 117i8, 109i8, 117i8, 108i8, 97i8, 116i8, 101i8, 40i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 61i8, 78i8, 111i8,
    110i8, 101i8, 44i8, 32i8, 42i8, 44i8, 32i8, 105i8, 110i8, 105i8, 116i8, 105i8, 97i8, 108i8,
    61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 115i8, 101i8, 114i8, 105i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 97i8,
    99i8, 99i8, 117i8, 109i8, 117i8, 108i8, 97i8, 116i8, 101i8, 100i8, 32i8, 115i8, 117i8, 109i8,
    115i8, 32i8, 40i8, 111i8, 114i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 98i8, 105i8,
    110i8, 97i8, 114i8, 121i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 115i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_accumulate")]
extern "C-unwind" fn itertools_accumulate(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut initial: *mut _object = unsafe { core::mem::zeroed() };
    let mut binop: *mut _object = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs__0.is_null() {
            __v5 = PyDict_GET_SIZE!(kwargs__0);
        } else {
            __v5 = 0;
        }
        noptargs = nargs.wrapping_add(__v5).wrapping_sub(1);
        binop = core::ptr::addr_of_mut!(_Py_NoneStruct);
        initial = core::ptr::addr_of_mut!(_Py_NoneStruct);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__755),
            1i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_9).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            iterable = *fastargs.wrapping_offset(0 as isize);
            if noptargs != 0 {
                if !unsafe { *fastargs.wrapping_offset(1 as isize) }.is_null() {
                    binop = *fastargs.wrapping_offset(1 as isize);
                    noptargs = noptargs.wrapping_sub(1);
                }
            }
            if noptargs != 0 {
                initial = *fastargs.wrapping_offset(2 as isize);
            }
            return_value = itertools_accumulate_impl(r#type, iterable, binop, initial);
        }
        return return_value;
    }
}

static mut _kwtuple__742: __SlateRecord_itertoolsmodule_c_h_742 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_742 = unsafe { core::mem::zeroed() };
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
                    ._py_iterable
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
                    ._py_func
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
                    ._py_initial
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__754: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"iterable\0".as_ptr() as *mut i8) as *const i8,
    (b"func\0".as_ptr() as *mut i8) as *const i8,
    (b"initial\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__755: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__754.0) as *const *const i8 };
    __t0.fname = (b"accumulate\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__742.ob_base.ob_base) };
    __t0
};

static itertools_compress__doc__: __SlateAlign16<[i8; 195]> = __SlateAlign16([
    99i8, 111i8, 109i8, 112i8, 114i8, 101i8, 115i8, 115i8, 40i8, 100i8, 97i8, 116i8, 97i8, 44i8,
    32i8, 115i8, 101i8, 108i8, 101i8, 99i8, 116i8, 111i8, 114i8, 115i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8,
    101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 99i8, 111i8, 114i8, 114i8, 101i8,
    115i8, 112i8, 111i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 114i8,
    117i8, 101i8, 32i8, 115i8, 101i8, 108i8, 101i8, 99i8, 116i8, 111i8, 114i8, 32i8, 101i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 46i8, 10i8, 10i8, 70i8, 111i8, 114i8, 109i8, 115i8,
    32i8, 97i8, 32i8, 115i8, 104i8, 111i8, 114i8, 116i8, 101i8, 114i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 115i8, 101i8, 108i8,
    101i8, 99i8, 116i8, 101i8, 100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 101i8, 108i8, 101i8,
    109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 101i8, 108i8, 101i8, 99i8, 116i8, 111i8, 114i8, 115i8, 10i8, 116i8, 111i8,
    32i8, 99i8, 104i8, 111i8, 111i8, 115i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 97i8,
    116i8, 97i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_compress")]
extern "C-unwind" fn itertools_compress(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let nargs: i64 = PyTuple_GET_SIZE!(args);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__833),
            2i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_4).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            let seq1: *mut _object = *fastargs.wrapping_offset(0 as isize);
            let seq2: *mut _object = *fastargs.wrapping_offset(1 as isize);
            return_value = itertools_compress_impl(r#type, seq1, seq2);
        }
        return return_value;
    }
}

static mut _kwtuple__820: __SlateRecord_itertoolsmodule_c_h_820 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_820 = unsafe { core::mem::zeroed() };
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
                    ._py_data
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
                    ._py_selectors
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__832: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"data\0".as_ptr() as *mut i8) as *const i8,
    (b"selectors\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__833: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__832.0) as *const *const i8 };
    __t0.fname = (b"compress\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__820.ob_base.ob_base) };
    __t0
};

static itertools_filterfalse__doc__: __SlateAlign16<[i8; 160]> = __SlateAlign16([
    102i8, 105i8, 108i8, 116i8, 101i8, 114i8, 102i8, 97i8, 108i8, 115i8, 101i8, 40i8, 102i8, 117i8,
    110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 111i8, 115i8, 101i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8,
    32i8, 111i8, 102i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 40i8, 105i8, 116i8, 101i8, 109i8, 41i8, 32i8, 105i8, 115i8, 32i8, 102i8,
    97i8, 108i8, 115i8, 101i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 102i8, 117i8, 110i8, 99i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8,
    109i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 97i8, 114i8, 101i8, 32i8, 102i8, 97i8,
    108i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_filterfalse")]
extern "C-unwind" fn itertools_filterfalse(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = (*clinic_state!(r#type)).filterfalse_type;
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"filterfalse".as_ptr() as *mut i8, kwargs__0);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"filterfalse".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                2i32,
                2i32
            ) {
                let func: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
                let seq: *mut _object = PyTuple_GET_ITEM!(args, 1i32);
                return_value = itertools_filterfalse_impl(r#type, func, seq);
            }
        }
        return return_value;
    }
}

static itertools_count__doc__: __SlateAlign16<[i8; 233]> = __SlateAlign16([
    99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 115i8, 116i8, 97i8, 114i8, 116i8, 61i8, 48i8, 44i8,
    32i8, 115i8, 116i8, 101i8, 112i8, 61i8, 49i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 46i8, 95i8,
    95i8, 110i8, 101i8, 120i8, 116i8, 95i8, 95i8, 40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8,
    111i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 99i8, 111i8, 110i8,
    115i8, 101i8, 99i8, 117i8, 116i8, 105i8, 118i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8,
    115i8, 46i8, 10i8, 10i8, 69i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8,
    32i8, 116i8, 111i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8, 100i8, 101i8, 102i8, 32i8, 99i8, 111i8,
    117i8, 110i8, 116i8, 40i8, 102i8, 105i8, 114i8, 115i8, 116i8, 118i8, 97i8, 108i8, 61i8, 48i8,
    44i8, 32i8, 115i8, 116i8, 101i8, 112i8, 61i8, 49i8, 41i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 120i8, 32i8, 61i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 118i8,
    97i8, 108i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 119i8, 104i8, 105i8, 108i8,
    101i8, 32i8, 49i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 121i8, 105i8, 101i8, 108i8, 100i8, 32i8, 120i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 120i8, 32i8, 43i8, 61i8, 32i8, 115i8, 116i8, 101i8,
    112i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertools_count")]
extern "C-unwind" fn itertools_count(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let mut long_step: *mut _object = unsafe { core::mem::zeroed() };
    let mut long_cnt: *mut _object = unsafe { core::mem::zeroed() };
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
        if !kwargs__0.is_null() {
            __v4 = PyDict_GET_SIZE!(kwargs__0);
        } else {
            __v4 = 0;
        }
        noptargs = nargs.wrapping_add(__v4).wrapping_sub(0);
        long_cnt = core::ptr::null_mut::<_object>();
        long_step = core::ptr::null_mut::<_object>();
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs__0,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__934),
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
            '__join_1: {
                if noptargs != 0 {
                    if !unsafe { *fastargs.wrapping_offset(0 as isize) }.is_null() {
                        long_cnt = *fastargs.wrapping_offset(0 as isize);
                        let __v0: i64 = noptargs.wrapping_sub(1);
                        noptargs = __v0;
                        if __v0 == 0 {
                            break '__join_1;
                        }
                    }
                    long_step = *fastargs.wrapping_offset(1 as isize);
                }
            }
            return_value = itertools_count_impl(r#type, long_cnt, long_step);
        }
        return return_value;
    }
}

static mut _kwtuple__921: __SlateRecord_itertoolsmodule_c_h_921 = {
    let mut __t0: __SlateRecord_itertoolsmodule_c_h_921 = unsafe { core::mem::zeroed() };
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
                    ._py_start
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
                    ._py_step
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__933: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"start\0".as_ptr() as *mut i8) as *const i8,
    (b"step\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__934: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__933.0) as *const *const i8 };
    __t0.fname = (b"count\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__921.ob_base.ob_base) };
    __t0
};

// [clinic input]
// module itertools
// class itertools.groupby "groupbyobject *" "clinic_state()->groupby_type"
// class itertools._grouper "_grouperobject *" "clinic_state()->_grouper_type"
// class itertools.teedataobject "teedataobject *" "clinic_state()->teedataobject_type"
// class itertools._tee "teeobject *" "clinic_state()->tee_type"
// class itertools.batched "batchedobject *" "clinic_state()->batched_type"
// class itertools.cycle "cycleobject *" "clinic_state()->cycle_type"
// class itertools.dropwhile "dropwhileobject *" "clinic_state()->dropwhile_type"
// class itertools.takewhile "takewhileobject *" "clinic_state()->takewhile_type"
// class itertools.starmap "starmapobject *" "clinic_state()->starmap_type"
// class itertools.chain "chainobject *" "clinic_state()->chain_type"
// class itertools.combinations "combinationsobject *" "clinic_state()->combinations_type"
// class itertools.combinations_with_replacement "cwr_object *" "clinic_state()->cwr_type"
// class itertools.permutations "permutationsobject *" "clinic_state()->permutations_type"
// class itertools.accumulate "accumulateobject *" "clinic_state()->accumulate_type"
// class itertools.compress "compressobject *" "clinic_state()->compress_type"
// class itertools.filterfalse "filterfalseobject *" "clinic_state()->filterfalse_type"
// class itertools.count "countobject *" "clinic_state()->count_type"
// class itertools.pairwise "pairwiseobject *" "clinic_state()->pairwise_type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=aa48fe4de9d4080f]
// batched object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_105 {
    ob_base: _object,
    it: *mut _object,
    batch_size: i64,
    strict: bool,
}

// [clinic input]
// @classmethod
// itertools.batched.__new__ as batched_new
//     iterable: object
//     n: Py_ssize_t
//     *
//     strict: bool = False
//
// Batch data into tuples of length n. The last batch may be shorter than n.
//
// Loops over the input iterable and accumulates data into tuples
// up to size n.  The input is consumed lazily, just enough to
// fill a batch.  The result is yielded as soon as a batch is full
// or when the input iterable is exhausted.
//
//     >>> for batch in batched('ABCDEFG', 3):
//     ...     print(batch)
//     ...
//     ('A', 'B', 'C')
//     ('D', 'E', 'F')
//     ('G',)
//
// If "strict" is True, raises a ValueError if the final batch is shorter
// than n.
//
// [clinic start generated code]
/// [clinic end generated code: output=c6de11b061529d3e input=7814b47e222f5467]
fn batched_new_impl(
    r#type: *mut _typeobject,
    iterable: *mut _object,
    n: i64,
    strict: i32,
) -> *mut _object {
    if n < 1 {
        // We could define the n==0 case to return an empty iterator
        // but that is at odds with the idea that batching should
        // never throw-away input data.
        unsafe { PyErr_SetString(PyExc_ValueError, c"n must be at least one".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create batchedobject structure
    let bo: *mut __SlateRecord_itertoolsmodule_c_105 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_105;
    if bo.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*bo).batch_size = n;
    }
    unsafe {
        (*bo).it = it;
    }
    unsafe {
        (*bo).strict = strict != 0;
    }
    bo as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.batched_dealloc")]
extern "C-unwind" fn batched_dealloc(op: *mut _object) {
    let bo: *mut __SlateRecord_itertoolsmodule_c_105 = batchedobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(bo);
    unsafe { PyObject_GC_UnTrack(bo as *mut ()) };
    Py_XDECREF!(unsafe { (*bo).it });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(bo as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.batched_traverse")]
extern "C-unwind" fn batched_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let bo: *mut __SlateRecord_itertoolsmodule_c_105 = batchedobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(bo as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*bo).it }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.batched_next")]
extern "C-unwind" fn batched_next(op: *mut _object) -> *mut _object {
    let mut items: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut iternext: core::mem::MaybeUninit<
        Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    > = core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        core::ptr::addr_of_mut!(iternext)
            as *mut Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>;
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut bo: *mut __SlateRecord_itertoolsmodule_c_105 = unsafe { core::mem::zeroed() };
    unsafe {
        bo = batchedobject_CAST!(op);
        n = (*bo).batch_size;
        it = (*bo).it;
        if n < 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            result = PyTuple_New(n);
            if result.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                core::ptr::write(__slate_slot_3, (*Py_TYPE!(it)).tp_iternext);
                items = _PyTuple_ITEMS!(result);
                i = 0;
                '__join_8: {
                    while i < n {
                        item = (*__slate_slot_3).unwrap()(it);
                        if item.is_null() {
                            break '__join_8;
                        } else {
                            *items.wrapping_offset(i as isize) = item;
                            i = i.wrapping_add(1);
                        }
                    }
                    return result;
                }
                if !PyErr_Occurred().is_null() {
                    if PyErr_ExceptionMatches(PyExc_StopIteration) == 0 {
                        // Input raised an exception other than StopIteration
                        FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*bo).batch_size, 1i32.wrapping_neg());
                        Py_CLEAR!((*bo).it);
                        Py_DECREF!(result);
                        return core::ptr::null_mut::<_object>();
                    } else {
                        PyErr_Clear();
                    }
                }
                if i == 0 {
                    FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*bo).batch_size, 1i32.wrapping_neg());
                    Py_CLEAR!((*bo).it);
                    Py_DECREF!(result);
                    return core::ptr::null_mut::<_object>();
                } else {
                    if (*bo).strict {
                        FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*bo).batch_size, 1i32.wrapping_neg());
                        Py_CLEAR!((*bo).it);
                        Py_DECREF!(result);
                        PyErr_SetString(PyExc_ValueError, c"batched(): incomplete batch".as_ptr());
                        return core::ptr::null_mut::<_object>();
                    } else {
                        _PyTuple_Resize(core::ptr::addr_of_mut!(result), i);
                        return result;
                    }
                }
            }
        }
    }
}

static mut batched_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(batched_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(batched_new__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(batched_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(batched_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 47i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
                >(PyType_GenericAlloc as *const ())
            })
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(batched_new))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut batched_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.batched\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_105>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(batched_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// pairwise object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_280 {
    ob_base: _object,
    it: *mut _object,
    old: *mut _object,
    result: *mut _object,
}

// [clinic input]
// @classmethod
// itertools.pairwise.__new__ as pairwise_new
//     iterable: object
//     /
// Return an iterator of overlapping pairs taken from the input iterator.
//
//     s -> (s0,s1), (s1,s2), (s2, s3), ...
//
// [clinic start generated code]
/// [clinic end generated code: output=9f0267062d384456 input=6e7c3cddb431a8d6]
fn pairwise_new_impl(r#type: *mut _typeobject, iterable: *mut _object) -> *mut _object {
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let po: *mut __SlateRecord_itertoolsmodule_c_280 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_280;
    if po.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*po).it = it;
    }
    unsafe {
        (*po).old = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*po).result = PyTuple_Pack(
            2i64,
            core::ptr::addr_of_mut!(_Py_NoneStruct),
            core::ptr::addr_of_mut!(_Py_NoneStruct),
        );
    }
    if unsafe { (*po).result }.is_null() {
        Py_DECREF__2!(po);
        return core::ptr::null_mut::<_object>();
    }
    po as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.pairwise_dealloc")]
extern "C-unwind" fn pairwise_dealloc(op: *mut _object) {
    let po: *mut __SlateRecord_itertoolsmodule_c_280 = pairwiseobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(po);
    unsafe { PyObject_GC_UnTrack(po as *mut ()) };
    Py_XDECREF!(unsafe { (*po).it });
    Py_XDECREF!(unsafe { (*po).old });
    Py_XDECREF!(unsafe { (*po).result });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(po as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.pairwise_traverse")]
extern "C-unwind" fn pairwise_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let po: *mut __SlateRecord_itertoolsmodule_c_280 = pairwiseobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(po as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*po).it }, visit, arg);
    Py_VISIT!(unsafe { (*po).old }, visit, arg);
    Py_VISIT!(unsafe { (*po).result }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.pairwise_next")]
extern "C-unwind" fn pairwise_next(op: *mut _object) -> *mut _object {
    let po: *mut __SlateRecord_itertoolsmodule_c_280 = pairwiseobject_CAST!(op);
    let mut it: *mut _object = unsafe { (*po).it };
    let mut old: *mut _object = unsafe { (*po).old };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if old.is_null() {
        old = unsafe { unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it) };
        Py_XSETREF!((*po).old, old);
        if old.is_null() {
            Py_CLEAR!((*po).it);
            return core::ptr::null_mut::<_object>();
        }
        it = unsafe { (*po).it };
        if it.is_null() {
            Py_CLEAR!((*po).old);
            return core::ptr::null_mut::<_object>();
        }
    }
    Py_INCREF!(old);
    let new: *mut _object = unsafe { unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it) };
    if new.is_null() {
        Py_CLEAR!((*po).it);
        Py_CLEAR!((*po).old);
        Py_DECREF!(old);
        return core::ptr::null_mut::<_object>();
    }
    result = unsafe { (*po).result };
    if _PyObject_IsUniquelyReferenced(result) != 0 {
        Py_INCREF!(result);
        let last_old: *mut _object = PyTuple_GET_ITEM!(result, 0i32);
        let last_new: *mut _object = PyTuple_GET_ITEM!(result, 1i32);
        PyTuple_SET_ITEM!(result, 0i32, Py_NewRef!(old));
        PyTuple_SET_ITEM!(result, 1i32, Py_NewRef!(new));
        Py_DECREF!(last_old);
        Py_DECREF!(last_new);
        // bpo-42536: The GC may have untracked this result tuple. Since we're
        //
        // recycling it, make sure it's tracked again:
        _PyTuple_Recycle(result);
    } else {
        result = unsafe { PyTuple_New(2i64) };
        if !result.is_null() {
            PyTuple_SET_ITEM!(result, 0i32, Py_NewRef!(old));
            PyTuple_SET_ITEM!(result, 1i32, Py_NewRef!(new));
        }
    }
    Py_XSETREF!((*po).old, new);
    Py_DECREF!(old);
    result
}

static mut pairwise_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(pairwise_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(pairwise_new__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(pairwise_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(pairwise_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 47i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
                >(PyType_GenericAlloc as *const ())
            })
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(pairwise_new))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut pairwise_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.pairwise\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_280>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(pairwise_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// groupby object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_437 {
    ob_base: _object,
    it: *mut _object,
    keyfunc: *mut _object,
    tgtkey: *mut _object,
    currkey: *mut _object,
    currvalue: *mut _object,
    /// borrowed reference
    currgrouper: *const (),
    state: *mut __SlateRecord_itertoolsmodule_c_40,
}

// [clinic input]
// @classmethod
// itertools.groupby.__new__
//
//     iterable as it: object
//         Elements to divide into groups according to the key function.
//     key as keyfunc: object = None
//         A function for computing the group category for each element.
//         If the key function is not specified or is None, the element itself
//         is used for grouping.
//
// make an iterator that returns consecutive keys and groups from the iterable
// [clinic start generated code]
/// [clinic end generated code: output=cbb1ae3a90fd4141 input=6b3d123e87ff65a1]
fn itertools_groupby_impl(
    r#type: *mut _typeobject,
    it: *mut _object,
    keyfunc: *mut _object,
) -> *mut _object {
    let gbo: *mut __SlateRecord_itertoolsmodule_c_437 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_437;
    if gbo.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*gbo).tgtkey = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*gbo).currkey = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*gbo).currvalue = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*gbo).keyfunc = Py_NewRef!(keyfunc);
    }
    unsafe {
        (*gbo).it = PyObject_GetIter(it);
    }
    if unsafe { (*gbo).it }.is_null() {
        Py_DECREF__2!(gbo);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*gbo).state = find_state_by_type(r#type);
    }
    gbo as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.groupby_dealloc")]
extern "C-unwind" fn groupby_dealloc(op: *mut _object) {
    let gbo: *mut __SlateRecord_itertoolsmodule_c_437 = groupbyobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(gbo);
    unsafe { PyObject_GC_UnTrack(gbo as *mut ()) };
    Py_XDECREF!(unsafe { (*gbo).it });
    Py_XDECREF!(unsafe { (*gbo).keyfunc });
    Py_XDECREF!(unsafe { (*gbo).tgtkey });
    Py_XDECREF!(unsafe { (*gbo).currkey });
    Py_XDECREF!(unsafe { (*gbo).currvalue });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(gbo as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.groupby_traverse")]
extern "C-unwind" fn groupby_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let gbo: *mut __SlateRecord_itertoolsmodule_c_437 = groupbyobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(gbo as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*gbo).it }, visit, arg);
    Py_VISIT!(unsafe { (*gbo).keyfunc }, visit, arg);
    Py_VISIT!(unsafe { (*gbo).tgtkey }, visit, arg);
    Py_VISIT!(unsafe { (*gbo).currkey }, visit, arg);
    Py_VISIT!(unsafe { (*gbo).currvalue }, visit, arg);
    0
}

fn groupby_step(gbo: *mut __SlateRecord_itertoolsmodule_c_437) -> i32 {
    let mut newkey: *mut _object = unsafe { core::mem::zeroed() };
    let newvalue: *mut _object = unsafe { PyIter_Next((*gbo).it) };
    if newvalue.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*gbo).keyfunc }) == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        newkey = Py_NewRef!(newvalue);
    } else {
        newkey = unsafe { PyObject_CallOneArg((*gbo).keyfunc, newvalue) };
        if newkey.is_null() {
            Py_DECREF!(newvalue);
            return 1i32.wrapping_neg();
        }
    }
    let oldvalue: *mut _object = unsafe { (*gbo).currvalue };
    unsafe {
        (*gbo).currvalue = newvalue;
    }
    Py_XSETREF!((*gbo).currkey, newkey);
    Py_XDECREF!(oldvalue);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.groupby_next")]
extern "C-unwind" fn groupby_next(op: *mut _object) -> *mut _object {
    let gbo: *mut __SlateRecord_itertoolsmodule_c_437 = groupbyobject_CAST!(op);
    unsafe {
        (*gbo).currgrouper = core::ptr::null::<()>();
    }
    // skip to next iteration group
    loop {
        if !unsafe { (*gbo).currkey }.is_null() {
            if unsafe { (*gbo).tgtkey }.is_null() {
                break;
            } else {
                // A user-defined __eq__ can re-enter groupby and advance the iterator,
                // mutating gbo->tgtkey / gbo->currkey while we are comparing them.
                // Take local snapshots and hold strong references so INCREF/DECREF
                // apply to the same objects even under re-entrancy.
                let tgtkey: *mut _object = unsafe { (*gbo).tgtkey };
                let currkey: *mut _object = unsafe { (*gbo).currkey };
                Py_INCREF!(tgtkey);
                Py_INCREF!(currkey);
                let rcmp: i32 = unsafe { PyObject_RichCompareBool(tgtkey, currkey, 2i32) };
                Py_DECREF!(tgtkey);
                Py_DECREF!(currkey);
                if rcmp == 1i32.wrapping_neg() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    if rcmp == 0 {
                        break;
                    }
                }
            }
        }
        // pass
        if groupby_step(gbo) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    Py_INCREF!(unsafe { (*gbo).currkey });
    Py_XSETREF!((*gbo).tgtkey, unsafe { (*gbo).currkey });
    let grouper: *mut _object = _grouper_create(gbo, unsafe { (*gbo).tgtkey });
    if grouper.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let r: *mut _object = unsafe { PyTuple_Pack(2i64, (*gbo).currkey, grouper) };
    Py_DECREF!(grouper);
    r
}

static mut groupby_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(groupby_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_groupby__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(groupby_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(groupby_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_groupby))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut groupby_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.groupby\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_437>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(groupby_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// _grouper object (internal)
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_607 {
    ob_base: _object,
    parent: *mut _object,
    tgtkey: *mut _object,
}

// [clinic input]
// @classmethod
// itertools._grouper.__new__
//
//     parent: object(subclass_of='clinic_state_by_cls()->groupby_type')
//     tgtkey: object
//     /
// [clinic start generated code]
/// [clinic end generated code: output=462efb1cdebb5914 input=afe05eb477118f12]
fn itertools__grouper_impl(
    r#type: *mut _typeobject,
    parent: *mut _object,
    tgtkey: *mut _object,
) -> *mut _object {
    _grouper_create(groupbyobject_CAST!(parent), tgtkey)
}

fn _grouper_create(
    parent: *mut __SlateRecord_itertoolsmodule_c_437,
    tgtkey: *mut _object,
) -> *mut _object {
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = unsafe { (*parent).state };
    let igo: *mut __SlateRecord_itertoolsmodule_c_607 =
        PyObject_GC_New!(__SlateRecord_itertoolsmodule_c_607, unsafe {
            (*state)._grouper_type
        });
    if igo.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*igo).parent = Py_NewRef__2!(parent);
    }
    unsafe {
        (*igo).tgtkey = Py_NewRef!(tgtkey);
    }
    unsafe {
        (*parent).currgrouper = igo as *const ();
    }
    // borrowed reference
    unsafe { PyObject_GC_Track(igo as *mut ()) };
    igo as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule._grouper_dealloc")]
extern "C-unwind" fn _grouper_dealloc(op: *mut _object) {
    let igo: *mut __SlateRecord_itertoolsmodule_c_607 = _grouperobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(igo);
    unsafe { PyObject_GC_UnTrack(igo as *mut ()) };
    Py_DECREF!(unsafe { (*igo).parent });
    Py_DECREF!(unsafe { (*igo).tgtkey });
    unsafe { PyObject_GC_Del(igo as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule._grouper_traverse")]
extern "C-unwind" fn _grouper_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let igo: *mut __SlateRecord_itertoolsmodule_c_607 = _grouperobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(igo as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*igo).parent }, visit, arg);
    Py_VISIT!(unsafe { (*igo).tgtkey }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule._grouper_next")]
extern "C-unwind" fn _grouper_next(op: *mut _object) -> *mut _object {
    let igo: *mut __SlateRecord_itertoolsmodule_c_607 = _grouperobject_CAST!(op);
    let gbo: *mut __SlateRecord_itertoolsmodule_c_437 =
        groupbyobject_CAST!(unsafe { (*igo).parent });
    let mut rcmp: i32 = 0;
    if (unsafe { (*gbo).currgrouper }) != (igo as *const ()) {
        return core::ptr::null_mut::<_object>();
    }
    if unsafe { (*gbo).currvalue }.is_null() {
        if groupby_step(gbo) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    // A user-defined __eq__ can re-enter the grouper and advance the iterator,
    // mutating gbo->currkey while we are comparing them.
    // Take local snapshots and hold strong references so INCREF/DECREF
    // apply to the same objects even under re-entrancy.
    let tgtkey: *mut _object = Py_NewRef!(unsafe { (*igo).tgtkey });
    let currkey: *mut _object = Py_NewRef!(unsafe { (*gbo).currkey });
    rcmp = unsafe { PyObject_RichCompareBool(tgtkey, currkey, 2i32) };
    Py_DECREF!(tgtkey);
    Py_DECREF!(currkey);
    if rcmp <= 0 {
        return core::ptr::null_mut::<_object>();
    }
    // got any error or current group is end
    let r: *mut _object = unsafe { (*gbo).currvalue };
    unsafe {
        (*gbo).currvalue = core::ptr::null_mut::<_object>();
    }
    Py_CLEAR!((*gbo).currkey);
    r
}

static mut _grouper_slots: __SlateAlign16<[__SlateRecord_object_h_377; 8]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(_grouper_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 71i32;
        __t2.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                        *mut (),
                    ) -> i32,
                >,
                *mut (),
            >(Some(_grouper_traverse))
        };
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 62i32;
        __t3.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 63i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(Some(_grouper_next))
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 65i32;
        __t5.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools__grouper))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 74i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
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

static mut _grouper_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools._grouper\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_607>() as u64) as i32;
    __t0.flags = 16640u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(_grouper_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// tee object and with supporting function and objects
// The teedataobject pre-allocates space for LINKCELLS number of objects.
// To help the object fit neatly inside cache lines (space for 16 to 32
// pointers), the value should be a multiple of 16 minus  space for
// the other structure members including PyHEAD overhead.  The larger the
// value, the less memory overhead per object and the less time spent
// allocating/deallocating new links.  The smaller the number, the less
// wasted space and the more rapid freeing of older data.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_741 {
    ob_base: _object,
    it: *mut _object,
    /// 0 <= numread <= LINKCELLS
    numread: i32,
    running: i32,
    nextlink: *mut _object,
    values: [*mut _object; 57],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_751 {
    ob_base: _object,
    dataobj: *mut __SlateRecord_itertoolsmodule_c_741,
    /// 0 <= index <= LINKCELLS
    index: i32,
    weakreflist: *mut _object,
    state: *mut __SlateRecord_itertoolsmodule_c_40,
}

fn teedataobject_newinternal(
    state: *mut __SlateRecord_itertoolsmodule_c_40,
    it: *mut _object,
) -> *mut _object {
    let tdo: *mut __SlateRecord_itertoolsmodule_c_741 =
        PyObject_GC_New!(__SlateRecord_itertoolsmodule_c_741, unsafe {
            (*state).teedataobject_type
        });
    if tdo.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*tdo).running = 0;
    }
    unsafe {
        (*tdo).numread = 0;
    }
    unsafe {
        (*tdo).nextlink = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*tdo).it = Py_NewRef!(it);
    }
    unsafe { PyObject_GC_Track(tdo as *mut ()) };
    tdo as *mut _object
}

use slate_support::link;

fn teedataobject_jumplink(
    state: *mut __SlateRecord_itertoolsmodule_c_40,
    tdo: *mut __SlateRecord_itertoolsmodule_c_741,
) -> *mut _object {
    if unsafe { (*tdo).nextlink }.is_null() {
        unsafe {
            (*tdo).nextlink = teedataobject_newinternal(state, (*tdo).it);
        }
    }
    let link: *mut _object = Py_XNewRef!(unsafe { (*tdo).nextlink });
    link
}

fn teedataobject_getitem_lock_held(
    tdo: *mut __SlateRecord_itertoolsmodule_c_741,
    i: i32,
) -> *mut _object {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if i < unsafe { (*tdo).numread } {
        value = unsafe {
            *((*tdo).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize)
        };
    } else {
        // this is the lead iterator, so fetch more data
        if (unsafe { (*tdo).running }) != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_RuntimeError,
                    c"cannot re-enter the tee iterator".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        unsafe {
            (*tdo).running = 1;
        }
        value = unsafe { PyIter_Next((*tdo).it) };
        unsafe {
            (*tdo).running = 0;
        }
        if value.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let __v1: i32 = unsafe { (*tdo).numread };
        unsafe {
            (*tdo).numread = __v1.wrapping_add(1);
        }
        unsafe {
            *((*tdo).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize) = value;
        }
    }
    Py_NewRef!(value)
}

fn teedataobject_getitem(tdo: *mut __SlateRecord_itertoolsmodule_c_741, i: i32) -> *mut _object {
    let result: *mut _object = teedataobject_getitem_lock_held(tdo, i);
    result
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.teedataobject_traverse")]
extern "C-unwind" fn teedataobject_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let mut i: i32 = 0;
    let tdo: *mut __SlateRecord_itertoolsmodule_c_741 = teedataobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(tdo as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*tdo).it }, visit, arg);
    while i < unsafe { (*tdo).numread } {
        Py_VISIT!(
            unsafe {
                *((*tdo).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize)
            },
            visit,
            arg
        );
        i = i.wrapping_add(1);
    }
    Py_VISIT!(unsafe { (*tdo).nextlink }, visit, arg);
    0
}

fn teedataobject_safe_decref(mut obj: *mut _object) {
    loop {
        let __v1: bool = !obj.is_null() && _PyObject_IsUniquelyReferenced(obj) != 0;
        if !__v1 {
            break;
        }
        let tmp: *mut __SlateRecord_itertoolsmodule_c_741 = teedataobject_CAST!(obj);
        let nextlink: *mut _object = unsafe { (*tmp).nextlink };
        unsafe {
            (*tmp).nextlink = core::ptr::null_mut::<_object>();
        }
        Py_SETREF!(obj, nextlink);
    }
    Py_XDECREF!(obj);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.teedataobject_clear")]
extern "C-unwind" fn teedataobject_clear(op: *mut _object) -> i32 {
    let mut i: i32 = 0;
    let tdo: *mut __SlateRecord_itertoolsmodule_c_741 = teedataobject_CAST!(op);
    Py_CLEAR!((*tdo).it);
    while i < unsafe { (*tdo).numread } {
        Py_CLEAR!(
            *unsafe { (*tdo).values.as_mut_ptr() as *mut *mut _object }.wrapping_offset(i as isize)
        );
        i = i.wrapping_add(1);
    }
    let tmp: *mut _object = unsafe { (*tdo).nextlink };
    unsafe {
        (*tdo).nextlink = core::ptr::null_mut::<_object>();
    }
    teedataobject_safe_decref(tmp);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.teedataobject_dealloc")]
extern "C-unwind" fn teedataobject_dealloc(op: *mut _object) {
    let tp: *mut _typeobject = Py_TYPE!(op);
    unsafe { PyObject_GC_UnTrack(op as *mut ()) };
    teedataobject_clear(op);
    unsafe { PyObject_GC_Del(op as *mut ()) };
    Py_DECREF__2!(tp);
}

// [clinic input]
// @classmethod
// itertools.teedataobject.__new__
//     iterable as it: object
//     values: object(subclass_of='&PyList_Type')
//     next: object
//     /
// Data container common to multiple tee objects.
// [clinic start generated code]
/// [clinic end generated code: output=3343ceb07e08df5e input=be60f2fabd2b72ba]
fn itertools_teedataobject_impl(
    r#type: *mut _typeobject,
    it: *mut _object,
    values: *mut _object,
    next: *mut _object,
) -> *mut _object {
    let mut state: *mut __SlateRecord_itertoolsmodule_c_40 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut tdo: *mut __SlateRecord_itertoolsmodule_c_741 = unsafe { core::mem::zeroed() };
    unsafe {
        state = get_module_state_by_cls(r#type);
        tdo = teedataobject_newinternal(state, it) as *mut __SlateRecord_itertoolsmodule_c_741;
        if tdo.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                len = PyList_GET_SIZE!(values);
                if len <= 57 {
                    i = 0;
                    while i < len {
                        *((*tdo).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(i as isize) = PyList_GET_ITEM!(values, i);
                        Py_INCREF!(unsafe {
                            *((*tdo).values.as_mut_ptr() as *mut *mut _object)
                                .wrapping_offset(i as isize)
                        });
                        i = i.wrapping_add(1);
                    }
                    // len <= LINKCELLS < INT_MAX
                    (*tdo).numread = Py_SAFE_DOWNCAST!(len, i32);
                    if len == 57 {
                        if next != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            if Py_IS_TYPE!(next, unsafe { (*state).teedataobject_type }) == 0 {
                                break '__join_0;
                            } else {
                                (*tdo).nextlink = Py_NewRef!(next);
                            }
                        }
                    } else {
                        if next != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            break '__join_0;
                        }
                        // shouldn't have a next if we are not full
                    }
                    return tdo as *mut _object;
                }
            }
            Py_XDECREF__2!(tdo);
            PyErr_SetString(PyExc_ValueError, c"Invalid arguments".as_ptr());
            return core::ptr::null_mut::<_object>();
        }
    }
}

static mut teedataobject_slots: __SlateAlign16<[__SlateRecord_object_h_377; 8]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(teedataobject_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(itertools_teedataobject__doc__.0) as *const i8 })
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
            >(Some(teedataobject_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 51i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(teedataobject_clear),
            )
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 65i32;
        __t5.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_teedataobject))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 74i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
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

static mut teedataobject_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools._tee_dataobject\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_741>() as u64) as i32;
    __t0.flags = 16640u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(teedataobject_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.tee_next")]
extern "C-unwind" fn tee_next(op: *mut _object) -> *mut _object {
    let to: *mut __SlateRecord_itertoolsmodule_c_751 = teeobject_CAST!(op);
    // The GIL already serializes access, so keep the simple path without the
    // snapshot and revalidation that the free-threaded build needs.
    if (unsafe { (*to).index }) >= 57 {
        let link: *mut _object =
            teedataobject_jumplink(unsafe { (*to).state }, unsafe { (*to).dataobj });
        if link.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        Py_SETREF__11!(
            (*to).dataobj,
            link as *mut __SlateRecord_itertoolsmodule_c_741
        );
        unsafe {
            (*to).index = 0;
        }
    }
    let value: *mut _object =
        teedataobject_getitem(unsafe { (*to).dataobj }, unsafe { (*to).index });
    if value.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let __v2: i32 = unsafe { (*to).index };
    unsafe {
        (*to).index = __v2.wrapping_add(1);
    }
    value
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.tee_traverse")]
extern "C-unwind" fn tee_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let to: *mut __SlateRecord_itertoolsmodule_c_751 = teeobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(to as *mut _object), visit, arg);
    Py_VISIT!((unsafe { (*to).dataobj }) as *mut _object, visit, arg);
    0
}

fn tee_copy_impl(
    to: *mut __SlateRecord_itertoolsmodule_c_751,
) -> *mut __SlateRecord_itertoolsmodule_c_751 {
    let newto: *mut __SlateRecord_itertoolsmodule_c_751 =
        PyObject_GC_New!(__SlateRecord_itertoolsmodule_c_751, Py_TYPE__2!(to));
    if newto.is_null() {
        return core::ptr::null_mut::<__SlateRecord_itertoolsmodule_c_751>();
    }
    unsafe {
        (*newto).dataobj =
            Py_NewRef__2!(unsafe { (*to).dataobj }) as *mut __SlateRecord_itertoolsmodule_c_741;
    }
    unsafe {
        (*newto).index = (*to).index;
    }
    unsafe {
        (*newto).weakreflist = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*newto).state = (*to).state;
    }
    unsafe { PyObject_GC_Track(newto as *mut ()) };
    newto
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.tee_copy")]
extern "C-unwind" fn tee_copy(op: *mut _object, _unused_ignored: *mut _object) -> *mut _object {
    let to: *mut __SlateRecord_itertoolsmodule_c_751 = teeobject_CAST!(op);
    tee_copy_impl(to) as *mut _object
}

static teecopy_doc: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 100i8,
    101i8, 112i8, 101i8, 110i8, 100i8, 101i8, 110i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    116i8, 111i8, 114i8, 46i8, 0i8,
]);

fn tee_fromiterable(
    state: *mut __SlateRecord_itertoolsmodule_c_40,
    iterable: *mut _object,
) -> *mut _object {
    let mut dataobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut to: *mut __SlateRecord_itertoolsmodule_c_751 = unsafe { core::mem::zeroed() };
    unsafe {
        it = PyObject_GetIter(iterable);
        if it.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            if PyObject_TypeCheck!(it, unsafe { (*state).tee_type }) != 0 {
                to = tee_copy_impl(it as *mut __SlateRecord_itertoolsmodule_c_751); // 'it' can be fast casted
            } else {
                dataobj = teedataobject_newinternal(state, it);
                if dataobj.is_null() {
                    to = core::ptr::null_mut::<__SlateRecord_itertoolsmodule_c_751>();
                } else {
                    to = PyObject_GC_New!(__SlateRecord_itertoolsmodule_c_751, unsafe {
                        (*state).tee_type
                    });
                    if to.is_null() {
                        Py_DECREF!(dataobj);
                    } else {
                        (*to).dataobj = dataobj as *mut __SlateRecord_itertoolsmodule_c_741;
                        (*to).index = 0;
                        (*to).weakreflist = core::ptr::null_mut::<_object>();
                        (*to).state = state;
                        PyObject_GC_Track(to as *mut ());
                    }
                }
            }
            Py_DECREF!(it);
            return to as *mut _object;
        }
    }
}

// [clinic input]
// @classmethod
// itertools._tee.__new__
//     iterable: object
//     /
// Iterator wrapped to make it copyable.
// [clinic start generated code]
/// [clinic end generated code: output=b02d3fd26c810c3f input=adc0779d2afe37a2]
fn itertools__tee_impl(r#type: *mut _typeobject, iterable: *mut _object) -> *mut _object {
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = get_module_state_by_cls(r#type);
    tee_fromiterable(state, iterable)
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.tee_clear")]
extern "C-unwind" fn tee_clear(op: *mut _object) -> i32 {
    let to: *mut __SlateRecord_itertoolsmodule_c_751 = teeobject_CAST!(op);
    if !unsafe { (*to).weakreflist }.is_null() {
        unsafe { PyObject_ClearWeakRefs(op) };
    }
    Py_CLEAR__28!((*to).dataobj);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.tee_dealloc")]
extern "C-unwind" fn tee_dealloc(op: *mut _object) {
    let tp: *mut _typeobject = Py_TYPE!(op);
    unsafe { PyObject_GC_UnTrack(op as *mut ()) };
    tee_clear(op);
    unsafe { PyObject_GC_Del(op as *mut ()) };
    Py_DECREF__2!(tp);
}

/// sentinel
static mut tee_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__copy__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(tee_copy);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(teecopy_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static mut tee_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 19i32;
        __t0.offset =
            (core::mem::offset_of!(__SlateRecord_itertoolsmodule_c_751, weakreflist) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut tee_slots: __SlateAlign16<[__SlateRecord_object_h_377; 11]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(tee_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 56i32;
        __t1.pfunc =
            (unsafe { core::ptr::addr_of!(itertools__tee__doc__.0) as *const i8 }) as *mut ();
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 71i32;
        __t2.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                        *mut (),
                    ) -> i32,
                >,
                *mut (),
            >(Some(tee_traverse))
        };
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 51i32;
        __t3.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(tee_clear),
            )
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(tee_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 64i32;
        __t6.pfunc =
            (unsafe { core::ptr::addr_of_mut!(tee_methods.0) as *mut PyMethodDef }) as *mut ();
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 72i32;
        __t7.pfunc =
            (unsafe { core::ptr::addr_of_mut!(tee_members.0) as *mut PyMemberDef }) as *mut ();
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 65i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools__tee))
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 74i32;
        __t9.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
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

static mut tee_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools._tee\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_751>() as u64) as i32;
    __t0.flags = 16640u32;
    __t0.slots = unsafe { core::ptr::addr_of_mut!(tee_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// [clinic input]
// itertools.tee
//     iterable: object
//     n: Py_ssize_t = 2
//     /
// Returns a tuple of n independent iterators.
// [clinic start generated code]
/// [clinic end generated code: output=1c64519cd859c2f0 input=c99a1472c425d66d]
fn itertools_tee_impl(module: *mut _object, iterable: *mut _object, n: i64) -> *mut _object {
    let mut to: *mut _object = unsafe { core::mem::zeroed() };
    if n < 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"n must be >= 0".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe { PyTuple_New(n) };
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if n == 0 {
        return result;
    }
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = get_module_state(module);
    to = tee_fromiterable(state, it);
    Py_DECREF!(it);
    if to.is_null() {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    PyTuple_SET_ITEM!(result, 0i32, to);
    for i in 1..n {
        to = tee_copy(to, core::ptr::null_mut::<_object>());
        if to.is_null() {
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(result, i, to);
    }
    result
}

// cycle object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_1214 {
    ob_base: _object,
    it: *mut _object,
    saved: *mut _object,
    index: i64,
}

// [clinic input]
// @classmethod
// itertools.cycle.__new__
//     iterable: object
//     /
// Return elements from the iterable until it is exhausted. Then repeat the sequence indefinitely.
// [clinic start generated code]
/// [clinic end generated code: output=f60e5ec17a45b35c input=9d1d84bcf66e908b]
fn itertools_cycle_impl(r#type: *mut _typeobject, iterable: *mut _object) -> *mut _object {
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let saved: *mut _object = unsafe { PyList_New(0i64) };
    if saved.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    // create cycleobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_1214 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_1214;
    if lz.is_null() {
        Py_DECREF!(it);
        Py_DECREF!(saved);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).it = it;
    }
    unsafe {
        (*lz).saved = saved;
    }
    unsafe {
        (*lz).index = 1i32.wrapping_neg() as i64;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cycle_dealloc")]
extern "C-unwind" fn cycle_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1214 = cycleobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).it });
    Py_XDECREF!(unsafe { (*lz).saved });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cycle_traverse")]
extern "C-unwind" fn cycle_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1214 = cycleobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).saved }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cycle_next")]
extern "C-unwind" fn cycle_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1214 = cycleobject_CAST!(op);
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut index: i64 = unsafe { (*lz).index };
    if index < 0 {
        item = unsafe { PyIter_Next((*lz).it) };
        if !item.is_null() {
            if (unsafe { PyList_Append((*lz).saved, item) }) != 0 {
                Py_DECREF!(item);
                return core::ptr::null_mut::<_object>();
            }
            return item;
        }
        // Note:  StopIteration is already cleared by PyIter_Next()
        if !unsafe { PyErr_Occurred() }.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        index = 0;
        FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*lz).index, 0i32);
        Py_CLEAR!((*lz).it);
    }
    if PyList_GET_SIZE!(unsafe { (*lz).saved }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    item = unsafe { PyList_GetItemRef((*lz).saved, index) };
    index = index.wrapping_add(1);
    if index >= PyList_GET_SIZE!(unsafe { (*lz).saved }) {
        index = 0;
    }
    FT_ATOMIC_STORE_SSIZE_RELAXED!((*lz).index, index);
    item
}

static mut cycle_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(cycle_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_cycle__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(cycle_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(cycle_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_cycle))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut cycle_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.cycle\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_1214>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(cycle_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// dropwhile object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_1347 {
    ob_base: _object,
    func: *mut _object,
    it: *mut _object,
    start: i64,
}

// [clinic input]
// @classmethod
// itertools.dropwhile.__new__
//     predicate as func: object
//     iterable as seq: object
//     /
// Drop items from the iterable while predicate(item) is true.
//
// Afterwards, return every element until the iterable is exhausted.
// [clinic start generated code]
/// [clinic end generated code: output=92f9d0d89af149e4 input=d39737147c9f0a26]
fn itertools_dropwhile_impl(
    r#type: *mut _typeobject,
    func: *mut _object,
    seq: *mut _object,
) -> *mut _object {
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(seq) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create dropwhileobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_1347 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_1347;
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
    unsafe {
        (*lz).start = 0;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.dropwhile_dealloc")]
extern "C-unwind" fn dropwhile_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1347 = dropwhileobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).func });
    Py_XDECREF!(unsafe { (*lz).it });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.dropwhile_traverse")]
extern "C-unwind" fn dropwhile_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1347 = dropwhileobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).func }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.dropwhile_next")]
extern "C-unwind" fn dropwhile_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1347 = dropwhileobject_CAST!(op);
    let it: *mut _object = unsafe { (*lz).it };
    let mut ok: i64 = 0;
    let iternext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(it)).tp_iternext };
    loop {
        let item: *mut _object = unsafe { iternext.unwrap()(it) };
        if item.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if (unsafe { (*lz).start }) == 1 {
            return item;
        }
        let good: *mut _object = unsafe { PyObject_CallOneArg((*lz).func, item) };
        if good.is_null() {
            Py_DECREF!(item);
            return core::ptr::null_mut::<_object>();
        }
        ok = (unsafe { PyObject_IsTrue(good) }) as i64;
        Py_DECREF!(good);
        if ok == 0 {
            unsafe {
                (*lz).start = 1;
            }
            return item;
        }
        Py_DECREF!(item);
        if ok < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::mem::zeroed() }
}

static mut dropwhile_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(dropwhile_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_dropwhile__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(dropwhile_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(dropwhile_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_dropwhile))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut dropwhile_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.dropwhile\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_1347>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(dropwhile_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// takewhile object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_1471 {
    ob_base: _object,
    func: *mut _object,
    it: *mut _object,
    stop: i64,
}

// [clinic input]
// @classmethod
// itertools.takewhile.__new__
//     predicate as func: object
//     iterable as seq: object
//     /
// Return successive entries from an iterable as long as the predicate evaluates to true for each entry.
// [clinic start generated code]
/// [clinic end generated code: output=bb179ea7864e2ef6 input=ba5255f7519aa119]
fn itertools_takewhile_impl(
    r#type: *mut _typeobject,
    func: *mut _object,
    seq: *mut _object,
) -> *mut _object {
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(seq) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create takewhileobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_1471 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_1471;
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
    unsafe {
        (*lz).stop = 0;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.takewhile_dealloc")]
extern "C-unwind" fn takewhile_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1471 = takewhileobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).func });
    Py_XDECREF!(unsafe { (*lz).it });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.takewhile_traverse")]
extern "C-unwind" fn takewhile_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1471 = takewhileobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).func }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.takewhile_next")]
extern "C-unwind" fn takewhile_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1471 = takewhileobject_CAST!(op);
    let it: *mut _object = unsafe { (*lz).it };
    let mut ok: i64 = 0;
    if (unsafe { (*lz).stop }) == 1 {
        return core::ptr::null_mut::<_object>();
    }
    let item: *mut _object = unsafe { unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it) };
    if item.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let good: *mut _object = unsafe { PyObject_CallOneArg((*lz).func, item) };
    if good.is_null() {
        Py_DECREF!(item);
        return core::ptr::null_mut::<_object>();
    }
    ok = (unsafe { PyObject_IsTrue(good) }) as i64;
    Py_DECREF!(good);
    if ok > 0 {
        return item;
    }
    Py_DECREF!(item);
    if ok == 0 {
        unsafe {
            (*lz).stop = 1;
        }
    }
    core::ptr::null_mut::<_object>()
}

static mut takewhile_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(takewhile_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_takewhile__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(takewhile_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(takewhile_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_takewhile))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut takewhile_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.takewhile\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_1471>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(takewhile_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// islice object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_1591 {
    ob_base: _object,
    it: *mut _object,
    next: i64,
    stop: i64,
    step: i64,
    cnt: i64,
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.islice_new")]
extern "C-unwind" fn islice_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut seq: *mut _object = unsafe { core::mem::zeroed() };
    let mut start: i64 = 0;
    let mut stop: i64 = 1i32.wrapping_neg() as i64;
    let mut step: i64 = 1;
    let mut a1: *mut _object = core::ptr::null_mut::<_object>();
    let mut a2: *mut _object = core::ptr::null_mut::<_object>();
    let mut a3: *mut _object = core::ptr::null_mut::<_object>();
    let mut numargs: i64 = 0;
    let st: *mut __SlateRecord_itertoolsmodule_c_40 = find_state_by_type(r#type);
    let islice_type: *mut _typeobject = unsafe { (*st).islice_type };
    let __v0: bool = (r#type == islice_type
        || (unsafe { (*r#type).tp_init }) == unsafe { (*islice_type).tp_init })
        && !_PyArg_NoKeywords!(c"islice".as_ptr() as *mut i8, kwds);
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe {
        PyArg_UnpackTuple(
            args,
            c"islice".as_ptr(),
            2i64,
            4i64,
            core::ptr::addr_of_mut!(seq),
            core::ptr::addr_of_mut!(a1),
            core::ptr::addr_of_mut!(a2),
            core::ptr::addr_of_mut!(a3),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    numargs = unsafe { PyTuple_Size(args) };
    if numargs == 2 {
        if a1 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            stop = unsafe { PyNumber_AsSsize_t(a1, PyExc_OverflowError) };
            if stop == (1i32.wrapping_neg() as i64) {
                if !unsafe { PyErr_Occurred() }.is_null() {
                    unsafe { PyErr_Clear() };
                }
                unsafe {
                    PyErr_SetString(PyExc_ValueError, c"Stop argument for islice() must be None or an integer: 0 <= x <= sys.maxsize.".as_ptr())
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    } else {
        if a1 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            start = unsafe { PyNumber_AsSsize_t(a1, PyExc_OverflowError) };
        }
        let __v2: bool =
            start == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v2 {
            unsafe { PyErr_Clear() };
        }
        if a2 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            stop = unsafe { PyNumber_AsSsize_t(a2, PyExc_OverflowError) };
            if stop == (1i32.wrapping_neg() as i64) {
                if !unsafe { PyErr_Occurred() }.is_null() {
                    unsafe { PyErr_Clear() };
                }
                unsafe {
                    PyErr_SetString(PyExc_ValueError, c"Stop argument for islice() must be None or an integer: 0 <= x <= sys.maxsize.".as_ptr())
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    if start < 0 || stop < (1i32.wrapping_neg() as i64) {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"Indices for islice() must be None or an integer: 0 <= x <= sys.maxsize.".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if !a3.is_null() {
        if a3 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            step = unsafe { PyNumber_AsSsize_t(a3, PyExc_OverflowError) };
        }
        let __v3: bool =
            step == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v3 {
            unsafe { PyErr_Clear() };
        }
    }
    if step < 1 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"Step for islice() must be a positive integer or None.".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(seq) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create isliceobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_1591 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_1591;
    if lz.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).it = it;
    }
    unsafe {
        (*lz).next = start;
    }
    unsafe {
        (*lz).stop = stop;
    }
    unsafe {
        (*lz).step = step;
    }
    unsafe {
        (*lz).cnt = 0;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.islice_dealloc")]
extern "C-unwind" fn islice_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1591 = isliceobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).it });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.islice_traverse")]
extern "C-unwind" fn islice_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1591 = isliceobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.islice_next")]
extern "C-unwind" fn islice_next(op: *mut _object) -> *mut _object {
    let mut iternext: core::mem::MaybeUninit<
        Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    > = core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        core::ptr::addr_of_mut!(iternext)
            as *mut Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>;
    let mut stop: i64 = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut lz: *mut __SlateRecord_itertoolsmodule_c_1591 = unsafe { core::mem::zeroed() };
    unsafe {
        lz = isliceobject_CAST!(op);
        it = (*lz).it;
        stop = (*lz).stop;
        if it.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            *__slate_slot_9 = (*Py_TYPE!(it)).tp_iternext;
            '__join_0: {
                while (*lz).cnt < (*lz).next {
                    item = (*__slate_slot_9).unwrap()(it);
                    if item.is_null() {
                        break '__join_0;
                    } else {
                        Py_DECREF!(item);
                        let __v7: i64 = (*lz).cnt;
                        (*lz).cnt = __v7.wrapping_add(1);
                    }
                }
                if !(stop != (1i32.wrapping_neg() as i64) && (*lz).cnt >= stop) {
                    item = (*__slate_slot_9).unwrap()(it);
                    if !item.is_null() {
                        let __v4: i64 = (*lz).cnt;
                        (*lz).cnt = __v4.wrapping_add(1);
                        let oldnext: i64 = (*lz).next;
                        // The (size_t) cast below avoids the danger of undefined
                        // behaviour from signed integer overflow.
                        (*lz).next = ((*lz).next as u64).wrapping_add((*lz).step as u64) as i64;
                        if (*lz).next < oldnext
                            || stop != (1i32.wrapping_neg() as i64) && (*lz).next > stop
                        {
                            (*lz).next = stop;
                        }
                        return item;
                    }
                }
            }
            Py_CLEAR!((*lz).it);
            return core::ptr::null_mut::<_object>();
        }
    }
}

static islice_doc: __SlateAlign16<[i8; 454]> = __SlateAlign16([
    105i8, 115i8, 108i8, 105i8, 99i8, 101i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 44i8, 32i8, 115i8, 116i8, 111i8, 112i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 105i8,
    115i8, 108i8, 105i8, 99i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 10i8, 105i8,
    115i8, 108i8, 105i8, 99i8, 101i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8,
    44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 44i8, 32i8, 115i8, 116i8, 111i8, 112i8, 91i8,
    44i8, 32i8, 115i8, 116i8, 101i8, 112i8, 93i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 105i8, 115i8,
    108i8, 105i8, 99i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    116i8, 111i8, 114i8, 32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 110i8, 101i8, 120i8, 116i8,
    40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 115i8, 32i8, 115i8, 101i8, 108i8, 101i8, 99i8, 116i8, 101i8, 100i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 110i8, 10i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8,
    115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 105i8, 115i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8,
    102i8, 105i8, 101i8, 100i8, 44i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 115i8, 107i8, 105i8,
    112i8, 32i8, 97i8, 108i8, 108i8, 32i8, 112i8, 114i8, 101i8, 99i8, 101i8, 100i8, 105i8, 110i8,
    103i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 59i8, 10i8, 111i8, 116i8,
    104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8,
    32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 122i8,
    101i8, 114i8, 111i8, 46i8, 32i8, 32i8, 83i8, 116i8, 101i8, 112i8, 32i8, 100i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 111i8, 110i8, 101i8, 46i8, 32i8,
    32i8, 73i8, 102i8, 10i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8,
    97i8, 115i8, 32i8, 97i8, 110i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 44i8, 32i8, 115i8, 116i8, 101i8, 112i8, 32i8, 100i8, 101i8, 116i8, 101i8, 114i8,
    109i8, 105i8, 110i8, 101i8, 115i8, 32i8, 104i8, 111i8, 119i8, 32i8, 109i8, 97i8, 110i8, 121i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 10i8, 115i8, 107i8,
    105i8, 112i8, 112i8, 101i8, 100i8, 32i8, 98i8, 101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 32i8,
    115i8, 117i8, 99i8, 99i8, 101i8, 115i8, 115i8, 105i8, 118i8, 101i8, 32i8, 99i8, 97i8, 108i8,
    108i8, 115i8, 46i8, 32i8, 32i8, 87i8, 111i8, 114i8, 107i8, 115i8, 32i8, 108i8, 105i8, 107i8,
    101i8, 32i8, 97i8, 32i8, 115i8, 108i8, 105i8, 99i8, 101i8, 40i8, 41i8, 32i8, 111i8, 110i8,
    32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8, 10i8, 98i8, 117i8, 116i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    116i8, 111i8, 114i8, 46i8, 0i8,
]);

static mut islice_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(islice_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(islice_doc.0) as *const i8 }) as *mut ();
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
            >(Some(islice_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(islice_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(islice_new))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut islice_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.islice\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_1591>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(islice_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// starmap object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_1780 {
    ob_base: _object,
    func: *mut _object,
    it: *mut _object,
}

// [clinic input]
// @classmethod
// itertools.starmap.__new__
//     function as func: object
//     iterable as seq: object
//     /
// Return an iterator whose values are returned from the function evaluated with an argument tuple taken from the given sequence.
// [clinic start generated code]
/// [clinic end generated code: output=79eeb81d452c6e8d input=844766df6a0d4dad]
fn itertools_starmap_impl(
    r#type: *mut _typeobject,
    func: *mut _object,
    seq: *mut _object,
) -> *mut _object {
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(seq) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create starmapobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_1780 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_1780;
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

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.starmap_dealloc")]
extern "C-unwind" fn starmap_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1780 = starmapobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).func });
    Py_XDECREF!(unsafe { (*lz).it });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.starmap_traverse")]
extern "C-unwind" fn starmap_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1780 = starmapobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).func }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.starmap_next")]
extern "C-unwind" fn starmap_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1780 = starmapobject_CAST!(op);
    let mut args: *mut _object = unsafe { core::mem::zeroed() };
    let it: *mut _object = unsafe { (*lz).it };
    args = unsafe { unsafe { (*Py_TYPE!(it)).tp_iternext }.unwrap()(it) };
    if args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyTuple_CheckExact!(args) == 0 {
        let newargs: *mut _object = unsafe { PySequence_Tuple(args) };
        Py_DECREF!(args);
        if newargs.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        args = newargs;
    }
    let result: *mut _object =
        unsafe { PyObject_Call((*lz).func, args, core::ptr::null_mut::<_object>()) };
    Py_DECREF!(args);
    result
}

static mut starmap_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(starmap_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_starmap__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(starmap_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(starmap_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_starmap))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut starmap_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.starmap\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_1780>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(starmap_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// chain object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_1889 {
    ob_base: _object,
    /// Iterator over input iterables
    source: *mut _object,
    /// Currently running input iterator
    active: *mut _object,
}

fn chain_new_internal(r#type: *mut _typeobject, source: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1889 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_1889;
    if lz.is_null() {
        Py_DECREF!(source);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).source = source;
    }
    unsafe {
        (*lz).active = core::ptr::null_mut::<_object>();
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.chain_new")]
extern "C-unwind" fn chain_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = find_state_by_type(r#type);
    let chain_type: *mut _typeobject = unsafe { (*state).chain_type };
    let __v0: bool = (r#type == chain_type
        || (unsafe { (*r#type).tp_init }) == unsafe { (*chain_type).tp_init })
        && !_PyArg_NoKeywords!(c"chain".as_ptr() as *mut i8, kwds);
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    let source: *mut _object = unsafe { PyObject_GetIter(args) };
    if source.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    chain_new_internal(r#type, source)
}

// [clinic input]
// @classmethod
// itertools.chain.from_iterable
//     iterable as arg: object
//     /
// Alternative chain() constructor taking a single iterable argument that evaluates lazily.
// [clinic start generated code]
/// [clinic end generated code: output=3d7ea7d46b9e43f5 input=72c39e3a2ca3be85]
fn itertools_chain_from_iterable_impl(r#type: *mut _typeobject, arg: *mut _object) -> *mut _object {
    let source: *mut _object = unsafe { PyObject_GetIter(arg) };
    if source.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    chain_new_internal(r#type, source)
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.chain_dealloc")]
extern "C-unwind" fn chain_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1889 = chainobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).active });
    Py_XDECREF!(unsafe { (*lz).source });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.chain_traverse")]
extern "C-unwind" fn chain_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1889 = chainobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).source }, visit, arg);
    Py_VISIT!(unsafe { (*lz).active }, visit, arg);
    0
}

fn chain_next_lock_held(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_1889 = chainobject_CAST!(op);
    // lz->source is the iterator of iterables. If it's NULL, we've already
    // consumed them all. lz->active is the current iterator. If it's NULL,
    // we should grab a new one from lz->source.
    while !unsafe { (*lz).source }.is_null() {
        if unsafe { (*lz).active }.is_null() {
            let iterable: *mut _object = unsafe { PyIter_Next((*lz).source) };
            if iterable.is_null() {
                Py_CLEAR!((*lz).source);
                return core::ptr::null_mut::<_object>(); // no more input sources
            }
            unsafe {
                (*lz).active = PyObject_GetIter(iterable);
            }
            Py_DECREF!(iterable);
            if unsafe { (*lz).active }.is_null() {
                Py_CLEAR!((*lz).source);
                return core::ptr::null_mut::<_object>(); // input not iterable
            }
        }
        let item: *mut _object = unsafe {
            unsafe { (*Py_TYPE!(unsafe { (*lz).active })).tp_iternext }.unwrap()((*lz).active)
        };
        if !item.is_null() {
            return item;
        }
        if !unsafe { PyErr_Occurred() }.is_null() {
            if (unsafe { PyErr_ExceptionMatches(PyExc_StopIteration) }) != 0 {
                unsafe { PyErr_Clear() };
            } else {
                return core::ptr::null_mut::<_object>();
            }
            // input raised an exception
        }
        // lz->active is consumed, try with the next iterable.
        Py_CLEAR!((*lz).active);
    }
    // Everything had been consumed already.
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.chain_next")]
extern "C-unwind" fn chain_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = chain_next_lock_held(op);
    result
}

static chain_doc: __SlateAlign16<[i8; 213]> = __SlateAlign16([
    99i8, 104i8, 97i8, 105i8, 110i8, 40i8, 42i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 97i8, 32i8, 99i8, 104i8, 97i8, 105i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 46i8, 95i8, 95i8, 110i8, 101i8, 120i8,
    116i8, 95i8, 95i8, 40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8,
    116i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 10i8, 102i8, 105i8,
    114i8, 115i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 117i8,
    110i8, 116i8, 105i8, 108i8, 32i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8, 104i8,
    97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 44i8, 32i8, 116i8, 104i8, 101i8, 110i8, 32i8, 101i8,
    108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 110i8, 101i8, 120i8, 116i8, 10i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 44i8, 32i8, 117i8, 110i8, 116i8, 105i8, 108i8, 32i8, 97i8, 108i8, 108i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 101i8, 120i8, 104i8, 97i8, 117i8, 115i8, 116i8,
    101i8, 100i8, 46i8, 0i8,
]);

static chain_class_getitem_doc: __SlateAlign16<[i8; 111]> = __SlateAlign16([
    99i8, 104i8, 97i8, 105i8, 110i8, 32i8, 105i8, 115i8, 32i8, 103i8, 101i8, 110i8, 101i8, 114i8,
    105i8, 99i8, 32i8, 111i8, 118i8, 101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 121i8,
    112i8, 101i8, 32i8, 111i8, 102i8, 32i8, 105i8, 116i8, 115i8, 32i8, 99i8, 111i8, 110i8, 116i8,
    101i8, 110i8, 116i8, 115i8, 46i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 101i8, 110i8, 116i8, 115i8, 46i8, 0i8,
]);

/// sentinel
static mut chain_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"from_iterable\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(itertools_chain_from_iterable);
        __t0.ml_flags = 24i32;
        __t0.ml_doc =
            unsafe { core::ptr::addr_of!(itertools_chain_from_iterable__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__class_getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(Py_GenericAlias as *const ())
        };
        __t1.ml_flags = 24i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(chain_class_getitem_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

static mut chain_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(chain_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(chain_doc.0) as *const i8 }) as *mut ();
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
            >(Some(chain_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(chain_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 64i32;
        __t6.pfunc =
            (unsafe { core::ptr::addr_of_mut!(chain_methods.0) as *mut PyMethodDef }) as *mut ();
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(chain_new))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut chain_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.chain\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_1889>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(chain_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// product object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_2068 {
    ob_base: _object,
    /// tuple of pool tuples
    pools: *mut _object,
    /// one index per pool
    indices: *mut i64,
    /// most recently returned result tuple
    result: *mut _object,
    /// set to 1 when the iterator is exhausted
    stopped: i32,
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.product_new")]
extern "C-unwind" fn product_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut pool_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut tmpargs: *mut _object = unsafe { core::mem::zeroed() };
    let mut kwlist: core::mem::MaybeUninit<__SlateAlign16<[*mut i8; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut [*mut i8; 2] = core::ptr::addr_of_mut!(kwlist) as *mut [*mut i8; 2];
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pools: *mut _object = unsafe { core::mem::zeroed() };
    let mut repeat: i64 = unsafe { core::mem::zeroed() };
    let mut npools: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut lz: *mut __SlateRecord_itertoolsmodule_c_2068 = unsafe { core::mem::zeroed() };
    unsafe {
        repeat = 1;
        pools = core::ptr::null_mut::<_object>();
        indices = core::ptr::null_mut::<i64>();
        if !kwds.is_null() {
            core::ptr::write(
                __slate_slot_8,
                [c"repeat".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()],
            );
            tmpargs = PyTuple_New(0i64);
            if tmpargs.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                if PyArg_ParseTupleAndKeywords(
                    tmpargs,
                    kwds,
                    c"|n:product".as_ptr(),
                    (*__slate_slot_8).as_mut_ptr() as *const *mut i8,
                    core::ptr::addr_of_mut!(repeat),
                ) == 0
                {
                    Py_DECREF!(tmpargs);
                    return core::ptr::null_mut::<_object>();
                } else {
                    Py_DECREF!(tmpargs);
                    if repeat < 0 {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"repeat argument cannot be negative".as_ptr(),
                        );
                        return core::ptr::null_mut::<_object>();
                    }
                }
            }
        }
        if repeat == 0 {
            nargs = 0;
        } else {
            nargs = PyTuple_GET_SIZE!(args);
            if (nargs as u64)
                > 9223372036854775807 / (core::mem::size_of::<i64>() as u64) / (repeat as u64)
            {
                PyErr_SetString(PyExc_OverflowError, c"repeat argument too large".as_ptr());
                return core::ptr::null_mut::<_object>();
            }
        }
        '__join_2: {
            npools = nargs.wrapping_mul(repeat);
            indices = PyMem_New!(i64, npools);
            if indices.is_null() {
                PyErr_NoMemory();
            } else {
                pools = PyTuple_New(npools);
                if !pools.is_null() {
                    i = 0;
                    while i < nargs {
                        let item: *mut _object = PyTuple_GET_ITEM!(args, i);
                        pool_1 = PySequence_Tuple(item);
                        if pool_1.is_null() {
                            break '__join_2;
                        } else {
                            PyTuple_SET_ITEM__2!(pools, i, pool_1);
                            *indices.wrapping_offset(i as isize) = 0;
                            i = i.wrapping_add(1);
                        }
                    }
                    while i < npools {
                        let pool: *mut _object = PyTuple_GET_ITEM!(pools, i.wrapping_sub(nargs));
                        Py_INCREF!(pool);
                        PyTuple_SET_ITEM__2!(pools, i, pool);
                        *indices.wrapping_offset(i as isize) = 0;
                        i = i.wrapping_add(1);
                    }
                    // create productobject structure
                    lz = unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64)
                        as *mut __SlateRecord_itertoolsmodule_c_2068;
                    if !lz.is_null() {
                        (*lz).pools = pools;
                        (*lz).indices = indices;
                        (*lz).result = core::ptr::null_mut::<_object>();
                        (*lz).stopped = 0;
                        return lz as *mut _object;
                    }
                }
            }
        }
        if !indices.is_null() {
            PyMem_Free(indices as *mut ());
        }
        Py_XDECREF!(pools);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.product_dealloc")]
extern "C-unwind" fn product_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_2068 = productobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).pools });
    Py_XDECREF!(unsafe { (*lz).result });
    unsafe { PyMem_Free((*lz).indices as *mut ()) };
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.product_sizeof")]
extern "C-unwind" fn product_sizeof(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_2068 = productobject_CAST!(op);
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(lz));
    let __v0: u64 = (PyTuple_GET_SIZE!(unsafe { (*lz).pools }) as u64)
        .wrapping_mul(core::mem::size_of::<i64>() as u64);
    res = res.wrapping_add(__v0);
    unsafe { PyLong_FromSize_t(res) }
}

static sizeof_doc: __SlateAlign16<[i8; 34]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 105i8,
    110i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 44i8, 32i8, 105i8, 110i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.product_traverse")]
extern "C-unwind" fn product_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_2068 = productobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).pools }, visit, arg);
    Py_VISIT!(unsafe { (*lz).result }, visit, arg);
    0
}

fn product_next_lock_held(op: *mut _object) -> *mut _object {
    let mut old_result: *mut _object = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut npools: i64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut pools: *mut _object = unsafe { core::mem::zeroed() };
    let mut oldelem: *mut _object = unsafe { core::mem::zeroed() };
    let mut elem: *mut _object = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut lz: *mut __SlateRecord_itertoolsmodule_c_2068 = unsafe { core::mem::zeroed() };
    unsafe {
        lz = productobject_CAST!(op);
        pools = (*lz).pools;
        result = (*lz).result;
        npools = PyTuple_GET_SIZE!(pools);
        if (*lz).stopped != 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if result.is_null() {
                    // On the first pass, return an initial tuple filled with the
                    // first element from each pool.
                    result = PyTuple_New(npools);
                    if result.is_null() {
                        break '__join_0;
                    } else {
                        (*lz).result = result;
                        i = 0;
                        while i < npools {
                            pool = PyTuple_GET_ITEM!(pools, i);
                            if PyTuple_GET_SIZE!(pool) == 0 {
                                break '__join_0;
                            } else {
                                elem = PyTuple_GET_ITEM!(pool, 0i32);
                                Py_INCREF!(elem);
                                PyTuple_SET_ITEM__2!(result, i, elem);
                                i = i.wrapping_add(1);
                            }
                        }
                    }
                } else {
                    indices = (*lz).indices;
                    // Copy the previous result tuple or re-use it if available
                    if _PyObject_IsUniquelyReferenced(result) == 0 {
                        old_result = result;
                        result = _PyTuple_FromArray(
                            _PyTuple_ITEMS!(old_result) as *const *mut _object,
                            npools,
                        );
                        if result.is_null() {
                            break '__join_0;
                        } else {
                            (*lz).result = result;
                            Py_DECREF!(old_result);
                        }
                    } else {
                        _PyTuple_Recycle(result);
                    }
                    // bpo-42536: The GC may have untracked this result tuple. Since we're
                    //
                    // recycling it, make sure it's tracked again:
                    // Now, we've got the only copy so we can update it in-place
                    // Update the pool indices right-to-left.  Only advance to the
                    // next pool when the previous one rolls-over
                    i = npools.wrapping_sub(1);
                    '__join_7: {
                        loop {
                            if i >= 0 {
                                pool = PyTuple_GET_ITEM!(pools, i);
                                let __v6: *mut i64 = indices.wrapping_offset(i as isize);
                                let __v5: i64 = *__v6;
                                *__v6 = __v5.wrapping_add(1);
                                if *indices.wrapping_offset(i as isize) == PyTuple_GET_SIZE!(pool) {
                                    // Roll-over and advance to next pool
                                    *indices.wrapping_offset(i as isize) = 0;
                                    elem = PyTuple_GET_ITEM!(pool, 0i32);
                                    Py_INCREF!(elem);
                                    oldelem = PyTuple_GET_ITEM!(result, i);
                                    PyTuple_SET_ITEM__2!(result, i, elem);
                                    Py_DECREF!(oldelem);
                                    i = i.wrapping_sub(1);
                                } else {
                                    break;
                                }
                            } else {
                                break '__join_7;
                            }
                        }
                        // No rollover. Just increment and stop here.
                        elem = PyTuple_GET_ITEM!(pool, unsafe {
                            *indices.wrapping_offset(i as isize)
                        });
                        Py_INCREF!(elem);
                        oldelem = PyTuple_GET_ITEM!(result, i);
                        PyTuple_SET_ITEM__2!(result, i, elem);
                        Py_DECREF!(oldelem);
                    }
                    // If i is negative, then the indices have all rolled-over
                    // and we're done.
                    if i < 0 {
                        break '__join_0;
                    }
                }
                return Py_NewRef!(result);
            }
            (*lz).stopped = 1;
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.product_next")]
extern "C-unwind" fn product_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = product_next_lock_held(op);
    result
}

/// sentinel
static mut product_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(product_sizeof);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(sizeof_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static product_doc: __SlateAlign16<[i8; 716]> = __SlateAlign16([
    112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 40i8, 42i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    98i8, 108i8, 101i8, 115i8, 44i8, 32i8, 114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 61i8, 49i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 97i8, 114i8, 116i8, 101i8, 115i8, 105i8, 97i8, 110i8,
    32i8, 112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 32i8, 111i8, 102i8, 32i8, 105i8, 110i8,
    112i8, 117i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 115i8, 46i8,
    32i8, 32i8, 69i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8, 32i8, 116i8,
    111i8, 32i8, 110i8, 101i8, 115i8, 116i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 45i8, 108i8,
    111i8, 111i8, 112i8, 115i8, 46i8, 10i8, 10i8, 70i8, 111i8, 114i8, 32i8, 101i8, 120i8, 97i8,
    109i8, 112i8, 108i8, 101i8, 44i8, 32i8, 112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 40i8,
    65i8, 44i8, 32i8, 66i8, 41i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 97i8, 115i8, 58i8, 32i8, 32i8,
    40i8, 40i8, 120i8, 44i8, 121i8, 41i8, 32i8, 102i8, 111i8, 114i8, 32i8, 120i8, 32i8, 105i8,
    110i8, 32i8, 65i8, 32i8, 102i8, 111i8, 114i8, 32i8, 121i8, 32i8, 105i8, 110i8, 32i8, 66i8,
    41i8, 46i8, 10i8, 84i8, 104i8, 101i8, 32i8, 108i8, 101i8, 102i8, 116i8, 109i8, 111i8, 115i8,
    116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 32i8, 97i8, 114i8,
    101i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 117i8, 116i8, 101i8, 114i8,
    109i8, 111i8, 115i8, 116i8, 32i8, 102i8, 111i8, 114i8, 45i8, 108i8, 111i8, 111i8, 112i8, 44i8,
    32i8, 115i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 117i8, 116i8, 112i8, 117i8, 116i8,
    32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8, 10i8, 99i8, 121i8, 99i8, 108i8, 101i8, 32i8,
    105i8, 110i8, 32i8, 97i8, 32i8, 109i8, 97i8, 110i8, 110i8, 101i8, 114i8, 32i8, 115i8, 105i8,
    109i8, 105i8, 108i8, 97i8, 114i8, 32i8, 116i8, 111i8, 32i8, 97i8, 110i8, 32i8, 111i8, 100i8,
    111i8, 109i8, 101i8, 116i8, 101i8, 114i8, 32i8, 40i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 114i8, 105i8, 103i8, 104i8, 116i8, 109i8, 111i8, 115i8, 116i8, 32i8, 101i8,
    108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 99i8, 104i8, 97i8, 110i8, 103i8, 105i8, 110i8,
    103i8, 10i8, 111i8, 110i8, 32i8, 101i8, 118i8, 101i8, 114i8, 121i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 116i8, 105i8, 111i8, 110i8, 41i8, 46i8, 10i8, 10i8, 84i8, 111i8, 32i8, 99i8,
    111i8, 109i8, 112i8, 117i8, 116i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 111i8,
    100i8, 117i8, 99i8, 116i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8,
    114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 105i8, 116i8, 115i8,
    101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 121i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 10i8, 111i8, 102i8, 32i8, 114i8,
    101i8, 112i8, 101i8, 116i8, 105i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 119i8, 105i8,
    116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    108i8, 32i8, 114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 32i8, 107i8, 101i8, 121i8, 119i8, 111i8,
    114i8, 100i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 32i8, 70i8,
    111i8, 114i8, 32i8, 101i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 44i8, 10i8, 112i8, 114i8,
    111i8, 100i8, 117i8, 99i8, 116i8, 40i8, 65i8, 44i8, 32i8, 114i8, 101i8, 112i8, 101i8, 97i8,
    116i8, 61i8, 52i8, 41i8, 32i8, 109i8, 101i8, 97i8, 110i8, 115i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 97i8, 115i8, 32i8, 112i8, 114i8, 111i8, 100i8, 117i8,
    99i8, 116i8, 40i8, 65i8, 44i8, 32i8, 65i8, 44i8, 32i8, 65i8, 44i8, 32i8, 65i8, 41i8, 46i8,
    10i8, 10i8, 112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 40i8, 39i8, 97i8, 98i8, 39i8, 44i8,
    32i8, 114i8, 97i8, 110i8, 103i8, 101i8, 40i8, 51i8, 41i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8,
    40i8, 39i8, 97i8, 39i8, 44i8, 48i8, 41i8, 32i8, 40i8, 39i8, 97i8, 39i8, 44i8, 49i8, 41i8, 32i8,
    40i8, 39i8, 97i8, 39i8, 44i8, 50i8, 41i8, 32i8, 40i8, 39i8, 98i8, 39i8, 44i8, 48i8, 41i8, 32i8,
    40i8, 39i8, 98i8, 39i8, 44i8, 49i8, 41i8, 32i8, 40i8, 39i8, 98i8, 39i8, 44i8, 50i8, 41i8, 10i8,
    112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 40i8, 40i8, 48i8, 44i8, 49i8, 41i8, 44i8, 32i8,
    40i8, 48i8, 44i8, 49i8, 41i8, 44i8, 32i8, 40i8, 48i8, 44i8, 49i8, 41i8, 41i8, 32i8, 45i8, 45i8,
    62i8, 32i8, 40i8, 48i8, 44i8, 48i8, 44i8, 48i8, 41i8, 32i8, 40i8, 48i8, 44i8, 48i8, 44i8, 49i8,
    41i8, 32i8, 40i8, 48i8, 44i8, 49i8, 44i8, 48i8, 41i8, 32i8, 40i8, 48i8, 44i8, 49i8, 44i8, 49i8,
    41i8, 32i8, 40i8, 49i8, 44i8, 48i8, 44i8, 48i8, 41i8, 32i8, 46i8, 46i8, 46i8, 0i8,
]);

static mut product_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(product_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(product_doc.0) as *const i8 }) as *mut ();
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
            >(Some(product_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(product_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 64i32;
        __t6.pfunc =
            (unsafe { core::ptr::addr_of_mut!(product_methods.0) as *mut PyMethodDef }) as *mut ();
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(product_new))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut product_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.product\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_2068>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(product_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// combinations object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_2337 {
    ob_base: _object,
    /// input converted to a tuple
    pool: *mut _object,
    /// one index per result element
    indices: *mut i64,
    /// most recently returned result tuple
    result: *mut _object,
    /// size of result tuple
    r: i64,
    /// set to 1 when the iterator is exhausted
    stopped: i32,
}

// [clinic input]
// @classmethod
// itertools.combinations.__new__
//     iterable: object
//     r: Py_ssize_t
// Return successive r-length combinations of elements in the iterable.
//
// combinations(range(4), 3) --> (0,1,2), (0,1,3), (0,2,3), (1,2,3)
// [clinic start generated code]
/// [clinic end generated code: output=87a689b39c40039c input=06bede09e3da20f8]
fn itertools_combinations_impl(
    r#type: *mut _typeobject,
    iterable: *mut _object,
    r: i64,
) -> *mut _object {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut co: *mut __SlateRecord_itertoolsmodule_c_2337 = unsafe { core::mem::zeroed() };
    unsafe {
        pool = core::ptr::null_mut::<_object>();
        indices = core::ptr::null_mut::<i64>();
        pool = PySequence_Tuple(iterable);
        if !pool.is_null() {
            n = PyTuple_GET_SIZE!(pool);
            if r < 0 {
                PyErr_SetString(PyExc_ValueError, c"r must be non-negative".as_ptr());
            } else {
                indices = PyMem_New!(i64, r);
                if indices.is_null() {
                    PyErr_NoMemory();
                } else {
                    i = 0;
                    while i < r {
                        *indices.wrapping_offset(i as isize) = i;
                        i = i.wrapping_add(1);
                    }
                    // create combinationsobject structure
                    co = unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64)
                        as *mut __SlateRecord_itertoolsmodule_c_2337;
                    if !co.is_null() {
                        (*co).pool = pool;
                        (*co).indices = indices;
                        (*co).result = core::ptr::null_mut::<_object>();
                        (*co).r = r;
                        (*co).stopped = if r > n { 1 } else { 0 };
                        return co as *mut _object;
                    }
                }
            }
        }
        if !indices.is_null() {
            PyMem_Free(indices as *mut ());
        }
        Py_XDECREF!(pool);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.combinations_dealloc")]
extern "C-unwind" fn combinations_dealloc(op: *mut _object) {
    let co: *mut __SlateRecord_itertoolsmodule_c_2337 = combinationsobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(co);
    unsafe { PyObject_GC_UnTrack(co as *mut ()) };
    Py_XDECREF!(unsafe { (*co).pool });
    Py_XDECREF!(unsafe { (*co).result });
    unsafe { PyMem_Free((*co).indices as *mut ()) };
    unsafe { unsafe { (*tp).tp_free }.unwrap()(co as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.combinations_sizeof")]
extern "C-unwind" fn combinations_sizeof(
    op: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    let co: *mut __SlateRecord_itertoolsmodule_c_2337 = combinationsobject_CAST!(op);
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(co));
    res = res.wrapping_add(
        ((unsafe { (*co).r }) as u64).wrapping_mul(core::mem::size_of::<i64>() as u64),
    );
    unsafe { PyLong_FromSize_t(res) }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.combinations_traverse")]
extern "C-unwind" fn combinations_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let co: *mut __SlateRecord_itertoolsmodule_c_2337 = combinationsobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(co as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*co).pool }, visit, arg);
    Py_VISIT!(unsafe { (*co).result }, visit, arg);
    0
}

fn combinations_next_lock_held(op: *mut _object) -> *mut _object {
    let mut old_result: *mut _object = unsafe { core::mem::zeroed() };
    let mut index: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut r: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut elem: *mut _object = unsafe { core::mem::zeroed() };
    let mut co: *mut __SlateRecord_itertoolsmodule_c_2337 = unsafe { core::mem::zeroed() };
    unsafe {
        co = combinationsobject_CAST!(op);
        pool = (*co).pool;
        indices = (*co).indices;
        result = (*co).result;
        n = PyTuple_GET_SIZE!(pool);
        r = (*co).r;
        if (*co).stopped != 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if result.is_null() {
                    // On the first pass, initialize result tuple using the indices
                    result = PyTuple_New(r);
                    if result.is_null() {
                        break '__join_0;
                    } else {
                        (*co).result = result;
                        i = 0;
                        while i < r {
                            index = *indices.wrapping_offset(i as isize);
                            elem = PyTuple_GET_ITEM!(pool, index);
                            Py_INCREF!(elem);
                            PyTuple_SET_ITEM__2!(result, i, elem);
                            i = i.wrapping_add(1);
                        }
                    }
                } else {
                    // Copy the previous result tuple or re-use it if available
                    if _PyObject_IsUniquelyReferenced(result) == 0 {
                        old_result = result;
                        result = _PyTuple_FromArray(
                            _PyTuple_ITEMS!(old_result) as *const *mut _object,
                            r,
                        );
                        if result.is_null() {
                            break '__join_0;
                        } else {
                            (*co).result = result;
                            Py_DECREF!(old_result);
                        }
                    } else {
                        _PyTuple_Recycle(result);
                    }
                    // bpo-42536: The GC may have untracked this result tuple. Since we're
                    //
                    // recycling it, make sure it's tracked again:
                    // Now, we've got the only copy so we can update it in-place
                    // CPython's empty tuple is a singleton and cached in
                    // PyTuple's freelist.
                    // Scan indices right-to-left until finding one that is not
                    // at its maximum (i + n - r).
                    i = r.wrapping_sub(1);
                    while i >= 0
                        && *indices.wrapping_offset(i as isize) == i.wrapping_add(n).wrapping_sub(r)
                    {
                        i = i.wrapping_sub(1);
                    }
                    // If i is negative, then the indices are all at
                    // their maximum value and we're done.
                    if i < 0 {
                        break '__join_0;
                    } else {
                        // Increment the current index which we know is not at its
                        // maximum.  Then move back to the right setting each index
                        // to its lowest possible value (one higher than the index
                        // to its left -- this maintains the sort order invariant).
                        let __v8: *mut i64 = indices.wrapping_offset(i as isize);
                        let __v7: i64 = *__v8;
                        *__v8 = __v7.wrapping_add(1);
                        j = i.wrapping_add(1);
                        while j < r {
                            *indices.wrapping_offset(j as isize) =
                                unsafe { *indices.wrapping_offset(j.wrapping_sub(1) as isize) }
                                    .wrapping_add(1);
                            j = j.wrapping_add(1);
                        }
                        // Update the result tuple for the new indices
                        // starting with i, the leftmost index that changed
                        while i < r {
                            index = *indices.wrapping_offset(i as isize);
                            elem = PyTuple_GET_ITEM!(pool, index);
                            Py_INCREF!(elem);
                            let oldelem: *mut _object = PyTuple_GET_ITEM!(result, i);
                            PyTuple_SET_ITEM__2!(result, i, elem);
                            Py_DECREF!(oldelem);
                            i = i.wrapping_add(1);
                        }
                    }
                }
                return Py_NewRef!(result);
            }
            (*co).stopped = 1;
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.combinations_next")]
extern "C-unwind" fn combinations_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = combinations_next_lock_held(op);
    result
}

/// sentinel
static mut combinations_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(combinations_sizeof);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(sizeof_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static mut combinations_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(combinations_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(itertools_combinations__doc__.0) as *const i8 })
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
            >(Some(combinations_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(combinations_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 64i32;
        __t6.pfunc =
            (unsafe { core::ptr::addr_of_mut!(combinations_methods.0) as *mut PyMethodDef })
                as *mut ();
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_combinations))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut combinations_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.combinations\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_2337>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(combinations_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// combinations with replacement object
/// Equivalent to:
///
/// def combinations_with_replacement(iterable, r):
///     "combinations_with_replacement('ABC', 2) --> AA AB AC BB BC CC"
///     # number items returned:  (n+r-1)! / r! / (n-1)!
///     pool = tuple(iterable)
///     n = len(pool)
///     indices = [0] * r
///     yield tuple(pool[i] for i in indices)
///     while 1:
///         for i in reversed(range(r)):
///             if indices[i] != n - 1:
///                 break
///         else:
///             return
///         indices[i:] = [indices[i] + 1] * (r - i)
///         yield tuple(pool[i] for i in indices)
///
/// def combinations_with_replacement2(iterable, r):
///     'Alternate version that filters from product()'
///     pool = tuple(iterable)
///     n = len(pool)
///     for indices in product(range(n), repeat=r):
///         if sorted(indices) == list(indices):
///             yield tuple(pool[i] for i in indices)
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_2590 {
    ob_base: _object,
    /// input converted to a tuple
    pool: *mut _object,
    /// one index per result element
    indices: *mut i64,
    /// most recently returned result tuple
    result: *mut _object,
    /// size of result tuple
    r: i64,
    /// set to 1 when the cwr iterator is exhausted
    stopped: i32,
}

// [clinic input]
// @classmethod
// itertools.combinations_with_replacement.__new__
//     iterable: object
//     r: Py_ssize_t
// Return successive r-length combinations of elements in the iterable allowing individual elements to have successive repeats.
//
// combinations_with_replacement('ABC', 2) --> ('A','A'), ('A','B'), ('A','C'), ('B','B'), ('B','C'), ('C','C')
// [clinic start generated code]
/// [clinic end generated code: output=48b26856d4e659ca input=1dc58e82a0878fdc]
fn itertools_combinations_with_replacement_impl(
    r#type: *mut _typeobject,
    iterable: *mut _object,
    r: i64,
) -> *mut _object {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut co: *mut __SlateRecord_itertoolsmodule_c_2590 = unsafe { core::mem::zeroed() };
    unsafe {
        pool = core::ptr::null_mut::<_object>();
        indices = core::ptr::null_mut::<i64>();
        pool = PySequence_Tuple(iterable);
        if !pool.is_null() {
            n = PyTuple_GET_SIZE!(pool);
            if r < 0 {
                PyErr_SetString(PyExc_ValueError, c"r must be non-negative".as_ptr());
            } else {
                indices = PyMem_New!(i64, r);
                if indices.is_null() {
                    PyErr_NoMemory();
                } else {
                    i = 0;
                    while i < r {
                        *indices.wrapping_offset(i as isize) = 0;
                        i = i.wrapping_add(1);
                    }
                    // create cwrobject structure
                    co = unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64)
                        as *mut __SlateRecord_itertoolsmodule_c_2590;
                    if !co.is_null() {
                        (*co).pool = pool;
                        (*co).indices = indices;
                        (*co).result = core::ptr::null_mut::<_object>();
                        (*co).r = r;
                        (*co).stopped = (n == 0 && r != 0) as i32;
                        return co as *mut _object;
                    }
                }
            }
        }
        if !indices.is_null() {
            PyMem_Free(indices as *mut ());
        }
        Py_XDECREF!(pool);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cwr_dealloc")]
extern "C-unwind" fn cwr_dealloc(op: *mut _object) {
    let co: *mut __SlateRecord_itertoolsmodule_c_2590 = cwrobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(co);
    unsafe { PyObject_GC_UnTrack(co as *mut ()) };
    Py_XDECREF!(unsafe { (*co).pool });
    Py_XDECREF!(unsafe { (*co).result });
    unsafe { PyMem_Free((*co).indices as *mut ()) };
    unsafe { unsafe { (*tp).tp_free }.unwrap()(co as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cwr_sizeof")]
extern "C-unwind" fn cwr_sizeof(op: *mut _object, _unused_args: *mut _object) -> *mut _object {
    let co: *mut __SlateRecord_itertoolsmodule_c_2590 = cwrobject_CAST!(op);
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(co));
    res = res.wrapping_add(
        ((unsafe { (*co).r }) as u64).wrapping_mul(core::mem::size_of::<i64>() as u64),
    );
    unsafe { PyLong_FromSize_t(res) }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cwr_traverse")]
extern "C-unwind" fn cwr_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let co: *mut __SlateRecord_itertoolsmodule_c_2590 = cwrobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(co as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*co).pool }, visit, arg);
    Py_VISIT!(unsafe { (*co).result }, visit, arg);
    0
}

fn cwr_next_lock_held(op: *mut _object) -> *mut _object {
    let mut old_result: *mut _object = unsafe { core::mem::zeroed() };
    let mut index: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut r: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut elem: *mut _object = unsafe { core::mem::zeroed() };
    let mut co: *mut __SlateRecord_itertoolsmodule_c_2590 = unsafe { core::mem::zeroed() };
    unsafe {
        co = cwrobject_CAST!(op);
        pool = (*co).pool;
        indices = (*co).indices;
        result = (*co).result;
        n = PyTuple_GET_SIZE!(pool);
        r = (*co).r;
        if (*co).stopped != 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if result.is_null() {
                    // On the first pass, initialize result tuple with pool[0]
                    result = PyTuple_New(r);
                    if result.is_null() {
                        break '__join_0;
                    } else {
                        (*co).result = result;
                        if n > 0 {
                            elem = PyTuple_GET_ITEM!(pool, 0i32);
                            i = 0;
                            while i < r {
                                Py_INCREF!(elem);
                                PyTuple_SET_ITEM__2!(result, i, elem);
                                i = i.wrapping_add(1);
                            }
                        }
                    }
                } else {
                    // Copy the previous result tuple or re-use it if available
                    if _PyObject_IsUniquelyReferenced(result) == 0 {
                        old_result = result;
                        result = _PyTuple_FromArray(
                            _PyTuple_ITEMS!(old_result) as *const *mut _object,
                            r,
                        );
                        if result.is_null() {
                            break '__join_0;
                        } else {
                            (*co).result = result;
                            Py_DECREF!(old_result);
                        }
                    } else {
                        _PyTuple_Recycle(result);
                    }
                    // bpo-42536: The GC may have untracked this result tuple. Since we're
                    //
                    // recycling it, make sure it's tracked again:
                    // Now, we've got the only copy so we can update it in-place CPython's
                    // empty tuple is a singleton and cached in PyTuple's freelist.
                    // Scan indices right-to-left until finding one that is not
                    // at its maximum (n-1).
                    i = r.wrapping_sub(1);
                    while i >= 0 && *indices.wrapping_offset(i as isize) == n.wrapping_sub(1) {
                        i = i.wrapping_sub(1);
                    }
                    // If i is negative, then the indices are all at
                    // their maximum value and we're done.
                    if i < 0 {
                        break '__join_0;
                    } else {
                        // Increment the current index which we know is not at its
                        // maximum.  Then set all to the right to the same value.
                        index = unsafe { *indices.wrapping_offset(i as isize) }.wrapping_add(1);
                        elem = PyTuple_GET_ITEM!(pool, index);
                        while i < r {
                            *indices.wrapping_offset(i as isize) = index;
                            Py_INCREF!(elem);
                            let oldelem: *mut _object = PyTuple_GET_ITEM!(result, i);
                            PyTuple_SET_ITEM__2!(result, i, elem);
                            Py_DECREF!(oldelem);
                            i = i.wrapping_add(1);
                        }
                    }
                }
                return Py_NewRef!(result);
            }
            (*co).stopped = 1;
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.cwr_next")]
extern "C-unwind" fn cwr_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = cwr_next_lock_held(op);
    result
}

/// sentinel
static mut cwr_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(cwr_sizeof);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(sizeof_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static mut cwr_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(cwr_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe {
            core::ptr::addr_of!(itertools_combinations_with_replacement__doc__.0) as *const i8
        }) as *mut ();
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
            >(Some(cwr_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(cwr_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 64i32;
        __t6.pfunc =
            (unsafe { core::ptr::addr_of_mut!(cwr_methods.0) as *mut PyMethodDef }) as *mut ();
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_combinations_with_replacement))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut cwr_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.combinations_with_replacement\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_2590>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots = unsafe { core::ptr::addr_of_mut!(cwr_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// permutations object ********************************************************
//
// def permutations(iterable, r=None):
//     # permutations('ABCD', 2) --> AB AC AD BA BC BD CA CB CD DA DB DC
//     # permutations(range(3)) --> 012 021 102 120 201 210
//     pool = tuple(iterable)
//     n = len(pool)
//     r = n if r is None else r
//     if r > n:
//         return
//     indices = list(range(n))
//     cycles = list(range(n, n-r, -1))
//     yield tuple(pool[i] for i in indices[:r])
//     while n:
//         for i in reversed(range(r)):
//             cycles[i] -= 1
//             if cycles[i] == 0:
//                 indices[i:] = indices[i+1:] + indices[i:i+1]
//                 cycles[i] = n - i
//             else:
//                 j = cycles[i]
//                 indices[i], indices[-j] = indices[-j], indices[i]
//                 yield tuple(pool[i] for i in indices[:r])
//                 break
//         else:
//             return
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_2839 {
    ob_base: _object,
    /// input converted to a tuple
    pool: *mut _object,
    /// one index per element in the pool
    indices: *mut i64,
    /// one rollover counter per element in the result
    cycles: *mut i64,
    /// most recently returned result tuple
    result: *mut _object,
    /// size of result tuple
    r: i64,
    /// set to 1 when the iterator is exhausted
    stopped: i32,
}

// [clinic input]
// @classmethod
// itertools.permutations.__new__
//     iterable: object
//     r as robj: object = None
// Return successive r-length permutations of elements in the iterable.
//
// permutations(range(3), 2) --> (0,1), (0,2), (1,0), (1,2), (2,0), (2,1)
// [clinic start generated code]
/// [clinic end generated code: output=296a72fa76d620ea input=57d0170a4ac0ec7a]
fn itertools_permutations_impl(
    r#type: *mut _typeobject,
    iterable: *mut _object,
    robj: *mut _object,
) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut cycles: *mut i64 = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut r: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut po: *mut __SlateRecord_itertoolsmodule_c_2839 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_4: {
            pool = core::ptr::null_mut::<_object>();
            indices = core::ptr::null_mut::<i64>();
            cycles = core::ptr::null_mut::<i64>();
            pool = PySequence_Tuple(iterable);
            if !pool.is_null() {
                n = PyTuple_GET_SIZE!(pool);
                r = n;
                if robj != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    if PyLong_Check!(robj) == 0 {
                        PyErr_SetString(PyExc_TypeError, c"Expected int as r".as_ptr());
                        break '__join_4;
                    } else {
                        r = PyLong_AsSsize_t(robj);
                        if r == (1i32.wrapping_neg() as i64) {
                            __v4 = !PyErr_Occurred().is_null();
                        } else {
                            __v4 = false;
                        }
                        if __v4 {
                            break '__join_4;
                        }
                    }
                }
                if r < 0 {
                    PyErr_SetString(PyExc_ValueError, c"r must be non-negative".as_ptr());
                } else {
                    indices = PyMem_New!(i64, n);
                    cycles = PyMem_New!(i64, r);
                    if indices.is_null() || cycles.is_null() {
                        PyErr_NoMemory();
                    } else {
                        i = 0;
                        while i < n {
                            *indices.wrapping_offset(i as isize) = i;
                            i = i.wrapping_add(1);
                        }
                        i = 0;
                        while i < r {
                            *cycles.wrapping_offset(i as isize) = n.wrapping_sub(i);
                            i = i.wrapping_add(1);
                        }
                        // create permutationsobject structure
                        po = unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64)
                            as *mut __SlateRecord_itertoolsmodule_c_2839;
                        if !po.is_null() {
                            (*po).pool = pool;
                            (*po).indices = indices;
                            (*po).cycles = cycles;
                            (*po).result = core::ptr::null_mut::<_object>();
                            (*po).r = r;
                            (*po).stopped = if r > n { 1 } else { 0 };
                            return po as *mut _object;
                        }
                    }
                }
            }
        }
        if !indices.is_null() {
            PyMem_Free(indices as *mut ());
        }
        if !cycles.is_null() {
            PyMem_Free(cycles as *mut ());
        }
        Py_XDECREF!(pool);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.permutations_dealloc")]
extern "C-unwind" fn permutations_dealloc(op: *mut _object) {
    let po: *mut __SlateRecord_itertoolsmodule_c_2839 = permutationsobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(po);
    unsafe { PyObject_GC_UnTrack(po as *mut ()) };
    Py_XDECREF!(unsafe { (*po).pool });
    Py_XDECREF!(unsafe { (*po).result });
    unsafe { PyMem_Free((*po).indices as *mut ()) };
    unsafe { PyMem_Free((*po).cycles as *mut ()) };
    unsafe { unsafe { (*tp).tp_free }.unwrap()(po as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.permutations_sizeof")]
extern "C-unwind" fn permutations_sizeof(
    op: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    let po: *mut __SlateRecord_itertoolsmodule_c_2839 = permutationsobject_CAST!(op);
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(po));
    let __v0: u64 = (PyTuple_GET_SIZE!(unsafe { (*po).pool }) as u64)
        .wrapping_mul(core::mem::size_of::<i64>() as u64);
    res = res.wrapping_add(__v0);
    res = res.wrapping_add(
        ((unsafe { (*po).r }) as u64).wrapping_mul(core::mem::size_of::<i64>() as u64),
    );
    unsafe { PyLong_FromSize_t(res) }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.permutations_traverse")]
extern "C-unwind" fn permutations_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let po: *mut __SlateRecord_itertoolsmodule_c_2839 = permutationsobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(po as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*po).pool }, visit, arg);
    Py_VISIT!(unsafe { (*po).result }, visit, arg);
    0
}

fn permutations_next_lock_held(op: *mut _object) -> *mut _object {
    let mut old_result: *mut _object = unsafe { core::mem::zeroed() };
    let mut index: i64 = unsafe { core::mem::zeroed() };
    let mut k: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut r: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut cycles: *mut i64 = unsafe { core::mem::zeroed() };
    let mut indices: *mut i64 = unsafe { core::mem::zeroed() };
    let mut pool: *mut _object = unsafe { core::mem::zeroed() };
    let mut elem: *mut _object = unsafe { core::mem::zeroed() };
    let mut po: *mut __SlateRecord_itertoolsmodule_c_2839 = unsafe { core::mem::zeroed() };
    unsafe {
        po = permutationsobject_CAST!(op);
        pool = (*po).pool;
        indices = (*po).indices;
        cycles = (*po).cycles;
        result = (*po).result;
        n = PyTuple_GET_SIZE!(pool);
        r = (*po).r;
        if (*po).stopped != 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if result.is_null() {
                    // On the first pass, initialize result tuple using the indices
                    result = PyTuple_New(r);
                    if result.is_null() {
                        break '__join_0;
                    } else {
                        (*po).result = result;
                        i = 0;
                        while i < r {
                            index = *indices.wrapping_offset(i as isize);
                            elem = PyTuple_GET_ITEM!(pool, index);
                            Py_INCREF!(elem);
                            PyTuple_SET_ITEM__2!(result, i, elem);
                            i = i.wrapping_add(1);
                        }
                    }
                } else {
                    if n == 0 {
                        break '__join_0;
                    } else {
                        // Copy the previous result tuple or re-use it if available
                        if _PyObject_IsUniquelyReferenced(result) == 0 {
                            old_result = result;
                            result = _PyTuple_FromArray(
                                _PyTuple_ITEMS!(old_result) as *const *mut _object,
                                r,
                            );
                            if result.is_null() {
                                break '__join_0;
                            } else {
                                (*po).result = result;
                                Py_DECREF!(old_result);
                            }
                        } else {
                            _PyTuple_Recycle(result);
                        }
                        // bpo-42536: The GC may have untracked this result tuple. Since we're
                        //
                        // recycling it, make sure it's tracked again:
                        // Now, we've got the only copy so we can update it in-place
                        // Decrement rightmost cycle, moving leftward upon zero rollover
                        i = r.wrapping_sub(1);
                        '__join_6: {
                            loop {
                                if i >= 0 {
                                    let __v10: *mut i64 = cycles.wrapping_offset(i as isize);
                                    let __v9: i64 = *__v10;
                                    *__v10 = __v9.wrapping_sub(1);
                                    if *cycles.wrapping_offset(i as isize) == 0 {
                                        // rotatation: indices[i:] = indices[i+1:] + indices[i:i+1]
                                        index = *indices.wrapping_offset(i as isize);
                                        j = i;
                                        while j < n.wrapping_sub(1) {
                                            *indices.wrapping_offset(j as isize) = *indices
                                                .wrapping_offset(j.wrapping_add(1) as isize);
                                            j = j.wrapping_add(1);
                                        }
                                        *indices.wrapping_offset(n.wrapping_sub(1) as isize) =
                                            index;
                                        *cycles.wrapping_offset(i as isize) = n.wrapping_sub(i);
                                        i = i.wrapping_sub(1);
                                    } else {
                                        break;
                                    }
                                } else {
                                    break '__join_6;
                                }
                            }
                            j = *cycles.wrapping_offset(i as isize);
                            index = *indices.wrapping_offset(i as isize);
                            *indices.wrapping_offset(i as isize) =
                                *indices.wrapping_offset(n.wrapping_sub(j) as isize);
                            *indices.wrapping_offset(n.wrapping_sub(j) as isize) = index;
                            k = i;
                            while k < r {
                                // start with i, the leftmost element that changed
                                //
                                // yield tuple(pool[k] for k in indices[:r])
                                index = *indices.wrapping_offset(k as isize);
                                elem = PyTuple_GET_ITEM!(pool, index);
                                Py_INCREF!(elem);
                                let oldelem: *mut _object = PyTuple_GET_ITEM!(result, k);
                                PyTuple_SET_ITEM__2!(result, k, elem);
                                Py_DECREF!(oldelem);
                                k = k.wrapping_add(1);
                            }
                        }
                        // If i is negative, then the cycles have all
                        // rolled-over and we're done.
                        if i < 0 {
                            break '__join_0;
                        }
                    }
                }
                return Py_NewRef!(result);
            }
            (*po).stopped = 1;
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.permutations_next")]
extern "C-unwind" fn permutations_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = permutations_next_lock_held(op);
    result
}

/// sentinel
static mut permuations_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(permutations_sizeof);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(sizeof_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static mut permutations_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(permutations_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(itertools_permutations__doc__.0) as *const i8 })
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
            >(Some(permutations_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(permutations_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 64i32;
        __t6.pfunc = (unsafe { core::ptr::addr_of_mut!(permuations_methods.0) as *mut PyMethodDef })
            as *mut ();
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_permutations))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut permutations_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.permutations\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_2839>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(permutations_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// accumulate object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_3092 {
    ob_base: _object,
    total: *mut _object,
    it: *mut _object,
    binop: *mut _object,
    initial: *mut _object,
    state: *mut __SlateRecord_itertoolsmodule_c_40,
}

// [clinic input]
// @classmethod
// itertools.accumulate.__new__
//     iterable: object
//     func as binop: object = None
//     *
//     initial: object = None
// Return series of accumulated sums (or other binary function results).
// [clinic start generated code]
/// [clinic end generated code: output=66da2650627128f8 input=c4ce20ac59bf7ffd]
fn itertools_accumulate_impl(
    r#type: *mut _typeobject,
    iterable: *mut _object,
    binop: *mut _object,
    initial: *mut _object,
) -> *mut _object {
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(iterable) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create accumulateobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_3092 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_3092;
    if lz.is_null() {
        Py_DECREF!(it);
        return core::ptr::null_mut::<_object>();
    }
    if binop != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        unsafe {
            (*lz).binop = Py_XNewRef!(binop);
        }
    }
    unsafe {
        (*lz).total = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).it = it;
    }
    unsafe {
        (*lz).initial = Py_XNewRef!(initial);
    }
    unsafe {
        (*lz).state = find_state_by_type(r#type);
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.accumulate_dealloc")]
extern "C-unwind" fn accumulate_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3092 = accumulateobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).binop });
    Py_XDECREF!(unsafe { (*lz).total });
    Py_XDECREF!(unsafe { (*lz).it });
    Py_XDECREF!(unsafe { (*lz).initial });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.accumulate_traverse")]
extern "C-unwind" fn accumulate_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3092 = accumulateobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).binop }, visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).total }, visit, arg);
    Py_VISIT!(unsafe { (*lz).initial }, visit, arg);
    0
}

fn accumulate_next_lock_held(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3092 = accumulateobject_CAST!(op);
    let mut newtotal: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { (*lz).initial }) != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        unsafe {
            (*lz).total = (*lz).initial;
        }
        unsafe {
            (*lz).initial = Py_NewRef!(core::ptr::addr_of_mut!(_Py_NoneStruct));
        }
        return Py_NewRef!(unsafe { (*lz).total });
    }
    let val: *mut _object =
        unsafe { unsafe { (*Py_TYPE!(unsafe { (*lz).it })).tp_iternext }.unwrap()((*lz).it) };
    if val.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if unsafe { (*lz).total }.is_null() {
        unsafe {
            (*lz).total = Py_NewRef!(val);
        }
        return unsafe { (*lz).total };
    }
    if unsafe { (*lz).binop }.is_null() {
        newtotal = unsafe { PyNumber_Add((*lz).total, val) };
    } else {
        newtotal = unsafe {
            PyObject_CallFunctionObjArgs((*lz).binop, (*lz).total, val, core::ptr::null_mut::<()>())
        };
    }
    Py_DECREF!(val);
    if newtotal.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_INCREF!(newtotal);
    Py_SETREF__2!((*lz).total, newtotal);
    newtotal
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.accumulate_next")]
extern "C-unwind" fn accumulate_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = accumulate_next_lock_held(op);
    result
}

static mut accumulate_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(accumulate_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_accumulate__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(accumulate_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(accumulate_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_accumulate))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut accumulate_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.accumulate\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_3092>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(accumulate_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// compress object
// Equivalent to:
//
// def compress(data, selectors):
//     "compress('ABCDEF', [1,0,1,0,1,1]) --> A C E F"
//     return (d for d, s in zip(data, selectors) if s)
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_3239 {
    ob_base: _object,
    data: *mut _object,
    selectors: *mut _object,
}

// [clinic input]
// @classmethod
// itertools.compress.__new__
//     data as seq1: object
//     selectors as seq2: object
// Return data elements corresponding to true selector elements.
//
// Forms a shorter iterator from selected data elements using the selectors
// to choose the data elements.
// [clinic start generated code]
/// [clinic end generated code: output=7e67157212ed09e0 input=32ca4347dbc46749]
fn itertools_compress_impl(
    r#type: *mut _typeobject,
    seq1: *mut _object,
    seq2: *mut _object,
) -> *mut _object {
    let mut lz: *mut __SlateRecord_itertoolsmodule_c_3239 = unsafe { core::mem::zeroed() };
    let mut selectors: *mut _object = unsafe { core::mem::zeroed() };
    let mut data: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        data = core::ptr::null_mut::<_object>();
        selectors = core::ptr::null_mut::<_object>();
        data = PyObject_GetIter(seq1);
        if !data.is_null() {
            selectors = PyObject_GetIter(seq2);
            if !selectors.is_null() {
                // create compressobject structure
                lz = unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64)
                    as *mut __SlateRecord_itertoolsmodule_c_3239;
                if !lz.is_null() {
                    (*lz).data = data;
                    (*lz).selectors = selectors;
                    return lz as *mut _object;
                }
            }
        }
        Py_XDECREF!(data);
        Py_XDECREF!(selectors);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.compress_dealloc")]
extern "C-unwind" fn compress_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3239 = compressobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).data });
    Py_XDECREF!(unsafe { (*lz).selectors });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.compress_traverse")]
extern "C-unwind" fn compress_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3239 = compressobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).data }, visit, arg);
    Py_VISIT!(unsafe { (*lz).selectors }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.compress_next")]
extern "C-unwind" fn compress_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3239 = compressobject_CAST!(op);
    let data: *mut _object = unsafe { (*lz).data };
    let selectors: *mut _object = unsafe { (*lz).selectors };
    let datanext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(data)).tp_iternext };
    let selectornext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(selectors)).tp_iternext };
    let mut ok: i32 = 0;
    while true {
        // Steps:  get datum, get selector, evaluate selector.
        // Order is important (to match the pure python version
        // in terms of which input gets a chance to raise an
        // exception first).
        let datum: *mut _object = unsafe { datanext.unwrap()(data) };
        if datum.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let selector: *mut _object = unsafe { selectornext.unwrap()(selectors) };
        if selector.is_null() {
            Py_DECREF!(datum);
            return core::ptr::null_mut::<_object>();
        }
        ok = unsafe { PyObject_IsTrue(selector) };
        Py_DECREF!(selector);
        if ok > 0 {
            return datum;
        }
        Py_DECREF!(datum);
        if ok < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::mem::zeroed() }
}

static mut compress_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(compress_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_compress__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(compress_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(compress_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_compress))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut compress_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.compress\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_3239>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(compress_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// filterfalse object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_3368 {
    ob_base: _object,
    func: *mut _object,
    it: *mut _object,
}

// [clinic input]
// @classmethod
// itertools.filterfalse.__new__
//     function as func: object
//     iterable as seq: object
//     /
// Return those items of iterable for which function(item) is false.
//
// If function is None, return the items that are false.
// [clinic start generated code]
/// [clinic end generated code: output=55f87eab9fc0484e input=2d684a2c66f99cde]
fn itertools_filterfalse_impl(
    r#type: *mut _typeobject,
    func: *mut _object,
    seq: *mut _object,
) -> *mut _object {
    // Get iterator.
    let it: *mut _object = unsafe { PyObject_GetIter(seq) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // create filterfalseobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_3368 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_3368;
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

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.filterfalse_dealloc")]
extern "C-unwind" fn filterfalse_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3368 = filterfalseobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).func });
    Py_XDECREF!(unsafe { (*lz).it });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.filterfalse_traverse")]
extern "C-unwind" fn filterfalse_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3368 = filterfalseobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).it }, visit, arg);
    Py_VISIT!(unsafe { (*lz).func }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.filterfalse_next")]
extern "C-unwind" fn filterfalse_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3368 = filterfalseobject_CAST!(op);
    let it: *mut _object = unsafe { (*lz).it };
    let mut ok: i64 = 0;
    let iternext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        unsafe { (*Py_TYPE!(it)).tp_iternext };
    loop {
        let item: *mut _object = unsafe { iternext.unwrap()(it) };
        if item.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if (unsafe { (*lz).func }) == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
            || (unsafe { (*lz).func })
                == ((unsafe { core::ptr::addr_of_mut!(PyBool_Type) }) as *mut _object)
        {
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
        if ok == 0 {
            return item;
        }
        Py_DECREF!(item);
        if ok < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::mem::zeroed() }
}

static mut filterfalse_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(filterfalse_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(itertools_filterfalse__doc__.0) as *const i8 })
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
            >(Some(filterfalse_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(filterfalse_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_filterfalse))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut filterfalse_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.filterfalse\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_3368>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(filterfalse_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// count object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_3492 {
    ob_base: _object,
    cnt: i64,
    long_cnt: *mut _object,
    long_step: *mut _object,
}

// Counting logic and invariants:
//
// fast_mode:  when cnt an integer < PY_SSIZE_T_MAX and no step is specified.
//
//     assert(long_cnt == NULL && long_step==PyLong(1));
//     Advances with:  cnt += 1
//     When count hits PY_SSIZE_T_MAX, switch to slow_mode.
//
// slow_mode:  when cnt == PY_SSIZE_T_MAX, step is not int(1), or cnt is a float.
//
//     assert(cnt == PY_SSIZE_T_MAX && long_cnt != NULL && long_step != NULL);
//     All counting is done with python objects (no overflows or underflows).
//     Advances with:  long_cnt += long_step
//     Step may be zero -- effectively a slow version of repeat(cnt).
//     Either long_cnt or long_step may be a float, Fraction, or Decimal.
// [clinic input]
// @classmethod
// itertools.count.__new__
//     start as long_cnt: object(c_default="NULL") = 0
//     step as long_step: object(c_default="NULL") = 1
// Return a count object whose .__next__() method returns consecutive values.
//
// Equivalent to:
//     def count(firstval=0, step=1):
//         x = firstval
//         while 1:
//             yield x
//             x += step
// [clinic start generated code]
/// [clinic end generated code: output=09a9250aebd00b1c input=d7a85eec18bfcd94]
fn itertools_count_impl(
    r#type: *mut _typeobject,
    mut long_cnt: *mut _object,
    mut long_step: *mut _object,
) -> *mut _object {
    let mut fast_mode: i32 = 0;
    let mut cnt: i64 = 0;
    let mut step: i64 = 0;
    let __v0: bool = !long_cnt.is_null() && (unsafe { PyNumber_Check(long_cnt) }) == 0;
    let __v1: bool = __v0 || !long_step.is_null() && (unsafe { PyNumber_Check(long_step) }) == 0;
    if __v1 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"a number is required".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let __v3: bool = long_cnt.is_null() || PyLong_Check!(long_cnt) != 0;
    let __v4: bool = __v3 && (long_step.is_null() || PyLong_Check!(long_step) != 0);
    fast_mode = __v4 as i32;
    // If not specified, start defaults to 0
    if !long_cnt.is_null() {
        if fast_mode != 0 {
            cnt = unsafe { PyLong_AsSsize_t(long_cnt) };
            let __v6: bool =
                cnt == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v6 {
                unsafe { PyErr_Clear() };
                fast_mode = 0;
            }
        }
    } else {
        cnt = 0;
        long_cnt = _PyLong_GetZero();
    }
    Py_INCREF!(long_cnt);
    // If not specified, step defaults to 1
    if long_step.is_null() {
        long_step = _PyLong_GetOne();
    }
    Py_INCREF!(long_step);
    // Fast mode only works when the step is 1
    if fast_mode != 0 {
        step = unsafe { PyLong_AsLong(long_step) };
        if step != 1 {
            fast_mode = 0;
            let __v7: bool =
                step == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v7 {
                unsafe { PyErr_Clear() };
            }
        }
    }
    if fast_mode != 0 {
        Py_CLEAR__2!(long_cnt);
    } else {
        cnt = 9223372036854775807;
    }
    // create countobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_3492 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_3492;
    if lz.is_null() {
        Py_XDECREF!(long_cnt);
        Py_DECREF!(long_step);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).cnt = cnt;
    }
    unsafe {
        (*lz).long_cnt = long_cnt;
    }
    unsafe {
        (*lz).long_step = long_step;
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.count_dealloc")]
extern "C-unwind" fn count_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3492 = countobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).long_cnt });
    Py_XDECREF!(unsafe { (*lz).long_step });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.count_traverse")]
extern "C-unwind" fn count_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3492 = countobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).long_cnt }, visit, arg);
    Py_VISIT!(unsafe { (*lz).long_step }, visit, arg);
    0
}

fn count_nextlong(lz: *mut __SlateRecord_itertoolsmodule_c_3492) -> *mut _object {
    let mut long_cnt: *mut _object = unsafe { core::mem::zeroed() };
    long_cnt = unsafe { (*lz).long_cnt };
    if long_cnt.is_null() {
        // Switch to slow_mode
        long_cnt = unsafe { PyLong_FromSsize_t(9223372036854775807i64) };
        if long_cnt.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    }
    let stepped_up: *mut _object = unsafe { PyNumber_Add(long_cnt, (*lz).long_step) };
    if stepped_up.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*lz).long_cnt = stepped_up;
    }
    long_cnt
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.count_next")]
extern "C-unwind" fn count_next(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3492 = countobject_CAST!(op);
    if (unsafe { (*lz).cnt }) == 9223372036854775807 {
        return count_nextlong(lz);
    }
    let __v1: i64 = unsafe { (*lz).cnt };
    unsafe {
        (*lz).cnt = __v1.wrapping_add(1);
    }
    unsafe { PyLong_FromSsize_t(__v1) }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.count_repr")]
extern "C-unwind" fn count_repr(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3492 = countobject_CAST!(op);
    if unsafe { (*lz).long_cnt }.is_null() {
        let cnt: i64 = unsafe { (*lz).cnt };
        return unsafe {
            PyUnicode_FromFormat(c"%s(%zd)".as_ptr(), _PyType_Name(Py_TYPE__2!(lz)), cnt)
        };
    }
    if PyLong_Check!(unsafe { (*lz).long_step }) != 0 {
        let step: i64 = unsafe { PyLong_AsLong((*lz).long_step) };
        let __v0: bool =
            step == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            unsafe { PyErr_Clear() };
        }
        if step == 1 {
            // Don't display step when it is an integer equal to 1
            return unsafe {
                PyUnicode_FromFormat(
                    c"%s(%R)".as_ptr(),
                    _PyType_Name(Py_TYPE__2!(lz)),
                    (*lz).long_cnt,
                )
            };
        }
    }
    unsafe {
        PyUnicode_FromFormat(
            c"%s(%R, %R)".as_ptr(),
            _PyType_Name(Py_TYPE__2!(lz)),
            (*lz).long_cnt,
            (*lz).long_step,
        )
    }
}

static mut count_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(count_dealloc),
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
            >(Some(count_repr))
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
                    Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
                >(PyObject_GenericGetAttr as *const ())
            })
        };
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 56i32;
        __t3.pfunc =
            (unsafe { core::ptr::addr_of!(itertools_count__doc__.0) as *const i8 }) as *mut ();
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
            >(Some(count_traverse))
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 62i32;
        __t5.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 63i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(Some(count_next))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 65i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(itertools_count))
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 74i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 0i32;
        __t9.pfunc = core::ptr::null_mut::<()>();
        __t9
    },
]);

static mut count_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.count\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_3492>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(count_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// repeat object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_3735 {
    ob_base: _object,
    element: *mut _object,
    cnt: i64,
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.repeat_new")]
extern "C-unwind" fn repeat_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut element: *mut _object = unsafe { core::mem::zeroed() };
    let mut cnt: i64 = 1i32.wrapping_neg() as i64;
    let mut n_args: i64 = PyTuple_GET_SIZE!(args);
    if !kwds.is_null() {
        let __v0: i64 = PyDict_GET_SIZE!(kwds);
        n_args = n_args.wrapping_add(__v0);
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kwds,
            c"O|n:repeat".as_ptr(),
            (core::ptr::addr_of_mut!(kwargs.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(element),
            core::ptr::addr_of_mut!(cnt),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    // Does user supply times argument?
    if n_args == 2 && cnt < 0 {
        cnt = 0;
    }
    let ro: *mut __SlateRecord_itertoolsmodule_c_3735 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_3735;
    if ro.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*ro).element = Py_NewRef!(element);
    }
    unsafe {
        (*ro).cnt = cnt;
    }
    ro as *mut _object
}

static mut kwargs: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"object\0".as_ptr() as *mut i8,
    b"times\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.repeat_dealloc")]
extern "C-unwind" fn repeat_dealloc(op: *mut _object) {
    let ro: *mut __SlateRecord_itertoolsmodule_c_3735 = repeatobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(ro);
    unsafe { PyObject_GC_UnTrack(ro as *mut ()) };
    Py_XDECREF!(unsafe { (*ro).element });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(ro as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.repeat_traverse")]
extern "C-unwind" fn repeat_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let ro: *mut __SlateRecord_itertoolsmodule_c_3735 = repeatobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(ro as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*ro).element }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.repeat_next")]
extern "C-unwind" fn repeat_next(op: *mut _object) -> *mut _object {
    let ro: *mut __SlateRecord_itertoolsmodule_c_3735 = repeatobject_CAST!(op);
    let mut cnt: i64 = unsafe { (*ro).cnt };
    if cnt == 0 {
        return core::ptr::null_mut::<_object>();
    }
    if cnt > 0 {
        cnt = cnt.wrapping_sub(1);
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*ro).cnt, cnt);
    }
    Py_NewRef!(unsafe { (*ro).element })
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.repeat_repr")]
extern "C-unwind" fn repeat_repr(op: *mut _object) -> *mut _object {
    let ro: *mut __SlateRecord_itertoolsmodule_c_3735 = repeatobject_CAST!(op);
    if (unsafe { (*ro).cnt }) == (1i32.wrapping_neg() as i64) {
        return unsafe {
            PyUnicode_FromFormat(
                c"%s(%R)".as_ptr(),
                _PyType_Name(Py_TYPE__2!(ro)),
                (*ro).element,
            )
        };
    } else {
        return unsafe {
            PyUnicode_FromFormat(
                c"%s(%R, %zd)".as_ptr(),
                _PyType_Name(Py_TYPE__2!(ro)),
                (*ro).element,
                (*ro).cnt,
            )
        };
    }
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.repeat_len")]
extern "C-unwind" fn repeat_len(op: *mut _object, _unused_args: *mut _object) -> *mut _object {
    let ro: *mut __SlateRecord_itertoolsmodule_c_3735 = repeatobject_CAST!(op);
    if (unsafe { (*ro).cnt }) == (1i32.wrapping_neg() as i64) {
        unsafe { PyErr_SetString(PyExc_TypeError, c"len() of unsized object".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSize_t((*ro).cnt as u64) }
}

static length_hint_doc: __SlateAlign16<[i8; 55]> = __SlateAlign16([
    80i8, 114i8, 105i8, 118i8, 97i8, 116i8, 101i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 32i8,
    101i8, 115i8, 116i8, 105i8, 109i8, 97i8, 116i8, 101i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8,
    110i8, 40i8, 108i8, 105i8, 115i8, 116i8, 40i8, 105i8, 116i8, 41i8, 41i8, 46i8, 0i8,
]);

/// sentinel
static mut repeat_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__length_hint__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(repeat_len);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(length_hint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

static repeat_doc: __SlateAlign16<[i8; 155]> = __SlateAlign16([
    114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 40i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    91i8, 44i8, 116i8, 105i8, 109i8, 101i8, 115i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 99i8, 114i8,
    101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8,
    111i8, 114i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 10i8,
    102i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 101i8, 100i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8,
    116i8, 105i8, 109i8, 101i8, 115i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 44i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 10i8, 101i8, 110i8, 100i8, 108i8, 101i8, 115i8, 115i8, 108i8, 121i8, 46i8, 0i8,
]);

static mut repeat_slots: __SlateAlign16<[__SlateRecord_object_h_377; 11]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(repeat_dealloc),
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
            >(Some(repeat_repr))
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
                    Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
                >(PyObject_GenericGetAttr as *const ())
            })
        };
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 56i32;
        __t3.pfunc = (unsafe { core::ptr::addr_of!(repeat_doc.0) as *const i8 }) as *mut ();
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
            >(Some(repeat_traverse))
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 62i32;
        __t5.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 63i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(Some(repeat_next))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 64i32;
        __t7.pfunc =
            (unsafe { core::ptr::addr_of_mut!(repeat_methods.0) as *mut PyMethodDef }) as *mut ();
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 65i32;
        __t8.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(repeat_new))
        };
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 74i32;
        __t9.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
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

static mut repeat_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.repeat\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_3735>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(repeat_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// ziplongest object
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_itertoolsmodule_c_3868 {
    ob_base: _object,
    tuplesize: i64,
    numactive: i64,
    /// tuple of iterators
    ittuple: *mut _object,
    result: *mut _object,
    fillvalue: *mut _object,
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.zip_longest_new")]
extern "C-unwind" fn zip_longest_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut i: i64 = 0;
    let mut fillvalue: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    let mut tuplesize: i64 = 0;
    let __v0: bool = !kwds.is_null() && PyDict_CheckExact!(kwds) != 0;
    let __v1: bool = __v0 && PyDict_GET_SIZE!(kwds) > 0;
    if __v1 {
        fillvalue = core::ptr::null_mut::<_object>();
        if PyDict_GET_SIZE!(kwds) == 1 {
            fillvalue = unsafe {
                PyDict_GetItemWithError(
                    kwds,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_fillvalue
                            ._ascii
                            .ob_base
                    ),
                )
            };
        }
        if fillvalue.is_null() {
            if unsafe { PyErr_Occurred() }.is_null() {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"zip_longest() got an unexpected keyword argument".as_ptr(),
                    )
                };
            }
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
        Py_INCREF!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
        PyTuple_SET_ITEM__2!(result, i, unsafe {
            core::ptr::addr_of_mut!(_Py_NoneStruct)
        });
        i = i.wrapping_add(1);
    }
    // create ziplongestobject structure
    let lz: *mut __SlateRecord_itertoolsmodule_c_3868 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_itertoolsmodule_c_3868;
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
        (*lz).numactive = tuplesize;
    }
    unsafe {
        (*lz).result = result;
    }
    unsafe {
        (*lz).fillvalue = Py_NewRef!(fillvalue);
    }
    lz as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.zip_longest_dealloc")]
extern "C-unwind" fn zip_longest_dealloc(op: *mut _object) {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3868 = ziplongestobject_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(lz);
    unsafe { PyObject_GC_UnTrack(lz as *mut ()) };
    Py_XDECREF!(unsafe { (*lz).ittuple });
    Py_XDECREF!(unsafe { (*lz).result });
    Py_XDECREF!(unsafe { (*lz).fillvalue });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(lz as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.zip_longest_traverse")]
extern "C-unwind" fn zip_longest_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3868 = ziplongestobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(lz as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*lz).ittuple }, visit, arg);
    Py_VISIT!(unsafe { (*lz).result }, visit, arg);
    Py_VISIT!(unsafe { (*lz).fillvalue }, visit, arg);
    0
}

fn zip_longest_next_lock_held(op: *mut _object) -> *mut _object {
    let lz: *mut __SlateRecord_itertoolsmodule_c_3868 = ziplongestobject_CAST!(op);
    let mut i: i64 = 0;
    let tuplesize: i64 = unsafe { (*lz).tuplesize };
    let mut result: *mut _object = unsafe { (*lz).result };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    if tuplesize == 0 {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*lz).numactive }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    if _PyObject_IsUniquelyReferenced(result) != 0 {
        Py_INCREF!(result);
        i = 0;
        while i < tuplesize {
            it = PyTuple_GET_ITEM!(unsafe { (*lz).ittuple }, i);
            if it.is_null() {
                item = Py_NewRef!(unsafe { (*lz).fillvalue });
            } else {
                item = unsafe { PyIter_Next(it) };
                if item.is_null() {
                    let __v1: i64 = unsafe { (*lz).numactive };
                    unsafe {
                        (*lz).numactive = __v1.wrapping_sub(1);
                    }
                    let __v3: bool =
                        (unsafe { (*lz).numactive }) == 0 || !unsafe { PyErr_Occurred() }.is_null();
                    if __v3 {
                        unsafe {
                            (*lz).numactive = 0;
                        }
                        Py_DECREF!(result);
                        return core::ptr::null_mut::<_object>();
                    } else {
                        item = Py_NewRef!(unsafe { (*lz).fillvalue });
                        PyTuple_SET_ITEM__4!(unsafe { (*lz).ittuple }, i);
                        Py_DECREF!(it);
                    }
                }
            }
            let olditem: *mut _object = PyTuple_GET_ITEM!(result, i);
            PyTuple_SET_ITEM__2!(result, i, item);
            Py_DECREF!(olditem);
            i = i.wrapping_add(1);
        }
        // bpo-42536: The GC may have untracked this result tuple. Since we're
        //
        // recycling it, make sure it's tracked again:
        _PyTuple_Recycle(result);
    } else {
        result = unsafe { PyTuple_New(tuplesize) };
        if result.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        i = 0;
        while i < tuplesize {
            it = PyTuple_GET_ITEM!(unsafe { (*lz).ittuple }, i);
            if it.is_null() {
                item = Py_NewRef!(unsafe { (*lz).fillvalue });
            } else {
                item = unsafe { PyIter_Next(it) };
                if item.is_null() {
                    let __v8: i64 = unsafe { (*lz).numactive };
                    unsafe {
                        (*lz).numactive = __v8.wrapping_sub(1);
                    }
                    let __v10: bool =
                        (unsafe { (*lz).numactive }) == 0 || !unsafe { PyErr_Occurred() }.is_null();
                    if __v10 {
                        unsafe {
                            (*lz).numactive = 0;
                        }
                        Py_DECREF!(result);
                        return core::ptr::null_mut::<_object>();
                    } else {
                        item = Py_NewRef!(unsafe { (*lz).fillvalue });
                        PyTuple_SET_ITEM__4!(unsafe { (*lz).ittuple }, i);
                        Py_DECREF!(it);
                    }
                }
            }
            PyTuple_SET_ITEM__2!(result, i, item);
            i = i.wrapping_add(1);
        }
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.zip_longest_next")]
extern "C-unwind" fn zip_longest_next(op: *mut _object) -> *mut _object {
    let result: *mut _object = zip_longest_next_lock_held(op);
    result
}

static zip_longest_doc: __SlateAlign16<[i8; 472]> = __SlateAlign16([
    122i8, 105i8, 112i8, 95i8, 108i8, 111i8, 110i8, 103i8, 101i8, 115i8, 116i8, 40i8, 42i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 115i8, 44i8, 32i8, 102i8, 105i8, 108i8, 108i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 122i8, 105i8, 112i8,
    95i8, 108i8, 111i8, 110i8, 103i8, 101i8, 115i8, 116i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 46i8, 95i8, 95i8, 110i8, 101i8, 120i8,
    116i8, 95i8, 95i8, 40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8,
    32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 10i8, 116i8, 104i8, 101i8, 32i8, 105i8, 45i8, 116i8,
    104i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 99i8, 111i8, 109i8, 101i8,
    115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 45i8, 116i8,
    104i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 114i8, 103i8,
    117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 46i8, 95i8,
    95i8, 110i8, 101i8, 120i8, 116i8, 95i8, 95i8, 40i8, 41i8, 10i8, 109i8, 101i8, 116i8, 104i8,
    111i8, 100i8, 32i8, 99i8, 111i8, 110i8, 116i8, 105i8, 110i8, 117i8, 101i8, 115i8, 32i8, 117i8,
    110i8, 116i8, 105i8, 108i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 110i8, 103i8, 101i8,
    115i8, 116i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    115i8, 101i8, 113i8, 117i8, 101i8, 110i8, 99i8, 101i8, 10i8, 105i8, 115i8, 32i8, 101i8, 120i8,
    104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8,
    101i8, 110i8, 32i8, 105i8, 116i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 115i8, 32i8, 83i8,
    116i8, 111i8, 112i8, 73i8, 116i8, 101i8, 114i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8,
    32i8, 87i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 104i8, 111i8, 114i8,
    116i8, 101i8, 114i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 115i8, 10i8,
    97i8, 114i8, 101i8, 32i8, 101i8, 120i8, 104i8, 97i8, 117i8, 115i8, 116i8, 101i8, 100i8, 44i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8, 108i8, 108i8, 118i8, 97i8, 108i8, 117i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 115i8, 117i8, 98i8, 115i8, 116i8, 105i8, 116i8, 117i8, 116i8, 101i8,
    100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 105i8, 114i8, 32i8, 112i8, 108i8, 97i8,
    99i8, 101i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 102i8, 105i8, 108i8, 108i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 10i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8,
    116i8, 111i8, 32i8, 78i8, 111i8, 110i8, 101i8, 32i8, 111i8, 114i8, 32i8, 99i8, 97i8, 110i8,
    32i8, 98i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8,
    98i8, 121i8, 32i8, 97i8, 32i8, 107i8, 101i8, 121i8, 119i8, 111i8, 114i8, 100i8, 32i8, 97i8,
    114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 10i8, 0i8,
]);

static mut ziplongest_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(zip_longest_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 58i32;
        __t1.pfunc = unsafe {
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
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 56i32;
        __t2.pfunc = (unsafe { core::ptr::addr_of!(zip_longest_doc.0) as *const i8 }) as *mut ();
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
            >(Some(zip_longest_traverse))
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 62i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                >(PyObject_SelfIter as *const ())
            })
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
            >(Some(zip_longest_next))
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 65i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(zip_longest_new))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 74i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 0i32;
        __t8.pfunc = core::ptr::null_mut::<()>();
        __t8
    },
]);

static mut ziplongest_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"itertools.zip_longest\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_itertoolsmodule_c_3868>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(ziplongest_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// module level code
static module_doc: __SlateAlign16<[i8; 1387]> = __SlateAlign16([
    70i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 116i8, 111i8, 111i8,
    108i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8, 99i8, 114i8, 101i8, 97i8, 116i8, 105i8, 110i8,
    103i8, 32i8, 97i8, 110i8, 100i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 105i8, 116i8,
    101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 46i8, 10i8, 10i8, 73i8, 110i8, 102i8, 105i8,
    110i8, 105i8, 116i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8,
    58i8, 10i8, 99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 115i8, 116i8, 97i8, 114i8, 116i8, 61i8,
    48i8, 44i8, 32i8, 115i8, 116i8, 101i8, 112i8, 61i8, 49i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8,
    115i8, 116i8, 97i8, 114i8, 116i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 43i8, 115i8,
    116i8, 101i8, 112i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 43i8, 50i8, 42i8, 115i8,
    116i8, 101i8, 112i8, 44i8, 32i8, 46i8, 46i8, 46i8, 10i8, 99i8, 121i8, 99i8, 108i8, 101i8, 40i8,
    112i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 112i8, 48i8, 44i8, 32i8, 112i8, 49i8, 44i8, 32i8,
    46i8, 46i8, 46i8, 32i8, 112i8, 108i8, 97i8, 115i8, 116i8, 44i8, 32i8, 112i8, 48i8, 44i8, 32i8,
    112i8, 49i8, 44i8, 32i8, 46i8, 46i8, 46i8, 10i8, 114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 40i8,
    101i8, 108i8, 101i8, 109i8, 32i8, 91i8, 44i8, 110i8, 93i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8,
    101i8, 108i8, 101i8, 109i8, 44i8, 32i8, 101i8, 108i8, 101i8, 109i8, 44i8, 32i8, 101i8, 108i8,
    101i8, 109i8, 44i8, 32i8, 46i8, 46i8, 46i8, 32i8, 101i8, 110i8, 100i8, 108i8, 101i8, 115i8,
    115i8, 108i8, 121i8, 32i8, 111i8, 114i8, 32i8, 117i8, 112i8, 32i8, 116i8, 111i8, 32i8, 110i8,
    32i8, 116i8, 105i8, 109i8, 101i8, 115i8, 10i8, 10i8, 73i8, 116i8, 101i8, 114i8, 97i8, 116i8,
    111i8, 114i8, 115i8, 32i8, 116i8, 101i8, 114i8, 109i8, 105i8, 110i8, 97i8, 116i8, 105i8, 110i8,
    103i8, 32i8, 111i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 104i8, 111i8, 114i8, 116i8,
    101i8, 115i8, 116i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 115i8, 101i8, 113i8, 117i8,
    101i8, 110i8, 99i8, 101i8, 58i8, 10i8, 97i8, 99i8, 99i8, 117i8, 109i8, 117i8, 108i8, 97i8,
    116i8, 101i8, 40i8, 112i8, 91i8, 44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 93i8, 41i8, 32i8, 45i8,
    45i8, 62i8, 32i8, 112i8, 48i8, 44i8, 32i8, 112i8, 48i8, 43i8, 112i8, 49i8, 44i8, 32i8, 112i8,
    48i8, 43i8, 112i8, 49i8, 43i8, 112i8, 50i8, 10i8, 98i8, 97i8, 116i8, 99i8, 104i8, 101i8, 100i8,
    40i8, 112i8, 44i8, 32i8, 110i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 91i8, 112i8, 48i8, 44i8,
    32i8, 112i8, 49i8, 44i8, 32i8, 46i8, 46i8, 46i8, 44i8, 32i8, 112i8, 95i8, 110i8, 45i8, 49i8,
    93i8, 44i8, 32i8, 91i8, 112i8, 95i8, 110i8, 44i8, 32i8, 112i8, 95i8, 110i8, 43i8, 49i8, 44i8,
    32i8, 46i8, 46i8, 46i8, 44i8, 32i8, 112i8, 95i8, 50i8, 110i8, 45i8, 49i8, 93i8, 44i8, 32i8,
    46i8, 46i8, 46i8, 10i8, 99i8, 104i8, 97i8, 105i8, 110i8, 40i8, 112i8, 44i8, 32i8, 113i8, 44i8,
    32i8, 46i8, 46i8, 46i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 112i8, 48i8, 44i8, 32i8, 112i8,
    49i8, 44i8, 32i8, 46i8, 46i8, 46i8, 32i8, 112i8, 108i8, 97i8, 115i8, 116i8, 44i8, 32i8, 113i8,
    48i8, 44i8, 32i8, 113i8, 49i8, 44i8, 32i8, 46i8, 46i8, 46i8, 10i8, 99i8, 104i8, 97i8, 105i8,
    110i8, 46i8, 102i8, 114i8, 111i8, 109i8, 95i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 40i8, 91i8, 112i8, 44i8, 32i8, 113i8, 44i8, 32i8, 46i8, 46i8, 46i8, 93i8, 41i8, 32i8,
    45i8, 45i8, 62i8, 32i8, 112i8, 48i8, 44i8, 32i8, 112i8, 49i8, 44i8, 32i8, 46i8, 46i8, 46i8,
    32i8, 112i8, 108i8, 97i8, 115i8, 116i8, 44i8, 32i8, 113i8, 48i8, 44i8, 32i8, 113i8, 49i8, 44i8,
    32i8, 46i8, 46i8, 46i8, 10i8, 99i8, 111i8, 109i8, 112i8, 114i8, 101i8, 115i8, 115i8, 40i8,
    100i8, 97i8, 116i8, 97i8, 44i8, 32i8, 115i8, 101i8, 108i8, 101i8, 99i8, 116i8, 111i8, 114i8,
    115i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 40i8, 100i8, 91i8, 48i8, 93i8, 32i8, 105i8, 102i8,
    32i8, 115i8, 91i8, 48i8, 93i8, 41i8, 44i8, 32i8, 40i8, 100i8, 91i8, 49i8, 93i8, 32i8, 105i8,
    102i8, 32i8, 115i8, 91i8, 49i8, 93i8, 41i8, 44i8, 32i8, 46i8, 46i8, 46i8, 10i8, 100i8, 114i8,
    111i8, 112i8, 119i8, 104i8, 105i8, 108i8, 101i8, 40i8, 112i8, 114i8, 101i8, 100i8, 105i8, 99i8,
    97i8, 116i8, 101i8, 44i8, 32i8, 115i8, 101i8, 113i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 115i8,
    101i8, 113i8, 91i8, 110i8, 93i8, 44i8, 32i8, 115i8, 101i8, 113i8, 91i8, 110i8, 43i8, 49i8,
    93i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 105i8, 110i8, 103i8, 32i8, 119i8, 104i8,
    101i8, 110i8, 32i8, 112i8, 114i8, 101i8, 100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 32i8, 102i8,
    97i8, 105i8, 108i8, 115i8, 10i8, 103i8, 114i8, 111i8, 117i8, 112i8, 98i8, 121i8, 40i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 91i8, 44i8, 32i8, 107i8, 101i8, 121i8, 102i8,
    117i8, 110i8, 99i8, 93i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 115i8, 117i8, 98i8, 45i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8,
    101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 107i8, 101i8, 121i8, 102i8, 117i8, 110i8, 99i8, 40i8, 118i8, 41i8, 10i8, 102i8, 105i8,
    108i8, 116i8, 101i8, 114i8, 102i8, 97i8, 108i8, 115i8, 101i8, 40i8, 112i8, 114i8, 101i8, 100i8,
    105i8, 99i8, 97i8, 116i8, 101i8, 44i8, 32i8, 115i8, 101i8, 113i8, 41i8, 32i8, 45i8, 45i8, 62i8,
    32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 111i8, 102i8, 32i8, 115i8,
    101i8, 113i8, 32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 112i8, 114i8, 101i8, 100i8, 105i8,
    99i8, 97i8, 116i8, 101i8, 40i8, 101i8, 108i8, 101i8, 109i8, 41i8, 32i8, 105i8, 115i8, 32i8,
    70i8, 97i8, 108i8, 115i8, 101i8, 10i8, 105i8, 115i8, 108i8, 105i8, 99i8, 101i8, 40i8, 115i8,
    101i8, 113i8, 44i8, 32i8, 91i8, 115i8, 116i8, 97i8, 114i8, 116i8, 44i8, 93i8, 32i8, 115i8,
    116i8, 111i8, 112i8, 32i8, 91i8, 44i8, 32i8, 115i8, 116i8, 101i8, 112i8, 93i8, 41i8, 32i8,
    45i8, 45i8, 62i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 102i8,
    114i8, 111i8, 109i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 115i8, 101i8, 113i8, 91i8,
    115i8, 116i8, 97i8, 114i8, 116i8, 58i8, 115i8, 116i8, 111i8, 112i8, 58i8, 115i8, 116i8, 101i8,
    112i8, 93i8, 10i8, 112i8, 97i8, 105i8, 114i8, 119i8, 105i8, 115i8, 101i8, 40i8, 115i8, 41i8,
    32i8, 45i8, 45i8, 62i8, 32i8, 40i8, 115i8, 91i8, 48i8, 93i8, 44i8, 115i8, 91i8, 49i8, 93i8,
    41i8, 44i8, 32i8, 40i8, 115i8, 91i8, 49i8, 93i8, 44i8, 115i8, 91i8, 50i8, 93i8, 41i8, 44i8,
    32i8, 40i8, 115i8, 91i8, 50i8, 93i8, 44i8, 32i8, 115i8, 91i8, 51i8, 93i8, 41i8, 44i8, 32i8,
    46i8, 46i8, 46i8, 10i8, 115i8, 116i8, 97i8, 114i8, 109i8, 97i8, 112i8, 40i8, 102i8, 117i8,
    110i8, 44i8, 32i8, 115i8, 101i8, 113i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 102i8, 117i8,
    110i8, 40i8, 42i8, 115i8, 101i8, 113i8, 91i8, 48i8, 93i8, 41i8, 44i8, 32i8, 102i8, 117i8,
    110i8, 40i8, 42i8, 115i8, 101i8, 113i8, 91i8, 49i8, 93i8, 41i8, 44i8, 32i8, 46i8, 46i8, 46i8,
    10i8, 116i8, 101i8, 101i8, 40i8, 105i8, 116i8, 44i8, 32i8, 110i8, 61i8, 50i8, 41i8, 32i8, 45i8,
    45i8, 62i8, 32i8, 40i8, 105i8, 116i8, 49i8, 44i8, 32i8, 105i8, 116i8, 50i8, 32i8, 44i8, 32i8,
    46i8, 46i8, 46i8, 32i8, 105i8, 116i8, 110i8, 41i8, 32i8, 115i8, 112i8, 108i8, 105i8, 116i8,
    115i8, 32i8, 111i8, 110i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8,
    32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 110i8, 10i8, 116i8, 97i8, 107i8, 101i8, 119i8, 104i8,
    105i8, 108i8, 101i8, 40i8, 112i8, 114i8, 101i8, 100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 44i8,
    32i8, 115i8, 101i8, 113i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 115i8, 101i8, 113i8, 91i8, 48i8,
    93i8, 44i8, 32i8, 115i8, 101i8, 113i8, 91i8, 49i8, 93i8, 44i8, 32i8, 117i8, 110i8, 116i8,
    105i8, 108i8, 32i8, 112i8, 114i8, 101i8, 100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 32i8, 102i8,
    97i8, 105i8, 108i8, 115i8, 10i8, 122i8, 105i8, 112i8, 95i8, 108i8, 111i8, 110i8, 103i8, 101i8,
    115i8, 116i8, 40i8, 112i8, 44i8, 32i8, 113i8, 44i8, 32i8, 46i8, 46i8, 46i8, 41i8, 32i8, 45i8,
    45i8, 62i8, 32i8, 40i8, 112i8, 91i8, 48i8, 93i8, 44i8, 32i8, 113i8, 91i8, 48i8, 93i8, 41i8,
    44i8, 32i8, 40i8, 112i8, 91i8, 49i8, 93i8, 44i8, 32i8, 113i8, 91i8, 49i8, 93i8, 41i8, 44i8,
    32i8, 46i8, 46i8, 46i8, 10i8, 10i8, 67i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 111i8,
    114i8, 105i8, 99i8, 32i8, 103i8, 101i8, 110i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8,
    58i8, 10i8, 112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 40i8, 112i8, 44i8, 32i8, 113i8,
    44i8, 32i8, 46i8, 46i8, 46i8, 32i8, 91i8, 114i8, 101i8, 112i8, 101i8, 97i8, 116i8, 61i8, 49i8,
    93i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 99i8, 97i8, 114i8, 116i8, 101i8, 115i8, 105i8, 97i8,
    110i8, 32i8, 112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 116i8, 10i8, 112i8, 101i8, 114i8, 109i8,
    117i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 40i8, 112i8, 91i8, 44i8, 32i8, 114i8,
    93i8, 41i8, 10i8, 99i8, 111i8, 109i8, 98i8, 105i8, 110i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    115i8, 40i8, 112i8, 44i8, 32i8, 114i8, 41i8, 10i8, 99i8, 111i8, 109i8, 98i8, 105i8, 110i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 95i8, 119i8, 105i8, 116i8, 104i8, 95i8, 114i8, 101i8,
    112i8, 108i8, 97i8, 99i8, 101i8, 109i8, 101i8, 110i8, 116i8, 40i8, 112i8, 44i8, 32i8, 114i8,
    41i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertoolsmodule_traverse")]
extern "C-unwind" fn itertoolsmodule_traverse(
    r#mod: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = get_module_state(r#mod);
    Py_VISIT__2!(unsafe { (*state).accumulate_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).batched_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).chain_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).combinations_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).compress_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).count_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).cwr_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).cycle_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).dropwhile_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).filterfalse_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).groupby_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state)._grouper_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).islice_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).pairwise_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).permutations_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).product_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).repeat_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).starmap_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).takewhile_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).tee_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).teedataobject_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).ziplongest_type }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertoolsmodule_clear")]
extern "C-unwind" fn itertoolsmodule_clear(r#mod: *mut _object) -> i32 {
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = get_module_state(r#mod);
    Py_CLEAR__3!((*state).accumulate_type);
    Py_CLEAR__3!((*state).batched_type);
    Py_CLEAR__3!((*state).chain_type);
    Py_CLEAR__3!((*state).combinations_type);
    Py_CLEAR__3!((*state).compress_type);
    Py_CLEAR__3!((*state).count_type);
    Py_CLEAR__3!((*state).cwr_type);
    Py_CLEAR__3!((*state).cycle_type);
    Py_CLEAR__3!((*state).dropwhile_type);
    Py_CLEAR__3!((*state).filterfalse_type);
    Py_CLEAR__3!((*state).groupby_type);
    Py_CLEAR__3!((*state)._grouper_type);
    Py_CLEAR__3!((*state).islice_type);
    Py_CLEAR__3!((*state).pairwise_type);
    Py_CLEAR__3!((*state).permutations_type);
    Py_CLEAR__3!((*state).product_type);
    Py_CLEAR__3!((*state).repeat_type);
    Py_CLEAR__3!((*state).starmap_type);
    Py_CLEAR__3!((*state).takewhile_type);
    Py_CLEAR__3!((*state).tee_type);
    Py_CLEAR__3!((*state).teedataobject_type);
    Py_CLEAR__3!((*state).ziplongest_type);
    0
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertoolsmodule_free")]
extern "C-unwind" fn itertoolsmodule_free(r#mod: *mut ()) {
    itertoolsmodule_clear(r#mod as *mut _object);
}

#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.itertoolsmodule_exec")]
extern "C-unwind" fn itertoolsmodule_exec(r#mod: *mut _object) -> i32 {
    let state: *mut __SlateRecord_itertoolsmodule_c_40 = get_module_state(r#mod);
    ADD_TYPE!(r#mod, (*state).accumulate_type, unsafe {
        core::ptr::addr_of_mut!(accumulate_spec)
    });
    ADD_TYPE!(r#mod, (*state).batched_type, unsafe {
        core::ptr::addr_of_mut!(batched_spec)
    });
    ADD_TYPE!(r#mod, (*state).chain_type, unsafe {
        core::ptr::addr_of_mut!(chain_spec)
    });
    ADD_TYPE!(r#mod, (*state).combinations_type, unsafe {
        core::ptr::addr_of_mut!(combinations_spec)
    });
    ADD_TYPE!(r#mod, (*state).compress_type, unsafe {
        core::ptr::addr_of_mut!(compress_spec)
    });
    ADD_TYPE!(r#mod, (*state).count_type, unsafe {
        core::ptr::addr_of_mut!(count_spec)
    });
    ADD_TYPE!(r#mod, (*state).cwr_type, unsafe {
        core::ptr::addr_of_mut!(cwr_spec)
    });
    ADD_TYPE!(r#mod, (*state).cycle_type, unsafe {
        core::ptr::addr_of_mut!(cycle_spec)
    });
    ADD_TYPE!(r#mod, (*state).dropwhile_type, unsafe {
        core::ptr::addr_of_mut!(dropwhile_spec)
    });
    ADD_TYPE!(r#mod, (*state).filterfalse_type, unsafe {
        core::ptr::addr_of_mut!(filterfalse_spec)
    });
    ADD_TYPE!(r#mod, (*state).groupby_type, unsafe {
        core::ptr::addr_of_mut!(groupby_spec)
    });
    ADD_TYPE!(r#mod, (*state)._grouper_type, unsafe {
        core::ptr::addr_of_mut!(_grouper_spec)
    });
    ADD_TYPE!(r#mod, (*state).islice_type, unsafe {
        core::ptr::addr_of_mut!(islice_spec)
    });
    ADD_TYPE!(r#mod, (*state).pairwise_type, unsafe {
        core::ptr::addr_of_mut!(pairwise_spec)
    });
    ADD_TYPE!(r#mod, (*state).permutations_type, unsafe {
        core::ptr::addr_of_mut!(permutations_spec)
    });
    ADD_TYPE!(r#mod, (*state).product_type, unsafe {
        core::ptr::addr_of_mut!(product_spec)
    });
    ADD_TYPE!(r#mod, (*state).repeat_type, unsafe {
        core::ptr::addr_of_mut!(repeat_spec)
    });
    ADD_TYPE!(r#mod, (*state).starmap_type, unsafe {
        core::ptr::addr_of_mut!(starmap_spec)
    });
    ADD_TYPE!(r#mod, (*state).takewhile_type, unsafe {
        core::ptr::addr_of_mut!(takewhile_spec)
    });
    ADD_TYPE!(r#mod, (*state).tee_type, unsafe {
        core::ptr::addr_of_mut!(tee_spec)
    });
    ADD_TYPE!(r#mod, (*state).teedataobject_type, unsafe {
        core::ptr::addr_of_mut!(teedataobject_spec)
    });
    ADD_TYPE!(r#mod, (*state).ziplongest_type, unsafe {
        core::ptr::addr_of_mut!(ziplongest_spec)
    });
    Py_SET_TYPE__2!(unsafe { (*state).teedataobject_type }, unsafe {
        core::ptr::addr_of_mut!(PyType_Type)
    });
    0
}

static mut itertoolsmodule_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(itertoolsmodule_exec),
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

/// sentinel
static mut module_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"tee\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(itertools_tee))
            })
        };
        __t0.ml_flags = 128i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(itertools_tee__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.itertoolsmodule.PyInit_itertools")]
extern "C-unwind" fn PyInit_itertools() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(itertoolsmodule)) }
}
