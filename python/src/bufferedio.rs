//! An implementation of Buffered I/O as defined by PEP 3116 - "New I/O"
//!
//! Classes defined here: BufferedIOBase, BufferedReader, BufferedWriter,
//! BufferedRandom.
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
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
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
macro_rules! ADJUST_POSITION {
    ($self:expr, $_new_pos:expr) => {
        loop {
            unsafe {
                (*$self).pos = $_new_pos;
            }
            if VALID_READ_BUFFER!($self) && (unsafe { (*$self).read_end }) < unsafe { (*$self).pos }
            {
                unsafe {
                    (*$self).read_end = (*$self).pos;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK_CLOSED__3 {
    ($self:expr, $error_msg:expr) => {{
        if IS_CLOSED!($self) && Py_SAFE_DOWNCAST!(READAHEAD!($self), i64) == 0 {
            unsafe { PyErr_SetString(PyExc_ValueError, ($error_msg) as *const i8) };
            return core::ptr::null_mut::<_object>();
        }
    }};
}

macro_rules! CHECK_INITIALIZED_INT {
    ($self:expr) => {{
        if (unsafe { (*$self).ok }) <= 0 {
            if (unsafe { (*$self).detached }) != 0 {
                unsafe {
                    PyErr_SetString(PyExc_ValueError, c"raw stream has been detached".as_ptr())
                };
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"I/O operation on uninitialized object".as_ptr(),
                    )
                };
            }
            return 1i32.wrapping_neg();
        }
    }};
}

macro_rules! CHECK_INITIALIZED {
    ($self:expr) => {{
        if (unsafe { (*$self).ok }) <= 0 {
            if (unsafe { (*$self).detached }) != 0 {
                unsafe {
                    PyErr_SetString(PyExc_ValueError, c"raw stream has been detached".as_ptr())
                };
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"I/O operation on uninitialized object".as_ptr(),
                    )
                };
            }
            return core::ptr::null_mut::<_object>();
        }
    }};
}

macro_rules! ENTER_BUFFERED {
    ($self:expr) => {{
        let __v: i32 = if (unsafe { PyThread_acquire_lock((*$self).lock, 0i32) }) != 0 {
            1
        } else {
            _enter_buffered_busy($self)
        };
        let __v2: bool = if __v != 0 {
            unsafe {
                core::ptr::write_volatile(
                    core::ptr::addr_of_mut!((*$self).owner),
                    PyThread_get_thread_ident(),
                )
            };
            true
        } else {
            false
        };
        __v2
    }};
}

macro_rules! EVAL_CALL_STAT_INC_IF_FUNCTION {
    () => {};
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

macro_rules! IS_CLOSED {
    ($self:expr) => {{
        let __v: bool = if unsafe { (*$self).buffer }.is_null() {
            true
        } else {
            let __v2: i32 = if (unsafe { (*$self).fast_closed_checks }) != 0 {
                unsafe { _PyFileIO_closed((*$self).raw) }
            } else {
                buffered_closed($self)
            };
            __v2 != 0
        };
        __v
    }};
}

macro_rules! LEAVE_BUFFERED {
    ($self:expr) => {
        loop {
            unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!((*$self).owner), 0u64) };
            unsafe { PyThread_release_lock((*$self).lock) };
            if true {
                break;
            }
        }
    };
}

