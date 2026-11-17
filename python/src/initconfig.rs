//! Version as a string
//! The size of 'void *', as computed by sizeof.
//! Use static_assert() on C11 and newer
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
//! A code object, the __code__ attribute
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! _Py_HasFileSystemDefaultEncodeErrors
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! Total tool ids available
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
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
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
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
// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
macro_rules! CLEAR {
    ($ATTR:expr) => {
        loop {
            unsafe { PyMem_RawFree(($ATTR) as *mut ()) };
            unsafe {
                $ATTR = core::ptr::null_mut::<i32>();
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! COPY_FLAG {
    ($ATTR:ident, $VALUE:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) == 1i32.wrapping_neg() {
            unsafe {
                (*$config).$ATTR = $VALUE;
            }
        }
    };
}

macro_rules! COPY_FLAG__2 {
    ($ATTR:ident, $VAR:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) != 1i32.wrapping_neg() {
            unsafe {
                $VAR = (*$config).$ATTR;
            }
        }
    };
}

macro_rules! COPY_NOT_FLAG {
    ($ATTR:ident, $VALUE:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) == 1i32.wrapping_neg() {
            unsafe {
                (*$config).$ATTR = (($VALUE) == 0) as i32;
            }
        }
    };
}

macro_rules! COPY_NOT_FLAG__2 {
    ($ATTR:ident, $VAR:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) != 1i32.wrapping_neg() {
            unsafe {
                $VAR = ((*$config).$ATTR == 0) as i32;
            }
        }
    };
}

