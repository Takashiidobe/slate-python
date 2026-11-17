//!  C implementation of the datetime module
//! The size of 'long', as computed by sizeof.
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
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! Total tool ids available
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
//! # of bytes for year, month, and day.
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
// Define macros for inline documentation.
// Round up size "n" to be a multiple of "a".
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! ADD_TYPE__4 {
    ($FIELD:ident, $SPEC:expr, $BASE:expr, $module:expr, $st:expr) => {{
        loop {
            let cls: *mut _object =
                unsafe { PyType_FromModuleAndSpec($module, $SPEC, ($BASE) as *mut _object) };
            if cls.is_null() {
                return 1i32.wrapping_neg();
            }
            unsafe {
                (*$st).$FIELD = cls as *mut _typeobject;
            }
            if true {
                break;
            }
        }
    }};
}

macro_rules! CONST_EPOCH {
    ($st:expr) => {
        unsafe { (*$st).epoch }
    };
}

macro_rules! CONST_SEC_PER_DAY {
    ($st:expr) => {
        unsafe { (*$st).seconds_per_day }
    };
}

macro_rules! CONST_US_PER_DAY {
    ($st:expr) => {
        unsafe { (*$st).us_per_day }
    };
}

macro_rules! CONST_US_PER_HOUR {
    ($st:expr) => {
        unsafe { (*$st).us_per_hour }
    };
}

macro_rules! CONST_US_PER_MINUTE {
    ($st:expr) => {
        unsafe { (*$st).us_per_minute }
    };
}

macro_rules! CONST_US_PER_MS {
    ($st:expr) => {
        unsafe { (*$st).us_per_ms }
    };
}

macro_rules! CONST_US_PER_SECOND {
    ($st:expr) => {
        unsafe { (*$st).us_per_second }
    };
}

macro_rules! CONST_US_PER_WEEK {
    ($st:expr) => {
        unsafe { (*$st).us_per_week }
    };
}

macro_rules! CONST_UTC {
    () => {
        (unsafe { core::ptr::addr_of_mut!(utc_timezone) }) as *mut _object
    };
}

macro_rules! DATETIME_TYPE {
    () => {
        unsafe { core::ptr::addr_of_mut!(PyDateTime_DateTimeType) }
    };
}

macro_rules! DATE_SET_FOLD {
    ($o:expr, $v:expr) => {
        unsafe {
            (*$o).fold = ($v) as u8;
        }
    };
}

/// Date/Time accessors for datetime.
macro_rules! DATE_SET_HOUR {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) = ($v) as u8;
        }
    };
}

macro_rules! DATE_SET_MICROSECOND {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) =
                ((($v) & 16711680) >> 16i32) as u8;
        }
        let __v: u8 = ((($v) & 65280) >> 8i32) as u8;
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) = __v;
        }
        let __v2: u8 = (($v) & 255) as u8;
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) = __v2;
        }
    };
}

macro_rules! DATE_SET_MINUTE {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) = ($v) as u8;
        }
    };
}

macro_rules! DATE_SET_SECOND {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) = ($v) as u8;
        }
    };
}

macro_rules! DATE_TYPE {
    () => {
        unsafe { core::ptr::addr_of_mut!(PyDateTime_DateType) }
    };
}

macro_rules! DELTA_TYPE {
    () => {
        unsafe { core::ptr::addr_of_mut!(PyDateTime_DeltaType) }
    };
}

macro_rules! FT_ATOMIC_STORE_SSIZE_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! GET_CURRENT_STATE {
    ($MOD_VAR:expr) => {
        _get_current_state(core::ptr::addr_of_mut!($MOD_VAR))
    };
}

/// Delta accessors for timedelta.
macro_rules! GET_TD_DAYS__2 {
    ($o:expr) => {
        unsafe { (*PyDelta_CAST__2!($o)).days }
    };
}

/// Delta accessors for timedelta.
macro_rules! GET_TD_DAYS {
    ($o:expr) => {
        unsafe { (*PyDelta_CAST!($o)).days }
    };
}

macro_rules! GET_TD_MICROSECONDS {
    ($o:expr) => {
        unsafe { (*PyDelta_CAST!($o)).microseconds }
    };
}

macro_rules! GET_TD_MICROSECONDS__2 {
    ($o:expr) => {
        unsafe { (*PyDelta_CAST__2!($o)).microseconds }
    };
}

macro_rules! GET_TD_SECONDS__2 {
    ($o:expr) => {
        unsafe { (*PyDelta_CAST__2!($o)).seconds }
    };
}

macro_rules! GET_TD_SECONDS {
    ($o:expr) => {
        unsafe { (*PyDelta_CAST!($o)).seconds }
    };
}

macro_rules! ISOCALENDAR_DATE_TYPE {
    ($st:expr) => {
        unsafe { (*$st).isocalendar_date_type }
    };
}

/// M is a char or int claiming to be a valid month.  The macro is equivalent
/// to the two-sided Python test
///      1 <= M <= 12
macro_rules! MONTH_IS_SANE {
    ($M:expr) => {
        (($M) as u32).wrapping_sub(1) < 12
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

macro_rules! PyDateTime_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_datetime_h_114
    };
}

macro_rules! PyDateTime_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, DATETIME_TYPE!())
    };
}

macro_rules! PyDate_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_datetime_h_98
    };
}

macro_rules! PyDate_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, DATE_TYPE!())
    };
}

macro_rules! PyDelta_CAST__2 {
    ($op:expr) => {
        $op
    };
}

macro_rules! PyDelta_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_datetime_h_41
    };
}

macro_rules! PyDelta_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, DELTA_TYPE!())
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

macro_rules! PyTZInfo_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, TZINFO_TYPE!())
    };
}

macro_rules! PyTimeZone_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord__datetimemodule_c_1414
    };
}

macro_rules! PyTime_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_datetime_h_86
    };
}

macro_rules! PyTime_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, TIME_TYPE!())
    };
}

macro_rules! PyTimezone_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, TIMEZONE_TYPE!())
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

macro_rules! PyTuple_GET_ITEM__3 {
    ($op:expr, $index:expr) => {
        unsafe {
            *((*_PyTuple_CAST!($op)).ob_item.as_mut_ptr() as *mut *mut _object)
                .wrapping_offset((($index) as i32) as isize)
        }
    };
}

macro_rules! PyTuple_GET_SIZE {
    ($op:expr) => {
        PyTuple_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyTuple_SET_ITEM__3 {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(
            _PyObject_CAST__2!($op),
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

macro_rules! PyUnicode_READ_CHAR {
    ($unicode:expr, $index:expr) => {
        PyUnicode_READ_CHAR(_PyObject_CAST!($unicode), $index)
    };
}

macro_rules! PyUnicode_READ__2 {
    ($kind:expr, $data:expr, $index:expr) => {
        PyUnicode_READ(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_STATIC_CAST__2!($data),
            ($index) as i64,
        )
    };
}

macro_rules! Py_ARRAY_LENGTH__3 {
    () => {
        (core::mem::size_of::<[[*const i8; 2]; 5]>() as u64)
            / (core::mem::size_of::<[*const i8; 2]>() as u64)
    };
}

macro_rules! Py_ARRAY_LENGTH__8 {
    () => {
        (core::mem::size_of::<[*mut _typeobject; 6]>() as u64)
            / (core::mem::size_of::<*mut _typeobject>() as u64)
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

macro_rules! RELEASE_CURRENT_STATE {
    ($MOD_VAR:expr) => {
        Py_DECREF!($MOD_VAR);
    };
}

macro_rules! SET_DAY {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) = ($v) as u8;
        }
    };
}

macro_rules! SET_MONTH {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) = ($v) as u8;
        }
    };
}

macro_rules! SET_TD_DAYS {
    ($o:expr, $v:expr) => {
        unsafe {
            (*$o).days = $v;
        }
    };
}

macro_rules! SET_TD_MICROSECONDS {
    ($o:expr, $v:expr) => {
        unsafe {
            (*$o).microseconds = $v;
        }
    };
}

macro_rules! SET_TD_SECONDS {
    ($o:expr, $v:expr) => {
        unsafe {
            (*$o).seconds = $v;
        }
    };
}

/// Date accessors for date and datetime.
macro_rules! SET_YEAR {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) =
                ((($v) & 65280) >> 8i32) as u8;
        }
        let __v: u8 = (($v) & 255) as u8;
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) = __v;
        }
    };
}

macro_rules! TIMEZONE_TYPE {
    () => {
        unsafe { core::ptr::addr_of_mut!(PyDateTime_TimeZoneType) }
    };
}

macro_rules! TIME_SET_FOLD {
    ($o:expr, $v:expr) => {
        unsafe {
            (*$o).fold = ($v) as u8;
        }
    };
}

macro_rules! TIME_SET_HOUR {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) = ($v) as u8;
        }
    };
}

macro_rules! TIME_SET_MICROSECOND {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) =
                ((($v) & 16711680) >> 16i32) as u8;
        }
        let __v: u8 = ((($v) & 65280) >> 8i32) as u8;
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) = __v;
        }
        let __v2: u8 = (($v) & 255) as u8;
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) = __v2;
        }
    };
}

macro_rules! TIME_SET_MINUTE {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) = ($v) as u8;
        }
    };
}

macro_rules! TIME_SET_SECOND {
    ($o:expr, $v:expr) => {
        unsafe {
            *((*$o).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) = ($v) as u8;
        }
    };
}

macro_rules! TIME_TYPE {
    () => {
        unsafe { core::ptr::addr_of_mut!(PyDateTime_TimeType) }
    };
}

macro_rules! TZINFO_TYPE {
    () => {
        unsafe { core::ptr::addr_of_mut!(PyDateTime_TZInfoType) }
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
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

/// o is a pointer to a time or a datetime object.
macro_rules! _PyDateTime_HAS_TZINFO {
    ($o:expr) => {
        unsafe { (*(($o) as *mut __SlateRecord_datetime_h_64)).hastzinfo }
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

macro_rules! _PyStatus_GET_FUNC__37 {
    () => {
        c"_PyDateTime_InitTypes".as_ptr() as *mut i8
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

macro_rules! microseconds_to_delta {
    ($pymicros:expr) => {
        microseconds_to_delta_ex($pymicros, DELTA_TYPE!())
    };
}

macro_rules! new_date {
    ($year:expr, $month:expr, $day:expr) => {
        new_date_ex($year, $month, $day, DATE_TYPE!())
    };
}

macro_rules! new_date__2 {
    ($year:expr, $month:expr, $day:expr) => {
        new_date_ex($year, ($month) as i32, ($day) as i32, DATE_TYPE!())
    };
}

macro_rules! new_datetime {
    ($y:expr, $m:expr, $d:expr, $hh:expr, $mm:expr, $ss:expr, $us:expr, $tzinfo:expr, $fold:expr) => {
        new_datetime_ex2(
            $y,
            $m,
            $d,
            $hh,
            $mm,
            $ss,
            $us,
            $tzinfo,
            $fold,
            DATETIME_TYPE!(),
        )
    };
}

macro_rules! new_delta {
    ($d:expr, $s:expr, $us:expr, $normalize:expr) => {
        new_delta_ex($d, $s, $us, $normalize, DELTA_TYPE!())
    };
}

macro_rules! new_delta__2 {
    ($d:expr, $s:expr, $us:expr, $normalize:expr) => {
        new_delta_ex($d, ($s) as i32, $us, $normalize, DELTA_TYPE!())
    };
}

macro_rules! new_time__2 {
    ($hh:expr, $mm:expr, $ss:expr, $us:expr, $tzinfo:expr, $fold:expr) => {
        new_time_ex2(
            ($hh) as i32,
            ($mm) as i32,
            ($ss) as i32,
            $us,
            $tzinfo,
            ($fold) as i32,
            TIME_TYPE!(),
        )
    };
}

macro_rules! new_time {
    ($hh:expr, $mm:expr, $ss:expr, $us:expr, $tzinfo:expr, $fold:expr) => {
        new_time_ex2($hh, $mm, $ss, $us, $tzinfo, $fold, TIME_TYPE!())
    };
}

unsafe extern "C" {
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_NotImplementedError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_UnicodeEncodeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_DeprecationWarning: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut _PyRuntime: pyruntimestate;
    fn modf(arg0: f64, arg1: *mut f64) -> f64;
    fn fabs(arg0: f64) -> f64;
    fn remainder(arg0: f64, arg1: f64) -> f64;
    fn round(arg0: f64) -> f64;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyType_FromModuleAndSpec(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_object_h_385,
        arg2: *mut _object,
    ) -> *mut _object;
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericNew(
        arg0: *mut _typeobject,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn PyObject_Str(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetOptionalAttr(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_Hash(arg0: *mut _object) -> i64;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn _Py_NewReference(arg0: *mut _object);
    fn _PyType_Name(arg0: *mut _typeobject) -> *const i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Malloc(arg0: u64) -> *mut ();
    fn Py_HashBuffer(arg0: *const (), arg1: i64) -> i64;
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_Substring(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyUnicode_GetLength(arg0: *mut _object) -> i64;
    fn PyUnicode_WriteChar(arg0: *mut _object, arg1: i64, arg2: u32) -> i32;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_AsLatin1String(arg0: *mut _object) -> *mut _object;
    fn PyUnicode_DecodeLocale(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn PyUnicode_AppendAndDel(arg0: *mut *mut _object, arg1: *mut _object);
    fn PyUnicode_FindChar(arg0: *mut _object, arg1: u32, arg2: i64, arg3: i64, arg4: i32) -> i64;
    fn PyUnicodeWriter_Create(arg0: i64) -> *mut PyUnicodeWriter;
    fn PyUnicodeWriter_Discard(arg0: *mut PyUnicodeWriter);
    fn PyUnicodeWriter_Finish(arg0: *mut PyUnicodeWriter) -> *mut _object;
    fn PyUnicodeWriter_WriteUTF8(arg0: *mut PyUnicodeWriter, arg1: *const i8, arg2: i64) -> i32;
    fn PyUnicodeWriter_WriteStr(arg0: *mut PyUnicodeWriter, arg1: *mut _object) -> i32;
    fn PyUnicodeWriter_WriteSubstring(
        arg0: *mut PyUnicodeWriter,
        arg1: *mut _object,
        arg2: i64,
        arg3: i64,
    ) -> i32;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_SetRaisedException(arg0: *mut _object);
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyOS_snprintf(arg0: *mut i8, arg1: u64, arg2: *const i8, ...) -> i32;
    fn PyErr_FormatUnraisable(arg0: *const i8, ...);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromDouble(arg0: f64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyLong_AsDouble(arg0: *mut _object) -> f64;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Size(arg0: *mut _object) -> i64;
    fn PyTuple_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyDict_GetItemRef(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn PyCapsule_New(
        arg0: *mut (),
        arg1: *const i8,
        arg2: Option<unsafe extern "C-unwind" fn(*mut _object)>,
    ) -> *mut _object;
    fn PyInterpreterState_Get() -> *mut _is;
    fn PyInterpreterState_GetDict(arg0: *mut _is) -> *mut _object;
    fn PyErr_WarnEx(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn PyWeakref_NewRef(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyWeakref_GetRef(arg0: *mut _object, arg1: *mut *mut _object) -> i32;
    fn PyTime_Time(arg0: *mut i64) -> i32;
    fn PyArg_ParseTuple(arg0: *mut _object, arg1: *const i8, ...) -> i32;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
    fn PyArg_UnpackTuple(arg0: *mut _object, arg1: *const i8, arg2: i64, arg3: i64, ...) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn Py_VaBuildValue(arg0: *const i8, arg1: core::ffi::VaList<'static>) -> *mut _object;
    fn PyModule_AddObjectRef(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_Add(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_AddIntConstant(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn PyImport_ImportModule(arg0: *const i8) -> *mut _object;
    fn PyImport_Import(arg0: *mut _object) -> *mut _object;
    fn PyImport_ImportModuleAttrString(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn PyObject_CallNoArgs(arg0: *mut _object) -> *mut _object;
    fn PyObject_Call(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyObject_CallFunction(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyObject_CallMethod(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        ...
    ) -> *mut _object;
    fn PyObject_CallFunctionObjArgs(arg0: *mut _object, ...) -> *mut _object;
    fn PyObject_CallMethodObjArgs(arg0: *mut _object, arg1: *mut _object, ...) -> *mut _object;
    fn PyObject_VectorcallMethod(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyNumber_Add(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Multiply(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_FloorDivide(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_TrueDivide(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Remainder(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Divmod(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_And(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Long(arg0: *mut _object) -> *mut _object;
    fn PySequence_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn _PyLong_DivmodNear(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyStaticType_InitForExtension(arg0: *mut _is, arg1: *mut _typeobject) -> i32;
    fn _PyType_GetDict(arg0: *mut _typeobject) -> *mut _object;
    fn _PyObject_GetState(arg0: *mut _object) -> *mut _object;
    fn _PyLong_AsTime_t(arg0: *mut _object) -> i64;
    fn _PyTime_ObjectToTime_t(arg0: *mut _object, arg1: *mut i64, arg2: u32) -> i32;
    fn _PyTime_ObjectToTimeval(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: u32,
    ) -> i32;
    fn _PyTime_AsTimevalTime_t(arg0: i64, arg1: *mut i64, arg2: *mut i32, arg3: u32) -> i32;
    fn _PyTime_localtime(arg0: i64, arg1: *mut slate_support::tm) -> i32;
    fn _PyTime_gmtime(arg0: i64, arg1: *mut slate_support::tm) -> i32;
    fn _PyUnicode_Copy(arg0: *mut _object) -> *mut _object;
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

// Track types initialized using _PyStaticType_InitBuiltin().
// Set if the type object is immutable: type attributes cannot be set nor deleted
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
// Macro for implementing rich comparisons
//
// Needs to be a macro because any C-comparable type can be used.
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
    __slate_bits_0: __slate_bits::__SlateBits153U0,
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
struct PyUnicodeWriter {}

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

fn Py_UNICODE_IS_SURROGATE(ch: u32) -> i32 {
    (55296 <= ch && ch <= 57343) as i32
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits209U0,
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

fn PyTuple_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    unsafe {
        *((*tuple).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(index as isize) =
            value;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct code_arena_st {}

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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_hashtable_h_35 {
    _Py_slist_item: _Py_slist_item_s,
    key_hash: u64,
    key: *mut (),
    value: *mut (),
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct _Py_unicode_runtime_ids {
    mutex: PyMutex,
    next_index: i64,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct __SlateRecord_pycore_instruments_h_94 {
    bytes_per_entry: u8,
    data: [u8; 1],
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

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct tracemalloc_frame {
    filename: *mut _object,
    lineno: u32,
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
struct __SlateRecord__datetimemodule_c_h_52 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_h_126 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_h_215 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_h_337 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_h_405 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 9],
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
    pub struct __SlateBits209U0 {
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
    pub struct __SlateBits153U0 {
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

fn _PyModule_GetState(r#mod: *mut _object) -> *mut () {
    unsafe { (*(r#mod as *mut __SlateRecord_pycore_moduleobject_h_30)).md_state }
}

fn _PyObject_Init(op: *mut _object, typeobj: *mut _typeobject) {
    Py_SET_TYPE!(op, typeobj);
    Py_INCREF(_PyObject_CAST__2!(typeobj));
    unsafe { _Py_NewReference(op) };
}

// Fast inlined version of PyType_IS_GC()
// # of bytes for hour, minute, second, and usecond.
// # of bytes for year, month, day, hour, minute, second, and usecond.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_41 {
    ob_base: _object,
    hashcode: i64,
    days: i32,
    seconds: i32,
    microseconds: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_46 {
    ob_base: _object,
}

// The datetime and time types have hashcodes, and an optional tzinfo member,
// present if and only if hastzinfo is true.
// boolean flag
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_64 {
    ob_base: _object,
    hashcode: i64,
    hastzinfo: i8,
}

// All time objects are of PyDateTime_TimeType, but that can be allocated
// in two ways, with or without a tzinfo member.  Without is the same as
// tzinfo == None, but consumes less memory.  _PyDateTime_BaseTime is an
// internal struct used to allocate the right amount of space for the
// "without" case.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_79 {
    ob_base: _object,
    hashcode: i64,
    hastzinfo: i8,
    data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_86 {
    ob_base: _object,
    hashcode: i64,
    hastzinfo: i8,
    data: [u8; 6],
    fold: u8,
    tzinfo: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_98 {
    ob_base: _object,
    hashcode: i64,
    hastzinfo: i8,
    data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_107 {
    ob_base: _object,
    hashcode: i64,
    hastzinfo: i8,
    data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_114 {
    ob_base: _object,
    hashcode: i64,
    hastzinfo: i8,
    data: [u8; 10],
    fold: u8,
    tzinfo: *mut _object,
}

// Apply for time instances.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_datetime_h_186 {
    DateType: *mut _typeobject,
    DateTimeType: *mut _typeobject,
    TimeType: *mut _typeobject,
    DeltaType: *mut _typeobject,
    TZInfoType: *mut _typeobject,
    TimeZone_UTC: *mut _object,
    Date_FromDate:
        Option<unsafe extern "C-unwind" fn(i32, i32, i32, *mut _typeobject) -> *mut _object>,
    DateTime_FromDateAndTime: Option<
        unsafe extern "C-unwind" fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            *mut _object,
            *mut _typeobject,
        ) -> *mut _object,
    >,
    Time_FromTime: Option<
        unsafe extern "C-unwind" fn(
            i32,
            i32,
            i32,
            i32,
            *mut _object,
            *mut _typeobject,
        ) -> *mut _object,
    >,
    Delta_FromDelta:
        Option<unsafe extern "C-unwind" fn(i32, i32, i32, i32, *mut _typeobject) -> *mut _object>,
    TimeZone_FromTimeZone:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    DateTime_FromTimestamp: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> *mut _object,
    >,
    Date_FromTimestamp:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    DateTime_FromDateAndTimeAndFold: Option<
        unsafe extern "C-unwind" fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            *mut _object,
            i32,
            *mut _typeobject,
        ) -> *mut _object,
    >,
    Time_FromTimeAndFold: Option<
        unsafe extern "C-unwind" fn(
            i32,
            i32,
            i32,
            i32,
            *mut _object,
            i32,
            *mut _typeobject,
        ) -> *mut _object,
    >,
}

// bpo-35081: Defining this prevents including the C API capsule;
// internal versions of the  Py*_Check macros which do not require
// the capsule are defined below
// _PyLong_GetOne()
// _PyObject_Init()
// _PyTime_ObjectToTime_t()
// _PyUnicode_Copy()
// _PyStatus_OK()
/// forward declarations
static mut PyDateTime_DateType: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"datetime.date\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_datetime_h_98>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = None;
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(date_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(date_as_number) };
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(date_hash);
    __t0.tp_call = None;
    __t0.tp_str = Some(date_str);
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(date_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(date_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(date_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(date_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(date_new);
    __t0.tp_free = None;
    __t0
};

static mut PyDateTime_DateTimeType: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"datetime.datetime\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_datetime_h_114>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(datetime_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(datetime_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(datetime_as_number) };
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(datetime_hash);
    __t0.tp_call = None;
    __t0.tp_str = Some(datetime_str);
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(datetime_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(datetime_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(datetime_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(datetime_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(PyDateTime_DateType) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = Some(datetime_alloc);
    __t0.tp_new = Some(datetime_new);
    __t0.tp_free = None;
    __t0
};

static mut PyDateTime_TimeType: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"datetime.time\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_datetime_h_86>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(time_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(time_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(time_hash);
    __t0.tp_call = None;
    __t0.tp_str = Some(time_str);
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(time_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(time_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(time_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(time_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = Some(time_alloc);
    __t0.tp_new = Some(time_new);
    __t0.tp_free = None;
    __t0
};

static mut PyDateTime_DeltaType: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"datetime.timedelta\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_datetime_h_41>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = None;
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(delta_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(delta_as_number) };
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(delta_hash);
    __t0.tp_call = None;
    __t0.tp_str = Some(delta_str);
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(delta_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(delta_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(delta_methods.0) as *mut PyMethodDef };
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(delta_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(delta_new);
    __t0.tp_free = None;
    __t0
};

static mut PyDateTime_TZInfoType: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"datetime.tzinfo\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_datetime_h_46>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = None;
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
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(tzinfo_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(tzinfo_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
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
    __t0.tp_free = None;
    __t0
};

static mut PyDateTime_TimeZoneType: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"datetime.timezone\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize =
        (core::mem::size_of::<__SlateRecord__datetimemodule_c_1414>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(timezone_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(timezone_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(timezone_hash);
    __t0.tp_call = None;
    __t0.tp_str = Some(timezone_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 0u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(timezone_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(timezone_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(timezone_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(PyDateTime_TZInfoType) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(timezone_new);
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_54 {
    /// Module heap types.
    isocalendar_date_type: *mut _typeobject,
    /// Conversion factors.
    /// 1_000
    us_per_ms: *mut _object,
    /// 1_000_000
    us_per_second: *mut _object,
    /// 1e6 * 60 as Python int
    us_per_minute: *mut _object,
    /// 1e6 * 3600 as Python int
    us_per_hour: *mut _object,
    /// 1e6 * 3600 * 24 as Python int
    us_per_day: *mut _object,
    /// 1e6 * 3600 * 24 * 7 as Python int
    us_per_week: *mut _object,
    /// 3600 * 24 as Python int
    seconds_per_day: *mut _object,
    /// The interned Unix epoch datetime instance
    epoch: *mut _object,
}

// The module has a fixed number of static objects, due to being exposed
// through the datetime C-API.  There are five types exposed directly,
// one type exposed indirectly, and one singleton constant (UTC).
//
// Each of these objects is hidden behind a macro in the same way as
// the per-module objects stored in module state.  The macros for the
// static objects don't need to be passed a state, but the consistency
// of doing so is more clear.  We use a dedicated noop macro, NO_STATE,
// to make the special case obvious.
//
// The casting macros perform a simple fast pointer cast without
// checking the runtime type. In the future, we may decide whether
// to include that check and whether to provide a fast pointer cast
// macro for pointers known to be of correct time.
fn get_module_state(module: *mut _object) -> *mut __SlateRecord__datetimemodule_c_54 {
    let state: *mut () = _PyModule_GetState(module);
    state as *mut __SlateRecord__datetimemodule_c_54
}

fn get_current_module(interp: *mut _is, p_mod: &mut *mut _object) -> i32 {
    let mut r#ref: *mut _object = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            r#mod = core::ptr::null_mut::<_object>();
            dict = PyInterpreterState_GetDict(interp);
            if !dict.is_null() {
                r#ref = core::ptr::null_mut::<_object>();
                if PyDict_GetItemRef(
                    dict,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_cached_datetime_module
                            ._ascii
                            .ob_base
                    ),
                    core::ptr::addr_of_mut!(r#ref),
                ) >= 0
                {
                    if !r#ref.is_null() && r#ref != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                        if PyWeakref_GetRef(r#ref, core::ptr::addr_of_mut!(r#mod)) < 0 {
                            Py_DECREF!(r#ref);
                            break '__join_0;
                        } else {
                            if r#mod == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                Py_CLEAR__2!(r#mod);
                            }
                            Py_DECREF!(r#ref);
                        }
                    }
                    *(p_mod as *mut *mut _object) = r#mod;
                    return !r#mod.is_null() as i32;
                }
            }
        }
        *(p_mod as *mut *mut _object) = core::ptr::null_mut::<_object>();
        return 1i32.wrapping_neg();
    }
}

static mut datetimemodule: PyModuleDef = PyModuleDef {
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
    m_name: (b"_datetime\0".as_ptr() as *mut i8) as *const i8,
    m_doc: (b"Fast implementation of the datetime module.\0".as_ptr() as *mut i8) as *const i8,
    m_size: (core::mem::size_of::<__SlateRecord__datetimemodule_c_54>() as u64) as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(module_methods.0) as *mut PyMethodDef },
    m_slots: unsafe { core::ptr::addr_of_mut!(module_slots.0) as *mut PyModuleDef_Slot },
    m_traverse: Some(module_traverse),
    m_clear: Some(module_clear),
    m_free: Some(module_free),
};

fn _get_current_state(p_mod: *mut *mut _object) -> *mut __SlateRecord__datetimemodule_c_54 {
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            let interp: *mut _is = PyInterpreterState_Get();
            if get_current_module(interp, &mut (*core::ptr::addr_of_mut!(r#mod))) >= 0 {
                if r#mod.is_null() {
                    // The static types can outlive the module,
                    // so we must re-import the module.
                    r#mod = PyImport_ImportModule(c"_datetime".as_ptr());
                    if r#mod.is_null() {
                        break '__join_0;
                    }
                }
                let st: *mut __SlateRecord__datetimemodule_c_54 = get_module_state(r#mod);
                *p_mod = r#mod;
                return st;
            }
        }
        *p_mod = core::ptr::null_mut::<_object>();
        return core::ptr::null_mut::<__SlateRecord__datetimemodule_c_54>();
    }
}

fn set_current_module(interp: *mut _is, r#mod: *mut _object) -> i32 {
    let dict: *mut _object = unsafe { PyInterpreterState_GetDict(interp) };
    if dict.is_null() {
        return 1i32.wrapping_neg();
    }
    let r#ref: *mut _object = unsafe { PyWeakref_NewRef(r#mod, core::ptr::null_mut::<_object>()) };
    if r#ref.is_null() {
        return 1i32.wrapping_neg();
    }
    let rc: i32 = unsafe {
        PyDict_SetItem(
            dict,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_cached_datetime_module
                    ._ascii
                    .ob_base
            ),
            r#ref,
        )
    };
    Py_DECREF!(r#ref);
    rc
}

fn clear_current_module(interp: *mut _is, expected: *mut _object) {
    let mut current: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#ref: *mut _object = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            '__join_1: {
                exc = PyErr_GetRaisedException();
                dict = PyInterpreterState_GetDict(interp);
                if !dict.is_null() {
                    if !expected.is_null() {
                        r#ref = core::ptr::null_mut::<_object>();
                        if PyDict_GetItemRef(
                            dict,
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_cached_datetime_module
                                    ._ascii
                                    .ob_base
                            ),
                            core::ptr::addr_of_mut!(r#ref),
                        ) < 0
                        {
                            break '__join_1;
                        } else {
                            if !r#ref.is_null() {
                                current = core::ptr::null_mut::<_object>();
                                let rc: i32 =
                                    PyWeakref_GetRef(r#ref, core::ptr::addr_of_mut!(current));
                                // We only need "current" for pointer comparison.
                                Py_XDECREF!(current);
                                Py_DECREF!(r#ref);
                                if rc < 0 {
                                    break '__join_1;
                                } else {
                                    if current != expected {
                                        break '__join_0;
                                    }
                                }
                            }
                        }
                    }
                    // We use None to identify that the module was previously loaded.
                    if PyDict_SetItem(
                        dict,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py_cached_datetime_module
                                ._ascii
                                .ob_base
                        ),
                        core::ptr::addr_of_mut!(_Py_NoneStruct),
                    ) >= 0
                    {
                        break '__join_0;
                    }
                }
            }
            PyErr_FormatUnraisable(c"Exception ignored while clearing _datetime module".as_ptr());
        }
        PyErr_SetRaisedException(exc);
    }
}

static datetime_date_fromtimestamp__doc__: __SlateAlign16<[i8; 170]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 116i8, 105i8, 109i8, 101i8, 115i8, 116i8, 97i8, 109i8, 112i8, 40i8,
    36i8, 116i8, 121i8, 112i8, 101i8, 44i8, 32i8, 116i8, 105i8, 109i8, 101i8, 115i8, 116i8, 97i8,
    109i8, 112i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 114i8, 101i8, 97i8,
    116i8, 101i8, 32i8, 97i8, 32i8, 100i8, 97i8, 116i8, 101i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 97i8, 32i8, 80i8, 79i8, 83i8, 73i8, 88i8, 32i8, 116i8, 105i8, 109i8, 101i8, 115i8, 116i8,
    97i8, 109i8, 112i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 116i8, 105i8, 109i8, 101i8,
    115i8, 116i8, 97i8, 109i8, 112i8, 32i8, 105i8, 115i8, 32i8, 97i8, 32i8, 110i8, 117i8, 109i8,
    98i8, 101i8, 114i8, 44i8, 32i8, 101i8, 46i8, 103i8, 46i8, 32i8, 99i8, 114i8, 101i8, 97i8,
    116i8, 101i8, 100i8, 32i8, 118i8, 105i8, 97i8, 32i8, 116i8, 105i8, 109i8, 101i8, 46i8, 116i8,
    105i8, 109i8, 101i8, 40i8, 41i8, 44i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 105i8, 115i8,
    10i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 100i8, 32i8, 97i8,
    115i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 32i8, 116i8, 105i8, 109i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_date_fromtimestamp")]
extern "C-unwind" fn datetime_date_fromtimestamp(
    r#type: *mut _object,
    timestamp: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = datetime_date_fromtimestamp_impl(r#type as *mut _typeobject, timestamp);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_new")]
extern "C-unwind" fn iso_calendar_date_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut weekday: i32 = unsafe { core::mem::zeroed() };
    let mut week: i32 = unsafe { core::mem::zeroed() };
    let mut year: i32 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let nargs: i64 = PyTuple_GET_SIZE!(args);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__65),
            3i32,
            3i32,
            0i32,
            0i32,
            (*__slate_slot_8).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            year = PyLong_AsInt(*fastargs.wrapping_offset(0 as isize));
            if year == 1i32.wrapping_neg() {
                __v2 = !PyErr_Occurred().is_null();
            } else {
                __v2 = false;
            }
            if !__v2 {
                week = PyLong_AsInt(*fastargs.wrapping_offset(1 as isize));
                if week == 1i32.wrapping_neg() {
                    __v1 = !PyErr_Occurred().is_null();
                } else {
                    __v1 = false;
                }
                if !__v1 {
                    weekday = PyLong_AsInt(*fastargs.wrapping_offset(2 as isize));
                    if weekday == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if !__v0 {
                        return_value = iso_calendar_date_new_impl(r#type, year, week, weekday);
                    }
                }
            }
        }
        return return_value;
    }
}

static mut _kwtuple__52: __SlateRecord__datetimemodule_c_h_52 = {
    let mut __t0: __SlateRecord__datetimemodule_c_h_52 = unsafe { core::mem::zeroed() };
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
                    ._py_year
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
                    ._py_week
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
                    ._py_weekday
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__64: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"year\0".as_ptr() as *mut i8) as *const i8,
    (b"week\0".as_ptr() as *mut i8) as *const i8,
    (b"weekday\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__65: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__64.0) as *const *const i8 };
    __t0.fname = (b"IsoCalendarDate\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__52.ob_base.ob_base) };
    __t0
};

static datetime_date_replace__doc__: __SlateAlign16<[i8; 109]> = __SlateAlign16([
    114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 121i8, 101i8, 97i8, 114i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8,
    110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 109i8, 111i8, 110i8, 116i8, 104i8, 61i8, 117i8, 110i8,
    99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 100i8, 97i8, 121i8, 61i8, 117i8,
    110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 100i8, 97i8, 116i8, 101i8, 32i8, 119i8, 105i8, 116i8,
    104i8, 32i8, 110i8, 101i8, 119i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8,
    100i8, 32i8, 102i8, 105i8, 101i8, 108i8, 100i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_date_replace")]
extern "C-unwind" fn datetime_date_replace(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v6: bool = unsafe { core::mem::zeroed() };
    let mut day: i32 = unsafe { core::mem::zeroed() };
    let mut month: i32 = unsafe { core::mem::zeroed() };
    let mut year: i32 = unsafe { core::mem::zeroed() };
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
            year = (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize) as i32)
                << 8i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize) as i32);
            month = *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize) as i32;
            day = *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize) as i32;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__139),
                0i32,
                3i32,
                0i32,
                0i32,
                (*__slate_slot_12).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            year = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                            if year == 1i32.wrapping_neg() {
                                __v6 = !PyErr_Occurred().is_null();
                            } else {
                                __v6 = false;
                            }
                            if __v6 {
                                break '__join_0;
                            } else {
                                let __v4: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v4;
                                if __v4 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                            month = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                            if month == 1i32.wrapping_neg() {
                                __v3 = !PyErr_Occurred().is_null();
                            } else {
                                __v3 = false;
                            }
                            if __v3 {
                                break '__join_0;
                            } else {
                                let __v1: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v1;
                                if __v1 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        day = PyLong_AsInt(*args.wrapping_offset(2 as isize));
                        if day == 1i32.wrapping_neg() {
                            __v0 = !PyErr_Occurred().is_null();
                        } else {
                            __v0 = false;
                        }
                        if __v0 {
                            break '__join_0;
                        }
                    }
                }
                return_value = datetime_date_replace_impl(
                    self_ as *mut __SlateRecord_datetime_h_98,
                    year,
                    month,
                    day,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__126: __SlateRecord__datetimemodule_c_h_126 = {
    let mut __t0: __SlateRecord__datetimemodule_c_h_126 = unsafe { core::mem::zeroed() };
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
                    ._py_year
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
                    ._py_month
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
                    ._py_day
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__138: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"year\0".as_ptr() as *mut i8) as *const i8,
    (b"month\0".as_ptr() as *mut i8) as *const i8,
    (b"day\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__139: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__138.0) as *const *const i8 };
    __t0.fname = (b"replace\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__126.ob_base.ob_base) };
    __t0
};

static datetime_time_replace__doc__: __SlateAlign16<[i8; 181]> = __SlateAlign16([
    114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 104i8, 111i8, 117i8, 114i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8,
    110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 109i8, 105i8, 110i8, 117i8, 116i8, 101i8, 61i8, 117i8,
    110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 115i8, 101i8, 99i8, 111i8,
    110i8, 100i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 10i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 109i8, 105i8, 99i8, 114i8, 111i8, 115i8, 101i8,
    99i8, 111i8, 110i8, 100i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8,
    44i8, 32i8, 116i8, 122i8, 105i8, 110i8, 102i8, 111i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8,
    110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 42i8, 44i8, 32i8, 102i8, 111i8, 108i8, 100i8, 61i8,
    117i8, 110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 105i8, 109i8, 101i8, 32i8, 119i8,
    105i8, 116i8, 104i8, 32i8, 110i8, 101i8, 119i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 101i8, 100i8, 32i8, 102i8, 105i8, 101i8, 108i8, 100i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_time_replace")]
extern "C-unwind" fn datetime_time_replace(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut __v11: bool = unsafe { core::mem::zeroed() };
    let mut __v14: bool = unsafe { core::mem::zeroed() };
    let mut fold: i32 = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut microsecond: i32 = unsafe { core::mem::zeroed() };
    let mut second: i32 = unsafe { core::mem::zeroed() };
    let mut minute: i32 = unsafe { core::mem::zeroed() };
    let mut hour: i32 = unsafe { core::mem::zeroed() };
    let mut __v21: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 6]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_23: *mut [*mut _object; 6] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 6];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v21 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v21 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v21).wrapping_sub(0);
            hour = *((*(self_ as *mut __SlateRecord_datetime_h_86))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize) as i32;
            minute = *((*(self_ as *mut __SlateRecord_datetime_h_86))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(1 as isize) as i32;
            second = *((*(self_ as *mut __SlateRecord_datetime_h_86))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize) as i32;
            microsecond = (*((*(self_ as *mut __SlateRecord_datetime_h_86))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize) as i32)
                << 16i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(4 as isize) as i32)
                    << 8i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(5 as isize) as i32);
            tzinfo = if (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo != 0 {
                (*(self_ as *mut __SlateRecord_datetime_h_86)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            };
            fold = (*(self_ as *mut __SlateRecord_datetime_h_86)).fold as i32;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__228),
                0i32,
                5i32,
                0i32,
                0i32,
                (*__slate_slot_23).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_6: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            hour = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                            if hour == 1i32.wrapping_neg() {
                                __v14 = !PyErr_Occurred().is_null();
                            } else {
                                __v14 = false;
                            }
                            if __v14 {
                                break '__join_0;
                            } else {
                                let __v12: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v12;
                                if __v12 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                            minute = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                            if minute == 1i32.wrapping_neg() {
                                __v11 = !PyErr_Occurred().is_null();
                            } else {
                                __v11 = false;
                            }
                            if __v11 {
                                break '__join_0;
                            } else {
                                let __v9: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v9;
                                if __v9 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(2 as isize) }.is_null() {
                            second = PyLong_AsInt(*args.wrapping_offset(2 as isize));
                            if second == 1i32.wrapping_neg() {
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
                        if !unsafe { *args.wrapping_offset(3 as isize) }.is_null() {
                            microsecond = PyLong_AsInt(*args.wrapping_offset(3 as isize));
                            if microsecond == 1i32.wrapping_neg() {
                                __v5 = !PyErr_Occurred().is_null();
                            } else {
                                __v5 = false;
                            }
                            if __v5 {
                                break '__join_0;
                            } else {
                                let __v3: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v3;
                                if __v3 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(4 as isize) }.is_null() {
                            tzinfo = *args.wrapping_offset(4 as isize);
                            noptargs = noptargs.wrapping_sub(1);
                        }
                    }
                }
                if noptargs != 0 {
                    fold = PyLong_AsInt(*args.wrapping_offset(5 as isize));
                    if fold == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = datetime_time_replace_impl(
                    self_ as *mut __SlateRecord_datetime_h_86,
                    hour,
                    minute,
                    second,
                    microsecond,
                    tzinfo,
                    fold,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__215: __SlateRecord__datetimemodule_c_h_215 = {
    let mut __t0: __SlateRecord__datetimemodule_c_h_215 = unsafe { core::mem::zeroed() };
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
                    ._py_hour
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
                    ._py_minute
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
                    ._py_second
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
                    ._py_microsecond
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
                    ._py_tzinfo
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
                    ._py_fold
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__227: __SlateAlign16<[*const i8; 7]> = __SlateAlign16([
    (b"hour\0".as_ptr() as *mut i8) as *const i8,
    (b"minute\0".as_ptr() as *mut i8) as *const i8,
    (b"second\0".as_ptr() as *mut i8) as *const i8,
    (b"microsecond\0".as_ptr() as *mut i8) as *const i8,
    (b"tzinfo\0".as_ptr() as *mut i8) as *const i8,
    (b"fold\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__228: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__227.0) as *const *const i8 };
    __t0.fname = (b"replace\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__215.ob_base.ob_base) };
    __t0
};

static datetime_datetime_now__doc__: __SlateAlign16<[i8; 166]> = __SlateAlign16([
    110i8, 111i8, 119i8, 40i8, 36i8, 116i8, 121i8, 112i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    116i8, 122i8, 61i8, 78i8, 111i8, 110i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 110i8, 101i8, 119i8, 32i8, 100i8, 97i8, 116i8, 101i8,
    116i8, 105i8, 109i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 114i8, 101i8,
    112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 105i8, 110i8, 103i8, 32i8, 99i8, 117i8, 114i8,
    114i8, 101i8, 110i8, 116i8, 32i8, 116i8, 105i8, 109i8, 101i8, 32i8, 108i8, 111i8, 99i8, 97i8,
    108i8, 32i8, 116i8, 111i8, 32i8, 116i8, 122i8, 46i8, 10i8, 10i8, 32i8, 32i8, 116i8, 122i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 84i8, 105i8, 109i8, 101i8, 122i8, 111i8, 110i8, 101i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 110i8, 111i8,
    32i8, 116i8, 122i8, 32i8, 105i8, 115i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8,
    101i8, 100i8, 44i8, 32i8, 117i8, 115i8, 101i8, 115i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8,
    32i8, 116i8, 105i8, 109i8, 101i8, 122i8, 111i8, 110i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_datetime_now")]
extern "C-unwind" fn datetime_datetime_now(
    r#type: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut tz: *mut _object = unsafe { core::mem::zeroed() };
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
        noptargs = nargs.wrapping_add(__v1).wrapping_sub(0);
        tz = core::ptr::addr_of_mut!(_Py_NoneStruct);
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__350),
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
                tz = *args.wrapping_offset(0 as isize);
            }
            return_value = datetime_datetime_now_impl(r#type as *mut _typeobject, tz);
        }
        return return_value;
    }
}

static mut _kwtuple__337: __SlateRecord__datetimemodule_c_h_337 = {
    let mut __t0: __SlateRecord__datetimemodule_c_h_337 = unsafe { core::mem::zeroed() };
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
                ._py_tz
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__349: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"tz\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__350: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__349.0) as *const *const i8 };
    __t0.fname = (b"now\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__337.ob_base.ob_base) };
    __t0
};

static datetime_datetime_replace__doc__: __SlateAlign16<[i8; 241]> = __SlateAlign16([
    114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 121i8, 101i8, 97i8, 114i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8,
    110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 109i8, 111i8, 110i8, 116i8, 104i8, 61i8, 117i8, 110i8,
    99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 100i8, 97i8, 121i8, 61i8, 117i8,
    110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 104i8, 111i8, 117i8, 114i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8,
    103i8, 101i8, 100i8, 44i8, 32i8, 109i8, 105i8, 110i8, 117i8, 116i8, 101i8, 61i8, 117i8, 110i8,
    99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 32i8, 115i8, 101i8, 99i8, 111i8, 110i8,
    100i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 109i8, 105i8, 99i8, 114i8, 111i8, 115i8, 101i8, 99i8,
    111i8, 110i8, 100i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 44i8,
    32i8, 116i8, 122i8, 105i8, 110i8, 102i8, 111i8, 61i8, 117i8, 110i8, 99i8, 104i8, 97i8, 110i8,
    103i8, 101i8, 100i8, 44i8, 32i8, 42i8, 44i8, 32i8, 102i8, 111i8, 108i8, 100i8, 61i8, 117i8,
    110i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 100i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 100i8, 97i8, 116i8, 101i8, 116i8, 105i8, 109i8, 101i8,
    32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 110i8, 101i8, 119i8, 32i8, 115i8, 112i8, 101i8, 99i8,
    105i8, 102i8, 105i8, 101i8, 100i8, 32i8, 102i8, 105i8, 101i8, 108i8, 100i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_datetime_replace")]
extern "C-unwind" fn datetime_datetime_replace(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut __v11: bool = unsafe { core::mem::zeroed() };
    let mut __v14: bool = unsafe { core::mem::zeroed() };
    let mut __v17: bool = unsafe { core::mem::zeroed() };
    let mut __v20: bool = unsafe { core::mem::zeroed() };
    let mut __v23: bool = unsafe { core::mem::zeroed() };
    let mut fold: i32 = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut microsecond: i32 = unsafe { core::mem::zeroed() };
    let mut second: i32 = unsafe { core::mem::zeroed() };
    let mut minute: i32 = unsafe { core::mem::zeroed() };
    let mut hour: i32 = unsafe { core::mem::zeroed() };
    let mut day: i32 = unsafe { core::mem::zeroed() };
    let mut month: i32 = unsafe { core::mem::zeroed() };
    let mut year: i32 = unsafe { core::mem::zeroed() };
    let mut __v33: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 9]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_35: *mut [*mut _object; 9] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 9];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v33 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v33 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v33).wrapping_sub(0);
            year = (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize) as i32)
                << 8i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize) as i32);
            month = *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize) as i32;
            day = *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize) as i32;
            hour = *((*(self_ as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(4 as isize) as i32;
            minute = *((*(self_ as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(5 as isize) as i32;
            second = *((*(self_ as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(6 as isize) as i32;
            microsecond = (*((*(self_ as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(7 as isize) as i32)
                << 16i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_114))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(8 as isize) as i32)
                    << 8i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_114))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(9 as isize) as i32);
            tzinfo = if (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo != 0 {
                (*(self_ as *mut __SlateRecord_datetime_h_114)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            };
            fold = (*(self_ as *mut __SlateRecord_datetime_h_114)).fold as i32;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__418),
                0i32,
                8i32,
                0i32,
                0i32,
                (*__slate_slot_35).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_6: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            year = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                            if year == 1i32.wrapping_neg() {
                                __v23 = !PyErr_Occurred().is_null();
                            } else {
                                __v23 = false;
                            }
                            if __v23 {
                                break '__join_0;
                            } else {
                                let __v21: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v21;
                                if __v21 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                            month = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                            if month == 1i32.wrapping_neg() {
                                __v20 = !PyErr_Occurred().is_null();
                            } else {
                                __v20 = false;
                            }
                            if __v20 {
                                break '__join_0;
                            } else {
                                let __v18: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v18;
                                if __v18 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(2 as isize) }.is_null() {
                            day = PyLong_AsInt(*args.wrapping_offset(2 as isize));
                            if day == 1i32.wrapping_neg() {
                                __v17 = !PyErr_Occurred().is_null();
                            } else {
                                __v17 = false;
                            }
                            if __v17 {
                                break '__join_0;
                            } else {
                                let __v15: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v15;
                                if __v15 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(3 as isize) }.is_null() {
                            hour = PyLong_AsInt(*args.wrapping_offset(3 as isize));
                            if hour == 1i32.wrapping_neg() {
                                __v14 = !PyErr_Occurred().is_null();
                            } else {
                                __v14 = false;
                            }
                            if __v14 {
                                break '__join_0;
                            } else {
                                let __v12: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v12;
                                if __v12 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(4 as isize) }.is_null() {
                            minute = PyLong_AsInt(*args.wrapping_offset(4 as isize));
                            if minute == 1i32.wrapping_neg() {
                                __v11 = !PyErr_Occurred().is_null();
                            } else {
                                __v11 = false;
                            }
                            if __v11 {
                                break '__join_0;
                            } else {
                                let __v9: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v9;
                                if __v9 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(5 as isize) }.is_null() {
                            second = PyLong_AsInt(*args.wrapping_offset(5 as isize));
                            if second == 1i32.wrapping_neg() {
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
                        if !unsafe { *args.wrapping_offset(6 as isize) }.is_null() {
                            microsecond = PyLong_AsInt(*args.wrapping_offset(6 as isize));
                            if microsecond == 1i32.wrapping_neg() {
                                __v5 = !PyErr_Occurred().is_null();
                            } else {
                                __v5 = false;
                            }
                            if __v5 {
                                break '__join_0;
                            } else {
                                let __v3: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v3;
                                if __v3 == 0 {
                                    break '__join_6;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(7 as isize) }.is_null() {
                            tzinfo = *args.wrapping_offset(7 as isize);
                            noptargs = noptargs.wrapping_sub(1);
                        }
                    }
                }
                if noptargs != 0 {
                    fold = PyLong_AsInt(*args.wrapping_offset(8 as isize));
                    if fold == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = datetime_datetime_replace_impl(
                    self_ as *mut __SlateRecord_datetime_h_114,
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                    microsecond,
                    tzinfo,
                    fold,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__405: __SlateRecord__datetimemodule_c_h_405 = {
    let mut __t0: __SlateRecord__datetimemodule_c_h_405 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 9i64,
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
                    ._py_year
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
                    ._py_month
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
                    ._py_day
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
                    ._py_hour
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
                    ._py_minute
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
                    ._py_second
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
                    ._py_microsecond
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
                    ._py_tzinfo
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
                    ._py_fold
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__417: __SlateAlign16<[*const i8; 10]> = __SlateAlign16([
    (b"year\0".as_ptr() as *mut i8) as *const i8,
    (b"month\0".as_ptr() as *mut i8) as *const i8,
    (b"day\0".as_ptr() as *mut i8) as *const i8,
    (b"hour\0".as_ptr() as *mut i8) as *const i8,
    (b"minute\0".as_ptr() as *mut i8) as *const i8,
    (b"second\0".as_ptr() as *mut i8) as *const i8,
    (b"microsecond\0".as_ptr() as *mut i8) as *const i8,
    (b"tzinfo\0".as_ptr() as *mut i8) as *const i8,
    (b"fold\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__418: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__417.0) as *const *const i8 };
    __t0.fname = (b"replace\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__405.ob_base.ob_base) };
    __t0
};

// We require that C int be at least 32 bits, and use int virtually
// everywhere.  In just a few cases we use a temp long, where a Python
// API returns a C long.  In such cases, we have to ensure that the
// final result fits in a C int (this can be an issue on 64-bit boxes).
// date(9999,12,31).toordinal()
// Nine decimal digits is easy to communicate, and leaves enough room
// so that two delta days can be added w/o fear of overflowing a signed
// 32-bit int, and with plenty of room left over to absorb any possible
// carries from adding seconds.
// Rename the long macros in datetime.h to more reasonable short names.
// Time accessors for time.
// [clinic input]
// module datetime
// class datetime.datetime "PyDateTime_DateTime *" "get_datetime_state()->datetime_type"
// class datetime.date "PyDateTime_Date *" "get_datetime_state()->date_type"
// class datetime.time "PyDateTime_Time *" "get_datetime_state()->time_type"
// class datetime.IsoCalendarDate "PyDateTime_IsoCalendarDate *" "get_datetime_state()->isocalendar_date_type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=c8f3d834a860d50a]
// Math utilities.
// k = i+j overflows iff k differs in sign from both inputs,
// iff k^i has sign bit set and k^j has sign bit set,
// iff (k^i)&(k^j) has sign bit set.
/// Compute Python divmod(x, y), returning the quotient and storing the
/// remainder into *r.  The quotient is the floor of x/y, and that's
/// the real point of this.  C will probably truncate instead (C99
/// requires truncation; C89 left it implementation-defined).
/// Simplification:  we *require* that y > 0 here.  That's appropriate
/// for all the uses made of it.  This simplifies the code and makes
/// the overflow case impossible (divmod(LONG_MIN, -1) is the only
/// overflow case).
fn divmod(x: i32, y: i32, r: *mut i32) -> i32 {
    let mut quo: i32 = 0;
    quo = x / y;
    unsafe {
        *r = x.wrapping_sub(quo.wrapping_mul(y));
    }
    if (unsafe { *r }) < 0 {
        quo = quo.wrapping_sub(1);
        let __v3: i32 = unsafe { *r };
        unsafe {
            *r = __v3.wrapping_add(y);
        }
    }
    quo
}

/// Nearest integer to m / n for integers m and n. Half-integer results
/// are rounded to even.
fn divide_nearest(m: *mut _object, n: *mut _object) -> *mut _object {
    let temp: *mut _object = unsafe { _PyLong_DivmodNear(m, n) };
    if temp.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = Py_NewRef!(PyTuple_GET_ITEM!(temp, 0i32));
    Py_DECREF!(temp);
    result
}

// General calendrical helper functions
/// For each month ordinal in 1..12, the number of days in that month,
/// and the number of days before that month in the same year.  These
/// are correct for non-leap years only.
/// unused; this vector uses 1-based indexing
static _days_in_month: __SlateAlign16<[i32; 13]> = __SlateAlign16([
    0i32, 31i32, 28i32, 31i32, 30i32, 31i32, 30i32, 31i32, 31i32, 30i32, 31i32, 30i32, 31i32,
]);

/// unused; this vector uses 1-based indexing
static _days_before_month: __SlateAlign16<[i32; 13]> = __SlateAlign16([
    0i32, 0i32, 31i32, 59i32, 90i32, 120i32, 151i32, 181i32, 212i32, 243i32, 273i32, 304i32, 334i32,
]);

/// year -> 1 if leap year, else 0.
fn is_leap(year: i32) -> i32 {
    // Cast year to unsigned.  The result is the same either way, but
    // C can generate faster code for unsigned mod than for signed
    // mod (especially for % 4 -- a good compiler should just grab
    // the last 2 bits when the LHS is unsigned).
    let ayear: u32 = year as u32;
    (ayear % 4 == 0 && (ayear % 100 != 0 || ayear % 400 == 0)) as i32
}

/// year, month -> number of days in that month in that year
fn days_in_month(year: i32, month: i32) -> i32 {
    let __v0: bool = month == 2 && is_leap(year) != 0;
    if __v0 {
        return 29;
    } else {
        return unsafe { _days_in_month.0[month as usize] };
    }
}

/// year, month -> number of days in year preceding first day of month
fn days_before_month(year: i32, month: i32) -> i32 {
    let mut days: i32 = unsafe { _days_before_month.0[month as usize] };
    let __v0: bool = month > 2 && is_leap(year) != 0;
    if __v0 {
        days = days.wrapping_add(1);
    }
    days
}

/// year -> number of days before January 1st of year.  Remember that we
/// start with year 1, so days_before_year(1) == 0.
fn days_before_year(year: i32) -> i32 {
    let y: i32 = year.wrapping_sub(1);
    // This is incorrect if year <= 0; we really want the floor
    // here.  But so long as MINYEAR is 1, the smallest year this
    // can see is 1.
    y.wrapping_mul(365)
        .wrapping_add(y / 4)
        .wrapping_sub(y / 100)
        .wrapping_add(y / 400)
}

// Number of days in 4, 100, and 400 year cycles.  That these have
// the correct values is asserted in the module init function.
// days_before_year(5); days in 4 years
// days_before_year(101); days in 100 years
// days_before_year(401); days in 400 years
/// ordinal -> year, month, day, considering 01-Jan-0001 as day 1.
fn ord_to_ymd(mut ordinal: i32, year: *mut i32, month: *mut i32, day: *mut i32) {
    let mut n: i32 = 0;
    let mut n1: i32 = 0;
    let mut n4: i32 = 0;
    let mut n100: i32 = 0;
    let mut n400: i32 = 0;
    let mut leapyear: i32 = 0;
    let mut preceding: i32 = 0;
    // ordinal is a 1-based index, starting at 1-Jan-1.  The pattern of
    // leap years repeats exactly every 400 years.  The basic strategy is
    // to find the closest 400-year boundary at or before ordinal, then
    // work with the offset from that boundary to ordinal.  Life is much
    // clearer if we subtract 1 from ordinal first -- then the values
    // of ordinal at 400-year boundaries are exactly those divisible
    // by DI400Y:
    //
    //    D  M   Y            n              n-1
    //    -- --- ----        ----------     ----------------
    //    31 Dec -400        -DI400Y       -DI400Y -1
    //     1 Jan -399         -DI400Y +1   -DI400Y      400-year boundary
    //    ...
    //    30 Dec  000        -1             -2
    //    31 Dec  000         0             -1
    //     1 Jan  001         1              0          400-year boundary
    //     2 Jan  001         2              1
    //     3 Jan  001         3              2
    //    ...
    //    31 Dec  400         DI400Y        DI400Y -1
    //     1 Jan  401         DI400Y +1     DI400Y      400-year boundary
    ordinal = ordinal.wrapping_sub(1);
    n400 = ordinal / 146097;
    n = ordinal % 146097;
    unsafe {
        *year = n400.wrapping_mul(400).wrapping_add(1);
    }
    // Now n is the (non-negative) offset, in days, from January 1 of
    // year, to the desired date.  Now compute how many 100-year cycles
    // precede n.
    // Note that it's possible for n100 to equal 4!  In that case 4 full
    // 100-year cycles precede the desired day, which implies the
    // desired day is December 31 at the end of a 400-year cycle.
    n100 = n / 36524;
    n %= 36524;
    // Now compute how many 4-year cycles precede it.
    n4 = n / 1461;
    n %= 1461;
    // And now how many single years.  Again n1 can be 4, and again
    // meaning that the desired day is December 31 at the end of the
    // 4-year cycle.
    n1 = n / 365;
    n %= 365;
    let __v3: i32 = unsafe { *year };
    unsafe {
        *year = __v3.wrapping_add(
            n100.wrapping_mul(100)
                .wrapping_add(n4.wrapping_mul(4))
                .wrapping_add(n1),
        );
    }
    if n1 == 4 || n100 == 4 {
        let __v6: i32 = unsafe { *year };
        unsafe {
            *year = __v6.wrapping_sub(1);
        }
        unsafe {
            *month = 12;
        }
        unsafe {
            *day = 31;
        }
        return;
    }
    // Now the year is correct, and n is the offset from January 1.  We
    // find the month via an estimate that's either exact or one too
    // large.
    leapyear = (n1 == 3 && (n4 != 24 || n100 == 3)) as i32;
    unsafe {
        *month = n.wrapping_add(50) >> 5i32;
    }
    preceding = unsafe { _days_before_month.0[*month as usize] }
        .wrapping_add(((unsafe { *month }) > 2 && leapyear != 0) as i32);
    if preceding > n {
        // estimate is too large
        let __v9: i32 = unsafe { *month };
        unsafe {
            *month = __v9.wrapping_sub(1);
        }
        let __v11: i32 = days_in_month(unsafe { *year }, unsafe { *month });
        preceding = preceding.wrapping_sub(__v11);
    }
    n = n.wrapping_sub(preceding);
    unsafe {
        *day = n.wrapping_add(1);
    }
}

/// year, month, day -> ordinal, considering 01-Jan-0001 as day 1.
fn ymd_to_ord(year: i32, month: i32, day: i32) -> i32 {
    days_before_year(year)
        .wrapping_add(days_before_month(year, month))
        .wrapping_add(day)
}

/// Day of week, where Monday==0, ..., Sunday==6.  1/1/1 was a Monday.
fn weekday(year: i32, month: i32, day: i32) -> i32 {
    ymd_to_ord(year, month, day).wrapping_add(6) % 7
}

/// Ordinal of the Monday starting week 1 of the ISO year.  Week 1 is the
/// first calendar week containing a Thursday.
fn iso_week1_monday(year: i32) -> i32 {
    let first_day: i32 = ymd_to_ord(year, 1i32, 1i32); // ord of 1/1
    // 0 if 1/1 is a Monday, 1 if a Tue, etc.
    let first_weekday: i32 = first_day.wrapping_add(6) % 7;
    // ordinal of closest Monday at or before 1/1
    let mut week1_monday: i32 = first_day.wrapping_sub(first_weekday);
    if first_weekday > 3 {
        week1_monday = week1_monday.wrapping_add(7);
    }
    // if 1/1 was Fri, Sat, Sun
    week1_monday
}

fn iso_to_ymd(
    iso_year: i32,
    iso_week: i32,
    iso_day: i32,
    year: *mut i32,
    month: *mut i32,
    day: *mut i32,
) -> i32 {
    // Year is bounded to 0 < year < 10000 because 9999-12-31 is (9999, 52, 5)
    if iso_year < 1 || iso_year > 9999 {
        return 4i32.wrapping_neg();
    }
    if iso_week <= 0 || iso_week >= 53 {
        let mut out_of_range: i32 = 1;
        if iso_week == 53 {
            // ISO years have 53 weeks in it on years starting with a Thursday
            //
            // and on leap years starting on Wednesday
            let first_weekday: i32 = weekday(iso_year, 1i32, 1i32);
            let __v0: bool = first_weekday == 3 || first_weekday == 2 && is_leap(iso_year) != 0;
            if __v0 {
                out_of_range = 0;
            }
        }
        if out_of_range != 0 {
            return 2i32.wrapping_neg();
        }
    }
    if iso_day <= 0 || iso_day >= 8 {
        return 3i32.wrapping_neg();
    }
    // Convert (Y, W, D) to (Y, M, D) in-place
    let day_1: i32 = iso_week1_monday(iso_year);
    let day_offset: i32 = iso_week
        .wrapping_sub(1)
        .wrapping_mul(7)
        .wrapping_add(iso_day)
        .wrapping_sub(1);
    ord_to_ymd(day_1.wrapping_add(day_offset), year, month, day);
    0
}

// Range checkers.
/// Check that -MAX_DELTA_DAYS <= days <= MAX_DELTA_DAYS.  If so, return 0.
/// If not, raise OverflowError and return -1.
fn check_delta_day_range(days: i32) -> i32 {
    if 999999999i32.wrapping_neg() <= days && days <= 999999999 {
        return 0;
    }
    unsafe {
        PyErr_Format(
            PyExc_OverflowError,
            c"days=%d; must have magnitude <= %d".as_ptr(),
            days,
            999999999i32,
        )
    };
    1i32.wrapping_neg()
}

/// Check that date arguments are in range.  Return 0 if they are.  If they
/// aren't, raise ValueError and return -1.
fn check_date_args(year: i32, month: i32, day: i32) -> i32 {
    if year < 1 || year > 9999 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"year must be in %d..%d, not %d".as_ptr(),
                1i32,
                9999i32,
                year,
            )
        };
        return 1i32.wrapping_neg();
    }
    if month < 1 || month > 12 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"month must be in 1..12, not %d".as_ptr(),
                month,
            )
        };
        return 1i32.wrapping_neg();
    }
    let dim: i32 = days_in_month(year, month);
    if day < 1 || day > dim {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"day %i must be in range 1..%d for month %i in year %i".as_ptr(),
                day,
                dim,
                month,
                year,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

/// Check that time arguments are in range.  Return 0 if they are.  If they
/// aren't, raise ValueError and return -1.
fn check_time_args(h: i32, m: i32, s: i32, us: i32, fold: i32) -> i32 {
    if h < 0 || h > 23 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"hour must be in 0..23, not %i".as_ptr(),
                h,
            )
        };
        return 1i32.wrapping_neg();
    }
    if m < 0 || m > 59 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"minute must be in 0..59, not %i".as_ptr(),
                m,
            )
        };
        return 1i32.wrapping_neg();
    }
    if s < 0 || s > 59 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"second must be in 0..59, not %i".as_ptr(),
                s,
            )
        };
        return 1i32.wrapping_neg();
    }
    if us < 0 || us > 999999 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"microsecond must be in 0..999999, not %i".as_ptr(),
                us,
            )
        };
        return 1i32.wrapping_neg();
    }
    if fold != 0 && fold != 1 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"fold must be either 0 or 1, not %i".as_ptr(),
                fold,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::sprintf;
use slate_support::strcmp;
use slate_support::strftime;
use slate_support::time;

// Normalization utilities.
/// One step of a mixed-radix conversion.  A "hi" unit is equivalent to
/// factor "lo" units.  factor must be > 0.  If *lo is less than 0, or
/// at least factor, enough of *lo is converted into "hi" units so that
/// 0 <= *lo < factor.  The input values must be such that int overflow
/// is impossible.
fn normalize_pair(hi: *mut i32, lo: *mut i32, factor: i32) {
    if (unsafe { *lo }) < 0 || (unsafe { *lo }) >= factor {
        let num_hi: i32 = divmod(unsafe { *lo }, factor, lo);
        let new_hi: i32 = unsafe { *hi }.wrapping_add(num_hi);
        unsafe {
            *hi = new_hi;
        }
    }
}

/// Fiddle days (d), seconds (s), and microseconds (us) so that
///      0 <= *s < 24*3600
///      0 <= *us < 1000000
/// The input values must be such that the internals don't overflow.
/// The way this routine is used, we don't get close.
fn normalize_d_s_us(d: *mut i32, s: *mut i32, us: *mut i32) {
    if (unsafe { *us }) < 0 || (unsafe { *us }) >= 1000000 {
        normalize_pair(s, us, 1000000i32);
        // |s| can't be bigger than about
        // |original s| + |original us|/1000000 now.
    }
    if (unsafe { *s }) < 0 || (unsafe { *s }) >= 86400i32 {
        normalize_pair(d, s, 86400i32);
        // |d| can't be bigger than about
        // |original d| +
        // (|original s| + |original us|/1000000) / (24*3600) now.
    }
}

/// Fiddle years (y), months (m), and days (d) so that
///      1 <= *m <= 12
///      1 <= *d <= days_in_month(*y, *m)
/// The input values must be such that the internals don't overflow.
/// The way this routine is used, we don't get close.
fn normalize_y_m_d(y: *mut i32, m: *mut i32, d: *mut i32) -> i32 {
    let mut ordinal: i32 = unsafe { core::mem::zeroed() };
    // # of days in month
    let mut dim: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            // In actual use, m is always the month component extracted from a
            // date/datetime object.  Therefore it is always in [1, 12] range.
            // Now only day can be out of bounds (year may also be out of bounds
            // for a datetime object, but we don't care about that here).
            // If day is out of bounds, what to do is arguable, but at least the
            // method here is principled and explainable.
            dim = days_in_month(*y, *m);
            if *d < 1 || *d > dim {
                // Move day-1 days from the first of the month.  First try to
                // get off cheap if we're only one day out of range
                // (adjustments for timezone alone can't be worse than that).
                if *d == 0 {
                    let __v4: i32 = *m;
                    *m = __v4.wrapping_sub(1);
                    if *m > 0 {
                        *d = days_in_month(*y, *m);
                    } else {
                        let __v1: i32 = *y;
                        *y = __v1.wrapping_sub(1);
                        *m = 12;
                        *d = 31;
                    }
                } else {
                    if *d == dim.wrapping_add(1) {
                        // move forward a day
                        let __v10: i32 = *m;
                        *m = __v10.wrapping_add(1);
                        *d = 1;
                        if *m > 12 {
                            *m = 1;
                            let __v7: i32 = *y;
                            *y = __v7.wrapping_add(1);
                        }
                    } else {
                        ordinal = ymd_to_ord(*y, *m, 1i32).wrapping_add(*d).wrapping_sub(1);
                        if ordinal < 1 || ordinal > 3652059 {
                            break '__join_0;
                        } else {
                            ord_to_ymd(ordinal, y, m, d);
                            return 0;
                        }
                    }
                }
            }
            if 1 <= *y && *y <= 9999 {
                return 0;
            }
        }
        PyErr_SetString(PyExc_OverflowError, c"date value out of range".as_ptr());
        return 1i32.wrapping_neg();
    }
}

/// Fiddle out-of-bounds months and days so that the result makes some kind
/// of sense.  The parameters are both inputs and outputs.  Returns < 0 on
/// failure, where failure means the adjusted year is out of bounds.
fn normalize_date(year: *mut i32, month: *mut i32, day: *mut i32) -> i32 {
    normalize_y_m_d(year, month, day)
}

/// Force all the datetime fields into range.  The parameters are both
/// inputs and outputs.  Returns < 0 on error.
fn normalize_datetime(
    year: *mut i32,
    month: *mut i32,
    day: *mut i32,
    hour: *mut i32,
    minute: *mut i32,
    second: *mut i32,
    microsecond: *mut i32,
) -> i32 {
    normalize_pair(second, microsecond, 1000000i32);
    normalize_pair(minute, second, 60i32);
    normalize_pair(hour, minute, 60i32);
    normalize_pair(day, hour, 24i32);
    normalize_date(year, month, day)
}

// Basic object allocation:  tp_alloc implementations.  These allocate
// Python objects of the right size and type, and do the Python object-
// initialization bit.  If there's not enough memory, they return NULL after
// setting MemoryError.  All data members remain uninitialized trash.
//
// We abuse the tp_alloc "nitems" argument to communicate whether a tzinfo
// member is needed.  This is ugly, imprecise, and possibly insecure.
// tp_basicsize for the time and datetime types is set to the size of the
// struct that has room for the tzinfo member, so subclasses in Python will
// allocate enough space for a tzinfo member whether or not one is actually
// needed.  That's the "ugly and imprecise" parts.  The "possibly insecure"
// part is that PyType_GenericAlloc() (which subclasses in Python end up
// using) just happens today to effectively ignore the nitems argument
// when tp_itemsize is 0, which it is for these type objects.  If that
// changes, perhaps the callers of tp_alloc slots in this file should
// be changed to force a 0 nitems argument unless the type being allocated
// is a base type implemented in this file (so that tp_alloc is time_alloc
// or datetime_alloc below, which know about the nitems abuse).
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_alloc")]
extern "C-unwind" fn time_alloc(r#type: *mut _typeobject, aware: i64) -> *mut _object {
    let size: u64 = if aware != 0 {
        core::mem::size_of::<__SlateRecord_datetime_h_86>() as u64
    } else {
        core::mem::size_of::<__SlateRecord_datetime_h_79>() as u64
    };
    let self_: *mut _object = (unsafe { PyObject_Malloc(size) }) as *mut _object;
    if self_.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    _PyObject_Init(self_, r#type);
    self_
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_alloc")]
extern "C-unwind" fn datetime_alloc(r#type: *mut _typeobject, aware: i64) -> *mut _object {
    let size: u64 = if aware != 0 {
        core::mem::size_of::<__SlateRecord_datetime_h_114>() as u64
    } else {
        core::mem::size_of::<__SlateRecord_datetime_h_107>() as u64
    };
    let self_: *mut _object = (unsafe { PyObject_Malloc(size) }) as *mut _object;
    if self_.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    _PyObject_Init(self_, r#type);
    self_
}

// Helpers for setting object fields.  These work on pointers to the
// appropriate base class.
/// For date and datetime.
fn set_date_fields(self_: *mut __SlateRecord_datetime_h_98, y: i32, m: i32, d: i32) {
    unsafe {
        (*self_).hashcode = 1i32.wrapping_neg() as i64;
    }
    SET_YEAR!(self_, y);
    SET_MONTH!(self_, m);
    SET_DAY!(self_, d);
}

// String parsing utilities and helper functions
fn is_digit(c: i8) -> u8 {
    (((c as i32).wrapping_sub(48) as u32) < 10) as u8
}

fn parse_digits(mut ptr: *const i8, var: *mut i32, num_digits: u64) -> *const i8 {
    for i in 0..num_digits {
        let mut tmp: u32 = 0;
        let __v0: *const i8 = ptr;
        ptr = __v0.wrapping_offset(1 as isize);
        tmp = ((unsafe { *__v0 }) as i32).wrapping_sub(48) as u32;
        if tmp > 9 {
            return core::ptr::null::<i8>();
        }
        let __v3: i32 = unsafe { *var };
        unsafe {
            *var = __v3.wrapping_mul(10);
        }
        let __v6: i32 = unsafe { *var };
        unsafe {
            *var = __v6.wrapping_add(tmp as i32);
        }
    }
    ptr
}

fn parse_isoformat_date(
    dtstr: *const i8,
    len: u64,
    year: *mut i32,
    month: *mut i32,
    day: *mut i32,
) -> i32 {
    // Parse the date components of the result of date.isoformat()
    //
    // Return codes:
    //      0:  Success
    //     -1:  Failed to parse date component
    //     -2:  Inconsistent date separator usage
    //     -3:  Failed to parse ISO week.
    //     -4:  Failed to parse ISO day.
    //     -5, -6, -7: Failure in iso_to_ymd
    let mut p: *const i8 = dtstr;
    p = parse_digits(p, year, 4u64);
    if p.is_null() {
        return 1i32.wrapping_neg();
    }
    let uses_separator: u8 = (((unsafe { *p }) as i32) == 45) as u8;
    if uses_separator != 0 {
        let __v0: *const i8 = p;
        p = __v0.wrapping_offset(1 as isize);
    }
    if ((unsafe { *p }) as i32) == 87 {
        // This is an isocalendar-style date string
        let __v2: *const i8 = p;
        p = __v2.wrapping_offset(1 as isize);
        let mut iso_week: i32 = 0;
        let mut iso_day: i32 = 0;
        p = parse_digits(p, core::ptr::addr_of_mut!(iso_week), 2u64);
        if p.is_null() {
            return 3i32.wrapping_neg();
        }
        if (((unsafe { p.offset_from(dtstr as *const i8) }) as i64) as u64) < len {
            let __v4: bool = if uses_separator != 0 {
                let __v5: *const i8 = p;
                p = __v5.wrapping_offset(1 as isize);
                ((unsafe { *__v5 }) as i32) != 45
            } else {
                false
            };
            if __v4 {
                return 2i32.wrapping_neg();
            }
            p = parse_digits(p, core::ptr::addr_of_mut!(iso_day), 1u64);
            if p.is_null() {
                return 4i32.wrapping_neg();
            }
        } else {
            iso_day = 1;
        }
        let rv: i32 = iso_to_ymd(unsafe { *year }, iso_week, iso_day, year, month, day);
        if rv != 0 {
            return 3i32.wrapping_neg().wrapping_add(rv);
        } else {
            return 0;
        }
    }
    p = parse_digits(p, month, 2u64);
    if p.is_null() {
        return 1i32.wrapping_neg();
    }
    let __v7: bool = if uses_separator != 0 {
        let __v8: *const i8 = p;
        p = __v8.wrapping_offset(1 as isize);
        ((unsafe { *__v8 }) as i32) != 45
    } else {
        false
    };
    if __v7 {
        return 2i32.wrapping_neg();
    }
    p = parse_digits(p, day, 2u64);
    if p.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

fn parse_hh_mm_ss_ff(
    mut p: *const i8,
    p_end: *const i8,
    hour: *mut i32,
    minute: *mut i32,
    second: *mut i32,
    microsecond: *mut i32,
) -> i32 {
    unsafe {
        *microsecond = 0;
    }
    unsafe {
        *second = 0;
    }
    unsafe {
        *minute = 0;
    }
    unsafe {
        *hour = 0;
    }
    let vals: __SlateAlign16<[*mut i32; 3]> = __SlateAlign16([hour, minute, second]);
    // This is initialized to satisfy an erroneous compiler warning.
    let mut has_separator: u8 = 1;
    // Parse [HH[:?MM[:?SS]]]
    for i in 0u64..3u64 {
        p = parse_digits(p, vals.0[i as usize], 2u64);
        if p.is_null() {
            return 3i32.wrapping_neg();
        }
        let mut c: i8 = 0;
        let __v0: *const i8 = p;
        p = __v0.wrapping_offset(1 as isize);
        c = unsafe { *__v0 };
        if i == 0 {
            has_separator = ((c as i32) == 58) as u8;
        }
        if (c as i32) == 46 || (c as i32) == 44 {
            if i < 2 {
                return 3i32.wrapping_neg(); // Decimal mark on hour or minute
            }
            if p >= p_end {
                return 3i32.wrapping_neg(); // Decimal mark not followed by any digit
            }
            break;
        } else {
            if p >= p_end {
                return ((c as i32) != 0) as i32;
            } else {
                if has_separator != 0 && (c as i32) == 58 {
                    if i == 2 {
                        return 4i32.wrapping_neg(); // Malformed microsecond separator
                    }
                    continue;
                } else {
                    if has_separator == 0 {
                        let __v3: *const i8 = p;
                        p = __v3.wrapping_offset(-(1 as isize));
                    } else {
                        return 4i32.wrapping_neg(); // Malformed time separator
                    }
                }
            }
        }
    }
    // Parse fractional components
    let len_remains: u64 = ((unsafe { p_end.offset_from(p as *const i8) }) as i64) as u64;
    let mut to_parse: u64 = len_remains;
    if len_remains >= 6 {
        to_parse = 6;
    }
    p = parse_digits(p, microsecond, to_parse);
    if p.is_null() {
        return 3i32.wrapping_neg();
    }
    if to_parse < 6 {
        let __v6: i32 = unsafe { *microsecond };
        unsafe {
            *microsecond = __v6.wrapping_mul(correction.0[to_parse.wrapping_sub(1) as usize]);
        }
    }
    while is_digit(unsafe { *p }) != 0 {
        let __v8: *const i8 = p;
        p = __v8.wrapping_offset(1 as isize); // skip truncated digits
    }
    // Return 1 if it's not the end of the string
    (((unsafe { *p }) as i32) != 0) as i32
}

static mut correction: __SlateAlign16<[i32; 5]> =
    __SlateAlign16([100000i32, 10000i32, 1000i32, 100i32, 10i32]);

fn parse_isoformat_time(
    dtstr: *const i8,
    dtlen: u64,
    hour: *mut i32,
    minute: *mut i32,
    second: *mut i32,
    microsecond: *mut i32,
    tzoffset: &mut i32,
    tzmicrosecond: *mut i32,
) -> i32 {
    // Parse the time portion of a datetime.isoformat() string
    //
    //
    //
    // Return codes:
    //
    //      0:  Success (no tzoffset)
    //
    //      1:  Success (with tzoffset)
    //
    //     -3:  Failed to parse time component
    //
    //     -4:  Failed to parse time separator
    //
    //     -5:  Malformed timezone string
    let p: *const i8 = dtstr;
    let p_end: *const i8 = dtstr.wrapping_offset(dtlen as isize);
    let mut tzinfo_pos: *const i8 = p;
    while !(((unsafe { *tzinfo_pos }) as i32) == 90
        || ((unsafe { *tzinfo_pos }) as i32) == 43
        || ((unsafe { *tzinfo_pos }) as i32) == 45)
    {
        let __v1: *const i8 = tzinfo_pos;
        let __v2: *const i8 = __v1.wrapping_offset(1 as isize);
        tzinfo_pos = __v2;
        if !(__v2 < p_end) {
            break;
        }
    }
    let mut rv: i32 = parse_hh_mm_ss_ff(dtstr, tzinfo_pos, hour, minute, second, microsecond);
    if rv < 0 {
        return rv;
    } else {
        if tzinfo_pos == p_end {
            // We know that there's no time zone, so if there's stuff at the
            //
            // end of the string it's an error.
            if rv == 1 {
                return 5i32.wrapping_neg();
            } else {
                return 0;
            }
        }
    }
    // Special case UTC / Zulu time.
    if ((unsafe { *tzinfo_pos }) as i32) == 90 {
        unsafe {
            *(tzoffset as *mut i32) = 0;
        }
        unsafe {
            *tzmicrosecond = 0;
        }
        if ((unsafe { *tzinfo_pos.wrapping_offset(1 as isize) }) as i32) != 0 {
            return 5i32.wrapping_neg();
        } else {
            return 1;
        }
    }
    let tzsign: i32 = if ((unsafe { *tzinfo_pos }) as i32) == 45 {
        1i32.wrapping_neg()
    } else {
        1
    };
    let __v3: *const i8 = tzinfo_pos;
    tzinfo_pos = __v3.wrapping_offset(1 as isize);
    let mut tzhour: i32 = 0;
    let mut tzminute: i32 = 0;
    let mut tzsecond: i32 = 0;
    rv = parse_hh_mm_ss_ff(
        tzinfo_pos,
        p_end,
        core::ptr::addr_of_mut!(tzhour),
        core::ptr::addr_of_mut!(tzminute),
        core::ptr::addr_of_mut!(tzsecond),
        tzmicrosecond,
    );
    unsafe {
        *(tzoffset as *mut i32) = tzsign.wrapping_mul(
            tzhour
                .wrapping_mul(3600)
                .wrapping_add(tzminute.wrapping_mul(60))
                .wrapping_add(tzsecond),
        );
    }
    let __v6: i32 = unsafe { *tzmicrosecond };
    unsafe {
        *tzmicrosecond = __v6.wrapping_mul(tzsign);
    }
    if rv != 0 { 5i32.wrapping_neg() } else { 1 }
}

// Create various objects, mostly without range checking.
/// Create a date instance with no range checking.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_date_ex")]
extern "C-unwind" fn new_date_ex(
    year: i32,
    month: i32,
    day: i32,
    r#type: *mut _typeobject,
) -> *mut _object {
    if check_date_args(year, month, day) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_datetime_h_98 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_datetime_h_98;
    if !self_.is_null() {
        set_date_fields(self_, year, month, day);
    }
    self_ as *mut _object
}

/// Create date instance with no range checking, or call subclass constructor
fn new_date_subclass_ex(year: i32, month: i32, day: i32, cls: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    // We have "fast path" constructors for two subclasses: date and datetime
    if (cls as *mut _typeobject) == DATE_TYPE!() {
        result = new_date_ex(year, month, day, cls as *mut _typeobject);
    } else {
        if (cls as *mut _typeobject) == DATETIME_TYPE!() {
            result = new_datetime_ex(
                year,
                month,
                day,
                0i32,
                0i32,
                0i32,
                0i32,
                unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
                cls as *mut _typeobject,
            );
        } else {
            result = unsafe { PyObject_CallFunction(cls, c"iii".as_ptr(), year, month, day) };
        }
    }
    result
}

/// Create a datetime instance with no range checking.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_datetime_ex2")]
extern "C-unwind" fn new_datetime_ex2(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    fold: i32,
    r#type: *mut _typeobject,
) -> *mut _object {
    let aware: i8 = (tzinfo != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }) as i8;
    if check_date_args(year, month, day) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if check_time_args(hour, minute, second, usecond, fold) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if check_tzinfo_subclass(tzinfo) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_datetime_h_114 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, aware as i64) })
            as *mut __SlateRecord_datetime_h_114;
    if !self_.is_null() {
        unsafe {
            (*self_).hastzinfo = aware;
        }
        set_date_fields(self_ as *mut __SlateRecord_datetime_h_98, year, month, day);
        DATE_SET_HOUR!(self_, hour);
        DATE_SET_MINUTE!(self_, minute);
        DATE_SET_SECOND!(self_, second);
        DATE_SET_MICROSECOND!(self_, usecond);
        if aware != 0 {
            unsafe {
                (*self_).tzinfo = Py_NewRef!(tzinfo);
            }
        }
        DATE_SET_FOLD!(self_, fold);
    }
    self_ as *mut _object
}

/// Forward declaration
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_datetime_ex")]
extern "C-unwind" fn new_datetime_ex(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    r#type: *mut _typeobject,
) -> *mut _object {
    new_datetime_ex2(
        year, month, day, hour, minute, second, usecond, tzinfo, 0i32, r#type,
    )
}

unsafe extern "C-unwind" fn call_subclass_fold(
    cls: *mut _object,
    fold: i32,
    format: *const i8,
    mut __va_args: ...
) -> *mut _object {
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut args: *mut _object = unsafe { core::mem::zeroed() };
    let mut va: core::mem::MaybeUninit<core::ffi::VaList<'static>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut core::ffi::VaList<'static> =
        core::ptr::addr_of_mut!(va) as *mut core::ffi::VaList<'static>;
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut kwargs: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        kwargs = core::ptr::null_mut::<_object>();
        res = core::ptr::null_mut::<_object>();
        *__slate_slot_3 = core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(
            __va_args.clone(),
        );
        args = Py_VaBuildValue(format, (*__slate_slot_3).clone());
        if args.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if fold != 0 {
                    kwargs = PyDict_New();
                    if kwargs.is_null() {
                        break '__join_0;
                    } else {
                        obj = PyLong_FromLong(fold as i64);
                        if obj.is_null() {
                            break '__join_0;
                        } else {
                            let err: i32 = PyDict_SetItemString(kwargs, c"fold".as_ptr(), obj);
                            Py_DECREF!(obj);
                            if err < 0 {
                                break '__join_0;
                            }
                        }
                    }
                }
                res = PyObject_Call(cls, args, kwargs);
            }
            Py_DECREF!(args);
            Py_XDECREF!(kwargs);
            return res;
        }
    }
}

fn new_datetime_subclass_fold_ex(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    fold: i32,
    cls: *mut _object,
) -> *mut _object {
    let mut dt: *mut _object = unsafe { core::mem::zeroed() };
    if (cls as *mut _typeobject) == DATETIME_TYPE!() {
        // Use the fast path constructor
        dt = new_datetime!(
            year, month, day, hour, minute, second, usecond, tzinfo, fold
        );
    } else {
        // Subclass
        dt = unsafe {
            call_subclass_fold(
                cls,
                fold,
                c"iiiiiiiO".as_ptr(),
                year,
                month,
                day,
                hour,
                minute,
                second,
                usecond,
                tzinfo,
            )
        };
    }
    dt
}

fn new_datetime_subclass_ex(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    cls: *mut _object,
) -> *mut _object {
    new_datetime_subclass_fold_ex(
        year, month, day, hour, minute, second, usecond, tzinfo, 0i32, cls,
    )
}

/// Create a time instance with no range checking.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_time_ex2")]
extern "C-unwind" fn new_time_ex2(
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    fold: i32,
    r#type: *mut _typeobject,
) -> *mut _object {
    let aware: i8 = (tzinfo != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }) as i8;
    if check_time_args(hour, minute, second, usecond, fold) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if check_tzinfo_subclass(tzinfo) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_datetime_h_86 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, aware as i64) })
            as *mut __SlateRecord_datetime_h_86;
    if !self_.is_null() {
        unsafe {
            (*self_).hastzinfo = aware;
        }
        unsafe {
            (*self_).hashcode = 1i32.wrapping_neg() as i64;
        }
        TIME_SET_HOUR!(self_, hour);
        TIME_SET_MINUTE!(self_, minute);
        TIME_SET_SECOND!(self_, second);
        TIME_SET_MICROSECOND!(self_, usecond);
        if aware != 0 {
            unsafe {
                (*self_).tzinfo = Py_NewRef!(tzinfo);
            }
        }
        TIME_SET_FOLD!(self_, fold);
    }
    self_ as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_time_ex")]
extern "C-unwind" fn new_time_ex(
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    r#type: *mut _typeobject,
) -> *mut _object {
    new_time_ex2(hour, minute, second, usecond, tzinfo, 0i32, r#type)
}

fn new_time_subclass_fold_ex(
    hour: i32,
    minute: i32,
    second: i32,
    usecond: i32,
    tzinfo: *mut _object,
    fold: i32,
    cls: *mut _object,
) -> *mut _object {
    let mut t: *mut _object = unsafe { core::mem::zeroed() };
    if (cls as *mut _typeobject) == TIME_TYPE!() {
        // Use the fast path constructor
        t = new_time!(hour, minute, second, usecond, tzinfo, fold);
    } else {
        // Subclass
        t = unsafe {
            call_subclass_fold(
                cls,
                fold,
                c"iiiiO".as_ptr(),
                hour,
                minute,
                second,
                usecond,
                tzinfo,
            )
        };
    }
    t
}

/// Create a timedelta instance.  Normalize the members iff normalize is
/// true.  Passing false is a speed optimization, if you know for sure
/// that seconds and microseconds are already in their proper ranges.  In any
/// case, raises OverflowError and returns NULL if the normalized days is out
/// of range.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_delta_ex")]
extern "C-unwind" fn new_delta_ex(
    mut days: i32,
    mut seconds: i32,
    mut microseconds: i32,
    normalize: i32,
    r#type: *mut _typeobject,
) -> *mut _object {
    let mut self_: *mut __SlateRecord_datetime_h_41 = unsafe { core::mem::zeroed() };
    if normalize != 0 {
        normalize_d_s_us(
            core::ptr::addr_of_mut!(days),
            core::ptr::addr_of_mut!(seconds),
            core::ptr::addr_of_mut!(microseconds),
        );
    }
    if check_delta_day_range(days) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    self_ = look_up_delta(days, seconds, microseconds, r#type);
    if !self_.is_null() {
        return self_ as *mut _object;
    }
    self_ = (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
        as *mut __SlateRecord_datetime_h_41;
    if !self_.is_null() {
        unsafe {
            (*self_).hashcode = 1i32.wrapping_neg() as i64;
        }
        SET_TD_DAYS!(self_, days);
        SET_TD_SECONDS!(self_, seconds);
        SET_TD_MICROSECONDS!(self_, microseconds);
    }
    self_ as *mut _object
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_1414 {
    ob_base: _object,
    offset: *mut _object,
    name: *mut _object,
}

/// Create new timezone instance checking offset range.  This
/// function does not check the name argument.  Caller must assure
/// that offset is a timedelta instance and name is either NULL
/// or a unicode object.
fn create_timezone(offset: *mut _object, name: *mut _object) -> *mut _object {
    let mut self_: *mut __SlateRecord__datetimemodule_c_1414 = unsafe { core::mem::zeroed() };
    let r#type: *mut _typeobject = TIMEZONE_TYPE!();
    self_ = look_up_timezone(offset, name);
    if !self_.is_null() {
        return self_ as *mut _object;
    }
    self_ = (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
        as *mut __SlateRecord__datetimemodule_c_1414;
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).offset = Py_NewRef!(offset);
    }
    unsafe {
        (*self_).name = Py_XNewRef!(name);
    }
    self_ as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.new_timezone")]
extern "C-unwind" fn new_timezone(offset: *mut _object, name: *mut _object) -> *mut _object {
    let __v0: bool = name.is_null() && delta_bool(offset) == 0;
    if __v0 {
        return Py_NewRef!(CONST_UTC!());
    }
    if GET_TD_DAYS!(offset) == 1i32.wrapping_neg()
        && GET_TD_SECONDS!(offset) == 0
        && GET_TD_MICROSECONDS!(offset) < 1
        || GET_TD_DAYS!(offset) < 1i32.wrapping_neg()
        || GET_TD_DAYS!(offset) >= 1
    {
        unsafe {
            PyErr_Format(PyExc_ValueError, c"offset must be a timedelta strictly between -timedelta(hours=24) and timedelta(hours=24), not %R".as_ptr(), offset)
        };
        return core::ptr::null_mut::<_object>();
    }
    create_timezone(offset, name)
}

// tzinfo helpers.
/// Ensure that p is None or of a tzinfo subclass.  Return 0 if OK; if not
/// raise TypeError and return -1.
fn check_tzinfo_subclass(p: *mut _object) -> i32 {
    let __v0: bool =
        p == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } || PyTZInfo_Check!(p) != 0;
    if __v0 {
        return 0;
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"tzinfo argument must be None or of a tzinfo subclass, not type '%s'".as_ptr(),
            (*Py_TYPE!(p)).tp_name,
        )
    };
    1i32.wrapping_neg()
}

/// If self has a tzinfo member, return a BORROWED reference to it.  Else
/// return NULL, which is NOT AN ERROR.  There are no error returns here,
/// and the caller must not decref the result.
fn get_tzinfo_member(self_: *mut _object) -> *mut _object {
    let mut tzinfo: *mut _object = core::ptr::null_mut::<_object>();
    if PyDateTime_Check!(self_) != 0
        && (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0
    {
        tzinfo = unsafe { (*(self_ as *mut __SlateRecord_datetime_h_114)).tzinfo };
    } else {
        if PyTime_Check!(self_) != 0
            && (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0
        {
            tzinfo = unsafe { (*(self_ as *mut __SlateRecord_datetime_h_86)).tzinfo };
        }
    }
    tzinfo
}

/// Call getattr(tzinfo, name)(tzinfoarg), and check the result.  tzinfo must
/// be an instance of the tzinfo class.  If the method returns None, this
/// returns None.  If the method doesn't return None or timedelta, TypeError is
/// raised and this returns NULL.  If it returns a timedelta and the value is
/// out of range or isn't a whole number of minutes, ValueError is raised and
/// this returns NULL.  Else result is returned.
fn call_tzinfo_method(
    tzinfo: *mut _object,
    name: *const i8,
    tzinfoarg: *mut _object,
) -> *mut _object {
    if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    let offset: *mut _object =
        unsafe { PyObject_CallMethod(tzinfo, name, c"O".as_ptr(), tzinfoarg) };
    if offset == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } || offset.is_null() {
        return offset;
    }
    if PyDelta_Check!(offset) != 0 {
        if GET_TD_DAYS!(offset) == 1i32.wrapping_neg()
            && GET_TD_SECONDS!(offset) == 0
            && GET_TD_MICROSECONDS!(offset) < 1
            || GET_TD_DAYS!(offset) < 1i32.wrapping_neg()
            || GET_TD_DAYS!(offset) >= 1
        {
            unsafe {
                PyErr_Format(PyExc_ValueError, c"offset must be a timedelta strictly between -timedelta(hours=24) and timedelta(hours=24), not %R".as_ptr(), offset)
            };
            Py_DECREF!(offset);
            return core::ptr::null_mut::<_object>();
        }
    } else {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"tzinfo.%s() must return None or timedelta, not '%.200s'".as_ptr(),
                name,
                (*Py_TYPE!(offset)).tp_name,
            )
        };
        Py_DECREF!(offset);
        return core::ptr::null_mut::<_object>();
    }
    offset
}

/// Call tzinfo.utcoffset(tzinfoarg), and extract an integer from the
/// result.  tzinfo must be an instance of the tzinfo class.  If utcoffset()
/// returns None, call_utcoffset returns 0 and sets *none to 1.  If uctoffset()
/// doesn't return None or timedelta, TypeError is raised and this returns -1.
/// If utcoffset() returns an out of range timedelta,
/// ValueError is raised and this returns -1.  Else *none is
/// set to 0 and the offset is returned (as timedelta, positive east of UTC).
fn call_utcoffset(tzinfo: *mut _object, tzinfoarg: *mut _object) -> *mut _object {
    call_tzinfo_method(tzinfo, c"utcoffset".as_ptr(), tzinfoarg)
}

/// Call tzinfo.dst(tzinfoarg), and extract an integer from the
/// result.  tzinfo must be an instance of the tzinfo class.  If dst()
/// returns None, call_dst returns 0 and sets *none to 1.  If dst()
/// doesn't return None or timedelta, TypeError is raised and this
/// returns -1.  If dst() returns an invalid timedelta for a UTC offset,
/// ValueError is raised and this returns -1.  Else *none is set to 0 and
/// the offset is returned (as timedelta, positive east of UTC).
fn call_dst(tzinfo: *mut _object, tzinfoarg: *mut _object) -> *mut _object {
    call_tzinfo_method(tzinfo, c"dst".as_ptr(), tzinfoarg)
}

/// Call tzinfo.tzname(tzinfoarg), and return the result.  tzinfo must be
/// an instance of the tzinfo class or None.  If tzinfo isn't None, and
/// tzname() doesn't return None or a string, TypeError is raised and this
/// returns NULL.  If the result is a string, we ensure it is a Unicode
/// string.
fn call_tzname(tzinfo: *mut _object, tzinfoarg: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    result = PyObject_CallMethodOneArg(
        tzinfo,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_tzname
                    ._ascii
                    .ob_base
            )
        },
        tzinfoarg,
    );
    if result.is_null() || result == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return result;
    }
    if PyUnicode_Check!(result) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"tzinfo.tzname() must return None or a string, not '%s'".as_ptr(),
                (*Py_TYPE!(result)).tp_name,
            )
        };
        Py_SETREF__3!(result);
    }
    result
}

/// repr is like "someclass(arg1, arg2)".  If tzinfo isn't None,
/// stuff
///     ", tzinfo=" + repr(tzinfo)
/// before the closing ")".
fn append_keyword_tzinfo(mut repr: *mut _object, tzinfo: *mut _object) -> *mut _object {
    if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return repr;
    }
    // Get rid of the trailing ')'.
    let temp: *mut _object =
        unsafe { PyUnicode_Substring(repr, 0i64, PyUnicode_GET_LENGTH!(repr).wrapping_sub(1)) };
    Py_DECREF!(repr);
    if temp.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    repr = unsafe { PyUnicode_FromFormat(c"%U, tzinfo=%R)".as_ptr(), temp, tzinfo) };
    Py_DECREF!(temp);
    repr
}

/// repr is like "someclass(arg1, arg2)".  If fold isn't 0,
/// stuff
///     ", fold=" + repr(tzinfo)
/// before the closing ")".
fn append_keyword_fold(mut repr: *mut _object, fold: i32) -> *mut _object {
    if fold == 0 {
        return repr;
    }
    // Get rid of the trailing ')'.
    let temp: *mut _object =
        unsafe { PyUnicode_Substring(repr, 0i64, PyUnicode_GET_LENGTH!(repr).wrapping_sub(1)) };
    Py_DECREF!(repr);
    if temp.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    repr = unsafe { PyUnicode_FromFormat(c"%U, fold=%d)".as_ptr(), temp, fold) };
    Py_DECREF!(temp);
    repr
}

fn tzinfo_from_isoformat_results(rv: i32, tzoffset: i32, tz_useconds: i32) -> *mut _object {
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    if rv == 1 {
        // Create a timezone from the offset (a zero offset returns UTC)
        if tzoffset == 0 && tz_useconds == 0 {
            return Py_NewRef!(CONST_UTC!());
        }
        let delta: *mut _object = new_delta!(0i32, tzoffset, tz_useconds, 1i32);
        if delta.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        tzinfo = new_timezone(delta, core::ptr::null_mut::<_object>());
        Py_DECREF!(delta);
    } else {
        tzinfo = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
    }
    tzinfo
}

// String format helpers.
fn format_ctime(date: *mut _object, hours: i32, minutes: i32, seconds: i32) -> *mut _object {
    let wday: i32 = weekday(
        ((unsafe {
            *((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(date as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
    );
    unsafe {
        PyUnicode_FromFormat(
            c"%s %s %2d %02d:%02d:%02d %04d".as_ptr(),
            DayNames.0[wday as usize],
            MonthNames.0[(*((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize) as i32)
                .wrapping_sub(1) as usize],
            *((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize) as i32,
            hours,
            minutes,
            seconds,
            (*((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize) as i32)
                << 8i32
                | (*((*(date as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize) as i32),
        )
    }
}

static DayNames: __SlateAlign16<[*const i8; 7]> = __SlateAlign16([
    (b"Mon\0".as_ptr() as *mut i8) as *const i8,
    (b"Tue\0".as_ptr() as *mut i8) as *const i8,
    (b"Wed\0".as_ptr() as *mut i8) as *const i8,
    (b"Thu\0".as_ptr() as *mut i8) as *const i8,
    (b"Fri\0".as_ptr() as *mut i8) as *const i8,
    (b"Sat\0".as_ptr() as *mut i8) as *const i8,
    (b"Sun\0".as_ptr() as *mut i8) as *const i8,
]);

static MonthNames: __SlateAlign16<[*const i8; 12]> = __SlateAlign16([
    (b"Jan\0".as_ptr() as *mut i8) as *const i8,
    (b"Feb\0".as_ptr() as *mut i8) as *const i8,
    (b"Mar\0".as_ptr() as *mut i8) as *const i8,
    (b"Apr\0".as_ptr() as *mut i8) as *const i8,
    (b"May\0".as_ptr() as *mut i8) as *const i8,
    (b"Jun\0".as_ptr() as *mut i8) as *const i8,
    (b"Jul\0".as_ptr() as *mut i8) as *const i8,
    (b"Aug\0".as_ptr() as *mut i8) as *const i8,
    (b"Sep\0".as_ptr() as *mut i8) as *const i8,
    (b"Oct\0".as_ptr() as *mut i8) as *const i8,
    (b"Nov\0".as_ptr() as *mut i8) as *const i8,
    (b"Dec\0".as_ptr() as *mut i8) as *const i8,
]);

/// Add formatted UTC offset string to buf.  buf has no more than
/// buflen bytes remaining.  The UTC offset is gotten by calling
/// tzinfo.uctoffset(tzinfoarg).  If that returns None, \0 is stored into
/// *buf, and that's all.  Else the returned value is checked for sanity (an
/// integer in range), and if that's OK it's converted to an hours & minutes
/// string of the form
///   sign HH sep MM [sep SS [. UUUUUU]]
/// Returns 0 if everything is OK.  If the return value from utcoffset() is
/// bogus, an appropriate exception is set and -1 is returned.
fn format_utcoffset(
    buf: *mut i8,
    buflen: u64,
    sep: *const i8,
    tzinfo: *mut _object,
    tzinfoarg: *mut _object,
) -> i32 {
    let mut offset: *mut _object = unsafe { core::mem::zeroed() };
    let mut hours: i32 = 0;
    let mut minutes: i32 = 0;
    let mut seconds: i32 = 0;
    let mut microseconds: i32 = 0;
    let mut sign: i8 = 0;
    offset = call_utcoffset(tzinfo, tzinfoarg);
    if offset.is_null() {
        return 1i32.wrapping_neg();
    }
    if offset == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        Py_DECREF!(offset);
        unsafe {
            *buf = 0;
        }
        return 0;
    }
    // Offset is normalized, so it is negative if days < 0
    if GET_TD_DAYS!(offset) < 0 {
        sign = 45;
        Py_SETREF!(offset, delta_negative(offset));
        if offset.is_null() {
            return 1i32.wrapping_neg();
        }
    } else {
        sign = 43;
    }
    // Offset is not negative here.
    microseconds = GET_TD_MICROSECONDS!(offset);
    seconds = GET_TD_SECONDS!(offset);
    Py_DECREF!(offset);
    minutes = divmod(seconds, 60i32, core::ptr::addr_of_mut!(seconds));
    hours = divmod(minutes, 60i32, core::ptr::addr_of_mut!(minutes));
    if microseconds != 0 {
        unsafe {
            PyOS_snprintf(
                buf,
                buflen,
                c"%c%02d%s%02d%s%02d.%06d".as_ptr(),
                sign as i32,
                hours,
                sep,
                minutes,
                sep,
                seconds,
                microseconds,
            )
        };
        return 0;
    }
    if seconds != 0 {
        unsafe {
            PyOS_snprintf(
                buf,
                buflen,
                c"%c%02d%s%02d%s%02d".as_ptr(),
                sign as i32,
                hours,
                sep,
                minutes,
                sep,
                seconds,
            )
        };
        return 0;
    }
    unsafe {
        PyOS_snprintf(
            buf,
            buflen,
            c"%c%02d%s%02d".as_ptr(),
            sign as i32,
            hours,
            sep,
            minutes,
        )
    };
    0
}

/// Check whether year with century should be normalized for strftime.
fn normalize_century() -> i32 {
    if (unsafe { cache }) < 0 {
        let mut year: [i8; 5] = [0i8; 5];
        let mut date: slate_support::tm = {
            let mut __t0: slate_support::tm = unsafe { core::mem::zeroed() };
            __t0.tm_mday = 1i32;
            __t0.tm_mon = 0i32;
            __t0.tm_year = 1801i32.wrapping_neg();
            __t0
        };
        unsafe {
            cache = ((strftime(
                (year.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                (core::mem::size_of::<[i8; 5]>() as u64) as usize,
                c"%Y".as_ptr() as *const core::ffi::c_char,
                (core::ptr::addr_of_mut!(date) as *const slate_support::tm)
                    as *const slate_support::tm,
            ) as u64)
                != 0
                && (strcmp(
                    (year.as_mut_ptr() as *const i8) as *const core::ffi::c_char,
                    c"0099".as_ptr() as *const core::ffi::c_char,
                ) as i32)
                    != 0) as i32;
        }
    }
    unsafe { cache }
}

static mut cache: i32 = 1i32.wrapping_neg();

fn make_somezreplacement(
    object: *mut _object,
    sep: *mut i8,
    tzinfoarg: *mut _object,
) -> *mut _object {
    let mut buf: __SlateAlign16<[i8; 100]> = __SlateAlign16([0i8; 100]);
    let tzinfo: *mut _object = get_tzinfo_member(object);
    if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } || tzinfo.is_null() {
        return unsafe { PyUnicode_FromStringAndSize(core::ptr::null::<i8>(), 0i64) };
    }
    if format_utcoffset(
        buf.0.as_mut_ptr() as *mut i8,
        core::mem::size_of::<[i8; 100]>() as u64,
        sep as *const i8,
        tzinfo,
        tzinfoarg,
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyUnicode_FromString(buf.0.as_mut_ptr() as *const i8) }
}

fn make_Zreplacement(object: *mut _object, tzinfoarg: *mut _object) -> *mut _object {
    let mut Zreplacement: *mut _object = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        tzinfo = get_tzinfo_member(object);
        Zreplacement = Py_GetConstant(7u32);
        if Zreplacement.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            if tzinfo == core::ptr::addr_of_mut!(_Py_NoneStruct) || tzinfo.is_null() {
                return Zreplacement;
            } else {
                temp = call_tzname(tzinfo, tzinfoarg);
                if !temp.is_null() {
                    if temp == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                        Py_DECREF!(temp);
                        return Zreplacement;
                    } else {
                        // Since the tzname is getting stuffed into the
                        // format, we have to double any % signs so that
                        // strftime doesn't treat them as format codes.
                        Py_DECREF!(Zreplacement);
                        Zreplacement = PyObject_CallMethod(
                            temp,
                            c"replace".as_ptr(),
                            c"ss".as_ptr(),
                            c"%".as_ptr() as *mut i8,
                            c"%%".as_ptr() as *mut i8,
                        );
                        Py_DECREF!(temp);
                        if Zreplacement.is_null() {
                            return core::ptr::null_mut::<_object>();
                        } else {
                            if PyUnicode_Check!(Zreplacement) == 0 {
                                PyErr_SetString(
                                    PyExc_TypeError,
                                    c"tzname.replace() did not return a string".as_ptr(),
                                );
                            } else {
                                return Zreplacement;
                            }
                        }
                    }
                }
                Py_DECREF!(Zreplacement);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

fn make_freplacement(object: *mut _object) -> *mut _object {
    let mut freplacement: __SlateAlign16<[i8; 64]> = __SlateAlign16([0i8; 64]);
    if PyTime_Check!(object) != 0 {
        unsafe {
            sprintf(
                (freplacement.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                c"%06d".as_ptr() as *const core::ffi::c_char,
                (*((*(object as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(3 as isize) as i32)
                    << 16i32
                    | (*((*(object as *mut __SlateRecord_datetime_h_86))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(4 as isize) as i32)
                        << 8i32
                    | (*((*(object as *mut __SlateRecord_datetime_h_86))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(5 as isize) as i32),
            ) as i32
        };
    } else {
        if PyDateTime_Check!(object) != 0 {
            unsafe {
                sprintf(
                    (freplacement.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                    c"%06d".as_ptr() as *const core::ffi::c_char,
                    (*((*(object as *mut __SlateRecord_datetime_h_114))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(7 as isize) as i32)
                        << 16i32
                        | (*((*(object as *mut __SlateRecord_datetime_h_114))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(8 as isize) as i32)
                            << 8i32
                        | (*((*(object as *mut __SlateRecord_datetime_h_114))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(9 as isize) as i32),
                ) as i32
            };
        } else {
            unsafe {
                sprintf(
                    (freplacement.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                    c"%06d".as_ptr() as *const core::ffi::c_char,
                    0i32,
                ) as i32
            };
        }
    }
    unsafe { PyUnicode_FromString(freplacement.0.as_mut_ptr() as *const i8) }
}

/// I sure don't want to reproduce the strftime code from the time module,
/// so this imports the module and calls it.  All the hair is due to
/// giving special meanings to the %z, %:z, %Z and %f format codes via a
/// preprocessing step on the format string.
/// tzinfoarg is the argument to pass to the object's tzinfo method, if
/// needed.
fn wrap_strftime(
    object: *mut _object,
    format: *mut _object,
    timetuple: *mut _object,
    tzinfoarg: *mut _object,
) -> *mut _object {
    let mut newformat: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    // Buffer of maximum size of formatted year permitted by long.
    // +6 to accommodate dashes, 2-digit month and day for %F.
    let mut buf: core::mem::MaybeUninit<__SlateAlign16<[i8; 28]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut [i8; 28] = core::ptr::addr_of_mut!(buf) as *mut [i8; 28];
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut year: *mut _object = unsafe { core::mem::zeroed() };
    let mut year_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    let mut year_long: i64 = unsafe { core::mem::zeroed() };
    // 0-pad year with century as necessary
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v13: bool = unsafe { core::mem::zeroed() };
    // A % has been seen and ch is the character after it.
    let mut replacement: *mut _object = unsafe { core::mem::zeroed() };
    let mut ch: u32 = unsafe { core::mem::zeroed() };
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut flen: i64 = unsafe { core::mem::zeroed() };
    let mut writer: *mut PyUnicodeWriter = unsafe { core::mem::zeroed() };
    let mut strftime: *mut _object = unsafe { core::mem::zeroed() };
    // py string, replacement for %f
    let mut freplacement: *mut _object = unsafe { core::mem::zeroed() };
    // py string, replacement for %Z
    let mut Zreplacement: *mut _object = unsafe { core::mem::zeroed() };
    // py string, replacement for %:z
    let mut colonzreplacement: *mut _object = unsafe { core::mem::zeroed() };
    // py string, replacement for %z
    let mut zreplacement: *mut _object = unsafe { core::mem::zeroed() };
    // guilty until proved innocent
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        zreplacement = core::ptr::null_mut::<_object>();
        colonzreplacement = core::ptr::null_mut::<_object>();
        Zreplacement = core::ptr::null_mut::<_object>();
        freplacement = core::ptr::null_mut::<_object>();
        strftime = PyImport_ImportModuleAttrString(c"time".as_ptr(), c"strftime".as_ptr());
        if strftime.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_1: {
                '__join_0: {
                    // Scan the input format, looking for %z/%Z/%f escapes, building
                    // a new format.  Since computing the replacements for those codes
                    // is expensive, don't unless they're actually used.
                    writer = PyUnicodeWriter_Create(0i64);
                    if !writer.is_null() {
                        flen = PyUnicode_GET_LENGTH!(format);
                        i = 0;
                        start = 0;
                        end = 0;
                        '__join_6: {
                            loop {
                                if i != flen {
                                    i = PyUnicode_FindChar(format, 37u32, i, flen, 1i32);
                                    if i < 0 {
                                        break;
                                    } else {
                                        end = i;
                                        i = i.wrapping_add(1);
                                        if i == flen {
                                            break;
                                        } else {
                                            ch = PyUnicode_READ_CHAR!(format, i);
                                            i = i.wrapping_add(1);
                                            replacement = core::ptr::null_mut::<_object>();
                                            if ch == 122 {
                                                // %z -> +HHMM
                                                if zreplacement.is_null() {
                                                    zreplacement = make_somezreplacement(
                                                        object,
                                                        c"".as_ptr() as *mut i8,
                                                        tzinfoarg,
                                                    );
                                                    if zreplacement.is_null() {
                                                        break '__join_0;
                                                    }
                                                }
                                                replacement = zreplacement;
                                            } else {
                                                if ch == 58 && i < flen {
                                                    __v13 = PyUnicode_READ_CHAR!(format, i) == 122;
                                                } else {
                                                    __v13 = false;
                                                }
                                                if __v13 {
                                                    // %:z -> +HH:MM
                                                    i = i.wrapping_add(1);
                                                    if colonzreplacement.is_null() {
                                                        colonzreplacement = make_somezreplacement(
                                                            object,
                                                            c":".as_ptr() as *mut i8,
                                                            tzinfoarg,
                                                        );
                                                        if colonzreplacement.is_null() {
                                                            break '__join_0;
                                                        }
                                                    }
                                                    replacement = colonzreplacement;
                                                } else {
                                                    if ch == 90 {
                                                        // format tzname
                                                        if Zreplacement.is_null() {
                                                            Zreplacement = make_Zreplacement(
                                                                object, tzinfoarg,
                                                            );
                                                            if Zreplacement.is_null() {
                                                                break '__join_0;
                                                            }
                                                        }
                                                        replacement = Zreplacement;
                                                    } else {
                                                        if ch == 102 {
                                                            // format microseconds
                                                            if freplacement.is_null() {
                                                                freplacement =
                                                                    make_freplacement(object);
                                                                if freplacement.is_null() {
                                                                    break '__join_0;
                                                                }
                                                            }
                                                            replacement = freplacement;
                                                        } else {
                                                            if normalize_century() != 0
                                                                && (ch == 89
                                                                    || ch == 71
                                                                    || ch == 70
                                                                    || ch == 67)
                                                            {
                                                                item = PySequence_GetItem(
                                                                    timetuple, 0i64,
                                                                );
                                                                if item.is_null() {
                                                                    break '__join_0;
                                                                } else {
                                                                    year_long = PyLong_AsLong(item);
                                                                    Py_DECREF!(item);
                                                                    if year_long
                                                                        == (1i32.wrapping_neg()
                                                                            as i64)
                                                                    {
                                                                        __v10 = !PyErr_Occurred()
                                                                            .is_null();
                                                                    } else {
                                                                        __v10 = false;
                                                                    }
                                                                    if __v10 {
                                                                        break '__join_0;
                                                                    } else {
                                                                        // Note that datetime(1000, 1, 1).strftime('%G') == '1000' so year
                                                                        // 1000 for %G can go on the fast path.
                                                                        if year_long >= 1000 {
                                                                            continue;
                                                                        } else {
                                                                            if ch == 71 {
                                                                                year_str = PyObject_CallFunction(strftime, c"sO".as_ptr(), c"%G".as_ptr() as *mut i8, timetuple);
                                                                                if year_str
                                                                                    .is_null()
                                                                                {
                                                                                    break '__join_0;
                                                                                } else {
                                                                                    year = PyNumber_Long(year_str);
                                                                                    Py_DECREF!(
                                                                                        year_str
                                                                                    );
                                                                                    if year
                                                                                        .is_null()
                                                                                    {
                                                                                        break '__join_0;
                                                                                    } else {
                                                                                        year_long = PyLong_AsLong(year);
                                                                                        Py_DECREF!(
                                                                                            year
                                                                                        );
                                                                                        if year_long == (1i32.wrapping_neg() as i64) {
__v7 = !PyErr_Occurred().is_null();
} else {
__v7 = false;
}
                                                                                        if __v7 {
                                                                                            break '__join_0;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            n = PyOS_snprintf(
                                                                                (*__slate_slot_6)
                                                                                    .as_mut_ptr()
                                                                                    as *mut i8,
                                                                                core::mem::size_of::<
                                                                                    [i8; 28],
                                                                                >(
                                                                                )
                                                                                    as u64,
                                                                                (if ch == 70 {
                                                                                    c"%04ld-%%m-%%d"
                                                                                        .as_ptr()
                                                                                        as *mut i8
                                                                                } else {
                                                                                    c"%04ld"
                                                                                        .as_ptr()
                                                                                        as *mut i8
                                                                                })
                                                                                    as *const i8,
                                                                                year_long,
                                                                            )
                                                                                as i64;
                                                                            if ch == 67 {
                                                                                n = n.wrapping_sub(
                                                                                    2,
                                                                                );
                                                                            }
                                                                            if PyUnicodeWriter_WriteSubstring(writer, format, start, end) < 0 {
break '__join_0;
} else {
start = i;
if PyUnicodeWriter_WriteUTF8(writer, (*__slate_slot_6).as_mut_ptr() as *const i8, n) < 0 {
break '__join_0;
} else {
continue;
}
}
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                // percent followed by something else
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if PyUnicodeWriter_WriteSubstring(
                                                writer, format, start, end,
                                            ) < 0
                                            {
                                                break '__join_0;
                                            } else {
                                                start = i;
                                                if PyUnicodeWriter_WriteStr(writer, replacement) < 0
                                                {
                                                    break '__join_0;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    break '__join_6;
                                }
                            }
                        }
                        // end while()
                        if start == 0 {
                            PyUnicodeWriter_Discard(writer);
                            newformat = Py_NewRef!(format);
                        } else {
                            if PyUnicodeWriter_WriteSubstring(writer, format, start, flen) < 0 {
                                break '__join_0;
                            } else {
                                newformat = PyUnicodeWriter_Finish(writer);
                                if newformat.is_null() {
                                    break '__join_1;
                                }
                            }
                        }
                        result = PyObject_CallFunctionObjArgs(
                            strftime,
                            newformat,
                            timetuple,
                            core::ptr::null_mut::<()>(),
                        );
                        Py_DECREF!(newformat);
                        break '__join_1;
                    }
                }
                PyUnicodeWriter_Discard(writer);
            }
            Py_XDECREF!(freplacement);
            Py_XDECREF!(zreplacement);
            Py_XDECREF!(colonzreplacement);
            Py_XDECREF!(Zreplacement);
            Py_XDECREF!(strftime);
            return result;
        }
    }
}

// Wrap functions from the time module.  These aren't directly available
// from C.  Perhaps they should be.
/// Call time.time() and return its result (a Python float).
fn time_time() -> *mut _object {
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    let time: *mut _object =
        unsafe { PyImport_ImportModuleAttrString(c"time".as_ptr(), c"time".as_ptr()) };
    if !time.is_null() {
        result = unsafe { PyObject_CallNoArgs(time) };
        Py_DECREF!(time);
    }
    result
}

/// Build a time.struct_time.  The weekday and day number are automatically
/// computed from the y,m,d args.
fn build_struct_time(
    y: i32,
    m: i32,
    d: i32,
    hh: i32,
    mm: i32,
    ss: i32,
    dstflag: i32,
) -> *mut _object {
    let struct_time: *mut _object =
        unsafe { PyImport_ImportModuleAttrString(c"time".as_ptr(), c"struct_time".as_ptr()) };
    if struct_time.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe {
        PyObject_CallFunction(
            struct_time,
            c"((iiiiiiiii))".as_ptr(),
            y,
            m,
            d,
            hh,
            mm,
            ss,
            weekday(y, m, d),
            days_before_month(y, m).wrapping_add(d),
            dstflag,
        )
    };
    Py_DECREF!(struct_time);
    result
}

// Miscellaneous helpers.
// The comparisons here all most naturally compute a cmp()-like result.
// This little helper turns that into a bool result for rich comparisons.
fn diff_to_bool(diff: i32, op: i32) -> *mut _object {
    loop {
        match op {
            2 => {
                if diff == 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            3 => {
                if diff != 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            0 => {
                if diff < 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            4 => {
                if diff > 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            1 => {
                if diff <= 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            5 => {
                if diff >= 0 {
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

// Class implementations.
// PyDateTime_Delta implementation.
/// Convert a timedelta to a number of us,
///      (24*3600*self.days + self.seconds)*1000000 + self.microseconds
/// as a Python int.
/// Doing mixed-radix arithmetic by hand instead is excruciating in C,
/// due to ubiquitous overflow possibilities.
fn delta_to_microseconds(self_: *mut __SlateRecord_datetime_h_41) -> *mut _object {
    let mut st: *mut __SlateRecord__datetimemodule_c_54 = unsafe { core::mem::zeroed() };
    let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut x3: *mut _object = unsafe { core::mem::zeroed() };
    let mut x2: *mut _object = unsafe { core::mem::zeroed() };
    let mut x1: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        x1 = core::ptr::null_mut::<_object>();
        x2 = core::ptr::null_mut::<_object>();
        x3 = core::ptr::null_mut::<_object>();
        result = core::ptr::null_mut::<_object>();
        st = GET_CURRENT_STATE!(current_mod);
        if st.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            x1 = PyLong_FromLong(GET_TD_DAYS__2!(self_) as i64);
            if !x1.is_null() {
                x2 = PyNumber_Multiply(x1, CONST_SEC_PER_DAY!(st)); // days in seconds
                if !x2.is_null() {
                    Py_SETREF__3!(x1);
                    // x2 has days in seconds
                    x1 = PyLong_FromLong(GET_TD_SECONDS__2!(self_) as i64); // seconds
                    if !x1.is_null() {
                        x3 = PyNumber_Add(x1, x2); // days and seconds in seconds
                        if !x3.is_null() {
                            Py_DECREF!(x1);
                            Py_DECREF!(x2);
                            // x1 =
                            x2 = core::ptr::null_mut::<_object>();
                            // x3 has days+seconds in seconds
                            x1 = PyNumber_Multiply(x3, CONST_US_PER_SECOND!(st)); // us
                            if !x1.is_null() {
                                Py_SETREF__3!(x3);
                                // x1 has days+seconds in us
                                x2 = PyLong_FromLong(GET_TD_MICROSECONDS__2!(self_) as i64);
                                if !x2.is_null() {
                                    result = PyNumber_Add(x1, x2);
                                }
                            }
                        }
                    }
                }
            }
            Py_XDECREF!(x1);
            Py_XDECREF!(x2);
            Py_XDECREF!(x3);
            RELEASE_CURRENT_STATE!(current_mod);
            return result;
        }
    }
}

fn checked_divmod(a: *mut _object, b: *mut _object) -> *mut _object {
    let result: *mut _object = unsafe { PyNumber_Divmod(a, b) };
    if !result.is_null() {
        if PyTuple_Check!(result) == 0 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"divmod() returned non-tuple (type %.200s)".as_ptr(),
                    (*Py_TYPE!(result)).tp_name,
                )
            };
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
        if PyTuple_GET_SIZE!(result) != 2 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"divmod() returned a tuple of size %zd".as_ptr(),
                    PyTuple_GET_SIZE!(result),
                )
            };
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
    }
    result
}

/// Convert a number of us (as a Python int) to a timedelta.
fn microseconds_to_delta_ex(pyus: *mut _object, r#type: *mut _typeobject) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut st: *mut __SlateRecord__datetimemodule_c_54 = unsafe { core::mem::zeroed() };
    let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut num: *mut _object = unsafe { core::mem::zeroed() };
    let mut tuple: *mut _object = unsafe { core::mem::zeroed() };
    let mut d: i32 = unsafe { core::mem::zeroed() };
    let mut s: i32 = unsafe { core::mem::zeroed() };
    let mut us: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        tuple = core::ptr::null_mut::<_object>();
        num = core::ptr::null_mut::<_object>();
        result = core::ptr::null_mut::<_object>();
        st = GET_CURRENT_STATE!(current_mod);
        if st.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_1: {
                tuple = checked_divmod(pyus, CONST_US_PER_SECOND!(st));
                if !tuple.is_null() {
                    num = PyTuple_GET_ITEM!(tuple, 1i32); // us
                    us = PyLong_AsInt(num);
                    num = core::ptr::null_mut::<_object>();
                    if us == 1i32.wrapping_neg() {
                        __v2 = !PyErr_Occurred().is_null();
                    } else {
                        __v2 = false;
                    }
                    if !__v2 {
                        if 0 <= us && us < 1000000 {
                            num = Py_NewRef!(PyTuple_GET_ITEM!(tuple, 0i32)); // leftover seconds
                            Py_DECREF!(tuple);
                            tuple = checked_divmod(num, CONST_SEC_PER_DAY!(st));
                            if tuple.is_null() {
                                break '__join_1;
                            } else {
                                Py_DECREF!(num);
                                num = PyTuple_GET_ITEM!(tuple, 1i32); // seconds
                                s = PyLong_AsInt(num);
                                num = core::ptr::null_mut::<_object>();
                                if s == 1i32.wrapping_neg() {
                                    __v1 = !PyErr_Occurred().is_null();
                                } else {
                                    __v1 = false;
                                }
                                if __v1 {
                                    break '__join_1;
                                } else {
                                    if 0 <= s && s < 86400i32 {
                                        num = Py_NewRef!(PyTuple_GET_ITEM!(tuple, 0i32)); // leftover days
                                        d = PyLong_AsInt(num);
                                        if d == 1i32.wrapping_neg() {
                                            __v0 = !PyErr_Occurred().is_null();
                                        } else {
                                            __v0 = false;
                                        }
                                        if __v0 {
                                            break '__join_1;
                                        } else {
                                            result = new_delta_ex(d, s, us, 0i32, r#type);
                                            break '__join_1;
                                        }
                                    }
                                }
                            }
                        }
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"divmod() returned a value out of range".as_ptr(),
                        );
                    }
                }
            }
            Py_XDECREF!(tuple);
            Py_XDECREF!(num);
            RELEASE_CURRENT_STATE!(current_mod);
            return result;
        }
    }
}

fn multiply_int_timedelta(
    intobj: *mut _object,
    delta: *mut __SlateRecord_datetime_h_41,
) -> *mut _object {
    let pyus_in: *mut _object = delta_to_microseconds(delta);
    if pyus_in.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_out: *mut _object = unsafe { PyNumber_Multiply(intobj, pyus_in) };
    Py_DECREF!(pyus_in);
    if pyus_out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = microseconds_to_delta!(pyus_out);
    Py_DECREF!(pyus_out);
    result
}

fn get_float_as_integer_ratio(floatobj: *mut _object) -> *mut _object {
    let ratio: *mut _object = PyObject_CallMethodNoArgs(floatobj, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_as_integer_ratio
                ._ascii
                .ob_base
        )
    });
    if ratio.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyTuple_Check!(ratio) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"unexpected return type from as_integer_ratio(): expected tuple, not '%.200s'"
                    .as_ptr(),
                (*Py_TYPE!(ratio)).tp_name,
            )
        };
        Py_DECREF!(ratio);
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyTuple_Size(ratio) }) != 2 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"as_integer_ratio() must return a 2-tuple".as_ptr(),
            )
        };
        Py_DECREF!(ratio);
        return core::ptr::null_mut::<_object>();
    }
    ratio
}

/// op is 0 for multiplication, 1 for division
fn multiply_truedivide_timedelta_float(
    delta: *mut __SlateRecord_datetime_h_41,
    floatobj: *mut _object,
    op: i32,
) -> *mut _object {
    let mut ratio: *mut _object = unsafe { core::mem::zeroed() };
    let mut pyus_out: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut pyus_in: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        pyus_in = core::ptr::null_mut::<_object>();
        ratio = core::ptr::null_mut::<_object>();
        pyus_in = delta_to_microseconds(delta);
        if pyus_in.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            ratio = get_float_as_integer_ratio(floatobj);
            if !ratio.is_null() {
                temp = PyNumber_Multiply(pyus_in, PyTuple_GET_ITEM!(ratio, op));
                Py_SETREF__3!(pyus_in);
                if !temp.is_null() {
                    pyus_out = divide_nearest(temp, PyTuple_GET_ITEM__3!(ratio, op == 0));
                    Py_DECREF!(temp);
                    if !pyus_out.is_null() {
                        result = microseconds_to_delta!(pyus_out);
                        Py_DECREF!(pyus_out);
                    }
                }
            }
            Py_XDECREF!(pyus_in);
            Py_XDECREF!(ratio);
            return result;
        }
    }
}

fn divide_timedelta_int(
    delta: *mut __SlateRecord_datetime_h_41,
    intobj: *mut _object,
) -> *mut _object {
    let pyus_in: *mut _object = delta_to_microseconds(delta);
    if pyus_in.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_out: *mut _object = unsafe { PyNumber_FloorDivide(pyus_in, intobj) };
    Py_DECREF!(pyus_in);
    if pyus_out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = microseconds_to_delta!(pyus_out);
    Py_DECREF!(pyus_out);
    result
}

fn divide_timedelta_timedelta(
    left: *mut __SlateRecord_datetime_h_41,
    right: *mut __SlateRecord_datetime_h_41,
) -> *mut _object {
    let pyus_left: *mut _object = delta_to_microseconds(left);
    if pyus_left.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_right: *mut _object = delta_to_microseconds(right);
    if pyus_right.is_null() {
        Py_DECREF!(pyus_left);
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe { PyNumber_FloorDivide(pyus_left, pyus_right) };
    Py_DECREF!(pyus_left);
    Py_DECREF!(pyus_right);
    result
}

fn truedivide_timedelta_timedelta(
    left: *mut __SlateRecord_datetime_h_41,
    right: *mut __SlateRecord_datetime_h_41,
) -> *mut _object {
    let pyus_left: *mut _object = delta_to_microseconds(left);
    if pyus_left.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_right: *mut _object = delta_to_microseconds(right);
    if pyus_right.is_null() {
        Py_DECREF!(pyus_left);
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe { PyNumber_TrueDivide(pyus_left, pyus_right) };
    Py_DECREF!(pyus_left);
    Py_DECREF!(pyus_right);
    result
}

fn truedivide_timedelta_int(
    delta: *mut __SlateRecord_datetime_h_41,
    i: *mut _object,
) -> *mut _object {
    let pyus_in: *mut _object = delta_to_microseconds(delta);
    if pyus_in.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_out: *mut _object = divide_nearest(pyus_in, i);
    Py_DECREF!(pyus_in);
    if pyus_out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = microseconds_to_delta!(pyus_out);
    Py_DECREF!(pyus_out);
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_add")]
extern "C-unwind" fn delta_add(left: *mut _object, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    let __v0: bool = PyDelta_Check!(left) != 0 && PyDelta_Check!(right) != 0;
    if __v0 {
        // delta + delta
        //
        // The C-level additions can't overflow because of the
        // invariant bounds.
        let days: i32 = GET_TD_DAYS!(left).wrapping_add(GET_TD_DAYS!(right));
        let seconds: i32 = GET_TD_SECONDS!(left).wrapping_add(GET_TD_SECONDS!(right));
        let microseconds: i32 =
            GET_TD_MICROSECONDS!(left).wrapping_add(GET_TD_MICROSECONDS!(right));
        result = new_delta!(days, seconds, microseconds, 1i32);
    }
    if result == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_INCREF!(result);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_negative")]
extern "C-unwind" fn delta_negative(self_: *mut _object) -> *mut _object {
    new_delta!(
        GET_TD_DAYS!(self_).wrapping_neg(),
        GET_TD_SECONDS!(self_).wrapping_neg(),
        GET_TD_MICROSECONDS!(self_).wrapping_neg(),
        1i32
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_positive")]
extern "C-unwind" fn delta_positive(self_: *mut _object) -> *mut _object {
    // Could optimize this (by returning self) if this isn't a
    // subclass -- but who uses unary + ?  Approximately nobody.
    new_delta!(
        GET_TD_DAYS!(self_),
        GET_TD_SECONDS!(self_),
        GET_TD_MICROSECONDS!(self_),
        0i32
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_abs")]
extern "C-unwind" fn delta_abs(self_: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if GET_TD_DAYS!(self_) < 0 {
        result = delta_negative(self_);
    } else {
        result = delta_positive(self_);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_subtract")]
extern "C-unwind" fn delta_subtract(left: *mut _object, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    let __v0: bool = PyDelta_Check!(left) != 0 && PyDelta_Check!(right) != 0;
    if __v0 {
        // delta - delta
        //
        // The C-level additions can't overflow because of the
        // invariant bounds.
        let days: i32 = GET_TD_DAYS!(left).wrapping_sub(GET_TD_DAYS!(right));
        let seconds: i32 = GET_TD_SECONDS!(left).wrapping_sub(GET_TD_SECONDS!(right));
        let microseconds: i32 =
            GET_TD_MICROSECONDS!(left).wrapping_sub(GET_TD_MICROSECONDS!(right));
        result = new_delta!(days, seconds, microseconds, 1i32);
    }
    if result == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_INCREF!(result);
    }
    result
}

fn delta_cmp(self_: *mut _object, other: *mut _object) -> i32 {
    let mut diff: i32 = GET_TD_DAYS!(self_).wrapping_sub(GET_TD_DAYS!(other));
    if diff == 0 {
        diff = GET_TD_SECONDS!(self_).wrapping_sub(GET_TD_SECONDS!(other));
        if diff == 0 {
            diff = GET_TD_MICROSECONDS!(self_).wrapping_sub(GET_TD_MICROSECONDS!(other));
        }
    }
    diff
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_richcompare")]
extern "C-unwind" fn delta_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    if PyDelta_Check!(other) != 0 {
        let diff: i32 = delta_cmp(self_, other);
        return diff_to_bool(diff, op);
    } else {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_hash")]
extern "C-unwind" fn delta_hash(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_datetime_h_41 = PyDelta_CAST!(op);
    let mut hash: i64 = unsafe { (*self_).hashcode };
    if hash == (1i32.wrapping_neg() as i64) {
        let temp: *mut _object = delta_getstate(self_);
        if !temp.is_null() {
            hash = unsafe { PyObject_Hash(temp) };
            FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).hashcode, hash);
            Py_DECREF!(temp);
        }
    }
    hash
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_multiply")]
extern "C-unwind" fn delta_multiply(left: *mut _object, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    if PyDelta_Check!(left) != 0 {
        // delta * ???
        if PyLong_Check!(right) != 0 {
            result = multiply_int_timedelta(right, left as *mut __SlateRecord_datetime_h_41);
        } else {
            if PyFloat_Check!(right) != 0 {
                result = multiply_truedivide_timedelta_float(
                    left as *mut __SlateRecord_datetime_h_41,
                    right,
                    0i32,
                );
            }
        }
    } else {
        if PyLong_Check!(left) != 0 {
            result = multiply_int_timedelta(left, right as *mut __SlateRecord_datetime_h_41);
        } else {
            if PyFloat_Check!(left) != 0 {
                result = multiply_truedivide_timedelta_float(
                    right as *mut __SlateRecord_datetime_h_41,
                    left,
                    0i32,
                );
            }
        }
    }
    if result == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_INCREF!(result);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_divide")]
extern "C-unwind" fn delta_divide(left: *mut _object, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    if PyDelta_Check!(left) != 0 {
        // delta * ???
        if PyLong_Check!(right) != 0 {
            result = divide_timedelta_int(left as *mut __SlateRecord_datetime_h_41, right);
        } else {
            if PyDelta_Check!(right) != 0 {
                result = divide_timedelta_timedelta(
                    left as *mut __SlateRecord_datetime_h_41,
                    right as *mut __SlateRecord_datetime_h_41,
                );
            }
        }
    }
    if result == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_INCREF!(result);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_truedivide")]
extern "C-unwind" fn delta_truedivide(left: *mut _object, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    if PyDelta_Check!(left) != 0 {
        if PyDelta_Check!(right) != 0 {
            result = truedivide_timedelta_timedelta(
                left as *mut __SlateRecord_datetime_h_41,
                right as *mut __SlateRecord_datetime_h_41,
            );
        } else {
            if PyFloat_Check!(right) != 0 {
                result = multiply_truedivide_timedelta_float(
                    left as *mut __SlateRecord_datetime_h_41,
                    right,
                    1i32,
                );
            } else {
                if PyLong_Check!(right) != 0 {
                    result =
                        truedivide_timedelta_int(left as *mut __SlateRecord_datetime_h_41, right);
                }
            }
        }
    }
    if result == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_INCREF!(result);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_remainder")]
extern "C-unwind" fn delta_remainder(left: *mut _object, right: *mut _object) -> *mut _object {
    let __v0: bool = PyDelta_Check!(left) == 0 || PyDelta_Check!(right) == 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    let pyus_left: *mut _object = delta_to_microseconds(left as *mut __SlateRecord_datetime_h_41);
    if pyus_left.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_right: *mut _object = delta_to_microseconds(right as *mut __SlateRecord_datetime_h_41);
    if pyus_right.is_null() {
        Py_DECREF!(pyus_left);
        return core::ptr::null_mut::<_object>();
    }
    let pyus_remainder: *mut _object = unsafe { PyNumber_Remainder(pyus_left, pyus_right) };
    Py_DECREF!(pyus_left);
    Py_DECREF!(pyus_right);
    if pyus_remainder.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let remainder: *mut _object = microseconds_to_delta!(pyus_remainder);
    Py_DECREF!(pyus_remainder);
    if remainder.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    remainder
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_divmod")]
extern "C-unwind" fn delta_divmod(left: *mut _object, right: *mut _object) -> *mut _object {
    let __v0: bool = PyDelta_Check!(left) == 0 || PyDelta_Check!(right) == 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    let pyus_left: *mut _object = delta_to_microseconds(left as *mut __SlateRecord_datetime_h_41);
    if pyus_left.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let pyus_right: *mut _object = delta_to_microseconds(right as *mut __SlateRecord_datetime_h_41);
    if pyus_right.is_null() {
        Py_DECREF!(pyus_left);
        return core::ptr::null_mut::<_object>();
    }
    let divmod: *mut _object = checked_divmod(pyus_left, pyus_right);
    Py_DECREF!(pyus_left);
    Py_DECREF!(pyus_right);
    if divmod.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let delta: *mut _object = microseconds_to_delta!(PyTuple_GET_ITEM!(divmod, 1i32));
    if delta.is_null() {
        Py_DECREF!(divmod);
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object =
        unsafe { PyTuple_Pack(2i64, PyTuple_GET_ITEM!(divmod, 0i32), delta) };
    Py_DECREF!(delta);
    Py_DECREF!(divmod);
    result
}

/// Fold in the value of the tag ("seconds", "weeks", etc) component of a
/// timedelta constructor.  sofar is the # of microseconds accounted for
/// so far, and there are factor microseconds per current unit, the number
/// of which is given by num.  num * factor is added to sofar in a
/// numerically careful way, and that's the result.  Any fractional
/// microseconds left over (this can happen if num is a float type) are
/// added into *leftover.
/// Note that there are many ways this can give an error (NULL) return.
fn accum(
    tag: *const i8,
    sofar: *mut _object,
    num: *mut _object,
    factor: *mut _object,
    leftover: &mut f64,
) -> *mut _object {
    let mut prod: *mut _object = unsafe { core::mem::zeroed() };
    let mut sum: *mut _object = unsafe { core::mem::zeroed() };
    if PyLong_Check!(num) != 0 {
        prod = unsafe { PyNumber_Multiply(num, factor) };
        if prod.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        sum = unsafe { PyNumber_Add(sofar, prod) };
        Py_DECREF!(prod);
        return sum;
    }
    if PyFloat_Check!(num) != 0 {
        let mut dnum: f64 = 0 as f64;
        let mut fracpart: f64 = 0 as f64;
        let mut intpart: f64 = 0 as f64;
        let mut x: *mut _object = unsafe { core::mem::zeroed() };
        // The Plan:  decompose num into an integer part and a
        // fractional part, num = intpart + fracpart.
        // Then num * factor ==
        //      intpart * factor + fracpart * factor
        // and the LHS can be computed exactly in long arithmetic.
        // The RHS is again broken into an int part and frac part.
        // and the frac part is added into *leftover.
        dnum = unsafe { PyFloat_AsDouble(num) };
        let __v0: bool = dnum == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return core::ptr::null_mut::<_object>();
        }
        fracpart = unsafe { modf(dnum, core::ptr::addr_of_mut!(intpart)) };
        x = unsafe { PyLong_FromDouble(intpart) };
        if x.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        prod = unsafe { PyNumber_Multiply(x, factor) };
        Py_DECREF!(x);
        if prod.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        sum = unsafe { PyNumber_Add(sofar, prod) };
        Py_DECREF!(prod);
        if sum.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if fracpart == 0.0f64 {
            return sum;
        }
        // So far we've lost no information.  Dealing with the
        // fractional part requires float arithmetic, and may
        // lose a little info.
        dnum = unsafe { PyLong_AsDouble(factor) };
        dnum *= fracpart;
        fracpart = unsafe { modf(dnum, core::ptr::addr_of_mut!(intpart)) };
        x = unsafe { PyLong_FromDouble(intpart) };
        if x.is_null() {
            Py_DECREF!(sum);
            return core::ptr::null_mut::<_object>();
        }
        let y: *mut _object = unsafe { PyNumber_Add(sum, x) };
        Py_DECREF!(sum);
        Py_DECREF!(x);
        unsafe {
            *leftover = *leftover + fracpart;
        }
        return y;
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"unsupported type for timedelta %s component: %s".as_ptr(),
            tag,
            (*Py_TYPE!(num)).tp_name,
        )
    };
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_new")]
extern "C-unwind" fn delta_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut x_is_odd: i32 = unsafe { core::mem::zeroed() };
    // Round to nearest whole # of us, and add into x.
    let mut whole_us: f64 = unsafe { core::mem::zeroed() };
    let mut leftover_us: f64 = unsafe { core::mem::zeroed() };
    // temp sum of microseconds
    let mut y: *mut _object = unsafe { core::mem::zeroed() };
    // running sum of microseconds
    let mut x: *mut _object = unsafe { core::mem::zeroed() };
    let mut week: *mut _object = unsafe { core::mem::zeroed() };
    let mut hour: *mut _object = unsafe { core::mem::zeroed() };
    let mut minute: *mut _object = unsafe { core::mem::zeroed() };
    let mut ms: *mut _object = unsafe { core::mem::zeroed() };
    let mut us: *mut _object = unsafe { core::mem::zeroed() };
    let mut second: *mut _object = unsafe { core::mem::zeroed() };
    // Argument objects.
    let mut day: *mut _object = unsafe { core::mem::zeroed() };
    let mut st: *mut __SlateRecord__datetimemodule_c_54 = unsafe { core::mem::zeroed() };
    let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
    let mut self_: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        self_ = core::ptr::null_mut::<_object>();
        st = GET_CURRENT_STATE!(current_mod);
        if st.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                day = core::ptr::null_mut::<_object>();
                second = core::ptr::null_mut::<_object>();
                us = core::ptr::null_mut::<_object>();
                ms = core::ptr::null_mut::<_object>();
                minute = core::ptr::null_mut::<_object>();
                hour = core::ptr::null_mut::<_object>();
                week = core::ptr::null_mut::<_object>();
                x = core::ptr::null_mut::<_object>();
                y = core::ptr::null_mut::<_object>();
                leftover_us = 0.0f64;
                if PyArg_ParseTupleAndKeywords(
                    args,
                    kw,
                    c"|OOOOOOO:__new__".as_ptr(),
                    (core::ptr::addr_of_mut!(keywords__2827.0) as *mut *mut i8) as *const *mut i8,
                    core::ptr::addr_of_mut!(day),
                    core::ptr::addr_of_mut!(second),
                    core::ptr::addr_of_mut!(us),
                    core::ptr::addr_of_mut!(ms),
                    core::ptr::addr_of_mut!(minute),
                    core::ptr::addr_of_mut!(hour),
                    core::ptr::addr_of_mut!(week),
                ) != 0
                {
                    x = PyLong_FromLong(0i64);
                    if !x.is_null() {
                        if !us.is_null() {
                            y = accum(
                                c"microseconds".as_ptr(),
                                x,
                                us,
                                _PyLong_GetOne(),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if !ms.is_null() {
                            y = accum(
                                c"milliseconds".as_ptr(),
                                x,
                                ms,
                                CONST_US_PER_MS!(st),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if !second.is_null() {
                            y = accum(
                                c"seconds".as_ptr(),
                                x,
                                second,
                                CONST_US_PER_SECOND!(st),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if !minute.is_null() {
                            y = accum(
                                c"minutes".as_ptr(),
                                x,
                                minute,
                                CONST_US_PER_MINUTE!(st),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if !hour.is_null() {
                            y = accum(
                                c"hours".as_ptr(),
                                x,
                                hour,
                                CONST_US_PER_HOUR!(st),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if !day.is_null() {
                            y = accum(
                                c"days".as_ptr(),
                                x,
                                day,
                                CONST_US_PER_DAY!(st),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if !week.is_null() {
                            y = accum(
                                c"weeks".as_ptr(),
                                x,
                                week,
                                CONST_US_PER_WEEK!(st),
                                &mut (*core::ptr::addr_of_mut!(leftover_us)),
                            );
                            Py_DECREF!(x);
                            x = y;
                            if x.is_null() {
                                break '__join_0;
                            }
                        }
                        if leftover_us != 0.0f64 {
                            whole_us = round(leftover_us);
                            if fabs(whole_us - leftover_us) == 0.5f64 {
                                // We're exactly halfway between two integers.  In order
                                // to do round-half-to-even, we must determine whether x
                                // is odd. Note that x is odd when it's last bit is 1. The
                                // code below uses bitwise and operation to check the last
                                // bit.
                                temp = PyNumber_And(x, _PyLong_GetOne()); // temp <- x & 1
                                if temp.is_null() {
                                    Py_DECREF!(x);
                                    break '__join_0;
                                } else {
                                    x_is_odd = PyObject_IsTrue(temp);
                                    Py_DECREF!(temp);
                                    if x_is_odd == 1i32.wrapping_neg() {
                                        Py_DECREF!(x);
                                        break '__join_0;
                                    } else {
                                        whole_us = 2.0f64
                                            * round((leftover_us + (x_is_odd as f64)) * 0.5f64)
                                            - (x_is_odd as f64);
                                    }
                                }
                            }
                            temp = PyLong_FromLong(whole_us as i64);
                            if temp.is_null() {
                                Py_DECREF!(x);
                                break '__join_0;
                            } else {
                                y = PyNumber_Add(x, temp);
                                Py_DECREF!(temp);
                                Py_DECREF!(x);
                                x = y;
                                if x.is_null() {
                                    break '__join_0;
                                }
                            }
                        }
                        self_ = microseconds_to_delta_ex(x, r#type);
                        Py_DECREF!(x);
                    }
                }
            }
            RELEASE_CURRENT_STATE!(current_mod);
            return self_;
        }
    }
}

static mut keywords__2827: __SlateAlign16<[*mut i8; 8]> = __SlateAlign16([
    b"days\0".as_ptr() as *mut i8,
    b"seconds\0".as_ptr() as *mut i8,
    b"microseconds\0".as_ptr() as *mut i8,
    b"milliseconds\0".as_ptr() as *mut i8,
    b"minutes\0".as_ptr() as *mut i8,
    b"hours\0".as_ptr() as *mut i8,
    b"weeks\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_bool")]
extern "C-unwind" fn delta_bool(self_: *mut _object) -> i32 {
    (GET_TD_DAYS!(self_) != 0 || GET_TD_SECONDS!(self_) != 0 || GET_TD_MICROSECONDS!(self_) != 0)
        as i32
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_repr")]
extern "C-unwind" fn delta_repr(self_: *mut _object) -> *mut _object {
    let mut args: *mut _object = unsafe { Py_GetConstant(7u32) };
    if args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut sep: *const i8 = c"".as_ptr();
    if GET_TD_DAYS!(self_) != 0 {
        Py_SETREF!(args, unsafe {
            PyUnicode_FromFormat(c"days=%d".as_ptr(), GET_TD_DAYS!(self_))
        });
        if args.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        sep = c", ".as_ptr();
    }
    if GET_TD_SECONDS!(self_) != 0 {
        Py_SETREF!(args, unsafe {
            PyUnicode_FromFormat(
                c"%U%sseconds=%d".as_ptr(),
                args,
                sep,
                GET_TD_SECONDS!(self_),
            )
        });
        if args.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        sep = c", ".as_ptr();
    }
    if GET_TD_MICROSECONDS!(self_) != 0 {
        Py_SETREF!(args, unsafe {
            PyUnicode_FromFormat(
                c"%U%smicroseconds=%d".as_ptr(),
                args,
                sep,
                GET_TD_MICROSECONDS!(self_),
            )
        });
        if args.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    }
    if PyUnicode_GET_LENGTH!(args) == 0 {
        Py_SETREF!(args, unsafe { PyUnicode_FromString(c"0".as_ptr()) });
        if args.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    }
    let repr: *mut _object =
        unsafe { PyUnicode_FromFormat(c"%s(%S)".as_ptr(), (*Py_TYPE!(self_)).tp_name, args) };
    Py_DECREF!(args);
    repr
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_str")]
extern "C-unwind" fn delta_str(self_: *mut _object) -> *mut _object {
    let us: i32 = GET_TD_MICROSECONDS!(self_);
    let mut seconds: i32 = GET_TD_SECONDS!(self_);
    let mut minutes: i32 = divmod(seconds, 60i32, core::ptr::addr_of_mut!(seconds));
    let hours: i32 = divmod(minutes, 60i32, core::ptr::addr_of_mut!(minutes));
    let days: i32 = GET_TD_DAYS!(self_);
    if days != 0 {
        if us != 0 {
            return unsafe {
                PyUnicode_FromFormat(
                    c"%d day%s, %d:%02d:%02d.%06d".as_ptr(),
                    days,
                    if days == 1 || days == 1i32.wrapping_neg() {
                        c"".as_ptr() as *mut i8
                    } else {
                        c"s".as_ptr() as *mut i8
                    },
                    hours,
                    minutes,
                    seconds,
                    us,
                )
            };
        } else {
            return unsafe {
                PyUnicode_FromFormat(
                    c"%d day%s, %d:%02d:%02d".as_ptr(),
                    days,
                    if days == 1 || days == 1i32.wrapping_neg() {
                        c"".as_ptr() as *mut i8
                    } else {
                        c"s".as_ptr() as *mut i8
                    },
                    hours,
                    minutes,
                    seconds,
                )
            };
        }
    } else {
        if us != 0 {
            return unsafe {
                PyUnicode_FromFormat(c"%d:%02d:%02d.%06d".as_ptr(), hours, minutes, seconds, us)
            };
        } else {
            return unsafe {
                PyUnicode_FromFormat(c"%d:%02d:%02d".as_ptr(), hours, minutes, seconds)
            };
        }
    }
}

// Pickle support, a simple use of __reduce__.
/// __getstate__ isn't exposed
fn delta_getstate(self_: *mut __SlateRecord_datetime_h_41) -> *mut _object {
    unsafe {
        Py_BuildValue(
            c"iii".as_ptr(),
            GET_TD_DAYS__2!(self_),
            GET_TD_SECONDS__2!(self_),
            GET_TD_MICROSECONDS__2!(self_),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_total_seconds")]
extern "C-unwind" fn delta_total_seconds(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let total_microseconds: *mut _object = delta_to_microseconds(PyDelta_CAST!(op));
    if total_microseconds.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
    let st: *mut __SlateRecord__datetimemodule_c_54 = GET_CURRENT_STATE!(current_mod);
    if st.is_null() {
        Py_DECREF!(total_microseconds);
        return core::ptr::null_mut::<_object>();
    }
    let total_seconds: *mut _object =
        unsafe { PyNumber_TrueDivide(total_microseconds, CONST_US_PER_SECOND!(st)) };
    RELEASE_CURRENT_STATE!(current_mod);
    Py_DECREF!(total_microseconds);
    total_seconds
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.delta_reduce")]
extern "C-unwind" fn delta_reduce(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_41 = PyDelta_CAST!(op);
    unsafe { Py_BuildValue(c"ON".as_ptr(), Py_TYPE__2!(self_), delta_getstate(self_)) }
}

static mut delta_members: __SlateAlign16<[PyMemberDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"days\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 1i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_datetime_h_41, days) as u64) as i64;
        __t0.flags = 1i32;
        __t0.doc = (b"Number of days.\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"seconds\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 1i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_datetime_h_41, seconds) as u64) as i64;
        __t1.flags = 1i32;
        __t1.doc =
            (b"Number of seconds (>= 0 and less than 1 day).\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"microseconds\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 1i32;
        __t2.offset =
            (core::mem::offset_of!(__SlateRecord_datetime_h_41, microseconds) as u64) as i64;
        __t2.flags = 1i32;
        __t2.doc = (b"Number of microseconds (>= 0 and less than 1 second).\0".as_ptr() as *mut i8)
            as *const i8;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut delta_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"total_seconds\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(delta_total_seconds);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = (b"Total seconds in the duration.\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(delta_reduce);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = (b"__reduce__() -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

static delta_doc: __SlateAlign16<[i8; 247]> = __SlateAlign16([
    68i8, 105i8, 102i8, 102i8, 101i8, 114i8, 101i8, 110i8, 99i8, 101i8, 32i8, 98i8, 101i8, 116i8,
    119i8, 101i8, 101i8, 110i8, 32i8, 116i8, 119i8, 111i8, 32i8, 100i8, 97i8, 116i8, 101i8, 116i8,
    105i8, 109i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 46i8, 10i8, 10i8, 116i8,
    105i8, 109i8, 101i8, 100i8, 101i8, 108i8, 116i8, 97i8, 40i8, 100i8, 97i8, 121i8, 115i8, 61i8,
    48i8, 44i8, 32i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 115i8, 61i8, 48i8, 44i8, 32i8,
    109i8, 105i8, 99i8, 114i8, 111i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 115i8, 61i8, 48i8,
    44i8, 32i8, 109i8, 105i8, 108i8, 108i8, 105i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 115i8,
    61i8, 48i8, 44i8, 32i8, 109i8, 105i8, 110i8, 117i8, 116i8, 101i8, 115i8, 61i8, 48i8, 44i8,
    32i8, 104i8, 111i8, 117i8, 114i8, 115i8, 61i8, 48i8, 44i8, 32i8, 119i8, 101i8, 101i8, 107i8,
    115i8, 61i8, 48i8, 41i8, 10i8, 10i8, 65i8, 108i8, 108i8, 32i8, 97i8, 114i8, 103i8, 117i8,
    109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 111i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 97i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 32i8, 116i8, 111i8, 32i8, 48i8, 46i8, 10i8, 65i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 105i8, 110i8,
    116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 32i8, 111i8, 114i8, 32i8, 102i8, 108i8, 111i8, 97i8,
    116i8, 115i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8,
    32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 118i8, 101i8, 32i8, 111i8, 114i8, 32i8, 110i8,
    101i8, 103i8, 97i8, 116i8, 105i8, 118i8, 101i8, 46i8, 0i8,
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
static mut delta_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = Some(delta_add);
    __t0.nb_subtract = Some(delta_subtract);
    __t0.nb_multiply = Some(delta_multiply);
    __t0.nb_remainder = Some(delta_remainder);
    __t0.nb_divmod = Some(delta_divmod);
    __t0.nb_power = None;
    __t0.nb_negative = Some(delta_negative);
    __t0.nb_positive = Some(delta_positive);
    __t0.nb_absolute = Some(delta_abs);
    __t0.nb_bool = Some(delta_bool);
    __t0.nb_invert = None;
    __t0.nb_lshift = None;
    __t0.nb_rshift = None;
    __t0.nb_and = None;
    __t0.nb_xor = None;
    __t0.nb_or = None;
    __t0.nb_int = None;
    __t0.nb_reserved = core::ptr::null_mut::<()>();
    __t0.nb_float = None;
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
    __t0.nb_floor_divide = Some(delta_divide);
    __t0.nb_true_divide = Some(delta_truedivide);
    __t0.nb_inplace_floor_divide = None;
    __t0.nb_inplace_true_divide = None;
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
/// XXX Can we make this const?
/// Letting this be set lazily is a benign race.
static mut zero_delta: __SlateRecord_datetime_h_41 = {
    let mut __t0: __SlateRecord_datetime_h_41 = unsafe { core::mem::zeroed() };
    __t0.ob_base = _object {
        __slate_anon_0: __SlateRecord_object_h_122 {
            ob_refcnt_full: 1407378104778752i64,
        },
        ob_type: unsafe { core::ptr::addr_of_mut!(PyDateTime_DeltaType) },
    };
    __t0.hashcode = 1i32.wrapping_neg() as i64;
    __t0
};

fn look_up_delta(
    days: i32,
    seconds: i32,
    microseconds: i32,
    r#type: *mut _typeobject,
) -> *mut __SlateRecord_datetime_h_41 {
    let __v0: bool = days == 0
        && seconds == 0
        && microseconds == 0
        && r#type == Py_TYPE__2!(unsafe { core::ptr::addr_of_mut!(zero_delta) });
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(zero_delta) };
    }
    core::ptr::null_mut::<__SlateRecord_datetime_h_41>()
}

// PyDateTime_Date implementation.
// Accessor properties.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_year")]
extern "C-unwind" fn date_year(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    unsafe {
        PyLong_FromLong(
            ((*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as i32) << 8i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) as i32))
                as i64,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_month")]
extern "C-unwind" fn date_month(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_day")]
extern "C-unwind" fn date_day(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) as i64)
    }
}

static mut date_getset: __SlateAlign16<[PyGetSetDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"year\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(date_year);
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"month\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(date_month);
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"day\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(date_day);
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

// Constructors.
static mut date_kws: __SlateAlign16<[*mut i8; 4]> = __SlateAlign16([
    b"year\0".as_ptr() as *mut i8,
    b"month\0".as_ptr() as *mut i8,
    b"day\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

fn date_from_pickle(r#type: *mut _typeobject, state: *mut _object) -> *mut _object {
    let me: *mut __SlateRecord_datetime_h_98 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_datetime_h_98;
    if !me.is_null() {
        let pdata: *const i8 = PyBytes_AS_STRING!(state) as *const i8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((pdata as *const ()) as *const core::ffi::c_void) as *const u8,
                ((((*me).data.as_mut_ptr() as *mut u8) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                (4u64 as usize) as usize,
            )
        };
        unsafe {
            (*me).hashcode = 1i32.wrapping_neg() as i64;
        }
    }
    me as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_new")]
extern "C-unwind" fn date_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut self_: *mut _object = core::ptr::null_mut::<_object>();
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    // Check for invocation from pickle with __getstate__ state
    if PyTuple_GET_SIZE!(args) == 1 {
        let mut state: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
        if PyBytes_Check!(state) != 0 {
            let __v0: bool = PyBytes_GET_SIZE!(state) == 4
                && MONTH_IS_SANE!(unsafe {
                    *PyBytes_AS_STRING!(state).wrapping_offset(2 as isize)
                });
            if __v0 {
                return date_from_pickle(r#type, state);
            }
        } else {
            if PyUnicode_Check!(state) != 0 {
                let __v1: bool = PyUnicode_GET_LENGTH!(state) == 4
                    && MONTH_IS_SANE!(PyUnicode_READ_CHAR__2!(state, 2i32));
                if __v1 {
                    state = unsafe { PyUnicode_AsLatin1String(state) };
                    if state.is_null() {
                        if (unsafe { PyErr_ExceptionMatches(PyExc_UnicodeEncodeError) }) != 0 {
                            // More informative error message.
                            unsafe {
                                PyErr_SetString(PyExc_ValueError, c"Failed to encode latin1 string when unpickling a date object. pickle.load(data, encoding='latin1') is assumed.".as_ptr())
                            };
                        }
                        return core::ptr::null_mut::<_object>();
                    }
                    self_ = date_from_pickle(r#type, state);
                    Py_DECREF!(state);
                    return self_;
                }
            }
        }
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"iii".as_ptr(),
            (core::ptr::addr_of_mut!(date_kws.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(year),
            core::ptr::addr_of_mut!(month),
            core::ptr::addr_of_mut!(day),
        )
    }) != 0
    {
        self_ = new_date_ex(year, month, day, r#type);
    }
    self_
}

fn date_fromtimestamp(cls: *mut _object, obj: *mut _object) -> *mut _object {
    let mut tm: slate_support::tm = unsafe { core::mem::zeroed() };
    let mut t: i64 = 0;
    if (unsafe { _PyTime_ObjectToTime_t(obj, core::ptr::addr_of_mut!(t), 0u32) })
        == 1i32.wrapping_neg()
    {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { _PyTime_localtime(t, core::ptr::addr_of_mut!(tm)) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    new_date_subclass_ex(
        tm.tm_year.wrapping_add(1900),
        tm.tm_mon.wrapping_add(1),
        tm.tm_mday,
        cls,
    )
}

/// Return new date from current time.
/// We say this is equivalent to fromtimestamp(time.time()), and the
/// only way to be sure of that is to *call* time.time().  That's not
/// generally the same as calling C's time.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_today")]
extern "C-unwind" fn date_today(cls: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let time: *mut _object = time_time();
    if time.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // Note well:  today() is a class method, so this may not call
    // date.fromtimestamp.  For example, it may call
    // datetime.fromtimestamp.  That's why we need all the accuracy
    // time.time() delivers; if someone were gonzo about optimization,
    // date.today() could get away with plain C time().
    let result: *mut _object = PyObject_CallMethodOneArg(
        cls,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_fromtimestamp
                    ._ascii
                    .ob_base
            )
        },
        time,
    );
    Py_DECREF!(time);
    result
}

// [clinic input]
// @classmethod
// datetime.date.fromtimestamp
//
//     timestamp: object
//     /
//
// Create a date from a POSIX timestamp.
//
// The timestamp is a number, e.g. created via time.time(), that is
// interpreted as local time.
// [clinic start generated code]
/// [clinic end generated code: output=59def4e32c028fb6 input=15720eef43b169a1]
fn datetime_date_fromtimestamp_impl(
    r#type: *mut _typeobject,
    timestamp: *mut _object,
) -> *mut _object {
    date_fromtimestamp(r#type as *mut _object, timestamp)
}

/// bpo-36025: This is a wrapper for API compatibility with the public C API,
/// which expects a function that takes an *args tuple, whereas the argument
/// clinic generates code that takes METH_O.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_date_fromtimestamp_capi")]
extern "C-unwind" fn datetime_date_fromtimestamp_capi(
    cls: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut timestamp: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyArg_UnpackTuple(
            args,
            c"fromtimestamp".as_ptr(),
            1i64,
            1i64,
            core::ptr::addr_of_mut!(timestamp),
        )
    }) != 0
    {
        result = date_fromtimestamp(cls, timestamp);
    }
    result
}

/// Return new date from proleptic Gregorian ordinal.  Raises ValueError if
/// the ordinal is out of range.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_fromordinal")]
extern "C-unwind" fn date_fromordinal(cls: *mut _object, args: *mut _object) -> *mut _object {
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    let mut ordinal: i32 = 0;
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"i:fromordinal".as_ptr(),
            core::ptr::addr_of_mut!(ordinal),
        )
    }) != 0
    {
        let mut year: i32 = 0;
        let mut month: i32 = 0;
        let mut day: i32 = 0;
        if ordinal < 1 {
            unsafe { PyErr_SetString(PyExc_ValueError, c"ordinal must be >= 1".as_ptr()) };
        } else {
            ord_to_ymd(
                ordinal,
                core::ptr::addr_of_mut!(year),
                core::ptr::addr_of_mut!(month),
                core::ptr::addr_of_mut!(day),
            );
            result = new_date_subclass_ex(year, month, day, cls);
        }
    }
    result
}

/// Return the new date from a string as generated by date.isoformat()
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_fromisoformat")]
extern "C-unwind" fn date_fromisoformat(cls: *mut _object, dtstr: *mut _object) -> *mut _object {
    let mut rv: i32 = unsafe { core::mem::zeroed() };
    let mut day: i32 = unsafe { core::mem::zeroed() };
    let mut month: i32 = unsafe { core::mem::zeroed() };
    let mut year: i32 = unsafe { core::mem::zeroed() };
    let mut dt_ptr: *const i8 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        if PyUnicode_Check!(dtstr) == 0 {
            PyErr_SetString(
                PyExc_TypeError,
                c"fromisoformat: argument must be str".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            dt_ptr = PyUnicode_AsUTF8AndSize(dtstr, core::ptr::addr_of_mut!(len));
            if !dt_ptr.is_null() {
                year = 0;
                month = 0;
                day = 0;
                if len == 7 || len == 8 || len == 10 {
                    rv = parse_isoformat_date(
                        dt_ptr,
                        len as u64,
                        core::ptr::addr_of_mut!(year),
                        core::ptr::addr_of_mut!(month),
                        core::ptr::addr_of_mut!(day),
                    );
                } else {
                    rv = 1i32.wrapping_neg();
                }
                if rv >= 0 {
                    return new_date_subclass_ex(year, month, day, cls);
                }
            }
            PyErr_Format(
                PyExc_ValueError,
                c"Invalid isoformat string: %R".as_ptr(),
                dtstr,
            );
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_fromisocalendar")]
extern "C-unwind" fn date_fromisocalendar(
    cls: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut year: i32 = 0;
    let mut week: i32 = 0;
    let mut day: i32 = 0;
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"iii:fromisocalendar".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__3428.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(year),
            core::ptr::addr_of_mut!(week),
            core::ptr::addr_of_mut!(day),
        )
    }) == 0
    {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"ISO calendar component out of range".as_ptr(),
                )
            };
        }
        return core::ptr::null_mut::<_object>();
    }
    let mut month: i32 = 0;
    let rv: i32 = iso_to_ymd(
        year,
        week,
        day,
        core::ptr::addr_of_mut!(year),
        core::ptr::addr_of_mut!(month),
        core::ptr::addr_of_mut!(day),
    );
    if rv == 4i32.wrapping_neg() {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"year must be in %d..%d, not %d".as_ptr(),
                1i32,
                9999i32,
                year,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if rv == 2i32.wrapping_neg() {
        unsafe { PyErr_Format(PyExc_ValueError, c"Invalid week: %d".as_ptr(), week) };
        return core::ptr::null_mut::<_object>();
    }
    if rv == 3i32.wrapping_neg() {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Invalid weekday: %d (range is [1, 7])".as_ptr(),
                day,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    new_date_subclass_ex(year, month, day, cls)
}

static mut keywords__3428: __SlateAlign16<[*mut i8; 4]> = __SlateAlign16([
    b"year\0".as_ptr() as *mut i8,
    b"week\0".as_ptr() as *mut i8,
    b"day\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

/// Return new date from _strptime.strptime_datetime_date().
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_strptime")]
extern "C-unwind" fn date_strptime(cls: *mut _object, args: *mut _object) -> *mut _object {
    let mut string: *mut _object = unsafe { core::mem::zeroed() };
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UU:strptime".as_ptr(),
            core::ptr::addr_of_mut!(string),
            core::ptr::addr_of_mut!(format),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let module: *mut _object = unsafe {
        PyImport_Import(core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py__strptime
                ._ascii
                .ob_base
        ))
    };
    if module.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe {
        PyObject_CallMethodObjArgs(
            module,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py__strptime_datetime_date
                    ._ascii
                    .ob_base
            ),
            cls,
            string,
            format,
            core::ptr::null_mut::<()>(),
        )
    };
    Py_DECREF!(module);
    result
}

// Date arithmetic.
/// date + timedelta -> date.  If arg negate is true, subtract the timedelta
/// instead.
fn add_date_timedelta(
    date: *mut __SlateRecord_datetime_h_98,
    delta: *mut __SlateRecord_datetime_h_41,
    negate: i32,
) -> *mut _object {
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    let mut year: i32 =
        ((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) }) as i32)
            << 8i32
            | ((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) })
                as i32);
    let mut month: i32 =
        (unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) }) as i32;
    let deltadays: i32 = GET_TD_DAYS__2!(delta);
    // C-level overflow is impossible because |deltadays| < 1e9.
    let mut day: i32 =
        ((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) }) as i32)
            .wrapping_add(if negate != 0 {
                deltadays.wrapping_neg()
            } else {
                deltadays
            });
    if normalize_date(
        core::ptr::addr_of_mut!(year),
        core::ptr::addr_of_mut!(month),
        core::ptr::addr_of_mut!(day),
    ) >= 0
    {
        result = new_date_subclass_ex(year, month, day, Py_TYPE__2!(date) as *mut _object);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_add")]
extern "C-unwind" fn date_add(left: *mut _object, right: *mut _object) -> *mut _object {
    let __v0: bool = PyDateTime_Check!(left) != 0 || PyDateTime_Check!(right) != 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    if PyDate_Check!(left) != 0 {
        // date + ???
        if PyDelta_Check!(right) != 0 {
            return add_date_timedelta(
                left as *mut __SlateRecord_datetime_h_98,
                right as *mut __SlateRecord_datetime_h_41,
                0i32,
            );
        }
    // date + delta
    } else {
        // ??? + date
        // 'right' must be one of us, or we wouldn't have been called
        if PyDelta_Check!(left) != 0 {
            return add_date_timedelta(
                right as *mut __SlateRecord_datetime_h_98,
                left as *mut __SlateRecord_datetime_h_41,
                0i32,
            );
        }
        // delta + date
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_subtract")]
extern "C-unwind" fn date_subtract(left: *mut _object, right: *mut _object) -> *mut _object {
    let __v0: bool = PyDateTime_Check!(left) != 0 || PyDateTime_Check!(right) != 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    if PyDate_Check!(left) != 0 {
        if PyDate_Check!(right) != 0 {
            // date - date
            let left_ord: i32 = ymd_to_ord(
                ((unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize)
                }) as i32)
                    << 8i32
                    | ((unsafe {
                        *((*(left as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize)
                    }) as i32),
                (unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize)
                }) as i32,
                (unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize)
                }) as i32,
            );
            let right_ord: i32 = ymd_to_ord(
                ((unsafe {
                    *((*(right as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize)
                }) as i32)
                    << 8i32
                    | ((unsafe {
                        *((*(right as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize)
                    }) as i32),
                (unsafe {
                    *((*(right as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize)
                }) as i32,
                (unsafe {
                    *((*(right as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize)
                }) as i32,
            );
            return new_delta!(left_ord.wrapping_sub(right_ord), 0i32, 0i32, 0i32);
        }
        if PyDelta_Check!(right) != 0 {
            // date - delta
            return add_date_timedelta(
                left as *mut __SlateRecord_datetime_h_98,
                right as *mut __SlateRecord_datetime_h_41,
                1i32,
            );
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) }
}

// Various ways to turn a date into a string.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_repr")]
extern "C-unwind" fn date_repr(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    unsafe {
        PyUnicode_FromFormat(
            c"%s(%d, %d, %d)".as_ptr(),
            (*Py_TYPE__2!(self_)).tp_name,
            (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as i32) << 8i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) as i32),
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) as i32,
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) as i32,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_isoformat")]
extern "C-unwind" fn date_isoformat(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    unsafe {
        PyUnicode_FromFormat(
            c"%04d-%02d-%02d".as_ptr(),
            (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as i32) << 8i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) as i32),
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) as i32,
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) as i32,
        )
    }
}

/// str() calls the appropriate isoformat() method.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_str")]
extern "C-unwind" fn date_str(self_: *mut _object) -> *mut _object {
    PyObject_CallMethodNoArgs(self_, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_isoformat
                ._ascii
                .ob_base
        )
    })
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_ctime")]
extern "C-unwind" fn date_ctime(self_: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    format_ctime(self_, 0i32, 0i32, 0i32)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_strftime")]
extern "C-unwind" fn date_strftime(
    self_: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"U:strftime".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__3609.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(format),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let tuple: *mut _object = PyObject_CallMethodNoArgs(self_, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_timetuple
                ._ascii
                .ob_base
        )
    });
    if tuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // This method can be inherited, and needs to call the
    // timetuple() method appropriate to self's class.
    let result: *mut _object = wrap_strftime(self_, format, tuple, self_);
    Py_DECREF!(tuple);
    result
}

static mut keywords__3609: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"format\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_format")]
extern "C-unwind" fn date_format(self_: *mut _object, args: *mut _object) -> *mut _object {
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"U:__format__".as_ptr(),
            core::ptr::addr_of_mut!(format),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    // if the format is zero length, return str(self)
    if (unsafe { PyUnicode_GetLength(format) }) == 0 {
        return unsafe { PyObject_Str(self_) };
    }
    PyObject_CallMethodOneArg(
        self_,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_strftime
                    ._ascii
                    .ob_base
            )
        },
        format,
    )
}

// ISO methods.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_isoweekday")]
extern "C-unwind" fn date_isoweekday(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let dow: i32 = weekday(
        ((unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
    );
    unsafe { PyLong_FromLong(dow.wrapping_add(1) as i64) }
}

static iso_calendar_date__doc__: __SlateAlign16<[i8; 192]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 32i8, 111i8, 102i8, 32i8,
    100i8, 97i8, 116i8, 101i8, 46i8, 105i8, 115i8, 111i8, 99i8, 97i8, 108i8, 101i8, 110i8, 100i8,
    97i8, 114i8, 40i8, 41i8, 32i8, 111i8, 114i8, 32i8, 100i8, 97i8, 116i8, 101i8, 116i8, 105i8,
    109i8, 101i8, 46i8, 105i8, 115i8, 111i8, 99i8, 97i8, 108i8, 101i8, 110i8, 100i8, 97i8, 114i8,
    40i8, 41i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 97i8, 99i8, 99i8, 101i8, 115i8,
    115i8, 101i8, 100i8, 32i8, 101i8, 105i8, 116i8, 104i8, 101i8, 114i8, 32i8, 97i8, 115i8, 32i8,
    97i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8, 102i8, 10i8, 32i8, 32i8, 40i8,
    40i8, 121i8, 101i8, 97i8, 114i8, 44i8, 32i8, 119i8, 101i8, 101i8, 107i8, 44i8, 32i8, 119i8,
    101i8, 101i8, 107i8, 100i8, 97i8, 121i8, 41i8, 10i8, 111i8, 114i8, 32i8, 118i8, 105i8, 97i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 97i8, 116i8,
    116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 32i8, 97i8, 115i8, 32i8, 110i8, 97i8,
    109i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 98i8, 111i8,
    118i8, 101i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 46i8, 0i8,
]);

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__datetimemodule_c_3656 {
    tuple: __SlateRecord_tupleobject_h_13,
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_repr")]
extern "C-unwind" fn iso_calendar_date_repr(self_: *mut _object) -> *mut _object {
    let year: *mut _object = unsafe { PyTuple_GetItem(self_, 0i64) };
    if year.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let week: *mut _object = unsafe { PyTuple_GetItem(self_, 1i64) };
    if week.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let weekday: *mut _object = unsafe { PyTuple_GetItem(self_, 2i64) };
    if weekday.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        PyUnicode_FromFormat(
            c"%.200s(year=%S, week=%S, weekday=%S)".as_ptr(),
            (*Py_TYPE!(self_)).tp_name,
            year,
            week,
            weekday,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_reduce")]
extern "C-unwind" fn iso_calendar_date_reduce(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    // Construct the tuple that this reduces to
    let reduce_tuple: *mut _object = unsafe {
        Py_BuildValue(
            c"O((OOO))".as_ptr(),
            core::ptr::addr_of_mut!(PyTuple_Type),
            PyTuple_GET_ITEM!(self_, 0i32),
            PyTuple_GET_ITEM!(self_, 1i32),
            PyTuple_GET_ITEM!(self_, 2i32),
        )
    };
    reduce_tuple
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_year")]
extern "C-unwind" fn iso_calendar_date_year(
    self_: *mut _object,
    _unused_closure: *mut (),
) -> *mut _object {
    let year: *mut _object = unsafe { PyTuple_GetItem(self_, 0i64) };
    if year.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(year)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_week")]
extern "C-unwind" fn iso_calendar_date_week(
    self_: *mut _object,
    _unused_closure: *mut (),
) -> *mut _object {
    let week: *mut _object = unsafe { PyTuple_GetItem(self_, 1i64) };
    if week.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(week)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_weekday")]
extern "C-unwind" fn iso_calendar_date_weekday(
    self_: *mut _object,
    _unused_closure: *mut (),
) -> *mut _object {
    let weekday: *mut _object = unsafe { PyTuple_GetItem(self_, 2i64) };
    if weekday.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(weekday)
}

static mut iso_calendar_date_getset: __SlateAlign16<[PyGetSetDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"year\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(iso_calendar_date_year);
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"week\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(iso_calendar_date_week);
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"weekday\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(iso_calendar_date_weekday);
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut iso_calendar_date_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(iso_calendar_date_reduce);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = (b"__reduce__() -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_traverse")]
extern "C-unwind" fn iso_calendar_date_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    Py_VISIT__2!(_Py_TYPE(self_), visit, arg);
    unsafe { unsafe { PyTuple_Type.tp_traverse }.unwrap()(self_, visit, arg) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.iso_calendar_date_dealloc")]
extern "C-unwind" fn iso_calendar_date_dealloc(self_: *mut _object) {
    let tp: *mut _typeobject = Py_TYPE!(self_);
    unsafe { unsafe { PyTuple_Type.tp_dealloc }.unwrap()(self_) }; // delegate GC-untrack as well
    Py_DECREF__2!(tp);
}

static mut isocal_slots: __SlateAlign16<[__SlateRecord_object_h_377; 8]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 66i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(Some(iso_calendar_date_repr))
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 56i32;
        __t1.pfunc =
            (unsafe { core::ptr::addr_of!(iso_calendar_date__doc__.0) as *const i8 }) as *mut ();
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 64i32;
        __t2.pfunc =
            (unsafe { core::ptr::addr_of_mut!(iso_calendar_date_methods.0) as *mut PyMethodDef })
                as *mut ();
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 73i32;
        __t3.pfunc =
            (unsafe { core::ptr::addr_of_mut!(iso_calendar_date_getset.0) as *mut PyGetSetDef })
                as *mut ();
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 65i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(iso_calendar_date_new))
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 52i32;
        __t5.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(iso_calendar_date_dealloc),
            )
        };
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 71i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                        *mut (),
                    ) -> i32,
                >,
                *mut (),
            >(Some(iso_calendar_date_traverse))
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

static mut isocal_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"datetime.IsoCalendarDate\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord__datetimemodule_c_3656>() as u64) as i32;
    __t0.flags = 16640u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(isocal_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// [clinic input]
// @classmethod
// datetime.IsoCalendarDate.__new__ as iso_calendar_date_new
//     year: int
//     week: int
//     weekday: int
// [clinic start generated code]
/// [clinic end generated code: output=383d33d8dc7183a2 input=4f2c663c9d19c4ee]
fn iso_calendar_date_new_impl(
    r#type: *mut _typeobject,
    year: i32,
    week: i32,
    weekday: i32,
) -> *mut _object {
    let self_: *mut __SlateRecord__datetimemodule_c_3656 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 3i64) })
            as *mut __SlateRecord__datetimemodule_c_3656;
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    PyTuple_SET_ITEM__3!(self_, 0i32, unsafe { PyLong_FromLong(year as i64) });
    PyTuple_SET_ITEM__3!(self_, 1i32, unsafe { PyLong_FromLong(week as i64) });
    PyTuple_SET_ITEM__3!(self_, 2i32, unsafe { PyLong_FromLong(weekday as i64) });
    self_ as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_isocalendar")]
extern "C-unwind" fn date_isocalendar(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let mut year: i32 = ((unsafe {
        *((*(self_ as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(0 as isize)
    }) as i32)
        << 8i32
        | ((unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(1 as isize)
        }) as i32);
    let mut week1_monday: i32 = iso_week1_monday(year);
    let today: i32 = ymd_to_ord(
        year,
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
    );
    let mut week: i32 = 0;
    let mut day: i32 = 0;
    week = divmod(
        today.wrapping_sub(week1_monday),
        7i32,
        core::ptr::addr_of_mut!(day),
    );
    if week < 0 {
        year = year.wrapping_sub(1);
        week1_monday = iso_week1_monday(year);
        week = divmod(
            today.wrapping_sub(week1_monday),
            7i32,
            core::ptr::addr_of_mut!(day),
        );
    } else {
        let __v2: bool = week >= 52 && today >= iso_week1_monday(year.wrapping_add(1));
        if __v2 {
            year = year.wrapping_add(1);
            week = 0;
        }
    }
    let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
    let st: *mut __SlateRecord__datetimemodule_c_54 = GET_CURRENT_STATE!(current_mod);
    if st.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let v: *mut _object = iso_calendar_date_new_impl(
        ISOCALENDAR_DATE_TYPE!(st),
        year,
        week.wrapping_add(1),
        day.wrapping_add(1),
    );
    RELEASE_CURRENT_STATE!(current_mod);
    if v.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    v
}

// Miscellaneous methods.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_richcompare")]
extern "C-unwind" fn date_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    // Since DateTime is a subclass of Date, if the other object is
    // a DateTime, it would compute an equality testing or an ordering
    // based on the date part alone, and we don't want that.
    // So return NotImplemented here in that case.
    // If a subclass wants to change this, it's up to the subclass to do so.
    // The behavior is the same as if Date and DateTime were independent
    // classes.
    let __v0: bool = PyDate_Check!(other) != 0 && PyDateTime_Check!(other) == 0;
    if __v0 {
        let diff: i32 = unsafe {
            memcmp(
                (((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8) as *const ())
                    as *const core::ffi::c_void,
                (((*(other as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8) as *const ())
                    as *const core::ffi::c_void,
                4u64 as usize,
            ) as i32
        };
        return diff_to_bool(diff, op);
    } else {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_timetuple")]
extern "C-unwind" fn date_timetuple(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    build_struct_time(
        ((unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
        0i32,
        0i32,
        0i32,
        1i32.wrapping_neg(),
    )
}

// [clinic input]
// datetime.date.replace
//
//     year: int(c_default="GET_YEAR(self)") = unchanged
//     month: int(c_default="GET_MONTH(self)") = unchanged
//     day: int(c_default="GET_DAY(self)") = unchanged
//
// Return date with new specified fields.
// [clinic start generated code]
/// [clinic end generated code: output=2a9430d1e6318aeb input=0d1f02685b3e90f6]
fn datetime_date_replace_impl(
    self_: *mut __SlateRecord_datetime_h_98,
    year: i32,
    month: i32,
    day: i32,
) -> *mut _object {
    new_date_subclass_ex(year, month, day, Py_TYPE__2!(self_) as *mut _object)
}

fn generic_hash(data: *mut u8, len: i32) -> i64 {
    unsafe { Py_HashBuffer(data as *const (), len as i64) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_hash")]
extern "C-unwind" fn date_hash(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    let mut hash: i64 = unsafe { (*self_).hashcode };
    if hash == (1i32.wrapping_neg() as i64) {
        hash = generic_hash(unsafe { (*self_).data.as_mut_ptr() as *mut u8 }, 4i32);
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).hashcode, hash);
    }
    hash
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_toordinal")]
extern "C-unwind" fn date_toordinal(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    unsafe {
        PyLong_FromLong(ymd_to_ord(
            (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize) as i32)
                << 8i32
                | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize) as i32),
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize) as i32,
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize) as i32,
        ) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_weekday")]
extern "C-unwind" fn date_weekday(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let dow: i32 = weekday(
        ((unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
    );
    unsafe { PyLong_FromLong(dow as i64) }
}

// Pickle support, a simple use of __reduce__.
/// __getstate__ isn't exposed
fn date_getstate(self_: *mut __SlateRecord_datetime_h_98) -> *mut _object {
    let field: *mut _object = unsafe {
        PyBytes_FromStringAndSize(
            (((*self_).data.as_mut_ptr() as *mut u8) as *mut i8) as *const i8,
            4i64,
        )
    };
    unsafe { Py_BuildValue(c"(N)".as_ptr(), field) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.date_reduce")]
extern "C-unwind" fn date_reduce(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_98 = PyDate_CAST!(op);
    unsafe { Py_BuildValue(c"(ON)".as_ptr(), Py_TYPE__2!(self_), date_getstate(self_)) }
}

/// Class methods:
/// Instance methods:
static mut date_methods: __SlateAlign16<[PyMethodDef; 19]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"fromtimestamp\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(datetime_date_fromtimestamp);
        __t0.ml_flags = 24i32;
        __t0.ml_doc =
            unsafe { core::ptr::addr_of!(datetime_date_fromtimestamp__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"fromordinal\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(date_fromordinal);
        __t1.ml_flags = 17i32;
        __t1.ml_doc = (b"int -> date corresponding to a proleptic Gregorian ordinal.\0".as_ptr()
            as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"fromisoformat\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(date_fromisoformat);
        __t2.ml_flags = 24i32;
        __t2.ml_doc = (b"str -> Construct a date from a string in ISO 8601 format.\0".as_ptr()
            as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"fromisocalendar\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = unsafe {
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
                >(Some(date_fromisocalendar))
            })
        };
        __t3.ml_flags = 19i32;
        __t3.ml_doc = (b"int, int, int -> Construct a date from the ISO year, week number and weekday.\n\nThis is the inverse of the date.isocalendar() function\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"strptime\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(date_strptime);
        __t4.ml_flags = 17i32;
        __t4.ml_doc = (b"string, format -> new date parsed from a string (like time.strptime()).\0"
            .as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"today\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(date_today);
        __t5.ml_flags = 20i32;
        __t5.ml_doc =
            (b"Current date or datetime:  same as self.__class__.fromtimestamp(time.time()).\0"
                .as_ptr() as *mut i8) as *const i8;
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"ctime\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(date_ctime);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = (b"Return ctime() style string.\0".as_ptr() as *mut i8) as *const i8;
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"strftime\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = unsafe {
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
                >(Some(date_strftime))
            })
        };
        __t7.ml_flags = 3i32;
        __t7.ml_doc = (b"format -> strftime() style string.\0".as_ptr() as *mut i8) as *const i8;
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"__format__\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(date_format);
        __t8.ml_flags = 1i32;
        __t8.ml_doc = (b"Formats self with strftime.\0".as_ptr() as *mut i8) as *const i8;
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"timetuple\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(date_timetuple);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = (b"Return time tuple, compatible with time.localtime().\0".as_ptr()
            as *mut i8) as *const i8;
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"isocalendar\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(date_isocalendar);
        __t10.ml_flags = 4i32;
        __t10.ml_doc = (b"Return a named tuple containing ISO year, week number, and weekday.\0"
            .as_ptr() as *mut i8) as *const i8;
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"isoformat\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(date_isoformat);
        __t11.ml_flags = 4i32;
        __t11.ml_doc =
            (b"Return string in ISO 8601 format, YYYY-MM-DD.\0".as_ptr() as *mut i8) as *const i8;
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"isoweekday\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(date_isoweekday);
        __t12.ml_flags = 4i32;
        __t12.ml_doc =
            (b"Return the day of the week represented by the date.\nMonday == 1 ... Sunday == 7\0"
                .as_ptr() as *mut i8) as *const i8;
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"toordinal\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(date_toordinal);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = (b"Return proleptic Gregorian ordinal.  January 1 of year 1 is day 1.\0"
            .as_ptr() as *mut i8) as *const i8;
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"weekday\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = Some(date_weekday);
        __t14.ml_flags = 4i32;
        __t14.ml_doc =
            (b"Return the day of the week represented by the date.\nMonday == 0 ... Sunday == 6\0"
                .as_ptr() as *mut i8) as *const i8;
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"replace\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(datetime_date_replace))
            })
        };
        __t15.ml_flags = 130i32;
        __t15.ml_doc = unsafe { core::ptr::addr_of!(datetime_date_replace__doc__.0) as *const i8 };
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"__replace__\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(datetime_date_replace))
            })
        };
        __t16.ml_flags = 130i32;
        __t16.ml_doc = (b"__replace__($self, /, **changes)\n--\n\nThe same as replace().\0".as_ptr()
            as *mut i8) as *const i8;
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t17.ml_meth = Some(date_reduce);
        __t17.ml_flags = 4i32;
        __t17.ml_doc = (b"__reduce__() -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = core::ptr::null::<i8>();
        __t18.ml_meth = None;
        __t18
    },
]);

static date_doc: __SlateAlign16<[i8; 39]> = __SlateAlign16([
    100i8, 97i8, 116i8, 101i8, 40i8, 121i8, 101i8, 97i8, 114i8, 44i8, 32i8, 109i8, 111i8, 110i8,
    116i8, 104i8, 44i8, 32i8, 100i8, 97i8, 121i8, 41i8, 32i8, 45i8, 45i8, 62i8, 32i8, 100i8, 97i8,
    116i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 0i8,
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
static mut date_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = Some(date_add);
    __t0.nb_subtract = Some(date_subtract);
    __t0.nb_multiply = None;
    __t0.nb_remainder = None;
    __t0.nb_divmod = None;
    __t0.nb_power = None;
    __t0.nb_negative = None;
    __t0.nb_positive = None;
    __t0.nb_absolute = None;
    __t0.nb_bool = None;
    __t0
};

// PyDateTime_TZInfo implementation.
// This is a pure abstract base class, so doesn't do anything beyond
// raising NotImplemented exceptions.  Real tzinfo classes need
// to derive from this.  This is mostly for clarity, and for efficiency in
// datetime and time constructors (their tzinfo arguments need to
// be subclasses of this tzinfo class, which is easy and quick to check).
//
// Note:  For reasons having to do with pickling of subclasses, we have
// to allow tzinfo objects to be instantiated.  This wasn't an issue
// in the Python implementation (__init__() could raise NotImplementedError
// there without ill effect), but doing so in the C implementation hit a
// brick wall.
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
fn tzinfo_nogo(methodname: *const i8) -> *mut _object {
    unsafe {
        PyErr_Format(
            PyExc_NotImplementedError,
            c"a tzinfo subclass must implement %s()".as_ptr(),
            methodname,
        )
    };
    core::ptr::null_mut::<_object>()
}

// Methods.  A subclass must implement these.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.tzinfo_tzname")]
extern "C-unwind" fn tzinfo_tzname(
    _unused_self: *mut _object,
    _unused_dt: *mut _object,
) -> *mut _object {
    tzinfo_nogo(c"tzname".as_ptr())
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.tzinfo_utcoffset")]
extern "C-unwind" fn tzinfo_utcoffset(
    _unused_self: *mut _object,
    _unused_dt: *mut _object,
) -> *mut _object {
    tzinfo_nogo(c"utcoffset".as_ptr())
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.tzinfo_dst")]
extern "C-unwind" fn tzinfo_dst(
    _unused_self: *mut _object,
    _unused_dt: *mut _object,
) -> *mut _object {
    tzinfo_nogo(c"dst".as_ptr())
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.tzinfo_fromutc")]
extern "C-unwind" fn tzinfo_fromutc(self_: *mut _object, dt: *mut _object) -> *mut _object {
    let mut delta: *mut __SlateRecord_datetime_h_41 = unsafe { core::mem::zeroed() };
    let mut dst: *mut _object = unsafe { core::mem::zeroed() };
    let mut off: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        off = core::ptr::null_mut::<_object>();
        dst = core::ptr::null_mut::<_object>();
        delta = core::ptr::null_mut::<__SlateRecord_datetime_h_41>();
        if PyDateTime_Check!(dt) == 0 {
            PyErr_SetString(
                PyExc_TypeError,
                c"fromutc: argument must be a datetime".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            if (if _PyDateTime_HAS_TZINFO!(dt) != 0 {
                (*(dt as *mut __SlateRecord_datetime_h_114)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            }) != self_
            {
                PyErr_SetString(PyExc_ValueError, c"fromutc: dt.tzinfo is not self".as_ptr());
                return core::ptr::null_mut::<_object>();
            } else {
                off = datetime_utcoffset(dt, core::ptr::null_mut::<_object>());
                if off.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    '__join_0: {
                        if off == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"fromutc: non-None utcoffset() result required".as_ptr(),
                            );
                        } else {
                            dst = datetime_dst(dt, core::ptr::null_mut::<_object>());
                            if !dst.is_null() {
                                if dst == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                    PyErr_SetString(
                                        PyExc_ValueError,
                                        c"fromutc: non-None dst() result required".as_ptr(),
                                    );
                                } else {
                                    delta = delta_subtract(off, dst)
                                        as *mut __SlateRecord_datetime_h_41;
                                    if !delta.is_null() {
                                        result = add_datetime_timedelta(
                                            dt as *mut __SlateRecord_datetime_h_114,
                                            delta,
                                            1i32,
                                        );
                                        if !result.is_null() {
                                            Py_DECREF!(dst);
                                            dst = call_dst(
                                                if _PyDateTime_HAS_TZINFO!(dt) != 0 {
                                                    (*(dt as *mut __SlateRecord_datetime_h_114))
                                                        .tzinfo
                                                } else {
                                                    core::ptr::addr_of_mut!(_Py_NoneStruct)
                                                },
                                                result,
                                            );
                                            if !dst.is_null() {
                                                if dst == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                                    PyErr_SetString(PyExc_ValueError, c"fromutc: tz.dst() gave inconsistent results; cannot convert".as_ptr());
                                                // fall through to failure
                                                } else {
                                                    if delta_bool(dst) != 0 {
                                                        Py_SETREF!(result, add_datetime_timedelta(result as *mut __SlateRecord_datetime_h_114, dst as *mut __SlateRecord_datetime_h_41, 1i32));
                                                        if result.is_null() {
                                                            break '__join_0;
                                                        }
                                                    }
                                                    Py_DECREF__2!(delta);
                                                    Py_DECREF!(dst);
                                                    Py_DECREF!(off);
                                                    return result;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Py_XDECREF!(off);
                    Py_XDECREF!(dst);
                    Py_XDECREF__2!(delta);
                    Py_XDECREF!(result);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
}

// Pickle support.  This is solely so that tzinfo subclasses can use
// pickling -- tzinfo itself is supposed to be uninstantiable.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.tzinfo_reduce")]
extern "C-unwind" fn tzinfo_reduce(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let mut args: *mut _object = unsafe { core::mem::zeroed() };
    let mut getinitargs: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyObject_GetOptionalAttr(
            self_,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___getinitargs__
                    ._ascii
                    .ob_base
            ),
            core::ptr::addr_of_mut!(getinitargs),
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if !getinitargs.is_null() {
        args = unsafe { PyObject_CallNoArgs(getinitargs) };
        Py_DECREF!(getinitargs);
    } else {
        args = unsafe { PyTuple_New(0i64) };
    }
    if args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let state: *mut _object = unsafe { _PyObject_GetState(self_) };
    if state.is_null() {
        Py_DECREF!(args);
        return core::ptr::null_mut::<_object>();
    }
    unsafe { Py_BuildValue(c"(ONN)".as_ptr(), Py_TYPE!(self_), args, state) }
}

static mut tzinfo_methods: __SlateAlign16<[PyMethodDef; 6]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"tzname\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(tzinfo_tzname);
        __t0.ml_flags = 8i32;
        __t0.ml_doc = (b"datetime -> string name of time zone.\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"utcoffset\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(tzinfo_utcoffset);
        __t1.ml_flags = 8i32;
        __t1.ml_doc = (b"datetime -> timedelta showing offset from UTC, negative values indicating West of UTC\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"dst\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(tzinfo_dst);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = (b"datetime -> DST offset as timedelta positive east of UTC.\0".as_ptr()
            as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"fromutc\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(tzinfo_fromutc);
        __t3.ml_flags = 8i32;
        __t3.ml_doc =
            (b"datetime in UTC -> datetime in local time.\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(tzinfo_reduce);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = (b"-> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = core::ptr::null::<i8>();
        __t5.ml_meth = None;
        __t5
    },
]);

static tzinfo_doc: __SlateAlign16<[i8; 48]> = __SlateAlign16([
    65i8, 98i8, 115i8, 116i8, 114i8, 97i8, 99i8, 116i8, 32i8, 98i8, 97i8, 115i8, 101i8, 32i8, 99i8,
    108i8, 97i8, 115i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 105i8, 109i8, 101i8, 32i8,
    122i8, 111i8, 110i8, 101i8, 32i8, 105i8, 110i8, 102i8, 111i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 115i8, 46i8, 0i8,
]);

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
static mut timezone_kws: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"offset\0".as_ptr() as *mut i8,
    b"name\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_new")]
extern "C-unwind" fn timezone_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut offset: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"O!|U:timezone".as_ptr(),
            (core::ptr::addr_of_mut!(timezone_kws.0) as *mut *mut i8) as *const *mut i8,
            DELTA_TYPE!(),
            core::ptr::addr_of_mut!(offset),
            core::ptr::addr_of_mut!(name),
        )
    }) != 0
    {
        return new_timezone(offset, name);
    }
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_dealloc")]
extern "C-unwind" fn timezone_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    Py_CLEAR!((*self_).offset);
    Py_CLEAR!((*self_).name);
    unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_richcompare")]
extern "C-unwind" fn timezone_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    if op != 2 && op != 3 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    if PyTimezone_Check!(other) == 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    let lhs: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(self_);
    let rhs: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(other);
    delta_richcompare(unsafe { (*lhs).offset }, unsafe { (*rhs).offset }, op)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_hash")]
extern "C-unwind" fn timezone_hash(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    delta_hash(unsafe { (*self_).offset })
}

/// Check argument type passed to tzname, utcoffset, or dst methods.
/// Returns 0 for good argument.  Returns -1 and sets exception info
/// otherwise.
fn _timezone_check_argument(dt: *mut _object, meth: *const i8) -> i32 {
    let __v0: bool =
        dt == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } || PyDateTime_Check!(dt) != 0;
    if __v0 {
        return 0;
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"%s(dt) argument must be a datetime instance or None, not %.200s".as_ptr(),
            meth,
            (*Py_TYPE!(dt)).tp_name,
        )
    };
    1i32.wrapping_neg()
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_repr")]
extern "C-unwind" fn timezone_repr(op: *mut _object) -> *mut _object {
    // Note that although timezone is not subclassable, it is convenient
    // to use Py_TYPE(self)->tp_name here.
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    let type_name: *const i8 = unsafe { (*Py_TYPE__2!(self_)).tp_name };
    if op == CONST_UTC!() {
        return unsafe { PyUnicode_FromFormat(c"%s.utc".as_ptr(), type_name) };
    }
    if unsafe { (*self_).name }.is_null() {
        return unsafe { PyUnicode_FromFormat(c"%s(%R)".as_ptr(), type_name, (*self_).offset) };
    }
    unsafe {
        PyUnicode_FromFormat(
            c"%s(%R, %R)".as_ptr(),
            type_name,
            (*self_).offset,
            (*self_).name,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_str")]
extern "C-unwind" fn timezone_str(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    let mut hours: i32 = 0;
    let mut minutes: i32 = 0;
    let mut seconds: i32 = 0;
    let mut microseconds: i32 = 0;
    let mut offset: *mut _object = unsafe { core::mem::zeroed() };
    let mut sign: i8 = 0;
    if !unsafe { (*self_).name }.is_null() {
        return Py_NewRef!(unsafe { (*self_).name });
    }
    if (self_ as *mut _object) == CONST_UTC!()
        || GET_TD_DAYS!(unsafe { (*self_).offset }) == 0
            && GET_TD_SECONDS!(unsafe { (*self_).offset }) == 0
            && GET_TD_MICROSECONDS!(unsafe { (*self_).offset }) == 0
    {
        return unsafe { PyUnicode_FromString(c"UTC".as_ptr()) };
    }
    // Offset is normalized, so it is negative if days < 0
    if GET_TD_DAYS!(unsafe { (*self_).offset }) < 0 {
        sign = 45;
        offset = delta_negative(unsafe { (*self_).offset });
        if offset.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        sign = 43;
        offset = Py_NewRef!(unsafe { (*self_).offset });
    }
    // Offset is not negative here.
    microseconds = GET_TD_MICROSECONDS!(offset);
    seconds = GET_TD_SECONDS!(offset);
    Py_DECREF!(offset);
    minutes = divmod(seconds, 60i32, core::ptr::addr_of_mut!(seconds));
    hours = divmod(minutes, 60i32, core::ptr::addr_of_mut!(minutes));
    if microseconds != 0 {
        return unsafe {
            PyUnicode_FromFormat(
                c"UTC%c%02d:%02d:%02d.%06d".as_ptr(),
                sign as i32,
                hours,
                minutes,
                seconds,
                microseconds,
            )
        };
    }
    if seconds != 0 {
        return unsafe {
            PyUnicode_FromFormat(
                c"UTC%c%02d:%02d:%02d".as_ptr(),
                sign as i32,
                hours,
                minutes,
                seconds,
            )
        };
    }
    unsafe { PyUnicode_FromFormat(c"UTC%c%02d:%02d".as_ptr(), sign as i32, hours, minutes) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_tzname")]
extern "C-unwind" fn timezone_tzname(op: *mut _object, dt: *mut _object) -> *mut _object {
    if _timezone_check_argument(dt, c"tzname".as_ptr()) == 1i32.wrapping_neg() {
        return core::ptr::null_mut::<_object>();
    }
    timezone_str(op)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_utcoffset")]
extern "C-unwind" fn timezone_utcoffset(op: *mut _object, dt: *mut _object) -> *mut _object {
    if _timezone_check_argument(dt, c"utcoffset".as_ptr()) == 1i32.wrapping_neg() {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    Py_NewRef!(unsafe { (*self_).offset })
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_dst")]
extern "C-unwind" fn timezone_dst(op: *mut _object, dt: *mut _object) -> *mut _object {
    if _timezone_check_argument(dt, c"dst".as_ptr()) == 1i32.wrapping_neg() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_fromutc")]
extern "C-unwind" fn timezone_fromutc(op: *mut _object, arg: *mut _object) -> *mut _object {
    if PyDateTime_Check!(arg) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"fromutc: argument must be a datetime".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let dt: *mut __SlateRecord_datetime_h_114 = arg as *mut __SlateRecord_datetime_h_114; // fast safe cast
    if (unsafe { (*(dt as *mut __SlateRecord_datetime_h_64)).hastzinfo }) == 0
        || (unsafe { (*dt).tzinfo }) != op
    {
        unsafe { PyErr_SetString(PyExc_ValueError, c"fromutc: dt.tzinfo is not self".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    add_datetime_timedelta(
        dt,
        (unsafe { (*self_).offset }) as *mut __SlateRecord_datetime_h_41,
        1i32,
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.timezone_getinitargs")]
extern "C-unwind" fn timezone_getinitargs(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord__datetimemodule_c_1414 = PyTimeZone_CAST!(op);
    if unsafe { (*self_).name }.is_null() {
        return unsafe { PyTuple_Pack(1i64, (*self_).offset) };
    }
    unsafe { PyTuple_Pack(2i64, (*self_).offset, (*self_).name) }
}

static mut timezone_methods: __SlateAlign16<[PyMethodDef; 6]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"tzname\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(timezone_tzname);
        __t0.ml_flags = 8i32;
        __t0.ml_doc = (b"If name is specified when timezone is created, returns the name.  Otherwise returns offset as 'UTC(+|-)HH:MM'.\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"utcoffset\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(timezone_utcoffset);
        __t1.ml_flags = 8i32;
        __t1.ml_doc = (b"Return fixed offset.\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"dst\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(timezone_dst);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = (b"Return None.\0".as_ptr() as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"fromutc\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(timezone_fromutc);
        __t3.ml_flags = 8i32;
        __t3.ml_doc =
            (b"datetime in UTC -> datetime in local time.\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"__getinitargs__\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(timezone_getinitargs);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = (b"pickle support\0".as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = core::ptr::null::<i8>();
        __t5.ml_meth = None;
        __t5
    },
]);

static timezone_doc: __SlateAlign16<[i8; 48]> = __SlateAlign16([
    70i8, 105i8, 120i8, 101i8, 100i8, 32i8, 111i8, 102i8, 102i8, 115i8, 101i8, 116i8, 32i8, 102i8,
    114i8, 111i8, 109i8, 32i8, 85i8, 84i8, 67i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 122i8,
    105i8, 110i8, 102i8, 111i8, 46i8, 0i8,
]);

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
/// XXX Can we make this const?
static mut utc_timezone: __SlateRecord__datetimemodule_c_1414 =
    __SlateRecord__datetimemodule_c_1414 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyDateTime_TimeZoneType) },
        },
        offset: (unsafe { core::ptr::addr_of_mut!(zero_delta) }) as *mut _object,
        name: core::ptr::null_mut::<_object>(),
    };

fn look_up_timezone(
    offset: *mut _object,
    name: *mut _object,
) -> *mut __SlateRecord__datetimemodule_c_1414 {
    if offset == unsafe { utc_timezone.offset } && name.is_null() {
        return CONST_UTC!() as *mut __SlateRecord__datetimemodule_c_1414;
    }
    core::ptr::null_mut::<__SlateRecord__datetimemodule_c_1414>()
}

// PyDateTime_Time implementation.
// Accessor properties.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_hour")]
extern "C-unwind" fn time_hour(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_minute")]
extern "C-unwind" fn time_minute(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) as i64)
    }
}

/// The name time_second conflicted with some platform header file.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.py_time_second")]
extern "C-unwind" fn py_time_second(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_microsecond")]
extern "C-unwind" fn time_microsecond(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe {
        PyLong_FromLong(
            ((*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) as i32) << 16i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) as i32)
                    << 8i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) as i32))
                as i64,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_tzinfo")]
extern "C-unwind" fn time_tzinfo(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    let result: *mut _object =
        if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        };
    Py_NewRef!(result)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_fold")]
extern "C-unwind" fn time_fold(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe { PyLong_FromLong((*self_).fold as i64) }
}

static mut time_getset: __SlateAlign16<[PyGetSetDef; 7]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"hour\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(time_hour);
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"minute\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(time_minute);
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"second\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(py_time_second);
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"microsecond\0".as_ptr() as *mut i8) as *const i8;
        __t3.get = Some(time_microsecond);
        __t3
    },
    {
        let mut __t4: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"tzinfo\0".as_ptr() as *mut i8) as *const i8;
        __t4.get = Some(time_tzinfo);
        __t4
    },
    {
        let mut __t5: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"fold\0".as_ptr() as *mut i8) as *const i8;
        __t5.get = Some(time_fold);
        __t5
    },
    {
        let mut __t6: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t6.name = core::ptr::null::<i8>();
        __t6
    },
]);

// Constructors.
static mut time_kws: __SlateAlign16<[*mut i8; 7]> = __SlateAlign16([
    b"hour\0".as_ptr() as *mut i8,
    b"minute\0".as_ptr() as *mut i8,
    b"second\0".as_ptr() as *mut i8,
    b"microsecond\0".as_ptr() as *mut i8,
    b"tzinfo\0".as_ptr() as *mut i8,
    b"fold\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

fn time_from_pickle(
    r#type: *mut _typeobject,
    state: *mut _object,
    tzinfo: *mut _object,
) -> *mut _object {
    let aware: i8 = (tzinfo != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }) as i8;
    let __v0: bool = aware != 0 && check_tzinfo_subclass(tzinfo) < 0;
    if __v0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"bad tzinfo state arg".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let me: *mut __SlateRecord_datetime_h_86 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, aware as i64) })
            as *mut __SlateRecord_datetime_h_86;
    if !me.is_null() {
        let pdata: *const i8 = PyBytes_AS_STRING!(state) as *const i8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((pdata as *const ()) as *const core::ffi::c_void) as *const u8,
                ((((*me).data.as_mut_ptr() as *mut u8) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                (6u64 as usize) as usize,
            )
        };
        unsafe {
            (*me).hashcode = 1i32.wrapping_neg() as i64;
        }
        unsafe {
            (*me).hastzinfo = aware;
        }
        if aware != 0 {
            unsafe {
                (*me).tzinfo = Py_NewRef!(tzinfo);
            }
        }
        if ((unsafe { *pdata.wrapping_offset(0 as isize) }) as i32) & 128i32 != 0 {
            let __v1: *mut u8 =
                unsafe { (*me).data.as_mut_ptr() as *mut u8 }.wrapping_offset(0 as isize);
            unsafe {
                *__v1 = (*__v1 as i32).wrapping_sub(128) as u8;
            }
            unsafe {
                (*me).fold = 1;
            }
        } else {
            unsafe {
                (*me).fold = 0;
            }
        }
    }
    me as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_new")]
extern "C-unwind" fn time_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut self_: *mut _object = core::ptr::null_mut::<_object>();
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;
    let mut second: i32 = 0;
    let mut usecond: i32 = 0;
    let mut tzinfo: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    let mut fold: i32 = 0;
    // Check for invocation from pickle with __getstate__ state
    let __v0: bool = PyTuple_GET_SIZE!(args) >= 1 && PyTuple_GET_SIZE!(args) <= 2;
    if __v0 {
        let mut state: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
        if PyTuple_GET_SIZE!(args) == 2 {
            tzinfo = PyTuple_GET_ITEM!(args, 1i32);
        }
        if PyBytes_Check!(state) != 0 {
            let __v1: bool = PyBytes_GET_SIZE!(state) == 6
                && 127
                    & (((unsafe { *PyBytes_AS_STRING!(state).wrapping_offset(0 as isize) }) as u8)
                        as i32)
                    < 24;
            if __v1 {
                return time_from_pickle(r#type, state, tzinfo);
            }
        } else {
            if PyUnicode_Check!(state) != 0 {
                let __v2: bool = PyUnicode_GET_LENGTH!(state) == 6
                    && 127 & PyUnicode_READ_CHAR__2!(state, 0i32) < 24;
                if __v2 {
                    state = unsafe { PyUnicode_AsLatin1String(state) };
                    if state.is_null() {
                        if (unsafe { PyErr_ExceptionMatches(PyExc_UnicodeEncodeError) }) != 0 {
                            // More informative error message.
                            unsafe {
                                PyErr_SetString(PyExc_ValueError, c"Failed to encode latin1 string when unpickling a time object. pickle.load(data, encoding='latin1') is assumed.".as_ptr())
                            };
                        }
                        return core::ptr::null_mut::<_object>();
                    }
                    self_ = time_from_pickle(r#type, state, tzinfo);
                    Py_DECREF!(state);
                    return self_;
                }
            }
        }
        tzinfo = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"|iiiiO$i".as_ptr(),
            (core::ptr::addr_of_mut!(time_kws.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(hour),
            core::ptr::addr_of_mut!(minute),
            core::ptr::addr_of_mut!(second),
            core::ptr::addr_of_mut!(usecond),
            core::ptr::addr_of_mut!(tzinfo),
            core::ptr::addr_of_mut!(fold),
        )
    }) != 0
    {
        self_ = new_time_ex2(hour, minute, second, usecond, tzinfo, fold, r#type);
    }
    self_
}

/// Return new time from _strptime.strptime_datetime_time().
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_strptime")]
extern "C-unwind" fn time_strptime(cls: *mut _object, args: *mut _object) -> *mut _object {
    let mut string: *mut _object = unsafe { core::mem::zeroed() };
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UU:strptime".as_ptr(),
            core::ptr::addr_of_mut!(string),
            core::ptr::addr_of_mut!(format),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let module: *mut _object = unsafe {
        PyImport_Import(core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py__strptime
                ._ascii
                .ob_base
        ))
    };
    if module.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe {
        PyObject_CallMethodObjArgs(
            module,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py__strptime_datetime_time
                    ._ascii
                    .ob_base
            ),
            cls,
            string,
            format,
            core::ptr::null_mut::<()>(),
        )
    };
    Py_DECREF!(module);
    result
}

// Destructor.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_dealloc")]
extern "C-unwind" fn time_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
        Py_XDECREF!(unsafe { (*self_).tzinfo });
    }
    unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

// Indirect access to tzinfo methods.
/// These are all METH_NOARGS, so don't need to check the arglist.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_utcoffset")]
extern "C-unwind" fn time_utcoffset(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    call_utcoffset(
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_dst")]
extern "C-unwind" fn time_dst(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    call_dst(
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_tzname")]
extern "C-unwind" fn time_tzname(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    call_tzname(
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
    )
}

// Various ways to turn a time into a string.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_repr")]
extern "C-unwind" fn time_repr(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    let type_name: *const i8 = unsafe { (*Py_TYPE__2!(self_)).tp_name };
    let h: i32 =
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) }) as i32;
    let m: i32 =
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) }) as i32;
    let s: i32 =
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) }) as i32;
    let us: i32 = ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) })
        as i32)
        << 16i32
        | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) })
            as i32)
            << 8i32
        | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) })
            as i32);
    let fold: i32 = (unsafe { (*self_).fold }) as i32;
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    if us != 0 {
        result =
            unsafe { PyUnicode_FromFormat(c"%s(%d, %d, %d, %d)".as_ptr(), type_name, h, m, s, us) };
    } else {
        if s != 0 {
            result =
                unsafe { PyUnicode_FromFormat(c"%s(%d, %d, %d)".as_ptr(), type_name, h, m, s) };
        } else {
            result = unsafe { PyUnicode_FromFormat(c"%s(%d, %d)".as_ptr(), type_name, h, m) };
        }
    }
    if !result.is_null()
        && (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0
    {
        result = append_keyword_tzinfo(result, unsafe { (*self_).tzinfo });
    }
    if !result.is_null() && fold != 0 {
        result = append_keyword_fold(result, fold);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_str")]
extern "C-unwind" fn time_str(op: *mut _object) -> *mut _object {
    PyObject_CallMethodNoArgs(op, unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_isoformat
                ._ascii
                .ob_base
        )
    })
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_isoformat")]
extern "C-unwind" fn time_isoformat(
    op: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut buf: __SlateAlign16<[i8; 100]> = __SlateAlign16([0i8; 100]);
    let mut timespec: *const i8 = core::ptr::null::<i8>();
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut us: i32 =
        ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) }) as i32)
            << 16i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) })
                as i32)
                << 8i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) })
                as i32);
    let mut given_spec: u64 = 0;
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"|s:isoformat".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__4808.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(timespec),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if timespec.is_null()
        || (unsafe {
            strcmp(
                timespec as *const core::ffi::c_char,
                c"auto".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
    {
        if us == 0 {
            // seconds
            given_spec = 2;
        } else {
            // microseconds
            given_spec = 4;
        }
    } else {
        given_spec = 0;
        while given_spec < Py_ARRAY_LENGTH__3!() {
            if (unsafe {
                strcmp(
                    timespec as *const core::ffi::c_char,
                    specs__4813.0[given_spec as usize][0] as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                if given_spec == 3 {
                    // milliseconds
                    us /= 1000;
                }
                break;
            }
            given_spec = given_spec.wrapping_add(1);
        }
    }
    if given_spec == Py_ARRAY_LENGTH__3!() {
        unsafe { PyErr_Format(PyExc_ValueError, c"Unknown timespec value".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    } else {
        result = unsafe {
            PyUnicode_FromFormat(
                specs__4813.0[given_spec as usize][1],
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) as i32,
                us,
            )
        };
    }
    if result.is_null()
        || (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) == 0
        || (unsafe { (*self_).tzinfo }) == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
    {
        return result;
    }
    // We need to append the UTC offset.
    if format_utcoffset(
        buf.0.as_mut_ptr() as *mut i8,
        core::mem::size_of::<[i8; 100]>() as u64,
        c":".as_ptr(),
        unsafe { (*self_).tzinfo },
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
    ) < 0
    {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        PyUnicode_AppendAndDel(
            core::ptr::addr_of_mut!(result),
            PyUnicode_FromString(buf.0.as_mut_ptr() as *const i8),
        )
    };
    result
}

static mut keywords__4808: __SlateAlign16<[*mut i8; 2]> = __SlateAlign16([
    b"timespec\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

static mut specs__4813: __SlateAlign16<[[*const i8; 2]; 5]> = __SlateAlign16([
    [
        (b"hours\0".as_ptr() as *mut i8) as *const i8,
        (b"%02d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"minutes\0".as_ptr() as *mut i8) as *const i8,
        (b"%02d:%02d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"seconds\0".as_ptr() as *mut i8) as *const i8,
        (b"%02d:%02d:%02d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"milliseconds\0".as_ptr() as *mut i8) as *const i8,
        (b"%02d:%02d:%02d.%03d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"microseconds\0".as_ptr() as *mut i8) as *const i8,
        (b"%02d:%02d:%02d.%06d\0".as_ptr() as *mut i8) as *const i8,
    ],
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_strftime")]
extern "C-unwind" fn time_strftime(
    op: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"U:strftime".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__4876.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(format),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    // Python's strftime does insane things with the year part of the
    // timetuple.  The year is forced to (the otherwise nonsensical)
    // 1900 to work around that.
    let tuple: *mut _object = unsafe {
        Py_BuildValue(
            c"iiiiiiiii".as_ptr(),
            1900i32,
            1i32,
            1i32,
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) as i32,
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) as i32,
            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) as i32,
            0i32,
            1i32,
            1i32.wrapping_neg(),
        )
    }; // year, month, day
    // weekday, daynum, dst
    if tuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = wrap_strftime(self_ as *mut _object, format, tuple, unsafe {
        core::ptr::addr_of_mut!(_Py_NoneStruct)
    });
    Py_DECREF!(tuple);
    result
}

static mut keywords__4876: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"format\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

// Miscellaneous methods.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_richcompare")]
extern "C-unwind" fn time_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    // If they're both naive, or both aware and have the same offsets,
    // we get off cheap.  Note that if they're both naive, offset1 ==
    // offset2 == Py_None at this point.
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut diff: i32 = unsafe { core::mem::zeroed() };
    let mut offset2: *mut _object = unsafe { core::mem::zeroed() };
    let mut offset1: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        if PyTime_Check!(other) == 0 {
            return core::ptr::addr_of_mut!(_Py_NotImplementedStruct);
        } else {
            if (if _PyDateTime_HAS_TZINFO!(self_) != 0 {
                (*(self_ as *mut __SlateRecord_datetime_h_86)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            }) == if _PyDateTime_HAS_TZINFO!(other) != 0 {
                (*(other as *mut __SlateRecord_datetime_h_86)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            } {
                diff = memcmp(
                    (((*(self_ as *mut __SlateRecord_datetime_h_86))
                        .data
                        .as_mut_ptr() as *mut u8) as *const ())
                        as *const core::ffi::c_void,
                    (((*(other as *mut __SlateRecord_datetime_h_86))
                        .data
                        .as_mut_ptr() as *mut u8) as *const ())
                        as *const core::ffi::c_void,
                    6u64 as usize,
                ) as i32;
                return diff_to_bool(diff, op);
            } else {
                offset1 = time_utcoffset(self_, core::ptr::null_mut::<_object>());
                if offset1.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    offset2 = time_utcoffset(other, core::ptr::null_mut::<_object>());
                    if !offset2.is_null() {
                        if offset1 == offset2 {
                            __v4 = true;
                        } else {
                            if PyDelta_Check!(offset1) != 0 {
                                __v3 = PyDelta_Check!(offset2) != 0;
                            } else {
                                __v3 = false;
                            }
                            if __v3 {
                                __v2 = delta_cmp(offset1, offset2) == 0;
                            } else {
                                __v2 = false;
                            }
                            __v4 = __v2;
                        }
                        if __v4 {
                            diff = memcmp(
                                (((*(self_ as *mut __SlateRecord_datetime_h_86))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    as *const ())
                                    as *const core::ffi::c_void,
                                (((*(other as *mut __SlateRecord_datetime_h_86))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    as *const ())
                                    as *const core::ffi::c_void,
                                6u64 as usize,
                            ) as i32;
                            result = diff_to_bool(diff, op);
                        } else {
                            if offset1 != core::ptr::addr_of_mut!(_Py_NoneStruct)
                                && offset2 != core::ptr::addr_of_mut!(_Py_NoneStruct)
                            {
                                // else last "if" handled it
                                let offsecs1: i32 = (*((*(self_
                                    as *mut __SlateRecord_datetime_h_86))
                                    .data
                                    .as_mut_ptr()
                                    as *mut u8)
                                    .wrapping_offset(0 as isize)
                                    as i32)
                                    .wrapping_mul(3600)
                                    .wrapping_add(
                                        (*((*(self_ as *mut __SlateRecord_datetime_h_86))
                                            .data
                                            .as_mut_ptr()
                                            as *mut u8)
                                            .wrapping_offset(1 as isize)
                                            as i32)
                                            .wrapping_mul(60),
                                    )
                                    .wrapping_add(
                                        *((*(self_ as *mut __SlateRecord_datetime_h_86))
                                            .data
                                            .as_mut_ptr()
                                            as *mut u8)
                                            .wrapping_offset(2 as isize)
                                            as i32,
                                    )
                                    .wrapping_sub(GET_TD_DAYS!(offset1).wrapping_mul(86400))
                                    .wrapping_sub(GET_TD_SECONDS!(offset1));
                                let offsecs2: i32 = (*((*(other
                                    as *mut __SlateRecord_datetime_h_86))
                                    .data
                                    .as_mut_ptr()
                                    as *mut u8)
                                    .wrapping_offset(0 as isize)
                                    as i32)
                                    .wrapping_mul(3600)
                                    .wrapping_add(
                                        (*((*(other as *mut __SlateRecord_datetime_h_86))
                                            .data
                                            .as_mut_ptr()
                                            as *mut u8)
                                            .wrapping_offset(1 as isize)
                                            as i32)
                                            .wrapping_mul(60),
                                    )
                                    .wrapping_add(
                                        *((*(other as *mut __SlateRecord_datetime_h_86))
                                            .data
                                            .as_mut_ptr()
                                            as *mut u8)
                                            .wrapping_offset(2 as isize)
                                            as i32,
                                    )
                                    .wrapping_sub(GET_TD_DAYS!(offset2).wrapping_mul(86400))
                                    .wrapping_sub(GET_TD_SECONDS!(offset2));
                                diff = offsecs1.wrapping_sub(offsecs2);
                                if diff == 0 {
                                    diff = ((*((*(self_ as *mut __SlateRecord_datetime_h_86))
                                        .data
                                        .as_mut_ptr()
                                        as *mut u8)
                                        .wrapping_offset(3 as isize)
                                        as i32)
                                        << 16i32
                                        | (*((*(self_ as *mut __SlateRecord_datetime_h_86))
                                            .data
                                            .as_mut_ptr()
                                            as *mut u8)
                                            .wrapping_offset(4 as isize)
                                            as i32)
                                            << 8i32
                                        | (*((*(self_ as *mut __SlateRecord_datetime_h_86))
                                            .data
                                            .as_mut_ptr()
                                            as *mut u8)
                                            .wrapping_offset(5 as isize)
                                            as i32))
                                        .wrapping_sub(
                                            (*((*(other as *mut __SlateRecord_datetime_h_86))
                                                .data
                                                .as_mut_ptr()
                                                as *mut u8)
                                                .wrapping_offset(3 as isize)
                                                as i32)
                                                << 16i32
                                                | (*((*(other as *mut __SlateRecord_datetime_h_86))
                                                    .data
                                                    .as_mut_ptr()
                                                    as *mut u8)
                                                    .wrapping_offset(4 as isize)
                                                    as i32)
                                                    << 8i32
                                                | (*((*(other as *mut __SlateRecord_datetime_h_86))
                                                    .data
                                                    .as_mut_ptr()
                                                    as *mut u8)
                                                    .wrapping_offset(5 as isize)
                                                    as i32),
                                        );
                                }
                                result = diff_to_bool(diff, op);
                            } else {
                                if op == 2 {
                                    result = Py_NewRef!(_PyObject_CAST__2!(
                                        core::ptr::addr_of_mut!(_Py_FalseStruct)
                                    ));
                                } else {
                                    if op == 3 {
                                        result = Py_NewRef!(_PyObject_CAST__2!(
                                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                                        ));
                                    } else {
                                        PyErr_SetString(
                                            PyExc_TypeError,
                                            c"can't compare offset-naive and offset-aware times"
                                                .as_ptr(),
                                        );
                                    }
                                }
                            }
                        }
                        // The hard case: both aware with different UTC offsets
                    }
                    Py_DECREF!(offset1);
                    Py_XDECREF!(offset2);
                    return result;
                }
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_hash")]
extern "C-unwind" fn time_hash(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    let mut hash: i64 = unsafe { (*self_).hashcode };
    if hash == (1i32.wrapping_neg() as i64) {
        let mut self0: *mut _object = unsafe { core::mem::zeroed() };
        if (unsafe { (*self_).fold }) != 0 {
            self0 = new_time_ex2(
                (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) })
                    as i32,
                (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize) })
                    as i32,
                (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize) })
                    as i32,
                ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) })
                    as i32)
                    << 16i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize)
                    }) as i32)
                        << 8i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize)
                    }) as i32),
                if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
                    unsafe { (*self_).tzinfo }
                } else {
                    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
                },
                0i32,
                Py_TYPE__2!(self_),
            );
            if self0.is_null() {
                return 1i32.wrapping_neg() as i64;
            }
        } else {
            self0 = Py_NewRef__2!(self_);
        }
        let offset: *mut _object = time_utcoffset(self0, core::ptr::null_mut::<_object>());
        Py_DECREF!(self0);
        if offset.is_null() {
            return 1i32.wrapping_neg() as i64;
        }
        // Reduce this to a hash of another object.
        if offset == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            hash = generic_hash(unsafe { (*self_).data.as_mut_ptr() as *mut u8 }, 6i32);
            FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).hashcode, hash);
        } else {
            let mut seconds: i32 = 0;
            let mut microseconds: i32 = 0;
            seconds =
                ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(0 as isize) })
                    as i32)
                    .wrapping_mul(3600)
                    .wrapping_add(
                        ((unsafe {
                            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(1 as isize)
                        }) as i32)
                            .wrapping_mul(60),
                    )
                    .wrapping_add(
                        (unsafe {
                            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(2 as isize)
                        }) as i32,
                    );
            microseconds =
                ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(3 as isize) })
                    as i32)
                    << 16i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize)
                    }) as i32)
                        << 8i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize)
                    }) as i32);
            let temp1: *mut _object = new_delta!(0i32, seconds, microseconds, 1i32);
            if temp1.is_null() {
                Py_DECREF!(offset);
                return 1i32.wrapping_neg() as i64;
            }
            let temp2: *mut _object = delta_subtract(temp1, offset);
            Py_DECREF!(temp1);
            if temp2.is_null() {
                Py_DECREF!(offset);
                return 1i32.wrapping_neg() as i64;
            }
            hash = unsafe { PyObject_Hash(temp2) };
            FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).hashcode, hash);
            Py_DECREF!(temp2);
        }
        Py_DECREF!(offset);
    }
    hash
}

// [clinic input]
// datetime.time.replace
//
//     hour: int(c_default="TIME_GET_HOUR(self)") = unchanged
//     minute: int(c_default="TIME_GET_MINUTE(self)") = unchanged
//     second: int(c_default="TIME_GET_SECOND(self)") = unchanged
//     microsecond: int(c_default="TIME_GET_MICROSECOND(self)") = unchanged
//     tzinfo: object(c_default="HASTZINFO(self) ? ((PyDateTime_Time *)self)->tzinfo : Py_None") = unchanged
//     *
//     fold: int(c_default="TIME_GET_FOLD(self)") = unchanged
//
// Return time with new specified fields.
// [clinic start generated code]
/// [clinic end generated code: output=0b89a44c299e4f80 input=abf23656e8df4e97]
fn datetime_time_replace_impl(
    self_: *mut __SlateRecord_datetime_h_86,
    hour: i32,
    minute: i32,
    second: i32,
    microsecond: i32,
    tzinfo: *mut _object,
    fold: i32,
) -> *mut _object {
    new_time_subclass_fold_ex(
        hour,
        minute,
        second,
        microsecond,
        tzinfo,
        fold,
        Py_TYPE__2!(self_) as *mut _object,
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_fromisoformat")]
extern "C-unwind" fn time_fromisoformat(cls: *mut _object, tstr: *mut _object) -> *mut _object {
    let mut t: *mut _object = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut rv: i32 = unsafe { core::mem::zeroed() };
    let mut tzimicrosecond: i32 = unsafe { core::mem::zeroed() };
    let mut tzoffset: i32 = unsafe { core::mem::zeroed() };
    let mut microsecond: i32 = unsafe { core::mem::zeroed() };
    let mut second: i32 = unsafe { core::mem::zeroed() };
    let mut minute: i32 = unsafe { core::mem::zeroed() };
    let mut hour: i32 = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        if PyUnicode_Check!(tstr) == 0 {
            PyErr_SetString(
                PyExc_TypeError,
                c"fromisoformat: argument must be str".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            p = PyUnicode_AsUTF8AndSize(tstr, core::ptr::addr_of_mut!(len));
            if !p.is_null() {
                // The spec actually requires that time-only ISO 8601 strings start with
                //
                // T, but the extended format allows this to be omitted as long as there
                //
                // is no ambiguity with date strings.
                if (*p as i32) == 84 {
                    let __v12: *const i8 = p;
                    p = __v12.wrapping_offset(1 as isize);
                    len = len.wrapping_sub(1);
                }
                hour = 0;
                minute = 0;
                second = 0;
                microsecond = 0;
                tzoffset = 0;
                tzimicrosecond = 0;
                rv = parse_isoformat_time(
                    p,
                    len as u64,
                    core::ptr::addr_of_mut!(hour),
                    core::ptr::addr_of_mut!(minute),
                    core::ptr::addr_of_mut!(second),
                    core::ptr::addr_of_mut!(microsecond),
                    &mut (*core::ptr::addr_of_mut!(tzoffset)),
                    core::ptr::addr_of_mut!(tzimicrosecond),
                );
                if rv >= 0 {
                    if hour == 24 {
                        if minute == 0 && second == 0 && microsecond == 0 {
                            hour = 0;
                        } else {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"minute, second, and microsecond must be 0 when hour is 24"
                                    .as_ptr(),
                            );
                            return core::ptr::null_mut::<_object>();
                        }
                    }
                    tzinfo = tzinfo_from_isoformat_results(rv, tzoffset, tzimicrosecond);
                    if tzinfo.is_null() {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        if (cls as *mut _typeobject) == TIME_TYPE!() {
                            t = new_time!(hour, minute, second, microsecond, tzinfo, 0i32);
                        } else {
                            t = PyObject_CallFunction(
                                cls,
                                c"iiiiO".as_ptr(),
                                hour,
                                minute,
                                second,
                                microsecond,
                                tzinfo,
                            );
                        }
                        Py_DECREF!(tzinfo);
                        return t;
                    }
                }
            }
            PyErr_Format(
                PyExc_ValueError,
                c"Invalid isoformat string: %R".as_ptr(),
                tstr,
            );
            return core::ptr::null_mut::<_object>();
        }
    }
}

// Pickle support, a simple use of __reduce__.
/// Let basestate be the non-tzinfo data string.
/// If tzinfo is None, this returns (basestate,), else (basestate, tzinfo).
/// So it's a tuple in any (non-error) case.
/// __getstate__ isn't exposed.
fn time_getstate(self_: *mut __SlateRecord_datetime_h_86, proto: i32) -> *mut _object {
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    let basestate: *mut _object = unsafe {
        PyBytes_FromStringAndSize(
            (((*self_).data.as_mut_ptr() as *mut u8) as *mut i8) as *const i8,
            6i64,
        )
    };
    if !basestate.is_null() {
        if proto > 3 && (unsafe { (*self_).fold }) != 0 {
            let __v0: *mut i8 = PyBytes_AS_STRING!(basestate).wrapping_offset(0 as isize);
            unsafe {
                *__v0 = ((*__v0 as i32) | 128i32) as i8;
            }
        }
        // Set the first bit of the first byte
        if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) == 0
            || (unsafe { (*self_).tzinfo }) == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        {
            result = unsafe { PyTuple_Pack(1i64, basestate) };
        } else {
            result = unsafe { PyTuple_Pack(2i64, basestate, (*self_).tzinfo) };
        }
        Py_DECREF!(basestate);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_reduce_ex")]
extern "C-unwind" fn time_reduce_ex(op: *mut _object, args: *mut _object) -> *mut _object {
    let mut proto: i32 = 0;
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"i:__reduce_ex__".as_ptr(),
            core::ptr::addr_of_mut!(proto),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe {
        Py_BuildValue(
            c"(ON)".as_ptr(),
            Py_TYPE__2!(self_),
            time_getstate(self_, proto),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.time_reduce")]
extern "C-unwind" fn time_reduce(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_86 = PyTime_CAST!(op);
    unsafe {
        Py_BuildValue(
            c"(ON)".as_ptr(),
            Py_TYPE__2!(self_),
            time_getstate(self_, 2i32),
        )
    }
}

/// Class method:
/// Instance methods:
static mut time_methods: __SlateAlign16<[PyMethodDef; 13]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"strptime\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(time_strptime);
        __t0.ml_flags = 17i32;
        __t0.ml_doc = (b"string, format -> new time parsed from a string (like time.strptime()).\0"
            .as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"isoformat\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = unsafe {
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
                >(Some(time_isoformat))
            })
        };
        __t1.ml_flags = 3i32;
        __t1.ml_doc = (b"Return string in ISO 8601 format, [HH[:MM[:SS[.mmm[uuu]]]]][+HH:MM].\n\nThe optional argument timespec specifies the number of additional terms\nof the time to include. Valid options are 'auto', 'hours', 'minutes',\n'seconds', 'milliseconds' and 'microseconds'.\n\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"strftime\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = unsafe {
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
                >(Some(time_strftime))
            })
        };
        __t2.ml_flags = 3i32;
        __t2.ml_doc = (b"format -> strftime() style string.\0".as_ptr() as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"__format__\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(date_format);
        __t3.ml_flags = 1i32;
        __t3.ml_doc = (b"Formats self with strftime.\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"utcoffset\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(time_utcoffset);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = (b"Return self.tzinfo.utcoffset(self).\0".as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"tzname\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(time_tzname);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = (b"Return self.tzinfo.tzname(self).\0".as_ptr() as *mut i8) as *const i8;
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"dst\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(time_dst);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = (b"Return self.tzinfo.dst(self).\0".as_ptr() as *mut i8) as *const i8;
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"replace\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(datetime_time_replace))
            })
        };
        __t7.ml_flags = 130i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(datetime_time_replace__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"__replace__\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(datetime_time_replace))
            })
        };
        __t8.ml_flags = 130i32;
        __t8.ml_doc = (b"__replace__($self, /, **changes)\n--\n\nThe same as replace().\0".as_ptr()
            as *mut i8) as *const i8;
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"fromisoformat\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(time_fromisoformat);
        __t9.ml_flags = 24i32;
        __t9.ml_doc =
            (b"string -> time from a string in ISO 8601 format\0".as_ptr() as *mut i8) as *const i8;
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"__reduce_ex__\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(time_reduce_ex);
        __t10.ml_flags = 1i32;
        __t10.ml_doc = (b"__reduce_ex__(proto) -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(time_reduce);
        __t11.ml_flags = 4i32;
        __t11.ml_doc = (b"__reduce__() -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = core::ptr::null::<i8>();
        __t12.ml_meth = None;
        __t12
    },
]);

static time_doc: __SlateAlign16<[i8; 200]> = __SlateAlign16([
    116i8, 105i8, 109i8, 101i8, 40i8, 91i8, 104i8, 111i8, 117i8, 114i8, 91i8, 44i8, 32i8, 109i8,
    105i8, 110i8, 117i8, 116i8, 101i8, 91i8, 44i8, 32i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8,
    91i8, 44i8, 32i8, 109i8, 105i8, 99i8, 114i8, 111i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8,
    91i8, 44i8, 32i8, 116i8, 122i8, 105i8, 110i8, 102i8, 111i8, 93i8, 93i8, 93i8, 93i8, 93i8, 41i8,
    32i8, 45i8, 45i8, 62i8, 32i8, 97i8, 32i8, 116i8, 105i8, 109i8, 101i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 10i8, 10i8, 65i8, 108i8, 108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8,
    110i8, 97i8, 108i8, 46i8, 32i8, 116i8, 122i8, 105i8, 110i8, 102i8, 111i8, 32i8, 109i8, 97i8,
    121i8, 32i8, 98i8, 101i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 111i8, 114i8, 32i8,
    97i8, 110i8, 32i8, 105i8, 110i8, 115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 32i8, 111i8, 102i8,
    10i8, 97i8, 32i8, 116i8, 122i8, 105i8, 110i8, 102i8, 111i8, 32i8, 115i8, 117i8, 98i8, 99i8,
    108i8, 97i8, 115i8, 115i8, 46i8, 32i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 109i8, 97i8,
    105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8,
    115i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8, 32i8, 105i8, 110i8, 116i8, 115i8, 46i8,
    10i8, 0i8,
]);

// PyDateTime_DateTime implementation.
// Accessor properties.  Properties for day, month, and year are inherited
// from date.
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
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_hour")]
extern "C-unwind" fn datetime_hour(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_minute")]
extern "C-unwind" fn datetime_minute(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_second")]
extern "C-unwind" fn datetime_second(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe {
        PyLong_FromLong(*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) as i64)
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_microsecond")]
extern "C-unwind" fn datetime_microsecond(
    op: *mut _object,
    _unused_closure: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe {
        PyLong_FromLong(
            ((*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) as i32) << 16i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) as i32)
                    << 8i32
                | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) as i32))
                as i64,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_tzinfo")]
extern "C-unwind" fn datetime_tzinfo(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let result: *mut _object =
        if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        };
    Py_NewRef!(result)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_fold")]
extern "C-unwind" fn datetime_fold(op: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe { PyLong_FromLong((*self_).fold as i64) }
}

static mut datetime_getset: __SlateAlign16<[PyGetSetDef; 7]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"hour\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(datetime_hour);
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"minute\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(datetime_minute);
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"second\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(datetime_second);
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"microsecond\0".as_ptr() as *mut i8) as *const i8;
        __t3.get = Some(datetime_microsecond);
        __t3
    },
    {
        let mut __t4: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"tzinfo\0".as_ptr() as *mut i8) as *const i8;
        __t4.get = Some(datetime_tzinfo);
        __t4
    },
    {
        let mut __t5: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"fold\0".as_ptr() as *mut i8) as *const i8;
        __t5.get = Some(datetime_fold);
        __t5
    },
    {
        let mut __t6: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t6.name = core::ptr::null::<i8>();
        __t6
    },
]);

// Constructors.
static mut datetime_kws: __SlateAlign16<[*mut i8; 10]> = __SlateAlign16([
    b"year\0".as_ptr() as *mut i8,
    b"month\0".as_ptr() as *mut i8,
    b"day\0".as_ptr() as *mut i8,
    b"hour\0".as_ptr() as *mut i8,
    b"minute\0".as_ptr() as *mut i8,
    b"second\0".as_ptr() as *mut i8,
    b"microsecond\0".as_ptr() as *mut i8,
    b"tzinfo\0".as_ptr() as *mut i8,
    b"fold\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

fn datetime_from_pickle(
    r#type: *mut _typeobject,
    state: *mut _object,
    tzinfo: *mut _object,
) -> *mut _object {
    let aware: i8 = (tzinfo != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }) as i8;
    let __v0: bool = aware != 0 && check_tzinfo_subclass(tzinfo) < 0;
    if __v0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"bad tzinfo state arg".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let me: *mut __SlateRecord_datetime_h_114 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, aware as i64) })
            as *mut __SlateRecord_datetime_h_114;
    if !me.is_null() {
        let pdata: *const i8 = PyBytes_AS_STRING!(state) as *const i8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((pdata as *const ()) as *const core::ffi::c_void) as *const u8,
                ((((*me).data.as_mut_ptr() as *mut u8) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                (10u64 as usize) as usize,
            )
        };
        unsafe {
            (*me).hashcode = 1i32.wrapping_neg() as i64;
        }
        unsafe {
            (*me).hastzinfo = aware;
        }
        if aware != 0 {
            unsafe {
                (*me).tzinfo = Py_NewRef!(tzinfo);
            }
        }
        if ((unsafe { *pdata.wrapping_offset(2 as isize) }) as i32) & 128i32 != 0 {
            let __v1: *mut u8 =
                unsafe { (*me).data.as_mut_ptr() as *mut u8 }.wrapping_offset(2 as isize);
            unsafe {
                *__v1 = (*__v1 as i32).wrapping_sub(128) as u8;
            }
            unsafe {
                (*me).fold = 1;
            }
        } else {
            unsafe {
                (*me).fold = 0;
            }
        }
    }
    me as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_new")]
extern "C-unwind" fn datetime_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut self_: *mut _object = core::ptr::null_mut::<_object>();
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;
    let mut second: i32 = 0;
    let mut usecond: i32 = 0;
    let mut fold: i32 = 0;
    let mut tzinfo: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    // Check for invocation from pickle with __getstate__ state
    let __v0: bool = PyTuple_GET_SIZE!(args) >= 1 && PyTuple_GET_SIZE!(args) <= 2;
    if __v0 {
        let mut state: *mut _object = PyTuple_GET_ITEM!(args, 0i32);
        if PyTuple_GET_SIZE!(args) == 2 {
            tzinfo = PyTuple_GET_ITEM!(args, 1i32);
        }
        if PyBytes_Check!(state) != 0 {
            let __v1: bool = PyBytes_GET_SIZE!(state) == 10
                && MONTH_IS_SANE!(
                    ((unsafe { *PyBytes_AS_STRING!(state).wrapping_offset(2 as isize) }) as i32)
                        & 127
                );
            if __v1 {
                return datetime_from_pickle(r#type, state, tzinfo);
            }
        } else {
            if PyUnicode_Check!(state) != 0 {
                let __v2: bool = PyUnicode_GET_LENGTH!(state) == 10
                    && MONTH_IS_SANE!(PyUnicode_READ_CHAR__2!(state, 2i32) & 127);
                if __v2 {
                    state = unsafe { PyUnicode_AsLatin1String(state) };
                    if state.is_null() {
                        if (unsafe { PyErr_ExceptionMatches(PyExc_UnicodeEncodeError) }) != 0 {
                            // More informative error message.
                            unsafe {
                                PyErr_SetString(PyExc_ValueError, c"Failed to encode latin1 string when unpickling a datetime object. pickle.load(data, encoding='latin1') is assumed.".as_ptr())
                            };
                        }
                        return core::ptr::null_mut::<_object>();
                    }
                    self_ = datetime_from_pickle(r#type, state, tzinfo);
                    Py_DECREF!(state);
                    return self_;
                }
            }
        }
        tzinfo = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"iii|iiiiO$i".as_ptr(),
            (core::ptr::addr_of_mut!(datetime_kws.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(year),
            core::ptr::addr_of_mut!(month),
            core::ptr::addr_of_mut!(day),
            core::ptr::addr_of_mut!(hour),
            core::ptr::addr_of_mut!(minute),
            core::ptr::addr_of_mut!(second),
            core::ptr::addr_of_mut!(usecond),
            core::ptr::addr_of_mut!(tzinfo),
            core::ptr::addr_of_mut!(fold),
        )
    }) != 0
    {
        self_ = new_datetime_ex2(
            year, month, day, hour, minute, second, usecond, tzinfo, fold, r#type,
        );
    }
    self_
}

/// TM_FUNC is the shared type of _PyTime_localtime() and
/// _PyTime_gmtime().
/// As of version 2015f max fold in IANA database is
/// 23 hours at 1969-09-30 13:00:00 in Kwajalein.
static mut max_fold_seconds: i64 = 86400;

/// NB: date(1970,1,1).toordinal() == 719163
static mut epoch: i64 = 62135683200;

fn utc_to_seconds(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> i64 {
    let mut ordinal: i64 = 0;
    // ymd_to_ord() doesn't support year <= 0
    if year < 1 || year > 9999 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"year must be in %d..%d, not %d".as_ptr(),
                1i32,
                9999i32,
                year,
            )
        };
        return 1i32.wrapping_neg() as i64;
    }
    ordinal = ymd_to_ord(year, month, day) as i64;
    ordinal
        .wrapping_mul(24)
        .wrapping_add(hour as i64)
        .wrapping_mul(60)
        .wrapping_add(minute as i64)
        .wrapping_mul(60)
        .wrapping_add(second as i64)
}

fn local(mut u: i64) -> i64 {
    let mut local_time: slate_support::tm = unsafe { core::mem::zeroed() };
    let mut t: i64 = 0;
    u = u.wrapping_sub(unsafe { epoch });
    t = u;
    if t != u {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"timestamp out of range for platform time_t".as_ptr(),
            )
        };
        return 1i32.wrapping_neg() as i64;
    }
    if (unsafe { _PyTime_localtime(t, core::ptr::addr_of_mut!(local_time)) }) != 0 {
        return 1i32.wrapping_neg() as i64;
    }
    utc_to_seconds(
        local_time.tm_year.wrapping_add(1900),
        local_time.tm_mon.wrapping_add(1),
        local_time.tm_mday,
        local_time.tm_hour,
        local_time.tm_min,
        local_time.tm_sec,
    )
}

/// Internal helper.
/// Build datetime from a time_t and a distinct count of microseconds.
/// Pass localtime or gmtime for f, to control the interpretation of timet.
fn datetime_from_timet_and_us(
    cls: *mut _object,
    f: Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
    timet: i64,
    us: i32,
    tzinfo: *mut _object,
) -> *mut _object {
    let mut tm: slate_support::tm = unsafe { core::mem::zeroed() };
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;
    let mut second: i32 = 0;
    let mut fold: i32 = 0;
    if (unsafe { f.unwrap()(timet, core::ptr::addr_of_mut!(tm)) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    year = tm.tm_year.wrapping_add(1900);
    month = tm.tm_mon.wrapping_add(1);
    day = tm.tm_mday;
    hour = tm.tm_hour;
    minute = tm.tm_min;
    // The platform localtime/gmtime may insert leap seconds,
    // indicated by tm.tm_sec > 59.  We don't care about them,
    // except to the extent that passing them on to the datetime
    // constructor would raise ValueError for a reason that
    // made no sense to the user.
    second = Py_MIN!(59i32, tm.tm_sec);
    // local timezone requires to compute fold
    if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        && f == unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
            >(_PyTime_localtime as *const ())
        }
    {
        let mut probe_seconds: i64 = 0;
        let result_seconds: i64 = utc_to_seconds(year, month, day, hour, minute, second);
        let mut transition: i64 = 0;
        let __v0: bool = result_seconds == (1i32.wrapping_neg() as i64)
            && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return core::ptr::null_mut::<_object>();
        }
        // Probe max_fold_seconds to detect a fold.
        probe_seconds = local(
            unsafe { epoch }
                .wrapping_add(timet)
                .wrapping_sub(unsafe { max_fold_seconds }),
        );
        if probe_seconds == (1i32.wrapping_neg() as i64) {
            return core::ptr::null_mut::<_object>();
        }
        transition = result_seconds
            .wrapping_sub(probe_seconds)
            .wrapping_sub(unsafe { max_fold_seconds });
        if transition < 0 {
            probe_seconds = local(
                unsafe { epoch }
                    .wrapping_add(timet)
                    .wrapping_add(transition),
            );
            if probe_seconds == (1i32.wrapping_neg() as i64) {
                return core::ptr::null_mut::<_object>();
            }
            if probe_seconds == result_seconds {
                fold = 1;
            }
        }
    }
    // On Windows, passing a negative value to local results
    // in an OSError because localtime_s on Windows does
    // not support negative timestamps. Unfortunately this
    // means that fold detection for time values between
    // 0 and max_fold_seconds will result in an identical
    // error since we subtract max_fold_seconds to detect a
    // fold. However, since we know there haven't been any
    // folds in the interval [0, max_fold_seconds) in any
    // timezone, we can hackily just forego fold detection
    // for this time range.
    new_datetime_subclass_fold_ex(
        year, month, day, hour, minute, second, us, tzinfo, fold, cls,
    )
}

/// Internal helper.
/// Build datetime from a Python timestamp.  Pass localtime or gmtime for f,
/// to control the interpretation of the timestamp.  Since a double doesn't
/// have enough bits to cover a datetime's full range of precision, it's
/// better to call datetime_from_timet_and_us provided you have a way
/// to get that much precision (e.g., C time() isn't good enough).
fn datetime_from_timestamp(
    cls: *mut _object,
    f: Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
    timestamp: *mut _object,
    tzinfo: *mut _object,
) -> *mut _object {
    let mut timet: i64 = 0;
    let mut us: i64 = 0;
    if (unsafe {
        _PyTime_ObjectToTimeval(
            timestamp,
            core::ptr::addr_of_mut!(timet),
            core::ptr::addr_of_mut!(us),
            2u32,
        )
    }) == 1i32.wrapping_neg()
    {
        return core::ptr::null_mut::<_object>();
    }
    datetime_from_timet_and_us(cls, f, timet, us as i32, tzinfo)
}

/// Internal helper.
/// Build most accurate possible datetime for current time.  Pass localtime or
/// gmtime for f as appropriate.
fn datetime_best_possible(
    cls: *mut _object,
    f: Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
    tzinfo: *mut _object,
) -> *mut _object {
    let mut ts: i64 = 0;
    if (unsafe { PyTime_Time(core::ptr::addr_of_mut!(ts)) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let mut secs: i64 = 0;
    let mut us: i32 = 0;
    if (unsafe {
        _PyTime_AsTimevalTime_t(
            ts,
            core::ptr::addr_of_mut!(secs),
            core::ptr::addr_of_mut!(us),
            0u32,
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    datetime_from_timet_and_us(cls, f, secs, us, tzinfo)
}

// [clinic input]
//
// @classmethod
// datetime.datetime.now
//
//     tz: object = None
//         Timezone object.
//
// Returns new datetime object representing current time local to tz.
//
// If no tz is specified, uses local timezone.
// [clinic start generated code]
/// [clinic end generated code: output=b3386e5345e2b47a input=80d09869c5267d00]
fn datetime_datetime_now_impl(r#type: *mut _typeobject, tz: *mut _object) -> *mut _object {
    // Return best possible local time -- this isn't constrained by the
    // precision of a timestamp.
    if check_tzinfo_subclass(tz) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut _object = datetime_best_possible(
        r#type as *mut _object,
        {
            let __t0: Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32> =
                if tz == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
                    unsafe {
                        core::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
                        >(_PyTime_localtime as *const ())
                    }
                } else {
                    unsafe {
                        core::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
                        >(_PyTime_gmtime as *const ())
                    }
                };
            __t0
        },
        tz,
    );
    if !self_.is_null() && tz != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        // Convert UTC to tzinfo's zone.
        let res: *mut _object = PyObject_CallMethodOneArg(
            tz,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_fromutc
                        ._ascii
                        .ob_base
                )
            },
            self_,
        );
        Py_DECREF!(self_);
        return res;
    }
    self_
}

/// Return best possible UTC time -- this isn't constrained by the
/// precision of a timestamp.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_utcnow")]
extern "C-unwind" fn datetime_utcnow(cls: *mut _object, dummy: *mut _object) -> *mut _object {
    if (unsafe {
        PyErr_WarnEx(PyExc_DeprecationWarning, c"datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).".as_ptr(), 1i64)
    }) != 0
    {
        return core::ptr::null_mut::<_object>();
    }
    datetime_best_possible(
        cls,
        unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
            >(_PyTime_gmtime as *const ())
        },
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
    )
}

/// Return new local datetime from timestamp (Python timestamp -- a double).
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_fromtimestamp")]
extern "C-unwind" fn datetime_fromtimestamp(
    cls: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut timestamp: *mut _object = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"O|O:fromtimestamp".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__5659.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(timestamp),
            core::ptr::addr_of_mut!(tzinfo),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if check_tzinfo_subclass(tzinfo) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut _object = datetime_from_timestamp(
        cls,
        {
            let __t0: Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32> =
                if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
                    unsafe {
                        core::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
                        >(_PyTime_localtime as *const ())
                    }
                } else {
                    unsafe {
                        core::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
                        >(_PyTime_gmtime as *const ())
                    }
                };
            __t0
        },
        timestamp,
        tzinfo,
    );
    if !self_.is_null() && tzinfo != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        // Convert UTC to tzinfo's zone.
        let res: *mut _object = PyObject_CallMethodOneArg(
            tzinfo,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_fromutc
                        ._ascii
                        .ob_base
                )
            },
            self_,
        );
        Py_DECREF!(self_);
        return res;
    }
    self_
}

static mut keywords__5659: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"timestamp\0".as_ptr() as *mut i8,
    b"tz\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

/// Return new UTC datetime from timestamp (Python timestamp -- a double).
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_utcfromtimestamp")]
extern "C-unwind" fn datetime_utcfromtimestamp(
    cls: *mut _object,
    args: *mut _object,
) -> *mut _object {
    if (unsafe {
        PyErr_WarnEx(PyExc_DeprecationWarning, c"datetime.datetime.utcfromtimestamp() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.fromtimestamp(timestamp, datetime.UTC).".as_ptr(), 1i64)
    }) != 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let mut timestamp: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"O:utcfromtimestamp".as_ptr(),
            core::ptr::addr_of_mut!(timestamp),
        )
    }) != 0
    {
        result = datetime_from_timestamp(
            cls,
            unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i64, *mut slate_support::tm) -> i32>,
                >(_PyTime_gmtime as *const ())
            },
            timestamp,
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
        );
    }
    result
}

/// Return new datetime from _strptime.strptime_datetime_datetime().
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_strptime")]
extern "C-unwind" fn datetime_strptime(cls: *mut _object, args: *mut _object) -> *mut _object {
    let mut string: *mut _object = unsafe { core::mem::zeroed() };
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UU:strptime".as_ptr(),
            core::ptr::addr_of_mut!(string),
            core::ptr::addr_of_mut!(format),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let module: *mut _object = unsafe {
        PyImport_Import(core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py__strptime
                ._ascii
                .ob_base
        ))
    };
    if module.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe {
        PyObject_CallMethodObjArgs(
            module,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py__strptime_datetime_datetime
                    ._ascii
                    .ob_base
            ),
            cls,
            string,
            format,
            core::ptr::null_mut::<()>(),
        )
    };
    Py_DECREF!(module);
    result
}

/// Return new datetime from date/datetime and time arguments.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_combine")]
extern "C-unwind" fn datetime_combine(
    cls: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut date: *mut _object = unsafe { core::mem::zeroed() };
    let mut time: *mut _object = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = core::ptr::null_mut::<_object>();
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"O!O!|O:combine".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__5725.0) as *mut *mut i8) as *const *mut i8,
            DATE_TYPE!(),
            core::ptr::addr_of_mut!(date),
            TIME_TYPE!(),
            core::ptr::addr_of_mut!(time),
            core::ptr::addr_of_mut!(tzinfo),
        )
    }) != 0
    {
        if tzinfo.is_null() {
            if (unsafe { (*(time as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
                tzinfo = unsafe { (*(time as *mut __SlateRecord_datetime_h_86)).tzinfo };
            } else {
                tzinfo = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
            }
        }
        result = new_datetime_subclass_fold_ex(
            ((unsafe {
                *((*(date as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(0 as isize)
            }) as i32)
                << 8i32
                | ((unsafe {
                    *((*(date as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(1 as isize)
                }) as i32),
            (unsafe {
                *((*(date as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(2 as isize)
            }) as i32,
            (unsafe {
                *((*(date as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(3 as isize)
            }) as i32,
            (unsafe {
                *((*(time as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(0 as isize)
            }) as i32,
            (unsafe {
                *((*(time as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32,
            (unsafe {
                *((*(time as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(2 as isize)
            }) as i32,
            ((unsafe {
                *((*(time as *mut __SlateRecord_datetime_h_86))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(3 as isize)
            }) as i32)
                << 16i32
                | ((unsafe {
                    *((*(time as *mut __SlateRecord_datetime_h_86))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(4 as isize)
                }) as i32)
                    << 8i32
                | ((unsafe {
                    *((*(time as *mut __SlateRecord_datetime_h_86))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(5 as isize)
                }) as i32),
            tzinfo,
            (unsafe { (*(time as *mut __SlateRecord_datetime_h_86)).fold }) as i32,
            cls,
        );
    }
    result
}

static mut keywords__5725: __SlateAlign16<[*mut i8; 4]> = __SlateAlign16([
    b"date\0".as_ptr() as *mut i8,
    b"time\0".as_ptr() as *mut i8,
    b"tzinfo\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

fn _sanitize_isoformat_str(dtstr: *mut _object) -> *mut _object {
    let len: i64 = unsafe { PyUnicode_GetLength(dtstr) };
    if len < 7 {
        // All valid ISO 8601 strings are at least 7 characters long
        return core::ptr::null_mut::<_object>();
    }
    // `fromisoformat` allows surrogate characters in exactly one position,
    //
    // the separator; to allow datetime_fromisoformat to make the simplifying
    //
    // assumption that all valid strings can be encoded in UTF-8, this function
    //
    // replaces any surrogate character separators with `T`.
    //
    //
    //
    // The result of this, if not NULL, returns a new reference
    let unicode_data: *const () = PyUnicode_DATA!(dtstr) as *const ();
    let kind: i32 = PyUnicode_KIND!(dtstr);
    // Depending on the format of the string, the separator can only ever be
    //
    // in positions 7, 8 or 10. We'll check each of these for a surrogate and
    //
    // if we find one, replace it with `T`. If there is more than one surrogate,
    //
    // we don't have to bother sanitizing it, because the function will later
    //
    // fail when we try to encode the string as ASCII.
    let mut surrogate_separator: u64 = 0;
    for idx in 0..((core::mem::size_of::<[u64; 3]>() as u64) / (core::mem::size_of::<u64>() as u64))
    {
        let pos: u64 = unsafe { potential_separators.0[idx as usize] };
        if pos > (len as u64) {
            break;
        }
        if Py_UNICODE_IS_SURROGATE(PyUnicode_READ__2!(kind, unicode_data, pos)) != 0 {
            surrogate_separator = pos;
            break;
        }
    }
    if surrogate_separator == 0 {
        return Py_NewRef!(dtstr);
    }
    let str_out: *mut _object = unsafe { _PyUnicode_Copy(dtstr) };
    if str_out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyUnicode_WriteChar(str_out, surrogate_separator as i64, 84u32) }) != 0 {
        Py_DECREF!(str_out);
        return core::ptr::null_mut::<_object>();
    }
    str_out
}

static potential_separators: __SlateAlign16<[u64; 3]> = __SlateAlign16([7u64, 8u64, 10u64]);

fn _find_isoformat_datetime_separator(dtstr: *const i8, len: i64) -> i64 {
    // The valid date formats can all be distinguished by characters 4 and 5
    //
    // and further narrowed down by character
    //
    // which tells us where to look for the separator character.
    //
    // Format    |  As-rendered |   Position
    //
    // %Y-%m-%d  |  YYYY-MM-DD  |    10
    //
    // %Y%m%d    |  YYYYMMDD    |     8
    //
    // %Y-W%V    |  YYYY-Www    |     8
    //
    // %YW%V     |  YYYYWww     |     7
    //
    // %Y-W%V-%u |  YYYY-Www-d  |    10
    //
    // %YW%V%u   |  YYYYWwwd    |     8
    //
    // %Y-%j     |  YYYY-DDD    |     8
    //
    // %Y%j      |  YYYYDDD     |     7
    //
    //
    //
    // Note that because we allow *any* character for the separator, in the
    //
    // case where character 4 is W, it's not straightforward to determine where
    //
    // the separator is — in the case of YYYY-Www-d, you have actual ambiguity,
    //
    // e.g. 2020-W01-0000 could be YYYY-Www-D0HH or YYYY-Www-HHMM, when the
    //
    // separator character is a number in the former case or a hyphen in the
    //
    // latter case.
    //
    //
    //
    // The case of YYYYWww can be distinguished from YYYYWwwd by tracking ahead
    //
    // to either the end of the string or the first non-numeric character —
    //
    // since the time components all come in pairs YYYYWww#HH can be
    //
    // distinguished from YYYYWwwd#HH by the fact that there will always be an
    //
    // odd number of digits before the first non-digit character in the former
    //
    // case.
    if len == 7 {
        return 7;
    }
    if ((unsafe { *dtstr.wrapping_offset(4 as isize) }) as i32)
        == ((unsafe { date_separator }) as i32)
    {
        // YYYY-???
        if ((unsafe { *dtstr.wrapping_offset(5 as isize) }) as i32)
            == ((unsafe { week_indicator }) as i32)
        {
            // YYYY-W??
            if len < 8 {
                return 1i32.wrapping_neg() as i64;
            }
            if len > 8
                && ((unsafe { *dtstr.wrapping_offset(8 as isize) }) as i32)
                    == ((unsafe { date_separator }) as i32)
            {
                // YYYY-Www-D (10) or YYYY-Www-HH (8)
                if len == 9 {
                    return 1i32.wrapping_neg() as i64;
                }
                let __v0: bool =
                    len > 10 && is_digit(unsafe { *dtstr.wrapping_offset(10 as isize) }) != 0;
                if __v0 {
                    // This is as far as we'll try to go to resolve the
                    //
                    // ambiguity for the moment — if we have YYYY-Www-##, the
                    //
                    // separator is either a hyphen at 8 or a number at 10.
                    //
                    //
                    //
                    // We'll assume it's a hyphen at 8 because it's way more
                    //
                    // likely that someone will use a hyphen as a separator
                    //
                    // than a number, but at this point it's really best effort
                    //
                    // because this is an extension of the spec anyway.
                    return 8;
                }
                return 10;
            } else {
                // YYYY-Www (8)
                return 8;
            }
        } else {
            // YYYY-MM-DD (10)
            return 10;
        }
    } else {
        // YYYY???
        if ((unsafe { *dtstr.wrapping_offset(4 as isize) }) as i32)
            == ((unsafe { week_indicator }) as i32)
        {
            // YYYYWww (7) or YYYYWwwd (8)
            let mut idx: u64 = 7;
            while idx < (len as u64) {
                // Keep going until we run out of digits.
                if is_digit(unsafe { *dtstr.wrapping_offset(idx as isize) }) == 0 {
                    break;
                }
                idx = idx.wrapping_add(1);
            }
            if idx < 9 {
                return idx as i64;
            }
            if idx % 2 == 0 {
                // If the index of the last number is even, it's YYYYWww
                return 7;
            } else {
                return 8;
            }
        } else {
            // YYYYMMDD (8)
            return 8;
        }
    }
}

static date_separator: i8 = 45;

static week_indicator: i8 = 87;

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_fromisoformat")]
extern "C-unwind" fn datetime_fromisoformat(
    cls: *mut _object,
    dtstr: *mut _object,
) -> *mut _object {
    let mut d_in_month: i32 = unsafe { core::mem::zeroed() };
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    // date runs up to separator_location
    let mut rv: i32 = unsafe { core::mem::zeroed() };
    let mut tzusec: i32 = unsafe { core::mem::zeroed() };
    let mut tzoffset: i32 = unsafe { core::mem::zeroed() };
    let mut microsecond: i32 = unsafe { core::mem::zeroed() };
    let mut second: i32 = unsafe { core::mem::zeroed() };
    let mut minute: i32 = unsafe { core::mem::zeroed() };
    let mut hour: i32 = unsafe { core::mem::zeroed() };
    let mut day: i32 = unsafe { core::mem::zeroed() };
    let mut month: i32 = unsafe { core::mem::zeroed() };
    let mut year: i32 = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut separator_location: i64 = unsafe { core::mem::zeroed() };
    let mut dt_ptr: *const i8 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    // We only need to sanitize this string if the separator is a surrogate
    //
    // character. In the situation where the separator location is ambiguous,
    //
    // we don't have to sanitize it anything because that can only happen when
    //
    // the separator is either '-' or a number. This should mostly be a noop
    //
    // but it makes the reference counting easier if we still sanitize.
    let mut dtstr_clean: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if PyUnicode_Check!(dtstr) == 0 {
            PyErr_SetString(
                PyExc_TypeError,
                c"fromisoformat: argument must be str".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                dtstr_clean = _sanitize_isoformat_str(dtstr);
                if !dtstr_clean.is_null() {
                    dt_ptr = PyUnicode_AsUTF8AndSize(dtstr_clean, core::ptr::addr_of_mut!(len));
                    if dt_ptr.is_null() {
                        if PyErr_ExceptionMatches(PyExc_UnicodeEncodeError) != 0 {
                            // Encoding errors are invalid string errors at this point
                        } else {
                            break '__join_0;
                        }
                    } else {
                        separator_location = _find_isoformat_datetime_separator(dt_ptr, len);
                        p = dt_ptr;
                        year = 0;
                        month = 0;
                        day = 0;
                        hour = 0;
                        minute = 0;
                        second = 0;
                        microsecond = 0;
                        tzoffset = 0;
                        tzusec = 0;
                        rv = parse_isoformat_date(
                            p,
                            separator_location as u64,
                            core::ptr::addr_of_mut!(year),
                            core::ptr::addr_of_mut!(month),
                            core::ptr::addr_of_mut!(day),
                        );
                        if rv == 0 && len > separator_location {
                            // In UTF-8, the length of multi-byte characters is encoded in the MSB
                            let __v20: *const i8 = p;
                            p = __v20.wrapping_offset(separator_location as isize);
                            if (*p.wrapping_offset(0 as isize) as i32) & 128 == 0 {
                                let __v12: *const i8 = p;
                                p = __v12.wrapping_offset(1 as isize);
                            } else {
                                let __t0: i32 = (*p.wrapping_offset(0 as isize) as i32) & 240;
                                if __t0 == 224 {
                                    let __v18: *const i8 = p;
                                    p = __v18.wrapping_offset(3 as isize);
                                } else {
                                    if __t0 == 240 {
                                        let __v16: *const i8 = p;
                                        p = __v16.wrapping_offset(4 as isize);
                                    } else {
                                        let __v14: *const i8 = p;
                                        p = __v14.wrapping_offset(2 as isize);
                                    }
                                }
                            }
                            let __v10: i64 = len;
                            len = __v10.wrapping_sub(p.offset_from(dt_ptr as *const i8) as i64);
                            rv = parse_isoformat_time(
                                p,
                                len as u64,
                                core::ptr::addr_of_mut!(hour),
                                core::ptr::addr_of_mut!(minute),
                                core::ptr::addr_of_mut!(second),
                                core::ptr::addr_of_mut!(microsecond),
                                &mut (*core::ptr::addr_of_mut!(tzoffset)),
                                core::ptr::addr_of_mut!(tzusec),
                            );
                        }
                        if rv >= 0 {
                            tzinfo = tzinfo_from_isoformat_results(rv, tzoffset, tzusec);
                            if tzinfo.is_null() {
                                break '__join_0;
                            } else {
                                if hour == 24 && (month >= 1 && month <= 12) {
                                    d_in_month = days_in_month(year, month);
                                    if day <= d_in_month {
                                        if minute == 0 && second == 0 && microsecond == 0 {
                                            // Calculate midnight of the next day
                                            hour = 0;
                                            day = day.wrapping_add(1);
                                            if day > d_in_month {
                                                day = 1;
                                                month = month.wrapping_add(1);
                                                if month > 12 {
                                                    month = 1;
                                                    year = year.wrapping_add(1);
                                                }
                                            }
                                        } else {
                                            PyErr_SetString(PyExc_ValueError, c"minute, second, and microsecond must be 0 when hour is 24".as_ptr());
                                            Py_DECREF!(tzinfo);
                                            Py_DECREF!(dtstr_clean);
                                            return core::ptr::null_mut::<_object>();
                                        }
                                    }
                                }
                                let dt: *mut _object = new_datetime_subclass_ex(
                                    year,
                                    month,
                                    day,
                                    hour,
                                    minute,
                                    second,
                                    microsecond,
                                    tzinfo,
                                    cls,
                                );
                                Py_DECREF!(tzinfo);
                                Py_DECREF!(dtstr_clean);
                                return dt;
                            }
                        }
                    }
                }
                PyErr_Format(
                    PyExc_ValueError,
                    c"Invalid isoformat string: %R".as_ptr(),
                    dtstr,
                );
            }
            Py_XDECREF!(dtstr_clean);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// Destructor.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_dealloc")]
extern "C-unwind" fn datetime_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
        Py_XDECREF!(unsafe { (*self_).tzinfo });
    }
    unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

// Indirect access to tzinfo methods.
/// These are all METH_NOARGS, so don't need to check the arglist.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_utcoffset")]
extern "C-unwind" fn datetime_utcoffset(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    call_utcoffset(
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        op,
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_dst")]
extern "C-unwind" fn datetime_dst(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    call_dst(
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        op,
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_tzname")]
extern "C-unwind" fn datetime_tzname(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    call_tzname(
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        op,
    )
}

// datetime arithmetic.
/// factor must be 1 (to add) or -1 (to subtract).  The result inherits
/// the tzinfo state of date.
fn add_datetime_timedelta(
    date: *mut __SlateRecord_datetime_h_114,
    delta: *mut __SlateRecord_datetime_h_41,
    factor: i32,
) -> *mut _object {
    // Note that the C-level additions can't overflow, because of
    // invariant bounds on the member values.
    let mut year: i32 = ((unsafe {
        *((*(date as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(0 as isize)
    }) as i32)
        << 8i32
        | ((unsafe {
            *((*(date as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(1 as isize)
        }) as i32);
    let mut month: i32 = (unsafe {
        *((*(date as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(2 as isize)
    }) as i32;
    let mut day: i32 = ((unsafe {
        *((*(date as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(3 as isize)
    }) as i32)
        .wrapping_add(GET_TD_DAYS__2!(delta).wrapping_mul(factor));
    let mut hour: i32 =
        (unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) }) as i32;
    let mut minute: i32 =
        (unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) }) as i32;
    let mut second: i32 =
        ((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) as i32)
            .wrapping_add(GET_TD_SECONDS__2!(delta).wrapping_mul(factor));
    let mut microsecond: i32 =
        (((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) }) as i32)
            << 16i32
            | ((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) })
                as i32)
                << 8i32
            | ((unsafe { *((*date).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) })
                as i32))
            .wrapping_add(GET_TD_MICROSECONDS__2!(delta).wrapping_mul(factor));
    if normalize_datetime(
        core::ptr::addr_of_mut!(year),
        core::ptr::addr_of_mut!(month),
        core::ptr::addr_of_mut!(day),
        core::ptr::addr_of_mut!(hour),
        core::ptr::addr_of_mut!(minute),
        core::ptr::addr_of_mut!(second),
        core::ptr::addr_of_mut!(microsecond),
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    new_datetime_subclass_ex(
        year,
        month,
        day,
        hour,
        minute,
        second,
        microsecond,
        if (unsafe { (*(date as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
            unsafe { (*date).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        Py_TYPE__2!(date) as *mut _object,
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_add")]
extern "C-unwind" fn datetime_add(left: *mut _object, right: *mut _object) -> *mut _object {
    if PyDateTime_Check!(left) != 0 {
        // datetime + ???
        if PyDelta_Check!(right) != 0 {
            return add_datetime_timedelta(
                left as *mut __SlateRecord_datetime_h_114,
                right as *mut __SlateRecord_datetime_h_41,
                1i32,
            );
        }
    // datetime + delta
    } else {
        if PyDelta_Check!(left) != 0 {
            // delta + datetime
            return add_datetime_timedelta(
                right as *mut __SlateRecord_datetime_h_114,
                left as *mut __SlateRecord_datetime_h_41,
                1i32,
            );
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_subtract")]
extern "C-unwind" fn datetime_subtract(left: *mut _object, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    if PyDateTime_Check!(left) != 0 {
        // datetime - ???
        if PyDateTime_Check!(right) != 0 {
            // datetime - datetime
            let mut offset1: *mut _object = unsafe { core::mem::zeroed() };
            let mut offset2: *mut _object = unsafe { core::mem::zeroed() };
            let mut offdiff: *mut _object = core::ptr::null_mut::<_object>();
            let mut delta_d: i32 = 0;
            let mut delta_s: i32 = 0;
            let mut delta_us: i32 = 0;
            if (if _PyDateTime_HAS_TZINFO!(left) != 0 {
                unsafe { (*(left as *mut __SlateRecord_datetime_h_114)).tzinfo }
            } else {
                unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
            }) == if _PyDateTime_HAS_TZINFO!(right) != 0 {
                unsafe { (*(right as *mut __SlateRecord_datetime_h_114)).tzinfo }
            } else {
                unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
            } {
                offset1 = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
                offset2 = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
            } else {
                offset1 = datetime_utcoffset(left, core::ptr::null_mut::<_object>());
                if offset1.is_null() {
                    return core::ptr::null_mut::<_object>();
                }
                offset2 = datetime_utcoffset(right, core::ptr::null_mut::<_object>());
                if offset2.is_null() {
                    Py_DECREF!(offset1);
                    return core::ptr::null_mut::<_object>();
                }
                if ((offset1 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }) as i32)
                    != ((offset2 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }) as i32)
                {
                    unsafe {
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"can't subtract offset-naive and offset-aware datetimes".as_ptr(),
                        )
                    };
                    Py_DECREF!(offset1);
                    Py_DECREF!(offset2);
                    return core::ptr::null_mut::<_object>();
                }
            }
            let __v0: bool = offset1 != offset2 && delta_cmp(offset1, offset2) != 0;
            if __v0 {
                offdiff = delta_subtract(offset1, offset2);
                if offdiff.is_null() {
                    Py_DECREF!(offset1);
                    Py_DECREF!(offset2);
                    return core::ptr::null_mut::<_object>();
                }
            }
            Py_DECREF!(offset1);
            Py_DECREF!(offset2);
            delta_d = ymd_to_ord(
                ((unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize)
                }) as i32)
                    << 8i32
                    | ((unsafe {
                        *((*(left as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize)
                    }) as i32),
                (unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize)
                }) as i32,
                (unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize)
                }) as i32,
            )
            .wrapping_sub(ymd_to_ord(
                ((unsafe {
                    *((*(right as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize)
                }) as i32)
                    << 8i32
                    | ((unsafe {
                        *((*(right as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize)
                    }) as i32),
                (unsafe {
                    *((*(right as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize)
                }) as i32,
                (unsafe {
                    *((*(right as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize)
                }) as i32,
            ));
            // These can't overflow, since the values are
            // normalized.  At most this gives the number of
            // seconds in one day.
            delta_s = ((unsafe {
                *((*(left as *mut __SlateRecord_datetime_h_114))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(4 as isize)
            }) as i32)
                .wrapping_sub(
                    (unsafe {
                        *((*(right as *mut __SlateRecord_datetime_h_114))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(4 as isize)
                    }) as i32,
                )
                .wrapping_mul(3600)
                .wrapping_add(
                    ((unsafe {
                        *((*(left as *mut __SlateRecord_datetime_h_114))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(5 as isize)
                    }) as i32)
                        .wrapping_sub(
                            (unsafe {
                                *((*(right as *mut __SlateRecord_datetime_h_114))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    .wrapping_offset(5 as isize)
                            }) as i32,
                        )
                        .wrapping_mul(60),
                )
                .wrapping_add(
                    ((unsafe {
                        *((*(left as *mut __SlateRecord_datetime_h_114))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(6 as isize)
                    }) as i32)
                        .wrapping_sub(
                            (unsafe {
                                *((*(right as *mut __SlateRecord_datetime_h_114))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    .wrapping_offset(6 as isize)
                            }) as i32,
                        ),
                );
            delta_us = (((unsafe {
                *((*(left as *mut __SlateRecord_datetime_h_114))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(7 as isize)
            }) as i32)
                << 16i32
                | ((unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_114))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(8 as isize)
                }) as i32)
                    << 8i32
                | ((unsafe {
                    *((*(left as *mut __SlateRecord_datetime_h_114))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(9 as isize)
                }) as i32))
                .wrapping_sub(
                    ((unsafe {
                        *((*(right as *mut __SlateRecord_datetime_h_114))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(7 as isize)
                    }) as i32)
                        << 16i32
                        | ((unsafe {
                            *((*(right as *mut __SlateRecord_datetime_h_114))
                                .data
                                .as_mut_ptr() as *mut u8)
                                .wrapping_offset(8 as isize)
                        }) as i32)
                            << 8i32
                        | ((unsafe {
                            *((*(right as *mut __SlateRecord_datetime_h_114))
                                .data
                                .as_mut_ptr() as *mut u8)
                                .wrapping_offset(9 as isize)
                        }) as i32),
                );
            result = new_delta!(delta_d, delta_s, delta_us, 1i32);
            if result.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            if !offdiff.is_null() {
                Py_SETREF!(result, delta_subtract(result, offdiff));
                Py_DECREF!(offdiff);
            }
        } else {
            if PyDelta_Check!(right) != 0 {
                // datetime - delta
                result = add_datetime_timedelta(
                    left as *mut __SlateRecord_datetime_h_114,
                    right as *mut __SlateRecord_datetime_h_41,
                    1i32.wrapping_neg(),
                );
            }
        }
    }
    if result == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_INCREF!(result);
    }
    result
}

// Various ways to turn a datetime into a string.
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_repr")]
extern "C-unwind" fn datetime_repr(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let type_name: *const i8 = unsafe { (*Py_TYPE__2!(self_)).tp_name };
    let mut baserepr: *mut _object = unsafe { core::mem::zeroed() };
    if ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) }) as i32)
        << 16i32
        | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) }) as i32)
            << 8i32
        | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) }) as i32)
        != 0
    {
        baserepr = unsafe {
            PyUnicode_FromFormat(
                c"%s(%d, %d, %d, %d, %d, %d, %d)".as_ptr(),
                type_name,
                (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(0 as isize) as i32)
                    << 8i32
                    | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(1 as isize) as i32),
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(2 as isize) as i32,
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(3 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) as i32,
                (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) as i32)
                    << 16i32
                    | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) as i32)
                        << 8i32
                    | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) as i32),
            )
        };
    } else {
        if (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) != 0 {
            baserepr = unsafe {
                PyUnicode_FromFormat(
                    c"%s(%d, %d, %d, %d, %d, %d)".as_ptr(),
                    type_name,
                    (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize) as i32)
                        << 8i32
                        | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize) as i32),
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize) as i32,
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize) as i32,
                    *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) as i32,
                    *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) as i32,
                    *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) as i32,
                )
            };
        } else {
            baserepr = unsafe {
                PyUnicode_FromFormat(
                    c"%s(%d, %d, %d, %d, %d)".as_ptr(),
                    type_name,
                    (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize) as i32)
                        << 8i32
                        | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize) as i32),
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize) as i32,
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize) as i32,
                    *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) as i32,
                    *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) as i32,
                )
            };
        }
    }
    if !baserepr.is_null() && ((unsafe { (*self_).fold }) as i32) != 0 {
        baserepr = append_keyword_fold(baserepr, (unsafe { (*self_).fold }) as i32);
    }
    if baserepr.is_null()
        || (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) == 0
    {
        return baserepr;
    }
    append_keyword_tzinfo(baserepr, unsafe { (*self_).tzinfo })
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_str")]
extern "C-unwind" fn datetime_str(op: *mut _object) -> *mut _object {
    let space: *mut _object = unsafe { PyUnicode_FromString(c" ".as_ptr()) };
    if space.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let res: *mut _object = PyObject_CallMethodOneArg(
        op,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_isoformat
                    ._ascii
                    .ob_base
            )
        },
        space,
    );
    Py_DECREF!(space);
    res
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_isoformat")]
extern "C-unwind" fn datetime_isoformat(
    op: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut sep: i32 = 84;
    let mut timespec: *mut i8 = core::ptr::null_mut::<i8>();
    let mut buffer: __SlateAlign16<[i8; 100]> = __SlateAlign16([0i8; 100]);
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    let mut us: i32 =
        ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) }) as i32)
            << 16i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) })
                as i32)
                << 8i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) })
                as i32);
    let mut given_spec: u64 = 0;
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"|Cs:isoformat".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__6267.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(sep),
            core::ptr::addr_of_mut!(timespec),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if timespec.is_null()
        || (unsafe {
            strcmp(
                (timespec as *const i8) as *const core::ffi::c_char,
                c"auto".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
    {
        if us == 0 {
            // seconds
            given_spec = 2;
        } else {
            // microseconds
            given_spec = 4;
        }
    } else {
        given_spec = 0;
        while given_spec < Py_ARRAY_LENGTH__3!() {
            if (unsafe {
                strcmp(
                    (timespec as *const i8) as *const core::ffi::c_char,
                    specs__6273.0[given_spec as usize][0] as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                if given_spec == 3 {
                    us /= 1000;
                }
                break;
            }
            given_spec = given_spec.wrapping_add(1);
        }
    }
    if given_spec == Py_ARRAY_LENGTH__3!() {
        unsafe { PyErr_Format(PyExc_ValueError, c"Unknown timespec value".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    } else {
        result = unsafe {
            PyUnicode_FromFormat(
                specs__6273.0[given_spec as usize][1],
                (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(0 as isize) as i32)
                    << 8i32
                    | (*((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(1 as isize) as i32),
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(2 as isize) as i32,
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(3 as isize) as i32,
                sep,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) as i32,
                *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) as i32,
                us,
            )
        };
    }
    if result.is_null()
        || (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) == 0
    {
        return result;
    }
    // We need to append the UTC offset.
    if format_utcoffset(
        buffer.0.as_mut_ptr() as *mut i8,
        core::mem::size_of::<[i8; 100]>() as u64,
        c":".as_ptr(),
        unsafe { (*self_).tzinfo },
        op,
    ) < 0
    {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        PyUnicode_AppendAndDel(
            core::ptr::addr_of_mut!(result),
            PyUnicode_FromString(buffer.0.as_mut_ptr() as *const i8),
        )
    };
    result
}

static mut keywords__6267: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"sep\0".as_ptr() as *mut i8,
    b"timespec\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

static mut specs__6273: __SlateAlign16<[[*const i8; 2]; 5]> = __SlateAlign16([
    [
        (b"hours\0".as_ptr() as *mut i8) as *const i8,
        (b"%04d-%02d-%02d%c%02d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"minutes\0".as_ptr() as *mut i8) as *const i8,
        (b"%04d-%02d-%02d%c%02d:%02d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"seconds\0".as_ptr() as *mut i8) as *const i8,
        (b"%04d-%02d-%02d%c%02d:%02d:%02d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"milliseconds\0".as_ptr() as *mut i8) as *const i8,
        (b"%04d-%02d-%02d%c%02d:%02d:%02d.%03d\0".as_ptr() as *mut i8) as *const i8,
    ],
    [
        (b"microseconds\0".as_ptr() as *mut i8) as *const i8,
        (b"%04d-%02d-%02d%c%02d:%02d:%02d.%06d\0".as_ptr() as *mut i8) as *const i8,
    ],
]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_ctime")]
extern "C-unwind" fn datetime_ctime(op: *mut _object, _unused_dummy: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    format_ctime(
        op,
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) }) as i32,
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) }) as i32,
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) as i32,
    )
}

// Miscellaneous methods.
fn flip_fold(dt: *mut _object) -> *mut _object {
    new_datetime_ex2(
        ((unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(dt as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
        (unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(4 as isize)
        }) as i32,
        (unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(5 as isize)
        }) as i32,
        (unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(6 as isize)
        }) as i32,
        ((unsafe {
            *((*(dt as *mut __SlateRecord_datetime_h_114))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(7 as isize)
        }) as i32)
            << 16i32
            | ((unsafe {
                *((*(dt as *mut __SlateRecord_datetime_h_114))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(8 as isize)
            }) as i32)
                << 8i32
            | ((unsafe {
                *((*(dt as *mut __SlateRecord_datetime_h_114))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(9 as isize)
            }) as i32),
        if (unsafe { (*(dt as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
            unsafe { (*(dt as *mut __SlateRecord_datetime_h_114)).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        ((unsafe { (*(dt as *mut __SlateRecord_datetime_h_114)).fold }) == 0) as i32,
        Py_TYPE!(dt),
    )
}

fn get_flip_fold_offset(dt: *mut _object) -> *mut _object {
    let flip_dt: *mut _object = flip_fold(dt);
    if flip_dt.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = datetime_utcoffset(flip_dt, core::ptr::null_mut::<_object>());
    Py_DECREF!(flip_dt);
    result
}

/// PEP 495 exception: Whenever one or both of the operands in
/// inter-zone comparison is such that its utcoffset() depends
/// on the value of its fold attribute, the result is False.
///
/// Return 1 if exception applies, 0 if not,  and -1 on error.
fn pep495_eq_exception(
    self_: *mut _object,
    other: *mut _object,
    offset_self: *mut _object,
    offset_other: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut flip_offset: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        result = 0;
        flip_offset = get_flip_fold_offset(self_);
        if flip_offset.is_null() {
            return 1i32.wrapping_neg();
        } else {
            if flip_offset != offset_self {
                __v1 = delta_cmp(flip_offset, offset_self) != 0;
            } else {
                __v1 = false;
            }
            if __v1 {
                result = 1;
            } else {
                Py_DECREF!(flip_offset);
                flip_offset = get_flip_fold_offset(other);
                if flip_offset.is_null() {
                    return 1i32.wrapping_neg();
                } else {
                    if flip_offset != offset_other {
                        __v0 = delta_cmp(flip_offset, offset_other) != 0;
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        result = 1;
                    }
                }
            }
            Py_DECREF!(flip_offset);
            return result;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_richcompare")]
extern "C-unwind" fn datetime_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    let mut ex: i32 = unsafe { core::mem::zeroed() };
    let mut ex_1: i32 = unsafe { core::mem::zeroed() };
    let mut delta: *mut __SlateRecord_datetime_h_41 = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    // If they're both naive, or both aware and have the same offsets,
    // we get off cheap.  Note that if they're both naive, offset1 ==
    // offset2 == Py_None at this point.
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut diff: i32 = unsafe { core::mem::zeroed() };
    let mut offset2: *mut _object = unsafe { core::mem::zeroed() };
    let mut offset1: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        if PyDateTime_Check!(other) == 0 {
            return core::ptr::addr_of_mut!(_Py_NotImplementedStruct);
        } else {
            if (if _PyDateTime_HAS_TZINFO!(self_) != 0 {
                (*(self_ as *mut __SlateRecord_datetime_h_114)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            }) == if _PyDateTime_HAS_TZINFO!(other) != 0 {
                (*(other as *mut __SlateRecord_datetime_h_114)).tzinfo
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            } {
                diff = memcmp(
                    (((*(self_ as *mut __SlateRecord_datetime_h_114))
                        .data
                        .as_mut_ptr() as *mut u8) as *const ())
                        as *const core::ffi::c_void,
                    (((*(other as *mut __SlateRecord_datetime_h_114))
                        .data
                        .as_mut_ptr() as *mut u8) as *const ())
                        as *const core::ffi::c_void,
                    10u64 as usize,
                ) as i32;
                return diff_to_bool(diff, op);
            } else {
                offset1 = datetime_utcoffset(self_, core::ptr::null_mut::<_object>());
                if offset1.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    '__join_0: {
                        offset2 = datetime_utcoffset(other, core::ptr::null_mut::<_object>());
                        if !offset2.is_null() {
                            if offset1 == offset2 {
                                __v5 = true;
                            } else {
                                if PyDelta_Check!(offset1) != 0 {
                                    __v4 = PyDelta_Check!(offset2) != 0;
                                } else {
                                    __v4 = false;
                                }
                                if __v4 {
                                    __v3 = delta_cmp(offset1, offset2) == 0;
                                } else {
                                    __v3 = false;
                                }
                                __v5 = __v3;
                            }
                            if __v5 {
                                diff = memcmp(
                                    (((*(self_ as *mut __SlateRecord_datetime_h_114))
                                        .data
                                        .as_mut_ptr()
                                        as *mut u8)
                                        as *const ())
                                        as *const core::ffi::c_void,
                                    (((*(other as *mut __SlateRecord_datetime_h_114))
                                        .data
                                        .as_mut_ptr()
                                        as *mut u8)
                                        as *const ())
                                        as *const core::ffi::c_void,
                                    10u64 as usize,
                                ) as i32;
                                if (op == 2 || op == 3) && diff == 0 {
                                    ex = pep495_eq_exception(self_, other, offset1, offset2);
                                    if ex == 1i32.wrapping_neg() {
                                        break '__join_0;
                                    } else {
                                        if ex != 0 {
                                            diff = 1;
                                        }
                                    }
                                }
                                result = diff_to_bool(diff, op);
                            } else {
                                if offset1 != core::ptr::addr_of_mut!(_Py_NoneStruct)
                                    && offset2 != core::ptr::addr_of_mut!(_Py_NoneStruct)
                                {
                                    // else last "if" handled it
                                    delta = datetime_subtract(self_, other)
                                        as *mut __SlateRecord_datetime_h_41;
                                    if !delta.is_null() {
                                        diff = GET_TD_DAYS__2!(delta);
                                        if diff == 0 {
                                            diff = GET_TD_SECONDS__2!(delta)
                                                | GET_TD_MICROSECONDS__2!(delta);
                                        }
                                        Py_DECREF__2!(delta);
                                        if (op == 2 || op == 3) && diff == 0 {
                                            ex_1 =
                                                pep495_eq_exception(self_, other, offset1, offset2);
                                            if ex_1 == 1i32.wrapping_neg() {
                                                break '__join_0;
                                            } else {
                                                if ex_1 != 0 {
                                                    diff = 1;
                                                }
                                            }
                                        }
                                        result = diff_to_bool(diff, op);
                                    }
                                } else {
                                    if op == 2 {
                                        result = Py_NewRef!(_PyObject_CAST__2!(
                                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                                        ));
                                    } else {
                                        if op == 3 {
                                            result = Py_NewRef!(_PyObject_CAST__2!(
                                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                                            ));
                                        } else {
                                            PyErr_SetString(PyExc_TypeError, c"can't compare offset-naive and offset-aware datetimes".as_ptr());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Py_DECREF!(offset1);
                    Py_XDECREF!(offset2);
                    return result;
                }
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_hash")]
extern "C-unwind" fn datetime_hash(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let mut hash: i64 = unsafe { (*self_).hashcode };
    if hash == (1i32.wrapping_neg() as i64) {
        let mut self0: *mut _object = unsafe { core::mem::zeroed() };
        if (unsafe { (*self_).fold }) != 0 {
            self0 = new_datetime_ex2(
                ((unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize)
                }) as i32)
                    << 8i32
                    | ((unsafe {
                        *((*(self_ as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize)
                    }) as i32),
                (unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize)
                }) as i32,
                (unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize)
                }) as i32,
                (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) })
                    as i32,
                (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) })
                    as i32,
                (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) })
                    as i32,
                ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) })
                    as i32)
                    << 16i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize)
                    }) as i32)
                        << 8i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize)
                    }) as i32),
                if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0 {
                    unsafe { (*self_).tzinfo }
                } else {
                    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
                },
                0i32,
                Py_TYPE__2!(self_),
            );
            if self0.is_null() {
                return 1i32.wrapping_neg() as i64;
            }
        } else {
            self0 = Py_NewRef__2!(self_);
        }
        let offset: *mut _object = datetime_utcoffset(self0, core::ptr::null_mut::<_object>());
        Py_DECREF!(self0);
        if offset.is_null() {
            return 1i32.wrapping_neg() as i64;
        }
        // Reduce this to a hash of another object.
        if offset == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            hash = generic_hash(unsafe { (*self_).data.as_mut_ptr() as *mut u8 }, 10i32);
            FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).hashcode, hash);
        } else {
            let mut days: i32 = 0;
            let mut seconds: i32 = 0;
            days = ymd_to_ord(
                ((unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(0 as isize)
                }) as i32)
                    << 8i32
                    | ((unsafe {
                        *((*(self_ as *mut __SlateRecord_datetime_h_98))
                            .data
                            .as_mut_ptr() as *mut u8)
                            .wrapping_offset(1 as isize)
                    }) as i32),
                (unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(2 as isize)
                }) as i32,
                (unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(3 as isize)
                }) as i32,
            );
            seconds =
                ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) })
                    as i32)
                    .wrapping_mul(3600)
                    .wrapping_add(
                        ((unsafe {
                            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize)
                        }) as i32)
                            .wrapping_mul(60),
                    )
                    .wrapping_add(
                        (unsafe {
                            *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize)
                        }) as i32,
                    );
            let temp1: *mut _object = new_delta!(
                days,
                seconds,
                ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) })
                    as i32)
                    << 16i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize)
                    }) as i32)
                        << 8i32
                    | ((unsafe {
                        *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize)
                    }) as i32),
                1i32
            );
            if temp1.is_null() {
                Py_DECREF!(offset);
                return 1i32.wrapping_neg() as i64;
            }
            let temp2: *mut _object = delta_subtract(temp1, offset);
            Py_DECREF!(temp1);
            if temp2.is_null() {
                Py_DECREF!(offset);
                return 1i32.wrapping_neg() as i64;
            }
            hash = unsafe { PyObject_Hash(temp2) };
            FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).hashcode, hash);
            Py_DECREF!(temp2);
        }
        Py_DECREF!(offset);
    }
    hash
}

// [clinic input]
// datetime.datetime.replace
//
//     year: int(c_default="GET_YEAR(self)") = unchanged
//     month: int(c_default="GET_MONTH(self)") = unchanged
//     day: int(c_default="GET_DAY(self)") = unchanged
//     hour: int(c_default="DATE_GET_HOUR(self)") = unchanged
//     minute: int(c_default="DATE_GET_MINUTE(self)") = unchanged
//     second: int(c_default="DATE_GET_SECOND(self)") = unchanged
//     microsecond: int(c_default="DATE_GET_MICROSECOND(self)") = unchanged
//     tzinfo: object(c_default="HASTZINFO(self) ? ((PyDateTime_DateTime *)self)->tzinfo : Py_None") = unchanged
//     *
//     fold: int(c_default="DATE_GET_FOLD(self)") = unchanged
//
// Return datetime with new specified fields.
// [clinic start generated code]
/// [clinic end generated code: output=00bc96536833fddb input=fd972762d604d3e7]
fn datetime_datetime_replace_impl(
    self_: *mut __SlateRecord_datetime_h_114,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    microsecond: i32,
    tzinfo: *mut _object,
    fold: i32,
) -> *mut _object {
    new_datetime_subclass_fold_ex(
        year,
        month,
        day,
        hour,
        minute,
        second,
        microsecond,
        tzinfo,
        fold,
        Py_TYPE__2!(self_) as *mut _object,
    )
}

fn local_timezone_from_timestamp(timestamp: i64) -> *mut _object {
    let mut zone: *const i8 = unsafe { core::mem::zeroed() };
    let mut nameo: *mut _object = unsafe { core::mem::zeroed() };
    let mut local_time_tm: core::mem::MaybeUninit<slate_support::tm> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut slate_support::tm =
        core::ptr::addr_of_mut!(local_time_tm) as *mut slate_support::tm;
    let mut delta: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        nameo = core::ptr::null_mut::<_object>();
        zone = core::ptr::null::<i8>();
        if _PyTime_localtime(timestamp, core::ptr::addr_of_mut!(*__slate_slot_2)) != 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            zone = (*__slate_slot_2).tm_zone;
            delta = new_delta__2!(0i32, (*__slate_slot_2).tm_gmtoff, 0i32, 1i32);
            if delta.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                '__join_0: {
                    if !zone.is_null() {
                        nameo = PyUnicode_DecodeLocale(zone, c"surrogateescape".as_ptr());
                        if nameo.is_null() {
                            break '__join_0;
                        }
                    }
                    result = new_timezone(delta, nameo);
                    Py_XDECREF!(nameo);
                }
                Py_DECREF!(delta);
                return result;
            }
        }
    }
}

fn local_timezone(utc_time: *mut __SlateRecord_datetime_h_114) -> *mut _object {
    let mut timestamp: i64 = 0;
    let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
    let st: *mut __SlateRecord__datetimemodule_c_54 = GET_CURRENT_STATE!(current_mod);
    if st.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let delta: *mut _object = datetime_subtract(utc_time as *mut _object, CONST_EPOCH!(st));
    RELEASE_CURRENT_STATE!(current_mod);
    if delta.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let one_second: *mut _object = new_delta!(0i32, 1i32, 0i32, 0i32);
    if one_second.is_null() {
        Py_DECREF!(delta);
        return core::ptr::null_mut::<_object>();
    }
    let seconds: *mut _object = divide_timedelta_timedelta(
        delta as *mut __SlateRecord_datetime_h_41,
        one_second as *mut __SlateRecord_datetime_h_41,
    );
    Py_DECREF!(one_second);
    Py_DECREF!(delta);
    if seconds.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    timestamp = unsafe { _PyLong_AsTime_t(seconds) };
    Py_DECREF!(seconds);
    let __v0: bool =
        timestamp == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    local_timezone_from_timestamp(timestamp)
}

fn local_timezone_from_local(local_dt: *mut __SlateRecord_datetime_h_114) -> *mut _object {
    let mut seconds: i64 = 0;
    let mut seconds2: i64 = 0;
    let mut timestamp: i64 = 0;
    let fold: i32 = (unsafe { (*local_dt).fold }) as i32;
    seconds = local_to_seconds(
        ((unsafe {
            *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
        (unsafe { *((*local_dt).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) }) as i32,
        (unsafe { *((*local_dt).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) }) as i32,
        (unsafe { *((*local_dt).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) as i32,
        fold,
    );
    if seconds == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    seconds2 = local_to_seconds(
        ((unsafe {
            *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(local_dt as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
        (unsafe { *((*local_dt).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) }) as i32,
        (unsafe { *((*local_dt).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) }) as i32,
        (unsafe { *((*local_dt).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) as i32,
        (fold == 0) as i32,
    );
    if seconds2 == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    // Detect gap
    if seconds2 != seconds && ((seconds2 > seconds) as i32) == fold {
        seconds = seconds2;
    }
    // XXX: add bounds check
    timestamp = seconds.wrapping_sub(unsafe { epoch });
    local_timezone_from_timestamp(timestamp)
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_astimezone")]
extern "C-unwind" fn datetime_astimezone(
    op: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut self_tzinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut offset: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut __SlateRecord_datetime_h_114 = unsafe { core::mem::zeroed() };
    let mut self_: *mut __SlateRecord_datetime_h_114 = unsafe { core::mem::zeroed() };
    let mut __slate_state: usize = 26usize;
    unsafe {
        self_ = PyDateTime_CAST!(op);
        tzinfo = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"|O:astimezone".as_ptr(),
            (core::ptr::addr_of_mut!(keywords__6733.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(tzinfo),
        ) == 0
        {
            return core::ptr::null_mut::<_object>();
        } else {
            if check_tzinfo_subclass(tzinfo) == 1i32.wrapping_neg() {
                return core::ptr::null_mut::<_object>();
            } else {
                if (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo == 0
                    || (*self_).tzinfo == core::ptr::addr_of_mut!(_Py_NoneStruct)
                {
                    __slate_state = 20;
                } else {
                    self_tzinfo = Py_NewRef!(unsafe { (*self_).tzinfo });
                    __slate_state = 18;
                }
                '__join_14: {
                    loop {
                        match __slate_state {
                            18 => {
                                // Conversion to self's own time zone is a NOP.
                                if self_tzinfo == tzinfo {
                                    Py_DECREF!(self_tzinfo);
                                    return Py_NewRef__2!(self_);
                                } else {
                                    // Convert self to UTC.
                                    offset = call_utcoffset(self_tzinfo, self_ as *mut _object);
                                    Py_DECREF!(self_tzinfo);
                                    if offset.is_null() {
                                        return core::ptr::null_mut::<_object>();
                                    } else {
                                        if offset == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                            Py_DECREF!(offset);
                                            __slate_state = 20;
                                        } else {
                                            break '__join_14;
                                        }
                                    }
                                }
                            }
                            _ => {
                                self_tzinfo = local_timezone_from_local(self_);
                                if self_tzinfo.is_null() {
                                    break;
                                } else {
                                    __slate_state = 18;
                                }
                            }
                        }
                    }
                    return core::ptr::null_mut::<_object>();
                }
                if PyDelta_Check!(offset) == 0 {
                    Py_DECREF!(offset);
                    PyErr_Format(
                        PyExc_TypeError,
                        c"utcoffset() returned %.200s, expected timedelta or None".as_ptr(),
                        (*Py_TYPE!(offset)).tp_name,
                    );
                    return core::ptr::null_mut::<_object>();
                } else {
                    // result = self - offset
                    result = add_datetime_timedelta(
                        self_,
                        offset as *mut __SlateRecord_datetime_h_41,
                        1i32.wrapping_neg(),
                    ) as *mut __SlateRecord_datetime_h_114;
                    Py_DECREF!(offset);
                    if result.is_null() {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        // Make sure result is aware and UTC.
                        if (*(result as *mut __SlateRecord_datetime_h_64)).hastzinfo == 0 {
                            temp = result as *mut _object;
                            result = new_datetime_ex2(
                                (*((*(result as *mut __SlateRecord_datetime_h_98))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    .wrapping_offset(0 as isize)
                                    as i32)
                                    << 8i32
                                    | (*((*(result as *mut __SlateRecord_datetime_h_98))
                                        .data
                                        .as_mut_ptr()
                                        as *mut u8)
                                        .wrapping_offset(1 as isize)
                                        as i32),
                                *((*(result as *mut __SlateRecord_datetime_h_98))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    .wrapping_offset(2 as isize)
                                    as i32,
                                *((*(result as *mut __SlateRecord_datetime_h_98))
                                    .data
                                    .as_mut_ptr() as *mut u8)
                                    .wrapping_offset(3 as isize)
                                    as i32,
                                *((*result).data.as_mut_ptr() as *mut u8)
                                    .wrapping_offset(4 as isize)
                                    as i32,
                                *((*result).data.as_mut_ptr() as *mut u8)
                                    .wrapping_offset(5 as isize)
                                    as i32,
                                *((*result).data.as_mut_ptr() as *mut u8)
                                    .wrapping_offset(6 as isize)
                                    as i32,
                                (*((*result).data.as_mut_ptr() as *mut u8)
                                    .wrapping_offset(7 as isize)
                                    as i32)
                                    << 16i32
                                    | (*((*result).data.as_mut_ptr() as *mut u8)
                                        .wrapping_offset(8 as isize)
                                        as i32)
                                        << 8i32
                                    | (*((*result).data.as_mut_ptr() as *mut u8)
                                        .wrapping_offset(9 as isize)
                                        as i32),
                                CONST_UTC!(),
                                (*result).fold as i32,
                                Py_TYPE__2!(result),
                            )
                                as *mut __SlateRecord_datetime_h_114;
                            Py_DECREF!(temp);
                            if result.is_null() {
                                return core::ptr::null_mut::<_object>();
                            }
                        } else {
                            // Result is already aware - just replace tzinfo.
                            Py_SETREF__2!((*result).tzinfo, Py_NewRef!(CONST_UTC!()));
                        }
                        // Attach new tzinfo and let fromutc() do the rest.
                        if tzinfo == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                            tzinfo = local_timezone(result);
                            if tzinfo.is_null() {
                                Py_DECREF__2!(result);
                                return core::ptr::null_mut::<_object>();
                            }
                        } else {
                            Py_INCREF!(tzinfo);
                        }
                        Py_SETREF__2!((*result).tzinfo, tzinfo);
                        temp = result as *mut _object;
                        result = PyObject_CallMethodOneArg(
                            tzinfo,
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_fromutc
                                    ._ascii
                                    .ob_base
                            ),
                            temp,
                        ) as *mut __SlateRecord_datetime_h_114;
                        Py_DECREF!(temp);
                        return result as *mut _object;
                    }
                }
            }
        }
    }
}

static mut keywords__6733: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"tz\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_timetuple")]
extern "C-unwind" fn datetime_timetuple(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let mut dstflag: i32 = 1i32.wrapping_neg();
    if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0
        && (unsafe { (*self_).tzinfo }) != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
    {
        let dst: *mut _object = call_dst(unsafe { (*self_).tzinfo }, op);
        if dst.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if dst != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            dstflag = delta_bool(dst);
        }
        Py_DECREF!(dst);
    }
    build_struct_time(
        ((unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        }) as i32,
        (unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }) as i32,
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) }) as i32,
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) }) as i32,
        (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) as i32,
        dstflag,
    )
}

fn local_to_seconds(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    fold: i32,
) -> i64 {
    let t: i64 = utc_to_seconds(year, month, day, hour, minute, second);
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut u1: i64 = 0;
    let mut u2: i64 = 0;
    let mut t1: i64 = 0;
    let mut t2: i64 = 0;
    let mut lt: i64 = 0;
    // Our goal is to solve t = local(u) for u.
    lt = local(t);
    if lt == (1i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg() as i64;
    }
    a = lt.wrapping_sub(t);
    u1 = t.wrapping_sub(a);
    t1 = local(u1);
    if t1 == (1i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg() as i64;
    }
    if t1 == t {
        // We found one solution, but it may not be the one we need.
        // Look for an earlier solution (if `fold` is 0), or a
        // later one (if `fold` is 1).
        if fold != 0 {
            u2 = u1.wrapping_add(unsafe { max_fold_seconds });
        } else {
            u2 = u1.wrapping_sub(unsafe { max_fold_seconds });
        }
        lt = local(u2);
        if lt == (1i32.wrapping_neg() as i64) {
            return 1i32.wrapping_neg() as i64;
        }
        b = lt.wrapping_sub(u2);
        if a == b {
            return u1;
        }
    } else {
        b = t1.wrapping_sub(u1);
    }
    u2 = t.wrapping_sub(b);
    t2 = local(u2);
    if t2 == (1i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg() as i64;
    }
    if t2 == t {
        return u2;
    }
    if t1 == t {
        return u1;
    }
    // We have found both offsets a and b, but neither t - a nor t - b is
    // a solution.  This means t is in the gap.
    if fold != 0 {
        Py_MIN!(u1, u2)
    } else {
        Py_MAX!(u1, u2)
    }
}

// date(1970,1,1).toordinal() == 719163
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_timestamp")]
extern "C-unwind" fn datetime_timestamp(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) != 0
        && (unsafe { (*self_).tzinfo }) != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
    {
        let mut current_mod: *mut _object = unsafe { core::mem::zeroed() };
        let st: *mut __SlateRecord__datetimemodule_c_54 = GET_CURRENT_STATE!(current_mod);
        if st.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let delta: *mut _object = datetime_subtract(op, CONST_EPOCH!(st));
        RELEASE_CURRENT_STATE!(current_mod);
        if delta.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        result = delta_total_seconds(delta, core::ptr::null_mut::<_object>());
        Py_DECREF!(delta);
    } else {
        let seconds: i64 = local_to_seconds(
            ((unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(0 as isize)
            }) as i32)
                << 8i32
                | ((unsafe {
                    *((*(self_ as *mut __SlateRecord_datetime_h_98))
                        .data
                        .as_mut_ptr() as *mut u8)
                        .wrapping_offset(1 as isize)
                }) as i32),
            (unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(2 as isize)
            }) as i32,
            (unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(3 as isize)
            }) as i32,
            (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) })
                as i32,
            (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) })
                as i32,
            (unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) })
                as i32,
            (unsafe { (*self_).fold }) as i32,
        );
        if seconds == (1i32.wrapping_neg() as i64) {
            return core::ptr::null_mut::<_object>();
        }
        result = unsafe {
            PyFloat_FromDouble(
                (seconds.wrapping_sub(62135683200i64) as f64)
                    + (((*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize)
                        as i32)
                        << 16i32
                        | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize)
                            as i32)
                            << 8i32
                        | (*((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize)
                            as i32)) as f64)
                        / 1000000.0f64,
            )
        };
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_getdate")]
extern "C-unwind" fn datetime_getdate(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    new_date__2!(
        ((unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(0 as isize)
        }) as i32)
            << 8i32
            | ((unsafe {
                *((*(self_ as *mut __SlateRecord_datetime_h_98))
                    .data
                    .as_mut_ptr() as *mut u8)
                    .wrapping_offset(1 as isize)
            }) as i32),
        unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(2 as isize)
        },
        unsafe {
            *((*(self_ as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(3 as isize)
        }
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_gettime")]
extern "C-unwind" fn datetime_gettime(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    new_time__2!(
        unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) },
        unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) },
        unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) },
        ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) }) as i32)
            << 16i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) })
                as i32)
                << 8i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) })
                as i32),
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) },
        unsafe { (*self_).fold }
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_gettimetz")]
extern "C-unwind" fn datetime_gettimetz(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    new_time__2!(
        unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) },
        unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) },
        unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) },
        ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(7 as isize) }) as i32)
            << 16i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(8 as isize) })
                as i32)
                << 8i32
            | ((unsafe { *((*self_).data.as_mut_ptr() as *mut u8).wrapping_offset(9 as isize) })
                as i32),
        if _PyDateTime_HAS_TZINFO!(self_) != 0 {
            unsafe { (*self_).tzinfo }
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        },
        unsafe { (*self_).fold }
    )
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_utctimetuple")]
extern "C-unwind" fn datetime_utctimetuple(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut hh: i32 = 0;
    let mut mm: i32 = 0;
    let mut ss: i32 = 0;
    let mut utcself: *mut __SlateRecord_datetime_h_114 = unsafe { core::mem::zeroed() };
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    let tzinfo: *mut _object = if _PyDateTime_HAS_TZINFO!(self_) != 0 {
        unsafe { (*self_).tzinfo }
    } else {
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
    };
    if tzinfo == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        utcself = Py_NewRef__2!(self_) as *mut __SlateRecord_datetime_h_114;
    } else {
        let offset: *mut _object = call_utcoffset(tzinfo, self_ as *mut _object);
        if offset.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if offset == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            Py_DECREF!(offset);
            utcself = Py_NewRef__2!(self_) as *mut __SlateRecord_datetime_h_114;
        } else {
            utcself = add_datetime_timedelta(
                self_,
                offset as *mut __SlateRecord_datetime_h_41,
                1i32.wrapping_neg(),
            ) as *mut __SlateRecord_datetime_h_114;
            Py_DECREF!(offset);
            if utcself.is_null() {
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    y = ((unsafe {
        *((*(utcself as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(0 as isize)
    }) as i32)
        << 8i32
        | ((unsafe {
            *((*(utcself as *mut __SlateRecord_datetime_h_98))
                .data
                .as_mut_ptr() as *mut u8)
                .wrapping_offset(1 as isize)
        }) as i32);
    m = (unsafe {
        *((*(utcself as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(2 as isize)
    }) as i32;
    d = (unsafe {
        *((*(utcself as *mut __SlateRecord_datetime_h_98))
            .data
            .as_mut_ptr() as *mut u8)
            .wrapping_offset(3 as isize)
    }) as i32;
    hh = (unsafe { *((*utcself).data.as_mut_ptr() as *mut u8).wrapping_offset(4 as isize) }) as i32;
    mm = (unsafe { *((*utcself).data.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize) }) as i32;
    ss = (unsafe { *((*utcself).data.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize) }) as i32;
    Py_DECREF__2!(utcself);
    build_struct_time(y, m, d, hh, mm, ss, 0i32)
}

// Pickle support, a simple use of __reduce__.
/// Let basestate be the non-tzinfo data string.
/// If tzinfo is None, this returns (basestate,), else (basestate, tzinfo).
/// So it's a tuple in any (non-error) case.
/// __getstate__ isn't exposed.
fn datetime_getstate(self_: *mut __SlateRecord_datetime_h_114, proto: i32) -> *mut _object {
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    let basestate: *mut _object = unsafe {
        PyBytes_FromStringAndSize(
            (((*self_).data.as_mut_ptr() as *mut u8) as *mut i8) as *const i8,
            10i64,
        )
    };
    if !basestate.is_null() {
        if proto > 3 && (unsafe { (*self_).fold }) != 0 {
            let __v0: *mut i8 = PyBytes_AS_STRING!(basestate).wrapping_offset(2 as isize);
            unsafe {
                *__v0 = ((*__v0 as i32) | 128i32) as i8;
            }
        }
        // Set the first bit of the third byte
        if (unsafe { (*(self_ as *mut __SlateRecord_datetime_h_64)).hastzinfo }) == 0
            || (unsafe { (*self_).tzinfo }) == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        {
            result = unsafe { PyTuple_Pack(1i64, basestate) };
        } else {
            result = unsafe { PyTuple_Pack(2i64, basestate, (*self_).tzinfo) };
        }
        Py_DECREF!(basestate);
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_reduce_ex")]
extern "C-unwind" fn datetime_reduce_ex(op: *mut _object, args: *mut _object) -> *mut _object {
    let mut proto: i32 = 0;
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"i:__reduce_ex__".as_ptr(),
            core::ptr::addr_of_mut!(proto),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe {
        Py_BuildValue(
            c"(ON)".as_ptr(),
            Py_TYPE__2!(self_),
            datetime_getstate(self_, proto),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.datetime_reduce")]
extern "C-unwind" fn datetime_reduce(op: *mut _object, _unused_arg: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_datetime_h_114 = PyDateTime_CAST!(op);
    unsafe {
        Py_BuildValue(
            c"(ON)".as_ptr(),
            Py_TYPE__2!(self_),
            datetime_getstate(self_, 2i32),
        )
    }
}

/// Class methods:
/// Instance methods:
static mut datetime_methods: __SlateAlign16<[PyMethodDef; 24]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"now\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(datetime_datetime_now))
            })
        };
        __t0.ml_flags = 146i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(datetime_datetime_now__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"utcnow\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(datetime_utcnow);
        __t1.ml_flags = 20i32;
        __t1.ml_doc = (b"Return a new datetime representing UTC day and time.\0".as_ptr()
            as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"fromtimestamp\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = unsafe {
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
                >(Some(datetime_fromtimestamp))
            })
        };
        __t2.ml_flags = 19i32;
        __t2.ml_doc = (b"timestamp[, tz] -> tz's local time from POSIX timestamp.\0".as_ptr()
            as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"utcfromtimestamp\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(datetime_utcfromtimestamp);
        __t3.ml_flags = 17i32;
        __t3.ml_doc = (b"Construct a naive UTC datetime from a POSIX timestamp.\0".as_ptr()
            as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"strptime\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(datetime_strptime);
        __t4.ml_flags = 17i32;
        __t4.ml_doc =
            (b"string, format -> new datetime parsed from a string (like time.strptime()).\0"
                .as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"combine\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = unsafe {
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
                >(Some(datetime_combine))
            })
        };
        __t5.ml_flags = 19i32;
        __t5.ml_doc = (b"date, time -> datetime with same date and time fields\0".as_ptr()
            as *mut i8) as *const i8;
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"fromisoformat\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(datetime_fromisoformat);
        __t6.ml_flags = 24i32;
        __t6.ml_doc = (b"string -> datetime from a string in most ISO 8601 formats\0".as_ptr()
            as *mut i8) as *const i8;
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"date\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(datetime_getdate);
        __t7.ml_flags = 4i32;
        __t7.ml_doc = (b"Return date object with same year, month and day.\0".as_ptr() as *mut i8)
            as *const i8;
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"time\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(datetime_gettime);
        __t8.ml_flags = 4i32;
        __t8.ml_doc = (b"Return time object with same time but with tzinfo=None.\0".as_ptr()
            as *mut i8) as *const i8;
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"timetz\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(datetime_gettimetz);
        __t9.ml_flags = 4i32;
        __t9.ml_doc =
            (b"Return time object with same time and tzinfo.\0".as_ptr() as *mut i8) as *const i8;
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"ctime\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(datetime_ctime);
        __t10.ml_flags = 4i32;
        __t10.ml_doc = (b"Return ctime() style string.\0".as_ptr() as *mut i8) as *const i8;
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"timetuple\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(datetime_timetuple);
        __t11.ml_flags = 4i32;
        __t11.ml_doc = (b"Return time tuple, compatible with time.localtime().\0".as_ptr()
            as *mut i8) as *const i8;
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"timestamp\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(datetime_timestamp);
        __t12.ml_flags = 4i32;
        __t12.ml_doc = (b"Return POSIX timestamp as float.\0".as_ptr() as *mut i8) as *const i8;
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"utctimetuple\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(datetime_utctimetuple);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = (b"Return UTC time tuple, compatible with time.localtime().\0".as_ptr()
            as *mut i8) as *const i8;
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"isoformat\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = unsafe {
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
                >(Some(datetime_isoformat))
            })
        };
        __t14.ml_flags = 3i32;
        __t14.ml_doc = (b"[sep] -> string in ISO 8601 format, YYYY-MM-DDT[HH[:MM[:SS[.mmm[uuu]]]]][+HH:MM].\nsep is used to separate the year from the time, and defaults to 'T'.\nThe optional argument timespec specifies the number of additional terms\nof the time to include. Valid options are 'auto', 'hours', 'minutes',\n'seconds', 'milliseconds' and 'microseconds'.\n\0".as_ptr() as *mut i8) as *const i8;
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"utcoffset\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = Some(datetime_utcoffset);
        __t15.ml_flags = 4i32;
        __t15.ml_doc = (b"Return self.tzinfo.utcoffset(self).\0".as_ptr() as *mut i8) as *const i8;
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"tzname\0".as_ptr() as *mut i8) as *const i8;
        __t16.ml_meth = Some(datetime_tzname);
        __t16.ml_flags = 4i32;
        __t16.ml_doc = (b"Return self.tzinfo.tzname(self).\0".as_ptr() as *mut i8) as *const i8;
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"dst\0".as_ptr() as *mut i8) as *const i8;
        __t17.ml_meth = Some(datetime_dst);
        __t17.ml_flags = 4i32;
        __t17.ml_doc = (b"Return self.tzinfo.dst(self).\0".as_ptr() as *mut i8) as *const i8;
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = (b"replace\0".as_ptr() as *mut i8) as *const i8;
        __t18.ml_meth = unsafe {
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
                >(Some(datetime_datetime_replace))
            })
        };
        __t18.ml_flags = 130i32;
        __t18.ml_doc =
            unsafe { core::ptr::addr_of!(datetime_datetime_replace__doc__.0) as *const i8 };
        __t18
    },
    {
        let mut __t19: PyMethodDef = unsafe { core::mem::zeroed() };
        __t19.ml_name = (b"__replace__\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(datetime_datetime_replace))
            })
        };
        __t19.ml_flags = 130i32;
        __t19.ml_doc = (b"__replace__($self, /, **changes)\n--\n\nThe same as replace().\0".as_ptr()
            as *mut i8) as *const i8;
        __t19
    },
    {
        let mut __t20: PyMethodDef = unsafe { core::mem::zeroed() };
        __t20.ml_name = (b"astimezone\0".as_ptr() as *mut i8) as *const i8;
        __t20.ml_meth = unsafe {
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
                >(Some(datetime_astimezone))
            })
        };
        __t20.ml_flags = 3i32;
        __t20.ml_doc = (b"tz -> convert to local time in new timezone tz\n\0".as_ptr() as *mut i8)
            as *const i8;
        __t20
    },
    {
        let mut __t21: PyMethodDef = unsafe { core::mem::zeroed() };
        __t21.ml_name = (b"__reduce_ex__\0".as_ptr() as *mut i8) as *const i8;
        __t21.ml_meth = Some(datetime_reduce_ex);
        __t21.ml_flags = 1i32;
        __t21.ml_doc = (b"__reduce_ex__(proto) -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t21
    },
    {
        let mut __t22: PyMethodDef = unsafe { core::mem::zeroed() };
        __t22.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t22.ml_meth = Some(datetime_reduce);
        __t22.ml_flags = 4i32;
        __t22.ml_doc = (b"__reduce__() -> (cls, state)\0".as_ptr() as *mut i8) as *const i8;
        __t22
    },
    {
        let mut __t23: PyMethodDef = unsafe { core::mem::zeroed() };
        __t23.ml_name = core::ptr::null::<i8>();
        __t23.ml_meth = None;
        __t23
    },
]);

static datetime_doc: __SlateAlign16<[i8; 223]> = __SlateAlign16([
    100i8, 97i8, 116i8, 101i8, 116i8, 105i8, 109i8, 101i8, 40i8, 121i8, 101i8, 97i8, 114i8, 44i8,
    32i8, 109i8, 111i8, 110i8, 116i8, 104i8, 44i8, 32i8, 100i8, 97i8, 121i8, 91i8, 44i8, 32i8,
    104i8, 111i8, 117i8, 114i8, 91i8, 44i8, 32i8, 109i8, 105i8, 110i8, 117i8, 116i8, 101i8, 91i8,
    44i8, 32i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 91i8, 44i8, 32i8, 109i8, 105i8, 99i8,
    114i8, 111i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 91i8, 44i8, 116i8, 122i8, 105i8, 110i8,
    102i8, 111i8, 93i8, 93i8, 93i8, 93i8, 93i8, 41i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 121i8,
    101i8, 97i8, 114i8, 44i8, 32i8, 109i8, 111i8, 110i8, 116i8, 104i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 100i8, 97i8, 121i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8,
    32i8, 97i8, 114i8, 101i8, 32i8, 114i8, 101i8, 113i8, 117i8, 105i8, 114i8, 101i8, 100i8, 46i8,
    32i8, 116i8, 122i8, 105i8, 110i8, 102i8, 111i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8,
    32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 111i8, 114i8, 32i8, 97i8, 110i8, 10i8, 105i8,
    110i8, 115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 32i8, 111i8, 102i8, 32i8, 97i8, 32i8, 116i8,
    122i8, 105i8, 110i8, 102i8, 111i8, 32i8, 115i8, 117i8, 98i8, 99i8, 108i8, 97i8, 115i8, 115i8,
    46i8, 32i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 109i8, 97i8, 105i8, 110i8, 105i8, 110i8,
    103i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 109i8, 97i8,
    121i8, 32i8, 98i8, 101i8, 32i8, 105i8, 110i8, 116i8, 115i8, 46i8, 10i8, 0i8,
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
static mut datetime_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = Some(datetime_add);
    __t0.nb_subtract = Some(datetime_subtract);
    __t0.nb_multiply = None;
    __t0.nb_remainder = None;
    __t0.nb_divmod = None;
    __t0.nb_power = None;
    __t0.nb_negative = None;
    __t0.nb_positive = None;
    __t0.nb_absolute = None;
    __t0.nb_bool = None;
    __t0
};

// datetime C-API.
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
/// Indirectly, via the utc object.
static capi_types: __SlateAlign16<[*mut _typeobject; 6]> = __SlateAlign16([
    unsafe { core::ptr::addr_of_mut!(PyDateTime_DateType) },
    unsafe { core::ptr::addr_of_mut!(PyDateTime_DateTimeType) },
    unsafe { core::ptr::addr_of_mut!(PyDateTime_TimeType) },
    unsafe { core::ptr::addr_of_mut!(PyDateTime_DeltaType) },
    unsafe { core::ptr::addr_of_mut!(PyDateTime_TZInfoType) },
    unsafe { core::ptr::addr_of_mut!(PyDateTime_TimeZoneType) },
]);

/// The C-API is process-global.  This violates interpreter isolation
/// due to the objects stored here.  Thus each of those objects must
/// be managed carefully.
///
/// XXX Can we make this const?
/// The classes must be readied before used here.
/// That will happen the first time the module is loaded.
/// They aren't safe to be shared between interpreters,
/// but that's okay as long as the module is single-phase init.
static mut capi: __SlateRecord_datetime_h_186 = __SlateRecord_datetime_h_186 {
    DateType: unsafe { core::ptr::addr_of_mut!(PyDateTime_DateType) },
    DateTimeType: unsafe { core::ptr::addr_of_mut!(PyDateTime_DateTimeType) },
    TimeType: unsafe { core::ptr::addr_of_mut!(PyDateTime_TimeType) },
    DeltaType: unsafe { core::ptr::addr_of_mut!(PyDateTime_DeltaType) },
    TZInfoType: unsafe { core::ptr::addr_of_mut!(PyDateTime_TZInfoType) },
    TimeZone_UTC: (unsafe { core::ptr::addr_of_mut!(utc_timezone) }) as *mut _object,
    Date_FromDate: Some(new_date_ex),
    DateTime_FromDateAndTime: Some(new_datetime_ex),
    Time_FromTime: Some(new_time_ex),
    Delta_FromDelta: Some(new_delta_ex),
    TimeZone_FromTimeZone: Some(new_timezone),
    DateTime_FromTimestamp: Some(datetime_fromtimestamp),
    Date_FromTimestamp: Some(datetime_date_fromtimestamp_capi),
    DateTime_FromDateAndTimeAndFold: Some(new_datetime_ex2),
    Time_FromTimeAndFold: Some(new_time_ex2),
};

/// Get a new C API by calling this function.
/// Clients get at C API via PyDateTime_IMPORT, defined in datetime.h.
fn get_datetime_capi() -> *mut __SlateRecord_datetime_h_186 {
    unsafe { core::ptr::addr_of_mut!(capi) }
}

fn create_timezone_from_delta(days: i32, sec: i32, ms: i32, normalize: i32) -> *mut _object {
    let delta: *mut _object = new_delta!(days, sec, ms, normalize);
    if delta.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let tz: *mut _object = create_timezone(delta, core::ptr::null_mut::<_object>());
    Py_DECREF!(delta);
    tz
}

// Module state lifecycle.
fn init_state(
    st: *mut __SlateRecord__datetimemodule_c_54,
    module: *mut _object,
    old_module: *mut _object,
) -> i32 {
    let mut __slate_compound_1: __SlateRecord__datetimemodule_c_54 = unsafe { core::mem::zeroed() };
    // Each module gets its own heap types.
    ADD_TYPE__4!(
        isocalendar_date_type,
        unsafe { core::ptr::addr_of_mut!(isocal_spec) },
        unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        module,
        st
    );
    if !old_module.is_null() {
        let st_old: *mut __SlateRecord__datetimemodule_c_54 = get_module_state(old_module);
        unsafe {
            *st = *{
                __slate_compound_1 = __SlateRecord__datetimemodule_c_54 {
                    isocalendar_date_type: (*st).isocalendar_date_type,
                    us_per_ms: Py_NewRef!(unsafe { (*st_old).us_per_ms }),
                    us_per_second: Py_NewRef!(unsafe { (*st_old).us_per_second }),
                    us_per_minute: Py_NewRef!(unsafe { (*st_old).us_per_minute }),
                    us_per_hour: Py_NewRef!(unsafe { (*st_old).us_per_hour }),
                    us_per_day: Py_NewRef!(unsafe { (*st_old).us_per_day }),
                    us_per_week: Py_NewRef!(unsafe { (*st_old).us_per_week }),
                    seconds_per_day: Py_NewRef!(unsafe { (*st_old).seconds_per_day }),
                    epoch: Py_NewRef!(unsafe { (*st_old).epoch }),
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            };
        }
        return 0;
    }
    unsafe {
        (*st).us_per_ms = PyLong_FromLong(1000i64);
    }
    if unsafe { (*st).us_per_ms }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*st).us_per_second = PyLong_FromLong(1000000i64);
    }
    if unsafe { (*st).us_per_second }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*st).us_per_minute = PyLong_FromLong(60000000i64);
    }
    if unsafe { (*st).us_per_minute }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*st).seconds_per_day = PyLong_FromLong(86400i64);
    }
    if unsafe { (*st).seconds_per_day }.is_null() {
        return 1i32.wrapping_neg();
    }
    // The rest are too big for 32-bit ints, but even
    // us_per_week fits in 40 bits, so doubles should be exact.
    unsafe {
        (*st).us_per_hour = PyLong_FromDouble(3600000000.0f64);
    }
    if unsafe { (*st).us_per_hour }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*st).us_per_day = PyLong_FromDouble(86400000000.0f64);
    }
    if unsafe { (*st).us_per_day }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*st).us_per_week = PyLong_FromDouble(604800000000.0f64);
    }
    if unsafe { (*st).us_per_week }.is_null() {
        return 1i32.wrapping_neg();
    }
    // Init Unix epoch
    unsafe {
        (*st).epoch = new_datetime!(
            1970i32,
            1i32,
            1i32,
            0i32,
            0i32,
            0i32,
            0i32,
            core::ptr::addr_of_mut!(utc_timezone) as *mut _object,
            0i32
        );
    }
    if unsafe { (*st).epoch }.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

fn traverse_state(
    st: *mut __SlateRecord__datetimemodule_c_54,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    // heap types
    Py_VISIT__2!(unsafe { (*st).isocalendar_date_type }, visit, arg);
    0
}

fn clear_state(st: *mut __SlateRecord__datetimemodule_c_54) -> i32 {
    Py_CLEAR__3!((*st).isocalendar_date_type);
    Py_CLEAR!((*st).us_per_ms);
    Py_CLEAR!((*st).us_per_second);
    Py_CLEAR!((*st).us_per_minute);
    Py_CLEAR!((*st).us_per_hour);
    Py_CLEAR!((*st).us_per_day);
    Py_CLEAR!((*st).us_per_week);
    Py_CLEAR!((*st).seconds_per_day);
    Py_CLEAR!((*st).epoch);
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDateTime_InitTypes(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_19: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_20: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_21: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_3: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_4: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_5: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_6: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_7: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_8: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_9: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_10: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_11: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_12: *mut _object = unsafe { core::mem::zeroed() };
    let mut value_13: *mut _object = unsafe { core::mem::zeroed() };
    // timedelta values
    let mut d: *mut _object = unsafe { core::mem::zeroed() };
    // Bases classes must be initialized before subclasses,
    // so capi_types must have the types in the appropriate order.
    let mut i: u64 = unsafe { core::mem::zeroed() };
    unsafe {
        i = 0;
        while i < Py_ARRAY_LENGTH__8!() {
            let r#type: *mut _typeobject = capi_types.0[i as usize];
            if _PyStaticType_InitForExtension(interp, r#type) < 0 {
                return *{
                    __slate_compound_21 = {
                        let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t2._type = 1u32;
                        __t2.func = _PyStatus_GET_FUNC__37!() as *const i8;
                        __t2.err_msg = c"could not initialize static types".as_ptr();
                        __t2
                    };
                    core::ptr::addr_of_mut!(__slate_compound_21)
                };
            } else {
                i = i.wrapping_add(1);
            }
        }
        d = _PyType_GetDict(core::ptr::addr_of_mut!(PyDateTime_DeltaType));
        '__join_0: {
            '__join_61: {
                loop {
                    value_13 = new_delta!(0i32, 0i32, 1i32, 0i32);
                    if value_13.is_null() {
                        break '__join_0;
                    } else {
                        if PyDict_SetItemString(d, c"resolution".as_ptr(), value_13) < 0 {
                            break '__join_61;
                        } else {
                            Py_DECREF!(value_13);
                            if true {
                                break;
                            }
                        }
                    }
                }
                '__join_57: {
                    loop {
                        value_12 = new_delta!(999999999i32.wrapping_neg(), 0i32, 0i32, 0i32);
                        if value_12.is_null() {
                            break '__join_0;
                        } else {
                            if PyDict_SetItemString(d, c"min".as_ptr(), value_12) < 0 {
                                break '__join_57;
                            } else {
                                Py_DECREF!(value_12);
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    '__join_53: {
                        loop {
                            value_11 = new_delta!(999999999i32, 86399i32, 999999i32, 0i32);
                            if value_11.is_null() {
                                break '__join_0;
                            } else {
                                if PyDict_SetItemString(d, c"max".as_ptr(), value_11) < 0 {
                                    break '__join_53;
                                } else {
                                    Py_DECREF!(value_11);
                                    if true {
                                        break;
                                    }
                                }
                            }
                        }
                        // date values
                        d = _PyType_GetDict(core::ptr::addr_of_mut!(PyDateTime_DateType));
                        '__join_48: {
                            loop {
                                value_10 = new_date!(1i32, 1i32, 1i32);
                                if value_10.is_null() {
                                    break '__join_0;
                                } else {
                                    if PyDict_SetItemString(d, c"min".as_ptr(), value_10) < 0 {
                                        break '__join_48;
                                    } else {
                                        Py_DECREF!(value_10);
                                        if true {
                                            break;
                                        }
                                    }
                                }
                            }
                            '__join_44: {
                                loop {
                                    value_9 = new_date!(9999i32, 12i32, 31i32);
                                    if value_9.is_null() {
                                        break '__join_0;
                                    } else {
                                        if PyDict_SetItemString(d, c"max".as_ptr(), value_9) < 0 {
                                            break '__join_44;
                                        } else {
                                            Py_DECREF!(value_9);
                                            if true {
                                                break;
                                            }
                                        }
                                    }
                                }
                                '__join_40: {
                                    loop {
                                        value_8 = new_delta!(1i32, 0i32, 0i32, 0i32);
                                        if value_8.is_null() {
                                            break '__join_0;
                                        } else {
                                            if PyDict_SetItemString(
                                                d,
                                                c"resolution".as_ptr(),
                                                value_8,
                                            ) < 0
                                            {
                                                break '__join_40;
                                            } else {
                                                Py_DECREF!(value_8);
                                                if true {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    // time values
                                    d = _PyType_GetDict(core::ptr::addr_of_mut!(
                                        PyDateTime_TimeType
                                    ));
                                    '__join_35: {
                                        loop {
                                            value_7 = new_time!(
                                                0i32,
                                                0i32,
                                                0i32,
                                                0i32,
                                                core::ptr::addr_of_mut!(_Py_NoneStruct),
                                                0i32
                                            );
                                            if value_7.is_null() {
                                                break '__join_0;
                                            } else {
                                                if PyDict_SetItemString(d, c"min".as_ptr(), value_7)
                                                    < 0
                                                {
                                                    break '__join_35;
                                                } else {
                                                    Py_DECREF!(value_7);
                                                    if true {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        '__join_31: {
                                            loop {
                                                value_6 = new_time!(
                                                    23i32,
                                                    59i32,
                                                    59i32,
                                                    999999i32,
                                                    core::ptr::addr_of_mut!(_Py_NoneStruct),
                                                    0i32
                                                );
                                                if value_6.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    if PyDict_SetItemString(
                                                        d,
                                                        c"max".as_ptr(),
                                                        value_6,
                                                    ) < 0
                                                    {
                                                        break '__join_31;
                                                    } else {
                                                        Py_DECREF!(value_6);
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            '__join_27: {
                                                loop {
                                                    value_5 = new_delta!(0i32, 0i32, 1i32, 0i32);
                                                    if value_5.is_null() {
                                                        break '__join_0;
                                                    } else {
                                                        if PyDict_SetItemString(
                                                            d,
                                                            c"resolution".as_ptr(),
                                                            value_5,
                                                        ) < 0
                                                        {
                                                            break '__join_27;
                                                        } else {
                                                            Py_DECREF!(value_5);
                                                            if true {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                // datetime values
                                                d = _PyType_GetDict(core::ptr::addr_of_mut!(
                                                    PyDateTime_DateTimeType
                                                ));
                                                '__join_22: {
                                                    loop {
                                                        value_4 = new_datetime!(
                                                            1i32,
                                                            1i32,
                                                            1i32,
                                                            0i32,
                                                            0i32,
                                                            0i32,
                                                            0i32,
                                                            core::ptr::addr_of_mut!(_Py_NoneStruct),
                                                            0i32
                                                        );
                                                        if value_4.is_null() {
                                                            break '__join_0;
                                                        } else {
                                                            if PyDict_SetItemString(
                                                                d,
                                                                c"min".as_ptr(),
                                                                value_4,
                                                            ) < 0
                                                            {
                                                                break '__join_22;
                                                            } else {
                                                                Py_DECREF!(value_4);
                                                                if true {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    '__join_18: {
                                                        loop {
                                                            value_3 = new_datetime!(
                                                                9999i32,
                                                                12i32,
                                                                31i32,
                                                                23i32,
                                                                59i32,
                                                                59i32,
                                                                999999i32,
                                                                core::ptr::addr_of_mut!(
                                                                    _Py_NoneStruct
                                                                ),
                                                                0i32
                                                            );
                                                            if value_3.is_null() {
                                                                break '__join_0;
                                                            } else {
                                                                if PyDict_SetItemString(
                                                                    d,
                                                                    c"max".as_ptr(),
                                                                    value_3,
                                                                ) < 0
                                                                {
                                                                    break '__join_18;
                                                                } else {
                                                                    Py_DECREF!(value_3);
                                                                    if true {
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        '__join_14: {
                                                            loop {
                                                                value_2 = new_delta!(
                                                                    0i32, 0i32, 1i32, 0i32
                                                                );
                                                                if value_2.is_null() {
                                                                    break '__join_0;
                                                                } else {
                                                                    if PyDict_SetItemString(
                                                                        d,
                                                                        c"resolution".as_ptr(),
                                                                        value_2,
                                                                    ) < 0
                                                                    {
                                                                        break '__join_14;
                                                                    } else {
                                                                        Py_DECREF!(value_2);
                                                                        if true {
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            // timezone values
                                                            d = _PyType_GetDict(
                                                                core::ptr::addr_of_mut!(
                                                                    PyDateTime_TimeZoneType
                                                                ),
                                                            );
                                                            if PyDict_SetItemString(
                                                                d,
                                                                c"utc".as_ptr(),
                                                                core::ptr::addr_of_mut!(
                                                                    utc_timezone
                                                                )
                                                                    as *mut _object,
                                                            ) < 0
                                                            {
                                                                break '__join_0;
                                                            } else {
                                                                // bpo-37642: These attributes are rounded to the nearest minute for backwards
                                                                // compatibility, even though the constructor will accept a wider range of
                                                                // values. This may change in the future.
                                                                // -23:59
                                                                '__join_8: {
                                                                    loop {
                                                                        value_1 = create_timezone_from_delta(1i32.wrapping_neg(), 60i32, 0i32, 1i32);
                                                                        if value_1.is_null() {
                                                                            break '__join_0;
                                                                        } else {
                                                                            if PyDict_SetItemString(
                                                                                d,
                                                                                c"min".as_ptr(),
                                                                                value_1,
                                                                            ) < 0
                                                                            {
                                                                                break '__join_8;
                                                                            } else {
                                                                                Py_DECREF!(value_1);
                                                                                if true {
                                                                                    break;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    // +23:59
                                                                    '__join_3: {
                                                                        loop {
                                                                            value = create_timezone_from_delta(0i32, 86340i32, 0i32, 0i32);
                                                                            if value.is_null() {
                                                                                break '__join_0;
                                                                            } else {
                                                                                if PyDict_SetItemString(d, c"max".as_ptr(), value) < 0 {
break '__join_3;
} else {
Py_DECREF!(value);
if true {
break;
}
}
                                                                            }
                                                                        }
                                                                        return *{
                                                                            __slate_compound_20 = {
                                                                                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                                                                __t1._type = 0u32;
                                                                                __t1
                                                                            };
                                                                            core::ptr::addr_of_mut!(
                                                                                __slate_compound_20
                                                                            )
                                                                        };
                                                                    }
                                                                    Py_DECREF!(value);
                                                                    break '__join_0;
                                                                }
                                                                Py_DECREF!(value_1);
                                                                break '__join_0;
                                                            }
                                                        }
                                                        Py_DECREF!(value_2);
                                                        break '__join_0;
                                                    }
                                                    Py_DECREF!(value_3);
                                                    break '__join_0;
                                                }
                                                Py_DECREF!(value_4);
                                                break '__join_0;
                                            }
                                            Py_DECREF!(value_5);
                                            break '__join_0;
                                        }
                                        Py_DECREF!(value_6);
                                        break '__join_0;
                                    }
                                    Py_DECREF!(value_7);
                                    break '__join_0;
                                }
                                Py_DECREF!(value_8);
                                break '__join_0;
                            }
                            Py_DECREF!(value_9);
                            break '__join_0;
                        }
                        Py_DECREF!(value_10);
                        break '__join_0;
                    }
                    Py_DECREF!(value_11);
                    break '__join_0;
                }
                Py_DECREF!(value_12);
                break '__join_0;
            }
            Py_DECREF!(value_13);
        }
        return *{
            __slate_compound_19 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__37!() as *const i8;
                __t0.err_msg = c"memory allocation failed".as_ptr();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_19)
        };
    }
}

// Module methods and initialization.
static mut module_methods: __SlateAlign16<[PyMethodDef; 1]> = __SlateAlign16([{
    let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
    __t0.ml_name = core::ptr::null::<i8>();
    __t0.ml_meth = None;
    __t0
}]);

#[unsafe(link_section = ".text.slate_distinct._datetimemodule._datetime_exec")]
extern "C-unwind" fn _datetime_exec(module: *mut _object) -> i32 {
    // At last, set up and add the encapsulated C API
    let mut capi__0: *mut __SlateRecord_datetime_h_186 = unsafe { core::mem::zeroed() };
    let mut i: u64 = unsafe { core::mem::zeroed() };
    let mut old_module: *mut _object = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    let mut st: *mut __SlateRecord__datetimemodule_c_54 = unsafe { core::mem::zeroed() };
    let mut rc: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            '__join_1: {
                rc = 1i32.wrapping_neg();
                st = get_module_state(module);
                interp = PyInterpreterState_Get();
                if get_current_module(interp, &mut (*core::ptr::addr_of_mut!(old_module))) >= 0 {
                    // We actually set the "current" module right before a successful return.
                    i = 0;
                    while i < Py_ARRAY_LENGTH__8!() {
                        let r#type: *mut _typeobject = capi_types.0[i as usize];
                        let name: *const i8 = _PyType_Name(r#type);
                        if PyModule_AddObjectRef(module, name, r#type as *mut _object) < 0 {
                            break '__join_1;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                    if init_state(st, module, old_module) >= 0 {
                        // Add module level attributes
                        if PyModule_AddIntConstant(module, c"MINYEAR".as_ptr(), 1i64) >= 0 {
                            if PyModule_AddIntConstant(module, c"MAXYEAR".as_ptr(), 9999i64) >= 0 {
                                if PyModule_AddObjectRef(
                                    module,
                                    c"UTC".as_ptr(),
                                    core::ptr::addr_of_mut!(utc_timezone) as *mut _object,
                                ) >= 0
                                {
                                    capi__0 = get_datetime_capi();
                                    if !capi__0.is_null() {
                                        let capsule: *mut _object = PyCapsule_New(
                                            capi__0 as *mut (),
                                            c"datetime.datetime_CAPI".as_ptr(),
                                            None,
                                        );
                                        // (capsule == NULL) is handled by PyModule_Add
                                        if PyModule_Add(module, c"datetime_CAPI".as_ptr(), capsule)
                                            >= 0
                                        {
                                            // A 4-year cycle has an extra leap day over what we'd get from
                                            // pasting together 4 single years.
                                            // Similarly, a 400-year cycle has an extra leap day over what we'd
                                            // get from pasting together 4 100-year cycles.
                                            // OTOH, a 100-year cycle has one fewer leap day than we'd get from
                                            // pasting together 25 4-year cycles.
                                            if set_current_module(interp, module) >= 0 {
                                                rc = 0;
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
            clear_state(st);
        }
        Py_XDECREF!(old_module);
        return rc;
    }
}

static mut module_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(_datetime_exec),
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

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.module_traverse")]
extern "C-unwind" fn module_traverse(
    r#mod: *mut _object,
    visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let st: *mut __SlateRecord__datetimemodule_c_54 = get_module_state(r#mod);
    traverse_state(st, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.module_clear")]
extern "C-unwind" fn module_clear(r#mod: *mut _object) -> i32 {
    let st: *mut __SlateRecord__datetimemodule_c_54 = get_module_state(r#mod);
    clear_state(st);
    let interp: *mut _is = unsafe { PyInterpreterState_Get() };
    clear_current_module(interp, r#mod);
    // The runtime takes care of the static types for us.
    //
    // See _PyTypes_FiniExtTypes()..
    0
}

#[unsafe(link_section = ".text.slate_distinct._datetimemodule.module_free")]
extern "C-unwind" fn module_free(r#mod: *mut ()) {
    module_clear(r#mod as *mut _object);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct._datetimemodule.PyInit__datetime")]
extern "C-unwind" fn PyInit__datetime() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(datetimemodule)) }
}

// Some time zone algebra.  For a datetime x, let
//     x.n = x stripped of its timezone -- its naive time.
//     x.o = x.utcoffset(), and assuming that doesn't raise an exception or
//       return None
//     x.d = x.dst(), and assuming that doesn't raise an exception or
//       return None
//     x.s = x's standard offset, x.o - x.d
//
// Now some derived rules, where k is a duration (timedelta).
//
// 1. x.o = x.s + x.d
//    This follows from the definition of x.s.
//
// 2. If x and y have the same tzinfo member, x.s = y.s.
//    This is actually a requirement, an assumption we need to make about
//    sane tzinfo classes.
//
// 3. The naive UTC time corresponding to x is x.n - x.o.
//    This is again a requirement for a sane tzinfo class.
//
// 4. (x+k).s = x.s
//    This follows from #2, and that datimetimetz+timedelta preserves tzinfo.
//
// 5. (x+k).n = x.n + k
//    Again follows from how arithmetic is defined.
//
// Now we can explain tz.fromutc(x).  Let's assume it's an interesting case
// (meaning that the various tzinfo methods exist, and don't blow up or return
// None when called).
//
// The function wants to return a datetime y with timezone tz, equivalent to x.
// x is already in UTC.
//
// By #3, we want
//
//     y.n - y.o = x.n                             [1]
//
// The algorithm starts by attaching tz to x.n, and calling that y.  So
// x.n = y.n at the start.  Then it wants to add a duration k to y, so that [1]
// becomes true; in effect, we want to solve [2] for k:
//
//    (y+k).n - (y+k).o = x.n                      [2]
//
// By #1, this is the same as
//
//    (y+k).n - ((y+k).s + (y+k).d) = x.n          [3]
//
// By #5, (y+k).n = y.n + k, which equals x.n + k because x.n=y.n at the start.
// Substituting that into [3],
//
//    x.n + k - (y+k).s - (y+k).d = x.n; the x.n terms cancel, leaving
//    k - (y+k).s - (y+k).d = 0; rearranging,
//    k = (y+k).s - (y+k).d; by #4, (y+k).s == y.s, so
//    k = y.s - (y+k).d
//
// On the RHS, (y+k).d can't be computed directly, but y.s can be, and we
// approximate k by ignoring the (y+k).d term at first.  Note that k can't be
// very large, since all offset-returning methods return a duration of magnitude
// less than 24 hours.  For that reason, if y is firmly in std time, (y+k).d must
// be 0, so ignoring it has no consequence then.
//
// In any case, the new value is
//
//     z = y + y.s                                 [4]
//
// It's helpful to step back at look at [4] from a higher level:  it's simply
// mapping from UTC to tz's standard time.
//
// At this point, if
//
//     z.n - z.o = x.n                             [5]
//
// we have an equivalent time, and are almost done.  The insecurity here is
// at the start of daylight time.  Picture US Eastern for concreteness.  The wall
// time jumps from 1:59 to 3:00, and wall hours of the form 2:MM don't make good
// sense then.  The docs ask that an Eastern tzinfo class consider such a time to
// be EDT (because it's "after 2"), which is a redundant spelling of 1:MM EST
// on the day DST starts.  We want to return the 1:MM EST spelling because that's
// the only spelling that makes sense on the local wall clock.
//
// In fact, if [5] holds at this point, we do have the standard-time spelling,
// but that takes a bit of proof.  We first prove a stronger result.  What's the
// difference between the LHS and RHS of [5]?  Let
//
//     diff = x.n - (z.n - z.o)                    [6]
//
// Now
//     z.n =                       by [4]
//     (y + y.s).n =               by #5
//     y.n + y.s =                 since y.n = x.n
//     x.n + y.s =                 since z and y are have the same tzinfo member,
//                                     y.s = z.s by #2
//     x.n + z.s
//
// Plugging that back into [6] gives
//
//     diff =
//     x.n - ((x.n + z.s) - z.o) =     expanding
//     x.n - x.n - z.s + z.o =         cancelling
//     - z.s + z.o =                   by #2
//     z.d
//
// So diff = z.d.
//
// If [5] is true now, diff = 0, so z.d = 0 too, and we have the standard-time
// spelling we wanted in the endcase described above.  We're done.  Contrarily,
// if z.d = 0, then we have a UTC equivalent, and are also done.
//
// If [5] is not true now, diff = z.d != 0, and z.d is the offset we need to
// add to z (in effect, z is in tz's standard time, and we need to shift the
// local clock into tz's daylight time).
//
// Let
//
//     z' = z + z.d = z + diff                     [7]
//
// and we can again ask whether
//
//     z'.n - z'.o = x.n                           [8]
//
// If so, we're done.  If not, the tzinfo class is insane, according to the
// assumptions we've made.  This also requires a bit of proof.  As before, let's
// compute the difference between the LHS and RHS of [8] (and skipping some of
// the justifications for the kinds of substitutions we've done several times
// already):
//
//     diff' = x.n - (z'.n - z'.o) =           replacing z'.n via [7]
//         x.n  - (z.n + diff - z'.o) =    replacing diff via [6]
//         x.n - (z.n + x.n - (z.n - z.o) - z'.o) =
//         x.n - z.n - x.n + z.n - z.o + z'.o =    cancel x.n
//         - z.n + z.n - z.o + z'.o =              cancel z.n
//         - z.o + z'.o =                      #1 twice
//         -z.s - z.d + z'.s + z'.d =          z and z' have same tzinfo
//         z'.d - z.d
//
// So z' is UTC-equivalent to x iff z'.d = z.d at this point.  If they are equal,
// we've found the UTC-equivalent so are done.  In fact, we stop with [7] and
// return z', not bothering to compute z'.d.
//
// How could z.d and z'd differ?  z' = z + z.d [7], so merely moving z' by
// a dst() offset, and starting *from* a time already in DST (we know z.d != 0),
// would have to change the result dst() returns:  we start in DST, and moving
// a little further into it takes us out of DST.
//
// There isn't a sane case where this can happen.  The closest it gets is at
// the end of DST, where there's an hour in UTC with no spelling in a hybrid
// tzinfo class.  In US Eastern, that's 5:MM UTC = 0:MM EST = 1:MM EDT.  During
// that hour, on an Eastern clock 1:MM is taken as being in standard time (6:MM
// UTC) because the docs insist on that, but 0:MM is taken as being in daylight
// time (4:MM UTC).  There is no local time mapping to 5:MM UTC.  The local
// clock jumps from 1:59 back to 1:00 again, and repeats the 1:MM hour in
// standard time.  Since that's what the local clock *does*, we want to map both
// UTC hours 5:MM and 6:MM to 1:MM Eastern.  The result is ambiguous
// in local time, but so it goes -- it's the way the local clock works.
//
// When x = 5:MM UTC is the input to this algorithm, x.o=0, y.o=-5 and y.d=0,
// so z=0:MM.  z.d=60 (minutes) then, so [5] doesn't hold and we keep going.
// z' = z + z.d = 1:MM then, and z'.d=0, and z'.d - z.d = -60 != 0 so [8]
// (correctly) concludes that z' is not UTC-equivalent to x.
//
// Because we know z.d said z was in daylight time (else [5] would have held and
// we would have stopped then), and we know z.d != z'.d (else [8] would have held
// and we would have stopped then), and there are only 2 possible values dst() can
// return in Eastern, it follows that z'.d must be 0 (which it is in the example,
// but the reasoning doesn't depend on the example -- it depends on there being
// two possible dst() outcomes, one zero and the other non-zero).  Therefore
// z' must be in standard time, and is the spelling we want in this case.
//
// Note again that z' is not UTC-equivalent as far as the hybrid tzinfo class is
// concerned (because it takes z' as being in standard time rather than the
// daylight time we intend here), but returning it gives the real-life "local
// clock repeats an hour" behavior when mapping the "unspellable" UTC hour into
// tz.
//
// When the input is 6:MM, z=1:MM and z.d=0, and we stop at once, again with
// the 1:MM standard time spelling we want.
//
// So how can this break?  One of the assumptions must be violated.  Two
// possibilities:
//
// 1) [2] effectively says that y.s is invariant across all y belong to a given
//    time zone.  This isn't true if, for political reasons or continental drift,
//    a region decides to change its base offset from UTC.
//
// 2) There may be versions of "double daylight" time where the tail end of
//    the analysis gives up a step too early.  I haven't thought about that
//    enough to say.
//
// In any case, it's clear that the default fromutc() is strong enough to handle
// "almost all" time zones:  so long as the standard offset is invariant, it
// doesn't matter if daylight time transition points change from year to year, or
// if daylight time is skipped in some years; it doesn't matter how large or
// small dst() may get within its bounds; and it doesn't even matter if some
// perverse time zone returns a negative dst()).  So a breaking case must be
// pretty bizarre, and a tzinfo subclass can override fromutc() if it is.