macro_rules! MINUS_LAST_BLOCK {
    ($self:expr, $size:expr) => {
        if (unsafe { (*$self).buffer_mask }) != 0 {
            ($size) & !unsafe { (*$self).buffer_mask }
        } else {
            unsafe { (*$self).buffer_size }.wrapping_mul(($size) / unsafe { (*$self).buffer_size })
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

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
    };
}

macro_rules! Py_CLEAR__8 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut __SlateRecord_bufferedio_c_264 =
                unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut __SlateRecord_bufferedio_c_264 = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<__SlateRecord_bufferedio_c_264>();
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

macro_rules! Py_SAFE_DOWNCAST {
    ($VALUE:expr, $NARROW:ty) => {
        _Py_STATIC_CAST!($NARROW, $VALUE)
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
macro_rules! Py_VISIT__6 {
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

macro_rules! Py_XINCREF {
    ($op:expr) => {
        Py_XINCREF(_PyObject_CAST!($op));
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

macro_rules! RAW_OFFSET {
    ($self:expr) => {
        if (VALID_READ_BUFFER!($self) || VALID_WRITE_BUFFER!($self))
            && (unsafe { (*$self).raw_pos }) >= 0
        {
            unsafe { (*$self).raw_pos }.wrapping_sub(unsafe { (*$self).pos })
        } else {
            0i64
        }
    };
}

macro_rules! RAW_TELL {
    ($self:expr) => {{
        let __v: i64 = if (unsafe { (*$self).abs_pos }) != (1i32.wrapping_neg() as i64) {
            unsafe { (*$self).abs_pos }
        } else {
            _buffered_raw_tell($self)
        };
        __v
    }};
}

macro_rules! READAHEAD {
    ($self:expr) => {
        if (unsafe { (*$self).readable }) != 0 && VALID_READ_BUFFER!($self) {
            unsafe { (*$self).read_end }.wrapping_sub(unsafe { (*$self).pos })
        } else {
            0i64
        }
    };
}

macro_rules! VALID_READ_BUFFER {
    ($self:expr) => {
        (unsafe { (*$self).readable }) != 0
            && (unsafe { (*$self).read_end }) != (1i32.wrapping_neg() as i64)
    };
}

macro_rules! VALID_WRITE_BUFFER {
    ($self:expr) => {
        (unsafe { (*$self).writable }) != 0
            && (unsafe { (*$self).write_end }) != (1i32.wrapping_neg() as i64)
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

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

macro_rules! buffered_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_bufferedio_c_264
    };
}

macro_rules! clinic_state__2 {
    ($self_:expr) => {
        find_io_state_by_def(Py_TYPE!($self_))
    };
}

macro_rules! rwpair_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_bufferedio_c_2238
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
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyTuple_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    static mut _PyIO_Module: PyModuleDef;
    fn rewind(arg0: *mut slate_support::FILE);
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
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
    fn PyObject_ASCII(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GetOptionalAttr(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn Py_ReprEnter(arg0: *mut _object) -> i32;
    fn Py_ReprLeave(arg0: *mut _object);
    fn PyType_GetModuleByDef(arg0: *mut _typeobject, arg1: *mut PyModuleDef) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyBytes_Concat(arg0: *mut *mut _object, arg1: *mut _object);
    fn _PyBytes_Resize(arg0: *mut *mut _object, arg1: i64) -> i32;
    fn PyBytes_Join(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_AsUTF8(arg0: *mut _object) -> *const i8;
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_SetRaisedException(arg0: *mut _object);
    fn PyErr_GivenExceptionMatches(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyErr_CheckSignals() -> i32;
    fn _PyErr_ChainExceptions1(arg0: *mut _object);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLongAndOverflow(arg0: *mut _object, arg1: *mut i32) -> i64;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyMemoryView_FromBuffer(arg0: *const __SlateRecord_pybuffer_h_33) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyThread_get_thread_ident() -> u64;
    fn PyThread_allocate_lock() -> *mut ();
    fn PyThread_free_lock(arg0: *mut ());
    fn PyThread_acquire_lock(arg0: *mut (), arg1: i32) -> i32;
    fn PyThread_acquire_lock_timed(arg0: *mut (), arg1: i64, arg2: i32) -> u32;
    fn PyThread_release_lock(arg0: *mut ());
    fn PyEval_SaveThread() -> *mut _ts;
    fn PyEval_RestoreThread(arg0: *mut _ts);
    fn PyObject_CallObject(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_CallFunction(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyObject_CallMethodObjArgs(arg0: *mut _object, arg1: *mut _object, ...) -> *mut _object;
    fn PyObject_VectorcallMethod(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn _Py_CheckFunctionResult(
        arg0: *mut _ts,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: *const i8,
    ) -> *mut _object;
    fn _PyObject_CallMethod(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        ...
    ) -> *mut _object;
    fn _PyObject_MakeTpCall(
        arg0: *mut _ts,
        arg1: *mut _object,
        arg2: *const *mut _object,
        arg3: i64,
        arg4: *mut _object,
    ) -> *mut _object;
    fn _PyErr_FormatFromCause(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _Py_FatalErrorFormat(arg0: *const i8, arg1: *const i8, ...);
    fn _PyFile_Flush(arg0: *mut _object) -> i32;
    fn _Py_IsInterpreterFinalizing(arg0: *mut _is) -> i32;
    fn _PyIOBase_check_readable(
        arg0: *mut _io_state,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn _PyIOBase_check_writable(
        arg0: *mut _io_state,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn _PyIOBase_check_seekable(
        arg0: *mut _io_state,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn _PyIOBase_finalize(arg0: *mut _object) -> i32;
    fn _PyFileIO_closed(arg0: *mut _object) -> i32;
    fn PyNumber_AsOff_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn _PyIOBase_cannot_pickle(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _Py_convert_optional_to_ssize_t(arg0: *mut _object, arg1: *mut ()) -> i32;
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
struct _dictkeysobject {}

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF!(obj);
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
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits148U0,
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
struct _dictvalues {}

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
struct __SlateRecord_pyerrors_h_69 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    myerrno: *mut _object,
    strerror: *mut _object,
    filename: *mut _object,
    filename2: *mut _object,
    written: i64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

fn PyTuple_GET_SIZE(op: *mut _object) -> i64 {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    Py_SIZE!(tuple)
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits201U0,
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct code_arena_st {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct _py_object_state {
    freelists: _Py_freelists,
    _not_used: i32,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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

fn _PyGCHead_SET_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    next: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    unsafe {
        (*gc)._gc_next = next as u64;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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
struct __SlateRecord_bufferedio_c_h_963 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bufferedio_c_h_1043 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bufferedio_c_h_1215 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
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
static mut bufferediobase_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io._BufferedIOBase\0".as_ptr() as *mut i8) as *const i8;
    __t0.flags = 1280u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(bufferediobase_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[unsafe(no_mangle)]
static mut bufferedrandom_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io.BufferedRandom\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_bufferedio_c_264>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(bufferedrandom_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[unsafe(no_mangle)]
static mut bufferedreader_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io.BufferedReader\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_bufferedio_c_264>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(bufferedreader_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[unsafe(no_mangle)]
static mut bufferedrwpair_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io.BufferedRWPair\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_bufferedio_c_2238>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(bufferedrwpair_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[unsafe(no_mangle)]
static mut bufferedwriter_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"_io.BufferedWriter\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord_bufferedio_c_264>() as u64) as i32;
    __t0.flags = 17664u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(bufferedwriter_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

// bytes
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

// _PyObject_CallNoArgs()
// _PyFile_Flush
// _PyObject_GC_UNTRACK()
// _Py_FatalErrorFormat()
// _Py_IsInterpreterFinalizing()
// FT_CLEAR_WEAKREFS()
// [clinic input]
// module _io
// class _io._BufferedIOBase "PyObject *" "clinic_state()->PyBufferedIOBase_Type"
// class _io._Buffered "buffered *" "clinic_state()->PyBufferedIOBase_Type"
// class _io.BufferedReader "buffered *" "clinic_state()->PyBufferedReader_Type"
// class _io.BufferedWriter "buffered *" "clinic_state()->PyBufferedWriter_Type"
// class _io.BufferedRWPair "rwpair *" "clinic_state()->PyBufferedRWPair_Type"
// class _io.BufferedRandom "buffered *" "clinic_state()->PyBufferedRandom_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=3b3ef9cbbbad4590]
/// BufferedIOBase class, inherits from IOBase.
static bufferediobase_doc: __SlateAlign16<[i8; 497]> = __SlateAlign16([
    66i8, 97i8, 115i8, 101i8, 32i8, 99i8, 108i8, 97i8, 115i8, 115i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 32i8, 73i8, 79i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 109i8,
    97i8, 105i8, 110i8, 32i8, 100i8, 105i8, 102i8, 102i8, 101i8, 114i8, 101i8, 110i8, 99i8, 101i8,
    32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 82i8, 97i8, 119i8, 73i8, 79i8, 66i8, 97i8, 115i8,
    101i8, 32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    114i8, 101i8, 97i8, 100i8, 40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 10i8,
    115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 115i8, 32i8, 111i8, 109i8, 105i8, 116i8,
    116i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8,
    97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    100i8, 111i8, 101i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 104i8, 97i8, 118i8, 101i8, 32i8,
    97i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 10i8, 105i8, 109i8, 112i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 104i8, 97i8,
    116i8, 32i8, 100i8, 101i8, 102i8, 101i8, 114i8, 115i8, 32i8, 116i8, 111i8, 32i8, 114i8, 101i8,
    97i8, 100i8, 105i8, 110i8, 116i8, 111i8, 40i8, 41i8, 46i8, 10i8, 10i8, 73i8, 110i8, 32i8, 97i8,
    100i8, 100i8, 105i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8, 114i8, 101i8, 97i8, 100i8, 40i8,
    41i8, 44i8, 32i8, 114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 116i8, 111i8, 40i8, 41i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 40i8, 41i8, 32i8, 109i8, 97i8,
    121i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 10i8, 66i8, 108i8, 111i8, 99i8, 107i8, 105i8,
    110i8, 103i8, 73i8, 79i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8,
    114i8, 97i8, 119i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 105i8, 115i8, 32i8,
    105i8, 110i8, 32i8, 110i8, 111i8, 110i8, 45i8, 98i8, 108i8, 111i8, 99i8, 107i8, 105i8, 110i8,
    103i8, 10i8, 109i8, 111i8, 100i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 114i8, 101i8, 97i8, 100i8, 121i8, 59i8, 32i8, 117i8, 110i8, 108i8, 105i8, 107i8, 101i8,
    32i8, 116i8, 104i8, 101i8, 105i8, 114i8, 32i8, 114i8, 97i8, 119i8, 32i8, 99i8, 111i8, 117i8,
    110i8, 116i8, 101i8, 114i8, 112i8, 97i8, 114i8, 116i8, 115i8, 44i8, 32i8, 116i8, 104i8, 101i8,
    121i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 110i8, 101i8, 118i8, 101i8, 114i8, 10i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 10i8, 10i8, 65i8,
    32i8, 116i8, 121i8, 112i8, 105i8, 99i8, 97i8, 108i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8,
    109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 115i8, 104i8, 111i8, 117i8,
    108i8, 100i8, 32i8, 110i8, 111i8, 116i8, 32i8, 105i8, 110i8, 104i8, 101i8, 114i8, 105i8, 116i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 82i8, 97i8, 119i8, 73i8, 79i8, 66i8, 97i8,
    115i8, 101i8, 10i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 44i8, 32i8, 98i8, 117i8, 116i8, 32i8, 119i8, 114i8, 97i8, 112i8, 32i8,
    111i8, 110i8, 101i8, 46i8, 10i8, 0i8,
]);

fn _bufferediobase_readinto_generic(
    self_: *mut _object,
    buffer: &__SlateRecord_pybuffer_h_33,
    readinto1: i8,
) -> *mut _object {
    let mut len: i64 = 0;
    let attr: *mut _object = if readinto1 != 0 {
        unsafe {
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
        }
    } else {
        unsafe {
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
        }
    };
    let data: *mut _object = unsafe {
        _PyObject_CallMethod(
            self_,
            attr,
            c"n".as_ptr(),
            (*(buffer as *const __SlateRecord_pybuffer_h_33)).len,
        )
    };
    if data.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyBytes_Check!(data) == 0 {
        Py_DECREF!(data);
        unsafe { PyErr_SetString(PyExc_TypeError, c"read() should return bytes".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    len = PyBytes_GET_SIZE!(data);
    if len > unsafe { (*(buffer as *const __SlateRecord_pybuffer_h_33)).len } {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"read() returned too much data: %zd bytes requested, %zd returned".as_ptr(),
                (*(buffer as *const __SlateRecord_pybuffer_h_33)).len,
                len,
            )
        };
        Py_DECREF!(data);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((PyBytes_AS_STRING!(data) as *const ()) as *const core::ffi::c_void) as *const u8,
            ((*(buffer as *const __SlateRecord_pybuffer_h_33)).buf as *mut core::ffi::c_void)
                as *mut u8,
            ((len as u64) as usize) as usize,
        )
    };
    Py_DECREF!(data);
    unsafe { PyLong_FromSsize_t(len) }
}

// [clinic input]
// @critical_section
// _io._BufferedIOBase.readinto
//     buffer: Py_buffer(accept={rwbuffer})
//     /
// [clinic start generated code]
/// [clinic end generated code: output=8c8cda6684af8038 input=5273d20db7f56e1a]
fn _io__BufferedIOBase_readinto_impl(
    self_: *mut _object,
    buffer: &__SlateRecord_pybuffer_h_33,
) -> *mut _object {
    _bufferediobase_readinto_generic(self_, buffer, 0i8)
}

// [clinic input]
// @critical_section
// _io._BufferedIOBase.readinto1
//     buffer: Py_buffer(accept={rwbuffer})
//     /
// [clinic start generated code]
/// [clinic end generated code: output=358623e4fd2b69d3 input=d6eb723dedcee654]
fn _io__BufferedIOBase_readinto1_impl(
    self_: *mut _object,
    buffer: &__SlateRecord_pybuffer_h_33,
) -> *mut _object {
    _bufferediobase_readinto_generic(self_, buffer, 1i8)
}

fn bufferediobase_unsupported(state: *mut _io_state, message: *const i8) -> *mut _object {
    unsafe { PyErr_SetString((*state).unsupported_operation, message) };
    core::ptr::null_mut::<_object>()
}

// [clinic input]
// _io._BufferedIOBase.detach
//
//     cls: defining_class
//     /
//
// Disconnect this buffer from its underlying raw stream and return it.
//
// After the raw stream has been detached, the buffer is in an unusable
// state.
// [clinic start generated code]
fn _io__BufferedIOBase_detach_impl(self_: *mut _object, cls: *mut _typeobject) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    bufferediobase_unsupported(state, c"detach".as_ptr())
}

// [clinic input]
// _io._BufferedIOBase.read
//
//     cls: defining_class
//     size: int(unused=True) = -1
//     /
//
// Read and return up to n bytes.
//
// If the size argument is omitted, None, or negative, read and
// return all data until EOF.
//
// If the size argument is positive, and the underlying raw stream is
// not 'interactive', multiple raw reads may be issued to satisfy
// the byte count (unless EOF is reached first).
// However, for interactive raw streams (as well as sockets and pipes),
// at most one raw read will be issued, and a short result does not
// imply that EOF is imminent.
//
// Return an empty bytes object on EOF.
//
// Return None if the underlying raw stream was open in non-blocking
// mode and no data is available at the moment.
// [clinic start generated code]
// [clinic end generated code: output=b87b135d67cd4448 input=0b61a7b4357c1ea7]
fn _io__BufferedIOBase_read_impl(
    self_: *mut _object,
    cls: *mut _typeobject,
    _unused_size: i32,
) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    bufferediobase_unsupported(state, c"read".as_ptr())
}

// [clinic input]
// _io._BufferedIOBase.read1
//
//     cls: defining_class
//     size: int(unused=True) = -1
//     /
//
// Read and return up to size bytes, with at most one read() call to the underlying raw stream.
//
// Return an empty bytes object on EOF.
// A short result does not imply that EOF is imminent.
// [clinic start generated code]
// [clinic end generated code: output=aceb2765587b0a29 input=824f6f910465e61a]
fn _io__BufferedIOBase_read1_impl(
    self_: *mut _object,
    cls: *mut _typeobject,
    _unused_size: i32,
) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    bufferediobase_unsupported(state, c"read1".as_ptr())
}

// [clinic input]
// _io._BufferedIOBase.write
//
//     cls: defining_class
//     b: object(unused=True)
//     /
//
// Write buffer b to the IO stream.
//
// Return the number of bytes written, which is always
// the length of b in bytes.
//
// Raise BlockingIOError if the buffer is full and the
// underlying raw stream cannot accept more data at the moment.
// [clinic start generated code]
// [clinic end generated code: output=2e7fc62972487eaa input=af76380e020fd9e6]
fn _io__BufferedIOBase_write_impl(
    self_: *mut _object,
    cls: *mut _typeobject,
    _unused_b: *mut _object,
) -> *mut _object {
    let state: *mut _io_state = get_io_state_by_cls(cls);
    bufferediobase_unsupported(state, c"write".as_ptr())
}

// [clinic end generated code: output=712c635246bf2306 input=9793f5c8f71029ad]
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bufferedio_c_264 {
    ob_base: _object,
    raw: *mut _object,
    /// Initialized?
    ok: i32,
    detached: i32,
    readable: i32,
    writable: i32,
    finalizing: i8,
    /// True if this is a vanilla Buffered object (rather than a user derived
    /// class) *and* the raw stream is a vanilla FileIO object.
    fast_closed_checks: i32,
    /// Absolute position inside the raw stream (-1 if unknown).
    abs_pos: i64,
    /// A static buffer of size `buffer_size`
    buffer: *mut i8,
    /// Current logical position in the buffer.
    pos: i64,
    /// Position of the raw stream in the buffer.
    raw_pos: i64,
    /// Just after the last buffered byte in the buffer, or -1 if the buffer
    /// isn't ready for reading.
    read_end: i64,
    /// Just after the last byte actually written
    write_pos: i64,
    /// Just after the last byte waiting to be written, or -1 if the buffer
    /// isn't ready for writing.
    write_end: i64,
    lock: *mut (),
    owner: u64,
    buffer_size: i64,
    buffer_mask: i64,
    dict: *mut _object,
    weakreflist: *mut _object,
}

// Implementation notes:
//
// * BufferedReader, BufferedWriter and BufferedRandom try to share most
//   methods (this is helped by the members `readable` and `writable`, which
//   are initialized in the respective constructors)
// * They also share a single buffer for reading and writing. This enables
//   interleaved reads and writes without flushing. It also makes the logic
//   a bit trickier to get right.
// * The absolute position of the raw stream is cached, if possible, in the
//   `abs_pos` member. It must be updated every time an operation is done
//   on the raw stream. If not sure, it can be reinitialized by calling
//   _buffered_raw_tell(), which queries the raw stream (_buffered_raw_seek()
//   also does it). To read it, use RAW_TELL().
// * Three helpers, _bufferedreader_raw_read, _bufferedwriter_raw_write and
//   _bufferedwriter_flush_unlocked do a lot of useful housekeeping.
//
// NOTE: we should try to maintain block alignment of reads and writes to the
// raw stream (according to the buffer size), but for now it is only done
// in read() and friends.
// These macros protect the buffered object against concurrent operations.
fn _enter_buffered_busy(self_: *mut __SlateRecord_bufferedio_c_264) -> i32 {
    let mut relax_locking: i32 = 0;
    let mut st: u32 = 0;
    if (unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*self_).owner)) })
        == unsafe { PyThread_get_thread_ident() }
    {
        unsafe {
            PyErr_Format(
                PyExc_RuntimeError,
                c"reentrant call inside %R".as_ptr(),
                self_,
            )
        };
        return 0;
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    relax_locking = unsafe { _Py_IsInterpreterFinalizing(interp) };
    let _save: *mut _ts = unsafe { PyEval_SaveThread() };
    if relax_locking == 0 {
        st = (unsafe { PyThread_acquire_lock((*self_).lock, 1i32) }) as u32;
    } else {
        // When finalizing, we don't want a deadlock to happen with daemon
        // threads abruptly shut down while they owned the lock.
        // Therefore, only wait for a grace period (1 s.).
        // Note that non-daemon threads have already exited here, so this
        // shouldn't affect carefully written threaded I/O code.
        st = unsafe { PyThread_acquire_lock_timed((*self_).lock, 1000000.0f64 as i64, 0i32) };
    }
    unsafe { PyEval_RestoreThread(_save) };
    if relax_locking != 0 && (st as u32) != 1 {
        let ascii: *mut _object = unsafe { PyObject_ASCII(self_ as *mut _object) };
        let __v0: *const i8 = if !ascii.is_null() {
            unsafe { PyUnicode_AsUTF8(ascii) }
        } else {
            c"<ascii(self) failed>".as_ptr()
        };
        unsafe {
            _Py_FatalErrorFormat(c"_enter_buffered_busy".as_ptr(), c"could not acquire lock for %s at interpreter shutdown, possibly due to daemon threads".as_ptr(), __v0)
        };
        unsafe { core::hint::unreachable_unchecked() };
    }
    1
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.buffered_clear")]
extern "C-unwind" fn buffered_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_bufferedio_c_264 = buffered_CAST!(op);
    unsafe {
        (*self_).ok = 0;
    }
    Py_CLEAR!((*self_).raw);
    Py_CLEAR!((*self_).dict);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.buffered_dealloc")]
extern "C-unwind" fn buffered_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_bufferedio_c_264 = buffered_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(self_);
    unsafe {
        (*self_).finalizing = 1;
    }
    if (unsafe { _PyIOBase_finalize(op) }) < 0 {
        return;
    }
    _PyObject_GC_UNTRACK__2!(self_);
    unsafe {
        (*self_).ok = 0;
    }
    FT_CLEAR_WEAKREFS!(op, unsafe { (*self_).weakreflist });
    if !unsafe { (*self_).buffer }.is_null() {
        unsafe { PyMem_Free((*self_).buffer as *mut ()) };
        unsafe {
            (*self_).buffer = core::ptr::null_mut::<i8>();
        }
    }
    if !unsafe { (*self_).lock }.is_null() {
        unsafe { PyThread_free_lock((*self_).lock) };
        unsafe {
            (*self_).lock = core::ptr::null_mut::<()>();
        }
    }
    buffered_clear(op);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(self_ as *mut ()) };
    Py_DECREF__2!(tp);
}

// [clinic input]
// @critical_section
// _io._Buffered.__sizeof__
// [clinic start generated code]
/// [clinic end generated code: output=0231ef7f5053134e input=07a32d578073ea64]
fn _io__Buffered___sizeof___impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(self_));
    if !unsafe { (*self_).buffer }.is_null() {
        res = res.wrapping_add((unsafe { (*self_).buffer_size }) as u64);
    }
    unsafe { PyLong_FromSize_t(res) }
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.buffered_traverse")]
extern "C-unwind" fn buffered_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_bufferedio_c_264 = buffered_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(self_ as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*self_).raw }, visit, arg);
    Py_VISIT!(unsafe { (*self_).dict }, visit, arg);
    0
}

// Because this can call arbitrary code, it shouldn't be called when
// the refcount is 0 (that is, not directly from tp_dealloc unless
// the refcount has been temporarily re-incremented).
//
// [clinic input]
// _io._Buffered._dealloc_warn
//
//     source: object
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=d8db21c6dec0e614 input=8f845f2a4786391c]
fn _io__Buffered__dealloc_warn_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    source: *mut _object,
) -> *mut _object {
    if (unsafe { (*self_).ok }) != 0 && !unsafe { (*self_).raw }.is_null() {
        let r: *mut _object = PyObject_CallMethodOneArg(
            unsafe { (*self_).raw },
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
            source,
        );
        if !r.is_null() {
            Py_DECREF!(r);
        } else {
            unsafe { PyErr_Clear() };
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// _BufferedIOMixin methods
// This is not a class, just a collection of methods that will be reused
// by BufferedReader and BufferedWriter
// Flush and close
//
// [clinic input]
// @critical_section
// _io._Buffered.flush as _io__Buffered_simple_flush
// [clinic start generated code]
fn _io__Buffered_simple_flush_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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

// [clinic end generated code: output=29ebb3820db1bdfd input=5248cb84a65f80bd]
fn buffered_closed(self_: *mut __SlateRecord_bufferedio_c_264) -> i32 {
    let mut closed: i32 = 0;
    CHECK_INITIALIZED_INT!(self_);
    let res: *mut _object = unsafe {
        PyObject_GetAttr(
            (*self_).raw,
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
    };
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    closed = unsafe { PyObject_IsTrue(res) };
    Py_DECREF!(res);
    closed
}

// [clinic input]
// @critical_section
// @getter
// _io._Buffered.closed
// [clinic start generated code]
fn _io__Buffered_closed_get_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    unsafe {
        PyObject_GetAttr(
            (*self_).raw,
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
// _io._Buffered.close
// [clinic start generated code]
// [clinic end generated code: output=f08ce57290703a1a input=18eddefdfe4a3d2f]
fn _io__Buffered_close_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    let mut r: *mut _object = unsafe { core::mem::zeroed() };
    let mut r_1: i32 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        CHECK_INITIALIZED!(self_);
        if !ENTER_BUFFERED!(self_) {
            return core::ptr::null_mut::<_object>();
        } else {
            r_1 = buffered_closed(self_);
            if r_1 >= 0 {
                if r_1 > 0 {
                    res = Py_NewRef!(core::ptr::addr_of_mut!(_Py_NoneStruct));
                } else {
                    if (*self_).finalizing != 0 {
                        r = _io__Buffered__dealloc_warn_impl(self_, self_ as *mut _object);
                        if !r.is_null() {
                            Py_DECREF!(r);
                        } else {
                            PyErr_Clear();
                        }
                    }
                    // flush() will most probably re-take the lock, so drop it first
                    LEAVE_BUFFERED!(self_);
                    r_1 = _PyFile_Flush(self_ as *mut _object);
                    if !ENTER_BUFFERED!(self_) {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        exc = core::ptr::null_mut::<_object>();
                        if r_1 < 0 {
                            exc = PyErr_GetRaisedException();
                        }
                        res = PyObject_CallMethodNoArgs(
                            (*self_).raw,
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_close
                                    ._ascii
                                    .ob_base
                            ),
                        );
                        if !unsafe { (*self_).buffer }.is_null() {
                            PyMem_Free((*self_).buffer as *mut ());
                            (*self_).buffer = core::ptr::null_mut::<i8>();
                        }
                        if !exc.is_null() {
                            _PyErr_ChainExceptions1(exc);
                            Py_CLEAR__2!(res);
                        }
                        (*self_).read_end = 0;
                        (*self_).pos = 0;
                    }
                }
            }
            LEAVE_BUFFERED!(self_);
            return res;
        }
    }
}

// [clinic input]
// @critical_section
// _io._Buffered.detach
// [clinic start generated code]
// [clinic end generated code: output=7280b7b42033be0c input=56d95935b03fd326]
fn _io__Buffered_detach_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    if (unsafe { _PyFile_Flush(self_ as *mut _object) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let raw: *mut _object = unsafe { (*self_).raw };
    unsafe {
        (*self_).raw = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).detached = 1;
    }
    unsafe {
        (*self_).ok = 0;
    }
    raw
}

// Inquiries
// [clinic input]
// @critical_section
// _io._Buffered.seekable
// [clinic start generated code]
// [clinic end generated code: output=dd0fc057b8b779f7 input=d4ef1828a678be37]
fn _io__Buffered_seekable_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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
// _io._Buffered.readable
// [clinic start generated code]
// [clinic end generated code: output=90172abb5ceb6e8f input=e3a4fc1d297b2fd3]
fn _io__Buffered_readable_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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
// _io._Buffered.writable
// [clinic start generated code]
// [clinic end generated code: output=92afa07661ecb698 input=abe54107d59bca9a]
fn _io__Buffered_writable_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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
// @getter
// _io._Buffered.name
// [clinic start generated code]
// [clinic end generated code: output=4e3eee8d6f9d8552 input=45eb76bf6a10e6f7]
fn _io__Buffered_name_get_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    unsafe {
        PyObject_GetAttr(
            (*self_).raw,
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
// _io._Buffered.mode
// [clinic start generated code]
// [clinic end generated code: output=d2adf384051d3d10 input=6b84a0e6126f545e]
fn _io__Buffered_mode_get_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    unsafe {
        PyObject_GetAttr(
            (*self_).raw,
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
        )
    }
}

// Lower-level APIs
// [clinic input]
// @critical_section
// _io._Buffered.fileno
// [clinic start generated code]
// [clinic end generated code: output=0feb205748892fa4 input=0762d5e28542fd8c]
fn _io__Buffered_fileno_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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
// _io._Buffered.isatty
// [clinic start generated code]
// [clinic end generated code: output=b717648d58a95ee3 input=1c4fead777bae20a]
fn _io__Buffered_isatty_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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

// Helpers
// [clinic end generated code: output=c20e55caae67baea input=e53d182d7e490e3a]
/// Sets the current error to BlockingIOError
fn _set_BlockingIOError(msg: *const i8, written: i64) {
    unsafe { PyErr_Clear() };
    let err: *mut _object = unsafe {
        PyObject_CallFunction(
            PyExc_BlockingIOError,
            c"isn".as_ptr(),
            *slate_support::errno_location(),
            msg,
            written,
        )
    };
    if !err.is_null() {
        unsafe { PyErr_SetObject(PyExc_BlockingIOError, err) };
    }
    Py_XDECREF!(err);
}

/// Returns the address of the `written` member if a BlockingIOError was
/// raised, NULL otherwise. The error is always re-raised.
fn _buffered_check_blocking_error() -> *mut i64 {
    let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
    let __v0: bool =
        exc.is_null() || (unsafe { PyErr_GivenExceptionMatches(exc, PyExc_BlockingIOError) }) == 0;
    if __v0 {
        unsafe { PyErr_SetRaisedException(exc) };
        return core::ptr::null_mut::<i64>();
    }
    // TODO: sanity check (err->written >= 0)
    unsafe { PyErr_SetRaisedException(exc) };
    unsafe { core::ptr::addr_of_mut!((*(exc as *mut __SlateRecord_pyerrors_h_69)).written) }
}

fn _buffered_raw_tell(self_: *mut __SlateRecord_bufferedio_c_264) -> i64 {
    let mut n: i64 = 0;
    let res: *mut _object = PyObject_CallMethodNoArgs(unsafe { (*self_).raw }, unsafe {
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
    if res.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    n = unsafe { PyNumber_AsOff_t(res, PyExc_ValueError) };
    Py_DECREF!(res);
    if n < 0 {
        if unsafe { PyErr_Occurred() }.is_null() {
            unsafe {
                PyErr_Format(
                    PyExc_OSError,
                    c"Raw stream returned invalid position %zd".as_ptr(),
                    n,
                )
            };
        }
        return 1i32.wrapping_neg() as i64;
    }
    unsafe {
        (*self_).abs_pos = n;
    }
    n
}

fn _buffered_raw_seek(self_: *mut __SlateRecord_bufferedio_c_264, target: i64, whence: i32) -> i64 {
    let mut n: i64 = 0;
    let posobj: *mut _object = unsafe { PyLong_FromSsize_t(target) };
    if posobj.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    let whenceobj: *mut _object = unsafe { PyLong_FromLong(whence as i64) };
    if whenceobj.is_null() {
        Py_DECREF!(posobj);
        return 1i32.wrapping_neg() as i64;
    }
    let res: *mut _object = unsafe {
        PyObject_CallMethodObjArgs(
            (*self_).raw,
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
            whenceobj,
            core::ptr::null_mut::<()>(),
        )
    };
    Py_DECREF!(posobj);
    Py_DECREF!(whenceobj);
    if res.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    n = unsafe { PyNumber_AsOff_t(res, PyExc_ValueError) };
    Py_DECREF!(res);
    if n < 0 {
        if unsafe { PyErr_Occurred() }.is_null() {
            unsafe {
                PyErr_Format(
                    PyExc_OSError,
                    c"Raw stream returned invalid position %zd".as_ptr(),
                    n,
                )
            };
        }
        return 1i32.wrapping_neg() as i64;
    }
    unsafe {
        (*self_).abs_pos = n;
    }
    n
}

fn _buffered_init(self_: *mut __SlateRecord_bufferedio_c_264) -> i32 {
    let mut n: i64 = 0;
    if (unsafe { (*self_).buffer_size }) <= 0 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"buffer size must be strictly positive".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if !unsafe { (*self_).buffer }.is_null() {
        unsafe { PyMem_Free((*self_).buffer as *mut ()) };
    }
    unsafe {
        (*self_).buffer = PyMem_Malloc((*self_).buffer_size as u64) as *mut i8;
    }
    if unsafe { (*self_).buffer }.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    if !unsafe { (*self_).lock }.is_null() {
        unsafe { PyThread_free_lock((*self_).lock) };
    }
    unsafe {
        (*self_).lock = PyThread_allocate_lock();
    }
    if unsafe { (*self_).lock }.is_null() {
        unsafe { PyErr_SetString(PyExc_RuntimeError, c"can't allocate read lock".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_).owner), 0u64) };
    // Find out whether buffer_size is a power of 2
    //
    // XXX is this optimization useful?
    n = unsafe { (*self_).buffer_size }.wrapping_sub(1);
    while n & 1 != 0 {
        n >>= 1i32;
    }
    if n == 0 {
        unsafe {
            (*self_).buffer_mask = unsafe { (*self_).buffer_size }.wrapping_sub(1);
        }
    } else {
        unsafe {
            (*self_).buffer_mask = 0;
        }
    }
    if _buffered_raw_tell(self_) == (1i32.wrapping_neg() as i64) {
        unsafe { PyErr_Clear() };
    }
    0
}

/// Return 1 if an OSError with errno == EINTR is set (and then
/// clears the error indicator), 0 otherwise.
/// Should only be called when PyErr_Occurred() is true.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyIO_trap_eintr() -> i32 {
    if (unsafe { PyErr_ExceptionMatches(PyExc_OSError) }) == 0 {
        return 0;
    }
    let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
    if !unsafe { (*(exc as *mut __SlateRecord_pyerrors_h_69)).myerrno }.is_null() {
        let mut overflow: i32 = 0;
        let myerrno: i32 = (unsafe {
            PyLong_AsLongAndOverflow(
                (*(exc as *mut __SlateRecord_pyerrors_h_69)).myerrno,
                core::ptr::addr_of_mut!(overflow),
            )
        }) as i32;
        unsafe { PyErr_Clear() };
        if myerrno == (slate_support::EINTR as i32) {
            Py_DECREF!(exc);
            return 1;
        }
    }
    // This silences any error set by PyObject_RichCompareBool()
    unsafe { PyErr_SetRaisedException(exc) };
    0
}

// Shared methods and wrappers
fn buffered_flush_and_rewind_unlocked(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let res: *mut _object = _bufferedwriter_flush_unlocked(self_);
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(res);
    if (unsafe { (*self_).readable }) != 0 {
        // Rewind the raw stream so that its position corresponds to
        // the current logical position.
        let n: i64 = _buffered_raw_seek(self_, RAW_OFFSET!(self_).wrapping_neg(), 1i32);
        _bufferedreader_reset_buf(self_);
        if n == (1i32.wrapping_neg() as i64) {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

use slate_support::memchr;
use slate_support::memcpy;
use slate_support::memmove;

// [clinic input]
// @critical_section
// _io._Buffered.flush
// [clinic start generated code]
fn _io__Buffered_flush_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    CHECK_CLOSED__3!(self_, c"flush of closed file".as_ptr() as *mut i8);
    if !ENTER_BUFFERED!(self_) {
        return core::ptr::null_mut::<_object>();
    }
    let res: *mut _object = buffered_flush_and_rewind_unlocked(self_);
    LEAVE_BUFFERED!(self_);
    res
}

// [clinic input]
// @critical_section
// _io._Buffered.peek
//     size: Py_ssize_t = 0
//     /
//
// [clinic start generated code]
// [clinic end generated code: output=da2674ef1ce71f3a input=6b30de9f083419c2]
fn _io__Buffered_peek_impl(self_: *mut __SlateRecord_bufferedio_c_264, size: i64) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        CHECK_INITIALIZED!(self_);
        CHECK_CLOSED__3!(self_, c"peek of closed file".as_ptr() as *mut i8);
        if !ENTER_BUFFERED!(self_) {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if (*self_).writable != 0 {
                    res = buffered_flush_and_rewind_unlocked(self_);
                    if res.is_null() {
                        break '__join_0;
                    } else {
                        Py_CLEAR__2!(res);
                    }
                }
                res = _bufferedreader_peek_unlocked(self_);
            }
            LEAVE_BUFFERED!(self_);
            return res;
        }
    }
}

// [clinic input]
// @critical_section
// _io._Buffered.read
//     size as n: Py_ssize_t(accept={int, NoneType}) = -1
//     /
// [clinic start generated code]
// [clinic end generated code: output=ba7a097ca230102b input=56733376f926d982]
fn _io__Buffered_read_impl(self_: *mut __SlateRecord_bufferedio_c_264, n: i64) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    CHECK_INITIALIZED!(self_);
    if n < (1i32.wrapping_neg() as i64) {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"read length must be non-negative or -1".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    CHECK_CLOSED__3!(self_, c"read of closed file".as_ptr() as *mut i8);
    if n == (1i32.wrapping_neg() as i64) {
        // The number of bytes is unspecified, read until the end of stream
        if !ENTER_BUFFERED!(self_) {
            return core::ptr::null_mut::<_object>();
        }
        res = _bufferedreader_read_all(self_);
    } else {
        res = _bufferedreader_read_fast(self_, n);
        if res != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            return res;
        }
        Py_DECREF!(res);
        if !ENTER_BUFFERED!(self_) {
            return core::ptr::null_mut::<_object>();
        }
        res = _bufferedreader_read_generic(self_, n);
    }
    LEAVE_BUFFERED!(self_);
    res
}

// [clinic input]
// @critical_section
// _io._Buffered.read1
//     size as n: Py_ssize_t = -1
//     /
// [clinic start generated code]
// [clinic end generated code: output=f41c78bb15b9bbe9 input=bdb4b0425b295472]
fn _io__Buffered_read1_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    mut n: i64,
) -> *mut _object {
    let mut have: i64 = 0;
    let mut r: i64 = 0;
    let mut res: *mut _object = core::ptr::null_mut::<_object>();
    CHECK_INITIALIZED!(self_);
    if n < 0 {
        n = unsafe { (*self_).buffer_size };
    }
    CHECK_CLOSED__3!(self_, c"read of closed file".as_ptr() as *mut i8);
    if n == 0 {
        return unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 0i64) };
    }
    // Return up to n bytes.  If at least one byte is buffered, we
    // only return buffered bytes.  Otherwise, we do one raw read.
    have = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
    if have > 0 {
        n = Py_MIN!(have, n);
        res = _bufferedreader_read_fast(self_, n);
        return res;
    }
    res = unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), n) };
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if !ENTER_BUFFERED!(self_) {
        Py_DECREF!(res);
        return core::ptr::null_mut::<_object>();
    }
    // Flush the write buffer if necessary
    if (unsafe { (*self_).writable }) != 0 {
        let r: *mut _object = buffered_flush_and_rewind_unlocked(self_);
        if r.is_null() {
            LEAVE_BUFFERED!(self_);
            Py_DECREF!(res);
            return core::ptr::null_mut::<_object>();
        }
        Py_DECREF!(r);
    }
    _bufferedreader_reset_buf(self_);
    r = _bufferedreader_raw_read(self_, PyBytes_AS_STRING!(res), n);
    LEAVE_BUFFERED!(self_);
    if r == (1i32.wrapping_neg() as i64) {
        Py_DECREF!(res);
        return core::ptr::null_mut::<_object>();
    }
    if r == (2i32.wrapping_neg() as i64) {
        r = 0;
    }
    if n > r {
        unsafe { _PyBytes_Resize(core::ptr::addr_of_mut!(res), r) };
    }
    res
}

// [clinic end generated code: output=bcc4fb4e54d103a3 input=3d0ad241aa52b36c]
fn _buffered_readinto_generic(
    self_: *mut __SlateRecord_bufferedio_c_264,
    buffer: *mut __SlateRecord_pybuffer_h_33,
    readinto1: i8,
) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut remaining: i64 = unsafe { core::mem::zeroed() };
    let mut written: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        written = 0;
        res = core::ptr::null_mut::<_object>();
        CHECK_INITIALIZED!(self_);
        CHECK_CLOSED__3!(self_, c"readinto of closed file".as_ptr() as *mut i8);
        n = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
        if n > 0 {
            if n >= (*buffer).len {
                core::ptr::copy_nonoverlapping(
                    ((unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize)
                        as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((*buffer).buf as *mut core::ffi::c_void) as *mut u8,
                    (((*buffer).len as u64) as usize) as usize,
                );
                let __v13: i64 = (*self_).pos;
                (*self_).pos = __v13.wrapping_add((*buffer).len);
                return PyLong_FromSsize_t((*buffer).len);
            } else {
                core::ptr::copy_nonoverlapping(
                    ((unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize)
                        as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((*buffer).buf as *mut core::ffi::c_void) as *mut u8,
                    ((n as u64) as usize) as usize,
                );
                let __v10: i64 = (*self_).pos;
                (*self_).pos = __v10.wrapping_add(n);
                written = n;
            }
        }
        if !ENTER_BUFFERED!(self_) {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if (*self_).writable != 0 {
                    res = buffered_flush_and_rewind_unlocked(self_);
                    if res.is_null() {
                        break '__join_0;
                    } else {
                        Py_CLEAR__2!(res);
                    }
                }
                _bufferedreader_reset_buf(self_);
                (*self_).pos = 0;
                remaining = unsafe { (*buffer).len }.wrapping_sub(written);
                '__join_1: {
                    '__join_7: {
                        '__loop_2: loop {
                            if remaining > 0 {
                                '__join_3: {
                                    // If remaining bytes is larger than internal buffer size, copy
                                    // directly into caller's buffer.
                                    if remaining > (*self_).buffer_size {
                                        n = _bufferedreader_raw_read(
                                            self_,
                                            ((*buffer).buf as *mut i8)
                                                .wrapping_offset(written as isize),
                                            remaining,
                                        );
                                    } else {
                                        if !(readinto1 != 0 && written != 0) {
                                            n = _bufferedreader_fill_buffer(self_);
                                            if n > 0 {
                                                if n > remaining {
                                                    n = remaining;
                                                }
                                                core::ptr::copy_nonoverlapping(
                                                    ((unsafe { (*self_).buffer }
                                                        .wrapping_offset((*self_).pos as isize)
                                                        as *const ())
                                                        as *const core::ffi::c_void)
                                                        as *const u8,
                                                    ((((*buffer).buf as *mut i8)
                                                        .wrapping_offset(written as isize)
                                                        as *mut ())
                                                        as *mut core::ffi::c_void)
                                                        as *mut u8,
                                                    ((n as u64) as usize) as usize,
                                                );
                                                let __v7: i64 = (*self_).pos;
                                                (*self_).pos = __v7.wrapping_add(n);
                                                break '__join_3;
                                            }
                                        } else {
                                            n = 0;
                                        }
                                    }
                                    // In readinto1 mode, we do not want to fill the internal
                                    // buffer if we already have some data to return
                                    if n == 0 || n == (2i32.wrapping_neg() as i64) && written > 0 {
                                        break '__join_1;
                                    } else {
                                        if n < 0 {
                                            break '__join_7;
                                        } else {
                                            // At most one read in readinto1 mode
                                            if readinto1 != 0 {
                                                break '__loop_2;
                                            }
                                        }
                                    }
                                }
                                written = written.wrapping_add(n);
                                remaining = remaining.wrapping_sub(n);
                            } else {
                                break '__join_1;
                            }
                        }
                        written = written.wrapping_add(n);
                        break '__join_1;
                    }
                    if n == (2i32.wrapping_neg() as i64) {
                        res = Py_NewRef!(core::ptr::addr_of_mut!(_Py_NoneStruct));
                        break '__join_0;
                    } else {
                        break '__join_0;
                    }
                }
                res = PyLong_FromSsize_t(written);
            }
            LEAVE_BUFFERED!(self_);
            return res;
        }
    }
    // short circuit
}

// [clinic input]
// @critical_section
// _io._Buffered.readinto
//     buffer: Py_buffer(accept={rwbuffer})
//     /
// [clinic start generated code]
/// [clinic end generated code: output=bcb376580b1d8170 input=777c33e7adaa2bcd]
fn _io__Buffered_readinto_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    buffer: *mut __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    _buffered_readinto_generic(self_, buffer, 0i8)
}

// [clinic input]
// @critical_section
// _io._Buffered.readinto1
//     buffer: Py_buffer(accept={rwbuffer})
//     /
// [clinic start generated code]
/// [clinic end generated code: output=6e5c6ac5868205d6 input=ef03cc5fc92a6895]
fn _io__Buffered_readinto1_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    buffer: *mut __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    _buffered_readinto_generic(self_, buffer, 1i8)
}

fn _buffered_readline(self_: *mut __SlateRecord_bufferedio_c_264, mut limit: i64) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut r: *mut _object = unsafe { core::mem::zeroed() };
    let mut end: *const i8 = unsafe { core::mem::zeroed() };
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut start: *const i8 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut chunks: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        chunks = core::ptr::null_mut::<_object>();
        CHECK_CLOSED__3!(self_, c"readline of closed file".as_ptr() as *mut i8);
        // First, try to find a line in the buffer. This can run unlocked because
        // the calls to the C API are simple enough that they can't trigger
        // any thread switch.
        n = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
        if limit >= 0 && n > limit {
            n = limit;
        }
        start = unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize) as *const i8;
        s = ((memchr(
            (start as *const ()) as *const core::ffi::c_void,
            10i32 as i32,
            (n as u64) as usize,
        ) as *mut ()) as *const ()) as *const i8;
        if !s.is_null() {
            res = PyBytes_FromStringAndSize(
                start,
                (s.offset_from(start as *const i8) as i64).wrapping_add(1),
            );
            if !res.is_null() {
                let __v15: i64 = (*self_).pos;
                let __v14: i64 =
                    __v15.wrapping_add((s.offset_from(start as *const i8) as i64).wrapping_add(1));
                (*self_).pos = __v14;
            }
        } else {
            if n == limit {
                res = PyBytes_FromStringAndSize(start, n);
                if !res.is_null() {
                    let __v12: i64 = (*self_).pos;
                    (*self_).pos = __v12.wrapping_add(n);
                }
            } else {
                if ENTER_BUFFERED!(self_) {
                    '__join_1: {
                        // Now we try to get some more from the raw stream
                        chunks = PyList_New(0i64);
                        if !chunks.is_null() {
                            if n > 0 {
                                res = PyBytes_FromStringAndSize(start, n);
                                if res.is_null() {
                                    break '__join_1;
                                } else {
                                    if PyList_Append(chunks, res) < 0 {
                                        Py_CLEAR__2!(res);
                                        break '__join_1;
                                    } else {
                                        Py_CLEAR__2!(res);
                                        let __v9: i64 = (*self_).pos;
                                        (*self_).pos = __v9.wrapping_add(n);
                                        if limit >= 0 {
                                            limit = limit.wrapping_sub(n);
                                        }
                                    }
                                }
                            }
                            if (*self_).writable != 0 {
                                r = buffered_flush_and_rewind_unlocked(self_);
                                if r.is_null() {
                                    break '__join_1;
                                } else {
                                    Py_DECREF!(r);
                                }
                            }
                            '__join_7: {
                                '__join_17: {
                                    '__join_12: {
                                        loop {
                                            _bufferedreader_reset_buf(self_);
                                            n = _bufferedreader_fill_buffer(self_);
                                            if n == (1i32.wrapping_neg() as i64) {
                                                break '__join_1;
                                            } else {
                                                if n <= 0 {
                                                    break '__join_7;
                                                } else {
                                                    if limit >= 0 && n > limit {
                                                        n = limit;
                                                    }
                                                    start = (*self_).buffer as *const i8;
                                                    end = start.wrapping_offset(n as isize);
                                                    s = start;
                                                    while s < end {
                                                        let __v4: *const i8 = s;
                                                        s = __v4.wrapping_offset(1 as isize);
                                                        if (*__v4 as i32) == 10 {
                                                            break '__join_17;
                                                        }
                                                    }
                                                    res = PyBytes_FromStringAndSize(start, n);
                                                    if res.is_null() {
                                                        break '__join_1;
                                                    } else {
                                                        if n == limit {
                                                            break '__join_12;
                                                        } else {
                                                            if PyList_Append(chunks, res) < 0 {
                                                                break;
                                                            } else {
                                                                Py_CLEAR__2!(res);
                                                                if limit >= 0 {
                                                                    limit = limit.wrapping_sub(n);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Py_CLEAR__2!(res);
                                        break '__join_1;
                                    }
                                    (*self_).pos = n;
                                    break '__join_7;
                                }
                                res = PyBytes_FromStringAndSize(
                                    start,
                                    s.offset_from(start as *const i8) as i64,
                                );
                                if res.is_null() {
                                    break '__join_1;
                                } else {
                                    (*self_).pos = s.offset_from(start as *const i8) as i64;
                                }
                            }
                            if !res.is_null() {
                                __v0 = PyList_Append(chunks, res) < 0;
                            } else {
                                __v0 = false;
                            }
                            if __v0 {
                                Py_CLEAR__2!(res);
                            } else {
                                Py_XSETREF__2!(
                                    res,
                                    PyBytes_Join(
                                        core::ptr::addr_of_mut!(
                                            _PyRuntime.static_objects.singletons.bytes_empty
                                        ) as *mut _object,
                                        chunks
                                    )
                                );
                            }
                        }
                    }
                    LEAVE_BUFFERED!(self_);
                }
            }
        }
        Py_XDECREF!(chunks);
        return res;
    }
}

// [clinic input]
// @critical_section
// _io._Buffered.readline
//     size: Py_ssize_t(accept={int, NoneType}) = -1
//     /
// [clinic start generated code]
fn _io__Buffered_readline_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    size: i64,
) -> *mut _object {
    CHECK_INITIALIZED!(self_);
    _buffered_readline(self_, size)
}

// [clinic input]
// @critical_section
// _io._Buffered.tell
// [clinic start generated code]
// [clinic end generated code: output=24dd2aa6e33be83c input=e81ca5abd4280776]
fn _io__Buffered_tell_impl(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let mut pos: i64 = 0;
    CHECK_INITIALIZED!(self_);
    pos = _buffered_raw_tell(self_);
    if pos == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    let __v0: i64 = pos;
    pos = __v0.wrapping_sub(RAW_OFFSET!(self_));
    // GH-95782
    if pos < 0 {
        pos = 0;
    }
    unsafe { PyLong_FromSsize_t(pos) }
}

// [clinic input]
// @critical_section
// _io._Buffered.seek
//     target as targetobj: object
//     whence: int = 0
//     /
// [clinic start generated code]
// [clinic end generated code: output=386972ae84716c1e input=ab12e67d8abcb42f]
fn _io__Buffered_seek_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    targetobj: *mut _object,
    whence: i32,
) -> *mut _object {
    let mut offset: i64 = unsafe { core::mem::zeroed() };
    let mut avail: i64 = unsafe { core::mem::zeroed() };
    let mut current: i64 = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut target: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        CHECK_INITIALIZED!(self_);
        // Do some error checking instead of trusting OS 'seek()'
        // error detection, just in case.
        if (whence < 0 || whence > 2) && whence != 4 && whence != 3 {
            PyErr_Format(
                PyExc_ValueError,
                c"whence value %d unsupported".as_ptr(),
                whence,
            );
            return core::ptr::null_mut::<_object>();
        } else {
            CHECK_CLOSED__3!(self_, c"seek of closed file".as_ptr() as *mut i8);
            let state: *mut _io_state = find_io_state_by_def(Py_TYPE__2!(self_));
            if _PyIOBase_check_seekable(
                state,
                (*self_).raw,
                _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
            )
            .is_null()
            {
                return core::ptr::null_mut::<_object>();
            } else {
                target = PyNumber_AsOff_t(targetobj, PyExc_ValueError);
                if target == (1i32.wrapping_neg() as i64) {
                    __v8 = !PyErr_Occurred().is_null();
                } else {
                    __v8 = false;
                }
                if __v8 {
                    return core::ptr::null_mut::<_object>();
                } else {
                    // SEEK_SET and SEEK_CUR are special because we could seek inside the
                    // buffer. Other whence values must be managed without this optimization.
                    // Some Operating Systems can provide additional values, like
                    // SEEK_HOLE/SEEK_DATA.
                    if (whence == 0 || whence == 1) && (*self_).readable != 0 {
                        // Check if seeking leaves us inside the current buffer,
                        // so as to return quickly if possible. Also, we needn't take the
                        // lock in this fast path.
                        // Don't know how to do that when whence == 2, though.
                        //
                        // NOTE: RAW_TELL() can release the GIL but the object is in a stable
                        // state at this point.
                        current = RAW_TELL!(self_);
                        avail = READAHEAD!(self_);
                        if avail > 0 {
                            if whence == 0 {
                                offset =
                                    target.wrapping_sub(current.wrapping_sub(RAW_OFFSET!(self_)));
                            } else {
                                offset = target;
                            }
                            if offset >= unsafe { (*self_).pos }.wrapping_neg() && offset <= avail {
                                let __v3: i64 = (*self_).pos;
                                (*self_).pos = __v3.wrapping_add(offset);
                                // GH-95782
                                if current.wrapping_sub(avail).wrapping_add(offset) < 0 {
                                    return PyLong_FromSsize_t(0i64);
                                } else {
                                    return PyLong_FromSsize_t(
                                        current.wrapping_sub(avail).wrapping_add(offset),
                                    );
                                }
                            }
                        }
                    }
                    if !ENTER_BUFFERED!(self_) {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        '__join_0: {
                            // Fallback: invoke raw seek() method and clear buffer
                            if (*self_).writable != 0 {
                                res = _bufferedwriter_flush_unlocked(self_);
                                if res.is_null() {
                                    break '__join_0;
                                } else {
                                    Py_CLEAR__2!(res);
                                }
                            }
                            // TODO: align on block boundary and read buffer if needed?
                            if whence == 1 {
                                let __v1: i64 = target;
                                target = __v1.wrapping_sub(RAW_OFFSET!(self_));
                            }
                            n = _buffered_raw_seek(self_, target, whence);
                            if n != (1i32.wrapping_neg() as i64) {
                                (*self_).raw_pos = 1i32.wrapping_neg() as i64;
                                res = PyLong_FromSsize_t(n);
                                if !res.is_null() && (*self_).readable != 0 {
                                    _bufferedreader_reset_buf(self_);
                                }
                            }
                        }
                        LEAVE_BUFFERED!(self_);
                        return res;
                    }
                }
            }
        }
    }
}