macro_rules! FROM_STRING {
    ($STR:expr) => {{
        let __v: *mut _object = if !$STR.is_null() {
            unsafe { PyUnicode_FromString($STR) }
        } else {
            Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
        };
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

macro_rules! PyDict_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 536870912u64)
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

macro_rules! PyLong_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 16777216u64)
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

macro_rules! Py_SIZE {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST__2!($ob))
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

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
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

macro_rules! _PyStatus_EXCEPTION__2 {
    ($err:expr) => {
        ((unsafe { $err._type }) as u32) != 0
    };
}

macro_rules! _PyStatus_EXCEPTION {
    ($err:expr) => {
        ($err._type as u32) != 0
    };
}

macro_rules! _PyStatus_GET_FUNC__38 {
    () => {
        c"PyInitConfig_AddModule".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__40 {
    () => {
        c"initconfig_set_error".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__41 {
    () => {
        c"core_read_precmdline".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__4 {
    () => {
        c"PyWideStringList_Insert".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__43 {
    () => {
        c"_PyConfig_Read".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__7 {
    () => {
        c"config_parse_cmdline".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__16 {
    () => {
        c"wstr_to_utf8".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__17 {
    () => {
        c"config_read_gil".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__47 {
    () => {
        c"Py_InitializeFromInitConfig".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__49 {
    () => {
        c"config_get_locale_encoding".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__18 {
    () => {
        c"config_init_int_max_str_digits".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__53 {
    () => {
        c"PyInitConfig_GetError".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__55 {
    () => {
        c"_PyConfig_Write".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__59 {
    () => {
        c"config_init_hash_seed".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__22 {
    () => {
        c"config_init_thread_inherit_context".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__64 {
    () => {
        c"_PyConfig_Copy".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__65 {
    () => {
        c"config_init_cpu_count".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__68 {
    () => {
        c"config_init_stdio_encoding".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__24 {
    () => {
        c"utf8_to_wstr".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__71 {
    () => {
        c"PyInitConfig_GetStrList".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__26 {
    () => {
        c"config_update_argv".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__76 {
    () => {
        c"config_init_pycache_prefix".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__28 {
    () => {
        c"config_set_bytes_string".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__77 {
    () => {
        c"config_run_filename_abspath".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__29 {
    () => {
        c"config_init_tracemalloc".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__78 {
    () => {
        c"PyConfig_SetWideStringList".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__81 {
    () => {
        c"initconfig_set_str_list".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__32 {
    () => {
        c"config_init_context_aware_warnings".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__35 {
    () => {
        c"config_init_import_time".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__84 {
    () => {
        c"PyConfig_SetString".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_IS_ERROR {
    ($err:expr) => {
        ($err._type as u32) == 1
    };
}

macro_rules! _PyStatus_IS_ERROR__2 {
    ($err:expr) => {
        ((unsafe { $err._type }) as u32) == 1
    };
}

macro_rules! _PyStatus_IS_EXIT {
    ($err:expr) => {
        ($err._type as u32) == 2
    };
}

macro_rules! _PyStatus_IS_EXIT__2 {
    ($err:expr) => {
        ((unsafe { $err._type }) as u32) == 2
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

unsafe extern "C" {
    static mut PyBool_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_TrueStruct: _longobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyList_Type: _typeobject;
    static mut PyDict_Type: _typeobject;
    static mut Py_FileSystemDefaultEncoding: *const i8;
    static mut Py_FileSystemDefaultEncodeErrors: *const i8;
    static mut Py_HasFileSystemDefaultEncoding: i32;
    static mut _Py_HasFileSystemDefaultEncodeErrors: i32;
    static mut _PyOS_optind: i64;
    static mut _PyOS_optarg: *const i32;
    static mut _PyRuntime: pyruntimestate;
    static mut _Py_HashSecret: __SlateRecord_pycore_pyhash_h_66;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    fn wcscmp(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcschr(arg0: *const i32, arg1: i32) -> *mut i32;
    fn wcstok(arg0: *mut i32, arg1: *const i32, arg2: *mut *mut i32) -> *mut i32;
    #[link_name = "__isoc23_wcstol"]
    fn wcstol(arg0: *const i32, arg1: *mut *mut i32, arg2: i32) -> i64;
    fn setvbuf(arg0: *mut slate_support::FILE, arg1: *mut i8, arg2: i32, arg3: u64) -> i32;
    #[link_name = "__isoc23_strtoul"]
    fn strtoul(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> u64;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Free(arg0: *mut ());
    fn PyMem_RawMalloc(arg0: u64) -> *mut ();
    fn PyMem_RawRealloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyMem_RawFree(arg0: *mut ());
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_FromWideChar(arg0: *const i32, arg1: i64) -> *mut _object;
    fn PyUnicode_AsWideCharString(arg0: *mut _object, arg1: *mut i64) -> *mut i32;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyOS_snprintf(arg0: *mut i8, arg1: u64, arg2: *const i8, ...) -> i32;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLong(arg0: u64) -> *mut _object;
    fn PyLong_AsUnsignedLong(arg0: *mut _object) -> u64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyDict_GetItemStringRef(arg0: *mut _object, arg1: *const i8, arg2: *mut *mut _object)
    -> i32;
    fn PyFrozenSet_New(arg0: *mut _object) -> *mut _object;
    fn PyPreConfig_InitIsolatedConfig(arg0: *mut PyPreConfig);
    fn Py_GetVersion() -> *const i8;
    fn Py_PreInitializeFromArgs(
        arg0: *const PyPreConfig,
        arg1: i64,
        arg2: *mut *mut i32,
    ) -> __SlateRecord_initconfig_h_19;
    fn Py_InitializeFromConfig(arg0: *const PyConfig) -> __SlateRecord_initconfig_h_19;
    fn PySys_SetObject(arg0: *const i8, arg1: *mut _object) -> i32;
    fn PySys_WriteStderr(arg0: *const i8, ...);
    fn PySys_FormatStderr(arg0: *const i8, ...);
    fn PySys_Audit(arg0: *const i8, arg1: *const i8, ...) -> i32;
    fn PyImport_ExtendInittab(arg0: *mut _inittab) -> i32;
    fn Py_DecodeLocale(arg0: *const i8, arg1: *mut u64) -> *mut i32;
    fn _PyMem_RawStrdup(arg0: *const i8) -> *mut i8;
    fn _PyMem_RawWcsdup(arg0: *const i32) -> *mut i32;
    fn _PyMem_DefaultRawMalloc(arg0: u64) -> *mut ();
    fn _PyMem_DefaultRawFree(arg0: *mut ());
    fn _PyMem_DefaultRawWcsdup(arg0: *const i32) -> *mut i32;
    fn _PyErr_GetRaisedException(arg0: *mut _ts) -> *mut _object;
    fn _PyErr_SetRaisedException(arg0: *mut _ts, arg1: *mut _object);
    fn _Py_DecodeUTF8Ex(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut *mut i32,
        arg3: *mut u64,
        arg4: *mut *const i8,
        arg5: u32,
    ) -> i32;
    fn _Py_EncodeUTF8Ex(
        arg0: *const i32,
        arg1: *mut *mut i8,
        arg2: *mut u64,
        arg3: *mut *const i8,
        arg4: i32,
        arg5: u32,
    ) -> i32;
    fn _Py_GetForceASCII() -> i32;
    fn _Py_GetLocaleEncoding() -> *mut i32;
    fn _Py_isabs(arg0: *const i32) -> i32;
    fn _Py_abspath(arg0: *const i32, arg1: *mut *mut i32) -> i32;
    fn _PyOS_ResetGetOpt();
    fn _PyOS_GetOpt(arg0: i64, arg1: *const *mut i32, arg2: *mut i32) -> i32;
    fn _PyArgv_AsWstrList(
        arg0: *const _PyArgv,
        arg1: *mut __SlateRecord_initconfig_h_36,
    ) -> __SlateRecord_initconfig_h_19;
    fn _Py_str_to_int(arg0: *const i8, arg1: *mut i32) -> i32;
    fn _Py_get_xoption(arg0: *const __SlateRecord_initconfig_h_36, arg1: *const i32) -> *const i32;
    fn _Py_GetEnv(arg0: i32, arg1: *const i8) -> *const i8;
    fn _Py_get_env_flag(arg0: i32, arg1: *mut i32, arg2: *const i8);
    fn _PyPreCmdline_Clear(arg0: *mut __SlateRecord_pycore_initconfig_h_108);
    fn _PyPreCmdline_SetConfig(
        arg0: *const __SlateRecord_pycore_initconfig_h_108,
        arg1: *mut PyConfig,
    ) -> __SlateRecord_initconfig_h_19;
    fn _PyPreCmdline_Read(
        arg0: *mut __SlateRecord_pycore_initconfig_h_108,
        arg1: *const PyPreConfig,
    ) -> __SlateRecord_initconfig_h_19;
    fn _PyPreConfig_InitFromPreConfig(
        arg0: *mut PyPreConfig,
        arg1: *const PyPreConfig,
    ) -> __SlateRecord_initconfig_h_19;
    fn _PyPreConfig_AsDict(arg0: *const PyPreConfig) -> *mut _object;
    fn _PyPreConfig_GetConfig(arg0: *mut PyPreConfig, arg1: *const PyConfig);
    fn _PyConfig_InitPathConfig(arg0: *mut PyConfig, arg1: i32) -> __SlateRecord_initconfig_h_19;
    fn _PyInterpreterState_GetConfig(arg0: *mut _is) -> *const PyConfig;
    fn _Py_IsLocaleCoercionTarget(arg0: *const i8) -> i32;
    fn _PySys_ReadPreinitWarnOptions(
        arg0: *mut __SlateRecord_initconfig_h_36,
    ) -> __SlateRecord_initconfig_h_19;
    fn _PySys_ReadPreinitXOptions(arg0: *mut PyConfig) -> __SlateRecord_initconfig_h_19;
    fn _Py_PreInitializeFromConfig(
        arg0: *const PyConfig,
        arg1: *const _PyArgv,
    ) -> __SlateRecord_initconfig_h_19;
    fn _Py_GetConfig() -> *const PyConfig;
    fn _PySys_GetOptionalAttrString(arg0: *const i8, arg1: *mut *mut _object) -> i32;
    fn _PySys_GetRequiredAttrString(arg0: *const i8) -> *mut _object;
    fn _PySys_SetFlagObj(arg0: i64, arg1: *mut _object) -> i32;
    fn _PySys_SetIntMaxStrDigits(arg0: i32) -> i32;
    fn __builtin_unreachable();
}

unsafe extern "C" {
    fn fflush(_0: *mut slate_support::FILE) -> i32;
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

fn putchar(__c: i32) -> i32 {
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let __slate_stdout_writer_tail = unsafe { slate_support::putc(__c, slate_support::stdout()) };
    unsafe { fflush(core::ptr::null_mut()) };
    __slate_stdout_writer_tail
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits136U0,
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits187U0,
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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

// other fields are set to 0
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyArgv {
    argc: i64,
    use_bytes_argv: i32,
    bytes_argv: *const *mut i8,
    wchar_argv: *const *mut i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_initconfig_h_108 {
    argv: __SlateRecord_initconfig_h_36,
    xoptions: __SlateRecord_initconfig_h_36,
    isolated: i32,
    use_environment: i32,
    dev_mode: i32,
    warn_default_encoding: i32,
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_Copy(
    config: *mut PyConfig,
    config2: *const PyConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_5: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    PyConfig_Clear(config);
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut spec: *const __SlateRecord_initconfig_c_99 =
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 };
    '__slate_break_2: while !unsafe { (*spec).name }.is_null() {
        let member: *mut () = config_get_spec_member(config as *const PyConfig, spec);
        let member2: *const () =
            config_get_spec_member(config2 as *const PyConfig, spec) as *const ();
        match (unsafe { (*spec).r#type }) as u32 {
            0 | 1 | 3 => unsafe {
                *(member as *mut i32) = *(member2 as *mut i32);
            },
            2 => unsafe {
                *(member as *mut u64) = *(member2 as *mut u64);
            },
            10 | 11 => {
                let str: *const i32 = unsafe { *(member2 as *mut *const i32) };
                status = PyConfig_SetString(config, member as *mut *mut i32, str);
                if _PyStatus_EXCEPTION!(status) {
                    return status;
                }
            }
            12 => {
                if _PyWideStringList_Copy(
                    member as *mut __SlateRecord_initconfig_h_36,
                    member2 as *const __SlateRecord_initconfig_h_36,
                ) < 0
                {
                    return unsafe {
                        *{
                            __slate_compound_1 = {
                                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t0._type = 1u32;
                                __t0.func = _PyStatus_GET_FUNC__64!() as *const i8;
                                __t0.err_msg = c"memory allocation failed".as_ptr();
                                __t0
                            };
                            core::ptr::addr_of_mut!(__slate_compound_1)
                        }
                    };
                }
            }
            _ => {
                Py_UNREACHABLE!();
            }
        }
        let __v3: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v3.wrapping_offset(1 as isize);
    }
    return unsafe {
        *{
            __slate_compound_5 = {
                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t1._type = 0u32;
                __t1
            };
            core::ptr::addr_of_mut!(__slate_compound_5)
        }
    };
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct _obmalloc_state {
    pools: _obmalloc_pools,
    mgmt: _obmalloc_mgmt,
    usage: _obmalloc_usage,
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
struct __SlateRecord_pycore_asdl_h_45 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [i32; 1],
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
struct __SlateRecord_pycore_ast_h_74 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _expr; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pyhash_h_66_2 {
    prefix: i64,
    suffix: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pyhash_h_66_3 {
    k0: u64,
    k1: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pyhash_h_66_4 {
    padding: [u8; 16],
    suffix: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pyhash_h_66_5 {
    hashsalt16: [u8; 16],
    hashsalt: i64,
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
    pub struct __SlateBits187U0 {
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
    pub struct __SlateBits136U0 {
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
union __SlateRecord_pycore_pyhash_h_66 {
    uc: [u8; 24],
    fnv: __SlateRecord_pycore_pyhash_h_66_2,
    siphash: __SlateRecord_pycore_pyhash_h_66_3,
    djbx33a: __SlateRecord_pycore_pyhash_h_66_4,
    expat: __SlateRecord_pycore_pyhash_h_66_5,
}

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

fn _config_dict_get(dict: *mut _object, name: *const i8, p_item: &mut *mut _object) -> i32 {
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyDict_GetItemStringRef(dict, name, core::ptr::addr_of_mut!(item)) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if item.is_null() {
        // We do not set an exception.
        return 1i32.wrapping_neg();
    }
    unsafe {
        *(p_item as *mut *mut _object) = item;
    }
    0
}

fn config_dict_get(dict: *mut _object, name: *const i8) -> *mut _object {
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    if _config_dict_get(dict, name, unsafe { &mut (*core::ptr::addr_of_mut!(item)) }) < 0 {
        if unsafe { PyErr_Occurred() }.is_null() {
            unsafe { PyErr_Format(PyExc_ValueError, c"missing config key: %s".as_ptr(), name) };
        }
        return core::ptr::null_mut::<_object>();
    }
    item
}

fn config_dict_invalid_type(name: *const i8) {
    unsafe { PyErr_Format(PyExc_TypeError, c"invalid config type: %s".as_ptr(), name) };
}

// _PyOS_GetOpt()
// _PyStatus_OK()
// _PyInterpreterState.runtime
// _PY_LONG_MAX_STR_DIGITS_THRESHOLD
// _Py_path_config
// _PyErr_GetRaisedException()
// _Py_PreInitializeFromConfig()
// _PyMem_DefaultRawMalloc()
// _Py_HashSecret
// _PyThreadState_GET()
// _Py_StatsOn()
// _PySys_SetIntMaxStrDigits()
// DELIM
// setlocale()
// getenv()
// --- PyConfig setters -------------------------------------------
fn config_sys_flag_long(value: i32) -> *mut _object {
    unsafe { PyLong_FromLong(value as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.initconfig.config_sys_flag_not")]
extern "C-unwind" fn config_sys_flag_not(mut value: i32) -> *mut _object {
    value = (value == 0) as i32;
    config_sys_flag_long(value)
}

// --- PyConfig spec ----------------------------------------------
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_c_85 {
    attr: *const i8,
    flag_index: i32,
    flag_setter: Option<unsafe extern "C-unwind" fn(i32) -> *mut _object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_c_90 {
    ptr: *mut i32,
    not: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_c_99 {
    name: *const i8,
    offset: u64,
    r#type: u32,
    visibility: u32,
    sys: __SlateRecord_initconfig_c_85,
    global_var: __SlateRecord_initconfig_c_90,
}

// Ignore deprecations on global variables such as Py_IsolatedFlag
// Update _test_embed_set_config when adding new members
/// --- Public options -----------
/// --- Read-only options -----------
/// sys.flags.dev_mode
/// sys.flags.isolated
/// sys.flags.no_site
/// sys.flags.no_user_site
/// --- Init-only options -----------
/// Array terminator
static PYCONFIG_SPEC: __SlateAlign16<[__SlateRecord_initconfig_c_99; 70]> = __SlateAlign16([
    __SlateRecord_initconfig_c_99 {
        name: (b"argv\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, argv) as u64,
        r#type: 12u32,
        visibility: 2u32,
        sys: {
            let mut __t0: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t0.attr = (b"argv\0".as_ptr() as *mut i8) as *const i8;
            __t0.flag_index = 1i32.wrapping_neg();
            __t0.flag_setter = None;
            __t0
        },
        global_var: {
            let mut __t1: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t1.ptr = core::ptr::null_mut::<i32>();
            __t1.not = 0i32;
            __t1
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"base_exec_prefix\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, base_exec_prefix) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t2: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t2.attr = (b"base_exec_prefix\0".as_ptr() as *mut i8) as *const i8;
            __t2.flag_index = 1i32.wrapping_neg();
            __t2.flag_setter = None;
            __t2
        },
        global_var: {
            let mut __t3: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t3.ptr = core::ptr::null_mut::<i32>();
            __t3.not = 0i32;
            __t3
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"base_executable\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, base_executable) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t4: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t4.attr = (b"_base_executable\0".as_ptr() as *mut i8) as *const i8;
            __t4.flag_index = 1i32.wrapping_neg();
            __t4.flag_setter = None;
            __t4
        },
        global_var: {
            let mut __t5: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t5.ptr = core::ptr::null_mut::<i32>();
            __t5.not = 0i32;
            __t5
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"base_prefix\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, base_prefix) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t6: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t6.attr = (b"base_prefix\0".as_ptr() as *mut i8) as *const i8;
            __t6.flag_index = 1i32.wrapping_neg();
            __t6.flag_setter = None;
            __t6
        },
        global_var: {
            let mut __t7: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t7.ptr = core::ptr::null_mut::<i32>();
            __t7.not = 0i32;
            __t7
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"bytes_warning\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, bytes_warning) as u64,
        r#type: 1u32,
        visibility: 2u32,
        sys: {
            let mut __t8: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t8.attr = core::ptr::null::<i8>();
            __t8.flag_index = 9i32;
            __t8.flag_setter = None;
            __t8
        },
        global_var: {
            let mut __t9: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t9.ptr = unsafe { core::ptr::addr_of_mut!(Py_BytesWarningFlag) };
            __t9.not = 0i32;
            __t9
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"cpu_count\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, cpu_count) as u64,
        r#type: 0u32,
        visibility: 2u32,
        sys: {
            let mut __t10: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t10.attr = core::ptr::null::<i8>();
            __t10.flag_index = 1i32.wrapping_neg();
            __t10.flag_setter = None;
            __t10
        },
        global_var: {
            let mut __t11: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t11.ptr = core::ptr::null_mut::<i32>();
            __t11.not = 0i32;
            __t11
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"exec_prefix\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, exec_prefix) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t12: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t12.attr = (b"exec_prefix\0".as_ptr() as *mut i8) as *const i8;
            __t12.flag_index = 1i32.wrapping_neg();
            __t12.flag_setter = None;
            __t12
        },
        global_var: {
            let mut __t13: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t13.ptr = core::ptr::null_mut::<i32>();
            __t13.not = 0i32;
            __t13
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"executable\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, executable) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t14: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t14.attr = (b"executable\0".as_ptr() as *mut i8) as *const i8;
            __t14.flag_index = 1i32.wrapping_neg();
            __t14.flag_setter = None;
            __t14
        },
        global_var: {
            let mut __t15: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t15.ptr = core::ptr::null_mut::<i32>();
            __t15.not = 0i32;
            __t15
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"inspect\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, inspect) as u64,
        r#type: 3u32,
        visibility: 2u32,
        sys: {
            let mut __t16: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t16.attr = core::ptr::null::<i8>();
            __t16.flag_index = 1i32;
            __t16.flag_setter = None;
            __t16
        },
        global_var: {
            let mut __t17: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t17.ptr = unsafe { core::ptr::addr_of_mut!(Py_InspectFlag) };
            __t17.not = 0i32;
            __t17
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"int_max_str_digits\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, int_max_str_digits) as u64,
        r#type: 1u32,
        visibility: 2u32,
        sys: {
            let mut __t18: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t18.attr = core::ptr::null::<i8>();
            __t18.flag_index = 1i32.wrapping_neg();
            __t18.flag_setter = None;
            __t18
        },
        global_var: {
            let mut __t19: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t19.ptr = core::ptr::null_mut::<i32>();
            __t19.not = 0i32;
            __t19
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"interactive\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, interactive) as u64,
        r#type: 3u32,
        visibility: 2u32,
        sys: {
            let mut __t20: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t20.attr = core::ptr::null::<i8>();
            __t20.flag_index = 2i32;
            __t20.flag_setter = None;
            __t20
        },
        global_var: {
            let mut __t21: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t21.ptr = unsafe { core::ptr::addr_of_mut!(Py_InteractiveFlag) };
            __t21.not = 0i32;
            __t21
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"module_search_paths\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, module_search_paths) as u64,
        r#type: 12u32,
        visibility: 2u32,
        sys: {
            let mut __t22: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t22.attr = (b"path\0".as_ptr() as *mut i8) as *const i8;
            __t22.flag_index = 1i32.wrapping_neg();
            __t22.flag_setter = None;
            __t22
        },
        global_var: {
            let mut __t23: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t23.ptr = core::ptr::null_mut::<i32>();
            __t23.not = 0i32;
            __t23
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"optimization_level\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, optimization_level) as u64,
        r#type: 1u32,
        visibility: 2u32,
        sys: {
            let mut __t24: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t24.attr = core::ptr::null::<i8>();
            __t24.flag_index = 3i32;
            __t24.flag_setter = None;
            __t24
        },
        global_var: {
            let mut __t25: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t25.ptr = unsafe { core::ptr::addr_of_mut!(Py_OptimizeFlag) };
            __t25.not = 0i32;
            __t25
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"parser_debug\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, parser_debug) as u64,
        r#type: 3u32,
        visibility: 2u32,
        sys: {
            let mut __t26: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t26.attr = core::ptr::null::<i8>();
            __t26.flag_index = 0i32;
            __t26.flag_setter = None;
            __t26
        },
        global_var: {
            let mut __t27: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t27.ptr = unsafe { core::ptr::addr_of_mut!(Py_DebugFlag) };
            __t27.not = 0i32;
            __t27
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"platlibdir\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, platlibdir) as u64,
        r#type: 10u32,
        visibility: 2u32,
        sys: {
            let mut __t28: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t28.attr = (b"platlibdir\0".as_ptr() as *mut i8) as *const i8;
            __t28.flag_index = 1i32.wrapping_neg();
            __t28.flag_setter = None;
            __t28
        },
        global_var: {
            let mut __t29: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t29.ptr = core::ptr::null_mut::<i32>();
            __t29.not = 0i32;
            __t29
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"prefix\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, prefix) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t30: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t30.attr = (b"prefix\0".as_ptr() as *mut i8) as *const i8;
            __t30.flag_index = 1i32.wrapping_neg();
            __t30.flag_setter = None;
            __t30
        },
        global_var: {
            let mut __t31: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t31.ptr = core::ptr::null_mut::<i32>();
            __t31.not = 0i32;
            __t31
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"pycache_prefix\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, pycache_prefix) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t32: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t32.attr = (b"pycache_prefix\0".as_ptr() as *mut i8) as *const i8;
            __t32.flag_index = 1i32.wrapping_neg();
            __t32.flag_setter = None;
            __t32
        },
        global_var: {
            let mut __t33: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t33.ptr = core::ptr::null_mut::<i32>();
            __t33.not = 0i32;
            __t33
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"quiet\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, quiet) as u64,
        r#type: 3u32,
        visibility: 2u32,
        sys: {
            let mut __t34: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t34.attr = core::ptr::null::<i8>();
            __t34.flag_index = 10i32;
            __t34.flag_setter = None;
            __t34
        },
        global_var: {
            let mut __t35: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t35.ptr = unsafe { core::ptr::addr_of_mut!(Py_QuietFlag) };
            __t35.not = 0i32;
            __t35
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"stdlib_dir\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, stdlib_dir) as u64,
        r#type: 11u32,
        visibility: 2u32,
        sys: {
            let mut __t36: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t36.attr = (b"_stdlib_dir\0".as_ptr() as *mut i8) as *const i8;
            __t36.flag_index = 1i32.wrapping_neg();
            __t36.flag_setter = None;
            __t36
        },
        global_var: {
            let mut __t37: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t37.ptr = core::ptr::null_mut::<i32>();
            __t37.not = 0i32;
            __t37
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"use_environment\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, use_environment) as u64,
        r#type: 3u32,
        visibility: 2u32,
        sys: {
            let mut __t38: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t38.attr = core::ptr::null::<i8>();
            __t38.flag_index = 7i32;
            __t38.flag_setter = Some(config_sys_flag_not);
            __t38
        },
        global_var: {
            let mut __t39: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t39.ptr = unsafe { core::ptr::addr_of_mut!(Py_IgnoreEnvironmentFlag) };
            __t39.not = 1i32;
            __t39
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"verbose\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, verbose) as u64,
        r#type: 1u32,
        visibility: 2u32,
        sys: {
            let mut __t40: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t40.attr = core::ptr::null::<i8>();
            __t40.flag_index = 8i32;
            __t40.flag_setter = None;
            __t40
        },
        global_var: {
            let mut __t41: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t41.ptr = unsafe { core::ptr::addr_of_mut!(Py_VerboseFlag) };
            __t41.not = 0i32;
            __t41
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"warnoptions\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, warnoptions) as u64,
        r#type: 12u32,
        visibility: 2u32,
        sys: {
            let mut __t42: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t42.attr = (b"warnoptions\0".as_ptr() as *mut i8) as *const i8;
            __t42.flag_index = 1i32.wrapping_neg();
            __t42.flag_setter = None;
            __t42
        },
        global_var: {
            let mut __t43: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t43.ptr = core::ptr::null_mut::<i32>();
            __t43.not = 0i32;
            __t43
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"write_bytecode\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, write_bytecode) as u64,
        r#type: 3u32,
        visibility: 2u32,
        sys: {
            let mut __t44: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t44.attr = core::ptr::null::<i8>();
            __t44.flag_index = 4i32;
            __t44.flag_setter = Some(config_sys_flag_not);
            __t44
        },
        global_var: {
            let mut __t45: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t45.ptr = unsafe { core::ptr::addr_of_mut!(Py_DontWriteBytecodeFlag) };
            __t45.not = 1i32;
            __t45
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"xoptions\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, xoptions) as u64,
        r#type: 12u32,
        visibility: 2u32,
        sys: {
            let mut __t46: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t46.attr = (b"_xoptions\0".as_ptr() as *mut i8) as *const i8;
            __t46.flag_index = 1i32.wrapping_neg();
            __t46.flag_setter = None;
            __t46
        },
        global_var: {
            let mut __t47: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t47.ptr = core::ptr::null_mut::<i32>();
            __t47.not = 0i32;
            __t47
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"buffered_stdio\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, buffered_stdio) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t48: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t48.attr = core::ptr::null::<i8>();
            __t48.flag_index = 1i32.wrapping_neg();
            __t48.flag_setter = None;
            __t48
        },
        global_var: {
            let mut __t49: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t49.ptr = unsafe { core::ptr::addr_of_mut!(Py_UnbufferedStdioFlag) };
            __t49.not = 1i32;
            __t49
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"check_hash_pycs_mode\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, check_hash_pycs_mode) as u64,
        r#type: 10u32,
        visibility: 1u32,
        sys: {
            let mut __t50: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t50.attr = core::ptr::null::<i8>();
            __t50.flag_index = 1i32.wrapping_neg();
            __t50.flag_setter = None;
            __t50
        },
        global_var: {
            let mut __t51: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t51.ptr = core::ptr::null_mut::<i32>();
            __t51.not = 0i32;
            __t51
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"code_debug_ranges\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, code_debug_ranges) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t52: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t52.attr = core::ptr::null::<i8>();
            __t52.flag_index = 1i32.wrapping_neg();
            __t52.flag_setter = None;
            __t52
        },
        global_var: {
            let mut __t53: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t53.ptr = core::ptr::null_mut::<i32>();
            __t53.not = 0i32;
            __t53
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"configure_c_stdio\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, configure_c_stdio) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t54: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t54.attr = core::ptr::null::<i8>();
            __t54.flag_index = 1i32.wrapping_neg();
            __t54.flag_setter = None;
            __t54
        },
        global_var: {
            let mut __t55: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t55.ptr = core::ptr::null_mut::<i32>();
            __t55.not = 0i32;
            __t55
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"dev_mode\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, dev_mode) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t56: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t56.attr = core::ptr::null::<i8>();
            __t56.flag_index = 1i32.wrapping_neg();
            __t56.flag_setter = None;
            __t56
        },
        global_var: {
            let mut __t57: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t57.ptr = core::ptr::null_mut::<i32>();
            __t57.not = 0i32;
            __t57
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"dump_refs\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, dump_refs) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t58: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t58.attr = core::ptr::null::<i8>();
            __t58.flag_index = 1i32.wrapping_neg();
            __t58.flag_setter = None;
            __t58
        },
        global_var: {
            let mut __t59: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t59.ptr = core::ptr::null_mut::<i32>();
            __t59.not = 0i32;
            __t59
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"dump_refs_file\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, dump_refs_file) as u64,
        r#type: 11u32,
        visibility: 1u32,
        sys: {
            let mut __t60: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t60.attr = core::ptr::null::<i8>();
            __t60.flag_index = 1i32.wrapping_neg();
            __t60.flag_setter = None;
            __t60
        },
        global_var: {
            let mut __t61: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t61.ptr = core::ptr::null_mut::<i32>();
            __t61.not = 0i32;
            __t61
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"faulthandler\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, faulthandler) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t62: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t62.attr = core::ptr::null::<i8>();
            __t62.flag_index = 1i32.wrapping_neg();
            __t62.flag_setter = None;
            __t62
        },
        global_var: {
            let mut __t63: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t63.ptr = core::ptr::null_mut::<i32>();
            __t63.not = 0i32;
            __t63
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"filesystem_encoding\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, filesystem_encoding) as u64,
        r#type: 10u32,
        visibility: 1u32,
        sys: {
            let mut __t64: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t64.attr = core::ptr::null::<i8>();
            __t64.flag_index = 1i32.wrapping_neg();
            __t64.flag_setter = None;
            __t64
        },
        global_var: {
            let mut __t65: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t65.ptr = core::ptr::null_mut::<i32>();
            __t65.not = 0i32;
            __t65
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"filesystem_errors\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, filesystem_errors) as u64,
        r#type: 10u32,
        visibility: 1u32,
        sys: {
            let mut __t66: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t66.attr = core::ptr::null::<i8>();
            __t66.flag_index = 1i32.wrapping_neg();
            __t66.flag_setter = None;
            __t66
        },
        global_var: {
            let mut __t67: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t67.ptr = core::ptr::null_mut::<i32>();
            __t67.not = 0i32;
            __t67
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"hash_seed\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, hash_seed) as u64,
        r#type: 2u32,
        visibility: 1u32,
        sys: {
            let mut __t68: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t68.attr = core::ptr::null::<i8>();
            __t68.flag_index = 1i32.wrapping_neg();
            __t68.flag_setter = None;
            __t68
        },
        global_var: {
            let mut __t69: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t69.ptr = core::ptr::null_mut::<i32>();
            __t69.not = 0i32;
            __t69
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"home\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, home) as u64,
        r#type: 11u32,
        visibility: 1u32,
        sys: {
            let mut __t70: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t70.attr = core::ptr::null::<i8>();
            __t70.flag_index = 1i32.wrapping_neg();
            __t70.flag_setter = None;
            __t70
        },
        global_var: {
            let mut __t71: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t71.ptr = core::ptr::null_mut::<i32>();
            __t71.not = 0i32;
            __t71
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"thread_inherit_context\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, thread_inherit_context) as u64,
        r#type: 0u32,
        visibility: 1u32,
        sys: {
            let mut __t72: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t72.attr = core::ptr::null::<i8>();
            __t72.flag_index = 1i32.wrapping_neg();
            __t72.flag_setter = None;
            __t72
        },
        global_var: {
            let mut __t73: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t73.ptr = core::ptr::null_mut::<i32>();
            __t73.not = 0i32;
            __t73
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"context_aware_warnings\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, context_aware_warnings) as u64,
        r#type: 0u32,
        visibility: 1u32,
        sys: {
            let mut __t74: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t74.attr = core::ptr::null::<i8>();
            __t74.flag_index = 1i32.wrapping_neg();
            __t74.flag_setter = None;
            __t74
        },
        global_var: {
            let mut __t75: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t75.ptr = core::ptr::null_mut::<i32>();
            __t75.not = 0i32;
            __t75
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"import_time\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, import_time) as u64,
        r#type: 1u32,
        visibility: 1u32,
        sys: {
            let mut __t76: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t76.attr = core::ptr::null::<i8>();
            __t76.flag_index = 1i32.wrapping_neg();
            __t76.flag_setter = None;
            __t76
        },
        global_var: {
            let mut __t77: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t77.ptr = core::ptr::null_mut::<i32>();
            __t77.not = 0i32;
            __t77
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"install_signal_handlers\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, install_signal_handlers) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t78: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t78.attr = core::ptr::null::<i8>();
            __t78.flag_index = 1i32.wrapping_neg();
            __t78.flag_setter = None;
            __t78
        },
        global_var: {
            let mut __t79: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t79.ptr = core::ptr::null_mut::<i32>();
            __t79.not = 0i32;
            __t79
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"isolated\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, isolated) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t80: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t80.attr = core::ptr::null::<i8>();
            __t80.flag_index = 1i32.wrapping_neg();
            __t80.flag_setter = None;
            __t80
        },
        global_var: {
            let mut __t81: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t81.ptr = unsafe { core::ptr::addr_of_mut!(Py_IsolatedFlag) };
            __t81.not = 0i32;
            __t81
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"malloc_stats\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, malloc_stats) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t82: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t82.attr = core::ptr::null::<i8>();
            __t82.flag_index = 1i32.wrapping_neg();
            __t82.flag_setter = None;
            __t82
        },
        global_var: {
            let mut __t83: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t83.ptr = core::ptr::null_mut::<i32>();
            __t83.not = 0i32;
            __t83
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"orig_argv\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, orig_argv) as u64,
        r#type: 12u32,
        visibility: 1u32,
        sys: {
            let mut __t84: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t84.attr = (b"orig_argv\0".as_ptr() as *mut i8) as *const i8;
            __t84.flag_index = 1i32.wrapping_neg();
            __t84.flag_setter = None;
            __t84
        },
        global_var: {
            let mut __t85: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t85.ptr = core::ptr::null_mut::<i32>();
            __t85.not = 0i32;
            __t85
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"parse_argv\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, parse_argv) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t86: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t86.attr = core::ptr::null::<i8>();
            __t86.flag_index = 1i32.wrapping_neg();
            __t86.flag_setter = None;
            __t86
        },
        global_var: {
            let mut __t87: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t87.ptr = core::ptr::null_mut::<i32>();
            __t87.not = 0i32;
            __t87
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"pathconfig_warnings\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, pathconfig_warnings) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t88: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t88.attr = core::ptr::null::<i8>();
            __t88.flag_index = 1i32.wrapping_neg();
            __t88.flag_setter = None;
            __t88
        },
        global_var: {
            let mut __t89: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t89.ptr = unsafe { core::ptr::addr_of_mut!(Py_FrozenFlag) };
            __t89.not = 1i32;
            __t89
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"perf_profiling\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, perf_profiling) as u64,
        r#type: 1u32,
        visibility: 1u32,
        sys: {
            let mut __t90: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t90.attr = core::ptr::null::<i8>();
            __t90.flag_index = 1i32.wrapping_neg();
            __t90.flag_setter = None;
            __t90
        },
        global_var: {
            let mut __t91: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t91.ptr = core::ptr::null_mut::<i32>();
            __t91.not = 0i32;
            __t91
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"remote_debug\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, remote_debug) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t92: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t92.attr = core::ptr::null::<i8>();
            __t92.flag_index = 1i32.wrapping_neg();
            __t92.flag_setter = None;
            __t92
        },
        global_var: {
            let mut __t93: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t93.ptr = core::ptr::null_mut::<i32>();
            __t93.not = 0i32;
            __t93
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"program_name\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, program_name) as u64,
        r#type: 10u32,
        visibility: 1u32,
        sys: {
            let mut __t94: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t94.attr = core::ptr::null::<i8>();
            __t94.flag_index = 1i32.wrapping_neg();
            __t94.flag_setter = None;
            __t94
        },
        global_var: {
            let mut __t95: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t95.ptr = core::ptr::null_mut::<i32>();
            __t95.not = 0i32;
            __t95
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"run_command\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, run_command) as u64,
        r#type: 11u32,
        visibility: 1u32,
        sys: {
            let mut __t96: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t96.attr = core::ptr::null::<i8>();
            __t96.flag_index = 1i32.wrapping_neg();
            __t96.flag_setter = None;
            __t96
        },
        global_var: {
            let mut __t97: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t97.ptr = core::ptr::null_mut::<i32>();
            __t97.not = 0i32;
            __t97
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"run_filename\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, run_filename) as u64,
        r#type: 11u32,
        visibility: 1u32,
        sys: {
            let mut __t98: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t98.attr = core::ptr::null::<i8>();
            __t98.flag_index = 1i32.wrapping_neg();
            __t98.flag_setter = None;
            __t98
        },
        global_var: {
            let mut __t99: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t99.ptr = core::ptr::null_mut::<i32>();
            __t99.not = 0i32;
            __t99
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"run_module\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, run_module) as u64,
        r#type: 11u32,
        visibility: 1u32,
        sys: {
            let mut __t100: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t100.attr = core::ptr::null::<i8>();
            __t100.flag_index = 1i32.wrapping_neg();
            __t100.flag_setter = None;
            __t100
        },
        global_var: {
            let mut __t101: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t101.ptr = core::ptr::null_mut::<i32>();
            __t101.not = 0i32;
            __t101
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"safe_path\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, safe_path) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t102: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t102.attr = core::ptr::null::<i8>();
            __t102.flag_index = 1i32.wrapping_neg();
            __t102.flag_setter = None;
            __t102
        },
        global_var: {
            let mut __t103: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t103.ptr = core::ptr::null_mut::<i32>();
            __t103.not = 0i32;
            __t103
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"show_ref_count\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, show_ref_count) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t104: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t104.attr = core::ptr::null::<i8>();
            __t104.flag_index = 1i32.wrapping_neg();
            __t104.flag_setter = None;
            __t104
        },
        global_var: {
            let mut __t105: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t105.ptr = core::ptr::null_mut::<i32>();
            __t105.not = 0i32;
            __t105
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"site_import\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, site_import) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t106: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t106.attr = core::ptr::null::<i8>();
            __t106.flag_index = 1i32.wrapping_neg();
            __t106.flag_setter = None;
            __t106
        },
        global_var: {
            let mut __t107: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t107.ptr = unsafe { core::ptr::addr_of_mut!(Py_NoSiteFlag) };
            __t107.not = 1i32;
            __t107
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"skip_source_first_line\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, skip_source_first_line) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t108: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t108.attr = core::ptr::null::<i8>();
            __t108.flag_index = 1i32.wrapping_neg();
            __t108.flag_setter = None;
            __t108
        },
        global_var: {
            let mut __t109: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t109.ptr = core::ptr::null_mut::<i32>();
            __t109.not = 0i32;
            __t109
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"stdio_encoding\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, stdio_encoding) as u64,
        r#type: 10u32,
        visibility: 1u32,
        sys: {
            let mut __t110: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t110.attr = core::ptr::null::<i8>();
            __t110.flag_index = 1i32.wrapping_neg();
            __t110.flag_setter = None;
            __t110
        },
        global_var: {
            let mut __t111: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t111.ptr = core::ptr::null_mut::<i32>();
            __t111.not = 0i32;
            __t111
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"stdio_errors\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, stdio_errors) as u64,
        r#type: 10u32,
        visibility: 1u32,
        sys: {
            let mut __t112: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t112.attr = core::ptr::null::<i8>();
            __t112.flag_index = 1i32.wrapping_neg();
            __t112.flag_setter = None;
            __t112
        },
        global_var: {
            let mut __t113: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t113.ptr = core::ptr::null_mut::<i32>();
            __t113.not = 0i32;
            __t113
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"tracemalloc\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, tracemalloc) as u64,
        r#type: 1u32,
        visibility: 1u32,
        sys: {
            let mut __t114: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t114.attr = core::ptr::null::<i8>();
            __t114.flag_index = 1i32.wrapping_neg();
            __t114.flag_setter = None;
            __t114
        },
        global_var: {
            let mut __t115: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t115.ptr = core::ptr::null_mut::<i32>();
            __t115.not = 0i32;
            __t115
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"use_frozen_modules\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, use_frozen_modules) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t116: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t116.attr = core::ptr::null::<i8>();
            __t116.flag_index = 1i32.wrapping_neg();
            __t116.flag_setter = None;
            __t116
        },
        global_var: {
            let mut __t117: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t117.ptr = core::ptr::null_mut::<i32>();
            __t117.not = 0i32;
            __t117
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"use_hash_seed\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, use_hash_seed) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t118: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t118.attr = core::ptr::null::<i8>();
            __t118.flag_index = 1i32.wrapping_neg();
            __t118.flag_setter = None;
            __t118
        },
        global_var: {
            let mut __t119: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t119.ptr = core::ptr::null_mut::<i32>();
            __t119.not = 0i32;
            __t119
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"user_site_directory\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, user_site_directory) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t120: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t120.attr = core::ptr::null::<i8>();
            __t120.flag_index = 1i32.wrapping_neg();
            __t120.flag_setter = None;
            __t120
        },
        global_var: {
            let mut __t121: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t121.ptr = unsafe { core::ptr::addr_of_mut!(Py_NoUserSiteDirectory) };
            __t121.not = 1i32;
            __t121
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"warn_default_encoding\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, warn_default_encoding) as u64,
        r#type: 3u32,
        visibility: 1u32,
        sys: {
            let mut __t122: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t122.attr = core::ptr::null::<i8>();
            __t122.flag_index = 1i32.wrapping_neg();
            __t122.flag_setter = None;
            __t122
        },
        global_var: {
            let mut __t123: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t123.ptr = core::ptr::null_mut::<i32>();
            __t123.not = 0i32;
            __t123
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"_config_init\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, _config_init) as u64,
        r#type: 1u32,
        visibility: 0u32,
        sys: {
            let mut __t124: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t124.attr = core::ptr::null::<i8>();
            __t124.flag_index = 1i32.wrapping_neg();
            __t124.flag_setter = None;
            __t124
        },
        global_var: {
            let mut __t125: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t125.ptr = core::ptr::null_mut::<i32>();
            __t125.not = 0i32;
            __t125
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"_init_main\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, _init_main) as u64,
        r#type: 3u32,
        visibility: 0u32,
        sys: {
            let mut __t126: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t126.attr = core::ptr::null::<i8>();
            __t126.flag_index = 1i32.wrapping_neg();
            __t126.flag_setter = None;
            __t126
        },
        global_var: {
            let mut __t127: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t127.ptr = core::ptr::null_mut::<i32>();
            __t127.not = 0i32;
            __t127
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"_install_importlib\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, _install_importlib) as u64,
        r#type: 3u32,
        visibility: 0u32,
        sys: {
            let mut __t128: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t128.attr = core::ptr::null::<i8>();
            __t128.flag_index = 1i32.wrapping_neg();
            __t128.flag_setter = None;
            __t128
        },
        global_var: {
            let mut __t129: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t129.ptr = core::ptr::null_mut::<i32>();
            __t129.not = 0i32;
            __t129
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"_is_python_build\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, _is_python_build) as u64,
        r#type: 3u32,
        visibility: 0u32,
        sys: {
            let mut __t130: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t130.attr = core::ptr::null::<i8>();
            __t130.flag_index = 1i32.wrapping_neg();
            __t130.flag_setter = None;
            __t130
        },
        global_var: {
            let mut __t131: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t131.ptr = core::ptr::null_mut::<i32>();
            __t131.not = 0i32;
            __t131
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"module_search_paths_set\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, module_search_paths_set) as u64,
        r#type: 3u32,
        visibility: 0u32,
        sys: {
            let mut __t132: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t132.attr = core::ptr::null::<i8>();
            __t132.flag_index = 1i32.wrapping_neg();
            __t132.flag_setter = None;
            __t132
        },
        global_var: {
            let mut __t133: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t133.ptr = core::ptr::null_mut::<i32>();
            __t133.not = 0i32;
            __t133
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"pythonpath_env\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, pythonpath_env) as u64,
        r#type: 11u32,
        visibility: 0u32,
        sys: {
            let mut __t134: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t134.attr = core::ptr::null::<i8>();
            __t134.flag_index = 1i32.wrapping_neg();
            __t134.flag_setter = None;
            __t134
        },
        global_var: {
            let mut __t135: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t135.ptr = core::ptr::null_mut::<i32>();
            __t135.not = 0i32;
            __t135
        },
    },
    __SlateRecord_initconfig_c_99 {
        name: (b"sys_path_0\0".as_ptr() as *mut i8) as *const i8,
        offset: core::mem::offset_of!(PyConfig, sys_path_0) as u64,
        r#type: 11u32,
        visibility: 0u32,
        sys: {
            let mut __t136: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t136.attr = core::ptr::null::<i8>();
            __t136.flag_index = 1i32.wrapping_neg();
            __t136.flag_setter = None;
            __t136
        },
        global_var: {
            let mut __t137: __SlateRecord_initconfig_c_90 = unsafe { core::mem::zeroed() };
            __t137.ptr = core::ptr::null_mut::<i32>();
            __t137.not = 0i32;
            __t137
        },
    },
    {
        let mut __t138: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t138.name = core::ptr::null::<i8>();
        __t138.offset = 0u64;
        __t138.r#type = 0u32;
        __t138.visibility = 0u32;
        __t138.sys = {
            let mut __t139: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t139.attr = core::ptr::null::<i8>();
            __t139.flag_index = 1i32.wrapping_neg();
            __t139.flag_setter = None;
            __t139
        };
        __t138
    },
]);

/// --- Read-only options -----------
/// --- Init-only options -----------
///
/// Members already present in PYCONFIG_SPEC
/// Array terminator
static PYPRECONFIG_SPEC: __SlateAlign16<[__SlateRecord_initconfig_c_99; 11]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t0.name = (b"allocator\0".as_ptr() as *mut i8) as *const i8;
        __t0.offset = core::mem::offset_of!(PyPreConfig, allocator) as u64;
        __t0.r#type = 0u32;
        __t0.visibility = 1u32;
        __t0.sys = {
            let mut __t1: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t1.attr = core::ptr::null::<i8>();
            __t1.flag_index = 1i32.wrapping_neg();
            __t1.flag_setter = None;
            __t1
        };
        __t0
    },
    {
        let mut __t2: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t2.name = (b"coerce_c_locale\0".as_ptr() as *mut i8) as *const i8;
        __t2.offset = core::mem::offset_of!(PyPreConfig, coerce_c_locale) as u64;
        __t2.r#type = 3u32;
        __t2.visibility = 1u32;
        __t2.sys = {
            let mut __t3: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t3.attr = core::ptr::null::<i8>();
            __t3.flag_index = 1i32.wrapping_neg();
            __t3.flag_setter = None;
            __t3
        };
        __t2
    },
    {
        let mut __t4: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t4.name = (b"coerce_c_locale_warn\0".as_ptr() as *mut i8) as *const i8;
        __t4.offset = core::mem::offset_of!(PyPreConfig, coerce_c_locale_warn) as u64;
        __t4.r#type = 3u32;
        __t4.visibility = 1u32;
        __t4.sys = {
            let mut __t5: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t5.attr = core::ptr::null::<i8>();
            __t5.flag_index = 1i32.wrapping_neg();
            __t5.flag_setter = None;
            __t5
        };
        __t4
    },
    {
        let mut __t6: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t6.name = (b"configure_locale\0".as_ptr() as *mut i8) as *const i8;
        __t6.offset = core::mem::offset_of!(PyPreConfig, configure_locale) as u64;
        __t6.r#type = 3u32;
        __t6.visibility = 1u32;
        __t6.sys = {
            let mut __t7: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t7.attr = core::ptr::null::<i8>();
            __t7.flag_index = 1i32.wrapping_neg();
            __t7.flag_setter = None;
            __t7
        };
        __t6
    },
    {
        let mut __t8: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t8.name = (b"utf8_mode\0".as_ptr() as *mut i8) as *const i8;
        __t8.offset = core::mem::offset_of!(PyPreConfig, utf8_mode) as u64;
        __t8.r#type = 3u32;
        __t8.visibility = 1u32;
        __t8.sys = {
            let mut __t9: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t9.attr = core::ptr::null::<i8>();
            __t9.flag_index = 1i32.wrapping_neg();
            __t9.flag_setter = None;
            __t9
        };
        __t8
    },
    {
        let mut __t10: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t10.name = (b"_config_init\0".as_ptr() as *mut i8) as *const i8;
        __t10.offset = core::mem::offset_of!(PyPreConfig, _config_init) as u64;
        __t10.r#type = 0u32;
        __t10.visibility = 0u32;
        __t10.sys = {
            let mut __t11: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t11.attr = core::ptr::null::<i8>();
            __t11.flag_index = 1i32.wrapping_neg();
            __t11.flag_setter = None;
            __t11
        };
        __t10
    },
    {
        let mut __t12: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t12.name = (b"dev_mode\0".as_ptr() as *mut i8) as *const i8;
        __t12.offset = core::mem::offset_of!(PyPreConfig, dev_mode) as u64;
        __t12.r#type = 3u32;
        __t12.visibility = 0u32;
        __t12.sys = {
            let mut __t13: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t13.attr = core::ptr::null::<i8>();
            __t13.flag_index = 1i32.wrapping_neg();
            __t13.flag_setter = None;
            __t13
        };
        __t12
    },
    {
        let mut __t14: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t14.name = (b"isolated\0".as_ptr() as *mut i8) as *const i8;
        __t14.offset = core::mem::offset_of!(PyPreConfig, isolated) as u64;
        __t14.r#type = 3u32;
        __t14.visibility = 0u32;
        __t14.sys = {
            let mut __t15: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t15.attr = core::ptr::null::<i8>();
            __t15.flag_index = 1i32.wrapping_neg();
            __t15.flag_setter = None;
            __t15
        };
        __t14
    },
    {
        let mut __t16: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t16.name = (b"parse_argv\0".as_ptr() as *mut i8) as *const i8;
        __t16.offset = core::mem::offset_of!(PyPreConfig, parse_argv) as u64;
        __t16.r#type = 3u32;
        __t16.visibility = 0u32;
        __t16.sys = {
            let mut __t17: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t17.attr = core::ptr::null::<i8>();
            __t17.flag_index = 1i32.wrapping_neg();
            __t17.flag_setter = None;
            __t17
        };
        __t16
    },
    {
        let mut __t18: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t18.name = (b"use_environment\0".as_ptr() as *mut i8) as *const i8;
        __t18.offset = core::mem::offset_of!(PyPreConfig, use_environment) as u64;
        __t18.r#type = 3u32;
        __t18.visibility = 0u32;
        __t18.sys = {
            let mut __t19: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t19.attr = core::ptr::null::<i8>();
            __t19.flag_index = 1i32.wrapping_neg();
            __t19.flag_setter = None;
            __t19
        };
        __t18
    },
    {
        let mut __t20: __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
        __t20.name = core::ptr::null::<i8>();
        __t20.offset = 0u64;
        __t20.r#type = 0u32;
        __t20.visibility = 0u32;
        __t20.sys = {
            let mut __t21: __SlateRecord_initconfig_c_85 = unsafe { core::mem::zeroed() };
            __t21.attr = core::ptr::null::<i8>();
            __t21.flag_index = 1i32.wrapping_neg();
            __t21.flag_setter = None;
            __t21
        };
        __t20
    },
]);

// --- Command line options ---------------------------------------
/// Short usage message (with %s for argv0)
static usage_line: __SlateAlign16<[i8; 64]> = __SlateAlign16([
    117i8, 115i8, 97i8, 103i8, 101i8, 58i8, 32i8, 37i8, 108i8, 115i8, 32i8, 91i8, 111i8, 112i8,
    116i8, 105i8, 111i8, 110i8, 93i8, 32i8, 46i8, 46i8, 46i8, 32i8, 91i8, 45i8, 99i8, 32i8, 99i8,
    109i8, 100i8, 32i8, 124i8, 32i8, 45i8, 109i8, 32i8, 109i8, 111i8, 100i8, 32i8, 124i8, 32i8,
    102i8, 105i8, 108i8, 101i8, 32i8, 124i8, 32i8, 45i8, 93i8, 32i8, 91i8, 97i8, 114i8, 103i8,
    93i8, 32i8, 46i8, 46i8, 46i8, 10i8, 0i8,
]);

/// Long help message
///
/// Lines sorted by option name; keep in sync with usage_envvars* below
static usage_help: __SlateAlign16<[i8; 2482]> = __SlateAlign16([
    79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 40i8, 97i8, 110i8, 100i8, 32i8, 99i8,
    111i8, 114i8, 114i8, 101i8, 115i8, 112i8, 111i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8,
    101i8, 110i8, 118i8, 105i8, 114i8, 111i8, 110i8, 109i8, 101i8, 110i8, 116i8, 32i8, 118i8, 97i8,
    114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 115i8, 41i8, 58i8, 10i8, 45i8, 98i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 58i8, 32i8, 105i8, 115i8, 115i8, 117i8, 101i8, 32i8, 119i8, 97i8, 114i8, 110i8,
    105i8, 110i8, 103i8, 115i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8, 99i8, 111i8, 110i8,
    118i8, 101i8, 114i8, 116i8, 105i8, 110i8, 103i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 47i8,
    98i8, 121i8, 116i8, 101i8, 97i8, 114i8, 114i8, 97i8, 121i8, 32i8, 116i8, 111i8, 32i8, 115i8,
    116i8, 114i8, 32i8, 97i8, 110i8, 100i8, 32i8, 99i8, 111i8, 109i8, 112i8, 97i8, 114i8, 105i8,
    110i8, 103i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 47i8, 98i8, 121i8, 116i8, 101i8, 97i8, 114i8, 114i8, 97i8, 121i8, 32i8, 119i8,
    105i8, 116i8, 104i8, 32i8, 115i8, 116i8, 114i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 105i8, 110i8, 116i8, 46i8, 32i8, 40i8,
    45i8, 98i8, 98i8, 58i8, 32i8, 105i8, 115i8, 115i8, 117i8, 101i8, 32i8, 101i8, 114i8, 114i8,
    111i8, 114i8, 115i8, 41i8, 10i8, 45i8, 66i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 100i8,
    111i8, 110i8, 39i8, 116i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 32i8, 46i8, 112i8, 121i8,
    99i8, 32i8, 102i8, 105i8, 108i8, 101i8, 115i8, 32i8, 111i8, 110i8, 32i8, 105i8, 109i8, 112i8,
    111i8, 114i8, 116i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 68i8, 79i8, 78i8, 84i8, 87i8, 82i8, 73i8, 84i8, 69i8, 66i8, 89i8, 84i8, 69i8, 67i8, 79i8,
    68i8, 69i8, 61i8, 120i8, 10i8, 45i8, 99i8, 32i8, 99i8, 109i8, 100i8, 32i8, 58i8, 32i8, 112i8,
    114i8, 111i8, 103i8, 114i8, 97i8, 109i8, 32i8, 112i8, 97i8, 115i8, 115i8, 101i8, 100i8, 32i8,
    105i8, 110i8, 32i8, 97i8, 115i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 40i8,
    116i8, 101i8, 114i8, 109i8, 105i8, 110i8, 97i8, 116i8, 101i8, 115i8, 32i8, 111i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 108i8, 105i8, 115i8, 116i8, 41i8, 10i8, 45i8, 100i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 58i8, 32i8, 116i8, 117i8, 114i8, 110i8, 32i8, 111i8, 110i8, 32i8, 112i8,
    97i8, 114i8, 115i8, 101i8, 114i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 103i8, 105i8, 110i8,
    103i8, 32i8, 111i8, 117i8, 116i8, 112i8, 117i8, 116i8, 32i8, 40i8, 102i8, 111i8, 114i8, 32i8,
    101i8, 120i8, 112i8, 101i8, 114i8, 116i8, 115i8, 32i8, 111i8, 110i8, 108i8, 121i8, 44i8, 32i8,
    111i8, 110i8, 108i8, 121i8, 32i8, 119i8, 111i8, 114i8, 107i8, 115i8, 32i8, 111i8, 110i8, 10i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8,
    98i8, 117i8, 105i8, 108i8, 100i8, 115i8, 41i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 68i8, 69i8, 66i8, 85i8, 71i8, 61i8, 120i8, 10i8, 45i8,
    69i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 105i8, 103i8, 110i8, 111i8, 114i8, 101i8, 32i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 42i8, 32i8, 101i8, 110i8, 118i8, 105i8, 114i8, 111i8,
    110i8, 109i8, 101i8, 110i8, 116i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8, 108i8, 101i8,
    115i8, 32i8, 40i8, 115i8, 117i8, 99i8, 104i8, 32i8, 97i8, 115i8, 32i8, 80i8, 89i8, 84i8, 72i8,
    79i8, 78i8, 80i8, 65i8, 84i8, 72i8, 41i8, 10i8, 45i8, 104i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    58i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 104i8,
    101i8, 108i8, 112i8, 32i8, 109i8, 101i8, 115i8, 115i8, 97i8, 103i8, 101i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 101i8, 120i8, 105i8, 116i8, 32i8, 40i8, 97i8, 108i8, 115i8, 111i8, 32i8, 45i8,
    63i8, 32i8, 111i8, 114i8, 32i8, 45i8, 45i8, 104i8, 101i8, 108i8, 112i8, 41i8, 10i8, 45i8,
    105i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 105i8, 110i8, 115i8, 112i8, 101i8, 99i8,
    116i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 108i8,
    121i8, 32i8, 97i8, 102i8, 116i8, 101i8, 114i8, 32i8, 114i8, 117i8, 110i8, 110i8, 105i8, 110i8,
    103i8, 32i8, 115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 59i8, 32i8, 102i8, 111i8, 114i8, 99i8,
    101i8, 115i8, 32i8, 97i8, 32i8, 112i8, 114i8, 111i8, 109i8, 112i8, 116i8, 32i8, 101i8, 118i8,
    101i8, 110i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 105i8, 102i8, 32i8,
    115i8, 116i8, 100i8, 105i8, 110i8, 32i8, 100i8, 111i8, 101i8, 115i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 97i8, 112i8, 112i8, 101i8, 97i8, 114i8, 32i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8,
    97i8, 32i8, 116i8, 101i8, 114i8, 109i8, 105i8, 110i8, 97i8, 108i8, 59i8, 32i8, 97i8, 108i8,
    115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 73i8, 78i8, 83i8, 80i8, 69i8, 67i8,
    84i8, 61i8, 120i8, 10i8, 45i8, 73i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 105i8, 115i8,
    111i8, 108i8, 97i8, 116i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 102i8,
    114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 115i8, 101i8, 114i8, 39i8, 115i8,
    32i8, 101i8, 110i8, 118i8, 105i8, 114i8, 111i8, 110i8, 109i8, 101i8, 110i8, 116i8, 32i8, 40i8,
    105i8, 109i8, 112i8, 108i8, 105i8, 101i8, 115i8, 32i8, 45i8, 69i8, 44i8, 32i8, 45i8, 80i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 45i8, 115i8, 41i8, 10i8, 45i8, 109i8, 32i8, 109i8, 111i8,
    100i8, 32i8, 58i8, 32i8, 114i8, 117i8, 110i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8,
    121i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8,
    115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 32i8, 40i8, 116i8, 101i8, 114i8, 109i8, 105i8, 110i8,
    97i8, 116i8, 101i8, 115i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 108i8, 105i8,
    115i8, 116i8, 41i8, 10i8, 45i8, 79i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 114i8, 101i8,
    109i8, 111i8, 118i8, 101i8, 32i8, 97i8, 115i8, 115i8, 101i8, 114i8, 116i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 95i8, 95i8, 100i8, 101i8, 98i8, 117i8, 103i8, 95i8, 95i8, 45i8, 100i8, 101i8,
    112i8, 101i8, 110i8, 100i8, 101i8, 110i8, 116i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 59i8, 32i8, 97i8, 100i8, 100i8, 32i8, 46i8, 111i8, 112i8, 116i8,
    45i8, 49i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 46i8, 112i8, 121i8, 99i8, 32i8, 101i8, 120i8, 116i8, 101i8, 110i8,
    115i8, 105i8, 111i8, 110i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8,
    72i8, 79i8, 78i8, 79i8, 80i8, 84i8, 73i8, 77i8, 73i8, 90i8, 69i8, 61i8, 120i8, 10i8, 45i8,
    79i8, 79i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 100i8, 111i8, 32i8, 45i8, 79i8, 32i8, 99i8,
    104i8, 97i8, 110i8, 103i8, 101i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 100i8, 105i8, 115i8, 99i8, 97i8, 114i8, 100i8, 32i8, 100i8, 111i8, 99i8, 115i8,
    116i8, 114i8, 105i8, 110i8, 103i8, 115i8, 59i8, 32i8, 97i8, 100i8, 100i8, 32i8, 46i8, 111i8,
    112i8, 116i8, 45i8, 50i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 46i8, 112i8, 121i8, 99i8, 32i8, 101i8, 120i8, 116i8,
    101i8, 110i8, 115i8, 105i8, 111i8, 110i8, 10i8, 45i8, 80i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8,
    32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 112i8, 114i8, 101i8, 112i8, 101i8, 110i8, 100i8,
    32i8, 97i8, 32i8, 112i8, 111i8, 116i8, 101i8, 110i8, 116i8, 105i8, 97i8, 108i8, 108i8, 121i8,
    32i8, 117i8, 110i8, 115i8, 97i8, 102i8, 101i8, 32i8, 112i8, 97i8, 116i8, 104i8, 32i8, 116i8,
    111i8, 32i8, 115i8, 121i8, 115i8, 46i8, 112i8, 97i8, 116i8, 104i8, 59i8, 32i8, 97i8, 108i8,
    115i8, 111i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 80i8, 89i8, 84i8,
    72i8, 79i8, 78i8, 83i8, 65i8, 70i8, 69i8, 80i8, 65i8, 84i8, 72i8, 10i8, 45i8, 113i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 112i8, 114i8,
    105i8, 110i8, 116i8, 32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 99i8, 111i8, 112i8, 121i8, 114i8, 105i8, 103i8, 104i8, 116i8, 32i8, 109i8, 101i8,
    115i8, 115i8, 97i8, 103i8, 101i8, 115i8, 32i8, 111i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 117i8,
    112i8, 10i8, 45i8, 115i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 100i8, 111i8, 110i8, 39i8,
    116i8, 32i8, 97i8, 100i8, 100i8, 32i8, 117i8, 115i8, 101i8, 114i8, 32i8, 115i8, 105i8, 116i8,
    101i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8, 114i8, 121i8, 32i8, 116i8, 111i8,
    32i8, 115i8, 121i8, 115i8, 46i8, 112i8, 97i8, 116i8, 104i8, 59i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 78i8, 79i8, 85i8, 83i8, 69i8, 82i8, 83i8,
    73i8, 84i8, 69i8, 61i8, 120i8, 10i8, 45i8, 83i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8,
    100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 105i8, 109i8, 112i8, 108i8, 121i8, 32i8, 39i8, 105i8,
    109i8, 112i8, 111i8, 114i8, 116i8, 32i8, 115i8, 105i8, 116i8, 101i8, 39i8, 32i8, 111i8, 110i8,
    32i8, 105i8, 110i8, 105i8, 116i8, 105i8, 97i8, 108i8, 105i8, 122i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 10i8, 45i8, 117i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 102i8, 111i8, 114i8, 99i8,
    101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 100i8, 111i8, 117i8, 116i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 115i8, 116i8, 100i8, 101i8, 114i8, 114i8, 32i8, 115i8, 116i8, 114i8, 101i8,
    97i8, 109i8, 115i8, 32i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 117i8, 110i8, 98i8, 117i8,
    102i8, 102i8, 101i8, 114i8, 101i8, 100i8, 59i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    104i8, 97i8, 115i8, 32i8, 110i8, 111i8, 32i8, 101i8, 102i8, 102i8, 101i8, 99i8, 116i8, 32i8,
    111i8, 110i8, 32i8, 115i8, 116i8, 100i8, 105i8, 110i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8,
    32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 85i8, 78i8, 66i8, 85i8, 70i8, 70i8, 69i8, 82i8, 69i8,
    68i8, 61i8, 120i8, 10i8, 45i8, 118i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 118i8, 101i8,
    114i8, 98i8, 111i8, 115i8, 101i8, 32i8, 40i8, 116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 105i8,
    109i8, 112i8, 111i8, 114i8, 116i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8,
    116i8, 115i8, 41i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 86i8, 69i8, 82i8, 66i8, 79i8, 83i8, 69i8, 61i8, 120i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 115i8, 117i8, 112i8,
    112i8, 108i8, 105i8, 101i8, 100i8, 32i8, 109i8, 117i8, 108i8, 116i8, 105i8, 112i8, 108i8,
    101i8, 32i8, 116i8, 105i8, 109i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 105i8, 110i8, 99i8,
    114i8, 101i8, 97i8, 115i8, 101i8, 32i8, 118i8, 101i8, 114i8, 98i8, 111i8, 115i8, 105i8, 116i8,
    121i8, 10i8, 45i8, 86i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 112i8, 114i8, 105i8, 110i8,
    116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 118i8,
    101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 101i8, 120i8, 105i8, 116i8, 32i8, 40i8, 97i8, 108i8, 115i8, 111i8,
    32i8, 45i8, 45i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 41i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 103i8, 105i8,
    118i8, 101i8, 110i8, 32i8, 116i8, 119i8, 105i8, 99i8, 101i8, 44i8, 32i8, 112i8, 114i8, 105i8,
    110i8, 116i8, 32i8, 109i8, 111i8, 114i8, 101i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 98i8, 117i8, 105i8, 108i8, 100i8, 10i8, 45i8, 87i8, 32i8, 97i8, 114i8, 103i8,
    32i8, 58i8, 32i8, 119i8, 97i8, 114i8, 110i8, 105i8, 110i8, 103i8, 32i8, 99i8, 111i8, 110i8,
    116i8, 114i8, 111i8, 108i8, 59i8, 32i8, 97i8, 114i8, 103i8, 32i8, 105i8, 115i8, 32i8, 97i8,
    99i8, 116i8, 105i8, 111i8, 110i8, 58i8, 109i8, 101i8, 115i8, 115i8, 97i8, 103i8, 101i8, 58i8,
    99i8, 97i8, 116i8, 101i8, 103i8, 111i8, 114i8, 121i8, 58i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 58i8, 108i8, 105i8, 110i8, 101i8, 110i8, 111i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8,
    87i8, 65i8, 82i8, 78i8, 73i8, 78i8, 71i8, 83i8, 61i8, 97i8, 114i8, 103i8, 10i8, 45i8, 120i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 115i8, 107i8, 105i8, 112i8, 32i8, 102i8, 105i8,
    114i8, 115i8, 116i8, 32i8, 108i8, 105i8, 110i8, 101i8, 32i8, 111i8, 102i8, 32i8, 115i8, 111i8,
    117i8, 114i8, 99i8, 101i8, 44i8, 32i8, 97i8, 108i8, 108i8, 111i8, 119i8, 105i8, 110i8, 103i8,
    32i8, 117i8, 115i8, 101i8, 32i8, 111i8, 102i8, 32i8, 110i8, 111i8, 110i8, 45i8, 85i8, 110i8,
    105i8, 120i8, 32i8, 102i8, 111i8, 114i8, 109i8, 115i8, 32i8, 111i8, 102i8, 32i8, 35i8, 33i8,
    99i8, 109i8, 100i8, 10i8, 45i8, 88i8, 32i8, 111i8, 112i8, 116i8, 32i8, 58i8, 32i8, 115i8,
    101i8, 116i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 45i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 99i8, 32i8, 111i8,
    112i8, 116i8, 105i8, 111i8, 110i8, 10i8, 45i8, 45i8, 99i8, 104i8, 101i8, 99i8, 107i8, 45i8,
    104i8, 97i8, 115i8, 104i8, 45i8, 98i8, 97i8, 115i8, 101i8, 100i8, 45i8, 112i8, 121i8, 99i8,
    115i8, 32i8, 97i8, 108i8, 119i8, 97i8, 121i8, 115i8, 124i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 124i8, 110i8, 101i8, 118i8, 101i8, 114i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 99i8, 111i8, 110i8, 116i8, 114i8, 111i8, 108i8, 32i8, 104i8,
    111i8, 119i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8, 110i8, 118i8, 97i8,
    108i8, 105i8, 100i8, 97i8, 116i8, 101i8, 115i8, 32i8, 104i8, 97i8, 115i8, 104i8, 45i8, 98i8,
    97i8, 115i8, 101i8, 100i8, 32i8, 46i8, 112i8, 121i8, 99i8, 32i8, 102i8, 105i8, 108i8, 101i8,
    115i8, 10i8, 45i8, 45i8, 104i8, 101i8, 108i8, 112i8, 45i8, 101i8, 110i8, 118i8, 58i8, 32i8,
    112i8, 114i8, 105i8, 110i8, 116i8, 32i8, 104i8, 101i8, 108i8, 112i8, 32i8, 97i8, 98i8, 111i8,
    117i8, 116i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 101i8, 110i8, 118i8, 105i8,
    114i8, 111i8, 110i8, 109i8, 101i8, 110i8, 116i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8,
    108i8, 101i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 101i8, 120i8, 105i8, 116i8, 10i8, 45i8,
    45i8, 104i8, 101i8, 108i8, 112i8, 45i8, 120i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 115i8,
    58i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 32i8, 104i8, 101i8, 108i8, 112i8, 32i8, 97i8,
    98i8, 111i8, 117i8, 116i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 45i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 99i8,
    32i8, 45i8, 88i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 101i8, 120i8, 105i8, 116i8, 10i8, 45i8, 45i8, 104i8, 101i8, 108i8, 112i8, 45i8,
    97i8, 108i8, 108i8, 58i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 32i8, 99i8, 111i8, 109i8,
    112i8, 108i8, 101i8, 116i8, 101i8, 32i8, 104i8, 101i8, 108i8, 112i8, 32i8, 105i8, 110i8, 102i8,
    111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 110i8, 100i8, 32i8, 101i8,
    120i8, 105i8, 116i8, 10i8, 10i8, 65i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8,
    58i8, 10i8, 102i8, 105i8, 108i8, 101i8, 32i8, 32i8, 32i8, 58i8, 32i8, 112i8, 114i8, 111i8,
    103i8, 114i8, 97i8, 109i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    32i8, 115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 32i8, 102i8, 105i8, 108i8, 101i8, 10i8, 45i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 112i8, 114i8, 111i8, 103i8, 114i8, 97i8, 109i8,
    32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 115i8, 116i8, 100i8,
    105i8, 110i8, 32i8, 40i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 59i8, 32i8, 105i8,
    110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 109i8, 111i8, 100i8,
    101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 32i8, 116i8, 116i8, 121i8, 41i8, 10i8, 97i8, 114i8,
    103i8, 32i8, 46i8, 46i8, 46i8, 58i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8,
    116i8, 115i8, 32i8, 112i8, 97i8, 115i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 112i8,
    114i8, 111i8, 103i8, 114i8, 97i8, 109i8, 32i8, 105i8, 110i8, 32i8, 115i8, 121i8, 115i8, 46i8,
    97i8, 114i8, 103i8, 118i8, 91i8, 49i8, 58i8, 93i8, 10i8, 0i8,
]);

static usage_xoptions: __SlateAlign16<[i8; 2374]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 102i8, 111i8, 108i8, 108i8, 111i8, 119i8, 105i8, 110i8, 103i8, 32i8,
    105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 45i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 99i8, 32i8, 111i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8,
    97i8, 98i8, 108i8, 101i8, 58i8, 10i8, 45i8, 88i8, 32i8, 99i8, 111i8, 110i8, 116i8, 101i8,
    120i8, 116i8, 95i8, 97i8, 119i8, 97i8, 114i8, 101i8, 95i8, 119i8, 97i8, 114i8, 110i8, 105i8,
    110i8, 103i8, 115i8, 61i8, 91i8, 48i8, 124i8, 49i8, 93i8, 58i8, 32i8, 105i8, 102i8, 32i8,
    116i8, 114i8, 117i8, 101i8, 32i8, 40i8, 49i8, 41i8, 32i8, 116i8, 104i8, 101i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 119i8, 97i8, 114i8, 110i8, 105i8, 110i8, 103i8, 115i8, 32i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 119i8, 105i8, 108i8, 108i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 117i8, 115i8, 101i8, 32i8, 97i8, 32i8, 99i8, 111i8, 110i8,
    116i8, 101i8, 120i8, 116i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 115i8,
    59i8, 32i8, 105i8, 102i8, 32i8, 102i8, 97i8, 108i8, 115i8, 101i8, 32i8, 40i8, 48i8, 41i8, 32i8,
    116i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 119i8, 97i8, 114i8, 110i8, 105i8,
    110i8, 103i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 119i8, 105i8, 108i8,
    108i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 117i8, 115i8, 101i8, 32i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 115i8,
    44i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 99i8, 111i8, 110i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 45i8, 115i8, 97i8,
    102i8, 101i8, 59i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8, 116i8, 114i8, 117i8,
    101i8, 32i8, 102i8, 111i8, 114i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    102i8, 114i8, 101i8, 101i8, 45i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 101i8, 100i8, 32i8,
    98i8, 117i8, 105i8, 108i8, 100i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 102i8, 97i8, 108i8,
    115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 59i8, 32i8,
    97i8, 108i8, 115i8, 111i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 80i8,
    89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 67i8, 79i8, 78i8, 84i8, 69i8, 88i8, 84i8, 95i8, 65i8, 87i8,
    65i8, 82i8, 69i8, 95i8, 87i8, 65i8, 82i8, 78i8, 73i8, 78i8, 71i8, 83i8, 10i8, 45i8, 88i8, 32i8,
    99i8, 112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8, 61i8, 78i8, 58i8, 32i8, 111i8,
    118i8, 101i8, 114i8, 114i8, 105i8, 100i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8, 32i8,
    111i8, 115i8, 46i8, 99i8, 112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 41i8,
    59i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 45i8, 88i8, 32i8, 99i8,
    112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8, 61i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 32i8, 99i8, 97i8, 110i8, 99i8, 101i8, 108i8, 115i8, 32i8, 111i8, 118i8, 101i8,
    114i8, 114i8, 105i8, 100i8, 105i8, 110i8, 103i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 67i8, 80i8, 85i8, 95i8, 67i8, 79i8, 85i8, 78i8, 84i8,
    10i8, 45i8, 88i8, 32i8, 100i8, 101i8, 118i8, 32i8, 58i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8,
    101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 68i8, 101i8, 118i8, 101i8, 108i8,
    111i8, 112i8, 109i8, 101i8, 110i8, 116i8, 32i8, 77i8, 111i8, 100i8, 101i8, 59i8, 32i8, 97i8,
    108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 68i8, 69i8, 86i8, 77i8, 79i8,
    68i8, 69i8, 10i8, 45i8, 88i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 45i8, 114i8,
    101i8, 109i8, 111i8, 116i8, 101i8, 45i8, 100i8, 101i8, 98i8, 117i8, 103i8, 58i8, 32i8, 100i8,
    105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8, 114i8, 101i8, 109i8, 111i8, 116i8, 101i8, 32i8,
    100i8, 101i8, 98i8, 117i8, 103i8, 103i8, 105i8, 110i8, 103i8, 59i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 68i8, 73i8, 83i8, 65i8, 66i8, 76i8,
    69i8, 95i8, 82i8, 69i8, 77i8, 79i8, 84i8, 69i8, 95i8, 68i8, 69i8, 66i8, 85i8, 71i8, 10i8, 45i8,
    88i8, 32i8, 102i8, 97i8, 117i8, 108i8, 116i8, 104i8, 97i8, 110i8, 100i8, 108i8, 101i8, 114i8,
    58i8, 32i8, 100i8, 117i8, 109i8, 112i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8,
    104i8, 111i8, 110i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8,
    111i8, 110i8, 32i8, 102i8, 97i8, 116i8, 97i8, 108i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8,
    115i8, 59i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 70i8, 65i8, 85i8, 76i8, 84i8, 72i8, 65i8,
    78i8, 68i8, 76i8, 69i8, 82i8, 10i8, 45i8, 88i8, 32i8, 102i8, 114i8, 111i8, 122i8, 101i8, 110i8,
    95i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 61i8, 91i8, 111i8, 110i8, 124i8, 111i8,
    102i8, 102i8, 93i8, 58i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8,
    111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 102i8, 114i8, 111i8, 122i8, 101i8, 110i8, 32i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 59i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 34i8, 111i8, 110i8, 34i8, 10i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 105i8, 110i8,
    115i8, 116i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 34i8, 111i8, 102i8, 102i8, 34i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 97i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 32i8, 98i8, 117i8, 105i8, 108i8, 100i8,
    59i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 97i8, 108i8, 115i8, 111i8,
    32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 70i8, 82i8, 79i8, 90i8, 69i8, 78i8, 95i8, 77i8,
    79i8, 68i8, 85i8, 76i8, 69i8, 83i8, 10i8, 45i8, 88i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8,
    116i8, 116i8, 105i8, 109i8, 101i8, 91i8, 61i8, 50i8, 93i8, 58i8, 32i8, 115i8, 104i8, 111i8,
    119i8, 32i8, 104i8, 111i8, 119i8, 32i8, 108i8, 111i8, 110i8, 103i8, 32i8, 101i8, 97i8, 99i8,
    104i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 32i8, 116i8, 97i8, 107i8, 101i8, 115i8,
    59i8, 32i8, 117i8, 115i8, 101i8, 32i8, 45i8, 88i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8,
    116i8, 116i8, 105i8, 109i8, 101i8, 61i8, 50i8, 32i8, 116i8, 111i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 108i8, 111i8, 103i8, 32i8, 105i8, 109i8, 112i8, 111i8,
    114i8, 116i8, 115i8, 32i8, 111i8, 102i8, 32i8, 97i8, 108i8, 114i8, 101i8, 97i8, 100i8, 121i8,
    45i8, 108i8, 111i8, 97i8, 100i8, 101i8, 100i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    115i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 80i8,
    82i8, 79i8, 70i8, 73i8, 76i8, 69i8, 73i8, 77i8, 80i8, 79i8, 82i8, 84i8, 84i8, 73i8, 77i8, 69i8,
    10i8, 45i8, 88i8, 32i8, 105i8, 110i8, 116i8, 95i8, 109i8, 97i8, 120i8, 95i8, 115i8, 116i8,
    114i8, 95i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 61i8, 78i8, 58i8, 32i8, 108i8, 105i8,
    109i8, 105i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8,
    102i8, 32i8, 105i8, 110i8, 116i8, 60i8, 45i8, 62i8, 115i8, 116i8, 114i8, 32i8, 99i8, 111i8,
    110i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 115i8, 59i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 48i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8,
    115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 109i8, 105i8, 116i8, 59i8, 32i8, 97i8,
    108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 73i8, 78i8, 84i8, 77i8, 65i8,
    88i8, 83i8, 84i8, 82i8, 68i8, 73i8, 71i8, 73i8, 84i8, 83i8, 10i8, 45i8, 88i8, 32i8, 110i8,
    111i8, 95i8, 100i8, 101i8, 98i8, 117i8, 103i8, 95i8, 114i8, 97i8, 110i8, 103i8, 101i8, 115i8,
    58i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 105i8, 110i8, 99i8, 108i8, 117i8, 100i8,
    101i8, 32i8, 101i8, 120i8, 116i8, 114i8, 97i8, 32i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 105i8, 110i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 115i8, 59i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 97i8, 108i8,
    115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 78i8, 79i8, 68i8, 69i8, 66i8, 85i8,
    71i8, 82i8, 65i8, 78i8, 71i8, 69i8, 83i8, 10i8, 45i8, 88i8, 32i8, 112i8, 101i8, 114i8, 102i8,
    58i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    76i8, 105i8, 110i8, 117i8, 120i8, 32i8, 34i8, 112i8, 101i8, 114i8, 102i8, 34i8, 32i8, 112i8,
    114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 80i8, 69i8, 82i8, 70i8, 83i8, 85i8, 80i8, 80i8, 79i8, 82i8,
    84i8, 61i8, 49i8, 10i8, 45i8, 88i8, 32i8, 112i8, 101i8, 114i8, 102i8, 95i8, 106i8, 105i8,
    116i8, 58i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 76i8, 105i8, 110i8, 117i8, 120i8, 32i8, 34i8, 112i8, 101i8, 114i8, 102i8, 34i8, 32i8,
    112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8,
    68i8, 87i8, 65i8, 82i8, 70i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 59i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 80i8, 69i8, 82i8, 70i8, 95i8, 74i8, 73i8, 84i8, 95i8,
    83i8, 85i8, 80i8, 80i8, 79i8, 82i8, 84i8, 61i8, 49i8, 10i8, 45i8, 88i8, 32i8, 112i8, 121i8,
    99i8, 97i8, 99i8, 104i8, 101i8, 95i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 61i8, 80i8,
    65i8, 84i8, 72i8, 58i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 32i8, 46i8, 112i8, 121i8,
    99i8, 32i8, 102i8, 105i8, 108i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 112i8,
    97i8, 114i8, 97i8, 108i8, 108i8, 101i8, 108i8, 32i8, 116i8, 114i8, 101i8, 101i8, 32i8, 105i8,
    110i8, 115i8, 116i8, 101i8, 97i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 111i8, 32i8, 116i8,
    104i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 99i8, 111i8, 100i8,
    101i8, 32i8, 116i8, 114i8, 101i8, 101i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8,
    89i8, 84i8, 72i8, 79i8, 78i8, 80i8, 89i8, 67i8, 65i8, 67i8, 72i8, 69i8, 80i8, 82i8, 69i8, 70i8,
    73i8, 88i8, 10i8, 45i8, 88i8, 32i8, 115i8, 104i8, 111i8, 119i8, 114i8, 101i8, 102i8, 99i8,
    111i8, 117i8, 110i8, 116i8, 58i8, 32i8, 111i8, 117i8, 116i8, 112i8, 117i8, 116i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 116i8, 111i8, 116i8, 97i8, 108i8, 32i8, 114i8, 101i8, 102i8, 101i8, 114i8,
    101i8, 110i8, 99i8, 101i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 117i8, 115i8, 101i8,
    100i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 109i8, 101i8, 109i8, 111i8,
    114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 115i8, 32i8, 119i8, 104i8, 101i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 111i8, 103i8, 114i8, 97i8, 109i8, 32i8, 102i8,
    105i8, 110i8, 105i8, 115i8, 104i8, 101i8, 115i8, 32i8, 111i8, 114i8, 32i8, 97i8, 102i8, 116i8,
    101i8, 114i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 32i8, 105i8, 110i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8, 105i8,
    118i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8,
    114i8, 59i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8, 119i8, 111i8, 114i8, 107i8, 115i8, 32i8,
    111i8, 110i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 98i8, 117i8, 105i8, 108i8, 100i8,
    115i8, 10i8, 45i8, 88i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 95i8, 105i8, 110i8,
    104i8, 101i8, 114i8, 105i8, 116i8, 95i8, 99i8, 111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 61i8,
    91i8, 48i8, 124i8, 49i8, 93i8, 58i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 40i8,
    49i8, 41i8, 32i8, 111i8, 114i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    40i8, 48i8, 41i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 115i8, 32i8, 105i8, 110i8,
    104i8, 101i8, 114i8, 105i8, 116i8, 105i8, 110i8, 103i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 99i8, 111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 32i8, 118i8, 97i8,
    114i8, 115i8, 32i8, 98i8, 121i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 59i8,
    32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8,
    114i8, 101i8, 101i8, 45i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 101i8, 100i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 98i8, 117i8, 105i8, 108i8, 100i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 100i8, 32i8, 111i8, 116i8,
    104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 59i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 84i8, 72i8, 82i8, 69i8, 65i8, 68i8, 95i8, 73i8, 78i8,
    72i8, 69i8, 82i8, 73i8, 84i8, 95i8, 67i8, 79i8, 78i8, 84i8, 69i8, 88i8, 84i8, 10i8, 45i8, 88i8,
    32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 91i8, 61i8,
    78i8, 93i8, 58i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8,
    111i8, 110i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8,
    99i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 59i8, 32i8, 78i8, 32i8, 115i8, 101i8, 116i8,
    115i8, 32i8, 97i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 108i8,
    105i8, 109i8, 105i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 111i8,
    102i8, 32i8, 78i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 115i8, 32i8, 40i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 49i8, 41i8, 59i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 84i8, 82i8, 65i8, 67i8, 69i8, 77i8, 65i8,
    76i8, 76i8, 79i8, 67i8, 61i8, 78i8, 10i8, 45i8, 88i8, 32i8, 117i8, 116i8, 102i8, 56i8, 91i8,
    61i8, 48i8, 124i8, 49i8, 93i8, 58i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 40i8,
    49i8, 41i8, 32i8, 111i8, 114i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    40i8, 48i8, 41i8, 32i8, 85i8, 84i8, 70i8, 45i8, 56i8, 32i8, 109i8, 111i8, 100i8, 101i8, 59i8,
    32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 85i8, 84i8, 70i8,
    56i8, 10i8, 45i8, 88i8, 32i8, 119i8, 97i8, 114i8, 110i8, 95i8, 100i8, 101i8, 102i8, 97i8,
    117i8, 108i8, 116i8, 95i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 58i8, 32i8,
    101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 112i8, 116i8, 45i8, 105i8, 110i8, 32i8,
    69i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 87i8, 97i8, 114i8, 110i8, 105i8, 110i8,
    103i8, 32i8, 102i8, 111i8, 114i8, 32i8, 39i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8,
    103i8, 61i8, 78i8, 111i8, 110i8, 101i8, 39i8, 59i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 97i8, 108i8, 115i8, 111i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 87i8,
    65i8, 82i8, 78i8, 68i8, 69i8, 70i8, 65i8, 85i8, 76i8, 84i8, 69i8, 78i8, 67i8, 79i8, 68i8, 73i8,
    78i8, 71i8, 0i8,
]);

/// Envvars that don't have equivalent command-line options are listed first
static usage_envvars: __SlateAlign16<[i8; 3976]> = __SlateAlign16([
    69i8, 110i8, 118i8, 105i8, 114i8, 111i8, 110i8, 109i8, 101i8, 110i8, 116i8, 32i8, 118i8, 97i8,
    114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8,
    104i8, 97i8, 110i8, 103i8, 101i8, 32i8, 98i8, 101i8, 104i8, 97i8, 118i8, 105i8, 111i8, 114i8,
    58i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 65i8, 83i8, 89i8, 78i8, 67i8, 73i8, 79i8, 68i8,
    69i8, 66i8, 85i8, 71i8, 58i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 115i8,
    121i8, 110i8, 99i8, 105i8, 111i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 109i8, 111i8,
    100i8, 101i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 66i8, 65i8, 83i8, 73i8, 67i8,
    95i8, 82i8, 69i8, 80i8, 76i8, 58i8, 32i8, 117i8, 115i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    116i8, 114i8, 97i8, 100i8, 105i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 112i8, 97i8,
    114i8, 115i8, 101i8, 114i8, 45i8, 98i8, 97i8, 115i8, 101i8, 100i8, 32i8, 82i8, 69i8, 80i8,
    76i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 66i8, 82i8, 69i8, 65i8, 75i8, 80i8, 79i8, 73i8,
    78i8, 84i8, 58i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 118i8, 97i8,
    114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 115i8, 101i8, 116i8, 32i8,
    116i8, 111i8, 32i8, 48i8, 44i8, 32i8, 105i8, 116i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8,
    108i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 103i8, 101i8, 114i8, 46i8,
    32i8, 32i8, 73i8, 116i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 115i8, 101i8, 116i8,
    32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 97i8, 98i8,
    108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 121i8, 111i8, 117i8, 114i8, 32i8, 100i8, 101i8, 98i8,
    117i8, 103i8, 103i8, 101i8, 114i8, 32i8, 111i8, 102i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 99i8, 104i8,
    111i8, 105i8, 99i8, 101i8, 46i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 67i8, 65i8, 83i8,
    69i8, 79i8, 75i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 105i8, 103i8, 110i8, 111i8, 114i8, 101i8,
    32i8, 99i8, 97i8, 115i8, 101i8, 32i8, 105i8, 110i8, 32i8, 39i8, 105i8, 109i8, 112i8, 111i8,
    114i8, 116i8, 39i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8,
    32i8, 40i8, 87i8, 105i8, 110i8, 100i8, 111i8, 119i8, 115i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8,
    79i8, 78i8, 67i8, 79i8, 69i8, 82i8, 67i8, 69i8, 67i8, 76i8, 79i8, 67i8, 65i8, 76i8, 69i8, 58i8,
    32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8,
    98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8,
    48i8, 44i8, 32i8, 105i8, 116i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 115i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    99i8, 111i8, 101i8, 114i8, 99i8, 105i8, 111i8, 110i8, 32i8, 98i8, 101i8, 104i8, 97i8, 118i8,
    105i8, 111i8, 114i8, 46i8, 32i8, 32i8, 85i8, 115i8, 101i8, 32i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 67i8, 79i8, 69i8, 82i8, 67i8, 69i8, 67i8, 76i8, 79i8, 67i8, 65i8, 76i8, 69i8, 61i8,
    119i8, 97i8, 114i8, 110i8, 32i8, 116i8, 111i8, 32i8, 114i8, 101i8, 113i8, 117i8, 101i8, 115i8,
    116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 32i8, 111i8,
    102i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 32i8, 99i8, 111i8, 101i8, 114i8, 99i8,
    105i8, 111i8, 110i8, 32i8, 97i8, 110i8, 100i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8,
    32i8, 99i8, 111i8, 109i8, 112i8, 97i8, 116i8, 105i8, 98i8, 105i8, 108i8, 105i8, 116i8, 121i8,
    32i8, 119i8, 97i8, 114i8, 110i8, 105i8, 110i8, 103i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 111i8,
    110i8, 32i8, 115i8, 116i8, 100i8, 101i8, 114i8, 114i8, 46i8, 10i8, 80i8, 89i8, 84i8, 72i8,
    79i8, 78i8, 95i8, 67i8, 79i8, 76i8, 79i8, 82i8, 83i8, 32i8, 32i8, 32i8, 58i8, 32i8, 105i8,
    102i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8, 108i8,
    101i8, 32i8, 105i8, 115i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8, 49i8, 44i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8,
    101i8, 114i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 99i8, 111i8, 108i8, 111i8, 114i8, 105i8,
    122i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 118i8, 97i8, 114i8, 105i8, 111i8, 117i8, 115i8, 32i8,
    107i8, 105i8, 110i8, 100i8, 115i8, 32i8, 111i8, 102i8, 32i8, 111i8, 117i8, 116i8, 112i8, 117i8,
    116i8, 46i8, 32i8, 32i8, 83i8, 101i8, 116i8, 116i8, 105i8, 110i8, 103i8, 32i8, 105i8, 116i8,
    32i8, 116i8, 111i8, 32i8, 48i8, 32i8, 100i8, 101i8, 97i8, 99i8, 116i8, 105i8, 118i8, 97i8,
    116i8, 101i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 98i8, 101i8, 104i8,
    97i8, 118i8, 105i8, 111i8, 114i8, 46i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 72i8, 65i8,
    83i8, 72i8, 83i8, 69i8, 69i8, 68i8, 32i8, 32i8, 58i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8,
    105i8, 115i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8,
    32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8, 39i8, 114i8, 97i8, 110i8, 100i8, 111i8,
    109i8, 39i8, 44i8, 32i8, 97i8, 32i8, 114i8, 97i8, 110i8, 100i8, 111i8, 109i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 32i8, 105i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 116i8, 111i8, 32i8, 115i8, 101i8, 101i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 104i8,
    97i8, 115i8, 104i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 115i8, 116i8, 114i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 115i8, 46i8, 32i8, 32i8, 73i8, 116i8, 32i8, 99i8, 97i8, 110i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 98i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8,
    97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 114i8, 97i8, 110i8, 103i8, 101i8, 32i8, 91i8, 48i8, 44i8, 52i8,
    50i8, 57i8, 52i8, 57i8, 54i8, 55i8, 50i8, 57i8, 53i8, 93i8, 32i8, 116i8, 111i8, 32i8, 103i8,
    101i8, 116i8, 32i8, 104i8, 97i8, 115i8, 104i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 32i8, 112i8, 114i8, 101i8, 100i8,
    105i8, 99i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 115i8, 101i8, 101i8, 100i8, 46i8, 10i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 72i8, 73i8, 83i8, 84i8, 79i8, 82i8, 89i8, 32i8, 32i8,
    58i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 111i8, 102i8, 32i8, 97i8, 32i8, 46i8, 112i8, 121i8, 116i8, 104i8, 111i8, 110i8, 95i8,
    104i8, 105i8, 115i8, 116i8, 111i8, 114i8, 121i8, 32i8, 102i8, 105i8, 108i8, 101i8, 46i8, 10i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 72i8, 79i8, 77i8, 69i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    58i8, 32i8, 97i8, 108i8, 116i8, 101i8, 114i8, 110i8, 97i8, 116i8, 101i8, 32i8, 60i8, 112i8,
    114i8, 101i8, 102i8, 105i8, 120i8, 62i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8,
    114i8, 121i8, 32i8, 40i8, 111i8, 114i8, 32i8, 60i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8,
    62i8, 37i8, 108i8, 99i8, 60i8, 101i8, 120i8, 101i8, 99i8, 95i8, 112i8, 114i8, 101i8, 102i8,
    105i8, 120i8, 62i8, 41i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 115i8,
    101i8, 97i8, 114i8, 99i8, 104i8, 32i8, 112i8, 97i8, 116i8, 104i8, 32i8, 117i8, 115i8, 101i8,
    115i8, 32i8, 37i8, 115i8, 46i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 73i8, 79i8, 69i8,
    78i8, 67i8, 79i8, 68i8, 73i8, 78i8, 71i8, 58i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8,
    110i8, 103i8, 91i8, 58i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 93i8, 32i8, 117i8, 115i8,
    101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 115i8, 116i8, 100i8, 105i8, 110i8, 47i8, 115i8,
    116i8, 100i8, 111i8, 117i8, 116i8, 47i8, 115i8, 116i8, 100i8, 101i8, 114i8, 114i8, 10i8, 80i8,
    89i8, 84i8, 72i8, 79i8, 78i8, 77i8, 65i8, 76i8, 76i8, 79i8, 67i8, 32i8, 32i8, 32i8, 32i8, 58i8,
    32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8,
    97i8, 116i8, 111i8, 114i8, 115i8, 32i8, 97i8, 110i8, 100i8, 47i8, 111i8, 114i8, 32i8, 105i8,
    110i8, 115i8, 116i8, 97i8, 108i8, 108i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 104i8,
    111i8, 111i8, 107i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 111i8, 110i8, 32i8, 80i8, 121i8, 116i8, 104i8,
    111i8, 110i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8,
    99i8, 97i8, 116i8, 111i8, 114i8, 115i8, 46i8, 32i8, 32i8, 85i8, 115i8, 101i8, 32i8, 80i8, 89i8,
    84i8, 72i8, 79i8, 78i8, 77i8, 65i8, 76i8, 76i8, 79i8, 67i8, 61i8, 100i8, 101i8, 98i8, 117i8,
    103i8, 32i8, 116i8, 111i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 105i8, 110i8, 115i8, 116i8, 97i8, 108i8, 108i8,
    32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 104i8, 111i8, 111i8, 107i8, 115i8, 46i8, 10i8,
    80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 77i8, 65i8, 76i8, 76i8, 79i8, 67i8, 83i8, 84i8, 65i8, 84i8,
    83i8, 58i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8,
    121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 111i8, 114i8, 32i8, 115i8, 116i8,
    97i8, 116i8, 105i8, 115i8, 116i8, 105i8, 99i8, 115i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8,
    80i8, 65i8, 84i8, 72i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 39i8, 37i8, 108i8,
    99i8, 39i8, 45i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 101i8, 100i8, 32i8, 108i8,
    105i8, 115i8, 116i8, 32i8, 111i8, 102i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8,
    114i8, 105i8, 101i8, 115i8, 32i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 101i8, 100i8, 32i8,
    116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 115i8, 101i8, 97i8, 114i8,
    99i8, 104i8, 32i8, 112i8, 97i8, 116i8, 104i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8,
    114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 115i8, 121i8, 115i8, 46i8,
    112i8, 97i8, 116i8, 104i8, 46i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 80i8, 76i8, 65i8,
    84i8, 76i8, 73i8, 66i8, 68i8, 73i8, 82i8, 58i8, 32i8, 111i8, 118i8, 101i8, 114i8, 114i8, 105i8,
    100i8, 101i8, 32i8, 115i8, 121i8, 115i8, 46i8, 112i8, 108i8, 97i8, 116i8, 108i8, 105i8, 98i8,
    100i8, 105i8, 114i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 83i8, 84i8, 65i8, 82i8, 84i8,
    85i8, 80i8, 32i8, 32i8, 32i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 101i8, 120i8,
    101i8, 99i8, 117i8, 116i8, 101i8, 100i8, 32i8, 111i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 117i8,
    112i8, 32i8, 40i8, 110i8, 111i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 41i8,
    10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 85i8, 83i8, 69i8, 82i8, 66i8, 65i8, 83i8, 69i8, 32i8,
    32i8, 58i8, 32i8, 100i8, 101i8, 102i8, 105i8, 110i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 117i8, 115i8, 101i8, 114i8, 32i8, 98i8, 97i8, 115i8, 101i8, 32i8, 100i8, 105i8, 114i8,
    101i8, 99i8, 116i8, 111i8, 114i8, 121i8, 32i8, 40i8, 115i8, 105i8, 116i8, 101i8, 46i8, 85i8,
    83i8, 69i8, 82i8, 95i8, 66i8, 65i8, 83i8, 69i8, 41i8, 10i8, 10i8, 84i8, 104i8, 101i8, 115i8,
    101i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 115i8, 32i8, 104i8, 97i8,
    118i8, 101i8, 32i8, 101i8, 113i8, 117i8, 105i8, 118i8, 97i8, 108i8, 101i8, 110i8, 116i8, 32i8,
    99i8, 111i8, 109i8, 109i8, 97i8, 110i8, 100i8, 45i8, 108i8, 105i8, 110i8, 101i8, 32i8, 111i8,
    112i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 40i8, 115i8, 101i8, 101i8, 32i8, 45i8, 45i8,
    104i8, 101i8, 108i8, 112i8, 32i8, 102i8, 111i8, 114i8, 32i8, 100i8, 101i8, 116i8, 97i8, 105i8,
    108i8, 115i8, 41i8, 58i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 67i8, 79i8, 78i8,
    84i8, 69i8, 88i8, 84i8, 95i8, 65i8, 87i8, 65i8, 82i8, 69i8, 95i8, 87i8, 65i8, 82i8, 78i8, 73i8,
    78i8, 71i8, 83i8, 58i8, 32i8, 105i8, 102i8, 32i8, 116i8, 114i8, 117i8, 101i8, 32i8, 40i8, 49i8,
    41i8, 44i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 116i8, 104i8, 114i8, 101i8,
    97i8, 100i8, 45i8, 115i8, 97i8, 102i8, 101i8, 32i8, 119i8, 97i8, 114i8, 110i8, 105i8, 110i8,
    103i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 98i8,
    101i8, 104i8, 97i8, 118i8, 105i8, 111i8, 117i8, 114i8, 32i8, 40i8, 45i8, 88i8, 32i8, 99i8,
    111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 95i8, 97i8, 119i8, 97i8, 114i8, 101i8, 95i8, 119i8,
    97i8, 114i8, 110i8, 105i8, 110i8, 103i8, 115i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8,
    95i8, 67i8, 80i8, 85i8, 95i8, 67i8, 79i8, 85i8, 78i8, 84i8, 58i8, 32i8, 111i8, 118i8, 101i8,
    114i8, 114i8, 105i8, 100i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8, 32i8, 111i8, 115i8,
    46i8, 99i8, 112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 41i8, 32i8, 40i8, 45i8,
    88i8, 32i8, 99i8, 112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8, 41i8, 10i8, 80i8, 89i8,
    84i8, 72i8, 79i8, 78i8, 68i8, 69i8, 66i8, 85i8, 71i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8,
    101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 112i8, 97i8, 114i8, 115i8, 101i8, 114i8, 32i8,
    100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 40i8, 45i8, 100i8,
    41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 68i8, 69i8, 86i8, 77i8, 79i8, 68i8, 69i8, 32i8,
    32i8, 32i8, 58i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 80i8, 121i8, 116i8,
    104i8, 111i8, 110i8, 32i8, 68i8, 101i8, 118i8, 101i8, 108i8, 111i8, 112i8, 109i8, 101i8, 110i8,
    116i8, 32i8, 77i8, 111i8, 100i8, 101i8, 32i8, 40i8, 45i8, 88i8, 32i8, 100i8, 101i8, 118i8,
    41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 68i8, 79i8, 78i8, 84i8, 87i8, 82i8, 73i8, 84i8,
    69i8, 66i8, 89i8, 84i8, 69i8, 67i8, 79i8, 68i8, 69i8, 58i8, 32i8, 100i8, 111i8, 110i8, 39i8,
    116i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 32i8, 46i8, 112i8, 121i8, 99i8, 32i8, 102i8,
    105i8, 108i8, 101i8, 115i8, 32i8, 40i8, 45i8, 66i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 70i8, 65i8, 85i8, 76i8, 84i8, 72i8, 65i8, 78i8, 68i8, 76i8, 69i8, 82i8, 58i8, 32i8,
    100i8, 117i8, 109i8, 112i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 111i8, 110i8,
    32i8, 102i8, 97i8, 116i8, 97i8, 108i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 32i8,
    40i8, 45i8, 88i8, 32i8, 102i8, 97i8, 117i8, 108i8, 116i8, 104i8, 97i8, 110i8, 100i8, 108i8,
    101i8, 114i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 70i8, 82i8, 79i8, 90i8,
    69i8, 78i8, 95i8, 77i8, 79i8, 68i8, 85i8, 76i8, 69i8, 83i8, 58i8, 32i8, 119i8, 104i8, 101i8,
    116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 102i8, 114i8,
    111i8, 122i8, 101i8, 110i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 59i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8,
    32i8, 34i8, 111i8, 110i8, 34i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 105i8, 110i8,
    115i8, 116i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 34i8, 111i8, 102i8, 102i8, 34i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 97i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 32i8, 98i8, 117i8, 105i8, 108i8, 100i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 40i8, 45i8, 88i8, 32i8, 102i8, 114i8, 111i8, 122i8, 101i8, 110i8, 95i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 73i8, 78i8, 83i8, 80i8, 69i8, 67i8, 84i8, 32i8, 32i8, 32i8, 58i8, 32i8, 105i8, 110i8,
    115i8, 112i8, 101i8, 99i8, 116i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8,
    105i8, 118i8, 101i8, 108i8, 121i8, 32i8, 97i8, 102i8, 116i8, 101i8, 114i8, 32i8, 114i8, 117i8,
    110i8, 110i8, 105i8, 110i8, 103i8, 32i8, 115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 32i8, 40i8,
    45i8, 105i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 73i8, 78i8, 84i8, 77i8, 65i8,
    88i8, 83i8, 84i8, 82i8, 68i8, 73i8, 71i8, 73i8, 84i8, 83i8, 58i8, 32i8, 108i8, 105i8, 109i8,
    105i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 105i8, 110i8, 116i8, 60i8, 45i8, 62i8, 115i8, 116i8, 114i8, 32i8, 99i8, 111i8, 110i8,
    118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 115i8, 59i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 48i8, 32i8,
    100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8,
    105i8, 109i8, 105i8, 116i8, 32i8, 40i8, 45i8, 88i8, 32i8, 105i8, 110i8, 116i8, 95i8, 109i8,
    97i8, 120i8, 95i8, 115i8, 116i8, 114i8, 95i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 61i8,
    78i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 78i8, 79i8, 68i8, 69i8, 66i8, 85i8, 71i8,
    82i8, 65i8, 78i8, 71i8, 69i8, 83i8, 58i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 105i8,
    110i8, 99i8, 108i8, 117i8, 100i8, 101i8, 32i8, 101i8, 120i8, 116i8, 114i8, 97i8, 32i8, 108i8,
    111i8, 99i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 40i8, 45i8, 88i8, 32i8,
    110i8, 111i8, 95i8, 100i8, 101i8, 98i8, 117i8, 103i8, 95i8, 114i8, 97i8, 110i8, 103i8, 101i8,
    115i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 78i8, 79i8, 85i8, 83i8, 69i8, 82i8,
    83i8, 73i8, 84i8, 69i8, 58i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8, 117i8,
    115i8, 101i8, 114i8, 32i8, 115i8, 105i8, 116i8, 101i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8,
    116i8, 111i8, 114i8, 121i8, 32i8, 40i8, 45i8, 115i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 79i8, 80i8, 84i8, 73i8, 77i8, 73i8, 90i8, 69i8, 32i8, 32i8, 58i8, 32i8, 101i8, 110i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 108i8, 101i8, 118i8, 101i8, 108i8, 32i8, 49i8, 32i8, 111i8,
    112i8, 116i8, 105i8, 109i8, 105i8, 122i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 40i8,
    45i8, 79i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 95i8, 80i8, 69i8, 82i8, 70i8, 95i8,
    74i8, 73i8, 84i8, 95i8, 83i8, 85i8, 80i8, 80i8, 79i8, 82i8, 84i8, 58i8, 32i8, 101i8, 110i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 76i8, 105i8, 110i8, 117i8, 120i8, 32i8, 34i8, 112i8, 101i8,
    114i8, 102i8, 34i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 32i8, 115i8,
    117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 74i8, 73i8,
    84i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 40i8, 45i8, 88i8, 32i8, 112i8, 101i8, 114i8, 102i8, 95i8, 106i8, 105i8,
    116i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 80i8, 69i8, 82i8, 70i8, 83i8, 85i8,
    80i8, 80i8, 79i8, 82i8, 84i8, 58i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 76i8, 105i8, 110i8, 117i8, 120i8, 32i8, 34i8, 112i8, 101i8,
    114i8, 102i8, 34i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 32i8, 40i8,
    45i8, 88i8, 32i8, 112i8, 101i8, 114i8, 102i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8,
    80i8, 82i8, 79i8, 70i8, 73i8, 76i8, 69i8, 73i8, 77i8, 80i8, 79i8, 82i8, 84i8, 84i8, 73i8, 77i8,
    69i8, 58i8, 32i8, 115i8, 104i8, 111i8, 119i8, 32i8, 104i8, 111i8, 119i8, 32i8, 108i8, 111i8,
    110i8, 103i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8,
    32i8, 116i8, 97i8, 107i8, 101i8, 115i8, 32i8, 40i8, 45i8, 88i8, 32i8, 105i8, 109i8, 112i8,
    111i8, 114i8, 116i8, 116i8, 105i8, 109i8, 101i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8,
    78i8, 80i8, 89i8, 67i8, 65i8, 67i8, 72i8, 69i8, 80i8, 82i8, 69i8, 70i8, 73i8, 88i8, 58i8, 32i8,
    114i8, 111i8, 111i8, 116i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8, 114i8, 121i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8, 101i8, 99i8, 111i8, 100i8, 101i8, 32i8,
    99i8, 97i8, 99i8, 104i8, 101i8, 32i8, 40i8, 112i8, 121i8, 99i8, 41i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 40i8, 45i8, 88i8, 32i8, 112i8, 121i8, 99i8, 97i8,
    99i8, 104i8, 101i8, 95i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 41i8, 10i8, 80i8, 89i8,
    84i8, 72i8, 79i8, 78i8, 83i8, 65i8, 70i8, 69i8, 80i8, 65i8, 84i8, 72i8, 32i8, 32i8, 58i8, 32i8,
    100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 112i8, 114i8, 101i8, 112i8, 101i8, 110i8, 100i8, 32i8,
    97i8, 32i8, 112i8, 111i8, 116i8, 101i8, 110i8, 116i8, 105i8, 97i8, 108i8, 108i8, 121i8, 32i8,
    117i8, 110i8, 115i8, 97i8, 102i8, 101i8, 32i8, 112i8, 97i8, 116i8, 104i8, 32i8, 116i8, 111i8,
    32i8, 115i8, 121i8, 115i8, 46i8, 112i8, 97i8, 116i8, 104i8, 46i8, 10i8, 80i8, 89i8, 84i8, 72i8,
    79i8, 78i8, 95i8, 84i8, 72i8, 82i8, 69i8, 65i8, 68i8, 95i8, 73i8, 78i8, 72i8, 69i8, 82i8, 73i8,
    84i8, 95i8, 67i8, 79i8, 78i8, 84i8, 69i8, 88i8, 84i8, 58i8, 32i8, 105i8, 102i8, 32i8, 116i8,
    114i8, 117i8, 101i8, 32i8, 40i8, 49i8, 41i8, 44i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8,
    100i8, 115i8, 32i8, 105i8, 110i8, 104i8, 101i8, 114i8, 105i8, 116i8, 32i8, 99i8, 111i8, 110i8,
    116i8, 101i8, 120i8, 116i8, 32i8, 118i8, 97i8, 114i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 40i8, 45i8,
    88i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 95i8, 105i8, 110i8, 104i8, 101i8, 114i8,
    105i8, 116i8, 95i8, 99i8, 111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 41i8, 10i8, 80i8, 89i8,
    84i8, 72i8, 79i8, 78i8, 84i8, 82i8, 65i8, 67i8, 69i8, 77i8, 65i8, 76i8, 76i8, 79i8, 67i8, 58i8,
    32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8,
    109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 115i8, 32i8, 40i8, 45i8, 88i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8,
    109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 85i8,
    78i8, 66i8, 85i8, 70i8, 70i8, 69i8, 82i8, 69i8, 68i8, 58i8, 32i8, 100i8, 105i8, 115i8, 97i8,
    98i8, 108i8, 101i8, 32i8, 115i8, 116i8, 100i8, 111i8, 117i8, 116i8, 47i8, 115i8, 116i8, 100i8,
    101i8, 114i8, 114i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 105i8, 110i8, 103i8, 32i8,
    40i8, 45i8, 117i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 85i8, 84i8, 70i8, 56i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 58i8, 32i8, 99i8, 111i8, 110i8, 116i8, 114i8, 111i8, 108i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 85i8, 84i8, 70i8, 45i8, 56i8, 32i8, 109i8, 111i8, 100i8,
    101i8, 32i8, 40i8, 45i8, 88i8, 32i8, 117i8, 116i8, 102i8, 56i8, 41i8, 10i8, 80i8, 89i8, 84i8,
    72i8, 79i8, 78i8, 86i8, 69i8, 82i8, 66i8, 79i8, 83i8, 69i8, 32i8, 32i8, 32i8, 58i8, 32i8,
    116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 32i8, 115i8,
    116i8, 97i8, 116i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 40i8, 45i8, 118i8, 41i8,
    10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 87i8, 65i8, 82i8, 78i8, 68i8, 69i8, 70i8, 65i8, 85i8,
    76i8, 84i8, 69i8, 78i8, 67i8, 79i8, 68i8, 73i8, 78i8, 71i8, 58i8, 32i8, 101i8, 110i8, 97i8,
    98i8, 108i8, 101i8, 32i8, 111i8, 112i8, 116i8, 45i8, 105i8, 110i8, 32i8, 69i8, 110i8, 99i8,
    111i8, 100i8, 105i8, 110i8, 103i8, 87i8, 97i8, 114i8, 110i8, 105i8, 110i8, 103i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 39i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 61i8, 78i8,
    111i8, 110i8, 101i8, 39i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 40i8, 45i8, 88i8, 32i8, 119i8, 97i8, 114i8,
    110i8, 95i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 95i8, 101i8, 110i8, 99i8, 111i8,
    100i8, 105i8, 110i8, 103i8, 41i8, 10i8, 80i8, 89i8, 84i8, 72i8, 79i8, 78i8, 87i8, 65i8, 82i8,
    78i8, 73i8, 78i8, 71i8, 83i8, 32i8, 32i8, 58i8, 32i8, 119i8, 97i8, 114i8, 110i8, 105i8, 110i8,
    103i8, 32i8, 99i8, 111i8, 110i8, 116i8, 114i8, 111i8, 108i8, 32i8, 40i8, 45i8, 87i8, 41i8,
    10i8, 0i8,
]);

// --- Global configuration variables -----------------------------
/// UTF-8 mode (PEP 540): if equals to 1, use the UTF-8 encoding, and change
/// stdin and stdout error handler to "surrogateescape".
#[unsafe(no_mangle)]
static mut Py_UTF8Mode: i32 = 0;

/// Needed by parser.c
#[unsafe(no_mangle)]
static mut Py_DebugFlag: i32 = 0;

/// Needed by import.c
#[unsafe(no_mangle)]
static mut Py_VerboseFlag: i32 = 0;

/// Needed by sysmodule.c
#[unsafe(no_mangle)]
static mut Py_QuietFlag: i32 = 0;

/// Previously, was used by Py_FdIsInteractive()
#[unsafe(no_mangle)]
static mut Py_InteractiveFlag: i32 = 0;

/// Needed to determine whether to exit at SystemExit
#[unsafe(no_mangle)]
static mut Py_InspectFlag: i32 = 0;

/// Needed by compile.c
#[unsafe(no_mangle)]
static mut Py_OptimizeFlag: i32 = 0;

/// Suppress 'import site'
#[unsafe(no_mangle)]
static mut Py_NoSiteFlag: i32 = 0;

/// Warn on str(bytes) and str(buffer)
#[unsafe(no_mangle)]
static mut Py_BytesWarningFlag: i32 = 0;

/// Needed by getpath.c
#[unsafe(no_mangle)]
static mut Py_FrozenFlag: i32 = 0;

/// e.g. PYTHONPATH, PYTHONHOME
#[unsafe(no_mangle)]
static mut Py_IgnoreEnvironmentFlag: i32 = 0;

/// Suppress writing bytecode files (*.pyc)
#[unsafe(no_mangle)]
static mut Py_DontWriteBytecodeFlag: i32 = 0;

/// for -s and site.py
#[unsafe(no_mangle)]
static mut Py_NoUserSiteDirectory: i32 = 0;

/// Unbuffered binary std{in,out,err}
#[unsafe(no_mangle)]
static mut Py_UnbufferedStdioFlag: i32 = 0;

/// for -R and PYTHONHASHSEED
#[unsafe(no_mangle)]
static mut Py_HashRandomizationFlag: i32 = 0;

/// for -I, isolate from user's env
#[unsafe(no_mangle)]
static mut Py_IsolatedFlag: i32 = 0;

fn _Py_GetGlobalVariablesAsDict() -> *mut _object {
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        dict = PyDict_New();
        if dict.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                loop {
                    obj = FROM_STRING!(unsafe { Py_FileSystemDefaultEncoding });
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(
                            dict,
                            c"Py_FileSystemDefaultEncoding".as_ptr(),
                            obj,
                        );
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_HasFileSystemDefaultEncoding as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(
                            dict,
                            c"Py_HasFileSystemDefaultEncoding".as_ptr(),
                            obj,
                        );
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = FROM_STRING!(unsafe { Py_FileSystemDefaultEncodeErrors });
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(
                            dict,
                            c"Py_FileSystemDefaultEncodeErrors".as_ptr(),
                            obj,
                        );
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(_Py_HasFileSystemDefaultEncodeErrors as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(
                            dict,
                            c"_Py_HasFileSystemDefaultEncodeErrors".as_ptr(),
                            obj,
                        );
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_UTF8Mode as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_UTF8Mode".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_DebugFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_DebugFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_VerboseFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_VerboseFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_QuietFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_QuietFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_InteractiveFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_InteractiveFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_InspectFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_InspectFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_OptimizeFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_OptimizeFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_NoSiteFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_NoSiteFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_BytesWarningFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_BytesWarningFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_FrozenFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_FrozenFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_IgnoreEnvironmentFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_IgnoreEnvironmentFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_DontWriteBytecodeFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_DontWriteBytecodeFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_NoUserSiteDirectory as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_NoUserSiteDirectory".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_UnbufferedStdioFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_UnbufferedStdioFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_HashRandomizationFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"Py_HashRandomizationFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong(Py_IsolatedFlag as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"Py_IsolatedFlag".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                return dict;
            }
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GETENV(name: *const i8) -> *mut i8 {
    if (unsafe { Py_IgnoreEnvironmentFlag }) != 0 {
        return core::ptr::null_mut::<i8>();
    }
    unsafe { getenv(name as *const core::ffi::c_char) as *mut i8 }
}

// --- PyStatus -----------------------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyStatus_Ok() -> __SlateRecord_initconfig_h_19 {
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
extern "C-unwind" fn PyStatus_Error(err_msg: *const i8) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    return unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.err_msg = err_msg;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyStatus_NoMemory() -> __SlateRecord_initconfig_h_19 {
    return PyStatus_Error(c"memory allocation failed".as_ptr());
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyStatus_Exit(exitcode: i32) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    return unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 2u32;
                __t0.exitcode = exitcode;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyStatus_IsError(status: __SlateRecord_initconfig_h_19) -> i32 {
    _PyStatus_IS_ERROR!(status) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyStatus_IsExit(status: __SlateRecord_initconfig_h_19) -> i32 {
    _PyStatus_IS_EXIT!(status) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyStatus_Exception(status: __SlateRecord_initconfig_h_19) -> i32 {
    _PyStatus_EXCEPTION!(status) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyErr_SetFromPyStatus(status: __SlateRecord_initconfig_h_19) {
    if !_PyStatus_IS_ERROR!(status) {
        unsafe {
            PyErr_Format(
                PyExc_SystemError,
                c"_PyErr_SetFromPyStatus() status is not an error".as_ptr(),
            )
        };
        return;
    }
    let err_msg: *const i8 = status.err_msg;
    if err_msg.is_null() || (unsafe { strlen(err_msg as *const core::ffi::c_char) as u64 }) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_SystemError,
                c"_PyErr_SetFromPyStatus() status has no error message".as_ptr(),
            )
        };
        return;
    }
    if (unsafe {
        strcmp(
            err_msg as *const core::ffi::c_char,
            c"memory allocation failed".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        unsafe { PyErr_NoMemory() };
        return;
    }
    let func: *const i8 = status.func;
    if !func.is_null() {
        unsafe { PyErr_Format(PyExc_RuntimeError, c"%s: %s".as_ptr(), func, err_msg) };
    } else {
        unsafe { PyErr_Format(PyExc_RuntimeError, c"%s".as_ptr(), err_msg) };
    }
}

// --- PyWideStringList ------------------------------------------------
fn _PyWideStringList_ClearEx(
    list: *mut __SlateRecord_initconfig_h_36,
    use_default_allocator: bool,
) {
    let mut i: i64 = 0;
    while i < unsafe { (*list).length } {
        if use_default_allocator {
            unsafe {
                _PyMem_DefaultRawFree(
                    *unsafe { (*list).items }.wrapping_offset(i as isize) as *mut ()
                )
            };
        } else {
            unsafe {
                PyMem_RawFree(*unsafe { (*list).items }.wrapping_offset(i as isize) as *mut ())
            };
        }
        i = i.wrapping_add(1);
    }
    if use_default_allocator {
        unsafe { _PyMem_DefaultRawFree((*list).items as *mut ()) };
    } else {
        unsafe { PyMem_RawFree((*list).items as *mut ()) };
    }
    unsafe {
        (*list).length = 0;
    }
    unsafe {
        (*list).items = core::ptr::null_mut::<*mut i32>();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyWideStringList_Clear(list: *mut __SlateRecord_initconfig_h_36) {
    _PyWideStringList_ClearEx(list, false);
}

fn _PyWideStringList_CopyEx(
    list: *mut __SlateRecord_initconfig_h_36,
    list2: *const __SlateRecord_initconfig_h_36,
    use_default_allocator: bool,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    if (unsafe { (*list2).length }) == 0 {
        _PyWideStringList_ClearEx(list, use_default_allocator);
        return 0;
    }
    let mut copy: __SlateRecord_initconfig_h_36 = unsafe {
        *{
            __slate_compound_0 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    let size: u64 =
        ((unsafe { (*list2).length }) as u64).wrapping_mul(core::mem::size_of::<*mut i32>() as u64);
    if use_default_allocator {
        copy.items = (unsafe { _PyMem_DefaultRawMalloc(size) }) as *mut *mut i32;
    } else {
        copy.items = (unsafe { PyMem_RawMalloc(size) }) as *mut *mut i32;
    }
    if copy.items.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut i: i64 = 0;
    while i < unsafe { (*list2).length } {
        let mut item: *mut i32 = unsafe { core::mem::zeroed() };
        if use_default_allocator {
            item = unsafe {
                _PyMem_DefaultRawWcsdup(
                    *unsafe { (*list2).items }.wrapping_offset(i as isize) as *const i32
                )
            };
        } else {
            item = unsafe {
                _PyMem_RawWcsdup(
                    *unsafe { (*list2).items }.wrapping_offset(i as isize) as *const i32
                )
            };
        }
        if item.is_null() {
            _PyWideStringList_ClearEx(core::ptr::addr_of_mut!(copy), use_default_allocator);
            return 1i32.wrapping_neg();
        }
        unsafe {
            *copy.items.wrapping_offset(i as isize) = item;
        }
        copy.length = i.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    _PyWideStringList_ClearEx(list, use_default_allocator);
    unsafe {
        *list = copy;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyWideStringList_Copy(
    list: *mut __SlateRecord_initconfig_h_36,
    list2: *const __SlateRecord_initconfig_h_36,
) -> i32 {
    _PyWideStringList_CopyEx(list, list2, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyWideStringList_Insert(
    list: *mut __SlateRecord_initconfig_h_36,
    mut index: i64,
    item: *const i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_7: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let len: i64 = unsafe { (*list).length };
    if len == 9223372036854775807 {
        // length+1 would overflow
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__4!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    if index < 0 {
        return unsafe {
            *{
                __slate_compound_1 = {
                    let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t1._type = 1u32;
                    __t1.func = _PyStatus_GET_FUNC__4!() as *const i8;
                    __t1.err_msg = c"PyWideStringList_Insert index must be >= 0".as_ptr();
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            }
        };
    }
    if index > len {
        index = len;
    }
    let item2: *mut i32 = unsafe { _PyMem_RawWcsdup(item) };
    if item2.is_null() {
        return unsafe {
            *{
                __slate_compound_2 = {
                    let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t2._type = 1u32;
                    __t2.func = _PyStatus_GET_FUNC__4!() as *const i8;
                    __t2.err_msg = c"memory allocation failed".as_ptr();
                    __t2
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            }
        };
    }
    let size: u64 =
        (len.wrapping_add(1) as u64).wrapping_mul(core::mem::size_of::<*mut i32>() as u64);
    let items2: *mut *mut i32 =
        (unsafe { PyMem_RawRealloc((*list).items as *mut (), size) }) as *mut *mut i32;
    if items2.is_null() {
        unsafe { PyMem_RawFree(item2 as *mut ()) };
        return unsafe {
            *{
                __slate_compound_3 = {
                    let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t3._type = 1u32;
                    __t3.func = _PyStatus_GET_FUNC__4!() as *const i8;
                    __t3.err_msg = c"memory allocation failed".as_ptr();
                    __t3
                };
                core::ptr::addr_of_mut!(__slate_compound_3)
            }
        };
    }
    if index < len {
        unsafe {
            core::ptr::copy(
                ((items2.wrapping_offset(index as isize) as *const ()) as *const core::ffi::c_void)
                    as *const u8,
                ((items2.wrapping_offset(index.wrapping_add(1) as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                ((len.wrapping_sub(index) as u64)
                    .wrapping_mul(core::mem::size_of::<*mut i32>() as u64)
                    as usize) as usize,
            )
        };
    }
    unsafe {
        *items2.wrapping_offset(index as isize) = item2;
    }
    unsafe {
        (*list).items = items2;
    }
    let __v5: i64 = unsafe { (*list).length };
    unsafe {
        (*list).length = __v5.wrapping_add(1);
    }
    return unsafe {
        *{
            __slate_compound_7 = {
                let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t4._type = 0u32;
                __t4
            };
            core::ptr::addr_of_mut!(__slate_compound_7)
        }
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyWideStringList_Append(
    list: *mut __SlateRecord_initconfig_h_36,
    item: *const i32,
) -> __SlateRecord_initconfig_h_19 {
    return PyWideStringList_Insert(list, unsafe { (*list).length }, item);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyWideStringList_Extend(
    list: *mut __SlateRecord_initconfig_h_36,
    list2: *const __SlateRecord_initconfig_h_36,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut i: i64 = 0;
    while i < unsafe { (*list2).length } {
        let status: __SlateRecord_initconfig_h_19 = PyWideStringList_Append(
            list,
            (unsafe { *unsafe { (*list2).items }.wrapping_offset(i as isize) }) as *const i32,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
        i = i.wrapping_add(1);
    }
    return unsafe {
        *{
            __slate_compound_3 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_3)
        }
    };
}

fn _PyWideStringList_Find(list: *mut __SlateRecord_initconfig_h_36, item: *const i32) -> i32 {
    let mut i: i64 = 0;
    while i < unsafe { (*list).length } {
        if (unsafe {
            wcscmp(
                *unsafe { (*list).items }.wrapping_offset(i as isize) as *const i32,
                item,
            )
        }) == 0
        {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyWideStringList_AsList(
    list: *const __SlateRecord_initconfig_h_36,
) -> *mut _object {
    let pylist: *mut _object = unsafe { PyList_New((*list).length) };
    if pylist.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut i: i64 = 0;
    while i < unsafe { (*list).length } {
        let item: *mut _object = unsafe {
            PyUnicode_FromWideChar(
                *unsafe { (*list).items }.wrapping_offset(i as isize) as *const i32,
                1i32.wrapping_neg() as i64,
            )
        };
        if item.is_null() {
            Py_DECREF!(pylist);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM!(pylist, i, item);
        i = i.wrapping_add(1);
    }
    pylist
}

fn _PyWideStringList_AsTuple(list: *const __SlateRecord_initconfig_h_36) -> *mut _object {
    let tuple: *mut _object = unsafe { PyTuple_New((*list).length) };
    if tuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut i: i64 = 0;
    while i < unsafe { (*list).length } {
        let item: *mut _object = unsafe {
            PyUnicode_FromWideChar(
                *unsafe { (*list).items }.wrapping_offset(i as isize) as *const i32,
                1i32.wrapping_neg() as i64,
            )
        };
        if item.is_null() {
            Py_DECREF!(tuple);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(tuple, i, item);
        i = i.wrapping_add(1);
    }
    tuple
}

// --- Py_GetArgcArgv() -------------------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_ClearArgcArgv() {
    _PyWideStringList_ClearEx(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.orig_argv) },
        true,
    );
}

fn _Py_SetArgcArgv(argc: i64, argv: *const *mut i32) -> i32 {
    let argv_list: __SlateRecord_initconfig_h_36 = __SlateRecord_initconfig_h_36 {
        length: argc,
        items: argv as *mut *mut i32,
    };
    // XXX _PyRuntime.orig_argv only gets cleared by Py_Main(),
    //
    // so it currently leaks for embedders.
    _PyWideStringList_CopyEx(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.orig_argv) },
        core::ptr::addr_of!(argv_list),
        true,
    )
}

/// _PyConfig_Write() calls _Py_SetArgcArgv() with PyConfig.orig_argv.
#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetArgcArgv(argc: *mut i32, argv: *mut *mut *mut i32) {
    unsafe {
        *argc = _PyRuntime.orig_argv.length as i32;
    }
    unsafe {
        *argv = _PyRuntime.orig_argv.items;
    }
}

// --- PyConfig ----------------------------------------------
/// Free memory allocated in config, but don't clear all attributes
#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_Clear(config: *mut PyConfig) {
    CLEAR!((*config).pycache_prefix);
    CLEAR!((*config).pythonpath_env);
    CLEAR!((*config).home);
    CLEAR!((*config).program_name);
    _PyWideStringList_Clear(unsafe { core::ptr::addr_of_mut!((*config).argv) });
    _PyWideStringList_Clear(unsafe { core::ptr::addr_of_mut!((*config).warnoptions) });
    _PyWideStringList_Clear(unsafe { core::ptr::addr_of_mut!((*config).xoptions) });
    _PyWideStringList_Clear(unsafe { core::ptr::addr_of_mut!((*config).module_search_paths) });
    unsafe {
        (*config).module_search_paths_set = 0;
    }
    CLEAR!((*config).stdlib_dir);
    CLEAR!((*config).executable);
    CLEAR!((*config).base_executable);
    CLEAR!((*config).prefix);
    CLEAR!((*config).base_prefix);
    CLEAR!((*config).exec_prefix);
    CLEAR!((*config).base_exec_prefix);
    CLEAR!((*config).platlibdir);
    CLEAR!((*config).sys_path_0);
    CLEAR!((*config).filesystem_encoding);
    CLEAR!((*config).filesystem_errors);
    CLEAR!((*config).stdio_encoding);
    CLEAR!((*config).stdio_errors);
    CLEAR!((*config).run_command);
    CLEAR!((*config).run_module);
    CLEAR!((*config).run_filename);
    CLEAR!((*config).check_hash_pycs_mode);
    _PyWideStringList_Clear(unsafe { core::ptr::addr_of_mut!((*config).orig_argv) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_InitCompatConfig(config: *mut PyConfig) {
    unsafe {
        core::ptr::write_bytes(
            ((config as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<PyConfig>() as u64) as usize) as usize,
        )
    };
    unsafe {
        (*config)._config_init = 1;
    }
    unsafe {
        (*config).import_time = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).isolated = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).use_environment = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).dev_mode = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).install_signal_handlers = 1;
    }
    unsafe {
        (*config).use_hash_seed = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).faulthandler = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).tracemalloc = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).perf_profiling = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).remote_debug = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).module_search_paths_set = 0;
    }
    unsafe {
        (*config).parse_argv = 0;
    }
    unsafe {
        (*config).site_import = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).bytes_warning = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).warn_default_encoding = 0;
    }
    unsafe {
        (*config).inspect = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).interactive = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).optimization_level = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).parser_debug = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).write_bytecode = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).verbose = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).quiet = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).user_site_directory = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).configure_c_stdio = 0;
    }
    unsafe {
        (*config).buffered_stdio = 1i32.wrapping_neg();
    }
    unsafe {
        (*config)._install_importlib = 1;
    }
    unsafe {
        (*config).check_hash_pycs_mode = core::ptr::null_mut::<i32>();
    }
    unsafe {
        (*config).pathconfig_warnings = 1i32.wrapping_neg();
    }
    unsafe {
        (*config)._init_main = 1;
    }
    unsafe {
        (*config).use_frozen_modules = 1;
    }
    unsafe {
        (*config).safe_path = 0;
    }
    unsafe {
        (*config).int_max_str_digits = 1i32.wrapping_neg();
    }
    unsafe {
        (*config)._is_python_build = 0;
    }
    unsafe {
        (*config).code_debug_ranges = 1;
    }
    unsafe {
        (*config).cpu_count = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).thread_inherit_context = 0;
    }
    unsafe {
        (*config).context_aware_warnings = 0;
    }
}

use slate_support::calloc;
use slate_support::free;
use slate_support::getenv;
use slate_support::malloc;
use slate_support::memcpy;
use slate_support::memmove;
use slate_support::memset;
use slate_support::setlocale;
use slate_support::strchr;
use slate_support::strcmp;
use slate_support::strdup;
use slate_support::strlen;
use slate_support::wcslen;

fn config_init_defaults(config: *mut PyConfig) {
    _PyConfig_InitCompatConfig(config);
    unsafe {
        (*config).isolated = 0;
    }
    unsafe {
        (*config).use_environment = 1;
    }
    unsafe {
        (*config).site_import = 1;
    }
    unsafe {
        (*config).bytes_warning = 0;
    }
    unsafe {
        (*config).inspect = 0;
    }
    unsafe {
        (*config).interactive = 0;
    }
    unsafe {
        (*config).optimization_level = 0;
    }
    unsafe {
        (*config).parser_debug = 0;
    }
    unsafe {
        (*config).write_bytecode = 1;
    }
    unsafe {
        (*config).verbose = 0;
    }
    unsafe {
        (*config).quiet = 0;
    }
    unsafe {
        (*config).user_site_directory = 1;
    }
    unsafe {
        (*config).buffered_stdio = 1;
    }
    unsafe {
        (*config).pathconfig_warnings = 1;
    }
    unsafe {
        (*config).thread_inherit_context = 0;
    }
    unsafe {
        (*config).context_aware_warnings = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_InitPythonConfig(config: *mut PyConfig) {
    config_init_defaults(config);
    unsafe {
        (*config)._config_init = 2;
    }
    unsafe {
        (*config).configure_c_stdio = 1;
    }
    unsafe {
        (*config).parse_argv = 1;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_InitIsolatedConfig(config: *mut PyConfig) {
    config_init_defaults(config);
    unsafe {
        (*config)._config_init = 3;
    }
    unsafe {
        (*config).isolated = 1;
    }
    unsafe {
        (*config).use_environment = 0;
    }
    unsafe {
        (*config).user_site_directory = 0;
    }
    unsafe {
        (*config).dev_mode = 0;
    }
    unsafe {
        (*config).install_signal_handlers = 0;
    }
    unsafe {
        (*config).use_hash_seed = 0;
    }
    unsafe {
        (*config).tracemalloc = 0;
    }
    unsafe {
        (*config).perf_profiling = 0;
    }
    unsafe {
        (*config).int_max_str_digits = 4300;
    }
    unsafe {
        (*config).safe_path = 1;
    }
    unsafe {
        (*config).pathconfig_warnings = 0;
    }
    unsafe {
        (*config).thread_inherit_context = 0;
    }
}

/// Copy str into *config_str (duplicate the string)
#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_SetString(
    config: *mut PyConfig,
    config_str: *mut *mut i32,
    str: *const i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 = unsafe {
        _Py_PreInitializeFromConfig(config as *const PyConfig, core::ptr::null::<_PyArgv>())
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    let mut str2: *mut i32 = unsafe { core::mem::zeroed() };
    if !str.is_null() {
        str2 = unsafe { _PyMem_RawWcsdup(str) };
        if str2.is_null() {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__84!() as *const i8;
                        __t0.err_msg = c"memory allocation failed".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
    } else {
        str2 = core::ptr::null_mut::<i32>();
    }
    unsafe { PyMem_RawFree(*config_str as *mut ()) };
    unsafe {
        *config_str = str2;
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

fn config_set_bytes_string(
    config: *mut PyConfig,
    config_str: *mut *mut i32,
    str: *const i8,
    decode_err_msg: *const i8,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 = unsafe {
        _Py_PreInitializeFromConfig(config as *const PyConfig, core::ptr::null::<_PyArgv>())
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    let mut str2: *mut i32 = unsafe { core::mem::zeroed() };
    if !str.is_null() {
        let mut len: u64 = 0;
        str2 = unsafe { Py_DecodeLocale(str, core::ptr::addr_of_mut!(len)) };
        if str2.is_null() {
            if len == (2i32.wrapping_neg() as u64) {
                return unsafe {
                    *{
                        __slate_compound_0 = {
                            let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t0._type = 1u32;
                            __t0.func = _PyStatus_GET_FUNC__28!() as *const i8;
                            __t0.err_msg = decode_err_msg;
                            __t0
                        };
                        core::ptr::addr_of_mut!(__slate_compound_0)
                    }
                };
            } else {
                return unsafe {
                    *{
                        __slate_compound_1 = {
                            let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t1._type = 1u32;
                            __t1.func = _PyStatus_GET_FUNC__28!() as *const i8;
                            __t1.err_msg = c"memory allocation failed".as_ptr();
                            __t1
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    }
                };
            }
        }
    } else {
        str2 = core::ptr::null_mut::<i32>();
    }
    unsafe { PyMem_RawFree(*config_str as *mut ()) };
    unsafe {
        *config_str = str2;
    }
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}

/// Decode str using Py_DecodeLocale() and set the result into *config_str.
/// Pre-initialize Python if needed to ensure that encodings are properly
/// configured.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_SetBytesString(
    config: *mut PyConfig,
    config_str: *mut *mut i32,
    str: *const i8,
) -> __SlateRecord_initconfig_h_19 {
    return config_set_bytes_string(config, config_str, str, c"cannot decode string".as_ptr());
}

fn config_get_spec_member(
    config: *const PyConfig,
    spec: *const __SlateRecord_initconfig_c_99,
) -> *mut () {
    (config as *mut i8).wrapping_offset((unsafe { (*spec).offset }) as isize) as *mut ()
}

fn preconfig_get_spec_member(
    preconfig: *const PyPreConfig,
    spec: *const __SlateRecord_initconfig_c_99,
) -> *mut () {
    (preconfig as *mut i8).wrapping_offset((unsafe { (*spec).offset }) as isize) as *mut ()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_AsDict(config: *const PyConfig) -> *mut _object {
    let dict: *mut _object = unsafe { PyDict_New() };
    if dict.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut spec: *const __SlateRecord_initconfig_c_99 =
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 };
    while !unsafe { (*spec).name }.is_null() {
        let obj: *mut _object = config_get(config, spec, 0i32);
        if obj.is_null() {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        let res: i32 = unsafe { PyDict_SetItemString(dict, (*spec).name, obj) };
        Py_DECREF!(obj);
        if res < 0 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        let __v1: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v1.wrapping_offset(1 as isize);
    }
    dict
}

fn config_dict_invalid_value(name: *const i8) {
    unsafe { PyErr_Format(PyExc_ValueError, c"invalid config value: %s".as_ptr(), name) };
}

fn config_dict_get_int(dict: *mut _object, name: *const i8, result: &mut i32) -> i32 {
    let item: *mut _object = config_dict_get(dict, name);
    if item.is_null() {
        return 1i32.wrapping_neg();
    }
    let value: i32 = unsafe { PyLong_AsInt(item) };
    Py_DECREF!(item);
    let __v0: bool = value == 1i32.wrapping_neg() && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) != 0 {
            config_dict_invalid_type(name);
        } else {
            if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
                config_dict_invalid_value(name);
            }
        }
        return 1i32.wrapping_neg();
    }
    unsafe {
        *(result as *mut i32) = value;
    }
    0
}

fn config_dict_get_ulong(dict: *mut _object, name: *const i8, result: *mut u64) -> i32 {
    let item: *mut _object = config_dict_get(dict, name);
    if item.is_null() {
        return 1i32.wrapping_neg();
    }
    let value: u64 = unsafe { PyLong_AsUnsignedLong(item) };
    Py_DECREF!(item);
    let __v0: bool =
        value == (1i32.wrapping_neg() as u64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) != 0 {
            config_dict_invalid_type(name);
        } else {
            if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
                config_dict_invalid_value(name);
            }
        }
        return 1i32.wrapping_neg();
    }
    unsafe {
        *result = value;
    }
    0
}

fn config_dict_get_wstr(
    dict: *mut _object,
    name: *const i8,
    config: *mut PyConfig,
    result: *mut *mut i32,
) -> i32 {
    let mut wstr: *mut i32 = unsafe { core::mem::zeroed() };
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        item = config_dict_get(dict, name);
        if item.is_null() {
            return 1i32.wrapping_neg();
        } else {
            '__join_0: {
                if item == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    *__slate_slot_1 = PyConfig_SetString(config, result, core::ptr::null::<i32>());
                } else {
                    if PyUnicode_Check!(item) == 0 {
                        config_dict_invalid_type(name);
                        break '__join_0;
                    } else {
                        wstr = PyUnicode_AsWideCharString(item, core::ptr::null_mut::<i64>());
                        if wstr.is_null() {
                            break '__join_0;
                        } else {
                            *__slate_slot_1 =
                                PyConfig_SetString(config, result, wstr as *const i32);
                            PyMem_Free(wstr as *mut ());
                        }
                    }
                }
                if _PyStatus_EXCEPTION!(*__slate_slot_1) {
                    PyErr_NoMemory();
                } else {
                    Py_DECREF!(item);
                    return 0;
                }
            }
            Py_DECREF!(item);
            return 1i32.wrapping_neg();
        }
    }
}

fn config_dict_get_wstrlist(
    dict: *mut _object,
    name: *const i8,
    config: *mut PyConfig,
    result: *mut __SlateRecord_initconfig_h_36,
) -> i32 {
    let mut __slate_compound_13: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    let mut wstr: *mut i32 = unsafe { core::mem::zeroed() };
    let mut __v4: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut __v7: i64 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut wstrlist: core::mem::MaybeUninit<__SlateRecord_initconfig_h_36> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut __SlateRecord_initconfig_h_36 =
        core::ptr::addr_of_mut!(wstrlist) as *mut __SlateRecord_initconfig_h_36;
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    let mut is_list: i32 = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        list = config_dict_get(dict, name);
        if list.is_null() {
            return 1i32.wrapping_neg();
        } else {
            is_list = PyList_CheckExact!(list);
            if is_list == 0 {
                __v10 = PyTuple_CheckExact!(list) == 0;
            } else {
                __v10 = false;
            }
            if __v10 {
                Py_DECREF!(list);
                config_dict_invalid_type(name);
                return 1i32.wrapping_neg();
            } else {
                core::ptr::write(__slate_slot_9, *{
                    __slate_compound_13 = __SlateRecord_initconfig_h_36 {
                        length: 0,
                        items: core::ptr::null_mut::<*mut i32>(),
                    };
                    core::ptr::addr_of_mut!(__slate_compound_13)
                });
                if is_list != 0 {
                    __v7 = PyList_GET_SIZE!(list);
                } else {
                    __v7 = PyTuple_GET_SIZE!(list);
                }
                len = __v7;
                i = 0;
                '__join_0: {
                    '__join_9: {
                        '__join_10: {
                            '__join_6: {
                                while i < len {
                                    if is_list != 0 {
                                        __v4 = PyList_GET_ITEM!(list, i);
                                    } else {
                                        __v4 = PyTuple_GET_ITEM!(list, i);
                                    }
                                    item = __v4;
                                    if item == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                                        break '__join_9;
                                    } else {
                                        if PyUnicode_Check!(item) == 0 {
                                            break '__join_10;
                                        } else {
                                            wstr = PyUnicode_AsWideCharString(
                                                item,
                                                core::ptr::null_mut::<i64>(),
                                            );
                                            if wstr.is_null() {
                                                break '__join_0;
                                            } else {
                                                core::ptr::write(
                                                    __slate_slot_2,
                                                    PyWideStringList_Append(
                                                        core::ptr::addr_of_mut!(*__slate_slot_9),
                                                        wstr as *const i32,
                                                    ),
                                                );
                                                PyMem_Free(wstr as *mut ());
                                                if _PyStatus_EXCEPTION!(*__slate_slot_2) {
                                                    break '__join_6;
                                                } else {
                                                    i = i.wrapping_add(1);
                                                }
                                            }
                                        }
                                    }
                                }
                                if _PyWideStringList_Copy(
                                    result,
                                    core::ptr::addr_of_mut!(*__slate_slot_9)
                                        as *const __SlateRecord_initconfig_h_36,
                                ) < 0
                                {
                                    PyErr_NoMemory();
                                    break '__join_0;
                                } else {
                                    _PyWideStringList_Clear(core::ptr::addr_of_mut!(
                                        *__slate_slot_9
                                    ));
                                    Py_DECREF!(list);
                                    return 0;
                                }
                            }
                            PyErr_NoMemory();
                            break '__join_0;
                        }
                        config_dict_invalid_type(name);
                        break '__join_0;
                    }
                    config_dict_invalid_value(name);
                }
                _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_9));
                Py_DECREF!(list);
                return 1i32.wrapping_neg();
            }
        }
    }
}

fn config_dict_get_xoptions(
    dict: *mut _object,
    name: *const i8,
    config: *mut PyConfig,
    result: *mut __SlateRecord_initconfig_h_36,
) -> i32 {
    let mut __slate_compound_8: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    let mut wstr: *mut i32 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut wstrlist: core::mem::MaybeUninit<__SlateRecord_initconfig_h_36> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut __SlateRecord_initconfig_h_36 =
        core::ptr::addr_of_mut!(wstrlist) as *mut __SlateRecord_initconfig_h_36;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut xoptions: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        xoptions = config_dict_get(dict, name);
        if xoptions.is_null() {
            return 1i32.wrapping_neg();
        } else {
            if PyDict_CheckExact!(xoptions) == 0 {
                Py_DECREF!(xoptions);
                config_dict_invalid_type(name);
                return 1i32.wrapping_neg();
            } else {
                pos = 0;
                core::ptr::write(__slate_slot_3, *{
                    __slate_compound_8 = __SlateRecord_initconfig_h_36 {
                        length: 0,
                        items: core::ptr::null_mut::<*mut i32>(),
                    };
                    core::ptr::addr_of_mut!(__slate_compound_8)
                });
                '__join_0: {
                    '__join_5: {
                        while PyDict_Next(
                            xoptions,
                            core::ptr::addr_of_mut!(pos),
                            core::ptr::addr_of_mut!(key),
                            core::ptr::addr_of_mut!(value),
                        ) != 0
                        {
                            if value != _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct))
                            {
                                item = PyUnicode_FromFormat(c"%S=%S".as_ptr(), key, value);
                                if item.is_null() {
                                    break '__join_0;
                                }
                            } else {
                                item = Py_NewRef!(key);
                            }
                            wstr = PyUnicode_AsWideCharString(item, core::ptr::null_mut::<i64>());
                            Py_DECREF!(item);
                            if wstr.is_null() {
                                break '__join_0;
                            } else {
                                core::ptr::write(
                                    __slate_slot_0,
                                    PyWideStringList_Append(
                                        core::ptr::addr_of_mut!(*__slate_slot_3),
                                        wstr as *const i32,
                                    ),
                                );
                                PyMem_Free(wstr as *mut ());
                                if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                                    break '__join_5;
                                }
                            }
                        }
                        if _PyWideStringList_Copy(
                            result,
                            core::ptr::addr_of_mut!(*__slate_slot_3)
                                as *const __SlateRecord_initconfig_h_36,
                        ) < 0
                        {
                            PyErr_NoMemory();
                            break '__join_0;
                        } else {
                            _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_3));
                            Py_DECREF!(xoptions);
                            return 0;
                        }
                    }
                    PyErr_NoMemory();
                }
                _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_3));
                Py_DECREF!(xoptions);
                return 1i32.wrapping_neg();
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_FromDict(config: *mut PyConfig, dict: *mut _object) -> i32 {
    if PyDict_Check!(dict) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"dict expected".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    let mut spec: *const __SlateRecord_initconfig_c_99 =
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 };
    while !unsafe { (*spec).name }.is_null() {
        let member: *mut i8 =
            (config as *mut i8).wrapping_offset((unsafe { (*spec).offset }) as isize);
        match (unsafe { (*spec).r#type }) as u32 {
            0 | 1 | 3 => {
                let mut value: i32 = 0;
                if config_dict_get_int(dict, unsafe { (*spec).name }, unsafe {
                    &mut (*core::ptr::addr_of_mut!(value))
                }) < 0
                {
                    return 1i32.wrapping_neg();
                }
                if ((unsafe { (*spec).r#type }) as u32) == 3
                    || ((unsafe { (*spec).r#type }) as u32) == 1
                {
                    if value < 0 {
                        config_dict_invalid_value(unsafe { (*spec).name });
                        return 1i32.wrapping_neg();
                    }
                }
                unsafe {
                    *(member as *mut i32) = value;
                }
            }
            2 => {
                if config_dict_get_ulong(dict, unsafe { (*spec).name }, member as *mut u64) < 0 {
                    return 1i32.wrapping_neg();
                }
            }
            10 => {
                let wstr: *mut *mut i32 = member as *mut *mut i32;
                if config_dict_get_wstr(dict, unsafe { (*spec).name }, config, wstr) < 0 {
                    return 1i32.wrapping_neg();
                }
                if unsafe { *wstr }.is_null() {
                    config_dict_invalid_value(unsafe { (*spec).name });
                    return 1i32.wrapping_neg();
                }
            }
            11 => {
                let wstr: *mut *mut i32 = member as *mut *mut i32;
                if config_dict_get_wstr(dict, unsafe { (*spec).name }, config, wstr) < 0 {
                    return 1i32.wrapping_neg();
                }
            }
            12 => {
                if (unsafe {
                    strcmp(
                        (*spec).name as *const core::ffi::c_char,
                        c"xoptions".as_ptr() as *const core::ffi::c_char,
                    ) as i32
                }) == 0
                {
                    if config_dict_get_xoptions(
                        dict,
                        unsafe { (*spec).name },
                        config,
                        member as *mut __SlateRecord_initconfig_h_36,
                    ) < 0
                    {
                        return 1i32.wrapping_neg();
                    }
                } else {
                    if config_dict_get_wstrlist(
                        dict,
                        unsafe { (*spec).name },
                        config,
                        member as *mut __SlateRecord_initconfig_h_36,
                    ) < 0
                    {
                        return 1i32.wrapping_neg();
                    }
                }
            }
            _ => {
                Py_UNREACHABLE!();
            }
        }
        let __v2: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v2.wrapping_offset(1 as isize);
    }
    if !((unsafe { (*config)._config_init }) == 1
        || (unsafe { (*config)._config_init }) == 2
        || (unsafe { (*config)._config_init }) == 3)
    {
        config_dict_invalid_value(c"_config_init".as_ptr());
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*config).hash_seed }) > 4294967295 {
        config_dict_invalid_value(c"hash_seed".as_ptr());
        return 1i32.wrapping_neg();
    }
    0
}

fn config_get_env(config: *const PyConfig, name: *const i8) -> *const i8 {
    unsafe { _Py_GetEnv((*config).use_environment, name) }
}

/// Get a copy of the environment variable as wchar_t*.
/// Return 0 on success, but *dest can be NULL.
/// Return -1 on memory allocation failure. Return -2 on decoding error.
fn config_get_env_dup(
    config: *mut PyConfig,
    dest: *mut *mut i32,
    wname: *mut i32,
    name: *mut i8,
    decode_err_msg: *const i8,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if (unsafe { (*config).use_environment }) == 0 {
        unsafe {
            *dest = core::ptr::null_mut::<i32>();
        }
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
    let var: *const i8 =
        (unsafe { getenv((name as *const i8) as *const core::ffi::c_char) as *mut i8 })
            as *const i8;
    if var.is_null() || ((unsafe { *var.wrapping_offset(0 as isize) }) as i32) == 0 {
        unsafe {
            *dest = core::ptr::null_mut::<i32>();
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
    return config_set_bytes_string(config, dest, var, decode_err_msg);
}

fn config_get_global_vars(config: *mut PyConfig) {
    if (unsafe { (*config)._config_init }) != 1 {
        // Python and Isolated configuration ignore global variables
        return;
    }
    COPY_FLAG!(isolated, unsafe { Py_IsolatedFlag }, config);
    COPY_NOT_FLAG!(use_environment, unsafe { Py_IgnoreEnvironmentFlag }, config);
    COPY_FLAG!(bytes_warning, unsafe { Py_BytesWarningFlag }, config);
    COPY_FLAG!(inspect, unsafe { Py_InspectFlag }, config);
    COPY_FLAG!(interactive, unsafe { Py_InteractiveFlag }, config);
    COPY_FLAG!(optimization_level, unsafe { Py_OptimizeFlag }, config);
    COPY_FLAG!(parser_debug, unsafe { Py_DebugFlag }, config);
    COPY_FLAG!(verbose, unsafe { Py_VerboseFlag }, config);
    COPY_FLAG!(quiet, unsafe { Py_QuietFlag }, config);
    COPY_NOT_FLAG!(pathconfig_warnings, unsafe { Py_FrozenFlag }, config);
    COPY_NOT_FLAG!(buffered_stdio, unsafe { Py_UnbufferedStdioFlag }, config);
    COPY_NOT_FLAG!(site_import, unsafe { Py_NoSiteFlag }, config);
    COPY_NOT_FLAG!(write_bytecode, unsafe { Py_DontWriteBytecodeFlag }, config);
    COPY_NOT_FLAG!(
        user_site_directory,
        unsafe { Py_NoUserSiteDirectory },
        config
    );
}

/// Set Py_xxx global configuration variables from 'config' configuration.
fn config_set_global_vars(config: *const PyConfig) {
    COPY_FLAG__2!(isolated, Py_IsolatedFlag, config);
    COPY_NOT_FLAG__2!(use_environment, Py_IgnoreEnvironmentFlag, config);
    COPY_FLAG__2!(bytes_warning, Py_BytesWarningFlag, config);
    COPY_FLAG__2!(inspect, Py_InspectFlag, config);
    COPY_FLAG__2!(interactive, Py_InteractiveFlag, config);
    COPY_FLAG__2!(optimization_level, Py_OptimizeFlag, config);
    COPY_FLAG__2!(parser_debug, Py_DebugFlag, config);
    COPY_FLAG__2!(verbose, Py_VerboseFlag, config);
    COPY_FLAG__2!(quiet, Py_QuietFlag, config);
    COPY_NOT_FLAG__2!(pathconfig_warnings, Py_FrozenFlag, config);
    COPY_NOT_FLAG__2!(buffered_stdio, Py_UnbufferedStdioFlag, config);
    COPY_NOT_FLAG__2!(site_import, Py_NoSiteFlag, config);
    COPY_NOT_FLAG__2!(write_bytecode, Py_DontWriteBytecodeFlag, config);
    COPY_NOT_FLAG__2!(user_site_directory, Py_NoUserSiteDirectory, config);
    // Random or non-zero hash seed
    unsafe {
        Py_HashRandomizationFlag =
            ((*config).use_hash_seed == 0 || (*config).hash_seed != 0) as i32;
    }
}

fn config_get_xoption(config: *const PyConfig, name: *mut i32) -> *const i32 {
    unsafe { _Py_get_xoption(core::ptr::addr_of!((*config).xoptions), name as *const i32) }
}

fn config_get_xoption_value(config: *const PyConfig, name: *mut i32) -> *const i32 {
    let xoption: *const i32 = config_get_xoption(config, name);
    if xoption.is_null() {
        return core::ptr::null::<i32>();
    }
    let sep: *const i32 = (unsafe { wcschr(xoption, 61i32) }) as *const i32;
    if !sep.is_null() {
        sep.wrapping_offset(1 as isize)
    } else {
        (unsafe { core::ptr::addr_of_mut!(__slate_str3144) as *mut i32 }) as *const i32
    }
}

static mut __slate_str3144: [i32; 1] = [0];

fn config_init_hash_seed(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let seed_text: *const i8 =
        config_get_env(config as *const PyConfig, c"PYTHONHASHSEED".as_ptr());
    // Convert a text seed to a numeric one
    if !seed_text.is_null()
        && (unsafe {
            strcmp(
                seed_text as *const core::ffi::c_char,
                c"random".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) != 0
    {
        let mut endptr: *const i8 = seed_text;
        let mut seed: u64 = 0;
        unsafe {
            *slate_support::errno_location() = 0;
        }
        seed = unsafe {
            strtoul(
                seed_text,
                core::ptr::addr_of_mut!(endptr) as *mut *mut i8,
                10i32,
            )
        };
        if ((unsafe { *endptr }) as i32) != 0
            || seed > 4294967295
            || (unsafe { *slate_support::errno_location() }) == (slate_support::ERANGE as i32)
                && seed == 18446744073709551615u64
        {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__59!() as *const i8;
                        __t0.err_msg = c"PYTHONHASHSEED must be \"random\" or an integer in range [0; 4294967295]".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        // Use a specific hash
        unsafe {
            (*config).use_hash_seed = 1;
        }
        unsafe {
            (*config).hash_seed = seed;
        }
    } else {
        // Use a random hash
        unsafe {
            (*config).use_hash_seed = 0;
        }
        unsafe {
            (*config).hash_seed = 0;
        }
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

fn config_wstr_to_int(wstr: *const i32, result: &mut i32) -> i32 {
    let mut endptr: *const i32 = wstr;
    unsafe {
        *slate_support::errno_location() = 0;
    }
    let value: i64 = unsafe {
        wcstol(
            wstr,
            core::ptr::addr_of_mut!(endptr) as *mut *mut i32,
            10i32,
        )
    };
    if (unsafe { *endptr }) != 0
        || (unsafe { *slate_support::errno_location() }) == (slate_support::ERANGE as i32)
    {
        return 1i32.wrapping_neg();
    }
    if value < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64) || value > 2147483647 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *(result as *mut i32) = value as i32;
    }
    0
}

fn config_read_gil(
    config: *mut PyConfig,
    len: u64,
    first_char: i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if len == 1 && first_char == 48 {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__17!() as *const i8;
                    __t0.err_msg = c"Disabling the GIL is not supported by this build".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    } else {
        if len == 1 && first_char == 49 {
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
        } else {
            return unsafe {
                *{
                    __slate_compound_2 = {
                        let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t2._type = 1u32;
                        __t2.func = _PyStatus_GET_FUNC__17!() as *const i8;
                        __t2.err_msg = c"PYTHON_GIL / -X gil must be \"0\" or \"1\"".as_ptr();
                        __t2
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
    }
}

fn config_read_env_vars(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let use_env: i32 = unsafe { (*config).use_environment };
    // Get environment variables
    unsafe {
        _Py_get_env_flag(
            use_env,
            core::ptr::addr_of_mut!((*config).parser_debug),
            c"PYTHONDEBUG".as_ptr(),
        )
    };
    unsafe {
        _Py_get_env_flag(
            use_env,
            core::ptr::addr_of_mut!((*config).verbose),
            c"PYTHONVERBOSE".as_ptr(),
        )
    };
    unsafe {
        _Py_get_env_flag(
            use_env,
            core::ptr::addr_of_mut!((*config).optimization_level),
            c"PYTHONOPTIMIZE".as_ptr(),
        )
    };
    let __v0: bool = (unsafe { (*config).inspect }) == 0
        && !unsafe { _Py_GetEnv(use_env, c"PYTHONINSPECT".as_ptr()) }.is_null();
    if __v0 {
        unsafe {
            (*config).inspect = 1;
        }
    }
    let mut dont_write_bytecode: i32 = 0;
    unsafe {
        _Py_get_env_flag(
            use_env,
            core::ptr::addr_of_mut!(dont_write_bytecode),
            c"PYTHONDONTWRITEBYTECODE".as_ptr(),
        )
    };
    if dont_write_bytecode != 0 {
        unsafe {
            (*config).write_bytecode = 0;
        }
    }
    let mut no_user_site_directory: i32 = 0;
    unsafe {
        _Py_get_env_flag(
            use_env,
            core::ptr::addr_of_mut!(no_user_site_directory),
            c"PYTHONNOUSERSITE".as_ptr(),
        )
    };
    if no_user_site_directory != 0 {
        unsafe {
            (*config).user_site_directory = 0;
        }
    }
    let mut unbuffered_stdio: i32 = 0;
    unsafe {
        _Py_get_env_flag(
            use_env,
            core::ptr::addr_of_mut!(unbuffered_stdio),
            c"PYTHONUNBUFFERED".as_ptr(),
        )
    };
    if unbuffered_stdio != 0 {
        unsafe {
            (*config).buffered_stdio = 0;
        }
    }
    if !config_get_env(config as *const PyConfig, c"PYTHONDUMPREFS".as_ptr()).is_null() {
        unsafe {
            (*config).dump_refs = 1;
        }
    }
    if !config_get_env(config as *const PyConfig, c"PYTHONMALLOCSTATS".as_ptr()).is_null() {
        unsafe {
            (*config).malloc_stats = 1;
        }
    }
    if unsafe { (*config).dump_refs_file }.is_null() {
        status = config_get_env_dup(
            config,
            unsafe { core::ptr::addr_of_mut!((*config).dump_refs_file) },
            unsafe { core::ptr::addr_of_mut!(__slate_str3168) as *mut i32 },
            c"PYTHONDUMPREFSFILE".as_ptr() as *mut i8,
            c"cannot decode PYTHONDUMPREFSFILE".as_ptr(),
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if unsafe { (*config).pythonpath_env }.is_null() {
        status = config_get_env_dup(
            config,
            unsafe { core::ptr::addr_of_mut!((*config).pythonpath_env) },
            unsafe { core::ptr::addr_of_mut!(__slate_str3171) as *mut i32 },
            c"PYTHONPATH".as_ptr() as *mut i8,
            c"cannot decode PYTHONPATH".as_ptr(),
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if unsafe { (*config).platlibdir }.is_null() {
        status = config_get_env_dup(
            config,
            unsafe { core::ptr::addr_of_mut!((*config).platlibdir) },
            unsafe { core::ptr::addr_of_mut!(__slate_str3174) as *mut i32 },
            c"PYTHONPLATLIBDIR".as_ptr() as *mut i8,
            c"cannot decode PYTHONPLATLIBDIR".as_ptr(),
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).use_hash_seed }) < 0 {
        status = config_init_hash_seed(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if !config_get_env(config as *const PyConfig, c"PYTHONSAFEPATH".as_ptr()).is_null() {
        unsafe {
            (*config).safe_path = 1;
        }
    }
    let gil: *const i8 = config_get_env(config as *const PyConfig, c"PYTHON_GIL".as_ptr());
    if !gil.is_null() {
        let len: u64 = unsafe { strlen(gil as *const core::ffi::c_char) as u64 };
        status = config_read_gil(
            config,
            len,
            (unsafe { *gil.wrapping_offset(0 as isize) }) as i32,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    return unsafe {
        *{
            __slate_compound_1 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        }
    };
}

static mut __slate_str3168: [i32; 19] = [
    80, 89, 84, 72, 79, 78, 68, 85, 77, 80, 82, 69, 70, 83, 70, 73, 76, 69, 0,
];

static mut __slate_str3171: [i32; 11] = [80, 89, 84, 72, 79, 78, 80, 65, 84, 72, 0];

static mut __slate_str3174: [i32; 17] = [
    80, 89, 84, 72, 79, 78, 80, 76, 65, 84, 76, 73, 66, 68, 73, 82, 0,
];

fn config_init_cpu_count(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_5: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_6: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut sep: *const i32 = unsafe { core::mem::zeroed() };
    let mut cpu_count: i32 = unsafe { core::mem::zeroed() };
    let mut xoption: *const i32 = unsafe { core::mem::zeroed() };
    let mut cpu_count_1: i32 = unsafe { core::mem::zeroed() };
    let mut env: *const i8 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            env = config_get_env(config as *const PyConfig, c"PYTHON_CPU_COUNT".as_ptr());
            if !env.is_null() {
                cpu_count_1 = 1i32.wrapping_neg();
                if (strcmp(
                    env as *const core::ffi::c_char,
                    c"default".as_ptr() as *const core::ffi::c_char,
                ) as i32)
                    == 0
                {
                    cpu_count_1 = 1i32.wrapping_neg();
                } else {
                    if _Py_str_to_int(env, core::ptr::addr_of_mut!(cpu_count_1)) < 0
                        || cpu_count_1 < 1
                    {
                        break '__join_0;
                    }
                }
                (*config).cpu_count = cpu_count_1;
            }
            xoption = config_get_xoption(
                config as *const PyConfig,
                core::ptr::addr_of_mut!(__slate_str3182) as *mut i32,
            );
            if !xoption.is_null() {
                cpu_count = 1i32.wrapping_neg();
                sep = wcschr(xoption, 61i32) as *const i32;
                if !sep.is_null() {
                    if wcscmp(
                        sep.wrapping_offset(1 as isize),
                        (core::ptr::addr_of_mut!(__slate_str3183) as *mut i32) as *const i32,
                    ) == 0
                    {
                        cpu_count = 1i32.wrapping_neg();
                    } else {
                        if config_wstr_to_int(
                            sep.wrapping_offset(1 as isize),
                            &mut (*core::ptr::addr_of_mut!(cpu_count)),
                        ) < 0
                            || cpu_count < 1
                        {
                            break '__join_0;
                        }
                    }
                    (*config).cpu_count = cpu_count;
                } else {
                    break '__join_0;
                }
            }
            return *{
                __slate_compound_6 = {
                    let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t1._type = 0u32;
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_6)
            };
        }
        return *{
            __slate_compound_5 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__65!() as *const i8;
                __t0.err_msg = c"-X cpu_count=n option: n is missing or an invalid number, n must be greater than 0".as_ptr();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_5)
        };
    }
}

static mut __slate_str3182: [i32; 10] = [99, 112, 117, 95, 99, 111, 117, 110, 116, 0];

static mut __slate_str3183: [i32; 8] = [100, 101, 102, 97, 117, 108, 116, 0];

fn config_init_thread_inherit_context(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let env: *const i8 = config_get_env(
        config as *const PyConfig,
        c"PYTHON_THREAD_INHERIT_CONTEXT".as_ptr(),
    );
    if !env.is_null() {
        let mut enabled: i32 = 0;
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(enabled)) }) < 0
            || enabled < 0
            || enabled > 1
        {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__22!() as *const i8;
                        __t0.err_msg =
                            c"PYTHON_THREAD_INHERIT_CONTEXT=N: N is missing or invalid".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        unsafe {
            (*config).thread_inherit_context = enabled;
        }
    }
    let xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3192) as *mut i32
    });
    if !xoption.is_null() {
        let mut enabled: i32 = 0;
        let sep: *const i32 = (unsafe { wcschr(xoption, 61i32) }) as *const i32;
        let __v1: bool = sep.is_null()
            || config_wstr_to_int(sep.wrapping_offset(1 as isize), unsafe {
                &mut (*core::ptr::addr_of_mut!(enabled))
            }) < 0;
        if __v1 || enabled < 0 || enabled > 1 {
            return unsafe {
                *{
                    __slate_compound_2 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__22!() as *const i8;
                        __t1.err_msg =
                            c"-X thread_inherit_context=n: n is missing or invalid".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
        unsafe {
            (*config).thread_inherit_context = enabled;
        }
    }
    return unsafe {
        *{
            __slate_compound_3 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_3)
        }
    };
}

static mut __slate_str3192: [i32; 23] = [
    116, 104, 114, 101, 97, 100, 95, 105, 110, 104, 101, 114, 105, 116, 95, 99, 111, 110, 116, 101,
    120, 116, 0,
];

fn config_init_context_aware_warnings(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let env: *const i8 = config_get_env(
        config as *const PyConfig,
        c"PYTHON_CONTEXT_AWARE_WARNINGS".as_ptr(),
    );
    if !env.is_null() {
        let mut enabled: i32 = 0;
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(enabled)) }) < 0
            || enabled < 0
            || enabled > 1
        {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__32!() as *const i8;
                        __t0.err_msg =
                            c"PYTHON_CONTEXT_AWARE_WARNINGS=N: N is missing or invalid".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        unsafe {
            (*config).context_aware_warnings = enabled;
        }
    }
    let xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3201) as *mut i32
    });
    if !xoption.is_null() {
        let mut enabled: i32 = 0;
        let sep: *const i32 = (unsafe { wcschr(xoption, 61i32) }) as *const i32;
        let __v1: bool = sep.is_null()
            || config_wstr_to_int(sep.wrapping_offset(1 as isize), unsafe {
                &mut (*core::ptr::addr_of_mut!(enabled))
            }) < 0;
        if __v1 || enabled < 0 || enabled > 1 {
            return unsafe {
                *{
                    __slate_compound_2 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__32!() as *const i8;
                        __t1.err_msg =
                            c"-X context_aware_warnings=n: n is missing or invalid".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
        unsafe {
            (*config).context_aware_warnings = enabled;
        }
    }
    return unsafe {
        *{
            __slate_compound_3 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_3)
        }
    };
}

static mut __slate_str3201: [i32; 23] = [
    99, 111, 110, 116, 101, 120, 116, 95, 97, 119, 97, 114, 101, 95, 119, 97, 114, 110, 105, 110,
    103, 115, 0,
];

fn config_init_tlbc(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
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

fn config_init_perf_profiling(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut active: i32 = 0;
    let mut env: *const i8 =
        config_get_env(config as *const PyConfig, c"PYTHONPERFSUPPORT".as_ptr());
    if !env.is_null() {
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(active)) }) != 0 {
            active = 0;
        }
        if active != 0 {
            unsafe {
                (*config).perf_profiling = 1;
            }
        }
    }
    let mut xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3208) as *mut i32
    });
    if !xoption.is_null() {
        unsafe {
            (*config).perf_profiling = 1;
        }
    }
    env = config_get_env(
        config as *const PyConfig,
        c"PYTHON_PERF_JIT_SUPPORT".as_ptr(),
    );
    if !env.is_null() {
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(active)) }) != 0 {
            active = 0;
        }
        if active != 0 {
            unsafe {
                (*config).perf_profiling = 2;
            }
        }
    }
    xoption = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3210) as *mut i32
    });
    if !xoption.is_null() {
        unsafe {
            (*config).perf_profiling = 2;
        }
    }
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

static mut __slate_str3208: [i32; 5] = [112, 101, 114, 102, 0];

static mut __slate_str3210: [i32; 9] = [112, 101, 114, 102, 95, 106, 105, 116, 0];

fn config_init_remote_debug(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut active: i32 = 1;
    let env: *const i8 = Py_GETENV(c"PYTHON_DISABLE_REMOTE_DEBUG".as_ptr()) as *const i8;
    if !env.is_null() {
        active = 0;
    }
    let xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3213) as *mut i32
    });
    if !xoption.is_null() {
        active = 0;
    }
    unsafe {
        (*config).remote_debug = active;
    }
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

static mut __slate_str3213: [i32; 21] = [
    100, 105, 115, 97, 98, 108, 101, 45, 114, 101, 109, 111, 116, 101, 45, 100, 101, 98, 117, 103,
    0,
];

fn config_init_tracemalloc(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut nframe: i32 = 0;
    let mut valid: i32 = 0;
    let env: *const i8 = config_get_env(config as *const PyConfig, c"PYTHONTRACEMALLOC".as_ptr());
    if !env.is_null() {
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(nframe)) }) == 0 {
            valid = (nframe >= 0) as i32;
        } else {
            valid = 0;
        }
        if valid == 0 {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__29!() as *const i8;
                        __t0.err_msg = c"PYTHONTRACEMALLOC: invalid number of frames".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        unsafe {
            (*config).tracemalloc = nframe;
        }
    }
    let xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3219) as *mut i32
    });
    if !xoption.is_null() {
        let sep: *const i32 = (unsafe { wcschr(xoption, 61i32) }) as *const i32;
        if !sep.is_null() {
            if config_wstr_to_int(sep.wrapping_offset(1 as isize), unsafe {
                &mut (*core::ptr::addr_of_mut!(nframe))
            }) == 0
            {
                valid = (nframe >= 0) as i32;
            } else {
                valid = 0;
            }
            if valid == 0 {
                return unsafe {
                    *{
                        __slate_compound_1 = {
                            let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t1._type = 1u32;
                            __t1.func = _PyStatus_GET_FUNC__29!() as *const i8;
                            __t1.err_msg =
                                c"-X tracemalloc=NFRAME: invalid number of frames".as_ptr();
                            __t1
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    }
                };
            }
        } else {
            // -X tracemalloc behaves as -X tracemalloc=1
            nframe = 1;
        }
        unsafe {
            (*config).tracemalloc = nframe;
        }
    }
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}

static mut __slate_str3219: [i32; 12] = [116, 114, 97, 99, 101, 109, 97, 108, 108, 111, 99, 0];

fn config_init_int_max_str_digits(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut maxdigits: i32 = 0;
    let env: *const i8 =
        config_get_env(config as *const PyConfig, c"PYTHONINTMAXSTRDIGITS".as_ptr());
    if !env.is_null() {
        let mut valid: bool = false;
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(maxdigits)) }) == 0 {
            valid = maxdigits == 0 || maxdigits >= 640;
        }
        if !valid {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__18!() as *const i8;
                        __t0.err_msg = c"PYTHONINTMAXSTRDIGITS: invalid limit; must be >= 640 or 0 for unlimited.".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        unsafe {
            (*config).int_max_str_digits = maxdigits;
        }
    }
    let xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3228) as *mut i32
    });
    if !xoption.is_null() {
        let sep: *const i32 = (unsafe { wcschr(xoption, 61i32) }) as *const i32;
        let mut valid: bool = false;
        if !sep.is_null() {
            if config_wstr_to_int(sep.wrapping_offset(1 as isize), unsafe {
                &mut (*core::ptr::addr_of_mut!(maxdigits))
            }) == 0
            {
                valid = maxdigits == 0 || maxdigits >= 640;
            }
        }
        if !valid {
            return unsafe {
                *{
                    __slate_compound_1 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__18!() as *const i8;
                        __t1.err_msg = c"-X int_max_str_digits: invalid limit; must be >= 640 or 0 for unlimited.".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            };
        }
        unsafe {
            (*config).int_max_str_digits = maxdigits;
        }
    }
    if (unsafe { (*config).int_max_str_digits }) < 0 {
        unsafe {
            (*config).int_max_str_digits = 4300;
        }
    }
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}

static mut __slate_str3228: [i32; 19] = [
    105, 110, 116, 95, 109, 97, 120, 95, 115, 116, 114, 95, 100, 105, 103, 105, 116, 115, 0,
];

fn config_init_pycache_prefix(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let xoption: *const i32 = config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3233) as *mut i32
    });
    if !xoption.is_null() {
        let sep: *const i32 = (unsafe { wcschr(xoption, 61i32) }) as *const i32;
        if !sep.is_null() && (unsafe { wcslen(sep as *const i32) as u64 }) > 1 {
            unsafe {
                (*config).pycache_prefix = _PyMem_RawWcsdup(sep.wrapping_offset(1 as isize));
            }
            if unsafe { (*config).pycache_prefix }.is_null() {
                return unsafe {
                    *{
                        __slate_compound_0 = {
                            let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t0._type = 1u32;
                            __t0.func = _PyStatus_GET_FUNC__76!() as *const i8;
                            __t0.err_msg = c"memory allocation failed".as_ptr();
                            __t0
                        };
                        core::ptr::addr_of_mut!(__slate_compound_0)
                    }
                };
            }
        } else {
            // PYTHONPYCACHEPREFIX env var ignored
            //
            // if "-X pycache_prefix=" option is used
            unsafe {
                (*config).pycache_prefix = core::ptr::null_mut::<i32>();
            }
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
    return config_get_env_dup(
        config,
        unsafe { core::ptr::addr_of_mut!((*config).pycache_prefix) },
        unsafe { core::ptr::addr_of_mut!(__slate_str3238) as *mut i32 },
        c"PYTHONPYCACHEPREFIX".as_ptr() as *mut i8,
        c"cannot decode PYTHONPYCACHEPREFIX".as_ptr(),
    );
}

static mut __slate_str3233: [i32; 15] = [
    112, 121, 99, 97, 99, 104, 101, 95, 112, 114, 101, 102, 105, 120, 0,
];

static mut __slate_str3238: [i32; 20] = [
    80, 89, 84, 72, 79, 78, 80, 89, 67, 65, 67, 72, 69, 80, 82, 69, 70, 73, 88, 0,
];

fn config_init_import_time(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut importtime: i32 = 0;
    let env: *const i8 = config_get_env(
        config as *const PyConfig,
        c"PYTHONPROFILEIMPORTTIME".as_ptr(),
    );
    if !env.is_null() {
        if (unsafe { _Py_str_to_int(env, core::ptr::addr_of_mut!(importtime)) }) != 0 {
            importtime = 1;
        }
        if importtime < 0 || importtime > 2 {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__35!() as *const i8;
                        __t0.err_msg = c"PYTHONPROFILEIMPORTTIME: numeric values other than 1 and 2 are reserved for future use.".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
    }
    let x_value: *const i32 = config_get_xoption_value(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3245) as *mut i32
    });
    if !x_value.is_null() {
        let __v1: bool = (unsafe { *x_value }) == 0
            || config_wstr_to_int(x_value, unsafe {
                &mut (*core::ptr::addr_of_mut!(importtime))
            }) != 0;
        if __v1 {
            importtime = 1;
        }
        if importtime < 0 || importtime > 2 {
            return unsafe {
                *{
                    __slate_compound_2 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__35!() as *const i8;
                        __t1.err_msg = c"-X importtime: values other than 1 and 2 are reserved for future use.".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
    }
    unsafe {
        (*config).import_time = importtime;
    }
    return unsafe {
        *{
            __slate_compound_3 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_3)
        }
    };
}

static mut __slate_str3245: [i32; 11] = [105, 109, 112, 111, 114, 116, 116, 105, 109, 101, 0];

fn config_read_complex_options(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // More complex options configured by env var and -X option
    if (unsafe { (*config).faulthandler }) < 0 {
        let __v0: bool = !config_get_env(config as *const PyConfig, c"PYTHONFAULTHANDLER".as_ptr())
            .is_null()
            || !config_get_xoption(config as *const PyConfig, unsafe {
                core::ptr::addr_of_mut!(__slate_str3251) as *mut i32
            })
            .is_null();
        if __v0 {
            unsafe {
                (*config).faulthandler = 1;
            }
        }
    }
    let __v1: bool = !config_get_env(config as *const PyConfig, c"PYTHONNODEBUGRANGES".as_ptr())
        .is_null()
        || !config_get_xoption(config as *const PyConfig, unsafe {
            core::ptr::addr_of_mut!(__slate_str3253) as *mut i32
        })
        .is_null();
    if __v1 {
        unsafe {
            (*config).code_debug_ranges = 0;
        }
    }
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if (unsafe { (*config).import_time }) < 0 {
        status = config_init_import_time(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).tracemalloc }) < 0 {
        status = config_init_tracemalloc(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).perf_profiling }) < 0 {
        status = config_init_perf_profiling(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).remote_debug }) < 0 {
        status = config_init_remote_debug(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).int_max_str_digits }) < 0 {
        status = config_init_int_max_str_digits(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).cpu_count }) < 0 {
        status = config_init_cpu_count(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if unsafe { (*config).pycache_prefix }.is_null() {
        status = config_init_pycache_prefix(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    status = config_init_thread_inherit_context(config);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    status = config_init_context_aware_warnings(config);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    status = config_init_tlbc(config);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}

static mut __slate_str3251: [i32; 13] =
    [102, 97, 117, 108, 116, 104, 97, 110, 100, 108, 101, 114, 0];

static mut __slate_str3253: [i32; 16] = [
    110, 111, 95, 100, 101, 98, 117, 103, 95, 114, 97, 110, 103, 101, 115, 0,
];

fn config_get_stdio_errors(preconfig: *const PyPreConfig) -> *const i32 {
    if (unsafe { (*preconfig).utf8_mode }) != 0 {
        // UTF-8 Mode uses UTF-8/surrogateescape
        return (unsafe { core::ptr::addr_of_mut!(__slate_str3255) as *mut i32 }) as *const i32;
    }
    let loc: *const i8 = (unsafe {
        setlocale(
            0i32 as i32,
            core::ptr::null::<i8>() as *const core::ffi::c_char,
        ) as *mut i8
    }) as *const i8;
    if !loc.is_null() {
        // surrogateescape is the default in the legacy C and POSIX locales
        if (unsafe {
            strcmp(
                loc as *const core::ffi::c_char,
                c"C".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
            || (unsafe {
                strcmp(
                    loc as *const core::ffi::c_char,
                    c"POSIX".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
        {
            return (unsafe { core::ptr::addr_of_mut!(__slate_str3258) as *mut i32 }) as *const i32;
        }
        // surrogateescape is the default in locale coercion target locales
        if (unsafe { _Py_IsLocaleCoercionTarget(loc) }) != 0 {
            return (unsafe { core::ptr::addr_of_mut!(__slate_str3259) as *mut i32 }) as *const i32;
        }
    }
    (unsafe { core::ptr::addr_of_mut!(__slate_str3260) as *mut i32 }) as *const i32
}

static mut __slate_str3255: [i32; 16] = [
    115, 117, 114, 114, 111, 103, 97, 116, 101, 101, 115, 99, 97, 112, 101, 0,
];

static mut __slate_str3258: [i32; 16] = [
    115, 117, 114, 114, 111, 103, 97, 116, 101, 101, 115, 99, 97, 112, 101, 0,
];

static mut __slate_str3259: [i32; 16] = [
    115, 117, 114, 114, 111, 103, 97, 116, 101, 101, 115, 99, 97, 112, 101, 0,
];

static mut __slate_str3260: [i32; 7] = [115, 116, 114, 105, 99, 116, 0];

/// See also config_get_fs_encoding()
fn config_get_locale_encoding(
    config: *mut PyConfig,
    preconfig: *const PyPreConfig,
    locale_encoding: *mut *mut i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut encoding: *mut i32 = unsafe { core::mem::zeroed() };
    if (unsafe { (*preconfig).utf8_mode }) != 0 {
        encoding = unsafe {
            _PyMem_RawWcsdup((core::ptr::addr_of_mut!(__slate_str3261) as *mut i32) as *const i32)
        };
    } else {
        encoding = unsafe { _Py_GetLocaleEncoding() };
    }
    if encoding.is_null() {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__49!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    let status: __SlateRecord_initconfig_h_19 =
        PyConfig_SetString(config, locale_encoding, encoding as *const i32);
    unsafe { PyMem_RawFree(encoding as *mut ()) };
    return status;
}

static mut __slate_str3261: [i32; 6] = [117, 116, 102, 45, 56, 0];

fn config_init_stdio_encoding(
    config: *mut PyConfig,
    preconfig: *const PyPreConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // Exit if encoding and errors are defined
    if !unsafe { (*config).stdio_encoding }.is_null()
        && !unsafe { (*config).stdio_errors }.is_null()
    {
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
    // PYTHONIOENCODING environment variable
    let opt: *const i8 = config_get_env(config as *const PyConfig, c"PYTHONIOENCODING".as_ptr());
    if !opt.is_null() {
        let pythonioencoding: *mut i8 = unsafe { _PyMem_RawStrdup(opt) };
        if pythonioencoding.is_null() {
            return unsafe {
                *{
                    __slate_compound_1 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__68!() as *const i8;
                        __t1.err_msg = c"memory allocation failed".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            };
        }
        let mut errors: *mut i8 = unsafe {
            strchr(
                (pythonioencoding as *const i8) as *const core::ffi::c_char,
                58i32 as i32,
            ) as *mut i8
        };
        if !errors.is_null() {
            unsafe {
                *errors = 0;
            }
            let __v2: *mut i8 = errors;
            errors = __v2.wrapping_offset(1 as isize);
            if (unsafe { *errors.wrapping_offset(0 as isize) }) == 0 {
                errors = core::ptr::null_mut::<i8>();
            }
        }
        // Does PYTHONIOENCODING contain an encoding?
        if (unsafe { *pythonioencoding.wrapping_offset(0 as isize) }) != 0 {
            if unsafe { (*config).stdio_encoding }.is_null() {
                status = config_set_bytes_string(
                    config,
                    unsafe { core::ptr::addr_of_mut!((*config).stdio_encoding) },
                    pythonioencoding as *const i8,
                    c"cannot decode PYTHONIOENCODING environment variable".as_ptr(),
                );
                if _PyStatus_EXCEPTION!(status) {
                    unsafe { PyMem_RawFree(pythonioencoding as *mut ()) };
                    return status;
                }
            }
            // If the encoding is set but not the error handler,
            // use "strict" error handler by default.
            // PYTHONIOENCODING=latin1 behaves as
            // PYTHONIOENCODING=latin1:strict.
            if errors.is_null() {
                errors = c"strict".as_ptr() as *mut i8;
            }
        }
        if unsafe { (*config).stdio_errors }.is_null() && !errors.is_null() {
            status = config_set_bytes_string(
                config,
                unsafe { core::ptr::addr_of_mut!((*config).stdio_errors) },
                errors as *const i8,
                c"cannot decode PYTHONIOENCODING environment variable".as_ptr(),
            );
            if _PyStatus_EXCEPTION!(status) {
                unsafe { PyMem_RawFree(pythonioencoding as *mut ()) };
                return status;
            }
        }
        unsafe { PyMem_RawFree(pythonioencoding as *mut ()) };
    }
    // Choose the default error handler based on the current locale.
    if unsafe { (*config).stdio_encoding }.is_null() {
        status = config_get_locale_encoding(config, preconfig, unsafe {
            core::ptr::addr_of_mut!((*config).stdio_encoding)
        });
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if unsafe { (*config).stdio_errors }.is_null() {
        let errors: *const i32 = config_get_stdio_errors(preconfig);
        status = PyConfig_SetString(
            config,
            unsafe { core::ptr::addr_of_mut!((*config).stdio_errors) },
            errors,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    return unsafe {
        *{
            __slate_compound_4 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_4)
        }
    };
}

/// See also config_get_locale_encoding()
fn config_get_fs_encoding(
    config: *mut PyConfig,
    preconfig: *const PyPreConfig,
    fs_encoding: *mut *mut i32,
) -> __SlateRecord_initconfig_h_19 {
    if (unsafe { (*preconfig).utf8_mode }) != 0 {
        return PyConfig_SetString(
            config,
            fs_encoding,
            (unsafe { core::ptr::addr_of_mut!(__slate_str3274) as *mut i32 }) as *const i32,
        );
    }
    if (unsafe { _Py_GetForceASCII() }) != 0 {
        return PyConfig_SetString(
            config,
            fs_encoding,
            (unsafe { core::ptr::addr_of_mut!(__slate_str3275) as *mut i32 }) as *const i32,
        );
    }
    return config_get_locale_encoding(config, preconfig, fs_encoding);
}

static mut __slate_str3274: [i32; 6] = [117, 116, 102, 45, 56, 0];

static mut __slate_str3275: [i32; 6] = [97, 115, 99, 105, 105, 0];

fn config_init_fs_encoding(
    config: *mut PyConfig,
    preconfig: *const PyPreConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if unsafe { (*config).filesystem_encoding }.is_null() {
        status = config_get_fs_encoding(config, preconfig, unsafe {
            core::ptr::addr_of_mut!((*config).filesystem_encoding)
        });
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if unsafe { (*config).filesystem_errors }.is_null() {
        let errors: *const i32 =
            (unsafe { core::ptr::addr_of_mut!(__slate_str3276) as *mut i32 }) as *const i32;
        status = PyConfig_SetString(
            config,
            unsafe { core::ptr::addr_of_mut!((*config).filesystem_errors) },
            errors,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
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

static mut __slate_str3276: [i32; 16] = [
    115, 117, 114, 114, 111, 103, 97, 116, 101, 101, 115, 99, 97, 112, 101, 0,
];

fn config_init_import(
    config: *mut PyConfig,
    compute_path_config: i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    status = unsafe { _PyConfig_InitPathConfig(config, compute_path_config) };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    let env: *const i8 =
        config_get_env(config as *const PyConfig, c"PYTHON_FROZEN_MODULES".as_ptr());
    if !env.is_null() {
        if (unsafe {
            strcmp(
                env as *const core::ffi::c_char,
                c"on".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            unsafe {
                (*config).use_frozen_modules = 1;
            }
        } else {
            if (unsafe {
                strcmp(
                    env as *const core::ffi::c_char,
                    c"off".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                unsafe {
                    (*config).use_frozen_modules = 0;
                }
            } else {
                return PyStatus_Error(
                    c"bad value for PYTHON_FROZEN_MODULES (expected \"on\" or \"off\")".as_ptr(),
                );
            }
        }
    }
    // -X frozen_modules=[on|off]
    let value: *const i32 = config_get_xoption_value(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3282) as *mut i32
    });
    if !value.is_null() {
        if (unsafe {
            wcscmp(
                value,
                (core::ptr::addr_of_mut!(__slate_str3283) as *mut i32) as *const i32,
            )
        }) == 0
        {
            unsafe {
                (*config).use_frozen_modules = 1;
            }
        } else {
            if (unsafe {
                wcscmp(
                    value,
                    (core::ptr::addr_of_mut!(__slate_str3284) as *mut i32) as *const i32,
                )
            }) == 0
            {
                unsafe {
                    (*config).use_frozen_modules = 0;
                }
            } else {
                if (unsafe { wcslen(value as *const i32) as u64 }) == 0 {
                    // "-X frozen_modules" and "-X frozen_modules=" both imply "on".
                    unsafe {
                        (*config).use_frozen_modules = 1;
                    }
                } else {
                    return PyStatus_Error(
                        c"bad value for option -X frozen_modules (expected \"on\" or \"off\")"
                            .as_ptr(),
                    );
                }
            }
        }
    }
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

static mut __slate_str3282: [i32; 15] = [
    102, 114, 111, 122, 101, 110, 95, 109, 111, 100, 117, 108, 101, 115, 0,
];

static mut __slate_str3283: [i32; 3] = [111, 110, 0];

static mut __slate_str3284: [i32; 4] = [111, 102, 102, 0];

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_InitImportConfig(
    config: *mut PyConfig,
) -> __SlateRecord_initconfig_h_19 {
    return config_init_import(config, 1i32);
}

fn config_read(config: *mut PyConfig, compute_path_config: i32) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let preconfig: *const PyPreConfig =
        (unsafe { core::ptr::addr_of_mut!(_PyRuntime.preconfig) }) as *const PyPreConfig;
    if (unsafe { (*config).use_environment }) != 0 {
        status = config_read_env_vars(config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    // -X options
    if !config_get_xoption(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3287) as *mut i32
    })
    .is_null()
    {
        unsafe {
            (*config).show_ref_count = 1;
        }
    }
    let x_gil: *const i32 = config_get_xoption_value(config as *const PyConfig, unsafe {
        core::ptr::addr_of_mut!(__slate_str3288) as *mut i32
    });
    if !x_gil.is_null() {
        let len: u64 = unsafe { wcslen(x_gil as *const i32) as u64 };
        status = config_read_gil(config, len, unsafe { *x_gil.wrapping_offset(0 as isize) });
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    status = config_read_complex_options(config);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    if (unsafe { (*config)._install_importlib }) != 0 {
        status = config_init_import(config, compute_path_config);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    // default values
    if (unsafe { (*config).dev_mode }) != 0 {
        if (unsafe { (*config).faulthandler }) < 0 {
            unsafe {
                (*config).faulthandler = 1;
            }
        }
    }
    if (unsafe { (*config).faulthandler }) < 0 {
        unsafe {
            (*config).faulthandler = 0;
        }
    }
    if (unsafe { (*config).tracemalloc }) < 0 {
        unsafe {
            (*config).tracemalloc = 0;
        }
    }
    if (unsafe { (*config).perf_profiling }) < 0 {
        unsafe {
            (*config).perf_profiling = 0;
        }
    }
    if (unsafe { (*config).remote_debug }) < 0 {
        unsafe {
            (*config).remote_debug = 1i32.wrapping_neg();
        }
    }
    if (unsafe { (*config).use_hash_seed }) < 0 {
        unsafe {
            (*config).use_hash_seed = 0;
        }
        unsafe {
            (*config).hash_seed = 0;
        }
    }
    if unsafe { (*config).filesystem_encoding }.is_null()
        || unsafe { (*config).filesystem_errors }.is_null()
    {
        status = config_init_fs_encoding(config, preconfig);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    status = config_init_stdio_encoding(config, preconfig);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    if (unsafe { (*config).argv.length }) < 1 {
        // Ensure at least one (empty) argument is seen
        status = PyWideStringList_Append(
            unsafe { core::ptr::addr_of_mut!((*config).argv) },
            (unsafe { core::ptr::addr_of_mut!(__slate_str3289) as *mut i32 }) as *const i32,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if unsafe { (*config).check_hash_pycs_mode }.is_null() {
        status = PyConfig_SetString(
            config,
            unsafe { core::ptr::addr_of_mut!((*config).check_hash_pycs_mode) },
            (unsafe { core::ptr::addr_of_mut!(__slate_str3290) as *mut i32 }) as *const i32,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    if (unsafe { (*config).configure_c_stdio }) < 0 {
        unsafe {
            (*config).configure_c_stdio = 1;
        }
    }
    // Only parse arguments once.
    if (unsafe { (*config).parse_argv }) == 1 {
        unsafe {
            (*config).parse_argv = 2;
        }
    }
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

static mut __slate_str3287: [i32; 13] =
    [115, 104, 111, 119, 114, 101, 102, 99, 111, 117, 110, 116, 0];

static mut __slate_str3288: [i32; 4] = [103, 105, 108, 0];

static mut __slate_str3289: [i32; 1] = [0];

static mut __slate_str3290: [i32; 8] = [100, 101, 102, 97, 117, 108, 116, 0];

fn config_init_stdio(config: *const PyConfig) {
    if (unsafe { (*config).buffered_stdio }) == 0 {
        unsafe {
            setvbuf(
                slate_support::stdin(),
                core::ptr::null_mut::<i8>(),
                2i32,
                (slate_support::BUFSIZ as i32) as u64,
            )
        };
        unsafe {
            setvbuf(
                slate_support::stdout(),
                core::ptr::null_mut::<i8>(),
                2i32,
                (slate_support::BUFSIZ as i32) as u64,
            )
        };
        unsafe {
            setvbuf(
                slate_support::stderr(),
                core::ptr::null_mut::<i8>(),
                2i32,
                (slate_support::BUFSIZ as i32) as u64,
            )
        };
    } else {
        if (unsafe { (*config).interactive }) != 0 {
            unsafe {
                setvbuf(
                    slate_support::stdin(),
                    core::ptr::null_mut::<i8>(),
                    1i32,
                    (slate_support::BUFSIZ as i32) as u64,
                )
            };
            unsafe {
                setvbuf(
                    slate_support::stdout(),
                    core::ptr::null_mut::<i8>(),
                    1i32,
                    (slate_support::BUFSIZ as i32) as u64,
                )
            };
            // Leave stderr alone - it should be unbuffered anyway.
        }
    }
}

/// Write the configuration:
///
/// - set Py_xxx global configuration variables
/// - initialize C standard streams (stdin, stdout, stderr)
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_Write(
    config: *const PyConfig,
    runtime: *mut pyruntimestate,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    config_set_global_vars(config);
    if (unsafe { (*config).configure_c_stdio }) != 0 {
        config_init_stdio(config);
    }
    // Write the new pre-configuration into _PyRuntime
    let preconfig: *mut PyPreConfig = unsafe { core::ptr::addr_of_mut!((*runtime).preconfig) };
    unsafe {
        (*preconfig).isolated = (*config).isolated;
    }
    unsafe {
        (*preconfig).use_environment = (*config).use_environment;
    }
    unsafe {
        (*preconfig).dev_mode = (*config).dev_mode;
    }
    if _Py_SetArgcArgv(
        unsafe { (*config).orig_argv.length },
        (unsafe { (*config).orig_argv.items }) as *const *mut i32,
    ) < 0
    {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__55!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
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

// --- PyConfig command line parser --------------------------
fn config_usage(error: i32, program: *const i32) {
    let f: *mut slate_support::FILE = if error != 0 {
        slate_support::stderr()
    } else {
        slate_support::stdout()
    };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fprintf(f, core::ptr::addr_of!(usage_line.0) as *const i8, program) };
    unsafe { fflush(core::ptr::null_mut()) };
    if error != 0 {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fprintf(f, c"Try `python -h' for more information.\n".as_ptr()) };
        unsafe { fflush(core::ptr::null_mut()) };
    } else {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fputs(core::ptr::addr_of!(usage_help.0) as *const i8, f) };
        unsafe { fflush(core::ptr::null_mut()) };
    }
}

fn config_envvars_usage() {
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::printf(
            core::ptr::addr_of!(usage_envvars.0) as *const i8,
            58u32,
            c"<prefix>/lib/pythonX.X".as_ptr() as *mut i8,
            58u32,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
}

fn config_xoptions_usage() {
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::puts(core::ptr::addr_of!(usage_xoptions.0) as *const i8) };
    unsafe { fflush(core::ptr::null_mut()) };
}

fn config_complete_usage(program: *const i32) {
    config_usage(0i32, program);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    putchar(10i32);
    unsafe { fflush(core::ptr::null_mut()) };
    config_envvars_usage();
    let _ = std::io::Write::flush(&mut std::io::stdout());
    putchar(10i32);
    unsafe { fflush(core::ptr::null_mut()) };
    config_xoptions_usage();
}

/// Parse the command line arguments
fn config_parse_cmdline(
    config: *mut PyConfig,
    warnoptions: *mut __SlateRecord_initconfig_h_36,
    opt_index: &mut i64,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_5: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_6: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_7: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_26: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_32: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_34: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_35: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_38: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let argv: *const __SlateRecord_initconfig_h_36 =
        (unsafe { core::ptr::addr_of_mut!((*config).argv) })
            as *const __SlateRecord_initconfig_h_36;
    let mut print_version: i32 = 0;
    let mut program: *const i32 = (unsafe { (*config).program_name }) as *const i32;
    if program.is_null() && (unsafe { (*argv).length }) >= 1 {
        program = (unsafe { *unsafe { (*argv).items }.wrapping_offset(0 as isize) }) as *const i32;
    }
    unsafe { _PyOS_ResetGetOpt() };
    '__slate_break_0: loop {
        let mut longindex: i32 = 1i32.wrapping_neg();
        let c: i32 = unsafe {
            _PyOS_GetOpt(
                (*argv).length,
                (*argv).items as *const *mut i32,
                core::ptr::addr_of_mut!(longindex),
            )
        };
        if c == (slate_support::EOF as i32) {
            break '__slate_break_0;
        }
        if c == 99 {
            if unsafe { (*config).run_command }.is_null() {
                // -c is the last option; following arguments
                // that look like options are left for the
                // command to interpret.
                let len: u64 = unsafe { wcslen(_PyOS_optarg as *const i32) as u64 }
                    .wrapping_add(1)
                    .wrapping_add(1);
                let command: *mut i32 = (unsafe {
                    PyMem_RawMalloc((core::mem::size_of::<i32>() as u64).wrapping_mul(len))
                }) as *mut i32;
                if command.is_null() {
                    return unsafe {
                        *{
                            __slate_compound_1 = {
                                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t0._type = 1u32;
                                __t0.func = _PyStatus_GET_FUNC__7!() as *const i8;
                                __t0.err_msg = c"memory allocation failed".as_ptr();
                                __t0
                            };
                            core::ptr::addr_of_mut!(__slate_compound_1)
                        }
                    };
                }
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((_PyOS_optarg as *const ()) as *const core::ffi::c_void) as *const u8,
                        ((command as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                        (len.wrapping_sub(2)
                            .wrapping_mul(core::mem::size_of::<i32>() as u64)
                            as usize) as usize,
                    )
                };
                unsafe {
                    *command.wrapping_offset(len.wrapping_sub(2) as isize) = 10;
                }
                unsafe {
                    *command.wrapping_offset(len.wrapping_sub(1) as isize) = 0;
                }
                unsafe {
                    (*config).run_command = command;
                }
            }
            break '__slate_break_0;
        }
        if c == 109 {
            // -m is the last option; following arguments
            // that look like options are left for the
            // module to interpret.
            if unsafe { (*config).run_module }.is_null() {
                unsafe {
                    (*config).run_module = _PyMem_RawWcsdup(_PyOS_optarg);
                }
                if unsafe { (*config).run_module }.is_null() {
                    return unsafe {
                        *{
                            __slate_compound_2 = {
                                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t1._type = 1u32;
                                __t1.func = _PyStatus_GET_FUNC__7!() as *const i8;
                                __t1.err_msg = c"memory allocation failed".as_ptr();
                                __t1
                            };
                            core::ptr::addr_of_mut!(__slate_compound_2)
                        }
                    };
                }
            }
            break '__slate_break_0;
        }
        '__slate_break_4: {
            match c {
                0 => {
                    // Integers represent long options, see Python/getopt.c
                    if (unsafe {
                        wcscmp(
                            _PyOS_optarg,
                            (core::ptr::addr_of_mut!(__slate_str3306) as *mut i32) as *const i32,
                        )
                    }) == 0
                        || (unsafe {
                            wcscmp(
                                _PyOS_optarg,
                                (core::ptr::addr_of_mut!(__slate_str3307) as *mut i32)
                                    as *const i32,
                            )
                        }) == 0
                        || (unsafe {
                            wcscmp(
                                _PyOS_optarg,
                                (core::ptr::addr_of_mut!(__slate_str3308) as *mut i32)
                                    as *const i32,
                            )
                        }) == 0
                    {
                        status = PyConfig_SetString(
                            config,
                            unsafe { core::ptr::addr_of_mut!((*config).check_hash_pycs_mode) },
                            unsafe { _PyOS_optarg },
                        );
                        if _PyStatus_EXCEPTION!(status) {
                            return status;
                        }
                    } else {
                        let _ = std::io::Write::flush(&mut std::io::stdout());
                        unsafe {
                            slate_support::fprintf(slate_support::stderr(), c"--check-hash-based-pycs must be one of 'default', 'always', or 'never'\n".as_ptr())
                        };
                        unsafe { fflush(core::ptr::null_mut()) };
                        config_usage(1i32, program);
                        return unsafe {
                            *{
                                __slate_compound_3 = {
                                    let mut __t2: __SlateRecord_initconfig_h_19 =
                                        core::mem::zeroed();
                                    __t2._type = 2u32;
                                    __t2.exitcode = 2i32;
                                    __t2
                                };
                                core::ptr::addr_of_mut!(__slate_compound_3)
                            }
                        };
                    }
                    // check-hash-based-pycs
                }
                1 => {
                    config_complete_usage(program); // help-all
                    return unsafe {
                        *{
                            __slate_compound_5 = {
                                let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t3._type = 2u32;
                                __t3.exitcode = 0i32;
                                __t3
                            };
                            core::ptr::addr_of_mut!(__slate_compound_5)
                        }
                    };
                }
                2 => {
                    config_envvars_usage(); // help-env
                    return unsafe {
                        *{
                            __slate_compound_6 = {
                                let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t4._type = 2u32;
                                __t4.exitcode = 0i32;
                                __t4
                            };
                            core::ptr::addr_of_mut!(__slate_compound_6)
                        }
                    };
                }
                3 => {
                    config_xoptions_usage(); // help-xoptions
                    return unsafe {
                        *{
                            __slate_compound_7 = {
                                let mut __t5: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t5._type = 2u32;
                                __t5.exitcode = 0i32;
                                __t5
                            };
                            core::ptr::addr_of_mut!(__slate_compound_7)
                        }
                    };
                }
                98 => {
                    let __v9: i32 = unsafe { (*config).bytes_warning };
                    unsafe {
                        (*config).bytes_warning = __v9.wrapping_add(1);
                    }
                }
                100 => {
                    let __v12: i32 = unsafe { (*config).parser_debug };
                    unsafe {
                        (*config).parser_debug = __v12.wrapping_add(1);
                    }
                }
                105 => {
                    let __v15: i32 = unsafe { (*config).inspect };
                    unsafe {
                        (*config).inspect = __v15.wrapping_add(1);
                    }
                    let __v18: i32 = unsafe { (*config).interactive };
                    unsafe {
                        (*config).interactive = __v18.wrapping_add(1);
                    }
                }
                69 | 73 | 88 => {
                    break '__slate_break_4; // option handled by _PyPreCmdline_Read()
                }
                79 => {
                    let __v21: i32 = unsafe { (*config).optimization_level };
                    unsafe {
                        (*config).optimization_level = __v21.wrapping_add(1);
                    }
                }
                80 => unsafe {
                    (*config).safe_path = 1;
                },
                66 => unsafe {
                    (*config).write_bytecode = 0;
                },
                115 => unsafe {
                    (*config).user_site_directory = 0;
                },
                83 => unsafe {
                    (*config).site_import = 0;
                },
                116 => {
                    break '__slate_break_4; // ignored for backwards compatibility
                }
                117 => unsafe {
                    (*config).buffered_stdio = 0;
                },
                118 => {
                    let __v24: i32 = unsafe { (*config).verbose };
                    unsafe {
                        (*config).verbose = __v24.wrapping_add(1);
                    }
                }
                120 => unsafe {
                    (*config).skip_source_first_line = 1;
                },
                63 | 104 => {
                    config_usage(0i32, program);
                    return unsafe {
                        *{
                            __slate_compound_26 = {
                                let mut __t6: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t6._type = 2u32;
                                __t6.exitcode = 0i32;
                                __t6
                            };
                            core::ptr::addr_of_mut!(__slate_compound_26)
                        }
                    };
                }
                86 => {
                    print_version = print_version.wrapping_add(1);
                }
                87 => {
                    status = PyWideStringList_Append(warnoptions, unsafe { _PyOS_optarg });
                    if _PyStatus_EXCEPTION!(status) {
                        return status;
                    }
                }
                113 => {
                    let __v30: i32 = unsafe { (*config).quiet };
                    unsafe {
                        (*config).quiet = __v30.wrapping_add(1);
                    }
                }
                82 => {
                    unsafe {
                        (*config).use_hash_seed = 0;
                    }
                    break '__slate_break_4;
                    // This space reserved for other options
                }
                _ => {
                    config_usage(1i32, program); // unknown argument: parsing failed
                    return unsafe {
                        *{
                            __slate_compound_32 = {
                                let mut __t7: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t7._type = 2u32;
                                __t7.exitcode = 2i32;
                                __t7
                            };
                            core::ptr::addr_of_mut!(__slate_compound_32)
                        }
                    };
                }
            }
        }
        if false {
            break;
        }
    }
    if print_version != 0 {
        let __v33: *const i8 = if print_version >= 2 {
            unsafe { Py_GetVersion() }
        } else {
            c"3.14.8".as_ptr()
        };
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::printf(c"Python %s\n".as_ptr(), __v33) };
        unsafe { fflush(core::ptr::null_mut()) };
        return unsafe {
            *{
                __slate_compound_34 = {
                    let mut __t8: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t8._type = 2u32;
                    __t8.exitcode = 0i32;
                    __t8
                };
                core::ptr::addr_of_mut!(__slate_compound_34)
            }
        };
    }
    if unsafe { (*config).run_command }.is_null()
        && unsafe { (*config).run_module }.is_null()
        && (unsafe { _PyOS_optind }) < unsafe { (*argv).length }
        && (unsafe {
            wcscmp(
                *unsafe { (*argv).items }.wrapping_offset(_PyOS_optind as isize) as *const i32,
                (core::ptr::addr_of_mut!(__slate_str3319) as *mut i32) as *const i32,
            )
        }) != 0
        && unsafe { (*config).run_filename }.is_null()
    {
        unsafe {
            (*config).run_filename = _PyMem_RawWcsdup(
                *unsafe { (*argv).items }.wrapping_offset(_PyOS_optind as isize) as *const i32,
            );
        }
        if unsafe { (*config).run_filename }.is_null() {
            return unsafe {
                *{
                    __slate_compound_35 = {
                        let mut __t9: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t9._type = 1u32;
                        __t9.func = _PyStatus_GET_FUNC__7!() as *const i8;
                        __t9.err_msg = c"memory allocation failed".as_ptr();
                        __t9
                    };
                    core::ptr::addr_of_mut!(__slate_compound_35)
                }
            };
        }
    }
    if !unsafe { (*config).run_command }.is_null() || !unsafe { (*config).run_module }.is_null() {
        // Backup _PyOS_optind
        let __v36: i64 = unsafe { _PyOS_optind };
        unsafe {
            _PyOS_optind = __v36.wrapping_sub(1);
        }
    }
    unsafe {
        *(opt_index as *mut i64) = _PyOS_optind;
    }
    return unsafe {
        *{
            __slate_compound_38 = {
                let mut __t10: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t10._type = 0u32;
                __t10
            };
            core::ptr::addr_of_mut!(__slate_compound_38)
        }
    };
}

static mut __slate_str3306: [i32; 7] = [97, 108, 119, 97, 121, 115, 0];

static mut __slate_str3307: [i32; 6] = [110, 101, 118, 101, 114, 0];

static mut __slate_str3308: [i32; 8] = [100, 101, 102, 97, 117, 108, 116, 0];

static mut __slate_str3319: [i32; 2] = [45, 0];

/// Get warning options from PYTHONWARNINGS environment variable.
fn config_init_env_warnoptions(
    config: *mut PyConfig,
    warnoptions: *mut __SlateRecord_initconfig_h_36,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // CONFIG_GET_ENV_DUP requires dest to be initialized to NULL
    let mut env: *mut i32 = core::ptr::null_mut::<i32>();
    status = config_get_env_dup(
        config,
        core::ptr::addr_of_mut!(env),
        unsafe { core::ptr::addr_of_mut!(__slate_str3324) as *mut i32 },
        c"PYTHONWARNINGS".as_ptr() as *mut i8,
        c"cannot decode PYTHONWARNINGS".as_ptr(),
    );
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    // env var is not set or is empty
    if env.is_null() {
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
    let mut warning: *mut i32 = unsafe { core::mem::zeroed() };
    let mut context: *mut i32 = core::ptr::null_mut::<i32>();
    warning = unsafe {
        wcstok(
            env,
            (core::ptr::addr_of_mut!(__slate_str3329) as *mut i32) as *const i32,
            core::ptr::addr_of_mut!(context),
        )
    };
    while !warning.is_null() {
        status = PyWideStringList_Append(warnoptions, warning as *const i32);
        if _PyStatus_EXCEPTION!(status) {
            unsafe { PyMem_RawFree(env as *mut ()) };
            return status;
        }
        warning = unsafe {
            wcstok(
                core::ptr::null_mut::<i32>(),
                (core::ptr::addr_of_mut!(__slate_str3330) as *mut i32) as *const i32,
                core::ptr::addr_of_mut!(context),
            )
        };
    }
    unsafe { PyMem_RawFree(env as *mut ()) };
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t1._type = 0u32;
                __t1
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}

static mut __slate_str3324: [i32; 15] = [80, 89, 84, 72, 79, 78, 87, 65, 82, 78, 73, 78, 71, 83, 0];

static mut __slate_str3329: [i32; 2] = [44, 0];

static mut __slate_str3330: [i32; 2] = [44, 0];

fn warnoptions_append(
    config: *mut PyConfig,
    options: *mut __SlateRecord_initconfig_h_36,
    option: *const i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // config_init_warnoptions() add existing config warnoptions at the end:
    // ensure that the new option is not already present in this list to
    // prevent change the options order when config_init_warnoptions() is
    // called twice.
    if _PyWideStringList_Find(
        unsafe { core::ptr::addr_of_mut!((*config).warnoptions) },
        option,
    ) != 0
    {
        // Already present: do nothing
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
    if _PyWideStringList_Find(options, option) != 0 {
        // Already present: do nothing
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
    return PyWideStringList_Append(options, option);
}

fn warnoptions_extend(
    config: *mut PyConfig,
    options: *mut __SlateRecord_initconfig_h_36,
    options2: &__SlateRecord_initconfig_h_36,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let len: i64 = unsafe { (*(options2 as *const __SlateRecord_initconfig_h_36)).length };
    let items: *const *mut i32 =
        (unsafe { (*(options2 as *const __SlateRecord_initconfig_h_36)).items }) as *const *mut i32;
    for i in 0..len {
        let status: __SlateRecord_initconfig_h_19 = warnoptions_append(
            config,
            options,
            (unsafe { *items.wrapping_offset(i as isize) }) as *const i32,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    return unsafe {
        *{
            __slate_compound_1 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        }
    };
}

fn config_init_warnoptions(
    config: *mut PyConfig,
    cmdline_warnoptions: &__SlateRecord_initconfig_h_36,
    env_warnoptions: &__SlateRecord_initconfig_h_36,
    sys_warnoptions: &__SlateRecord_initconfig_h_36,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut filter: *const i32 = unsafe { core::mem::zeroed() };
    let mut options: core::mem::MaybeUninit<__SlateRecord_initconfig_h_36> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut __SlateRecord_initconfig_h_36 =
        core::ptr::addr_of_mut!(options) as *mut __SlateRecord_initconfig_h_36;
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    unsafe {
        '__join_0: {
            core::ptr::write(__slate_slot_1, *{
                __slate_compound_4 = __SlateRecord_initconfig_h_36 {
                    length: 0,
                    items: core::ptr::null_mut::<*mut i32>(),
                };
                core::ptr::addr_of_mut!(__slate_compound_4)
            });
            // Priority of warnings options, lowest to highest:
            //
            // - any implicit filters added by _warnings.c/warnings.py
            // - PyConfig.dev_mode: "default" filter
            // - PYTHONWARNINGS environment variable
            // - '-W' command line options
            // - PyConfig.bytes_warning ('-b' and '-bb' command line options):
            //   "default::BytesWarning" or "error::BytesWarning" filter
            // - early PySys_AddWarnOption() calls
            // - PyConfig.warnoptions
            //
            // PyConfig.warnoptions is copied to sys.warnoptions. Since the warnings
            // module works on the basis of "the most recently added filter will be
            // checked first", we add the lowest precedence entries first so that later
            // entries override them.
            if (*config).dev_mode != 0 {
                *__slate_slot_2 = warnoptions_append(
                    config,
                    core::ptr::addr_of_mut!(*__slate_slot_1),
                    (core::ptr::addr_of_mut!(__slate_str3337) as *mut i32) as *const i32,
                );
                if _PyStatus_EXCEPTION!(*__slate_slot_2) {
                    break '__join_0;
                }
            }
            *__slate_slot_2 = warnoptions_extend(
                config,
                core::ptr::addr_of_mut!(*__slate_slot_1),
                env_warnoptions,
            );
            if !_PyStatus_EXCEPTION!(*__slate_slot_2) {
                *__slate_slot_2 = warnoptions_extend(
                    config,
                    core::ptr::addr_of_mut!(*__slate_slot_1),
                    cmdline_warnoptions,
                );
                if !_PyStatus_EXCEPTION!(*__slate_slot_2) {
                    // If the bytes_warning_flag isn't set, bytesobject.c and bytearrayobject.c
                    // don't even try to emit a warning, so we skip setting the filter in that
                    // case.
                    if (*config).bytes_warning != 0 {
                        if (*config).bytes_warning > 1 {
                            filter = (core::ptr::addr_of_mut!(__slate_str3338) as *mut i32)
                                as *const i32;
                        } else {
                            filter = (core::ptr::addr_of_mut!(__slate_str3339) as *mut i32)
                                as *const i32;
                        }
                        *__slate_slot_2 = warnoptions_append(
                            config,
                            core::ptr::addr_of_mut!(*__slate_slot_1),
                            filter,
                        );
                        if _PyStatus_EXCEPTION!(*__slate_slot_2) {
                            break '__join_0;
                        }
                    }
                    *__slate_slot_2 = warnoptions_extend(
                        config,
                        core::ptr::addr_of_mut!(*__slate_slot_1),
                        sys_warnoptions,
                    );
                    if !_PyStatus_EXCEPTION!(*__slate_slot_2) {
                        // Always add all PyConfig.warnoptions options
                        *__slate_slot_2 = _PyWideStringList_Extend(
                            core::ptr::addr_of_mut!(*__slate_slot_1),
                            core::ptr::addr_of_mut!((*config).warnoptions)
                                as *const __SlateRecord_initconfig_h_36,
                        );
                        if !_PyStatus_EXCEPTION!(*__slate_slot_2) {
                            _PyWideStringList_Clear(core::ptr::addr_of_mut!((*config).warnoptions));
                            (*config).warnoptions = *__slate_slot_1;
                            return *{
                                __slate_compound_3 = {
                                    let mut __t0: __SlateRecord_initconfig_h_19 =
                                        core::mem::zeroed();
                                    __t0._type = 0u32;
                                    __t0
                                };
                                core::ptr::addr_of_mut!(__slate_compound_3)
                            };
                        }
                    }
                }
            }
        }
        _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_1));
        return *__slate_slot_2;
    }
}

static mut __slate_str3337: [i32; 8] = [100, 101, 102, 97, 117, 108, 116, 0];

static mut __slate_str3338: [i32; 20] = [
    101, 114, 114, 111, 114, 58, 58, 66, 121, 116, 101, 115, 87, 97, 114, 110, 105, 110, 103, 0,
];

static mut __slate_str3339: [i32; 22] = [
    100, 101, 102, 97, 117, 108, 116, 58, 58, 66, 121, 116, 101, 115, 87, 97, 114, 110, 105, 110,
    103, 0,
];

fn config_update_argv(config: *mut PyConfig, opt_index: i64) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let cmdline_argv: *const __SlateRecord_initconfig_h_36 =
        (unsafe { core::ptr::addr_of_mut!((*config).argv) })
            as *const __SlateRecord_initconfig_h_36;
    let mut config_argv: __SlateRecord_initconfig_h_36 = unsafe {
        *{
            __slate_compound_0 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    // Copy argv to be able to modify it (to force -c/-m)
    if (unsafe { (*cmdline_argv).length }) <= opt_index {
        // Ensure at least one (empty) argument is seen
        let status: __SlateRecord_initconfig_h_19 = PyWideStringList_Append(
            core::ptr::addr_of_mut!(config_argv),
            (unsafe { core::ptr::addr_of_mut!(__slate_str3342) as *mut i32 }) as *const i32,
        );
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    } else {
        let mut slice: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
        slice.length = unsafe { (*cmdline_argv).length }.wrapping_sub(opt_index);
        slice.items = unsafe { (*cmdline_argv).items }.wrapping_offset(opt_index as isize);
        if _PyWideStringList_Copy(
            core::ptr::addr_of_mut!(config_argv),
            core::ptr::addr_of_mut!(slice) as *const __SlateRecord_initconfig_h_36,
        ) < 0
        {
            return unsafe {
                *{
                    __slate_compound_1 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__26!() as *const i8;
                        __t0.err_msg = c"memory allocation failed".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            };
        }
    }
    let mut arg0: *mut i32 = core::ptr::null_mut::<i32>();
    if !unsafe { (*config).run_command }.is_null() {
        // Force sys.argv[0] = '-c'
        arg0 = unsafe { core::ptr::addr_of_mut!(__slate_str3346) as *mut i32 };
    } else {
        if !unsafe { (*config).run_module }.is_null() {
            // Force sys.argv[0] = '-m'
            arg0 = unsafe { core::ptr::addr_of_mut!(__slate_str3347) as *mut i32 };
        }
    }
    if !arg0.is_null() {
        arg0 = unsafe { _PyMem_RawWcsdup(arg0 as *const i32) };
        if arg0.is_null() {
            _PyWideStringList_Clear(core::ptr::addr_of_mut!(config_argv));
            return unsafe {
                *{
                    __slate_compound_2 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__26!() as *const i8;
                        __t1.err_msg = c"memory allocation failed".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
        unsafe { PyMem_RawFree(*config_argv.items.wrapping_offset(0 as isize) as *mut ()) };
        unsafe {
            *config_argv.items.wrapping_offset(0 as isize) = arg0;
        }
    }
    _PyWideStringList_Clear(unsafe { core::ptr::addr_of_mut!((*config).argv) });
    unsafe {
        (*config).argv = config_argv;
    }
    return unsafe {
        *{
            __slate_compound_3 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_3)
        }
    };
}

static mut __slate_str3342: [i32; 1] = [0];

static mut __slate_str3346: [i32; 3] = [45, 99, 0];

static mut __slate_str3347: [i32; 3] = [45, 109, 0];

fn core_read_precmdline(
    config: *mut PyConfig,
    precmdline: *mut __SlateRecord_pycore_initconfig_h_108,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if (unsafe { (*config).parse_argv }) == 1 {
        if _PyWideStringList_Copy(
            unsafe { core::ptr::addr_of_mut!((*precmdline).argv) },
            (unsafe { core::ptr::addr_of_mut!((*config).argv) })
                as *const __SlateRecord_initconfig_h_36,
        ) < 0
        {
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__41!() as *const i8;
                        __t0.err_msg = c"memory allocation failed".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
    }
    let mut preconfig: PyPreConfig = unsafe { core::mem::zeroed() };
    status = unsafe {
        _PyPreConfig_InitFromPreConfig(
            core::ptr::addr_of_mut!(preconfig),
            core::ptr::addr_of_mut!(_PyRuntime.preconfig) as *const PyPreConfig,
        )
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    unsafe {
        _PyPreConfig_GetConfig(
            core::ptr::addr_of_mut!(preconfig),
            config as *const PyConfig,
        )
    };
    status = unsafe {
        _PyPreCmdline_Read(
            precmdline,
            core::ptr::addr_of_mut!(preconfig) as *const PyPreConfig,
        )
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    status = unsafe {
        _PyPreCmdline_SetConfig(
            precmdline as *const __SlateRecord_pycore_initconfig_h_108,
            config,
        )
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
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

/// Get run_filename absolute path
fn config_run_filename_abspath(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if unsafe { (*config).run_filename }.is_null() {
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
    if (unsafe { _Py_isabs((*config).run_filename as *const i32) }) != 0 {
        // path is already absolute
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
    let mut abs_filename: *mut i32 = unsafe { core::mem::zeroed() };
    if (unsafe {
        _Py_abspath(
            (*config).run_filename as *const i32,
            core::ptr::addr_of_mut!(abs_filename),
        )
    }) < 0
    {
        // failed to get the absolute path of the command line filename:
        // ignore the error, keep the relative path
        return unsafe {
            *{
                __slate_compound_2 = {
                    let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t2._type = 0u32;
                    __t2
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            }
        };
    }
    if abs_filename.is_null() {
        return unsafe {
            *{
                __slate_compound_3 = {
                    let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t3._type = 1u32;
                    __t3.func = _PyStatus_GET_FUNC__77!() as *const i8;
                    __t3.err_msg = c"memory allocation failed".as_ptr();
                    __t3
                };
                core::ptr::addr_of_mut!(__slate_compound_3)
            }
        };
    }
    unsafe { PyMem_RawFree((*config).run_filename as *mut ()) };
    unsafe {
        (*config).run_filename = abs_filename;
    }
    return unsafe {
        *{
            __slate_compound_4 = {
                let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t4._type = 0u32;
                __t4
            };
            core::ptr::addr_of_mut!(__slate_compound_4)
        }
    };
}

fn config_read_cmdline(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_5: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_6: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_7: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_8: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut opt_index: i64 = unsafe { core::mem::zeroed() };
    let mut sys_warnoptions: core::mem::MaybeUninit<__SlateRecord_initconfig_h_36> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut __SlateRecord_initconfig_h_36 =
        core::ptr::addr_of_mut!(sys_warnoptions) as *mut __SlateRecord_initconfig_h_36;
    let mut env_warnoptions: core::mem::MaybeUninit<__SlateRecord_initconfig_h_36> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_initconfig_h_36 =
        core::ptr::addr_of_mut!(env_warnoptions) as *mut __SlateRecord_initconfig_h_36;
    let mut cmdline_warnoptions: core::mem::MaybeUninit<__SlateRecord_initconfig_h_36> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut __SlateRecord_initconfig_h_36 =
        core::ptr::addr_of_mut!(cmdline_warnoptions) as *mut __SlateRecord_initconfig_h_36;
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    unsafe {
        core::ptr::write(__slate_slot_3, *{
            __slate_compound_6 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_6)
        });
        core::ptr::write(__slate_slot_2, *{
            __slate_compound_7 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_7)
        });
        core::ptr::write(__slate_slot_1, *{
            __slate_compound_8 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_8)
        });
        if (*config).parse_argv < 0 {
            (*config).parse_argv = 1;
        }
        '__join_0: {
            if (*config).parse_argv == 1 {
                *__slate_slot_4 = config_parse_cmdline(
                    config,
                    core::ptr::addr_of_mut!(*__slate_slot_3),
                    &mut (*core::ptr::addr_of_mut!(opt_index)),
                );
                if _PyStatus_EXCEPTION!(*__slate_slot_4) {
                    break '__join_0;
                } else {
                    *__slate_slot_4 = config_run_filename_abspath(config);
                    if _PyStatus_EXCEPTION!(*__slate_slot_4) {
                        break '__join_0;
                    } else {
                        *__slate_slot_4 = config_update_argv(config, opt_index);
                        if _PyStatus_EXCEPTION!(*__slate_slot_4) {
                            break '__join_0;
                        }
                    }
                }
            } else {
                *__slate_slot_4 = config_run_filename_abspath(config);
                if _PyStatus_EXCEPTION!(*__slate_slot_4) {
                    break '__join_0;
                }
            }
            if (*config).use_environment != 0 {
                *__slate_slot_4 =
                    config_init_env_warnoptions(config, core::ptr::addr_of_mut!(*__slate_slot_2));
                if _PyStatus_EXCEPTION!(*__slate_slot_4) {
                    break '__join_0;
                }
            }
            // Handle early PySys_AddWarnOption() calls
            *__slate_slot_4 =
                _PySys_ReadPreinitWarnOptions(core::ptr::addr_of_mut!(*__slate_slot_1));
            if !_PyStatus_EXCEPTION!(*__slate_slot_4) {
                *__slate_slot_4 = config_init_warnoptions(
                    config,
                    &(*(core::ptr::addr_of_mut!(*__slate_slot_3)
                        as *const __SlateRecord_initconfig_h_36)),
                    &(*(core::ptr::addr_of_mut!(*__slate_slot_2)
                        as *const __SlateRecord_initconfig_h_36)),
                    &(*(core::ptr::addr_of_mut!(*__slate_slot_1)
                        as *const __SlateRecord_initconfig_h_36)),
                );
                if !_PyStatus_EXCEPTION!(*__slate_slot_4) {
                    *__slate_slot_4 = *{
                        __slate_compound_5 = {
                            let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t0._type = 0u32;
                            __t0
                        };
                        core::ptr::addr_of_mut!(__slate_compound_5)
                    };
                }
            }
        }
        _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_3));
        _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_2));
        _PyWideStringList_Clear(core::ptr::addr_of_mut!(*__slate_slot_1));
        return *__slate_slot_4;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_SetPyArgv(
    config: *mut PyConfig,
    args: *const _PyArgv,
) -> __SlateRecord_initconfig_h_19 {
    let status: __SlateRecord_initconfig_h_19 =
        unsafe { _Py_PreInitializeFromConfig(config as *const PyConfig, args) };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    return unsafe { _PyArgv_AsWstrList(args, core::ptr::addr_of_mut!((*config).argv)) };
}

/// Set config.argv: decode argv using Py_DecodeLocale(). Pre-initialize Python
/// if needed to ensure that encodings are properly configured.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_SetBytesArgv(
    config: *mut PyConfig,
    argc: i64,
    argv: *const *mut i8,
) -> __SlateRecord_initconfig_h_19 {
    let mut args: _PyArgv = {
        let mut __t0: _PyArgv = unsafe { core::mem::zeroed() };
        __t0.argc = argc;
        __t0.use_bytes_argv = 1i32;
        __t0.bytes_argv = argv;
        __t0.wchar_argv = core::ptr::null::<*mut i32>();
        __t0
    };
    return _PyConfig_SetPyArgv(config, core::ptr::addr_of_mut!(args) as *const _PyArgv);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_SetArgv(
    config: *mut PyConfig,
    argc: i64,
    argv: *const *mut i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut args: _PyArgv = {
        let mut __t0: _PyArgv = unsafe { core::mem::zeroed() };
        __t0.argc = argc;
        __t0.use_bytes_argv = 0i32;
        __t0.bytes_argv = core::ptr::null::<*mut i8>();
        __t0.wchar_argv = argv;
        __t0
    };
    return _PyConfig_SetPyArgv(config, core::ptr::addr_of_mut!(args) as *const _PyArgv);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_SetWideStringList(
    config: *mut PyConfig,
    list: *mut __SlateRecord_initconfig_h_36,
    length: i64,
    items: *mut *mut i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 = unsafe {
        _Py_PreInitializeFromConfig(config as *const PyConfig, core::ptr::null::<_PyArgv>())
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    let mut list2: __SlateRecord_initconfig_h_36 = __SlateRecord_initconfig_h_36 {
        length: length,
        items: items,
    };
    if _PyWideStringList_Copy(
        list,
        core::ptr::addr_of_mut!(list2) as *const __SlateRecord_initconfig_h_36,
    ) < 0
    {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__78!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
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

/// Read the configuration into PyConfig from:
///
/// * Command line arguments
/// * Environment variables
/// * Py_xxx global configuration variables
///
/// The only side effects are to modify config and to call _Py_SetArgcArgv().
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_Read(
    config: *mut PyConfig,
    compute_path_config: i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_initconfig_h_108 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut precmdline: core::mem::MaybeUninit<__SlateRecord_pycore_initconfig_h_108> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pycore_initconfig_h_108 =
        core::ptr::addr_of_mut!(precmdline) as *mut __SlateRecord_pycore_initconfig_h_108;
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    unsafe {
        *__slate_slot_1 =
            _Py_PreInitializeFromConfig(config as *const PyConfig, core::ptr::null::<_PyArgv>());
        if _PyStatus_EXCEPTION!(*__slate_slot_1) {
            return *__slate_slot_1;
        } else {
            config_get_global_vars(config);
            if (*config).orig_argv.length == 0
                && !((*config).argv.length == 1
                    && wcscmp(
                        *unsafe { (*config).argv.items }.wrapping_offset(0 as isize) as *const i32,
                        (core::ptr::addr_of_mut!(__slate_str3371) as *mut i32) as *const i32,
                    ) == 0)
            {
                if _PyWideStringList_Copy(
                    core::ptr::addr_of_mut!((*config).orig_argv),
                    core::ptr::addr_of_mut!((*config).argv) as *const __SlateRecord_initconfig_h_36,
                ) < 0
                {
                    return *{
                        __slate_compound_4 = {
                            let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t2._type = 1u32;
                            __t2.func = _PyStatus_GET_FUNC__43!() as *const i8;
                            __t2.err_msg = c"memory allocation failed".as_ptr();
                            __t2
                        };
                        core::ptr::addr_of_mut!(__slate_compound_4)
                    };
                }
            }
            core::ptr::write(__slate_slot_0, *{
                __slate_compound_3 = {
                    let mut __t1: __SlateRecord_pycore_initconfig_h_108 = core::mem::zeroed();
                    __t1.isolated = 1i32.wrapping_neg();
                    __t1.use_environment = 1i32.wrapping_neg();
                    __t1.dev_mode = 1i32.wrapping_neg();
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_3)
            });
            *__slate_slot_1 =
                core_read_precmdline(config, core::ptr::addr_of_mut!(*__slate_slot_0));
            if !_PyStatus_EXCEPTION!(*__slate_slot_1) {
                if (*config).isolated != 0 {
                    (*config).safe_path = 1;
                    (*config).use_environment = 0;
                    (*config).user_site_directory = 0;
                }
                *__slate_slot_1 = config_read_cmdline(config);
                if !_PyStatus_EXCEPTION!(*__slate_slot_1) {
                    // Handle early PySys_AddXOption() calls
                    *__slate_slot_1 = _PySys_ReadPreinitXOptions(config);
                    if !_PyStatus_EXCEPTION!(*__slate_slot_1) {
                        *__slate_slot_1 = config_read(config, compute_path_config);
                        if !_PyStatus_EXCEPTION!(*__slate_slot_1) {
                            *__slate_slot_1 = *{
                                __slate_compound_2 = {
                                    let mut __t0: __SlateRecord_initconfig_h_19 =
                                        core::mem::zeroed();
                                    __t0._type = 0u32;
                                    __t0
                                };
                                core::ptr::addr_of_mut!(__slate_compound_2)
                            };
                        }
                    }
                }
            }
            _PyPreCmdline_Clear(core::ptr::addr_of_mut!(*__slate_slot_0));
            return *__slate_slot_1;
        }
    }
}

static mut __slate_str3371: [i32; 1] = [0];

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_Read(config: *mut PyConfig) -> __SlateRecord_initconfig_h_19 {
    return _PyConfig_Read(config, 0i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_GetConfigsAsDict() -> *mut _object {
    // pre config
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        dict = core::ptr::null_mut::<_object>();
        result = PyDict_New();
        if !result.is_null() {
            // global result
            dict = _Py_GetGlobalVariablesAsDict();
            if !dict.is_null() {
                if PyDict_SetItemString(result, c"global_config".as_ptr(), dict) >= 0 {
                    Py_CLEAR__2!(dict);
                    interp = _PyInterpreterState_GET();
                    let pre_config: *const PyPreConfig =
                        core::ptr::addr_of_mut!((*(*interp).runtime).preconfig)
                            as *const PyPreConfig;
                    dict = _PyPreConfig_AsDict(pre_config);
                    if !dict.is_null() {
                        if PyDict_SetItemString(result, c"pre_config".as_ptr(), dict) >= 0 {
                            Py_CLEAR__2!(dict);
                            // core config
                            let config: *const PyConfig = _PyInterpreterState_GetConfig(interp);
                            dict = _PyConfig_AsDict(config);
                            if !dict.is_null() {
                                if PyDict_SetItemString(result, c"config".as_ptr(), dict) >= 0 {
                                    Py_CLEAR__2!(dict);
                                    return result;
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(result);
        Py_XDECREF!(dict);
        return core::ptr::null_mut::<_object>();
    }
}

fn init_dump_ascii_wstr(mut str: *const i32) {
    if str.is_null() {
        unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        return;
    }
    unsafe { PySys_WriteStderr(c"'".as_ptr()) };
    while (unsafe { *str }) != 0 {
        let ch: u32 = (unsafe { *str }) as u32;
        if ch == 39 {
            unsafe { PySys_WriteStderr(c"\\'".as_ptr()) };
        } else {
            if 32 <= ch && ch < 127 {
                unsafe { PySys_WriteStderr(c"%c".as_ptr(), ch) };
            } else {
                if ch <= 255 {
                    unsafe { PySys_WriteStderr(c"\\x%02x".as_ptr(), ch) };
                } else {
                    if ch > 65535 {
                        unsafe { PySys_WriteStderr(c"\\U%08x".as_ptr(), ch) };
                    } else {
                        unsafe { PySys_WriteStderr(c"\\u%04x".as_ptr(), ch) };
                    }
                }
            }
        }
        let __v1: *const i32 = str;
        str = __v1.wrapping_offset(1 as isize);
    }
    unsafe { PySys_WriteStderr(c"'".as_ptr()) };
}

/// Dump the Python path configuration into sys.stderr
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_DumpPathConfig(tstate: *mut _ts) {
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    unsafe { PySys_WriteStderr(c"Python path configuration:\n".as_ptr()) };
    let config: *const PyConfig = unsafe { _PyInterpreterState_GetConfig((*tstate).interp) };
    loop {
        unsafe { PySys_WriteStderr(c"  PYTHONHOME = ".as_ptr()) };
        init_dump_ascii_wstr((unsafe { (*config).home }) as *const i32);
        unsafe { PySys_WriteStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_WriteStderr(c"  PYTHONPATH = ".as_ptr()) };
        init_dump_ascii_wstr((unsafe { (*config).pythonpath_env }) as *const i32);
        unsafe { PySys_WriteStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_WriteStderr(c"  program name = ".as_ptr()) };
        init_dump_ascii_wstr((unsafe { (*config).program_name }) as *const i32);
        unsafe { PySys_WriteStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    unsafe { PySys_WriteStderr(c"  isolated = %i\n".as_ptr(), (*config).isolated) };
    unsafe { PySys_WriteStderr(c"  environment = %i\n".as_ptr(), (*config).use_environment) };
    unsafe {
        PySys_WriteStderr(
            c"  user site = %i\n".as_ptr(),
            (*config).user_site_directory,
        )
    };
    unsafe { PySys_WriteStderr(c"  safe_path = %i\n".as_ptr(), (*config).safe_path) };
    unsafe { PySys_WriteStderr(c"  import site = %i\n".as_ptr(), (*config).site_import) };
    unsafe {
        PySys_WriteStderr(
            c"  is in build tree = %i\n".as_ptr(),
            (*config)._is_python_build,
        )
    };
    loop {
        unsafe { PySys_WriteStderr(c"  stdlib dir = ".as_ptr()) };
        init_dump_ascii_wstr((unsafe { (*config).stdlib_dir }) as *const i32);
        unsafe { PySys_WriteStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_WriteStderr(c"  sys.path[0] = ".as_ptr()) };
        init_dump_ascii_wstr((unsafe { (*config).sys_path_0 }) as *const i32);
        unsafe { PySys_WriteStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    loop {
        unsafe {
            PySys_FormatStderr(
                c"  sys.%s = ".as_ptr(),
                c"_base_executable".as_ptr() as *mut i8,
            )
        };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"_base_executable".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_FormatStderr(c"  sys.%s = ".as_ptr(), c"base_prefix".as_ptr() as *mut i8) };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"base_prefix".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe {
            PySys_FormatStderr(
                c"  sys.%s = ".as_ptr(),
                c"base_exec_prefix".as_ptr() as *mut i8,
            )
        };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"base_exec_prefix".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_FormatStderr(c"  sys.%s = ".as_ptr(), c"platlibdir".as_ptr() as *mut i8) };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"platlibdir".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_FormatStderr(c"  sys.%s = ".as_ptr(), c"executable".as_ptr() as *mut i8) };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"executable".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_FormatStderr(c"  sys.%s = ".as_ptr(), c"prefix".as_ptr() as *mut i8) };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"prefix".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    loop {
        unsafe { PySys_FormatStderr(c"  sys.%s = ".as_ptr(), c"exec_prefix".as_ptr() as *mut i8) };
        if (unsafe {
            _PySys_GetOptionalAttrString(c"exec_prefix".as_ptr(), core::ptr::addr_of_mut!(obj))
        }) < 0
        {
            unsafe { PyErr_Clear() };
        }
        if !obj.is_null() {
            unsafe { PySys_FormatStderr(c"%A".as_ptr(), obj) };
            Py_DECREF!(obj);
        } else {
            unsafe { PySys_WriteStderr(c"(not set)".as_ptr()) };
        }
        unsafe { PySys_FormatStderr(c"\n".as_ptr()) };
        if true {
            break;
        }
    }
    let mut sys_path: *mut _object = unsafe { core::mem::zeroed() };
    unsafe { _PySys_GetOptionalAttrString(c"path".as_ptr(), core::ptr::addr_of_mut!(sys_path)) };
    let __v12: bool = !sys_path.is_null() && PyList_Check!(sys_path) != 0;
    if __v12 {
        unsafe { PySys_WriteStderr(c"  sys.path = [\n".as_ptr()) };
        let len: i64 = PyList_GET_SIZE!(sys_path);
        for i in 0..len {
            let path: *mut _object = PyList_GET_ITEM!(sys_path, i);
            unsafe { PySys_FormatStderr(c"    %A,\n".as_ptr(), path) };
        }
        unsafe { PySys_WriteStderr(c"  ]\n".as_ptr()) };
    }
    Py_XDECREF!(sys_path);
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
}

// --- PyInitConfig API ---------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy)]
struct PyInitConfig {
    preconfig: PyPreConfig,
    config: PyConfig,
    inittab: *mut _inittab,
    inittab_size: i64,
    status: __SlateRecord_initconfig_h_19,
    err_msg: *mut i8,
}

fn initconfig_alloc() -> *mut PyInitConfig {
    (unsafe {
        calloc(
            1u64 as usize,
            (core::mem::size_of::<PyInitConfig>() as u64) as usize,
        ) as *mut ()
    }) as *mut PyInitConfig
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_Create() -> *mut PyInitConfig {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let config: *mut PyInitConfig = initconfig_alloc();
    if config.is_null() {
        return core::ptr::null_mut::<PyInitConfig>();
    }
    unsafe { PyPreConfig_InitIsolatedConfig(core::ptr::addr_of_mut!((*config).preconfig)) };
    PyConfig_InitIsolatedConfig(unsafe { core::ptr::addr_of_mut!((*config).config) });
    unsafe {
        (*config).status = *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    config
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_Free(config: *mut PyInitConfig) {
    if config.is_null() {
        return;
    }
    initconfig_free_config(
        (unsafe { core::ptr::addr_of_mut!((*config).config) }) as *const PyConfig,
    );
    unsafe { PyMem_RawFree((*config).inittab as *mut ()) };
    unsafe { free(((*config).err_msg as *mut ()) as *mut core::ffi::c_void) };
    unsafe { free((config as *mut ()) as *mut core::ffi::c_void) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_GetError(
    config: *mut PyInitConfig,
    perr_msg: *mut *const i8,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if _PyStatus_IS_EXIT__2!((*config).status) {
        let mut buffer: __SlateAlign16<[i8; 22]> = __SlateAlign16([0i8; 22]); // len("exit code -2147483648\0")
        unsafe {
            PyOS_snprintf(
                buffer.0.as_mut_ptr() as *mut i8,
                core::mem::size_of::<[i8; 22]>() as u64,
                c"exit code %i".as_ptr(),
                (*config).status.exitcode,
            )
        };
        if !unsafe { (*config).err_msg }.is_null() {
            unsafe { free(((*config).err_msg as *mut ()) as *mut core::ffi::c_void) };
        }
        unsafe {
            (*config).err_msg =
                strdup((buffer.0.as_mut_ptr() as *const i8) as *const core::ffi::c_char) as *mut i8;
        }
        if !unsafe { (*config).err_msg }.is_null() {
            unsafe {
                *perr_msg = (*config).err_msg as *const i8;
            }
            return 1;
        }
        unsafe {
            (*config).status = *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__53!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
    }
    if _PyStatus_IS_ERROR__2!((*config).status) && !unsafe { (*config).status.err_msg }.is_null() {
        unsafe {
            *perr_msg = (*config).status.err_msg;
        }
        return 1;
    } else {
        unsafe {
            *perr_msg = core::ptr::null::<i8>();
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_GetExitCode(
    config: *mut PyInitConfig,
    exitcode: *mut i32,
) -> i32 {
    if _PyStatus_IS_EXIT__2!((*config).status) {
        unsafe {
            *exitcode = (*config).status.exitcode;
        }
        return 1;
    } else {
        return 0;
    }
}

fn initconfig_set_error(config: *mut PyInitConfig, err_msg: *const i8) {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    unsafe {
        (*config).status = *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__40!() as *const i8;
                __t0.err_msg = err_msg;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
}

fn initconfig_find_spec(
    mut spec: *const __SlateRecord_initconfig_c_99,
    name: *const i8,
) -> *const __SlateRecord_initconfig_c_99 {
    while !unsafe { (*spec).name }.is_null() {
        if (unsafe {
            strcmp(
                name as *const core::ffi::c_char,
                (*spec).name as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            return spec;
        }
        let __v1: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v1.wrapping_offset(1 as isize);
    }
    core::ptr::null::<__SlateRecord_initconfig_c_99>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_HasOption(config: *mut PyInitConfig, name: *const i8) -> i32 {
    let mut spec: *const __SlateRecord_initconfig_c_99 = initconfig_find_spec(
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 },
        name,
    );
    if spec.is_null() {
        spec = initconfig_find_spec(
            unsafe {
                core::ptr::addr_of!(PYPRECONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99
            },
            name,
        );
    }
    !spec.is_null() as i32
}

fn initconfig_prepare(
    config: *mut PyInitConfig,
    name: *const i8,
    raw_member: &mut *mut (),
) -> *const __SlateRecord_initconfig_c_99 {
    let mut spec: *const __SlateRecord_initconfig_c_99 = initconfig_find_spec(
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 },
        name,
    );
    if !spec.is_null() {
        unsafe {
            *(raw_member as *mut *mut ()) = config_get_spec_member(
                core::ptr::addr_of_mut!((*config).config) as *const PyConfig,
                spec,
            );
        }
        return spec;
    }
    spec = initconfig_find_spec(
        unsafe { core::ptr::addr_of!(PYPRECONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 },
        name,
    );
    if !spec.is_null() {
        unsafe {
            *(raw_member as *mut *mut ()) = preconfig_get_spec_member(
                core::ptr::addr_of_mut!((*config).preconfig) as *const PyPreConfig,
                spec,
            );
        }
        return spec;
    }
    initconfig_set_error(config, c"unknown config option name".as_ptr());
    core::ptr::null::<__SlateRecord_initconfig_c_99>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_GetInt(
    config: *mut PyInitConfig,
    name: *const i8,
    value: *mut i64,
) -> i32 {
    let mut raw_member: *mut () = unsafe { core::mem::zeroed() };
    let spec: *const __SlateRecord_initconfig_c_99 = initconfig_prepare(config, name, unsafe {
        &mut (*core::ptr::addr_of_mut!(raw_member))
    });
    if spec.is_null() {
        return 1i32.wrapping_neg();
    }
    match (unsafe { (*spec).r#type }) as u32 {
        0 | 1 | 3 => unsafe {
            *value = *(raw_member as *mut i32) as i64;
        },
        2 => {
            if 9223372036854775807 < unsafe { *(raw_member as *mut u64) } {
                initconfig_set_error(
                    config,
                    c"config option value doesn't fit into int64_t".as_ptr(),
                );
                return 1i32.wrapping_neg();
            }
            unsafe {
                *value = *(raw_member as *mut u64) as i64;
            }
        }
        _ => {
            initconfig_set_error(config, c"config option type is not int".as_ptr());
            return 1i32.wrapping_neg();
        }
    }
    0
}

fn wstr_to_utf8(config: *mut PyInitConfig, wstr: *mut i32) -> *mut i8 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut utf8: *mut i8 = unsafe { core::mem::zeroed() };
    let res: i32 = unsafe {
        _Py_EncodeUTF8Ex(
            wstr as *const i32,
            core::ptr::addr_of_mut!(utf8),
            core::ptr::null_mut::<u64>(),
            core::ptr::null_mut::<*const i8>(),
            1i32,
            1u32,
        )
    };
    if res == 2i32.wrapping_neg() {
        initconfig_set_error(config, c"encoding error".as_ptr());
        return core::ptr::null_mut::<i8>();
    }
    if res < 0 {
        unsafe {
            (*config).status = *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__16!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
        return core::ptr::null_mut::<i8>();
    }
    // Copy to use the malloc() memory allocator
    let size: u64 =
        unsafe { strlen((utf8 as *const i8) as *const core::ffi::c_char) as u64 }.wrapping_add(1);
    let str: *mut i8 = (unsafe { malloc(size as usize) as *mut () }) as *mut i8;
    if str.is_null() {
        unsafe { PyMem_RawFree(utf8 as *mut ()) };
        unsafe {
            (*config).status = *{
                __slate_compound_1 = {
                    let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t1._type = 1u32;
                    __t1.func = _PyStatus_GET_FUNC__16!() as *const i8;
                    __t1.err_msg = c"memory allocation failed".as_ptr();
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            };
        }
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((utf8 as *const ()) as *const core::ffi::c_void) as *const u8,
            ((str as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (size as usize) as usize,
        )
    };
    unsafe { PyMem_RawFree(utf8 as *mut ()) };
    str
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_GetStr(
    config: *mut PyInitConfig,
    name: *const i8,
    value: *mut *mut i8,
) -> i32 {
    let mut raw_member: *mut () = unsafe { core::mem::zeroed() };
    let spec: *const __SlateRecord_initconfig_c_99 = initconfig_prepare(config, name, unsafe {
        &mut (*core::ptr::addr_of_mut!(raw_member))
    });
    if spec.is_null() {
        return 1i32.wrapping_neg();
    }
    if ((unsafe { (*spec).r#type }) as u32) != 10 && ((unsafe { (*spec).r#type }) as u32) != 11 {
        initconfig_set_error(config, c"config option type is not string".as_ptr());
        return 1i32.wrapping_neg();
    }
    if unsafe { *(raw_member as *mut *mut i32) }.is_null() {
        unsafe {
            *value = core::ptr::null_mut::<i8>();
        }
        return 0;
    }
    unsafe {
        *value = wstr_to_utf8(config, *(raw_member as *mut *mut i32));
    }
    if unsafe { *value }.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_GetStrList(
    config: *mut PyInitConfig,
    name: *const i8,
    length: *mut u64,
    items: *mut *mut *mut i8,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut raw_member: *mut () = unsafe { core::mem::zeroed() };
    let spec: *const __SlateRecord_initconfig_c_99 = initconfig_prepare(config, name, unsafe {
        &mut (*core::ptr::addr_of_mut!(raw_member))
    });
    if spec.is_null() {
        return 1i32.wrapping_neg();
    }
    if ((unsafe { (*spec).r#type }) as u32) != 12 {
        initconfig_set_error(config, c"config option type is not string list".as_ptr());
        return 1i32.wrapping_neg();
    }
    unsafe {
        *length = (*(raw_member as *mut __SlateRecord_initconfig_h_36)).length as u64;
    }
    unsafe {
        *items = (malloc(
            ((*(raw_member as *mut __SlateRecord_initconfig_h_36)).length as u64)
                .wrapping_mul(core::mem::size_of::<*mut i8>() as u64) as usize,
        ) as *mut ()) as *mut *mut i8;
    }
    if unsafe { *items }.is_null() {
        unsafe {
            (*config).status = *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__71!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
        return 1i32.wrapping_neg();
    }
    let mut i: i64 = 0;
    while i < unsafe { (*(raw_member as *mut __SlateRecord_initconfig_h_36)).length } {
        unsafe {
            *unsafe { *items }.wrapping_offset(i as isize) = wstr_to_utf8(
                config,
                *unsafe { (*(raw_member as *mut __SlateRecord_initconfig_h_36)).items }
                    .wrapping_offset(i as isize),
            );
        }
        if unsafe { *unsafe { *items }.wrapping_offset(i as isize) }.is_null() {
            PyInitConfig_FreeStrList(i as u64, unsafe { *items });
            return 1i32.wrapping_neg();
        }
        i = i.wrapping_add(1);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_FreeStrList(length: u64, items: *mut *mut i8) {
    for i in 0..length {
        unsafe { free((*items.wrapping_offset(i as isize) as *mut ()) as *mut core::ffi::c_void) };
    }
    unsafe { free((items as *mut ()) as *mut core::ffi::c_void) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_SetInt(
    config: *mut PyInitConfig,
    name: *const i8,
    value: i64,
) -> i32 {
    let mut raw_member: *mut () = unsafe { core::mem::zeroed() };
    let spec: *const __SlateRecord_initconfig_c_99 = initconfig_prepare(config, name, unsafe {
        &mut (*core::ptr::addr_of_mut!(raw_member))
    });
    if spec.is_null() {
        return 1i32.wrapping_neg();
    }
    match (unsafe { (*spec).r#type }) as u32 {
        0 => {
            if value < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64) || 2147483647 < value {
                initconfig_set_error(config, c"config option value is out of int range".as_ptr());
                return 1i32.wrapping_neg();
            }
            let int_value: i32 = value as i32;
            unsafe {
                *(raw_member as *mut i32) = int_value;
            }
        }
        1 | 3 => {
            if value < 0 || 4294967295u64 < (value as u64) {
                initconfig_set_error(
                    config,
                    c"config option value is out of unsigned int range".as_ptr(),
                );
                return 1i32.wrapping_neg();
            }
            let int_value: i32 = value as i32;
            unsafe {
                *(raw_member as *mut i32) = int_value;
            }
        }
        2 => {
            if value < 0 || 18446744073709551615u64 < (value as u64) {
                initconfig_set_error(
                    config,
                    c"config option value is out of unsigned long range".as_ptr(),
                );
                return 1i32.wrapping_neg();
            }
            let ulong_value: u64 = value as u64;
            unsafe {
                *(raw_member as *mut u64) = ulong_value;
            }
        }
        _ => {
            initconfig_set_error(config, c"config option type is not int".as_ptr());
            return 1i32.wrapping_neg();
        }
    }
    if (unsafe {
        strcmp(
            name as *const core::ffi::c_char,
            c"hash_seed".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        unsafe {
            (*config).config.use_hash_seed = 1;
        }
    }
    0
}

fn utf8_to_wstr(config: *mut PyInitConfig, str: *const i8) -> *mut i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut wstr: *mut i32 = unsafe { core::mem::zeroed() };
    let mut wlen: u64 = 0;
    let res: i32 = unsafe {
        _Py_DecodeUTF8Ex(
            str,
            (strlen(str as *const core::ffi::c_char) as u64) as i64,
            core::ptr::addr_of_mut!(wstr),
            core::ptr::addr_of_mut!(wlen),
            core::ptr::null_mut::<*const i8>(),
            1u32,
        )
    };
    if res == 2i32.wrapping_neg() {
        initconfig_set_error(config, c"decoding error".as_ptr());
        return core::ptr::null_mut::<i32>();
    }
    if res < 0 {
        unsafe {
            (*config).status = *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__24!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
        return core::ptr::null_mut::<i32>();
    }
    // Copy to use the malloc() memory allocator
    let size: u64 = wlen
        .wrapping_add(1)
        .wrapping_mul(core::mem::size_of::<i32>() as u64);
    let wstr2: *mut i32 = (unsafe { malloc(size as usize) as *mut () }) as *mut i32;
    if wstr2.is_null() {
        unsafe { PyMem_RawFree(wstr as *mut ()) };
        unsafe {
            (*config).status = *{
                __slate_compound_1 = {
                    let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t1._type = 1u32;
                    __t1.func = _PyStatus_GET_FUNC__24!() as *const i8;
                    __t1.err_msg = c"memory allocation failed".as_ptr();
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            };
        }
        return core::ptr::null_mut::<i32>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((wstr as *const ()) as *const core::ffi::c_void) as *const u8,
            ((wstr2 as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (size as usize) as usize,
        )
    };
    unsafe { PyMem_RawFree(wstr as *mut ()) };
    wstr2
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_SetStr(
    config: *mut PyInitConfig,
    name: *const i8,
    value: *const i8,
) -> i32 {
    let mut raw_member: *mut () = unsafe { core::mem::zeroed() };
    let spec: *const __SlateRecord_initconfig_c_99 = initconfig_prepare(config, name, unsafe {
        &mut (*core::ptr::addr_of_mut!(raw_member))
    });
    if spec.is_null() {
        return 1i32.wrapping_neg();
    }
    if ((unsafe { (*spec).r#type }) as u32) != 10 && ((unsafe { (*spec).r#type }) as u32) != 11 {
        initconfig_set_error(config, c"config option type is not string".as_ptr());
        return 1i32.wrapping_neg();
    }
    if value.is_null() && ((unsafe { (*spec).r#type }) as u32) != 11 {
        initconfig_set_error(config, c"config option string cannot be NULL".as_ptr());
    }
    unsafe {
        *(raw_member as *mut *mut i32) = utf8_to_wstr(config, value);
    }
    if unsafe { *(raw_member as *mut *mut i32) }.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

fn initconfig_free_wstr(member: *mut i32) {
    if !member.is_null() {
        unsafe { free((member as *mut ()) as *mut core::ffi::c_void) };
    }
}

fn initconfig_free_wstr_list(list: *mut __SlateRecord_initconfig_h_36) {
    let mut i: i64 = 0;
    while i < unsafe { (*list).length } {
        unsafe {
            free(
                (*unsafe { (*list).items }.wrapping_offset(i as isize) as *mut ())
                    as *mut core::ffi::c_void,
            )
        };
        i = i.wrapping_add(1);
    }
    unsafe { free(((*list).items as *mut ()) as *mut core::ffi::c_void) };
}

fn initconfig_free_config(config: *const PyConfig) {
    let mut spec: *const __SlateRecord_initconfig_c_99 =
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 };
    while !unsafe { (*spec).name }.is_null() {
        let member: *mut () = config_get_spec_member(config, spec);
        if ((unsafe { (*spec).r#type }) as u32) == 10 || ((unsafe { (*spec).r#type }) as u32) == 11
        {
            let wstr: *mut i32 = unsafe { *(member as *mut *mut i32) };
            initconfig_free_wstr(wstr);
        } else {
            if ((unsafe { (*spec).r#type }) as u32) == 12 {
                initconfig_free_wstr_list(member as *mut __SlateRecord_initconfig_h_36);
            }
        }
        let __v1: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v1.wrapping_offset(1 as isize);
    }
}

fn initconfig_set_str_list(
    config: *mut PyInitConfig,
    list: *mut __SlateRecord_initconfig_h_36,
    length: i64,
    items: *const *mut i8,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut wlist: __SlateRecord_initconfig_h_36 = unsafe {
        *{
            __slate_compound_0 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    let size: u64 = (core::mem::size_of::<*mut i32>() as u64).wrapping_mul(length as u64);
    wlist.items = (unsafe { malloc(size as usize) as *mut () }) as *mut *mut i32;
    if wlist.items.is_null() {
        unsafe {
            (*config).status = *{
                __slate_compound_1 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__81!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            };
        }
        return 1i32.wrapping_neg();
    }
    for i in 0..length {
        let arg: *mut i32 = utf8_to_wstr(
            config,
            (unsafe { *items.wrapping_offset(i as isize) }) as *const i8,
        );
        if arg.is_null() {
            initconfig_free_wstr_list(core::ptr::addr_of_mut!(wlist));
            return 1i32.wrapping_neg();
        }
        unsafe {
            *wlist.items.wrapping_offset(i as isize) = arg;
        }
        let __v2: i64 = wlist.length;
        wlist.length = __v2.wrapping_add(1);
    }
    initconfig_free_wstr_list(list);
    unsafe {
        *list = wlist;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_SetStrList(
    config: *mut PyInitConfig,
    name: *const i8,
    length: u64,
    items: *const *mut i8,
) -> i32 {
    let mut raw_member: *mut () = unsafe { core::mem::zeroed() };
    let spec: *const __SlateRecord_initconfig_c_99 = initconfig_prepare(config, name, unsafe {
        &mut (*core::ptr::addr_of_mut!(raw_member))
    });
    if spec.is_null() {
        return 1i32.wrapping_neg();
    }
    if ((unsafe { (*spec).r#type }) as u32) != 12 {
        initconfig_set_error(config, c"config option type is not strings list".as_ptr());
        return 1i32.wrapping_neg();
    }
    let list: *mut __SlateRecord_initconfig_h_36 = raw_member as *mut __SlateRecord_initconfig_h_36;
    if initconfig_set_str_list(config, list, length as i64, items) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        strcmp(
            name as *const core::ffi::c_char,
            c"module_search_paths".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        unsafe {
            (*config).config.module_search_paths_set = 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyInitConfig_AddModule(
    config: *mut PyInitConfig,
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let size: u64 = (core::mem::size_of::<_inittab>() as u64)
        .wrapping_mul(unsafe { (*config).inittab_size }.wrapping_add(2) as u64);
    let new_inittab: *mut _inittab =
        (unsafe { PyMem_RawRealloc((*config).inittab as *mut (), size) }) as *mut _inittab;
    if new_inittab.is_null() {
        unsafe {
            (*config).status = *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__38!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*config).inittab = new_inittab;
    }
    let mut entry: *mut _inittab =
        unsafe { (*config).inittab }.wrapping_offset((unsafe { (*config).inittab_size }) as isize);
    unsafe {
        (*entry).name = name;
    }
    unsafe {
        (*entry).initfunc = initfunc;
    }
    // Terminator entry
    entry = unsafe { (*config).inittab }
        .wrapping_offset(unsafe { (*config).inittab_size }.wrapping_add(1) as isize);
    unsafe {
        (*entry).name = core::ptr::null::<i8>();
    }
    unsafe {
        (*entry).initfunc = None;
    }
    let __v2: i64 = unsafe { (*config).inittab_size };
    unsafe {
        (*config).inittab_size = __v2.wrapping_add(1);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_InitializeFromInitConfig(config: *mut PyInitConfig) -> i32 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if (unsafe { (*config).inittab_size }) >= 1 {
        if (unsafe { PyImport_ExtendInittab((*config).inittab) }) < 0 {
            unsafe {
                (*config).status = *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__47!() as *const i8;
                        __t0.err_msg = c"memory allocation failed".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                };
            }
            return 1i32.wrapping_neg();
        }
    }
    unsafe {
        _PyPreConfig_GetConfig(
            core::ptr::addr_of_mut!((*config).preconfig),
            core::ptr::addr_of_mut!((*config).config) as *const PyConfig,
        )
    };
    unsafe {
        (*config).status = Py_PreInitializeFromArgs(
            core::ptr::addr_of_mut!((*config).preconfig) as *const PyPreConfig,
            (*config).config.argv.length,
            (*config).config.argv.items,
        );
    }
    if _PyStatus_EXCEPTION__2!((*config).status) {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*config).status =
            Py_InitializeFromConfig(core::ptr::addr_of_mut!((*config).config) as *const PyConfig);
    }
    if _PyStatus_EXCEPTION__2!((*config).status) {
        return 1i32.wrapping_neg();
    }
    0
}

// --- PyConfig_Get() -------------------------------------------------------
fn config_generic_find_spec(
    mut spec: *const __SlateRecord_initconfig_c_99,
    name: *const i8,
) -> *const __SlateRecord_initconfig_c_99 {
    while !unsafe { (*spec).name }.is_null() {
        if ((unsafe { (*spec).visibility }) as u32) != 0 {
            if (unsafe {
                strcmp(
                    name as *const core::ffi::c_char,
                    (*spec).name as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                return spec;
            }
        }
        let __v1: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v1.wrapping_offset(1 as isize);
    }
    core::ptr::null::<__SlateRecord_initconfig_c_99>()
}

fn config_find_spec(name: *const i8) -> *const __SlateRecord_initconfig_c_99 {
    config_generic_find_spec(
        unsafe { core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 },
        name,
    )
}

fn preconfig_find_spec(name: *const i8) -> *const __SlateRecord_initconfig_c_99 {
    config_generic_find_spec(
        unsafe { core::ptr::addr_of!(PYPRECONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99 },
        name,
    )
}

fn config_add_xoption(dict: *mut _object, str: *const i32) -> i32 {
    let mut name_end: *const i32 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            name = core::ptr::null_mut::<_object>();
            value = core::ptr::null_mut::<_object>();
            name_end = wcschr(str, 61i32) as *const i32;
            if name_end.is_null() {
                name = PyUnicode_FromWideChar(str, 1i32.wrapping_neg() as i64);
                if name.is_null() {
                    break '__join_0;
                } else {
                    value = Py_NewRef!(_PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)));
                }
            } else {
                name = PyUnicode_FromWideChar(str, name_end.offset_from(str as *const i32) as i64);
                if name.is_null() {
                    break '__join_0;
                } else {
                    value = PyUnicode_FromWideChar(
                        name_end.wrapping_offset(1 as isize),
                        1i32.wrapping_neg() as i64,
                    );
                    if value.is_null() {
                        break '__join_0;
                    }
                }
            }
            if PyDict_SetItem(dict, name, value) >= 0 {
                Py_DECREF!(name);
                Py_DECREF!(value);
                return 0;
            }
        }
        Py_XDECREF!(name);
        Py_XDECREF!(value);
        return 1i32.wrapping_neg();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_CreateXOptionsDict(config: *const PyConfig) -> *mut _object {
    let dict: *mut _object = unsafe { PyDict_New() };
    if dict.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let nxoption: i64 = unsafe { (*config).xoptions.length };
    let xoptions: *mut *mut i32 = unsafe { (*config).xoptions.items };
    for i in 0..nxoption {
        let option: *const i32 = (unsafe { *xoptions.wrapping_offset(i as isize) }) as *const i32;
        if config_add_xoption(dict, option) < 0 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
    }
    dict
}

fn config_get_sys_write_bytecode(config: *const PyConfig, value: &mut i32) -> i32 {
    let attr: *mut _object =
        unsafe { _PySys_GetRequiredAttrString(c"dont_write_bytecode".as_ptr()) };
    if attr.is_null() {
        return 1i32.wrapping_neg();
    }
    let is_true: i32 = unsafe { PyObject_IsTrue(attr) };
    Py_DECREF!(attr);
    if is_true < 0 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *(value as *mut i32) = (is_true == 0) as i32;
    }
    0
}

/// End of ignoring deprecations on global variables
/// Forward declarations
fn config_get(
    config: *const PyConfig,
    spec: *const __SlateRecord_initconfig_c_99,
    use_sys: i32,
) -> *mut _object {
    if use_sys != 0 {
        if !unsafe { (*spec).sys.attr }.is_null() {
            return unsafe { _PySys_GetRequiredAttrString((*spec).sys.attr) };
        }
        if (unsafe {
            strcmp(
                (*spec).name as *const core::ffi::c_char,
                c"write_bytecode".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            let mut value: i32 = 0;
            if config_get_sys_write_bytecode(config, unsafe {
                &mut (*core::ptr::addr_of_mut!(value))
            }) < 0
            {
                return core::ptr::null_mut::<_object>();
            }
            return unsafe { PyBool_FromLong(value as i64) };
        }
        if (unsafe {
            strcmp(
                (*spec).name as *const core::ffi::c_char,
                c"int_max_str_digits".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            let interp: *mut _is = _PyInterpreterState_GET();
            let maxdigits: i32 = _Py_atomic_load_int(
                (unsafe { core::ptr::addr_of_mut!((*interp).long_state.max_str_digits) })
                    as *const i32,
            );
            return unsafe { PyLong_FromLong(maxdigits as i64) };
        }
    }
    let member: *mut () = config_get_spec_member(config, spec);
    match (unsafe { (*spec).r#type }) as u32 {
        0 | 1 => {
            let value: i32 = unsafe { *(member as *mut i32) };
            return unsafe { PyLong_FromLong(value as i64) };
        }
        3 => {
            let value: i32 = unsafe { *(member as *mut i32) };
            return unsafe { PyBool_FromLong((value != 0) as i64) };
        }
        2 => {
            let value: u64 = unsafe { *(member as *mut u64) };
            return unsafe { PyLong_FromUnsignedLong(value) };
        }
        10 | 11 => {
            let wstr: *mut i32 = unsafe { *(member as *mut *mut i32) };
            if !wstr.is_null() {
                return unsafe {
                    PyUnicode_FromWideChar(wstr as *const i32, 1i32.wrapping_neg() as i64)
                };
            } else {
                return Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
            }
        }
        12 => {
            if (unsafe {
                strcmp(
                    (*spec).name as *const core::ffi::c_char,
                    c"xoptions".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                return _PyConfig_CreateXOptionsDict(config);
            } else {
                let list: *const __SlateRecord_initconfig_h_36 =
                    member as *const __SlateRecord_initconfig_h_36;
                return _PyWideStringList_AsTuple(list);
            }
        }
        _ => {
            Py_UNREACHABLE!();
        }
    }
    unsafe { core::mem::zeroed() }
}

fn preconfig_get(
    preconfig: *const PyPreConfig,
    spec: *const __SlateRecord_initconfig_c_99,
) -> *mut _object {
    // The type of all PYPRECONFIG_SPEC members is INT or BOOL.
    let member: *mut i8 =
        (preconfig as *mut i8).wrapping_offset((unsafe { (*spec).offset }) as isize);
    let value: i32 = unsafe { *(member as *mut i32) };
    if ((unsafe { (*spec).r#type }) as u32) == 3 {
        return unsafe { PyBool_FromLong((value != 0) as i64) };
    } else {
        return unsafe { PyLong_FromLong(value as i64) };
    }
}

fn config_unknown_name_error(name: *const i8) {
    unsafe {
        PyErr_Format(
            PyExc_ValueError,
            c"unknown config option name: %s".as_ptr(),
            name,
        )
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_Get(name: *const i8) -> *mut _object {
    let mut spec: *const __SlateRecord_initconfig_c_99 = config_find_spec(name);
    if !spec.is_null() {
        let config: *const PyConfig = unsafe { _Py_GetConfig() };
        return config_get(config, spec, 1i32);
    }
    spec = preconfig_find_spec(name);
    if !spec.is_null() {
        let preconfig: *const PyPreConfig =
            (unsafe { core::ptr::addr_of_mut!(_PyRuntime.preconfig) }) as *const PyPreConfig;
        return preconfig_get(preconfig, spec);
    }
    config_unknown_name_error(name);
    core::ptr::null_mut::<_object>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_GetInt(name: *const i8, value: *mut i32) -> i32 {
    let obj: *mut _object = PyConfig_Get(name);
    if obj.is_null() {
        return 1i32.wrapping_neg();
    }
    if PyLong_Check!(obj) == 0 {
        Py_DECREF!(obj);
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"config option %s is not an int".as_ptr(),
                name,
            )
        };
        return 1i32.wrapping_neg();
    }
    let as_int: i32 = unsafe { PyLong_AsInt(obj) };
    Py_DECREF!(obj);
    let __v0: bool = as_int == 1i32.wrapping_neg() && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        unsafe {
            PyErr_Format(
                PyExc_OverflowError,
                c"config option %s value does not fit into a C int".as_ptr(),
                name,
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe {
        *value = as_int;
    }
    0
}

fn config_names_add(names: *mut _object, mut spec: *const __SlateRecord_initconfig_c_99) -> i32 {
    while !unsafe { (*spec).name }.is_null() {
        if ((unsafe { (*spec).visibility }) as u32) != 0 {
            let name: *mut _object = unsafe { PyUnicode_FromString((*spec).name) };
            if name.is_null() {
                return 1i32.wrapping_neg();
            }
            let res: i32 = unsafe { PyList_Append(names, name) };
            Py_DECREF!(name);
            if res < 0 {
                return 1i32.wrapping_neg();
            }
        }
        let __v1: *const __SlateRecord_initconfig_c_99 = spec;
        spec = __v1.wrapping_offset(1 as isize);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_Names() -> *mut _object {
    let mut names: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        names = PyList_New(0i64);
        if !names.is_null() {
            if config_names_add(
                names,
                core::ptr::addr_of!(PYCONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99,
            ) >= 0
            {
                if config_names_add(
                    names,
                    core::ptr::addr_of!(PYPRECONFIG_SPEC.0) as *const __SlateRecord_initconfig_c_99,
                ) >= 0
                {
                    let frozen: *mut _object = PyFrozenSet_New(names);
                    Py_DECREF!(names);
                    return frozen;
                }
            }
        }
        Py_XDECREF!(names);
        return core::ptr::null_mut::<_object>();
    }
}

// --- PyConfig_Set() -------------------------------------------------------
fn config_set_sys_flag(spec: *const __SlateRecord_initconfig_c_99, mut int_value: i32) -> i32 {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut config: *mut PyConfig = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    unsafe {
        interp = _PyInterpreterState_GET();
        config = core::ptr::addr_of_mut!((*interp).config);
        if ((*spec).r#type as u32) == 3 {
            if int_value != 0 {
                // convert values < 0 and values > 1 to 1
                int_value = 1;
            }
        }
        if (*spec).sys.flag_setter != None {
            value = unsafe { (*spec).sys.flag_setter }.unwrap()(int_value);
        } else {
            value = config_sys_flag_long(int_value);
        }
        if value.is_null() {
            return 1i32.wrapping_neg();
        } else {
            '__join_0: {
                // Set sys.flags.FLAG
                let pos: i64 = (*spec).sys.flag_index as i64;
                if _PySys_SetFlagObj(pos, value) >= 0 {
                    // Set PyConfig.ATTR
                    let member: *mut i32 =
                        config_get_spec_member(config as *const PyConfig, spec) as *mut i32;
                    *member = int_value;
                    // Set sys.dont_write_bytecode attribute
                    if (strcmp(
                        (*spec).name as *const core::ffi::c_char,
                        c"write_bytecode".as_ptr() as *const core::ffi::c_char,
                    ) as i32)
                        == 0
                    {
                        if PySys_SetObject(c"dont_write_bytecode".as_ptr(), value) < 0 {
                            break '__join_0;
                        }
                    }
                    Py_DECREF!(value);
                    return 0;
                }
            }
            Py_DECREF!(value);
            return 1i32.wrapping_neg();
        }
    }
}

/// Set PyConfig.ATTR integer member
fn config_set_int_attr(spec: *const __SlateRecord_initconfig_c_99, value: i32) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    let config: *mut PyConfig = unsafe { core::ptr::addr_of_mut!((*interp).config) };
    let member: *mut i32 = config_get_spec_member(config as *const PyConfig, spec) as *mut i32;
    unsafe {
        *member = value;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyConfig_Set(name: *const i8, value: *mut _object) -> i32 {
    let mut value__1: i32 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut item_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut __v9: bool = unsafe { core::mem::zeroed() };
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    let mut has_int_value: i32 = unsafe { core::mem::zeroed() };
    let mut int_value: i32 = unsafe { core::mem::zeroed() };
    let mut spec: *const __SlateRecord_initconfig_c_99 = unsafe { core::mem::zeroed() };
    unsafe {
        if PySys_Audit(
            c"cpython.PyConfig_Set".as_ptr(),
            c"sO".as_ptr(),
            name,
            value,
        ) < 0
        {
            return 1i32.wrapping_neg();
        } else {
            spec = config_find_spec(name);
            if spec.is_null() {
                spec = preconfig_find_spec(name);
                if spec.is_null() {
                    config_unknown_name_error(name);
                    return 1i32.wrapping_neg();
                }
            }
            if ((*spec).visibility as u32) != 2 {
                PyErr_Format(
                    PyExc_ValueError,
                    c"cannot set read-only option %s".as_ptr(),
                    name,
                );
                return 1i32.wrapping_neg();
            } else {
                '__join_0: {
                    '__join_12: {
                        int_value = 0;
                        has_int_value = 0;
                        let __t0: u32 = (*spec).r#type as u32;
                        if __t0 != 0 {
                            if __t0 != 1 {
                                if __t0 != 3 {
                                    if __t0 == 2 {
                                        break '__join_0;
                                    } else {
                                        if __t0 == 10 {
                                            if PyUnicode_CheckExact!(value) == 0 {
                                                PyErr_Format(
                                                    PyExc_TypeError,
                                                    c"expected str, got %T".as_ptr(),
                                                    value,
                                                );
                                                return 1i32.wrapping_neg();
                                            } else {
                                                break '__join_12;
                                            }
                                        } else {
                                            if __t0 == 11 {
                                                if value != core::ptr::addr_of_mut!(_Py_NoneStruct)
                                                {
                                                    __v9 = PyUnicode_CheckExact!(value) == 0;
                                                } else {
                                                    __v9 = false;
                                                }
                                                if __v9 {
                                                    PyErr_Format(
                                                        PyExc_TypeError,
                                                        c"expected str or None, got %T".as_ptr(),
                                                        value,
                                                    );
                                                    return 1i32.wrapping_neg();
                                                } else {
                                                    break '__join_12;
                                                }
                                            } else {
                                                if __t0 == 12 {
                                                    if (strcmp(
                                                        (*spec).name as *const core::ffi::c_char,
                                                        c"xoptions".as_ptr()
                                                            as *const core::ffi::c_char,
                                                    )
                                                        as i32)
                                                        != 0
                                                    {
                                                        if PyList_Check!(value) == 0 {
                                                            PyErr_Format(
                                                                PyExc_TypeError,
                                                                c"expected list[str], got %T"
                                                                    .as_ptr(),
                                                                value,
                                                            );
                                                            return 1i32.wrapping_neg();
                                                        } else {
                                                            i = 0;
                                                            loop {
                                                                if i < PyList_GET_SIZE!(value) {
                                                                    item =
                                                                        PyList_GET_ITEM!(value, i);
                                                                    if PyUnicode_Check!(item) == 0 {
                                                                        break;
                                                                    } else {
                                                                        i = i.wrapping_add(1);
                                                                    }
                                                                } else {
                                                                    break '__join_12;
                                                                }
                                                            }
                                                            PyErr_Format(PyExc_TypeError, c"expected str, list item %zd has type %T".as_ptr(), i, item);
                                                            return 1i32.wrapping_neg();
                                                        }
                                                    } else {
                                                        // xoptions type is dict[str, str]
                                                        if PyDict_Check!(value) == 0 {
                                                            PyErr_Format(PyExc_TypeError, c"expected dict[str, str | bool], got %T".as_ptr(), value);
                                                            return 1i32.wrapping_neg();
                                                        } else {
                                                            pos = 0;
                                                            loop {
                                                                if PyDict_Next(
                                                                    value,
                                                                    core::ptr::addr_of_mut!(pos),
                                                                    core::ptr::addr_of_mut!(key),
                                                                    core::ptr::addr_of_mut!(item_1),
                                                                ) != 0
                                                                {
                                                                    if PyUnicode_Check!(key) == 0 {
                                                                        PyErr_Format(PyExc_TypeError, c"expected str, got dict key type %T".as_ptr(), key);
                                                                        return 1i32.wrapping_neg();
                                                                    } else {
                                                                        if PyUnicode_Check!(item_1)
                                                                            == 0
                                                                        {
                                                                            __v5 = PyBool_Check!(
                                                                                item_1
                                                                            ) == 0;
                                                                        } else {
                                                                            __v5 = false;
                                                                        }
                                                                        if __v5 {
                                                                            break;
                                                                        }
                                                                    }
                                                                } else {
                                                                    break '__join_12;
                                                                }
                                                            }
                                                            PyErr_Format(PyExc_TypeError, c"expected str or bool, got dict value type %T".as_ptr(), key);
                                                            return 1i32.wrapping_neg();
                                                        }
                                                    }
                                                } else {
                                                    Py_UNREACHABLE!();
                                                    break '__join_12;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if PyLong_Check!(value) == 0 {
                            PyErr_Format(
                                PyExc_TypeError,
                                c"expected int or bool, got %T".as_ptr(),
                                value,
                            );
                            return 1i32.wrapping_neg();
                        } else {
                            int_value = PyLong_AsInt(value);
                            if int_value == 1i32.wrapping_neg() {
                                __v10 = !PyErr_Occurred().is_null();
                            } else {
                                __v10 = false;
                            }
                            if __v10 {
                                return 1i32.wrapping_neg();
                            } else {
                                if int_value < 0 && ((*spec).r#type as u32) != 0 {
                                    PyErr_Format(PyExc_ValueError, c"value must be >= 0".as_ptr());
                                    return 1i32.wrapping_neg();
                                } else {
                                    has_int_value = 1;
                                }
                            }
                        }
                    }
                    // Set the global variable
                    if !unsafe { (*spec).global_var.ptr }.is_null() {
                        value__1 = int_value;
                        if (*spec).global_var.not != 0 {
                            value__1 = (value__1 == 0) as i32;
                        }
                        *(*spec).global_var.ptr = value__1;
                    }
                    if !unsafe { (*spec).sys.attr }.is_null() {
                        // Set the sys attribute, but don't set PyInterpreterState.config
                        //
                        // to keep the code simple.
                        return PySys_SetObject((*spec).sys.attr, value);
                    } else {
                        if has_int_value != 0 {
                            if (*spec).sys.flag_index >= 0 {
                                return config_set_sys_flag(spec, int_value);
                            } else {
                                if (strcmp(
                                    (*spec).name as *const core::ffi::c_char,
                                    c"int_max_str_digits".as_ptr() as *const core::ffi::c_char,
                                ) as i32)
                                    == 0
                                {
                                    return _PySys_SetIntMaxStrDigits(int_value);
                                } else {
                                    return config_set_int_attr(spec, int_value);
                                }
                            }
                        }
                    }
                }
                PyErr_Format(PyExc_ValueError, c"cannot set option %s".as_ptr(), name);
                return 1i32.wrapping_neg();
            }
        }
    }
    // not implemented: only hash_seed uses this type, and it's read-only
}