// [clinic input]
// @critical_section
// _io._Buffered.truncate
//     cls: defining_class
//     pos: object = None
//     /
// [clinic start generated code]
// [clinic end generated code: output=7ae0e8dc46efdefb input=b5a12be70e0ad07b]
fn _io__Buffered_truncate_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    cls: *mut _typeobject,
    pos: *mut _object,
) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        CHECK_INITIALIZED!(self_);
        CHECK_CLOSED__3!(self_, c"truncate of closed file".as_ptr() as *mut i8);
        if (*self_).writable == 0 {
            let state: *mut _io_state = get_io_state_by_cls(cls);
            return bufferediobase_unsupported(state, c"truncate".as_ptr());
        } else {
            if !ENTER_BUFFERED!(self_) {
                return core::ptr::null_mut::<_object>();
            } else {
                res = buffered_flush_and_rewind_unlocked(self_);
                if !res.is_null() {
                    Py_CLEAR__2!(res);
                    res = PyObject_CallMethodOneArg(
                        (*self_).raw,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py_truncate
                                ._ascii
                                .ob_base
                        ),
                        pos,
                    );
                    if !res.is_null() {
                        // Reset cached position
                        if _buffered_raw_tell(self_) == (1i32.wrapping_neg() as i64) {
                            PyErr_Clear();
                        }
                    }
                }
                LEAVE_BUFFERED!(self_);
                return res;
            }
        }
    }
}

// [clinic end generated code: output=fe3882fbffe79f1a input=e3cbf794575bd794]
#[unsafe(link_section = ".text.slate_distinct.bufferedio.buffered_iternext")]
extern "C-unwind" fn buffered_iternext(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_264 = buffered_CAST!(op);
    let mut line: *mut _object = unsafe { core::mem::zeroed() };
    CHECK_INITIALIZED!(self_);
    let state: *mut _io_state = find_io_state_by_def(Py_TYPE__2!(self_));
    let tp: *mut _typeobject = Py_TYPE__2!(self_);
    if tp == unsafe { (*state).PyBufferedReader_Type }
        || tp == unsafe { (*state).PyBufferedRandom_Type }
    {
        // Skip method call overhead for speed
        line = _buffered_readline(self_, 1i32.wrapping_neg() as i64);
    } else {
        line = PyObject_CallMethodNoArgs(self_ as *mut _object, unsafe {
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
        let __v0: bool = !line.is_null() && PyBytes_Check!(line) == 0;
        if __v0 {
            unsafe {
                PyErr_Format(
                    PyExc_OSError,
                    c"readline() should have returned a bytes object, not '%.200s'".as_ptr(),
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
    if PyBytes_GET_SIZE!(line) == 0 {
        // Reached EOF or would have blocked
        Py_DECREF!(line);
        return core::ptr::null_mut::<_object>();
    }
    line
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.buffered_repr")]
extern "C-unwind" fn buffered_repr(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_264 = buffered_CAST!(op);
    let mut nameobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyObject_GetOptionalAttr(
            self_ as *mut _object,
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
        )
    }) < 0
    {
        if (unsafe { PyErr_ExceptionMatches(PyExc_ValueError) }) == 0 {
            return core::ptr::null_mut::<_object>();
        }
        // Ignore ValueError raised if the underlying stream was detached
        unsafe { PyErr_Clear() };
    }
    if nameobj.is_null() {
        res = unsafe { PyUnicode_FromFormat(c"<%s>".as_ptr(), (*Py_TYPE__2!(self_)).tp_name) };
    } else {
        let status: i32 = unsafe { Py_ReprEnter(self_ as *mut _object) };
        res = core::ptr::null_mut::<_object>();
        if status == 0 {
            res = unsafe {
                PyUnicode_FromFormat(
                    c"<%s name=%R>".as_ptr(),
                    (*Py_TYPE__2!(self_)).tp_name,
                    nameobj,
                )
            };
            unsafe { Py_ReprLeave(self_ as *mut _object) };
        } else {
            if status > 0 {
                unsafe {
                    PyErr_Format(
                        PyExc_RuntimeError,
                        c"reentrant call inside %s.__repr__".as_ptr(),
                        (*Py_TYPE__2!(self_)).tp_name,
                    )
                };
            }
        }
        Py_DECREF!(nameobj);
    }
    res
}

// class BufferedReader
fn _bufferedreader_reset_buf(self_: *mut __SlateRecord_bufferedio_c_264) {
    unsafe {
        (*self_).read_end = 1i32.wrapping_neg() as i64;
    }
}

// [clinic input]
// _io.BufferedReader.__init__
//     raw: object
//     buffer_size: Py_ssize_t(c_default="DEFAULT_BUFFER_SIZE") = DEFAULT_BUFFER_SIZE
//
// Create a new buffered reader using the given readable raw IO object.
// [clinic start generated code]
/// [clinic end generated code: output=cddcfefa0ed294c4 input=fb887e06f11b4e48]
fn _io_BufferedReader___init___impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    raw: *mut _object,
    buffer_size: i64,
) -> i32 {
    unsafe {
        (*self_).ok = 0;
    }
    unsafe {
        (*self_).detached = 0;
    }
    let state: *mut _io_state = find_io_state_by_def(Py_TYPE__2!(self_));
    if unsafe {
        _PyIOBase_check_readable(
            state,
            raw,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    Py_XSETREF!((*self_).raw, Py_NewRef!(raw));
    unsafe {
        (*self_).buffer_size = buffer_size;
    }
    unsafe {
        (*self_).readable = 1;
    }
    unsafe {
        (*self_).writable = 0;
    }
    if _buffered_init(self_) < 0 {
        return 1i32.wrapping_neg();
    }
    _bufferedreader_reset_buf(self_);
    let __v1: bool = Py_IS_TYPE__2!(self_, unsafe { (*state).PyBufferedReader_Type }) != 0
        && Py_IS_TYPE!(raw, unsafe { (*state).PyFileIO_Type }) != 0;
    unsafe {
        (*self_).fast_closed_checks = __v1 as i32;
    }
    unsafe {
        (*self_).ok = 1;
    }
    0
}

fn _bufferedreader_raw_read(
    self_: *mut __SlateRecord_bufferedio_c_264,
    start: *mut i8,
    len: i64,
) -> i64 {
    let mut buf: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = 0;
    // NOTE: the buffer needn't be released as its object is NULL.
    if (unsafe {
        PyBuffer_FillInfo(
            core::ptr::addr_of_mut!(buf),
            core::ptr::null_mut::<_object>(),
            start as *mut (),
            len,
            0i32,
            9i32,
        )
    }) == 1i32.wrapping_neg()
    {
        return 1i32.wrapping_neg() as i64;
    }
    let memobj: *mut _object = unsafe {
        PyMemoryView_FromBuffer(core::ptr::addr_of_mut!(buf) as *const __SlateRecord_pybuffer_h_33)
    };
    if memobj.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    // NOTE: PyErr_SetFromErrno() calls PyErr_CheckSignals() when EINTR
    // occurs so we needn't do it ourselves.
    // We then retry reading, ignoring the signal if no handler has
    // raised (see issue #10956).
    loop {
        res = PyObject_CallMethodOneArg(
            unsafe { (*self_).raw },
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_readinto
                        ._ascii
                        .ob_base
                )
            },
            memobj,
        );
        let __v1: bool = res.is_null() && _PyIO_trap_eintr() != 0;
        if !__v1 {
            break;
        }
    }
    Py_DECREF!(memobj);
    if res.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    if res == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        // Non-blocking stream would have blocked. Special return code!
        Py_DECREF!(res);
        return 2i32.wrapping_neg() as i64;
    }
    n = unsafe { PyNumber_AsSsize_t(res, PyExc_ValueError) };
    Py_DECREF!(res);
    let __v2: bool = n == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v2 {
        unsafe { _PyErr_FormatFromCause(PyExc_OSError, c"raw readinto() failed".as_ptr()) };
        return 1i32.wrapping_neg() as i64;
    }
    if n < 0 || n > len {
        unsafe {
            PyErr_Format(
                PyExc_OSError,
                c"raw readinto() returned invalid length %zd (should have been between 0 and %zd)"
                    .as_ptr(),
                n,
                len,
            )
        };
        return 1i32.wrapping_neg() as i64;
    }
    if n > 0 && (unsafe { (*self_).abs_pos }) != (1i32.wrapping_neg() as i64) {
        let __v4: i64 = unsafe { (*self_).abs_pos };
        unsafe {
            (*self_).abs_pos = __v4.wrapping_add(n);
        }
    }
    n
}

fn _bufferedreader_fill_buffer(self_: *mut __SlateRecord_bufferedio_c_264) -> i64 {
    let mut start: i64 = 0;
    let mut len: i64 = 0;
    let mut n: i64 = 0;
    if VALID_READ_BUFFER!(self_) {
        start = Py_SAFE_DOWNCAST!(unsafe { (*self_).read_end }, i64);
    } else {
        start = 0;
    }
    len = unsafe { (*self_).buffer_size }.wrapping_sub(start);
    n = _bufferedreader_raw_read(
        self_,
        unsafe { (*self_).buffer }.wrapping_offset(start as isize),
        len,
    );
    if n <= 0 {
        return n;
    }
    unsafe {
        (*self_).read_end = start.wrapping_add(n);
    }
    unsafe {
        (*self_).raw_pos = start.wrapping_add(n);
    }
    n
}

fn _bufferedreader_read_all(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut __v9: bool = unsafe { core::mem::zeroed() };
    let mut readall: *mut _object = unsafe { core::mem::zeroed() };
    let mut chunks: *mut _object = unsafe { core::mem::zeroed() };
    let mut tmp: *mut _object = unsafe { core::mem::zeroed() };
    let mut data: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut current_size: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        data = core::ptr::null_mut::<_object>();
        tmp = core::ptr::null_mut::<_object>();
        chunks = core::ptr::null_mut::<_object>();
        // First copy what we have in the current buffer.
        current_size = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
        if current_size != 0 {
            data = PyBytes_FromStringAndSize(
                unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize) as *const i8,
                current_size,
            );
            if data.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                let __v11: i64 = (*self_).pos;
                (*self_).pos = __v11.wrapping_add(current_size);
            }
        }
        '__join_0: {
            // We're going past the buffer's bounds, flush it
            if (*self_).writable != 0 {
                tmp = buffered_flush_and_rewind_unlocked(self_);
                if tmp.is_null() {
                    break '__join_0;
                } else {
                    Py_CLEAR__2!(tmp);
                }
            }
            _bufferedreader_reset_buf(self_);
            if PyObject_GetOptionalAttr(
                (*self_).raw,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_readall
                        ._ascii
                        .ob_base
                ),
                core::ptr::addr_of_mut!(readall),
            ) >= 0
            {
                if !readall.is_null() {
                    tmp = _PyObject_CallNoArgs(readall);
                    Py_DECREF!(readall);
                    if !tmp.is_null() {
                        if tmp != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            __v9 = PyBytes_Check!(tmp) == 0;
                        } else {
                            __v9 = false;
                        }
                        if __v9 {
                            PyErr_SetString(
                                PyExc_TypeError,
                                c"readall() should return bytes".as_ptr(),
                            );
                        } else {
                            if current_size == 0 {
                                res = tmp;
                            } else {
                                if tmp != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                    PyBytes_Concat(core::ptr::addr_of_mut!(data), tmp);
                                }
                                res = data;
                            }
                        }
                    }
                } else {
                    chunks = PyList_New(0i64);
                    if !chunks.is_null() {
                        '__join_11: {
                            loop {
                                if true {
                                    if !data.is_null() {
                                        if PyList_Append(chunks, data) < 0 {
                                            break '__join_0;
                                        } else {
                                            Py_CLEAR__2!(data);
                                        }
                                    }
                                    // Read until EOF or until read() would block.
                                    data = PyObject_CallMethodNoArgs(
                                        (*self_).raw,
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
                                    if data.is_null() {
                                        break '__join_0;
                                    } else {
                                        if data != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                            __v8 = PyBytes_Check!(data) == 0;
                                        } else {
                                            __v8 = false;
                                        }
                                        if __v8 {
                                            break '__join_11;
                                        } else {
                                            if data == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                                __v7 = true;
                                            } else {
                                                __v7 = PyBytes_GET_SIZE!(data) == 0;
                                            }
                                            if __v7 {
                                                break;
                                            } else {
                                                let __v6: i64 = PyBytes_GET_SIZE!(data);
                                                current_size = current_size.wrapping_add(__v6);
                                                if (*self_).abs_pos != (1i32.wrapping_neg() as i64)
                                                {
                                                    let __v3: i64 = PyBytes_GET_SIZE!(data);
                                                    let __v1: i64 = (*self_).abs_pos;
                                                    (*self_).abs_pos = __v1.wrapping_add(__v3);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    break '__join_0;
                                }
                            }
                            if current_size == 0 {
                                res = data;
                                break '__join_0;
                            } else {
                                tmp = PyBytes_Join(
                                    core::ptr::addr_of_mut!(
                                        _PyRuntime.static_objects.singletons.bytes_empty
                                    ) as *mut _object,
                                    chunks,
                                );
                                res = tmp;
                                break '__join_0;
                            }
                        }
                        PyErr_SetString(PyExc_TypeError, c"read() should return bytes".as_ptr());
                    }
                }
            }
        }
        Py_XINCREF!(res); // res is either NULL or a borrowed ref
        Py_XDECREF!(data);
        Py_XDECREF!(tmp);
        Py_XDECREF!(chunks);
        return res;
    }
}

/// Read n bytes from the buffer if it can, otherwise return None.
/// This function is simple enough that it can run unlocked.
fn _bufferedreader_read_fast(self_: *mut __SlateRecord_bufferedio_c_264, n: i64) -> *mut _object {
    let current_size: i64 = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
    if n <= current_size {
        // Fast path: the data to read is fully buffered.
        let res: *mut _object = unsafe {
            PyBytes_FromStringAndSize(
                unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize) as *const i8,
                n,
            )
        };
        if !res.is_null() {
            let __v1: i64 = unsafe { (*self_).pos };
            unsafe {
                (*self_).pos = __v1.wrapping_add(n);
            }
        }
        return res;
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

/// Generic read function: read from the stream until enough bytes are read,
/// or until an EOF occurs or until read() would block.
fn _bufferedreader_read_generic(
    self_: *mut __SlateRecord_bufferedio_c_264,
    n: i64,
) -> *mut _object {
    let mut r: i64 = unsafe { core::mem::zeroed() };
    // We want to read a whole block at the end into buffer.
    // If we had readv() we could do this in one pass.
    let mut r_1: i64 = unsafe { core::mem::zeroed() };
    let mut r_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut out: *mut i8 = unsafe { core::mem::zeroed() };
    let mut written: i64 = unsafe { core::mem::zeroed() };
    let mut remaining: i64 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        let current_size: i64 = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
        if n <= current_size {
            return _bufferedreader_read_fast(self_, n);
        } else {
            '__join_0: {
                res = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), n);
                if !res.is_null() {
                    out = PyBytes_AS_STRING!(res);
                    remaining = n;
                    written = 0;
                    if current_size > 0 {
                        core::ptr::copy_nonoverlapping(
                            ((unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize)
                                as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((out as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                            ((current_size as u64) as usize) as usize,
                        );
                        remaining = remaining.wrapping_sub(current_size);
                        written = written.wrapping_add(current_size);
                        let __v20: i64 = (*self_).pos;
                        (*self_).pos = __v20.wrapping_add(current_size);
                    }
                    // Flush the write buffer if necessary
                    if (*self_).writable != 0 {
                        r_2 = buffered_flush_and_rewind_unlocked(self_);
                        if r_2.is_null() {
                            break '__join_0;
                        } else {
                            Py_DECREF!(r_2);
                        }
                    }
                    _bufferedreader_reset_buf(self_);
                    '__join_14: {
                        loop {
                            if remaining > 0 {
                                r_1 = MINUS_LAST_BLOCK!(self_, remaining);
                                if r_1 == 0 {
                                    break '__join_14;
                                } else {
                                    r_1 = _bufferedreader_raw_read(
                                        self_,
                                        out.wrapping_offset(written as isize),
                                        r_1,
                                    );
                                    if r_1 == (1i32.wrapping_neg() as i64) {
                                        break '__join_0;
                                    } else {
                                        if r_1 == 0 || r_1 == (2i32.wrapping_neg() as i64) {
                                            break;
                                        } else {
                                            remaining = remaining.wrapping_sub(r_1);
                                            written = written.wrapping_add(r_1);
                                        }
                                    }
                                }
                            } else {
                                break '__join_14;
                            }
                        }
                        // EOF occurred or read() would block.
                        if r_1 == 0 || written > 0 {
                            if _PyBytes_Resize(core::ptr::addr_of_mut!(res), written) != 0 {
                                break '__join_0;
                            } else {
                                return res;
                            }
                        } else {
                            Py_DECREF!(res);
                            return core::ptr::addr_of_mut!(_Py_NoneStruct);
                        }
                    }
                    (*self_).pos = 0;
                    (*self_).raw_pos = 0;
                    (*self_).read_end = 0;
                    // NOTE: when the read is satisfied, we avoid issuing any additional
                    // reads, which could block indefinitely (e.g. on a socket).
                    // See issue #9550.
                    '__join_11: {
                        while remaining > 0 && (*self_).read_end < (*self_).buffer_size {
                            r = _bufferedreader_fill_buffer(self_);
                            if r == (1i32.wrapping_neg() as i64) {
                                break '__join_0;
                            } else {
                                if r == 0 || r == (2i32.wrapping_neg() as i64) {
                                    break '__join_11;
                                } else {
                                    if remaining > r {
                                        core::ptr::copy_nonoverlapping(
                                            ((unsafe { (*self_).buffer }
                                                .wrapping_offset((*self_).pos as isize)
                                                as *const ())
                                                as *const core::ffi::c_void)
                                                as *const u8,
                                            ((out.wrapping_offset(written as isize) as *mut ())
                                                as *mut core::ffi::c_void)
                                                as *mut u8,
                                            ((r as u64) as usize) as usize,
                                        );
                                        written = written.wrapping_add(r);
                                        let __v3: i64 = (*self_).pos;
                                        (*self_).pos = __v3.wrapping_add(r);
                                        remaining = remaining.wrapping_sub(r);
                                    } else {
                                        if remaining > 0 {
                                            core::ptr::copy_nonoverlapping(
                                                ((unsafe { (*self_).buffer }
                                                    .wrapping_offset((*self_).pos as isize)
                                                    as *const ())
                                                    as *const core::ffi::c_void)
                                                    as *const u8,
                                                ((out.wrapping_offset(written as isize) as *mut ())
                                                    as *mut core::ffi::c_void)
                                                    as *mut u8,
                                                ((remaining as u64) as usize) as usize,
                                            );
                                            written = written.wrapping_add(remaining);
                                            let __v8: i64 = (*self_).pos;
                                            (*self_).pos = __v8.wrapping_add(remaining);
                                            remaining = 0;
                                        }
                                    }
                                    if remaining == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        return res;
                    }
                    // EOF occurred or read() would block.
                    if r == 0 || written > 0 {
                        if _PyBytes_Resize(core::ptr::addr_of_mut!(res), written) == 0 {
                            return res;
                        }
                    } else {
                        Py_DECREF!(res);
                        return core::ptr::addr_of_mut!(_Py_NoneStruct);
                    }
                }
            }
            Py_XDECREF!(res);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn _bufferedreader_peek_unlocked(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let have: i64 = Py_SAFE_DOWNCAST!(READAHEAD!(self_), i64);
    let mut r: i64 = 0;
    // Constraints:
    // 1. we don't want to advance the file position.
    // 2. we don't want to lose block alignment, so we can't shift the buffer
    //    to make some place.
    // Therefore, we either return `have` bytes (if > 0), or a full buffer.
    if have > 0 {
        return unsafe {
            PyBytes_FromStringAndSize(
                unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize) as *const i8,
                have,
            )
        };
    }
    // Fill the buffer from the raw stream, and copy it to the result.
    _bufferedreader_reset_buf(self_);
    r = _bufferedreader_fill_buffer(self_);
    if r == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if r == (2i32.wrapping_neg() as i64) {
        r = 0;
    }
    unsafe {
        (*self_).pos = 0;
    }
    unsafe { PyBytes_FromStringAndSize((*self_).buffer as *const i8, r) }
}

/// class BufferedWriter
fn _bufferedwriter_reset_buf(self_: *mut __SlateRecord_bufferedio_c_264) {
    unsafe {
        (*self_).write_pos = 0;
    }
    unsafe {
        (*self_).write_end = 1i32.wrapping_neg() as i64;
    }
}

// [clinic input]
// _io.BufferedWriter.__init__
//     raw: object
//     buffer_size: Py_ssize_t(c_default="DEFAULT_BUFFER_SIZE") = DEFAULT_BUFFER_SIZE
//
// A buffer for a writeable sequential RawIO object.
//
// The constructor creates a BufferedWriter for the given writeable raw
// stream. If the buffer_size is not given, it defaults to
// DEFAULT_BUFFER_SIZE.
// [clinic start generated code]
/// [clinic end generated code: output=c8942a020c0dee64 input=914be9b95e16007b]
fn _io_BufferedWriter___init___impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    raw: *mut _object,
    buffer_size: i64,
) -> i32 {
    unsafe {
        (*self_).ok = 0;
    }
    unsafe {
        (*self_).detached = 0;
    }
    let state: *mut _io_state = find_io_state_by_def(Py_TYPE__2!(self_));
    if unsafe {
        _PyIOBase_check_writable(
            state,
            raw,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    Py_INCREF!(raw);
    Py_XSETREF!((*self_).raw, raw);
    unsafe {
        (*self_).readable = 0;
    }
    unsafe {
        (*self_).writable = 1;
    }
    unsafe {
        (*self_).buffer_size = buffer_size;
    }
    if _buffered_init(self_) < 0 {
        return 1i32.wrapping_neg();
    }
    _bufferedwriter_reset_buf(self_);
    unsafe {
        (*self_).pos = 0;
    }
    let __v1: bool = Py_IS_TYPE__2!(self_, unsafe { (*state).PyBufferedWriter_Type }) != 0
        && Py_IS_TYPE!(raw, unsafe { (*state).PyFileIO_Type }) != 0;
    unsafe {
        (*self_).fast_closed_checks = __v1 as i32;
    }
    unsafe {
        (*self_).ok = 1;
    }
    0
}

fn _bufferedwriter_raw_write(
    self_: *mut __SlateRecord_bufferedio_c_264,
    start: *mut i8,
    len: i64,
) -> i64 {
    let mut buf: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = 0;
    let mut errnum: i32 = 0;
    // NOTE: the buffer needn't be released as its object is NULL.
    if (unsafe {
        PyBuffer_FillInfo(
            core::ptr::addr_of_mut!(buf),
            core::ptr::null_mut::<_object>(),
            start as *mut (),
            len,
            1i32,
            8i32,
        )
    }) == 1i32.wrapping_neg()
    {
        return 1i32.wrapping_neg() as i64;
    }
    let memobj: *mut _object = unsafe {
        PyMemoryView_FromBuffer(core::ptr::addr_of_mut!(buf) as *const __SlateRecord_pybuffer_h_33)
    };
    if memobj.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    // NOTE: PyErr_SetFromErrno() calls PyErr_CheckSignals() when EINTR
    // occurs so we needn't do it ourselves.
    // We then retry writing, ignoring the signal if no handler has
    // raised (see issue #10956).
    loop {
        unsafe {
            *slate_support::errno_location() = 0;
        }
        res = PyObject_CallMethodOneArg(
            unsafe { (*self_).raw },
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
            memobj,
        );
        errnum = unsafe { *slate_support::errno_location() };
        let __v1: bool = res.is_null() && _PyIO_trap_eintr() != 0;
        if !__v1 {
            break;
        }
    }
    Py_DECREF!(memobj);
    if res.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    if res == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        // Non-blocking stream would have blocked. Special return code!
        // Being paranoid we reset errno in case it is changed by code
        // triggered by a decref.  errno is used by _set_BlockingIOError().
        Py_DECREF!(res);
        unsafe {
            *slate_support::errno_location() = errnum;
        }
        return 2i32.wrapping_neg() as i64;
    }
    n = unsafe { PyNumber_AsSsize_t(res, PyExc_ValueError) };
    Py_DECREF!(res);
    if n < 0 || n > len {
        unsafe {
            PyErr_Format(
                PyExc_OSError,
                c"raw write() returned invalid length %zd (should have been between 0 and %zd)"
                    .as_ptr(),
                n,
                len,
            )
        };
        return 1i32.wrapping_neg() as i64;
    }
    if n > 0 && (unsafe { (*self_).abs_pos }) != (1i32.wrapping_neg() as i64) {
        let __v3: i64 = unsafe { (*self_).abs_pos };
        unsafe {
            (*self_).abs_pos = __v3.wrapping_add(n);
        }
    }
    n
}

/// Forward decls
fn _bufferedwriter_flush_unlocked(self_: *mut __SlateRecord_bufferedio_c_264) -> *mut _object {
    let mut rewind: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_1: {
            if !(!VALID_WRITE_BUFFER!(self_) || (*self_).write_pos == (*self_).write_end) {
                '__join_0: {
                    // First, rewind
                    rewind = RAW_OFFSET!(self_)
                        .wrapping_add(unsafe { (*self_).pos }.wrapping_sub((*self_).write_pos));
                    if rewind != 0 {
                        n = _buffered_raw_seek(self_, rewind.wrapping_neg(), 1i32);
                        if n < 0 {
                            break '__join_0;
                        } else {
                            let __v4: i64 = (*self_).raw_pos;
                            (*self_).raw_pos = __v4.wrapping_sub(rewind);
                        }
                    }
                    loop {
                        if (*self_).write_pos < (*self_).write_end {
                            n = _bufferedwriter_raw_write(
                                self_,
                                unsafe { (*self_).buffer }
                                    .wrapping_offset((*self_).write_pos as isize),
                                Py_SAFE_DOWNCAST!(
                                    unsafe { (*self_).write_end }.wrapping_sub((*self_).write_pos),
                                    i64
                                ),
                            );
                            if n == (1i32.wrapping_neg() as i64) {
                                break '__join_0;
                            } else {
                                if n == (2i32.wrapping_neg() as i64) {
                                    break;
                                } else {
                                    let __v1: i64 = (*self_).write_pos;
                                    (*self_).write_pos = __v1.wrapping_add(n);
                                    (*self_).raw_pos = (*self_).write_pos;
                                    // Partial writes can return successfully when interrupted by a
                                    // signal (see write(2)).  We must run signal handlers before
                                    // blocking another time, possibly indefinitely.
                                    if PyErr_CheckSignals() < 0 {
                                        break '__join_0;
                                    }
                                }
                            }
                        } else {
                            break '__join_1;
                        }
                    }
                    _set_BlockingIOError(
                        c"write could not complete without blocking".as_ptr(),
                        0i64,
                    );
                }
                return core::ptr::null_mut::<_object>();
            }
        }
        _bufferedwriter_reset_buf(self_);
        // This ensures that after return from this function,
        // VALID_WRITE_BUFFER(self) returns false.
        //
        // This is a required condition because when a tell() is called
        // after flushing and if VALID_READ_BUFFER(self) is false, we need
        // VALID_WRITE_BUFFER(self) to be false to have
        // RAW_OFFSET(self) == 0.
        //
        // Issue: https://bugs.python.org/issue32228
        return core::ptr::addr_of_mut!(_Py_NoneStruct);
    }
}

// [clinic input]
// @critical_section
// _io.BufferedWriter.write
//     buffer: Py_buffer
//     /
// [clinic start generated code]
fn _io_BufferedWriter_write_impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    buffer: *mut __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut offset: i64 = unsafe { core::mem::zeroed() };
    let mut remaining: i64 = unsafe { core::mem::zeroed() };
    let mut avail: i64 = unsafe { core::mem::zeroed() };
    let mut written: i64 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        res = core::ptr::null_mut::<_object>();
        CHECK_INITIALIZED!(self_);
        if !ENTER_BUFFERED!(self_) {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                // Issue #31976: Check for closed file after acquiring the lock. Another
                // thread could be holding the lock while closing the file.
                if IS_CLOSED!(self_) {
                    PyErr_SetString(PyExc_ValueError, c"write to closed file".as_ptr());
                } else {
                    // Fast path: the data to write can be fully buffered.
                    if !VALID_READ_BUFFER!(self_) && !VALID_WRITE_BUFFER!(self_) {
                        (*self_).pos = 0;
                        (*self_).raw_pos = 0;
                    }
                    avail = Py_SAFE_DOWNCAST!(
                        unsafe { (*self_).buffer_size }.wrapping_sub((*self_).pos),
                        i64
                    );
                    if (*buffer).len <= avail && (*buffer).len < (*self_).buffer_size {
                        core::ptr::copy_nonoverlapping(
                            (((*buffer).buf as *const ()) as *const core::ffi::c_void) as *const u8,
                            ((unsafe { (*self_).buffer }.wrapping_offset((*self_).pos as isize)
                                as *mut ()) as *mut core::ffi::c_void)
                                as *mut u8,
                            (((*buffer).len as u64) as usize) as usize,
                        );
                        if !VALID_WRITE_BUFFER!(self_) || (*self_).write_pos > (*self_).pos {
                            (*self_).write_pos = (*self_).pos;
                        }
                        ADJUST_POSITION!(
                            self_,
                            unsafe { (*self_).pos }.wrapping_add((*buffer).len)
                        );
                        if (*self_).pos > (*self_).write_end {
                            (*self_).write_end = (*self_).pos;
                        }
                        written = (*buffer).len;
                    } else {
                        // First write the current buffer
                        res = _bufferedwriter_flush_unlocked(self_);
                        if res.is_null() {
                            let w: *mut i64 = _buffered_check_blocking_error();
                            if w.is_null() {
                                break '__join_0;
                            } else {
                                if (*self_).readable != 0 {
                                    _bufferedreader_reset_buf(self_);
                                }
                                // Make some place by shifting the buffer.
                                core::ptr::copy(
                                    ((unsafe { (*self_).buffer }
                                        .wrapping_offset((*self_).write_pos as isize)
                                        as *const ())
                                        as *const core::ffi::c_void)
                                        as *const u8,
                                    (((*self_).buffer as *mut ()) as *mut core::ffi::c_void)
                                        as *mut u8,
                                    ((Py_SAFE_DOWNCAST!(
                                        unsafe { (*self_).write_end }
                                            .wrapping_sub((*self_).write_pos),
                                        i64
                                    ) as u64) as usize)
                                        as usize,
                                );
                                let __v31: i64 = (*self_).write_end;
                                (*self_).write_end = __v31.wrapping_sub((*self_).write_pos);
                                let __v28: i64 = (*self_).raw_pos;
                                (*self_).raw_pos = __v28.wrapping_sub((*self_).write_pos);
                                let __v25: i64 = (*self_).pos;
                                (*self_).pos = __v25.wrapping_sub((*self_).write_pos);
                                (*self_).write_pos = 0;
                                avail = Py_SAFE_DOWNCAST!(
                                    unsafe { (*self_).buffer_size }
                                        .wrapping_sub((*self_).write_end),
                                    i64
                                );
                                if (*buffer).len <= avail {
                                    // Everything can be buffered
                                    PyErr_Clear();
                                    core::ptr::copy_nonoverlapping(
                                        (((*buffer).buf as *const ()) as *const core::ffi::c_void)
                                            as *const u8,
                                        ((unsafe { (*self_).buffer }
                                            .wrapping_offset((*self_).write_end as isize)
                                            as *mut ())
                                            as *mut core::ffi::c_void)
                                            as *mut u8,
                                        (((*buffer).len as u64) as usize) as usize,
                                    );
                                    let __v22: i64 = (*self_).write_end;
                                    (*self_).write_end = __v22.wrapping_add((*buffer).len);
                                    let __v19: i64 = (*self_).pos;
                                    (*self_).pos = __v19.wrapping_add((*buffer).len);
                                    written = (*buffer).len;
                                } else {
                                    // Buffer as much as possible.
                                    core::ptr::copy_nonoverlapping(
                                        (((*buffer).buf as *const ()) as *const core::ffi::c_void)
                                            as *const u8,
                                        ((unsafe { (*self_).buffer }
                                            .wrapping_offset((*self_).write_end as isize)
                                            as *mut ())
                                            as *mut core::ffi::c_void)
                                            as *mut u8,
                                        ((avail as u64) as usize) as usize,
                                    );
                                    let __v16: i64 = (*self_).write_end;
                                    (*self_).write_end = __v16.wrapping_add(avail);
                                    let __v13: i64 = (*self_).pos;
                                    (*self_).pos = __v13.wrapping_add(avail);
                                    // XXX Modifying the existing exception e using the pointer w
                                    // will change e.characters_written but not e.args[2].
                                    // Therefore we just replace with a new error.
                                    _set_BlockingIOError(
                                        c"write could not complete without blocking".as_ptr(),
                                        avail,
                                    );
                                    break '__join_0;
                                }
                            }
                        } else {
                            Py_CLEAR__2!(res);
                            // Adjust the raw stream position if it is away from the logical stream
                            // position. This happens if the read buffer has been filled but not
                            // modified (and therefore _bufferedwriter_flush_unlocked() didn't rewind
                            // the raw stream by itself).
                            // Fixes issue #6629.
                            offset = RAW_OFFSET!(self_);
                            if offset != 0 {
                                if _buffered_raw_seek(self_, offset.wrapping_neg(), 1i32) < 0 {
                                    break '__join_0;
                                } else {
                                    let __v10: i64 = (*self_).raw_pos;
                                    (*self_).raw_pos = __v10.wrapping_sub(offset);
                                }
                            }
                            // Then write buf itself. At this point the buffer has been emptied.
                            remaining = (*buffer).len;
                            written = 0;
                            '__join_6: {
                                loop {
                                    if remaining >= (*self_).buffer_size {
                                        n = _bufferedwriter_raw_write(
                                            self_,
                                            ((*buffer).buf as *mut i8)
                                                .wrapping_offset(written as isize),
                                            unsafe { (*buffer).len }.wrapping_sub(written),
                                        );
                                        if n == (1i32.wrapping_neg() as i64) {
                                            break '__join_0;
                                        } else {
                                            if n == (2i32.wrapping_neg() as i64) {
                                                break;
                                            } else {
                                                written = written.wrapping_add(n);
                                                remaining = remaining.wrapping_sub(n);
                                                // Partial writes can return successfully when interrupted by a
                                                // signal (see write(2)).  We must run signal handlers before
                                                // blocking another time, possibly indefinitely.
                                                if PyErr_CheckSignals() < 0 {
                                                    break '__join_0;
                                                }
                                            }
                                        }
                                    } else {
                                        break '__join_6;
                                    }
                                }
                                // Write failed because raw file is non-blocking
                                if remaining > (*self_).buffer_size {
                                    // Can't buffer everything, still buffer as much as possible
                                    core::ptr::copy_nonoverlapping(
                                        ((((*buffer).buf as *mut i8)
                                            .wrapping_offset(written as isize)
                                            as *const ())
                                            as *const core::ffi::c_void)
                                            as *const u8,
                                        (((*self_).buffer as *mut ()) as *mut core::ffi::c_void)
                                            as *mut u8,
                                        (((*self_).buffer_size as u64) as usize) as usize,
                                    );
                                    (*self_).raw_pos = 0;
                                    ADJUST_POSITION!(self_, unsafe { (*self_).buffer_size });
                                    (*self_).write_end = (*self_).buffer_size;
                                    written = written.wrapping_add((*self_).buffer_size);
                                    _set_BlockingIOError(
                                        c"write could not complete without blocking".as_ptr(),
                                        written,
                                    );
                                    break '__join_0;
                                } else {
                                    PyErr_Clear();
                                }
                            }
                            if (*self_).readable != 0 {
                                _bufferedreader_reset_buf(self_);
                            }
                            if remaining > 0 {
                                core::ptr::copy_nonoverlapping(
                                    ((((*buffer).buf as *mut i8).wrapping_offset(written as isize)
                                        as *const ())
                                        as *const core::ffi::c_void)
                                        as *const u8,
                                    (((*self_).buffer as *mut ()) as *mut core::ffi::c_void)
                                        as *mut u8,
                                    ((remaining as u64) as usize) as usize,
                                );
                                written = written.wrapping_add(remaining);
                            }
                            (*self_).write_pos = 0;
                            // TODO: sanity check (remaining >= 0)
                            (*self_).write_end = remaining;
                            ADJUST_POSITION!(self_, remaining);
                            (*self_).raw_pos = 0;
                        }
                    }
                    res = PyLong_FromSsize_t(written);
                }
            }
            LEAVE_BUFFERED!(self_);
            return res;
        }
    }
}

// BufferedRWPair
// XXX The usefulness of this (compared to having two separate IO objects) is
// questionable.
// [clinic end generated code: output=7f8d1365759bfc6b input=6a9c041de0c337be]
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bufferedio_c_2238 {
    ob_base: _object,
    reader: *mut __SlateRecord_bufferedio_c_264,
    writer: *mut __SlateRecord_bufferedio_c_264,
    dict: *mut _object,
    weakreflist: *mut _object,
}

// [clinic input]
// _io.BufferedRWPair.__init__
//     reader: object
//     writer: object
//     buffer_size: Py_ssize_t(c_default="DEFAULT_BUFFER_SIZE") = DEFAULT_BUFFER_SIZE
//     /
//
// A buffered reader and writer object together.
//
// A buffered reader object and buffered writer object put together to
// form a sequential IO object that can read and write. This is typically
// used with a socket or two-way pipe.
//
// reader and writer are RawIOBase objects that are readable and
// writeable respectively. If the buffer_size is omitted it defaults to
// DEFAULT_BUFFER_SIZE.
// [clinic start generated code]
fn _io_BufferedRWPair___init___impl(
    self_: *mut __SlateRecord_bufferedio_c_2238,
    reader: *mut _object,
    writer: *mut _object,
    buffer_size: i64,
) -> i32 {
    let state: *mut _io_state = find_io_state_by_def(Py_TYPE__2!(self_));
    if unsafe {
        _PyIOBase_check_readable(
            state,
            reader,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    if unsafe {
        _PyIOBase_check_writable(
            state,
            writer,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*self_).reader = PyObject_CallFunction(
            (*state).PyBufferedReader_Type as *mut _object,
            c"On".as_ptr(),
            reader,
            buffer_size,
        ) as *mut __SlateRecord_bufferedio_c_264;
    }
    if unsafe { (*self_).reader }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*self_).writer = PyObject_CallFunction(
            (*state).PyBufferedWriter_Type as *mut _object,
            c"On".as_ptr(),
            writer,
            buffer_size,
        ) as *mut __SlateRecord_bufferedio_c_264;
    }
    if unsafe { (*self_).writer }.is_null() {
        Py_CLEAR__8!((*self_).reader);
        return 1i32.wrapping_neg();
    }
    0
}

// [clinic end generated code: output=327e73d1aee8f984 input=620d42d71f33a031]
#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_traverse")]
extern "C-unwind" fn bufferedrwpair_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(self_ as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*self_).dict }, visit, arg);
    Py_VISIT__6!(unsafe { (*self_).reader }, visit, arg);
    Py_VISIT__6!(unsafe { (*self_).writer }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_clear")]
extern "C-unwind" fn bufferedrwpair_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    Py_CLEAR__8!((*self_).reader);
    Py_CLEAR__8!((*self_).writer);
    Py_CLEAR!((*self_).dict);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_dealloc")]
extern "C-unwind" fn bufferedrwpair_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(self_);
    _PyObject_GC_UNTRACK__2!(self_);
    FT_CLEAR_WEAKREFS!(op, unsafe { (*self_).weakreflist });
    bufferedrwpair_clear(op);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(self_ as *mut ()) };
    Py_DECREF__2!(tp);
}

fn _forward_call(
    self_: *mut __SlateRecord_bufferedio_c_264,
    name: *mut _object,
    args: *mut _object,
) -> *mut _object {
    if self_.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"I/O operation on uninitialized object".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let func: *mut _object = unsafe { PyObject_GetAttr(self_ as *mut _object, name) };
    if func.is_null() {
        unsafe { PyErr_SetObject(PyExc_AttributeError, name) };
        return core::ptr::null_mut::<_object>();
    }
    let ret: *mut _object = unsafe { PyObject_CallObject(func, args) };
    Py_DECREF!(func);
    ret
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_read")]
extern "C-unwind" fn bufferedrwpair_read(op: *mut _object, args: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
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
        args,
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_peek")]
extern "C-unwind" fn bufferedrwpair_peek(op: *mut _object, args: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_peek
                    ._ascii
                    .ob_base
            )
        },
        args,
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_read1")]
extern "C-unwind" fn bufferedrwpair_read1(op: *mut _object, args: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
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
        },
        args,
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_readinto")]
extern "C-unwind" fn bufferedrwpair_readinto(op: *mut _object, args: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_readinto
                    ._ascii
                    .ob_base
            )
        },
        args,
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_readinto1")]
extern "C-unwind" fn bufferedrwpair_readinto1(
    op: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_readinto1
                    ._ascii
                    .ob_base
            )
        },
        args,
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_write")]
extern "C-unwind" fn bufferedrwpair_write(op: *mut _object, args: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).writer },
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
        args,
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_flush")]
extern "C-unwind" fn bufferedrwpair_flush(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).writer },
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
        core::ptr::null_mut::<_object>(),
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_readable")]
extern "C-unwind" fn bufferedrwpair_readable(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
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
        },
        core::ptr::null_mut::<_object>(),
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_writable")]
extern "C-unwind" fn bufferedrwpair_writable(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    _forward_call(
        unsafe { (*self_).writer },
        unsafe {
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
        },
        core::ptr::null_mut::<_object>(),
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_close")]
extern "C-unwind" fn bufferedrwpair_close(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    let mut exc: *mut _object = core::ptr::null_mut::<_object>();
    let mut ret: *mut _object = _forward_call(
        unsafe { (*self_).writer },
        unsafe {
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
        },
        core::ptr::null_mut::<_object>(),
    );
    if ret.is_null() {
        exc = unsafe { PyErr_GetRaisedException() };
    } else {
        Py_DECREF!(ret);
    }
    ret = _forward_call(
        unsafe { (*self_).reader },
        unsafe {
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
        },
        core::ptr::null_mut::<_object>(),
    );
    if !exc.is_null() {
        unsafe { _PyErr_ChainExceptions1(exc) };
        Py_CLEAR__2!(ret);
    }
    ret
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_isatty")]
extern "C-unwind" fn bufferedrwpair_isatty(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    let ret: *mut _object = _forward_call(
        unsafe { (*self_).writer },
        unsafe {
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
        },
        core::ptr::null_mut::<_object>(),
    );
    if ret != _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) }) {
        // either True or exception
        return ret;
    }
    Py_DECREF!(ret);
    _forward_call(
        unsafe { (*self_).reader },
        unsafe {
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
        },
        core::ptr::null_mut::<_object>(),
    )
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio.bufferedrwpair_closed_get")]
extern "C-unwind" fn bufferedrwpair_closed_get(
    op: *mut _object,
    _unused_dummy: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_bufferedio_c_2238 = rwpair_CAST!(op);
    if unsafe { (*self_).writer }.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_RuntimeError,
                c"the BufferedRWPair object is being garbage-collected".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        PyObject_GetAttr(
            (*self_).writer as *mut _object,
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

// BufferedRandom
// [clinic input]
// _io.BufferedRandom.__init__
//     raw: object
//     buffer_size: Py_ssize_t(c_default="DEFAULT_BUFFER_SIZE") = DEFAULT_BUFFER_SIZE
//
// A buffered interface to random access streams.
//
// The constructor creates a reader and writer for a seekable stream,
// raw, given in the first argument. If the buffer_size is omitted it
// defaults to DEFAULT_BUFFER_SIZE.
// [clinic start generated code]
/// [clinic end generated code: output=d3d64eb0f64e64a3 input=a4e818fb86d0e50c]
fn _io_BufferedRandom___init___impl(
    self_: *mut __SlateRecord_bufferedio_c_264,
    raw: *mut _object,
    buffer_size: i64,
) -> i32 {
    unsafe {
        (*self_).ok = 0;
    }
    unsafe {
        (*self_).detached = 0;
    }
    let state: *mut _io_state = find_io_state_by_def(Py_TYPE__2!(self_));
    if unsafe {
        _PyIOBase_check_seekable(
            state,
            raw,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    if unsafe {
        _PyIOBase_check_readable(
            state,
            raw,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    if unsafe {
        _PyIOBase_check_writable(
            state,
            raw,
            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
        )
    }
    .is_null()
    {
        return 1i32.wrapping_neg();
    }
    Py_INCREF!(raw);
    Py_XSETREF!((*self_).raw, raw);
    unsafe {
        (*self_).buffer_size = buffer_size;
    }
    unsafe {
        (*self_).readable = 1;
    }
    unsafe {
        (*self_).writable = 1;
    }
    if _buffered_init(self_) < 0 {
        return 1i32.wrapping_neg();
    }
    _bufferedreader_reset_buf(self_);
    _bufferedwriter_reset_buf(self_);
    unsafe {
        (*self_).pos = 0;
    }
    let __v1: bool = Py_IS_TYPE__2!(self_, unsafe { (*state).PyBufferedRandom_Type }) != 0
        && Py_IS_TYPE!(raw, unsafe { (*state).PyFileIO_Type }) != 0;
    unsafe {
        (*self_).fast_closed_checks = __v1 as i32;
    }
    unsafe {
        (*self_).ok = 1;
    }
    0
}

static _io__BufferedIOBase_readinto__doc__: __SlateAlign16<[i8; 32]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 116i8, 111i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__BufferedIOBase_readinto")]
extern "C-unwind" fn _io__BufferedIOBase_readinto(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut buffer: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(buffer) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 1i32) < 0 {
            _PyArg_BadArgument(
                c"readinto".as_ptr(),
                c"argument".as_ptr(),
                c"read-write bytes-like object".as_ptr(),
                arg,
            );
        } else {
            return_value = _io__BufferedIOBase_readinto_impl(
                self_,
                &(*core::ptr::addr_of_mut!(*__slate_slot_0)),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for buffer
        return return_value;
    }
}

static _io__BufferedIOBase_readinto1__doc__: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 116i8, 111i8, 49i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__BufferedIOBase_readinto1")]
extern "C-unwind" fn _io__BufferedIOBase_readinto1(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut buffer: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(buffer) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 1i32) < 0 {
            _PyArg_BadArgument(
                c"readinto1".as_ptr(),
                c"argument".as_ptr(),
                c"read-write bytes-like object".as_ptr(),
                arg,
            );
        } else {
            return_value = _io__BufferedIOBase_readinto1_impl(
                self_,
                &(*core::ptr::addr_of_mut!(*__slate_slot_0)),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for buffer
        return return_value;
    }
}

static _io__BufferedIOBase_detach__doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    100i8, 101i8, 116i8, 97i8, 99i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 68i8, 105i8, 115i8, 99i8, 111i8, 110i8, 110i8, 101i8,
    99i8, 116i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 105i8, 116i8, 115i8, 32i8, 117i8, 110i8, 100i8, 101i8,
    114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8, 114i8, 97i8, 119i8, 32i8, 115i8, 116i8, 114i8,
    101i8, 97i8, 109i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 105i8, 116i8, 46i8, 10i8, 10i8, 65i8, 102i8, 116i8, 101i8, 114i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 114i8, 97i8, 119i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 104i8,
    97i8, 115i8, 32i8, 98i8, 101i8, 101i8, 110i8, 32i8, 100i8, 101i8, 116i8, 97i8, 99i8, 104i8,
    101i8, 100i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8,
    32i8, 105i8, 115i8, 32i8, 105i8, 110i8, 32i8, 97i8, 110i8, 32i8, 117i8, 110i8, 117i8, 115i8,
    97i8, 98i8, 108i8, 101i8, 10i8, 115i8, 116i8, 97i8, 116i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__BufferedIOBase_detach")]
extern "C-unwind" fn _io__BufferedIOBase_detach(
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
    _io__BufferedIOBase_detach_impl(self_, cls)
}

static _io__BufferedIOBase_read__doc__: __SlateAlign16<[i8; 637]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 105i8,
    122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 97i8, 100i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 117i8, 112i8, 32i8, 116i8, 111i8, 32i8, 110i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8,
    32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 111i8,
    109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 44i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8,
    111i8, 114i8, 32i8, 110i8, 101i8, 103i8, 97i8, 116i8, 105i8, 118i8, 101i8, 44i8, 32i8, 114i8,
    101i8, 97i8, 100i8, 32i8, 97i8, 110i8, 100i8, 10i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 97i8, 108i8, 108i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 117i8, 110i8, 116i8, 105i8,
    108i8, 32i8, 69i8, 79i8, 70i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    115i8, 105i8, 122i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    105i8, 115i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 118i8, 101i8, 44i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8,
    105i8, 110i8, 103i8, 32i8, 114i8, 97i8, 119i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8,
    32i8, 105i8, 115i8, 10i8, 110i8, 111i8, 116i8, 32i8, 39i8, 105i8, 110i8, 116i8, 101i8, 114i8,
    97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 39i8, 44i8, 32i8, 109i8, 117i8, 108i8, 116i8, 105i8,
    112i8, 108i8, 101i8, 32i8, 114i8, 97i8, 119i8, 32i8, 114i8, 101i8, 97i8, 100i8, 115i8, 32i8,
    109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 105i8, 115i8, 115i8, 117i8, 101i8, 100i8, 32i8,
    116i8, 111i8, 32i8, 115i8, 97i8, 116i8, 105i8, 115i8, 102i8, 121i8, 10i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 32i8, 40i8, 117i8,
    110i8, 108i8, 101i8, 115i8, 115i8, 32i8, 69i8, 79i8, 70i8, 32i8, 105i8, 115i8, 32i8, 114i8,
    101i8, 97i8, 99i8, 104i8, 101i8, 100i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 41i8, 46i8,
    10i8, 72i8, 111i8, 119i8, 101i8, 118i8, 101i8, 114i8, 44i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 114i8, 97i8,
    119i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 115i8, 32i8, 40i8, 97i8, 115i8, 32i8,
    119i8, 101i8, 108i8, 108i8, 32i8, 97i8, 115i8, 32i8, 115i8, 111i8, 99i8, 107i8, 101i8, 116i8,
    115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 112i8, 105i8, 112i8, 101i8, 115i8, 41i8, 44i8, 10i8,
    97i8, 116i8, 32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 111i8, 110i8, 101i8, 32i8, 114i8, 97i8,
    119i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 98i8, 101i8,
    32i8, 105i8, 115i8, 115i8, 117i8, 101i8, 100i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 97i8,
    32i8, 115i8, 104i8, 111i8, 114i8, 116i8, 32i8, 114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 32i8,
    100i8, 111i8, 101i8, 115i8, 32i8, 110i8, 111i8, 116i8, 10i8, 105i8, 109i8, 112i8, 108i8, 121i8,
    32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 69i8, 79i8, 70i8, 32i8, 105i8, 115i8, 32i8, 105i8,
    109i8, 109i8, 105i8, 110i8, 101i8, 110i8, 116i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 111i8, 110i8, 32i8,
    69i8, 79i8, 70i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 78i8, 111i8,
    110i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8,
    114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8, 114i8, 97i8, 119i8, 32i8, 115i8, 116i8, 114i8,
    101i8, 97i8, 109i8, 32i8, 119i8, 97i8, 115i8, 32i8, 111i8, 112i8, 101i8, 110i8, 32i8, 105i8,
    110i8, 32i8, 110i8, 111i8, 110i8, 45i8, 98i8, 108i8, 111i8, 99i8, 107i8, 105i8, 110i8, 103i8,
    10i8, 109i8, 111i8, 100i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 110i8, 111i8, 32i8, 100i8,
    97i8, 116i8, 97i8, 32i8, 105i8, 115i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8,
    108i8, 101i8, 32i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 109i8, 101i8,
    110i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__BufferedIOBase_read")]
extern "C-unwind" fn _io__BufferedIOBase_read(
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
                core::ptr::addr_of_mut!(_parser__145),
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
                return_value = _io__BufferedIOBase_read_impl(self_, cls, size);
            }
        }
        return return_value;
    }
}

static _keywords__144: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__145: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__144.0) as *const *const i8 };
    __t0.fname = (b"read\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io__BufferedIOBase_read1__doc__: __SlateAlign16<[i8; 212]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 49i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8,
    105i8, 122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 97i8, 100i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 117i8, 112i8, 32i8, 116i8, 111i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 44i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 116i8, 32i8,
    109i8, 111i8, 115i8, 116i8, 32i8, 111i8, 110i8, 101i8, 32i8, 114i8, 101i8, 97i8, 100i8, 40i8,
    41i8, 32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8, 114i8, 97i8, 119i8,
    32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 111i8, 110i8, 32i8,
    69i8, 79i8, 70i8, 46i8, 10i8, 65i8, 32i8, 115i8, 104i8, 111i8, 114i8, 116i8, 32i8, 114i8,
    101i8, 115i8, 117i8, 108i8, 116i8, 32i8, 100i8, 111i8, 101i8, 115i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 105i8, 109i8, 112i8, 108i8, 121i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 69i8, 79i8,
    70i8, 32i8, 105i8, 115i8, 32i8, 105i8, 109i8, 109i8, 105i8, 110i8, 101i8, 110i8, 116i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__BufferedIOBase_read1")]
extern "C-unwind" fn _io__BufferedIOBase_read1(
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
                core::ptr::addr_of_mut!(_parser__200),
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
                return_value = _io__BufferedIOBase_read1_impl(self_, cls, size);
            }
        }
        return return_value;
    }
}

static _keywords__199: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__200: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__199.0) as *const *const i8 };
    __t0.fname = (b"read1\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io__BufferedIOBase_write__doc__: __SlateAlign16<[i8; 249]> = __SlateAlign16([
    119i8, 114i8, 105i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 98i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 87i8, 114i8, 105i8, 116i8, 101i8, 32i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 98i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 73i8, 79i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 46i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8,
    101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 119i8, 114i8,
    105i8, 116i8, 116i8, 101i8, 110i8, 44i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 105i8,
    115i8, 32i8, 97i8, 108i8, 119i8, 97i8, 121i8, 115i8, 10i8, 116i8, 104i8, 101i8, 32i8, 108i8,
    101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 98i8, 32i8, 105i8, 110i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 10i8, 82i8, 97i8, 105i8, 115i8, 101i8, 32i8,
    66i8, 108i8, 111i8, 99i8, 107i8, 105i8, 110i8, 103i8, 73i8, 79i8, 69i8, 114i8, 114i8, 111i8,
    114i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8,
    114i8, 32i8, 105i8, 115i8, 32i8, 102i8, 117i8, 108i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    116i8, 104i8, 101i8, 10i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8,
    103i8, 32i8, 114i8, 97i8, 119i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 32i8, 99i8,
    97i8, 110i8, 110i8, 111i8, 116i8, 32i8, 97i8, 99i8, 99i8, 101i8, 112i8, 116i8, 32i8, 109i8,
    111i8, 114i8, 101i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 97i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 111i8, 109i8, 101i8, 110i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__BufferedIOBase_write")]
extern "C-unwind" fn _io__BufferedIOBase_write(
    self_: *mut _object,
    cls: *mut _typeobject,
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
            core::ptr::addr_of_mut!(_parser__258),
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
            let b: *mut _object = *args.wrapping_offset(0 as isize);
            return_value = _io__BufferedIOBase_write_impl(self_, cls, b);
        }
        return return_value;
    }
}

static _keywords__257: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__258: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__257.0) as *const *const i8 };
    __t0.fname = (b"write\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io__Buffered___sizeof____doc__: __SlateAlign16<[i8; 26]> = __SlateAlign16([
    95i8, 95i8, 115i8, 105i8, 122i8, 101i8, 111i8, 102i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered___sizeof__")]
extern "C-unwind" fn _io__Buffered___sizeof__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered___sizeof___impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered__dealloc_warn__doc__: __SlateAlign16<[i8; 37]> = __SlateAlign16([
    95i8, 100i8, 101i8, 97i8, 108i8, 108i8, 111i8, 99i8, 95i8, 119i8, 97i8, 114i8, 110i8, 40i8,
    36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered__dealloc_warn")]
extern "C-unwind" fn _io__Buffered__dealloc_warn(
    self_: *mut _object,
    source: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value =
        _io__Buffered__dealloc_warn_impl(self_ as *mut __SlateRecord_bufferedio_c_264, source);
    return_value
}

static _io__Buffered_simple_flush__doc__: __SlateAlign16<[i8; 21]> = __SlateAlign16([
    102i8, 108i8, 117i8, 115i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_simple_flush")]
extern "C-unwind" fn _io__Buffered_simple_flush(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_simple_flush_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_closed_get")]
extern "C-unwind" fn _io__Buffered_closed_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_closed_get_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_close__doc__: __SlateAlign16<[i8; 21]> = __SlateAlign16([
    99i8, 108i8, 111i8, 115i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_close")]
extern "C-unwind" fn _io__Buffered_close(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_close_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_detach__doc__: __SlateAlign16<[i8; 22]> = __SlateAlign16([
    100i8, 101i8, 116i8, 97i8, 99i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_detach")]
extern "C-unwind" fn _io__Buffered_detach(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_detach_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_seekable__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    115i8, 101i8, 101i8, 107i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_seekable")]
extern "C-unwind" fn _io__Buffered_seekable(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_seekable_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_readable__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_readable")]
extern "C-unwind" fn _io__Buffered_readable(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_readable_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_writable__doc__: __SlateAlign16<[i8; 24]> = __SlateAlign16([
    119i8, 114i8, 105i8, 116i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_writable")]
extern "C-unwind" fn _io__Buffered_writable(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_writable_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_name_get")]
extern "C-unwind" fn _io__Buffered_name_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_name_get_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_mode_get")]
extern "C-unwind" fn _io__Buffered_mode_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_mode_get_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_fileno__doc__: __SlateAlign16<[i8; 22]> = __SlateAlign16([
    102i8, 105i8, 108i8, 101i8, 110i8, 111i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_fileno")]
extern "C-unwind" fn _io__Buffered_fileno(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_fileno_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_isatty__doc__: __SlateAlign16<[i8; 22]> = __SlateAlign16([
    105i8, 115i8, 97i8, 116i8, 116i8, 121i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_isatty")]
extern "C-unwind" fn _io__Buffered_isatty(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_isatty_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_flush__doc__: __SlateAlign16<[i8; 21]> = __SlateAlign16([
    102i8, 108i8, 117i8, 115i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_flush")]
extern "C-unwind" fn _io__Buffered_flush(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_flush_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_peek__doc__: __SlateAlign16<[i8; 28]> = __SlateAlign16([
    112i8, 101i8, 101i8, 107i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 105i8,
    122i8, 101i8, 61i8, 48i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_peek")]
extern "C-unwind" fn _io__Buffered_peek(
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
            size = 0;
            if _PyArg_CheckPositional!(c"peek".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
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
                return_value =
                    _io__Buffered_peek_impl(self_ as *mut __SlateRecord_bufferedio_c_264, size);
            }
        }
        return return_value;
    }
}

static _io__Buffered_read__doc__: __SlateAlign16<[i8; 29]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 105i8,
    122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_read")]
extern "C-unwind" fn _io__Buffered_read(
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
                return_value =
                    _io__Buffered_read_impl(self_ as *mut __SlateRecord_bufferedio_c_264, n);
            }
        }
        return return_value;
    }
}

static _io__Buffered_read1__doc__: __SlateAlign16<[i8; 30]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 49i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8,
    105i8, 122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_read1")]
extern "C-unwind" fn _io__Buffered_read1(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            n = 1i32.wrapping_neg() as i64;
            if _PyArg_CheckPositional!(c"read1".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
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
                        n = ival;
                    }
                }
                return_value =
                    _io__Buffered_read1_impl(self_ as *mut __SlateRecord_bufferedio_c_264, n);
            }
        }
        return return_value;
    }
}

static _io__Buffered_readinto__doc__: __SlateAlign16<[i8; 32]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 116i8, 111i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_readinto")]
extern "C-unwind" fn _io__Buffered_readinto(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut buffer: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(buffer) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 1i32) < 0 {
            _PyArg_BadArgument(
                c"readinto".as_ptr(),
                c"argument".as_ptr(),
                c"read-write bytes-like object".as_ptr(),
                arg,
            );
        } else {
            return_value = _io__Buffered_readinto_impl(
                self_ as *mut __SlateRecord_bufferedio_c_264,
                core::ptr::addr_of_mut!(*__slate_slot_0),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for buffer
        return return_value;
    }
}

static _io__Buffered_readinto1__doc__: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 116i8, 111i8, 49i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_readinto1")]
extern "C-unwind" fn _io__Buffered_readinto1(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut buffer: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(buffer) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 1i32) < 0 {
            _PyArg_BadArgument(
                c"readinto1".as_ptr(),
                c"argument".as_ptr(),
                c"read-write bytes-like object".as_ptr(),
                arg,
            );
        } else {
            return_value = _io__Buffered_readinto1_impl(
                self_ as *mut __SlateRecord_bufferedio_c_264,
                core::ptr::addr_of_mut!(*__slate_slot_0),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for buffer
        return return_value;
    }
}

static _io__Buffered_readline__doc__: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    114i8, 101i8, 97i8, 100i8, 108i8, 105i8, 110i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 115i8, 105i8, 122i8, 101i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_readline")]
extern "C-unwind" fn _io__Buffered_readline(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut size: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            size = 1i32.wrapping_neg() as i64;
            if _PyArg_CheckPositional!(c"readline".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
                if nargs >= 1 {
                    if _Py_convert_optional_to_ssize_t(
                        *args.wrapping_offset(0 as isize),
                        core::ptr::addr_of_mut!(size) as *mut (),
                    ) == 0
                    {
                        break '__join_0;
                    }
                }
                return_value =
                    _io__Buffered_readline_impl(self_ as *mut __SlateRecord_bufferedio_c_264, size);
            }
        }
        return return_value;
    }
}

static _io__Buffered_tell__doc__: __SlateAlign16<[i8; 20]> = __SlateAlign16([
    116i8, 101i8, 108i8, 108i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_tell")]
extern "C-unwind" fn _io__Buffered_tell(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _io__Buffered_tell_impl(self_ as *mut __SlateRecord_bufferedio_c_264);
    return_value
}

static _io__Buffered_seek__doc__: __SlateAlign16<[i8; 38]> = __SlateAlign16([
    115i8, 101i8, 101i8, 107i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 116i8, 97i8,
    114i8, 103i8, 101i8, 116i8, 44i8, 32i8, 119i8, 104i8, 101i8, 110i8, 99i8, 101i8, 61i8, 48i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_seek")]
extern "C-unwind" fn _io__Buffered_seek(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut whence: i32 = unsafe { core::mem::zeroed() };
    let mut targetobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            whence = 0;
            if _PyArg_CheckPositional!(c"seek".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
                targetobj = *args.wrapping_offset(0 as isize);
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
                return_value = _io__Buffered_seek_impl(
                    self_ as *mut __SlateRecord_bufferedio_c_264,
                    targetobj,
                    whence,
                );
            }
        }
        return return_value;
    }
}

static _io__Buffered_truncate__doc__: __SlateAlign16<[i8; 34]> = __SlateAlign16([
    116i8, 114i8, 117i8, 110i8, 99i8, 97i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 112i8, 111i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io__Buffered_truncate")]
extern "C-unwind" fn _io__Buffered_truncate(
    self_: *mut _object,
    cls: *mut _typeobject,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut pos: *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        pos = core::ptr::addr_of_mut!(_Py_NoneStruct);
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__914),
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
                pos = *args.wrapping_offset(0 as isize);
            }
            return_value =
                _io__Buffered_truncate_impl(self_ as *mut __SlateRecord_bufferedio_c_264, cls, pos);
        }
        return return_value;
    }
}

static _keywords__913: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__914: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__913.0) as *const *const i8 };
    __t0.fname = (b"truncate\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty) })
            as *mut _object;
    __t0
};

static _io_BufferedReader___init____doc__: __SlateAlign16<[i8; 126]> = __SlateAlign16([
    66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 82i8, 101i8, 97i8, 100i8, 101i8, 114i8,
    40i8, 114i8, 97i8, 119i8, 44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8, 115i8,
    105i8, 122i8, 101i8, 61i8, 68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 95i8, 66i8, 85i8, 70i8,
    70i8, 69i8, 82i8, 95i8, 83i8, 73i8, 90i8, 69i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8,
    114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 98i8, 117i8,
    102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 32i8, 114i8, 101i8, 97i8, 100i8, 101i8, 114i8, 32i8,
    117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8,
    110i8, 32i8, 114i8, 101i8, 97i8, 100i8, 97i8, 98i8, 108i8, 101i8, 32i8, 114i8, 97i8, 119i8,
    32i8, 73i8, 79i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io_BufferedReader___init__")]
extern "C-unwind" fn _io_BufferedReader___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut buffer_size: i64 = unsafe { core::mem::zeroed() };
    let mut raw: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v5 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v5 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v5).wrapping_sub(1);
            buffer_size = 131072i64;
            fastargs = _PyArg_UnpackKeywords__2!(
                (*(args as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object,
                nargs,
                kwargs,
                core::ptr::null_mut::<()>(),
                core::ptr::addr_of_mut!(_parser__976),
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
                raw = *fastargs.wrapping_offset(0 as isize);
                if noptargs != 0 {
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
                    if __v0 {
                        break '__join_0;
                    } else {
                        buffer_size = ival;
                    }
                }
                return_value = _io_BufferedReader___init___impl(
                    self_ as *mut __SlateRecord_bufferedio_c_264,
                    raw,
                    buffer_size,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__963: __SlateRecord_bufferedio_c_h_963 = {
    let mut __t0: __SlateRecord_bufferedio_c_h_963 = unsafe { core::mem::zeroed() };
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
                    ._py_raw
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
                    ._py_buffer_size
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__975: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"raw\0".as_ptr() as *mut i8) as *const i8,
    (b"buffer_size\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__976: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__975.0) as *const *const i8 };
    __t0.fname = (b"BufferedReader\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__963.ob_base.ob_base) };
    __t0
};

static _io_BufferedWriter___init____doc__: __SlateAlign16<[i8; 254]> = __SlateAlign16([
    66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 87i8, 114i8, 105i8, 116i8, 101i8, 114i8,
    40i8, 114i8, 97i8, 119i8, 44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8, 115i8,
    105i8, 122i8, 101i8, 61i8, 68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 95i8, 66i8, 85i8, 70i8,
    70i8, 69i8, 82i8, 95i8, 83i8, 73i8, 90i8, 69i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 32i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 32i8, 119i8,
    114i8, 105i8, 116i8, 101i8, 97i8, 98i8, 108i8, 101i8, 32i8, 115i8, 101i8, 113i8, 117i8, 101i8,
    110i8, 116i8, 105i8, 97i8, 108i8, 32i8, 82i8, 97i8, 119i8, 73i8, 79i8, 32i8, 111i8, 98i8,
    106i8, 101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 99i8, 111i8, 110i8,
    115i8, 116i8, 114i8, 117i8, 99i8, 116i8, 111i8, 114i8, 32i8, 99i8, 114i8, 101i8, 97i8, 116i8,
    101i8, 115i8, 32i8, 97i8, 32i8, 66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 87i8,
    114i8, 105i8, 116i8, 101i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 97i8, 98i8, 108i8,
    101i8, 32i8, 114i8, 97i8, 119i8, 10i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 46i8, 32i8,
    73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8,
    115i8, 105i8, 122i8, 101i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 103i8, 105i8,
    118i8, 101i8, 110i8, 44i8, 32i8, 105i8, 116i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 115i8, 32i8, 116i8, 111i8, 10i8, 68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 95i8, 66i8,
    85i8, 70i8, 70i8, 69i8, 82i8, 95i8, 83i8, 73i8, 90i8, 69i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io_BufferedWriter___init__")]
extern "C-unwind" fn _io_BufferedWriter___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut buffer_size: i64 = unsafe { core::mem::zeroed() };
    let mut raw: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v5 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v5 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v5).wrapping_sub(1);
            buffer_size = 131072i64;
            fastargs = _PyArg_UnpackKeywords__2!(
                (*(args as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object,
                nargs,
                kwargs,
                core::ptr::null_mut::<()>(),
                core::ptr::addr_of_mut!(_parser__1056),
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
                raw = *fastargs.wrapping_offset(0 as isize);
                if noptargs != 0 {
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
                    if __v0 {
                        break '__join_0;
                    } else {
                        buffer_size = ival;
                    }
                }
                return_value = _io_BufferedWriter___init___impl(
                    self_ as *mut __SlateRecord_bufferedio_c_264,
                    raw,
                    buffer_size,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1043: __SlateRecord_bufferedio_c_h_1043 = {
    let mut __t0: __SlateRecord_bufferedio_c_h_1043 = unsafe { core::mem::zeroed() };
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
                    ._py_raw
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
                    ._py_buffer_size
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1055: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"raw\0".as_ptr() as *mut i8) as *const i8,
    (b"buffer_size\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1056: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1055.0) as *const *const i8 };
    __t0.fname = (b"BufferedWriter\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1043.ob_base.ob_base) };
    __t0
};

static _io_BufferedWriter_write__doc__: __SlateAlign16<[i8; 29]> = __SlateAlign16([
    119i8, 114i8, 105i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 98i8,
    117i8, 102i8, 102i8, 101i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io_BufferedWriter_write")]
extern "C-unwind" fn _io_BufferedWriter_write(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut buffer: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(buffer) as *mut __SlateRecord_pybuffer_h_33;
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
            return_value = _io_BufferedWriter_write_impl(
                self_ as *mut __SlateRecord_bufferedio_c_264,
                core::ptr::addr_of_mut!(*__slate_slot_0),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for buffer
        return return_value;
    }
}

static _io_BufferedRWPair___init____doc__: __SlateAlign16<[i8; 446]> = __SlateAlign16([
    66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 82i8, 87i8, 80i8, 97i8, 105i8, 114i8,
    40i8, 114i8, 101i8, 97i8, 100i8, 101i8, 114i8, 44i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8,
    114i8, 44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8, 115i8, 105i8, 122i8, 101i8,
    61i8, 68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 95i8, 66i8, 85i8, 70i8, 70i8, 69i8, 82i8, 95i8,
    83i8, 73i8, 90i8, 69i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 32i8, 98i8,
    117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 32i8, 114i8, 101i8, 97i8, 100i8, 101i8, 114i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 114i8, 32i8, 111i8, 98i8,
    106i8, 101i8, 99i8, 116i8, 32i8, 116i8, 111i8, 103i8, 101i8, 116i8, 104i8, 101i8, 114i8, 46i8,
    10i8, 10i8, 65i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 32i8, 114i8,
    101i8, 97i8, 100i8, 101i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 32i8, 119i8, 114i8,
    105i8, 116i8, 101i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 112i8, 117i8,
    116i8, 32i8, 116i8, 111i8, 103i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 111i8, 10i8,
    102i8, 111i8, 114i8, 109i8, 32i8, 97i8, 32i8, 115i8, 101i8, 113i8, 117i8, 101i8, 110i8, 116i8,
    105i8, 97i8, 108i8, 32i8, 73i8, 79i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 97i8, 110i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 46i8, 32i8, 84i8, 104i8, 105i8,
    115i8, 32i8, 105i8, 115i8, 32i8, 116i8, 121i8, 112i8, 105i8, 99i8, 97i8, 108i8, 108i8, 121i8,
    10i8, 117i8, 115i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 32i8, 115i8,
    111i8, 99i8, 107i8, 101i8, 116i8, 32i8, 111i8, 114i8, 32i8, 116i8, 119i8, 111i8, 45i8, 119i8,
    97i8, 121i8, 32i8, 112i8, 105i8, 112i8, 101i8, 46i8, 10i8, 10i8, 114i8, 101i8, 97i8, 100i8,
    101i8, 114i8, 32i8, 97i8, 110i8, 100i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 114i8, 32i8,
    97i8, 114i8, 101i8, 32i8, 82i8, 97i8, 119i8, 73i8, 79i8, 66i8, 97i8, 115i8, 101i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 97i8, 114i8,
    101i8, 32i8, 114i8, 101i8, 97i8, 100i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 110i8, 100i8,
    10i8, 119i8, 114i8, 105i8, 116i8, 101i8, 97i8, 98i8, 108i8, 101i8, 32i8, 114i8, 101i8, 115i8,
    112i8, 101i8, 99i8, 116i8, 105i8, 118i8, 101i8, 108i8, 121i8, 46i8, 32i8, 73i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8, 115i8, 105i8, 122i8,
    101i8, 32i8, 105i8, 115i8, 32i8, 111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 105i8,
    116i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 10i8,
    68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 95i8, 66i8, 85i8, 70i8, 70i8, 69i8, 82i8, 95i8, 83i8,
    73i8, 90i8, 69i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io_BufferedRWPair___init__")]
extern "C-unwind" fn _io_BufferedRWPair___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut buffer_size: i64 = unsafe { core::mem::zeroed() };
    let mut writer: *mut _object = unsafe { core::mem::zeroed() };
    let mut reader: *mut _object = unsafe { core::mem::zeroed() };
    let mut base_tp: *mut _typeobject = unsafe { core::mem::zeroed() };
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        base_tp = (*clinic_state__2!(self_)).PyBufferedRWPair_Type;
        buffer_size = 131072i64;
        if Py_IS_TYPE!(self_, base_tp) != 0 {
            __v4 = true;
        } else {
            __v4 = (*Py_TYPE!(self_)).tp_new == (*base_tp).tp_new;
        }
        if __v4 {
            __v3 = !_PyArg_NoKeywords!(c"BufferedRWPair".as_ptr() as *mut i8, kwargs);
        } else {
            __v3 = false;
        }
        '__join_0: {
            if !__v3 {
                if _PyArg_CheckPositional__2!(
                    c"BufferedRWPair".as_ptr() as *mut i8,
                    PyTuple_GET_SIZE(args),
                    2i32,
                    3i32
                ) {
                    reader = PyTuple_GET_ITEM!(args, 0i32);
                    writer = PyTuple_GET_ITEM!(args, 1i32);
                    if PyTuple_GET_SIZE!(args) >= 3 {
                        ival = 1i32.wrapping_neg() as i64;
                        iobj = _PyNumber_Index(PyTuple_GET_ITEM!(args, 2i32));
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
                            buffer_size = ival;
                        }
                    }
                    return_value = _io_BufferedRWPair___init___impl(
                        self_ as *mut __SlateRecord_bufferedio_c_2238,
                        reader,
                        writer,
                        buffer_size,
                    );
                }
            }
        }
        return return_value;
    }
}

static _io_BufferedRandom___init____doc__: __SlateAlign16<[i8; 272]> = __SlateAlign16([
    66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 82i8, 97i8, 110i8, 100i8, 111i8, 109i8,
    40i8, 114i8, 97i8, 119i8, 44i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8, 115i8,
    105i8, 122i8, 101i8, 61i8, 68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 95i8, 66i8, 85i8, 70i8,
    70i8, 69i8, 82i8, 95i8, 83i8, 73i8, 90i8, 69i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 32i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8,
    102i8, 97i8, 99i8, 101i8, 32i8, 116i8, 111i8, 32i8, 114i8, 97i8, 110i8, 100i8, 111i8, 109i8,
    32i8, 97i8, 99i8, 99i8, 101i8, 115i8, 115i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8,
    115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 99i8, 111i8, 110i8, 115i8, 116i8, 114i8,
    117i8, 99i8, 116i8, 111i8, 114i8, 32i8, 99i8, 114i8, 101i8, 97i8, 116i8, 101i8, 115i8, 32i8,
    97i8, 32i8, 114i8, 101i8, 97i8, 100i8, 101i8, 114i8, 32i8, 97i8, 110i8, 100i8, 32i8, 119i8,
    114i8, 105i8, 116i8, 101i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 32i8, 115i8, 101i8,
    101i8, 107i8, 97i8, 98i8, 108i8, 101i8, 32i8, 115i8, 116i8, 114i8, 101i8, 97i8, 109i8, 44i8,
    10i8, 114i8, 97i8, 119i8, 44i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 105i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 97i8, 114i8, 103i8,
    117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 32i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 95i8, 115i8, 105i8, 122i8, 101i8, 32i8, 105i8, 115i8,
    32i8, 111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 105i8, 116i8, 10i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 68i8, 69i8, 70i8, 65i8,
    85i8, 76i8, 84i8, 95i8, 66i8, 85i8, 70i8, 70i8, 69i8, 82i8, 95i8, 83i8, 73i8, 90i8, 69i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bufferedio._io_BufferedRandom___init__")]
extern "C-unwind" fn _io_BufferedRandom___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut buffer_size: i64 = unsafe { core::mem::zeroed() };
    let mut raw: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v5 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v5 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v5).wrapping_sub(1);
            buffer_size = 131072i64;
            fastargs = _PyArg_UnpackKeywords__2!(
                (*(args as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object,
                nargs,
                kwargs,
                core::ptr::null_mut::<()>(),
                core::ptr::addr_of_mut!(_parser__1228),
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
                raw = *fastargs.wrapping_offset(0 as isize);
                if noptargs != 0 {
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
                    if __v0 {
                        break '__join_0;
                    } else {
                        buffer_size = ival;
                    }
                }
                return_value = _io_BufferedRandom___init___impl(
                    self_ as *mut __SlateRecord_bufferedio_c_264,
                    raw,
                    buffer_size,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1215: __SlateRecord_bufferedio_c_h_1215 = {
    let mut __t0: __SlateRecord_bufferedio_c_h_1215 = unsafe { core::mem::zeroed() };
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
                    ._py_raw
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
                    ._py_buffer_size
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1227: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"raw\0".as_ptr() as *mut i8) as *const i8,
    (b"buffer_size\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1228: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1227.0) as *const *const i8 };
    __t0.fname = (b"BufferedRandom\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1215.ob_base.ob_base) };
    __t0
};

static mut bufferediobase_methods: __SlateAlign16<[PyMethodDef; 7]> = __SlateAlign16([
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
                >(Some(_io__BufferedIOBase_detach))
            })
        };
        __t0.ml_flags = 642i32;
        __t0.ml_doc =
            unsafe { core::ptr::addr_of!(_io__BufferedIOBase_detach__doc__.0) as *const i8 };
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
                >(Some(_io__BufferedIOBase_read))
            })
        };
        __t1.ml_flags = 642i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(_io__BufferedIOBase_read__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"read1\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__BufferedIOBase_read1))
            })
        };
        __t2.ml_flags = 642i32;
        __t2.ml_doc =
            unsafe { core::ptr::addr_of!(_io__BufferedIOBase_read1__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"readinto\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_io__BufferedIOBase_readinto);
        __t3.ml_flags = 8i32;
        __t3.ml_doc =
            unsafe { core::ptr::addr_of!(_io__BufferedIOBase_readinto__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"readinto1\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(_io__BufferedIOBase_readinto1);
        __t4.ml_flags = 8i32;
        __t4.ml_doc =
            unsafe { core::ptr::addr_of!(_io__BufferedIOBase_readinto1__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"write\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = unsafe {
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
                >(Some(_io__BufferedIOBase_write))
            })
        };
        __t5.ml_flags = 642i32;
        __t5.ml_doc =
            unsafe { core::ptr::addr_of!(_io__BufferedIOBase_write__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = core::ptr::null::<i8>();
        __t6.ml_meth = None;
        __t6
    },
]);

static mut bufferediobase_slots: __SlateAlign16<[__SlateRecord_object_h_377; 3]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 56i32;
            __t0.pfunc =
                (unsafe { core::ptr::addr_of!(bufferediobase_doc.0) as *const i8 }) as *mut ();
            __t0
        },
        {
            let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t1.slot = 64i32;
            __t1.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferediobase_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 0i32;
            __t2.pfunc = core::ptr::null_mut::<()>();
            __t2
        },
    ]);

// Do not set Py_TPFLAGS_HAVE_GC so that tp_traverse and tp_clear are inherited
/// BufferedIOMixin methods
static mut bufferedreader_methods: __SlateAlign16<[PyMethodDef; 20]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"detach\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_io__Buffered_detach);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_detach__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"flush\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_io__Buffered_simple_flush);
        __t1.ml_flags = 4i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered_simple_flush__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"close\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_io__Buffered_close);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_close__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"seekable\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_io__Buffered_seekable);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_seekable__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"readable\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(_io__Buffered_readable);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_readable__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"fileno\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(_io__Buffered_fileno);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_fileno__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"isatty\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(_io__Buffered_isatty);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_isatty__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"_dealloc_warn\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(_io__Buffered__dealloc_warn);
        __t7.ml_flags = 8i32;
        __t7.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered__dealloc_warn__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"read\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_read))
            })
        };
        __t8.ml_flags = 128i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_read__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"peek\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_peek))
            })
        };
        __t9.ml_flags = 128i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_peek__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"read1\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_read1))
            })
        };
        __t10.ml_flags = 128i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_read1__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"readinto\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(_io__Buffered_readinto);
        __t11.ml_flags = 8i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_readinto__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"readinto1\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(_io__Buffered_readinto1);
        __t12.ml_flags = 8i32;
        __t12.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered_readinto1__doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"readline\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_readline))
            })
        };
        __t13.ml_flags = 128i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_readline__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"seek\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_seek))
            })
        };
        __t14.ml_flags = 128i32;
        __t14.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_seek__doc__.0) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"tell\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = Some(_io__Buffered_tell);
        __t15.ml_flags = 4i32;
        __t15.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_tell__doc__.0) as *const i8 };
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"truncate\0".as_ptr() as *mut i8) as *const i8;
        __t16.ml_meth = unsafe {
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
                >(Some(_io__Buffered_truncate))
            })
        };
        __t16.ml_flags = 642i32;
        __t16.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_truncate__doc__.0) as *const i8 };
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t17.ml_meth = Some(_io__Buffered___sizeof__);
        __t17.ml_flags = 4i32;
        __t17.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered___sizeof____doc__.0) as *const i8 };
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = (b"__getstate__\0".as_ptr() as *mut i8) as *const i8;
        __t18.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(_PyIOBase_cannot_pickle as *const ())
        };
        __t18.ml_flags = 4i32;
        __t18
    },
    {
        let mut __t19: PyMethodDef = unsafe { core::mem::zeroed() };
        __t19.ml_name = core::ptr::null::<i8>();
        __t19.ml_meth = None;
        __t19
    },
]);

static mut bufferedreader_members: __SlateAlign16<[PyMemberDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"raw\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, raw) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"_finalizing\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 14i32;
        __t1.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, finalizing) as u64) as i64;
        __t1.flags = 0i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 19i32;
        __t2.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, weakreflist) as u64) as i64;
        __t2.flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 19i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, dict) as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut bufferedreader_getset: __SlateAlign16<[PyGetSetDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"closed\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(_io__Buffered_closed_get);
        __t0.set = None;
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(_io__Buffered_name_get);
        __t1.set = None;
        __t1.doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"mode\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(_io__Buffered_mode_get);
        __t2.set = None;
        __t2.doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut bufferedreader_slots: __SlateAlign16<[__SlateRecord_object_h_377; 11]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(buffered_dealloc),
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
                >(Some(buffered_repr))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 56i32;
            __t2.pfunc =
                (unsafe { core::ptr::addr_of!(_io_BufferedReader___init____doc__.0) as *const i8 })
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
                >(Some(buffered_traverse))
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
                >(Some(buffered_clear))
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
                >(Some(buffered_iternext))
            };
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 64i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedreader_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 72i32;
            __t7.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedreader_members.0) as *mut PyMemberDef })
                    as *mut ();
            __t7
        },
        {
            let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t8.slot = 73i32;
            __t8.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedreader_getset.0) as *mut PyGetSetDef })
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
                >(Some(_io_BufferedReader___init__))
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

/// BufferedIOMixin methods
static mut bufferedwriter_methods: __SlateAlign16<[PyMethodDef; 15]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"close\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_io__Buffered_close);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_close__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"detach\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_io__Buffered_detach);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_detach__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"seekable\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_io__Buffered_seekable);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_seekable__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"writable\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_io__Buffered_writable);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_writable__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"fileno\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(_io__Buffered_fileno);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_fileno__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"isatty\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(_io__Buffered_isatty);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_isatty__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"_dealloc_warn\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(_io__Buffered__dealloc_warn);
        __t6.ml_flags = 8i32;
        __t6.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered__dealloc_warn__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"write\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(_io_BufferedWriter_write);
        __t7.ml_flags = 8i32;
        __t7.ml_doc =
            unsafe { core::ptr::addr_of!(_io_BufferedWriter_write__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"truncate\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = unsafe {
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
                >(Some(_io__Buffered_truncate))
            })
        };
        __t8.ml_flags = 642i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_truncate__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"flush\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(_io__Buffered_flush);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_flush__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"seek\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_seek))
            })
        };
        __t10.ml_flags = 128i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_seek__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"tell\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(_io__Buffered_tell);
        __t11.ml_flags = 4i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_tell__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(_io__Buffered___sizeof__);
        __t12.ml_flags = 4i32;
        __t12.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered___sizeof____doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"__getstate__\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(_PyIOBase_cannot_pickle as *const ())
        };
        __t13.ml_flags = 4i32;
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = core::ptr::null::<i8>();
        __t14.ml_meth = None;
        __t14
    },
]);

static mut bufferedwriter_members: __SlateAlign16<[PyMemberDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"raw\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, raw) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"_finalizing\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 14i32;
        __t1.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, finalizing) as u64) as i64;
        __t1.flags = 0i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 19i32;
        __t2.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, weakreflist) as u64) as i64;
        __t2.flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 19i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, dict) as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut bufferedwriter_getset: __SlateAlign16<[PyGetSetDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"closed\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(_io__Buffered_closed_get);
        __t0.set = None;
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(_io__Buffered_name_get);
        __t1.set = None;
        __t1.doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"mode\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(_io__Buffered_mode_get);
        __t2.set = None;
        __t2.doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut bufferedwriter_slots: __SlateAlign16<[__SlateRecord_object_h_377; 10]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(buffered_dealloc),
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
                >(Some(buffered_repr))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 56i32;
            __t2.pfunc =
                (unsafe { core::ptr::addr_of!(_io_BufferedWriter___init____doc__.0) as *const i8 })
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
                >(Some(buffered_traverse))
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
                >(Some(buffered_clear))
            };
            __t4
        },
        {
            let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t5.slot = 64i32;
            __t5.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedwriter_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 72i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedwriter_members.0) as *mut PyMemberDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 73i32;
            __t7.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedwriter_getset.0) as *mut PyGetSetDef })
                    as *mut ();
            __t7
        },
        {
            let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t8.slot = 60i32;
            __t8.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >,
                    *mut (),
                >(Some(_io_BufferedWriter___init__))
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

static mut bufferedrwpair_methods: __SlateAlign16<[PyMethodDef; 12]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"read\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(bufferedrwpair_read);
        __t0.ml_flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"peek\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(bufferedrwpair_peek);
        __t1.ml_flags = 1i32;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"read1\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(bufferedrwpair_read1);
        __t2.ml_flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"readinto\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(bufferedrwpair_readinto);
        __t3.ml_flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"readinto1\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(bufferedrwpair_readinto1);
        __t4.ml_flags = 1i32;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"write\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(bufferedrwpair_write);
        __t5.ml_flags = 1i32;
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"flush\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(bufferedrwpair_flush);
        __t6.ml_flags = 4i32;
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"readable\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(bufferedrwpair_readable);
        __t7.ml_flags = 4i32;
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"writable\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(bufferedrwpair_writable);
        __t8.ml_flags = 4i32;
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"close\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(bufferedrwpair_close);
        __t9.ml_flags = 4i32;
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"isatty\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(bufferedrwpair_isatty);
        __t10.ml_flags = 4i32;
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = core::ptr::null::<i8>();
        __t11.ml_meth = None;
        __t11
    },
]);

static mut bufferedrwpair_members: __SlateAlign16<[PyMemberDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 19i32;
        __t0.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_2238, weakreflist) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 19i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_2238, dict) as u64) as i64;
        __t1.flags = 1i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = core::ptr::null::<i8>();
        __t2
    },
]);

static mut bufferedrwpair_getset: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"closed\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(bufferedrwpair_closed_get);
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

static mut bufferedrwpair_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(bufferedrwpair_dealloc),
                )
            };
            __t0
        },
        {
            let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t1.slot = 56i32;
            __t1.pfunc =
                (unsafe { core::ptr::addr_of!(_io_BufferedRWPair___init____doc__.0) as *const i8 })
                    as *mut ();
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
                >(Some(bufferedrwpair_traverse))
            };
            __t2
        },
        {
            let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t3.slot = 51i32;
            __t3.pfunc = unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
                    *mut (),
                >(Some(bufferedrwpair_clear))
            };
            __t3
        },
        {
            let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t4.slot = 64i32;
            __t4.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedrwpair_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t4
        },
        {
            let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t5.slot = 72i32;
            __t5.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedrwpair_members.0) as *mut PyMemberDef })
                    as *mut ();
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 73i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedrwpair_getset.0) as *mut PyGetSetDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 60i32;
            __t7.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >,
                    *mut (),
                >(Some(_io_BufferedRWPair___init__))
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

/// BufferedIOMixin methods
static mut bufferedrandom_methods: __SlateAlign16<[PyMethodDef; 22]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"close\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_io__Buffered_close);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_close__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"detach\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_io__Buffered_detach);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_detach__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"seekable\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_io__Buffered_seekable);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_seekable__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"readable\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_io__Buffered_readable);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_readable__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"writable\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(_io__Buffered_writable);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_writable__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"fileno\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(_io__Buffered_fileno);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_fileno__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"isatty\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(_io__Buffered_isatty);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_isatty__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"_dealloc_warn\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(_io__Buffered__dealloc_warn);
        __t7.ml_flags = 8i32;
        __t7.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered__dealloc_warn__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"flush\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(_io__Buffered_flush);
        __t8.ml_flags = 4i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_flush__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"seek\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_seek))
            })
        };
        __t9.ml_flags = 128i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_seek__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"tell\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(_io__Buffered_tell);
        __t10.ml_flags = 4i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_tell__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"truncate\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = unsafe {
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
                >(Some(_io__Buffered_truncate))
            })
        };
        __t11.ml_flags = 642i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_truncate__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"read\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_read))
            })
        };
        __t12.ml_flags = 128i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_read__doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"read1\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_read1))
            })
        };
        __t13.ml_flags = 128i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_read1__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"readinto\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = Some(_io__Buffered_readinto);
        __t14.ml_flags = 8i32;
        __t14.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_readinto__doc__.0) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"readinto1\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = Some(_io__Buffered_readinto1);
        __t15.ml_flags = 8i32;
        __t15.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered_readinto1__doc__.0) as *const i8 };
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"readline\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_readline))
            })
        };
        __t16.ml_flags = 128i32;
        __t16.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_readline__doc__.0) as *const i8 };
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"peek\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_io__Buffered_peek))
            })
        };
        __t17.ml_flags = 128i32;
        __t17.ml_doc = unsafe { core::ptr::addr_of!(_io__Buffered_peek__doc__.0) as *const i8 };
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = (b"write\0".as_ptr() as *mut i8) as *const i8;
        __t18.ml_meth = Some(_io_BufferedWriter_write);
        __t18.ml_flags = 8i32;
        __t18.ml_doc =
            unsafe { core::ptr::addr_of!(_io_BufferedWriter_write__doc__.0) as *const i8 };
        __t18
    },
    {
        let mut __t19: PyMethodDef = unsafe { core::mem::zeroed() };
        __t19.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t19.ml_meth = Some(_io__Buffered___sizeof__);
        __t19.ml_flags = 4i32;
        __t19.ml_doc =
            unsafe { core::ptr::addr_of!(_io__Buffered___sizeof____doc__.0) as *const i8 };
        __t19
    },
    {
        let mut __t20: PyMethodDef = unsafe { core::mem::zeroed() };
        __t20.ml_name = (b"__getstate__\0".as_ptr() as *mut i8) as *const i8;
        __t20.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(_PyIOBase_cannot_pickle as *const ())
        };
        __t20.ml_flags = 4i32;
        __t20
    },
    {
        let mut __t21: PyMethodDef = unsafe { core::mem::zeroed() };
        __t21.ml_name = core::ptr::null::<i8>();
        __t21.ml_meth = None;
        __t21
    },
]);

static mut bufferedrandom_members: __SlateAlign16<[PyMemberDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"raw\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, raw) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"_finalizing\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 14i32;
        __t1.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, finalizing) as u64) as i64;
        __t1.flags = 0i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 19i32;
        __t2.offset =
            (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, weakreflist) as u64) as i64;
        __t2.flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 19i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_bufferedio_c_264, dict) as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut bufferedrandom_getset: __SlateAlign16<[PyGetSetDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"closed\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(_io__Buffered_closed_get);
        __t0.set = None;
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(_io__Buffered_name_get);
        __t1.set = None;
        __t1.doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"mode\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(_io__Buffered_mode_get);
        __t2.set = None;
        __t2.doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut bufferedrandom_slots: __SlateAlign16<[__SlateRecord_object_h_377; 11]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(buffered_dealloc),
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
                >(Some(buffered_repr))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 56i32;
            __t2.pfunc =
                (unsafe { core::ptr::addr_of!(_io_BufferedRandom___init____doc__.0) as *const i8 })
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
                >(Some(buffered_traverse))
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
                >(Some(buffered_clear))
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
                >(Some(buffered_iternext))
            };
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 64i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedrandom_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 72i32;
            __t7.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedrandom_members.0) as *mut PyMemberDef })
                    as *mut ();
            __t7
        },
        {
            let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t8.slot = 73i32;
            __t8.pfunc =
                (unsafe { core::ptr::addr_of_mut!(bufferedrandom_getset.0) as *mut PyGetSetDef })
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
                >(Some(_io_BufferedRandom___init__))
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
