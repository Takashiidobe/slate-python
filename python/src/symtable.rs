//! The size of 'void *', as computed by sizeof.
//! Round up size "n" to be a multiple of "a".
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
//! stmt_ty
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! Total tool ids available
// Signed variants of the above
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! ENTER_RECURSIVE {
    () => {{
        if (unsafe { Py_EnterRecursiveCall(c" during compilation".as_ptr()) }) != 0 {
            return 0i32;
        }
    }};
}

macro_rules! IS_ASYNC_DEF {
    ($st:expr) => {
        ((unsafe { (*(*$st).st_cur).ste_type }) as u32) == 0
            && ((unsafe { (*(*$st).st_cur).__slate_bits_0.__get_ste_coroutine() }) as i32) != 0
    };
}

macro_rules! LEAVE_RECURSIVE {
    () => {
        unsafe { Py_LeaveRecursiveCall() };
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

macro_rules! PyObject_New {
    ($type:ty, $typeobj:expr) => {
        (unsafe { _PyObject_New($typeobj) }) as *mut $type
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

macro_rules! SET_ERROR_LOCATION {
    ($FNAME:expr, $L:expr) => {
        unsafe {
            PyErr_RangedSyntaxLocationObject(
                $FNAME,
                $L.lineno,
                $L.col_offset.wrapping_add(1),
                $L.end_lineno,
                $L.end_col_offset.wrapping_add(1),
            )
        };
    };
}

macro_rules! SET_SCOPE {
    ($DICT:expr, $NAME:expr, $I:expr) => {{
        loop {
            let o: *mut _object = unsafe { PyLong_FromLong(($I) as i64) };
            if o.is_null() {
                return 0i32;
            }
            if (unsafe { PyDict_SetItem($DICT, $NAME, o) }) < 0 {
                Py_DECREF!(o);
                return 0i32;
            }
            Py_DECREF!(o);
            if true {
                break;
            }
        }
    }};
}

macro_rules! SYMBOL_TO_SCOPE {
    ($S:expr) => {
        ($S) >> 12i32 & ((1 | 2 | 4i32 | 8i32) as i64)
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
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

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

macro_rules! asdl_seq_GET__9 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _alias).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__10 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _excepthandler)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__7 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _type_param)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__2 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _stmt).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__3 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _pattern)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__13 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _match_case)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__11 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _withitem)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__6 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _arg).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _expr).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__12 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _object).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__5 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _comprehension)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__4 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _keyword)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_LEN__14 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__4 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__9 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__8 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__7 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__10 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__6 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__11 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__12 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__5 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__2 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

unsafe extern "C" {
    static mut PyType_Type: _typeobject;
    static mut PyExc_KeyError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_SyntaxError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Free(arg0: *mut ());
    fn _PyObject_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyUnicode_Substring(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_FindChar(arg0: *mut _object, arg1: u32, arg2: i64, arg3: i64, arg4: i32) -> i64;
    fn PyUnicode_Compare(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyUnicodeWriter_Create(arg0: i64) -> *mut PyUnicodeWriter;
    fn PyUnicodeWriter_Discard(arg0: *mut PyUnicodeWriter);
    fn PyUnicodeWriter_Finish(arg0: *mut PyUnicodeWriter) -> *mut _object;
    fn PyUnicodeWriter_WriteChar(arg0: *mut PyUnicodeWriter, arg1: u32) -> i32;
    fn PyUnicodeWriter_WriteStr(arg0: *mut PyUnicodeWriter, arg1: *mut _object) -> i32;
    fn PyUnicodeWriter_WriteSubstring(
        arg0: *mut PyUnicodeWriter,
        arg1: *mut _object,
        arg2: i64,
        arg3: i64,
    ) -> i32;
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyErr_RangedSyntaxLocationObject(
        arg0: *mut _object,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
    );
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_FromVoidPtr(arg0: *mut ()) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyList_SetSlice(arg0: *mut _object, arg1: i64, arg2: i64, arg3: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_GetItemWithError(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_DelItemString(arg0: *mut _object, arg1: *const i8) -> i32;
    fn PyDict_GetItemRef(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PySet_New(arg0: *mut _object) -> *mut _object;
    fn PySet_Add(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PySet_Contains(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PySet_Discard(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn Py_EnterRecursiveCall(arg0: *const i8) -> i32;
    fn Py_LeaveRecursiveCall();
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyNumber_InPlaceOr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyArena_New() -> *mut _arena;
    fn _PyArena_Free(arg0: *mut _arena);
    fn _PyAST_GetDocString(arg0: *mut __SlateRecord_pycore_ast_h_67) -> *mut _object;
    fn _PyParser_ASTFromString(
        arg0: *const i8,
        arg1: *mut _object,
        arg2: i32,
        arg3: *mut __SlateRecord_compile_h_30,
        arg4: *mut _arena,
    ) -> *mut _mod;
    fn _PyFuture_FromAST(
        arg0: *mut _mod,
        arg1: *mut _object,
        arg2: *mut __SlateRecord_pycore_symtable_h_67,
    ) -> i32;
    fn _PyUnicode_EqualToASCIIString(arg0: *mut _object, arg1: *const i8) -> i32;
    fn _PyUnicode_Equal(arg0: *mut _object, arg1: *mut _object) -> i32;
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
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits137U0,
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
struct __SlateRecord_tupleobject_h_13 {
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
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
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits189U0,
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
struct _arena {}

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
struct _mod {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_165,
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
struct _type_ignore {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_656,
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
struct _Py_global_strings {
    literals: __SlateRecord_pycore_global_strings_h_32,
    identifiers: __SlateRecord_pycore_global_strings_h_58,
    ascii: [__SlateRecord_pycore_global_strings_h_815; 128],
    latin1: [__SlateRecord_pycore_global_strings_h_819; 128],
}

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
struct llist_node {
    next: *mut llist_node,
    prev: *mut llist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct code_arena_st {}

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
struct _pending_call {
    func: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
    flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_crossinterp_h_255 {
    data_lookup: _xid_lookup_state,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct _obmalloc_usage {
    arena_map_root: arena_map_top,
    arena_map_mid_count: i32,
    arena_map_bot_count: i32,
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
    pub struct __SlateBits137U0 {
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
    pub struct __SlateBits189U0 {
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
    pub struct __SlateBits1394U0 {
        #[bits(1)]
        pub ste_generator: u32,
        #[bits(1)]
        pub ste_coroutine: u32,
        #[bits(1)]
        pub ste_annotations_used: u32,
        #[bits(5, access = na)]
        pub __slate_pad_3: u8,
    }
    #[bitfields::bitfield(
        u16,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits1394U1 {
        #[bits(1)]
        pub ste_varargs: u32,
        #[bits(1)]
        pub ste_varkeywords: u32,
        #[bits(1)]
        pub ste_returns_value: u32,
        #[bits(1)]
        pub ste_needs_class_closure: u32,
        #[bits(1)]
        pub ste_needs_classdict: u32,
        #[bits(1)]
        pub ste_comp_inlined: u32,
        #[bits(1)]
        pub ste_comp_iter_target: u32,
        #[bits(1)]
        pub ste_can_see_class_scope: u32,
        #[bits(1)]
        pub ste_has_docstring: u32,
        #[bits(1)]
        pub ste_method: u32,
        #[bits(1)]
        pub ste_has_conditional_annotations: u32,
        #[bits(1)]
        pub ste_in_conditional_block: u32,
        #[bits(1)]
        pub ste_in_unevaluated_annotation: u32,
        #[bits(3, access = na)]
        pub __slate_pad_13: u8,
    }
}

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

use slate_support::free;

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
struct __SlateRecord_pycore_symtable_h_51 {
    lineno: i32,
    end_lineno: i32,
    col_offset: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_symtable_h_67 {
    ff_features: i32,
    ff_location: __SlateRecord_pycore_symtable_h_51,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct symtable {
    st_filename: *mut _object,
    st_cur: *mut _symtable_entry,
    st_top: *mut _symtable_entry,
    st_blocks: *mut _object,
    st_stack: *mut _object,
    st_global: *mut _object,
    st_nblocks: i32,
    st_private: *mut _object,
    st_future: *mut __SlateRecord_pycore_symtable_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _symtable_entry {
    ob_base: _object,
    ste_id: *mut _object,
    ste_symbols: *mut _object,
    ste_name: *mut _object,
    ste_varnames: *mut _object,
    ste_children: *mut _object,
    ste_directives: *mut _object,
    ste_mangled_names: *mut _object,
    ste_type: u32,
    ste_scope_info: *const i8,
    ste_nested: i32,
    __slate_bits_0: __slate_bits::__SlateBits1394U0,
    ste_comprehension: u32,
    __slate_bits_1: __slate_bits::__SlateBits1394U1,
    ste_comp_iter_expr: i32,
    ste_loc: __SlateRecord_pycore_symtable_h_51,
    ste_annotation_block: *mut _symtable_entry,
    ste_table: *mut symtable,
}

#[unsafe(no_mangle)]
static mut PySTEntry_Type: _typeobject = {
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
    __t0.tp_name = (b"symtable entry\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<_symtable_entry>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(ste_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(ste_repr);
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
    __t0.tp_flags = 0u64;
    __t0.tp_doc = core::ptr::null::<i8>();
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(ste_memberlist.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

// _PyParser_ASTFromString()
// _PyThreadState_GET()
// _Py_ID()
// PySTEntryObject
// global stmt
// assignment in code block
// formal parameter
// nonlocal stmt
// name is used
// free variable from class's method
// assignment occurred via import
// this name is annotated
// this name is a comprehension iteration variable
// this name is a type parameter
// this name is a cell in an inlined comprehension
// GLOBAL_EXPLICIT and GLOBAL_IMPLICIT are used internally by the symbol
// table.  GLOBAL is returned from _PyST_GetScope() for either of them.
// It is stored in ste_symbols at bits 13-16.
// _PyUnicode_EqualToASCIIString
// offsetof()
// Set this to 1 to dump all symtables to stdout for debugging
// error strings used for warnings
fn ste_new(
    st: *mut symtable,
    name: *mut _object,
    block: u32,
    key: *mut (),
    loc: __SlateRecord_pycore_symtable_h_51,
) -> *mut _symtable_entry {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut k: *mut _object = unsafe { core::mem::zeroed() };
    let mut ste: *mut _symtable_entry = unsafe { core::mem::zeroed() };
    unsafe {
        ste = core::ptr::null_mut::<_symtable_entry>();
        k = core::ptr::null_mut::<_object>();
        k = PyLong_FromVoidPtr(key);
        if !k.is_null() {
            ste = PyObject_New!(_symtable_entry, core::ptr::addr_of_mut!(PySTEntry_Type));
            if ste.is_null() {
                Py_DECREF!(k);
            } else {
                (*ste).ste_table = st;
                (*ste).ste_id = k;
                // ste owns reference to k
                (*ste).ste_name = Py_NewRef!(name);
                (*ste).ste_symbols = core::ptr::null_mut::<_object>();
                (*ste).ste_varnames = core::ptr::null_mut::<_object>();
                (*ste).ste_children = core::ptr::null_mut::<_object>();
                (*ste).ste_directives = core::ptr::null_mut::<_object>();
                (*ste).ste_mangled_names = core::ptr::null_mut::<_object>();
                (*ste).ste_type = block;
                (*ste).ste_scope_info = core::ptr::null::<i8>();
                (*ste).ste_nested = 0;
                (*ste).__slate_bits_1.__set_ste_varargs(0u32);
                (*ste).__slate_bits_1.__set_ste_varkeywords(0u32);
                (*ste).__slate_bits_0.__set_ste_annotations_used(0u32);
                (*ste).ste_loc = loc;
                if !unsafe { (*st).st_cur }.is_null() {
                    if (*(*st).st_cur).ste_nested != 0 {
                        __v0 = true;
                    } else {
                        __v0 = _PyST_IsFunctionLike((*st).st_cur) != 0;
                    }
                    __v1 = __v0;
                } else {
                    __v1 = false;
                }
                if __v1 {
                    (*ste).ste_nested = 1;
                }
                (*ste).__slate_bits_0.__set_ste_generator(0u32);
                (*ste).__slate_bits_0.__set_ste_coroutine(0u32);
                (*ste).ste_comprehension = 0;
                (*ste).__slate_bits_1.__set_ste_returns_value(0u32);
                (*ste).__slate_bits_1.__set_ste_needs_class_closure(0u32);
                (*ste).__slate_bits_1.__set_ste_comp_inlined(0u32);
                (*ste).__slate_bits_1.__set_ste_comp_iter_target(0u32);
                (*ste).__slate_bits_1.__set_ste_can_see_class_scope(0u32);
                (*ste).ste_comp_iter_expr = 0;
                (*ste).__slate_bits_1.__set_ste_needs_classdict(0u32);
                (*ste)
                    .__slate_bits_1
                    .__set_ste_has_conditional_annotations(0u32);
                (*ste).__slate_bits_1.__set_ste_in_conditional_block(0u32);
                (*ste)
                    .__slate_bits_1
                    .__set_ste_in_unevaluated_annotation(0u32);
                (*ste).ste_annotation_block = core::ptr::null_mut::<_symtable_entry>();
                (*ste).__slate_bits_1.__set_ste_has_docstring(0u32);
                (*ste).__slate_bits_1.__set_ste_method(0u32);
                if !unsafe { (*st).st_cur }.is_null()
                    && ((*(*st).st_cur).ste_type as u32) == 1
                    && (block as u32) == 0
                {
                    (*ste).__slate_bits_1.__set_ste_method(1u32);
                }
                (*ste).ste_symbols = PyDict_New();
                (*ste).ste_varnames = PyList_New(0i64);
                (*ste).ste_children = PyList_New(0i64);
                if !(unsafe { (*ste).ste_symbols }.is_null()
                    || unsafe { (*ste).ste_varnames }.is_null()
                    || unsafe { (*ste).ste_children }.is_null())
                {
                    if PyDict_SetItem((*st).st_blocks, (*ste).ste_id, ste as *mut _object) >= 0 {
                        return ste;
                    }
                }
            }
        }
        Py_XDECREF__2!(ste);
        return core::ptr::null_mut::<_symtable_entry>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.symtable.ste_repr")]
extern "C-unwind" fn ste_repr(op: *mut _object) -> *mut _object {
    unsafe {
        PyUnicode_FromFormat(
            c"<symtable entry %U(%R), line %d>".as_ptr(),
            (*(op as *mut _symtable_entry)).ste_name,
            (*(op as *mut _symtable_entry)).ste_id,
            (*(op as *mut _symtable_entry)).ste_loc.lineno,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.symtable.ste_dealloc")]
extern "C-unwind" fn ste_dealloc(op: *mut _object) {
    let ste: *mut _symtable_entry = op as *mut _symtable_entry;
    unsafe {
        (*ste).ste_table = core::ptr::null_mut::<symtable>();
    }
    Py_XDECREF!(unsafe { (*ste).ste_id });
    Py_XDECREF!(unsafe { (*ste).ste_name });
    Py_XDECREF!(unsafe { (*ste).ste_symbols });
    Py_XDECREF!(unsafe { (*ste).ste_varnames });
    Py_XDECREF!(unsafe { (*ste).ste_children });
    Py_XDECREF!(unsafe { (*ste).ste_directives });
    Py_XDECREF__2!(unsafe { (*ste).ste_annotation_block });
    Py_XDECREF!(unsafe { (*ste).ste_mangled_names });
    unsafe { PyObject_Free(ste as *mut ()) };
}

static mut ste_memberlist: __SlateAlign16<[PyMemberDef; 9]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"id\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(_symtable_entry, ste_id) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(_symtable_entry, ste_name) as u64) as i64;
        __t1.flags = 1i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"symbols\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 6i32;
        __t2.offset = (core::mem::offset_of!(_symtable_entry, ste_symbols) as u64) as i64;
        __t2.flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"varnames\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 6i32;
        __t3.offset = (core::mem::offset_of!(_symtable_entry, ste_varnames) as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"children\0".as_ptr() as *mut i8) as *const i8;
        __t4.r#type = 6i32;
        __t4.offset = (core::mem::offset_of!(_symtable_entry, ste_children) as u64) as i64;
        __t4.flags = 1i32;
        __t4
    },
    {
        let mut __t5: PyMemberDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"nested\0".as_ptr() as *mut i8) as *const i8;
        __t5.r#type = 1i32;
        __t5.offset = (core::mem::offset_of!(_symtable_entry, ste_nested) as u64) as i64;
        __t5.flags = 1i32;
        __t5
    },
    {
        let mut __t6: PyMemberDef = unsafe { core::mem::zeroed() };
        __t6.name = (b"type\0".as_ptr() as *mut i8) as *const i8;
        __t6.r#type = 1i32;
        __t6.offset = (core::mem::offset_of!(_symtable_entry, ste_type) as u64) as i64;
        __t6.flags = 1i32;
        __t6
    },
    {
        let mut __t7: PyMemberDef = unsafe { core::mem::zeroed() };
        __t7.name = (b"lineno\0".as_ptr() as *mut i8) as *const i8;
        __t7.r#type = 1i32;
        __t7.offset = (core::mem::offset_of!(_symtable_entry, ste_loc.lineno) as u64) as i64;
        __t7.flags = 1i32;
        __t7
    },
    {
        let mut __t8: PyMemberDef = unsafe { core::mem::zeroed() };
        __t8.name = core::ptr::null::<i8>();
        __t8
    },
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
/// For debugging purposes only
fn symtable_new() -> *mut symtable {
    let mut st: *mut symtable = unsafe { core::mem::zeroed() };
    unsafe {
        st = PyMem_Malloc(core::mem::size_of::<symtable>() as u64) as *mut symtable;
        if st.is_null() {
            PyErr_NoMemory();
            return core::ptr::null_mut::<symtable>();
        } else {
            (*st).st_filename = core::ptr::null_mut::<_object>();
            (*st).st_blocks = core::ptr::null_mut::<_object>();
            let __v1: *mut _object = PyList_New(0i64);
            (*st).st_stack = __v1;
            if !__v1.is_null() {
                let __v0: *mut _object = PyDict_New();
                (*st).st_blocks = __v0;
                if !__v0.is_null() {
                    (*st).st_cur = core::ptr::null_mut::<_symtable_entry>();
                    (*st).st_private = core::ptr::null_mut::<_object>();
                    return st;
                }
            }
            _PySymtable_Free(st);
            return core::ptr::null_mut::<symtable>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySymtable_Build(
    r#mod: *mut _mod,
    filename: *mut _object,
    future: *mut __SlateRecord_pycore_symtable_h_67,
) -> *mut symtable {
    let mut loc0: core::mem::MaybeUninit<__SlateRecord_pycore_symtable_h_51> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_pycore_symtable_h_51 =
        core::ptr::addr_of_mut!(loc0) as *mut __SlateRecord_pycore_symtable_h_51;
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { core::mem::zeroed() };
    let mut st: *mut symtable = unsafe { core::mem::zeroed() };
    unsafe {
        st = symtable_new();
        if st.is_null() {
            return core::ptr::null_mut::<symtable>();
        } else {
            if filename.is_null() {
                _PySymtable_Free(st);
                return core::ptr::null_mut::<symtable>();
            } else {
                (*st).st_filename = Py_NewRef!(filename);
                (*st).st_future = future;
                // Setup recursion depth check counters
                let tstate: *mut _ts = _PyThreadState_GET();
                if tstate.is_null() {
                    _PySymtable_Free(st);
                    return core::ptr::null_mut::<symtable>();
                } else {
                    // Make the initial symbol information gathering pass
                    core::ptr::write(
                        __slate_slot_4,
                        __SlateRecord_pycore_symtable_h_51 {
                            lineno: 0,
                            end_lineno: 0,
                            col_offset: 0,
                            end_col_offset: 0,
                        },
                    );
                    if symtable_enter_block(
                        st,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py_top
                                ._ascii
                                .ob_base
                        ),
                        2u32,
                        r#mod as *mut (),
                        *__slate_slot_4,
                    ) == 0
                    {
                        _PySymtable_Free(st);
                        return core::ptr::null_mut::<symtable>();
                    } else {
                        '__join_0: {
                            (*st).st_top = (*st).st_cur;
                            let __t0: u32 = (*r#mod).kind as u32;
                            if __t0 == 1 {
                                seq = (*r#mod).v.Module.body;
                                if !_PyAST_GetDocString(seq).is_null() {
                                    (*(*st).st_cur).__slate_bits_1.__set_ste_has_docstring(1u32);
                                }
                                i = 0;
                                while i < asdl_seq_LEN__2!(seq) {
                                    if symtable_visit_stmt(st, asdl_seq_GET__2!(seq, i)) == 0 {
                                        break '__join_0;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                            } else {
                                if __t0 == 3 {
                                    if symtable_visit_expr(st, (*r#mod).v.Expression.body) == 0 {
                                        break '__join_0;
                                    }
                                } else {
                                    if __t0 == 2 {
                                        seq = (*r#mod).v.Interactive.body;
                                        i = 0;
                                        while i < asdl_seq_LEN__2!(seq) {
                                            if symtable_visit_stmt(st, asdl_seq_GET__2!(seq, i))
                                                == 0
                                            {
                                                break '__join_0;
                                            } else {
                                                i = i.wrapping_add(1);
                                            }
                                        }
                                    } else {
                                        if __t0 == 4 {
                                            PyErr_SetString(
                                                PyExc_RuntimeError,
                                                c"this compiler does not handle FunctionTypes"
                                                    .as_ptr(),
                                            );
                                            break '__join_0;
                                        }
                                    }
                                }
                            }
                            if symtable_exit_block(st) == 0 {
                                _PySymtable_Free(st);
                                return core::ptr::null_mut::<symtable>();
                            } else {
                                // Make the second symbol analysis pass
                                if symtable_analyze(st) != 0 {
                                    return st;
                                } else {
                                    _PySymtable_Free(st);
                                    return core::ptr::null_mut::<symtable>();
                                }
                            }
                        }
                        symtable_exit_block(st);
                        _PySymtable_Free(st);
                        return core::ptr::null_mut::<symtable>();
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySymtable_Free(st: *mut symtable) {
    Py_XDECREF!(unsafe { (*st).st_filename });
    Py_XDECREF!(unsafe { (*st).st_blocks });
    Py_XDECREF!(unsafe { (*st).st_stack });
    unsafe { PyMem_Free(st as *mut ()) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySymtable_Lookup(st: *mut symtable, key: *mut ()) -> *mut _symtable_entry {
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let k: *mut _object = unsafe { PyLong_FromVoidPtr(key) };
    if k.is_null() {
        return core::ptr::null_mut::<_symtable_entry>();
    }
    if (unsafe { PyDict_GetItemRef((*st).st_blocks, k, core::ptr::addr_of_mut!(v)) }) == 0 {
        unsafe { PyErr_SetString(PyExc_KeyError, c"unknown symbol table entry".as_ptr()) };
    }
    Py_DECREF!(k);
    v as *mut _symtable_entry
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySymtable_LookupOptional(
    st: *mut symtable,
    key: *mut (),
    out: *mut *mut _symtable_entry,
) -> i32 {
    let k: *mut _object = unsafe { PyLong_FromVoidPtr(key) };
    if k.is_null() {
        unsafe {
            *out = core::ptr::null_mut::<_symtable_entry>();
        }
        return 1i32.wrapping_neg();
    }
    let result: i32 = unsafe { PyDict_GetItemRef((*st).st_blocks, k, out as *mut *mut _object) };
    Py_DECREF!(k);
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyST_GetSymbol(ste: *mut _symtable_entry, name: *mut _object) -> i64 {
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyDict_GetItemRef((*ste).ste_symbols, name, core::ptr::addr_of_mut!(v)) }) < 0 {
        return 1i32.wrapping_neg() as i64;
    }
    if v.is_null() {
        return 0;
    }
    let symbol: i64 = unsafe { PyLong_AsLong(v) };
    Py_DECREF!(v);
    if symbol < 0 {
        if unsafe { PyErr_Occurred() }.is_null() {
            unsafe { PyErr_SetString(PyExc_SystemError, c"invalid symbol".as_ptr()) };
        }
        return 1i32.wrapping_neg() as i64;
    }
    symbol
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyST_GetScope(ste: *mut _symtable_entry, name: *mut _object) -> i32 {
    let symbol: i64 = _PyST_GetSymbol(ste, name);
    if symbol < 0 {
        return 1i32.wrapping_neg();
    }
    SYMBOL_TO_SCOPE!(symbol) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyST_IsFunctionLike(ste: *mut _symtable_entry) -> i32 {
    (((unsafe { (*ste).ste_type }) as u32) == 0
        || ((unsafe { (*ste).ste_type }) as u32) == 3
        || ((unsafe { (*ste).ste_type }) as u32) == 6
        || ((unsafe { (*ste).ste_type }) as u32) == 4
        || ((unsafe { (*ste).ste_type }) as u32) == 5) as i32
}

fn error_at_directive(ste: *mut _symtable_entry, name: *mut _object) -> i32 {
    let mut i: i64 = 0;
    while i < PyList_GET_SIZE!(unsafe { (*ste).ste_directives }) {
        let data: *mut _object = PyList_GET_ITEM!(unsafe { (*ste).ste_directives }, i);
        if (unsafe { PyUnicode_Compare(PyTuple_GET_ITEM!(data, 0i32), name) }) == 0 {
            unsafe {
                PyErr_RangedSyntaxLocationObject(
                    (*(*ste).ste_table).st_filename,
                    PyLong_AsLong(PyTuple_GET_ITEM!(data, 1i32)) as i32,
                    PyLong_AsLong(PyTuple_GET_ITEM!(data, 2i32)).wrapping_add(1) as i32,
                    PyLong_AsLong(PyTuple_GET_ITEM!(data, 3i32)) as i32,
                    PyLong_AsLong(PyTuple_GET_ITEM!(data, 4i32)).wrapping_add(1) as i32,
                )
            };
            return 0;
        }
        i = i.wrapping_add(1);
    }
    unsafe {
        PyErr_SetString(
            PyExc_RuntimeError,
            c"BUG: internal directive bookkeeping broken".as_ptr(),
        )
    };
    0
}

// Analyze raw symbol information to determine scope of each name.
//
// The next several functions are helpers for symtable_analyze(),
// which determines whether a name is local, global, or free.  In addition,
// it determines which local variables are cell variables; they provide
// bindings that are used for free variables in enclosed blocks.
//
// There are also two kinds of global variables, implicit and explicit.  An
// explicit global is declared with the global statement.  An implicit
// global is a free variable for which the compiler has found no binding
// in an enclosing function scope.  The implicit global is either a global
// or a builtin.  Python's module and class blocks use the xxx_NAME opcodes
// to handle these names to implement slightly odd semantics.  In such a
// block, the name is treated as global until it is assigned to; then it
// is treated as a local.
//
// The symbol table requires two passes to determine the scope of each name.
// The first pass collects raw facts from the AST via the symtable_visit_*
// functions: the name is a parameter here, the name is used but not defined
// here, etc.  The second pass analyzes these facts during a pass over the
// PySTEntryObjects created during pass 1.
//
// When a function is entered during the second pass, the parent passes
// the set of all name bindings visible to its children.  These bindings
// are used to determine if non-local variables are free or implicit globals.
// Names which are explicitly declared nonlocal must exist in this set of
// visible names - if they do not, a syntax error is raised. After doing
// the local analysis, it analyzes each of its child blocks using an
// updated set of name bindings.
//
// The children update the free variable set.  If a local variable is added to
// the free variable set by the child, the variable is marked as a cell.  The
// function object being defined must provide runtime storage for the variable
// that may outlive the function's frame.  Cell variables are removed from the
// free set before the analyze function returns to its parent.
//
// During analysis, the names are:
//    symbols: dict mapping from symbol names to flag values (including offset scope values)
//    scopes: dict mapping from symbol names to scope values (no offset)
//    local: set of all symbol names local to the current scope
//    bound: set of all symbol names local to a containing function scope
//    free: set of all symbol names referenced but not bound in child scopes
//    global: set of all symbol names explicitly declared as global
// Decide on scope of name, given flags.
//
// The namespace dictionaries may be modified to record information
// about the new name.  For example, a new global will add an entry to
// global.  A name that was global can be changed to local.
fn analyze_name(
    ste: *mut _symtable_entry,
    scopes: *mut _object,
    name: *mut _object,
    flags: i64,
    bound: *mut _object,
    local: *mut _object,
    free: *mut _object,
    global: *mut _object,
    type_params: *mut _object,
    class_entry: *mut _symtable_entry,
) -> i32 {
    let mut contains: i32 = 0;
    if flags & 1 != 0 {
        if flags & 8i64 != 0 {
            unsafe {
                PyErr_Format(
                    PyExc_SyntaxError,
                    c"name '%U' is nonlocal and global".as_ptr(),
                    name,
                )
            };
            return error_at_directive(ste, name);
        }
        SET_SCOPE!(scopes, name, 2i32);
        if (unsafe { PySet_Add(global, name) }) < 0 {
            return 0;
        }
        let __v1: bool = !bound.is_null() && (unsafe { PySet_Discard(bound, name) }) < 0;
        if __v1 {
            return 0;
        }
        return 1;
    }
    if flags & 8i64 != 0 {
        if bound.is_null() {
            unsafe {
                PyErr_Format(
                    PyExc_SyntaxError,
                    c"nonlocal declaration not allowed at module level".as_ptr(),
                )
            };
            return error_at_directive(ste, name);
        }
        contains = unsafe { PySet_Contains(bound, name) };
        if contains < 0 {
            return 0;
        }
        if contains == 0 {
            unsafe {
                PyErr_Format(
                    PyExc_SyntaxError,
                    c"no binding for nonlocal '%U' found".as_ptr(),
                    name,
                )
            };
            return error_at_directive(ste, name);
        }
        contains = unsafe { PySet_Contains(type_params, name) };
        if contains < 0 {
            return 0;
        }
        if contains != 0 {
            unsafe {
                PyErr_Format(
                    PyExc_SyntaxError,
                    c"nonlocal binding not allowed for type parameter '%U'".as_ptr(),
                    name,
                )
            };
            return error_at_directive(ste, name);
        }
        SET_SCOPE!(scopes, name, 4i32);
        return ((unsafe { PySet_Add(free, name) }) >= 0) as i32;
    }
    if flags & 134i64 != 0 {
        SET_SCOPE!(scopes, name, 1i32);
        if (unsafe { PySet_Add(local, name) }) < 0 {
            return 0;
        }
        if (unsafe { PySet_Discard(global, name) }) < 0 {
            return 0;
        }
        if flags & 1024i64 != 0 {
            if (unsafe { PySet_Add(type_params, name) }) < 0 {
                return 0;
            }
        } else {
            if (unsafe { PySet_Discard(type_params, name) }) < 0 {
                return 0;
            }
        }
        return 1;
    }
    // If we were passed class_entry (i.e., we're in an ste_can_see_class_scope scope)
    //
    // and the bound name is in that set, then the name is potentially bound both by
    //
    // the immediately enclosing class namespace, and also by an outer function namespace.
    //
    // In that case, we want the runtime name resolution to look at only the class
    //
    // namespace and the globals (not the namespace providing the bound).
    //
    // Similarly, if the name is explicitly global in the class namespace (through the
    //
    // global statement), we want to also treat it as a global in this scope.
    if !class_entry.is_null() {
        let class_flags: i64 = _PyST_GetSymbol(class_entry, name);
        if class_flags < 0 {
            return 0;
        }
        if class_flags & 1 != 0 {
            SET_SCOPE!(scopes, name, 2i32);
            return 1;
        } else {
            if class_flags & 134i64 != 0 && class_flags & 8i64 == 0 {
                SET_SCOPE!(scopes, name, 3i32);
                return 1;
            }
        }
    }
    // If an enclosing block has a binding for this name, it
    // is a free variable rather than a global variable.
    // Note that having a non-NULL bound implies that the block
    // is nested.
    if !bound.is_null() {
        contains = unsafe { PySet_Contains(bound, name) };
        if contains < 0 {
            return 0;
        }
        if contains != 0 {
            SET_SCOPE!(scopes, name, 4i32);
            return ((unsafe { PySet_Add(free, name) }) >= 0) as i32;
        }
    }
    // If a parent has a global statement, then call it global
    // explicit?  It could also be global implicit.
    if !global.is_null() {
        contains = unsafe { PySet_Contains(global, name) };
        if contains < 0 {
            return 0;
        }
        if contains != 0 {
            SET_SCOPE!(scopes, name, 3i32);
            return 1;
        }
    }
    SET_SCOPE!(scopes, name, 3i32);
    1
}

fn is_free_in_any_child(entry: *mut _symtable_entry, key: *mut _object) -> i32 {
    let mut i: i64 = 0;
    while i < PyList_GET_SIZE!(unsafe { (*entry).ste_children }) {
        let child_ste: *mut _symtable_entry =
            PyList_GET_ITEM!(unsafe { (*entry).ste_children }, i) as *mut _symtable_entry;
        let scope: i64 = _PyST_GetScope(child_ste, key) as i64;
        if scope < 0 {
            return 1i32.wrapping_neg();
        }
        if scope == 4 {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}

fn inline_comprehension(
    ste: *mut _symtable_entry,
    comp: *mut _symtable_entry,
    scopes: *mut _object,
    comp_free: *mut _object,
    inlined_cells: *mut _object,
) -> i32 {
    let mut k: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = 0;
    let mut remove_dunder_class: i32 = 0;
    let mut remove_dunder_classdict: i32 = 0;
    let mut remove_dunder_cond_annotations: i32 = 0;
    while (unsafe {
        PyDict_Next(
            (*comp).ste_symbols,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(k),
            core::ptr::addr_of_mut!(v),
        )
    }) != 0
    {
        // skip comprehension parameter
        let comp_flags: i64 = unsafe { PyLong_AsLong(v) };
        let __v0: bool =
            comp_flags == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return 0;
        }
        if comp_flags & 4i64 == 0 {
            let mut scope: i32 = SYMBOL_TO_SCOPE!(comp_flags) as i32;
            let only_flags: i32 = (comp_flags & 4095i64) as i32;
            if scope == 5 || only_flags & 2048i32 != 0 {
                if (unsafe { PySet_Add(inlined_cells, k) }) < 0 {
                    return 0;
                }
            }
            let existing: *mut _object = unsafe { PyDict_GetItemWithError((*ste).ste_symbols, k) };
            let __v2: bool = existing.is_null() && !unsafe { PyErr_Occurred() }.is_null();
            if __v2 {
                return 0;
            }
            // __class__, __classdict__ and __conditional_annotations__ are
            //
            // not allowed to be free through a class scope (see
            //
            // drop_class_free) unless children scopes need it
            let __v3: bool = if scope == 4 && ((unsafe { (*ste).ste_type }) as u32) == 1 {
                let __v4: bool =
                    (unsafe { _PyUnicode_EqualToASCIIString(k, c"__class__".as_ptr()) }) != 0
                        || (unsafe { _PyUnicode_EqualToASCIIString(k, c"__classdict__".as_ptr()) })
                            != 0;
                let __v5: bool = __v4
                    || (unsafe {
                        _PyUnicode_EqualToASCIIString(k, c"__conditional_annotations__".as_ptr())
                    }) != 0;
                __v5
            } else {
                false
            };
            if __v3 {
                scope = 3;
                let child_needs_free: i32 = is_free_in_any_child(comp, k);
                if child_needs_free < 0 {
                    return 0;
                }
                if child_needs_free == 0 {
                    if (unsafe { PySet_Discard(comp_free, k) }) < 0 {
                        return 0;
                    }
                }
                if (unsafe { _PyUnicode_EqualToASCIIString(k, c"__class__".as_ptr()) }) != 0 {
                    remove_dunder_class = 1;
                } else {
                    if (unsafe {
                        _PyUnicode_EqualToASCIIString(k, c"__conditional_annotations__".as_ptr())
                    }) != 0
                    {
                        remove_dunder_cond_annotations = 1;
                    } else {
                        remove_dunder_classdict = 1;
                    }
                }
            }
            if existing.is_null() {
                // name does not exist in scope, copy from comprehension
                let v_flags: *mut _object = unsafe { PyLong_FromLong(only_flags as i64) };
                if v_flags.is_null() {
                    return 0;
                }
                let ok: i32 = unsafe { PyDict_SetItem((*ste).ste_symbols, k, v_flags) };
                Py_DECREF!(v_flags);
                if ok < 0 {
                    return 0;
                }
                SET_SCOPE!(scopes, k, scope);
            } else {
                let flags: i64 = unsafe { PyLong_AsLong(existing) };
                let __v7: bool =
                    flags == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
                if __v7 {
                    return 0;
                }
                if flags & 134i64 != 0 && ((unsafe { (*ste).ste_type }) as u32) != 1 {
                    // free vars in comprehension that are locals in outer scope can
                    //
                    // now simply be locals, unless they are free in comp children,
                    //
                    // or if the outer scope is a class block
                    let ok: i32 = is_free_in_any_child(comp, k);
                    if ok < 0 {
                        return 0;
                    }
                    if ok == 0 {
                        if (unsafe { PySet_Discard(comp_free, k) }) < 0 {
                            return 0;
                        }
                    }
                }
            }
        }
    }
    let __v8: bool = remove_dunder_class != 0
        && (unsafe { PyDict_DelItemString((*comp).ste_symbols, c"__class__".as_ptr()) }) < 0;
    if __v8 {
        return 0;
    }
    let __v9: bool = remove_dunder_classdict != 0
        && (unsafe { PyDict_DelItemString((*comp).ste_symbols, c"__classdict__".as_ptr()) }) < 0;
    if __v9 {
        return 0;
    }
    let __v10: bool = remove_dunder_cond_annotations != 0
        && (unsafe {
            PyDict_DelItemString((*comp).ste_symbols, c"__conditional_annotations__".as_ptr())
        }) < 0;
    if __v10 {
        return 0;
    }
    1
}

// If a name is defined in free and also in locals, then this block
// provides the binding for the free variable.  The name should be
// marked CELL in this block and removed from the free list.
//
// Note that the current block's free variables are included in free.
// That's safe because no name can be free and local in the same scope.
fn analyze_cells(scopes: *mut _object, free: *mut _object, inlined_cells: *mut _object) -> i32 {
    let mut contains: i32 = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut scope: i64 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut success: i32 = unsafe { core::mem::zeroed() };
    let mut v_cell: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        success = 0;
        pos = 0;
        v_cell = PyLong_FromLong(5i64);
        if v_cell.is_null() {
            return 0;
        } else {
            '__join_0: {
                while PyDict_Next(
                    scopes,
                    core::ptr::addr_of_mut!(pos),
                    core::ptr::addr_of_mut!(name),
                    core::ptr::addr_of_mut!(v),
                ) != 0
                {
                    scope = PyLong_AsLong(v);
                    if scope == (1i32.wrapping_neg() as i64) {
                        __v1 = !PyErr_Occurred().is_null();
                    } else {
                        __v1 = false;
                    }
                    if __v1 {
                        break '__join_0;
                    } else {
                        if scope == 1 {
                            contains = PySet_Contains(free, name);
                            if contains < 0 {
                                break '__join_0;
                            } else {
                                if contains == 0 {
                                    contains = PySet_Contains(inlined_cells, name);
                                    if contains < 0 {
                                        break '__join_0;
                                    } else {
                                        if contains == 0 {
                                            continue;
                                        }
                                    }
                                }
                                // Replace LOCAL with CELL for this name, and remove
                                // from free. It is safe to replace the value of name
                                // in the dict, because it will not cause a resize.
                                if PyDict_SetItem(scopes, name, v_cell) < 0 {
                                    break '__join_0;
                                } else {
                                    if PySet_Discard(free, name) < 0 {
                                        break '__join_0;
                                    }
                                }
                            }
                        }
                    }
                }
                success = 1;
            }
            Py_DECREF!(v_cell);
            return success;
        }
    }
}

fn drop_class_free(ste: *mut _symtable_entry, free: *mut _object) -> i32 {
    let mut res: i32 = unsafe {
        PySet_Discard(
            free,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___class__
                    ._ascii
                    .ob_base
            ),
        )
    };
    if res < 0 {
        return 0;
    }
    if res != 0 {
        unsafe {
            (*ste).__slate_bits_1.__set_ste_needs_class_closure(1u32);
        }
    }
    res = unsafe {
        PySet_Discard(
            free,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___classdict__
                    ._ascii
                    .ob_base
            ),
        )
    };
    if res < 0 {
        return 0;
    }
    if res != 0 {
        unsafe {
            (*ste).__slate_bits_1.__set_ste_needs_classdict(1u32);
        }
    }
    res = unsafe {
        PySet_Discard(
            free,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___conditional_annotations__
                    ._ascii
                    .ob_base
            ),
        )
    };
    if res < 0 {
        return 0;
    }
    if res != 0 {
        unsafe {
            (*ste)
                .__slate_bits_1
                .__set_ste_has_conditional_annotations(1u32);
        }
    }
    1
}

/// Enter the final scope information into the ste_symbols dict.
///
/// All arguments are dicts.  Modifies symbols, others are read-only.
fn update_symbols(
    symbols: *mut _object,
    scopes: *mut _object,
    bound: *mut _object,
    free: *mut _object,
    inlined_cells: *mut _object,
    classflag: i32,
) -> i32 {
    let mut contains: i32 = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut flags: i64 = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut scope: i64 = unsafe { core::mem::zeroed() };
    let mut contains_1: i32 = unsafe { core::mem::zeroed() };
    let mut __v13: bool = unsafe { core::mem::zeroed() };
    let mut flags_1: i64 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut v_free: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_new: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_scope: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut itr: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        name = core::ptr::null_mut::<_object>();
        itr = core::ptr::null_mut::<_object>();
        v = core::ptr::null_mut::<_object>();
        v_scope = core::ptr::null_mut::<_object>();
        v_new = core::ptr::null_mut::<_object>();
        v_free = core::ptr::null_mut::<_object>();
        pos = 0;
        // Update scope information for all symbols in this scope
        while PyDict_Next(
            symbols,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(name),
            core::ptr::addr_of_mut!(v),
        ) != 0
        {
            flags_1 = PyLong_AsLong(v);
            if flags_1 == (1i32.wrapping_neg() as i64) {
                __v13 = !PyErr_Occurred().is_null();
            } else {
                __v13 = false;
            }
            if __v13 {
                return 0;
            } else {
                contains_1 = PySet_Contains(inlined_cells, name);
                if contains_1 < 0 {
                    return 0;
                } else {
                    if contains_1 != 0 {
                        flags_1 |= 2048i64;
                    }
                    if PyDict_GetItemRef(scopes, name, core::ptr::addr_of_mut!(v_scope)) < 0 {
                        return 0;
                    } else {
                        if v_scope.is_null() {
                            PyErr_SetObject(PyExc_KeyError, name);
                            return 0;
                        } else {
                            scope = PyLong_AsLong(v_scope);
                            Py_DECREF!(v_scope);
                            if scope == (1i32.wrapping_neg() as i64) {
                                __v8 = !PyErr_Occurred().is_null();
                            } else {
                                __v8 = false;
                            }
                            if __v8 {
                                return 0;
                            } else {
                                flags_1 |= scope << 12i32;
                                v_new = PyLong_FromLong(flags_1);
                                if v_new.is_null() {
                                    return 0;
                                } else {
                                    if PyDict_SetItem(symbols, name, v_new) < 0 {
                                        Py_DECREF!(v_new);
                                        return 0;
                                    } else {
                                        Py_DECREF!(v_new);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Record not yet resolved free variables from children (if any)
        v_free = PyLong_FromLong(16384i64);
        if v_free.is_null() {
            return 0;
        } else {
            itr = PyObject_GetIter(free);
            if itr.is_null() {
                Py_DECREF!(v_free);
                return 0;
            } else {
                '__join_0: {
                    '__join_12: {
                        loop {
                            let __v0: *mut _object = PyIter_Next(itr);
                            name = __v0;
                            if !__v0.is_null() {
                                v = PyDict_GetItemWithError(symbols, name);
                                // Handle symbol that already exists in this scope
                                if !v.is_null() {
                                    // Handle a free variable in a method of
                                    // the class that has the same name as a local
                                    // or global in the class scope.
                                    if classflag != 0 {
                                        flags = PyLong_AsLong(v);
                                        if flags == (1i32.wrapping_neg() as i64) {
                                            __v4 = !PyErr_Occurred().is_null();
                                        } else {
                                            __v4 = false;
                                        }
                                        if __v4 {
                                            break '__join_0;
                                        } else {
                                            flags |= 64i64;
                                            v_new = PyLong_FromLong(flags);
                                            if v_new.is_null() {
                                                break '__join_0;
                                            } else {
                                                if PyDict_SetItem(symbols, name, v_new) < 0 {
                                                    break '__join_12;
                                                } else {
                                                    Py_DECREF!(v_new);
                                                }
                                            }
                                        }
                                    }
                                    // It's a cell, or already free in this scope
                                    Py_DECREF!(name);
                                } else {
                                    if !PyErr_Occurred().is_null() {
                                        break '__join_0;
                                    } else {
                                        // Handle global symbol
                                        if !bound.is_null() {
                                            contains = PySet_Contains(bound, name);
                                            if contains < 0 {
                                                break '__join_0;
                                            } else {
                                                if contains == 0 {
                                                    Py_DECREF!(name);
                                                    continue;
                                                }
                                            }
                                        }
                                        // Propagate new free symbol up the lexical stack
                                        if PyDict_SetItem(symbols, name, v_free) < 0 {
                                            break '__join_0;
                                        } else {
                                            Py_DECREF!(name);
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        // Check if loop ended because of exception in PyIter_Next
                        if !PyErr_Occurred().is_null() {
                            break '__join_0;
                        } else {
                            Py_DECREF!(itr);
                            Py_DECREF!(v_free);
                            return 1;
                        }
                    }
                    Py_DECREF!(v_new);
                }
                Py_XDECREF!(v_free);
                Py_XDECREF!(itr);
                Py_XDECREF!(name);
                return 0;
            }
        }
    }
    // it's a global
}

// Make final symbol table decisions for block of ste.
//
// Arguments:
// ste -- current symtable entry (input/output)
// bound -- set of variables bound in enclosing scopes (input).  bound
//     is NULL for module blocks.
// free -- set of free variables in enclosed scopes (output)
// globals -- set of declared global variables in enclosing scopes (input)
//
// The implementation uses two mutually recursive functions,
// analyze_block() and analyze_child_block().  analyze_block() is
// responsible for analyzing the individual names defined in a block.
// analyze_child_block() prepares temporary namespace dictionaries
// used to evaluated nested blocks.
//
// The two functions exist because a child block should see the name
// bindings of its enclosing blocks, but those bindings should not
// propagate back to a parent block.
fn analyze_block(
    ste: *mut _symtable_entry,
    bound: *mut _object,
    free: *mut _object,
    global: *mut _object,
    type_params: *mut _object,
    class_entry: *mut _symtable_entry,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    // Check if any local variables must be converted to cell variables
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut entry: *mut _symtable_entry = unsafe { core::mem::zeroed() };
    // we inline all non-generator-expression comprehensions,
    //
    // except those in annotation scopes that are nested in classes
    let mut inline_comp: i32 = unsafe { core::mem::zeroed() };
    let mut new_class_entry: *mut _symtable_entry = unsafe { core::mem::zeroed() };
    let mut entry_1: *mut _symtable_entry = unsafe { core::mem::zeroed() };
    let mut child_free: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v14: bool = unsafe { core::mem::zeroed() };
    let mut flags: i64 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut success: i32 = unsafe { core::mem::zeroed() };
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut inlined_cells: *mut _object = unsafe { core::mem::zeroed() };
    let mut newfree: *mut _object = unsafe { core::mem::zeroed() };
    let mut newglobal: *mut _object = unsafe { core::mem::zeroed() };
    let mut newbound: *mut _object = unsafe { core::mem::zeroed() };
    let mut scopes: *mut _object = unsafe { core::mem::zeroed() };
    let mut local: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_2: {
            local = core::ptr::null_mut::<_object>();
            scopes = core::ptr::null_mut::<_object>();
            newbound = core::ptr::null_mut::<_object>();
            newglobal = core::ptr::null_mut::<_object>();
            newfree = core::ptr::null_mut::<_object>();
            inlined_cells = core::ptr::null_mut::<_object>();
            success = 0;
            pos = 0;
            local = PySet_New(core::ptr::null_mut::<_object>()); // collect new names bound in block
            if !local.is_null() {
                scopes = PyDict_New(); // collect scopes defined for each name
                if !scopes.is_null() {
                    // Allocate new global, bound and free variable sets.  These
                    // sets hold the names visible in nested blocks.  For
                    // ClassBlocks, the bound and global names are initialized
                    // before analyzing names, because class bindings aren't
                    // visible in methods.  For other blocks, they are initialized
                    // after names are analyzed.
                    // TODO(jhylton): Package these dicts in a struct so that we
                    // can write reasonable helper functions?
                    newglobal = PySet_New(core::ptr::null_mut::<_object>());
                    if !newglobal.is_null() {
                        newfree = PySet_New(core::ptr::null_mut::<_object>());
                        if !newfree.is_null() {
                            newbound = PySet_New(core::ptr::null_mut::<_object>());
                            if !newbound.is_null() {
                                inlined_cells = PySet_New(core::ptr::null_mut::<_object>());
                                if !inlined_cells.is_null() {
                                    // Class namespace has no effect on names visible in
                                    // nested functions, so populate the global and bound
                                    // sets to be passed to child blocks before analyzing
                                    // this one.
                                    if ((*ste).ste_type as u32) == 1 {
                                        // Pass down known globals
                                        temp = PyNumber_InPlaceOr(newglobal, global);
                                        if temp.is_null() {
                                            break '__join_2;
                                        } else {
                                            Py_DECREF!(temp);
                                            // Pass down previously bound symbols
                                            if !bound.is_null() {
                                                temp = PyNumber_InPlaceOr(newbound, bound);
                                                if temp.is_null() {
                                                    break '__join_2;
                                                } else {
                                                    Py_DECREF!(temp);
                                                }
                                            }
                                        }
                                    }
                                    while PyDict_Next(
                                        (*ste).ste_symbols,
                                        core::ptr::addr_of_mut!(pos),
                                        core::ptr::addr_of_mut!(name),
                                        core::ptr::addr_of_mut!(v),
                                    ) != 0
                                    {
                                        flags = PyLong_AsLong(v);
                                        if flags == (1i32.wrapping_neg() as i64) {
                                            __v14 = !PyErr_Occurred().is_null();
                                        } else {
                                            __v14 = false;
                                        }
                                        if __v14 {
                                            break '__join_2;
                                        } else {
                                            if analyze_name(
                                                ste,
                                                scopes,
                                                name,
                                                flags,
                                                bound,
                                                local,
                                                free,
                                                global,
                                                type_params,
                                                class_entry,
                                            ) == 0
                                            {
                                                break '__join_2;
                                            }
                                        }
                                    }
                                    // Populate global and bound sets to be passed to children.
                                    if ((*ste).ste_type as u32) != 1 {
                                        // Add function locals to bound set
                                        if _PyST_IsFunctionLike(ste) != 0 {
                                            temp = PyNumber_InPlaceOr(newbound, local);
                                            if temp.is_null() {
                                                break '__join_2;
                                            } else {
                                                Py_DECREF!(temp);
                                            }
                                        }
                                        // Pass down previously bound symbols
                                        if !bound.is_null() {
                                            temp = PyNumber_InPlaceOr(newbound, bound);
                                            if temp.is_null() {
                                                break '__join_2;
                                            } else {
                                                Py_DECREF!(temp);
                                            }
                                        }
                                        // Pass down known globals
                                        temp = PyNumber_InPlaceOr(newglobal, global);
                                        if temp.is_null() {
                                            break '__join_2;
                                        } else {
                                            Py_DECREF!(temp);
                                        }
                                    } else {
                                        // Special-case __class__ and __classdict__
                                        if PySet_Add(
                                            newbound,
                                            core::ptr::addr_of_mut!(
                                                _PyRuntime
                                                    .static_objects
                                                    .singletons
                                                    .strings
                                                    .identifiers
                                                    ._py___class__
                                                    ._ascii
                                                    .ob_base
                                            ),
                                        ) < 0
                                        {
                                            break '__join_2;
                                        } else {
                                            if PySet_Add(
                                                newbound,
                                                core::ptr::addr_of_mut!(
                                                    _PyRuntime
                                                        .static_objects
                                                        .singletons
                                                        .strings
                                                        .identifiers
                                                        ._py___classdict__
                                                        ._ascii
                                                        .ob_base
                                                ),
                                            ) < 0
                                            {
                                                break '__join_2;
                                            } else {
                                                if PySet_Add(
                                                    newbound,
                                                    core::ptr::addr_of_mut!(
                                                        _PyRuntime
                                                            .static_objects
                                                            .singletons
                                                            .strings
                                                            .identifiers
                                                            ._py___conditional_annotations__
                                                            ._ascii
                                                            .ob_base
                                                    ),
                                                ) < 0
                                                {
                                                    break '__join_2;
                                                }
                                            }
                                        }
                                    }
                                    // Recursively call analyze_child_block() on each child block.
                                    //
                                    // newbound, newglobal now contain the names visible in
                                    // nested blocks.  The free variables in the children will
                                    // be added to newfree.
                                    i = 0;
                                    '__join_25: {
                                        while i < PyList_GET_SIZE!(unsafe { (*ste).ste_children }) {
                                            child_free = core::ptr::null_mut::<_object>();
                                            let c: *mut _object =
                                                PyList_GET_ITEM!(unsafe { (*ste).ste_children }, i);
                                            entry_1 = c as *mut _symtable_entry;
                                            new_class_entry =
                                                core::ptr::null_mut::<_symtable_entry>();
                                            if ((*entry_1)
                                                .__slate_bits_1
                                                .__get_ste_can_see_class_scope()
                                                as i32)
                                                != 0
                                            {
                                                if ((*ste).ste_type as u32) == 1 {
                                                    new_class_entry = ste;
                                                } else {
                                                    if !class_entry.is_null() {
                                                        new_class_entry = class_entry;
                                                    }
                                                }
                                            }
                                            inline_comp = (((*entry_1).ste_comprehension as u32)
                                                != 0
                                                && ((*entry_1).__slate_bits_0.__get_ste_generator()
                                                    as i32)
                                                    == 0
                                                && ((*ste)
                                                    .__slate_bits_1
                                                    .__get_ste_can_see_class_scope()
                                                    as i32)
                                                    == 0)
                                                as i32;
                                            if analyze_child_block(
                                                entry_1,
                                                newbound,
                                                newfree,
                                                newglobal,
                                                type_params,
                                                new_class_entry,
                                                &mut (*core::ptr::addr_of_mut!(child_free)),
                                            ) == 0
                                            {
                                                break '__join_2;
                                            } else {
                                                if inline_comp != 0 {
                                                    if inline_comprehension(
                                                        ste,
                                                        entry_1,
                                                        scopes,
                                                        child_free,
                                                        inlined_cells,
                                                    ) == 0
                                                    {
                                                        break '__join_25;
                                                    } else {
                                                        (*entry_1)
                                                            .__slate_bits_1
                                                            .__set_ste_comp_inlined(1u32);
                                                    }
                                                }
                                                temp = PyNumber_InPlaceOr(newfree, child_free);
                                                Py_DECREF!(child_free);
                                                if temp.is_null() {
                                                    break '__join_2;
                                                } else {
                                                    Py_DECREF!(temp);
                                                    i = i.wrapping_add(1);
                                                }
                                            }
                                        }
                                        // Splice children of inlined comprehensions into our children list
                                        i = PyList_GET_SIZE!(unsafe { (*ste).ste_children })
                                            .wrapping_sub(1);
                                        while i >= 0 {
                                            let c: *mut _object =
                                                PyList_GET_ITEM!(unsafe { (*ste).ste_children }, i);
                                            entry = c as *mut _symtable_entry;
                                            if ((*entry).__slate_bits_1.__get_ste_comp_inlined()
                                                as i32)
                                                != 0
                                            {
                                                __v4 = PyList_SetSlice(
                                                    (*ste).ste_children,
                                                    i,
                                                    i.wrapping_add(1),
                                                    (*entry).ste_children,
                                                ) < 0;
                                            } else {
                                                __v4 = false;
                                            }
                                            if __v4 {
                                                break '__join_2;
                                            } else {
                                                i = i.wrapping_sub(1);
                                            }
                                        }
                                        if _PyST_IsFunctionLike(ste) != 0 {
                                            __v1 =
                                                analyze_cells(scopes, newfree, inlined_cells) == 0;
                                        } else {
                                            __v1 = false;
                                        }
                                        if __v1 {
                                            break '__join_2;
                                        } else {
                                            if ((*ste).ste_type as u32) == 1 {
                                                __v0 = drop_class_free(ste, newfree) == 0;
                                            } else {
                                                __v0 = false;
                                            }
                                            if __v0 {
                                                break '__join_2;
                                            } else {
                                                // Records the results of the analysis in the symbol table entry
                                                if update_symbols(
                                                    (*ste).ste_symbols,
                                                    scopes,
                                                    bound,
                                                    newfree,
                                                    inlined_cells,
                                                    (((*ste).ste_type as u32) == 1
                                                        || ((*ste)
                                                            .__slate_bits_1
                                                            .__get_ste_can_see_class_scope()
                                                            as i32)
                                                            != 0)
                                                        as i32,
                                                ) == 0
                                                {
                                                    break '__join_2;
                                                } else {
                                                    temp = PyNumber_InPlaceOr(free, newfree);
                                                    if temp.is_null() {
                                                        break '__join_2;
                                                    } else {
                                                        Py_DECREF!(temp);
                                                        success = 1;
                                                        break '__join_2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Py_DECREF!(child_free);
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(scopes);
        Py_XDECREF!(local);
        Py_XDECREF!(newbound);
        Py_XDECREF!(newglobal);
        Py_XDECREF!(newfree);
        Py_XDECREF!(inlined_cells);
        return success;
    }
}

fn analyze_child_block(
    entry: *mut _symtable_entry,
    bound: *mut _object,
    free: *mut _object,
    global: *mut _object,
    type_params: *mut _object,
    class_entry: *mut _symtable_entry,
    child_free: &mut *mut _object,
) -> i32 {
    let mut temp_type_params: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp_free: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp_global: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp_bound: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        temp_bound = core::ptr::null_mut::<_object>();
        temp_global = core::ptr::null_mut::<_object>();
        temp_free = core::ptr::null_mut::<_object>();
        temp_type_params = core::ptr::null_mut::<_object>();
        // Copy the bound/global/free sets.
        //
        // These sets are used by all blocks enclosed by the
        // current block.  The analyze_block() call modifies these
        // sets.
        temp_bound = PySet_New(bound);
        if !temp_bound.is_null() {
            temp_free = PySet_New(free);
            if !temp_free.is_null() {
                temp_global = PySet_New(global);
                if !temp_global.is_null() {
                    temp_type_params = PySet_New(type_params);
                    if !temp_type_params.is_null() {
                        if analyze_block(
                            entry,
                            temp_bound,
                            temp_free,
                            temp_global,
                            temp_type_params,
                            class_entry,
                        ) != 0
                        {
                            *(child_free as *mut *mut _object) = temp_free;
                            Py_DECREF!(temp_bound);
                            Py_DECREF!(temp_global);
                            Py_DECREF!(temp_type_params);
                            return 1;
                        }
                    }
                }
            }
        }
        Py_XDECREF!(temp_bound);
        Py_XDECREF!(temp_free);
        Py_XDECREF!(temp_global);
        Py_XDECREF!(temp_type_params);
        return 0;
    }
}

fn symtable_analyze(st: *mut symtable) -> i32 {
    let mut r: i32 = 0;
    let free: *mut _object = unsafe { PySet_New(core::ptr::null_mut::<_object>()) };
    if free.is_null() {
        return 0;
    }
    let global: *mut _object = unsafe { PySet_New(core::ptr::null_mut::<_object>()) };
    if global.is_null() {
        Py_DECREF!(free);
        return 0;
    }
    let type_params: *mut _object = unsafe { PySet_New(core::ptr::null_mut::<_object>()) };
    if type_params.is_null() {
        Py_DECREF!(free);
        Py_DECREF!(global);
        return 0;
    }
    r = analyze_block(
        unsafe { (*st).st_top },
        core::ptr::null_mut::<_object>(),
        free,
        global,
        type_params,
        core::ptr::null_mut::<_symtable_entry>(),
    );
    Py_DECREF!(free);
    Py_DECREF!(global);
    Py_DECREF!(type_params);
    r
}

// symtable_enter_block() gets a reference via ste_new.
// This reference is released when the block is exited, via the DECREF
// in symtable_exit_block().
fn symtable_exit_block(st: *mut symtable) -> i32 {
    let mut size: i64 = 0;
    unsafe {
        (*st).st_cur = core::ptr::null_mut::<_symtable_entry>();
    }
    size = PyList_GET_SIZE!(unsafe { (*st).st_stack });
    if size != 0 {
        if (unsafe {
            PyList_SetSlice(
                (*st).st_stack,
                size.wrapping_sub(1),
                size,
                core::ptr::null_mut::<_object>(),
            )
        }) < 0
        {
            return 0;
        }
        let __v1: i64 = size.wrapping_sub(1);
        size = __v1;
        if __v1 != 0 {
            unsafe {
                (*st).st_cur = PyList_GET_ITEM!(unsafe { (*st).st_stack }, size.wrapping_sub(1))
                    as *mut _symtable_entry;
            }
        }
    }
    1
}

fn symtable_enter_existing_block(
    st: *mut symtable,
    ste: *mut _symtable_entry,
    add_to_children: bool,
) -> i32 {
    if (unsafe { PyList_Append((*st).st_stack, ste as *mut _object) }) < 0 {
        return 0;
    }
    let prev: *mut _symtable_entry = unsafe { (*st).st_cur };
    // bpo-37757: For now, disallow *all* assignment expressions in the
    // outermost iterator expression of a comprehension, even those inside
    // a nested comprehension or a lambda expression.
    if !prev.is_null() {
        unsafe {
            (*ste).ste_comp_iter_expr = (*prev).ste_comp_iter_expr;
        }
    }
    // No need to inherit ste_mangled_names in classes, where all names
    // are mangled.
    if !prev.is_null()
        && !unsafe { (*prev).ste_mangled_names }.is_null()
        && ((unsafe { (*ste).ste_type }) as u32) != 1
    {
        unsafe {
            (*ste).ste_mangled_names = Py_NewRef!(unsafe { (*prev).ste_mangled_names });
        }
    }
    // The entry is owned by the stack. Borrow it for st_cur.
    unsafe {
        (*st).st_cur = ste;
    }
    // If "from __future__ import annotations" is active,
    // annotation blocks shouldn't have any affect on the symbol table since in
    // the compilation stage, they will all be transformed to strings.
    if (unsafe { (*(*st).st_future).ff_features }) & 16777216 != 0
        && ((unsafe { (*ste).ste_type }) as u32) == 3
    {
        return 1;
    }
    if ((unsafe { (*ste).ste_type }) as u32) == 2 {
        unsafe {
            (*st).st_global = (*(*st).st_cur).ste_symbols;
        }
    }
    if add_to_children && !prev.is_null() {
        if (unsafe { PyList_Append((*prev).ste_children, ste as *mut _object) }) < 0 {
            symtable_exit_block(st);
            return 0;
        }
    }
    1
}

fn symtable_enter_block(
    st: *mut symtable,
    name: *mut _object,
    block: u32,
    ast: *mut (),
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    let ste: *mut _symtable_entry = ste_new(st, name, block, ast, loc);
    if ste.is_null() {
        return 0;
    }
    let result: i32 = symtable_enter_existing_block(st, ste, true); // add_to_children
    Py_DECREF__2!(ste);
    if result == 0 {
        return 0;
    }
    if (block as u32) == 3 || (block as u32) == 6 || (block as u32) == 4 {
        // We need to insert code that reads this "parameter" to the function.
        if symtable_add_def(
            st,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .literals
                        ._py_format
                        ._ascii
                        .ob_base
                )
            },
            4i32,
            loc,
        ) == 0
        {
            symtable_exit_block(st);
            return 0;
        }
        if symtable_add_def(
            st,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .literals
                        ._py_format
                        ._ascii
                        .ob_base
                )
            },
            16i32,
            loc,
        ) == 0
        {
            symtable_exit_block(st);
            return 0;
        }
    }
    1
}

fn symtable_lookup_entry(st: *mut symtable, ste: *mut _symtable_entry, name: *mut _object) -> i64 {
    let mangled: *mut _object = _Py_MaybeMangle(unsafe { (*st).st_private }, ste, name);
    if mangled.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    let ret: i64 = _PyST_GetSymbol(ste, mangled);
    Py_DECREF!(mangled);
    if ret < 0 {
        return 1i32.wrapping_neg() as i64;
    }
    ret
}

fn symtable_lookup(st: *mut symtable, name: *mut _object) -> i64 {
    symtable_lookup_entry(st, unsafe { (*st).st_cur }, name)
}

fn symtable_add_def_helper(
    st: *mut symtable,
    name: *mut _object,
    flag: i32,
    ste: *mut _symtable_entry,
    mut loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut mangled: *mut _object = unsafe { core::mem::zeroed() };
    let mut val: i64 = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    let mut o: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        mangled = _Py_MaybeMangle((*st).st_private, (*st).st_cur, name);
        if mangled.is_null() {
            return 0;
        } else {
            '__join_0: {
                dict = (*ste).ste_symbols;
                let __v9: *mut _object = PyDict_GetItemWithError(dict, mangled);
                o = __v9;
                if !__v9.is_null() {
                    val = PyLong_AsLong(o);
                    if val == (1i32.wrapping_neg() as i64) {
                        __v8 = !PyErr_Occurred().is_null();
                    } else {
                        __v8 = false;
                    }
                    if __v8 {
                        break '__join_0;
                    } else {
                        if flag & 4i32 != 0 && val & 4i64 != 0 {
                            // Is it better to use 'mangled' or 'name' here?
                            PyErr_Format(
                                PyExc_SyntaxError,
                                c"duplicate argument '%U' in function definition".as_ptr(),
                                name,
                            );
                            SET_ERROR_LOCATION!(unsafe { (*st).st_filename }, loc);
                            break '__join_0;
                        } else {
                            if flag & 1024i32 != 0 && val & 1024i64 != 0 {
                                PyErr_Format(
                                    PyExc_SyntaxError,
                                    c"duplicate type parameter '%U'".as_ptr(),
                                    name,
                                );
                                SET_ERROR_LOCATION!(unsafe { (*st).st_filename }, loc);
                                break '__join_0;
                            } else {
                                val |= flag as i64;
                            }
                        }
                    }
                } else {
                    if !PyErr_Occurred().is_null() {
                        break '__join_0;
                    } else {
                        val = flag as i64;
                    }
                }
                if ((*ste).__slate_bits_1.__get_ste_comp_iter_target() as i32) != 0 {
                    // This name is an iteration variable in a comprehension,
                    // so check for a binding conflict with any named expressions.
                    // Otherwise, mark it as an iteration variable so subsequent
                    // named expressions can check for conflicts.
                    if val & 9i64 != 0 {
                        PyErr_Format(PyExc_SyntaxError, c"comprehension inner loop cannot rebind assignment expression target '%U'".as_ptr(), name);
                        SET_ERROR_LOCATION!(unsafe { (*st).st_filename }, loc);
                        break '__join_0;
                    } else {
                        val |= 512i64;
                    }
                }
                o = PyLong_FromLong(val);
                if !o.is_null() {
                    if PyDict_SetItem(dict, mangled, o) < 0 {
                        Py_DECREF!(o);
                    } else {
                        Py_DECREF!(o);
                        if flag & 4i32 != 0 {
                            if PyList_Append((*ste).ste_varnames, mangled) < 0 {
                                break '__join_0;
                            }
                        } else {
                            if flag & 1 != 0 {
                                // XXX need to update DEF_GLOBAL for other flags too;
                                // perhaps only DEF_FREE_GLOBAL
                                val = 0;
                                let __v3: *mut _object =
                                    PyDict_GetItemWithError((*st).st_global, mangled);
                                o = __v3;
                                if !__v3.is_null() {
                                    val = PyLong_AsLong(o);
                                    if val == (1i32.wrapping_neg() as i64) {
                                        __v2 = !PyErr_Occurred().is_null();
                                    } else {
                                        __v2 = false;
                                    }
                                    if __v2 {
                                        break '__join_0;
                                    }
                                } else {
                                    if !PyErr_Occurred().is_null() {
                                        break '__join_0;
                                    }
                                }
                                val |= flag as i64;
                                o = PyLong_FromLong(val);
                                if o.is_null() {
                                    break '__join_0;
                                } else {
                                    if PyDict_SetItem((*st).st_global, mangled, o) < 0 {
                                        Py_DECREF!(o);
                                        break '__join_0;
                                    } else {
                                        Py_DECREF!(o);
                                    }
                                }
                            }
                        }
                        Py_DECREF!(mangled);
                        return 1;
                    }
                }
            }
            Py_DECREF!(mangled);
            return 0;
        }
    }
}

fn check_name(
    st: *mut symtable,
    name: *mut _object,
    mut loc: __SlateRecord_pycore_symtable_h_51,
    ctx: u32,
) -> i32 {
    let __v0: bool = (ctx as u32) == 2
        && (unsafe { _PyUnicode_EqualToASCIIString(name, c"__debug__".as_ptr()) }) != 0;
    if __v0 {
        unsafe { PyErr_SetString(PyExc_SyntaxError, c"cannot assign to __debug__".as_ptr()) };
        SET_ERROR_LOCATION!(unsafe { (*st).st_filename }, loc);
        return 0;
    }
    let __v1: bool = (ctx as u32) == 3
        && (unsafe { _PyUnicode_EqualToASCIIString(name, c"__debug__".as_ptr()) }) != 0;
    if __v1 {
        unsafe { PyErr_SetString(PyExc_SyntaxError, c"cannot delete __debug__".as_ptr()) };
        SET_ERROR_LOCATION!(unsafe { (*st).st_filename }, loc);
        return 0;
    }
    1
}

fn check_keywords(st: *mut symtable, keywords: *mut __SlateRecord_pycore_ast_h_111) -> i32 {
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut i: i64 = 0;
    '__slate_break_2: while i < asdl_seq_LEN__7!(keywords) {
        let key: *mut _keyword = asdl_seq_GET__4!(keywords, i);
        let __v0: bool = !unsafe { (*key).arg }.is_null()
            && check_name(
                st,
                unsafe { (*key).arg },
                unsafe {
                    *{
                        __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*key).lineno,
                            end_lineno: (*key).end_lineno,
                            col_offset: (*key).col_offset,
                            end_col_offset: (*key).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    }
                },
                2u32,
            ) == 0;
        if __v0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn check_kwd_patterns(st: *mut symtable, p: *mut _pattern) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let kwd_attrs: *mut __SlateRecord_pycore_asdl_h_40 = unsafe { (*p).v.MatchClass.kwd_attrs };
    let kwd_patterns: *mut __SlateRecord_pycore_ast_h_140 =
        unsafe { (*p).v.MatchClass.kwd_patterns };
    let mut i: i64 = 0;
    '__slate_break_1: while i < asdl_seq_LEN__12!(kwd_attrs) {
        let loc: __SlateRecord_pycore_symtable_h_51 = unsafe {
            *{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*(*((*kwd_patterns).typed_elements.as_mut_ptr()
                        as *mut *mut _pattern)
                        .wrapping_offset(i as isize)))
                    .lineno,
                    end_lineno: (*(*((*kwd_patterns).typed_elements.as_mut_ptr()
                        as *mut *mut _pattern)
                        .wrapping_offset(i as isize)))
                    .end_lineno,
                    col_offset: (*(*((*kwd_patterns).typed_elements.as_mut_ptr()
                        as *mut *mut _pattern)
                        .wrapping_offset(i as isize)))
                    .col_offset,
                    end_col_offset: (*(*((*kwd_patterns).typed_elements.as_mut_ptr()
                        as *mut *mut _pattern)
                        .wrapping_offset(i as isize)))
                    .end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
        if check_name(st, asdl_seq_GET__12!(kwd_attrs, i), loc, 2u32) == 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn symtable_add_def_ctx(
    st: *mut symtable,
    name: *mut _object,
    flag: i32,
    loc: __SlateRecord_pycore_symtable_h_51,
    ctx: u32,
) -> i32 {
    let write_mask: i32 = 134i32;
    let __v0: bool = flag & write_mask != 0 && check_name(st, name, loc, ctx) == 0;
    if __v0 {
        return 0;
    }
    if flag & 1024i32 != 0 && !unsafe { (*(*st).st_cur).ste_mangled_names }.is_null() {
        if (unsafe { PySet_Add((*(*st).st_cur).ste_mangled_names, name) }) < 0 {
            return 0;
        }
    }
    symtable_add_def_helper(st, name, flag, unsafe { (*st).st_cur }, loc)
}

fn symtable_add_def(
    st: *mut symtable,
    name: *mut _object,
    flag: i32,
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    symtable_add_def_ctx(
        st,
        name,
        flag,
        loc,
        (if flag == 16i32 { 1i32 } else { 2i32 }) as u32,
    )
}

fn symtable_enter_type_param_block(
    st: *mut symtable,
    name: *mut _object,
    ast: *mut (),
    has_defaults: i32,
    has_kwdefaults: i32,
    kind: u32,
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    let mut current_type: u32 = unsafe { core::mem::zeroed() };
    unsafe {
        current_type = (*(*st).st_cur).ste_type;
        if symtable_enter_block(st, name, 5u32, ast, loc) == 0 {
            return 0;
        } else {
            '__join_0: {
                if (current_type as u32) == 1 {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_can_see_class_scope(1u32);
                    if symtable_add_def(
                        st,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py___classdict__
                                ._ascii
                                .ob_base
                        ),
                        16i32,
                        loc,
                    ) == 0
                    {
                        break '__join_0;
                    }
                }
                if (kind as u32) == 3 {
                    // It gets "set" when we create the type params tuple and
                    //
                    // "used" when we build up the bases.
                    if symtable_add_def(
                        st,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .literals
                                ._py_type_params
                                ._ascii
                                .ob_base
                        ),
                        2i32,
                        loc,
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if symtable_add_def(
                            st,
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .literals
                                    ._py_type_params
                                    ._ascii
                                    .ob_base
                            ),
                            16i32,
                            loc,
                        ) == 0
                        {
                            break '__join_0;
                        } else {
                            // This is used for setting the generic base
                            if symtable_add_def(
                                st,
                                core::ptr::addr_of_mut!(
                                    _PyRuntime
                                        .static_objects
                                        .singletons
                                        .strings
                                        .literals
                                        ._py_generic_base
                                        ._ascii
                                        .ob_base
                                ),
                                2i32,
                                loc,
                            ) == 0
                            {
                                break '__join_0;
                            } else {
                                if symtable_add_def(
                                    st,
                                    core::ptr::addr_of_mut!(
                                        _PyRuntime
                                            .static_objects
                                            .singletons
                                            .strings
                                            .literals
                                            ._py_generic_base
                                            ._ascii
                                            .ob_base
                                    ),
                                    16i32,
                                    loc,
                                ) == 0
                                {
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
                if has_defaults != 0 {
                    if symtable_add_def(
                        st,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .literals
                                ._py_defaults
                                ._ascii
                                .ob_base
                        ),
                        4i32,
                        loc,
                    ) == 0
                    {
                        break '__join_0;
                    }
                }
                if has_kwdefaults != 0 {
                    if symtable_add_def(
                        st,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .literals
                                ._py_kwdefaults
                                ._ascii
                                .ob_base
                        ),
                        4i32,
                        loc,
                    ) == 0
                    {
                        break '__join_0;
                    }
                }
                return 1;
            }
            symtable_exit_block(st);
            return 0;
        }
    }
}

// VISIT, VISIT_SEQ and VIST_SEQ_TAIL take an ASDL type as their second argument.
// They use the ASDL name to synthesize the name of the C type and the visit
// function.
//
// VISIT_SEQ_TAIL permits the start of an ASDL sequence to be skipped, which is
// useful if the first node in the sequence requires special treatment.
//
// ENTER_RECURSIVE macro increments the current recursion depth counter.
// It should be used at the beginning of the recursive function.
//
// LEAVE_RECURSIVE macro decrements the current recursion depth counter.
// It should be used at the end of the recursive function.
// avoid variable capture
// avoid variable capture
// avoid variable capture
// can be NULL
fn symtable_record_directive(
    st: *mut symtable,
    name: *mut _object,
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    let mut res: i32 = 0;
    if unsafe { (*(*st).st_cur).ste_directives }.is_null() {
        unsafe {
            (*(*st).st_cur).ste_directives = PyList_New(0i64);
        }
        if unsafe { (*(*st).st_cur).ste_directives }.is_null() {
            return 0;
        }
    }
    let mangled: *mut _object =
        _Py_MaybeMangle(unsafe { (*st).st_private }, unsafe { (*st).st_cur }, name);
    if mangled.is_null() {
        return 0;
    }
    let data: *mut _object = unsafe {
        Py_BuildValue(
            c"(Niiii)".as_ptr(),
            mangled,
            loc.lineno,
            loc.col_offset,
            loc.end_lineno,
            loc.end_col_offset,
        )
    };
    if data.is_null() {
        return 0;
    }
    res = unsafe { PyList_Append((*(*st).st_cur).ste_directives, data) };
    Py_DECREF!(data);
    (res == 0) as i32
}

fn has_kwonlydefaults(
    kwonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    kw_defaults: *mut __SlateRecord_pycore_ast_h_74,
) -> i32 {
    let mut i: i32 = 0;
    while (i as i64) < asdl_seq_LEN__4!(kwonlyargs) {
        let default_: *mut _expr = asdl_seq_GET!(kw_defaults, i);
        if !default_.is_null() {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}

fn check_import_from(st: *mut symtable, s: *mut _stmt) -> i32 {
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let fut: __SlateRecord_pycore_symtable_h_51 = unsafe { (*(*st).st_future).ff_location };
    let __v0: bool = !unsafe { (*s).v.ImportFrom.module }.is_null()
        && (unsafe { (*s).v.ImportFrom.level }) == 0
        && (unsafe {
            _PyUnicode_EqualToASCIIString((*s).v.ImportFrom.module, c"__future__".as_ptr())
        }) != 0;
    if __v0
        && ((unsafe { (*s).lineno }) > fut.lineno
            || (unsafe { (*s).lineno }) == fut.end_lineno
                && (unsafe { (*s).col_offset }) > fut.end_col_offset)
    {
        unsafe {
            PyErr_SetString(
                PyExc_SyntaxError,
                c"from __future__ imports must occur at the beginning of the file".as_ptr(),
            )
        };
        unsafe {
            PyErr_RangedSyntaxLocationObject(
                (*st).st_filename,
                (*{
                    __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*s).lineno,
                        end_lineno: (*s).end_lineno,
                        col_offset: (*s).col_offset,
                        end_col_offset: (*s).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                })
                .lineno,
                unsafe {
                    (*{
                        __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*s).lineno,
                            end_lineno: (*s).end_lineno,
                            col_offset: (*s).col_offset,
                            end_col_offset: (*s).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_2)
                    })
                    .col_offset
                }
                .wrapping_add(1),
                (*{
                    __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*s).lineno,
                        end_lineno: (*s).end_lineno,
                        col_offset: (*s).col_offset,
                        end_col_offset: (*s).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_3)
                })
                .end_lineno,
                unsafe {
                    (*{
                        __slate_compound_4 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*s).lineno,
                            end_lineno: (*s).end_lineno,
                            col_offset: (*s).col_offset,
                            end_col_offset: (*s).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_4)
                    })
                    .end_col_offset
                }
                .wrapping_add(1),
            )
        };
        return 0;
    }
    1
}

fn allows_top_level_await(st: *mut symtable) -> bool {
    (unsafe { (*(*st).st_future).ff_features }) & 8192 != 0
        && ((unsafe { (*(*st).st_cur).ste_type }) as u32) == 2
}

fn maybe_set_ste_coroutine_for_module(st: *mut symtable, s: *mut _stmt) {
    if allows_top_level_await(st) {
        unsafe {
            (*(*st).st_cur).__slate_bits_0.__set_ste_coroutine(1u32);
        }
    }
}

fn symtable_visit_stmt(st: *mut symtable, s: *mut _stmt) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_13: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_18: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_25: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_30: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_43: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_44: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_45: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_51: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_56: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_58: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_70: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_71: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_72: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_73: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_75: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_77: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_159: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_160: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_161: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_162: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_163: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_164: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_168: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_169: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_170: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_171: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_172: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_173: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_186: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_199: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_204: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_210: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_219: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    ENTER_RECURSIVE!();
    '__slate_break_24: {
        match (unsafe { (*s).kind }) as u32 {
            1 => {
                if symtable_add_def(st, unsafe { (*s).v.FunctionDef.name }, 2i32, unsafe {
                    *{
                        __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*s).lineno,
                            end_lineno: (*s).end_lineno,
                            col_offset: (*s).col_offset,
                            end_col_offset: (*s).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_0)
                    }
                }) == 0
                {
                    return 0;
                }
                if !unsafe { (*(*s).v.FunctionDef.args).defaults }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*(*s).v.FunctionDef.args).defaults };
                        while i < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*(*s).v.FunctionDef.args).kw_defaults }.is_null() {
                    loop {
                        let mut i: i32 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*(*s).v.FunctionDef.args).kw_defaults };
                        while (i as i64) < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if !elt.is_null() {
                                if symtable_visit_expr(st, elt) == 0 {
                                    return 0;
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*s).v.FunctionDef.decorator_list }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*s).v.FunctionDef.decorator_list };
                        while i < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if asdl_seq_LEN__6!(unsafe { (*s).v.FunctionDef.type_params }) > 0 {
                    if symtable_enter_type_param_block(
                        st,
                        unsafe { (*s).v.FunctionDef.name },
                        (unsafe { (*s).v.FunctionDef.type_params }) as *mut (),
                        !unsafe { (*(*s).v.FunctionDef.args).defaults }.is_null() as i32,
                        has_kwonlydefaults(
                            unsafe { (*(*s).v.FunctionDef.args).kwonlyargs },
                            unsafe { (*(*s).v.FunctionDef.args).kw_defaults },
                        ),
                        unsafe { (*s).kind },
                        unsafe {
                            *{
                                __slate_compound_13 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*s).lineno,
                                    end_lineno: (*s).end_lineno,
                                    col_offset: (*s).col_offset,
                                    end_col_offset: (*s).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_13)
                            }
                        },
                    ) == 0
                    {
                        return 0;
                    }
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_155 =
                            unsafe { (*s).v.FunctionDef.type_params };
                        while i < asdl_seq_LEN__6!(seq) {
                            let elt: *mut _type_param = asdl_seq_GET__7!(seq, i);
                            if symtable_visit_type_param(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                let new_ste: *mut _symtable_entry = ste_new(
                    st,
                    unsafe { (*s).v.FunctionDef.name },
                    0u32,
                    s as *mut (),
                    unsafe {
                        *{
                            __slate_compound_18 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_18)
                        }
                    },
                );
                if new_ste.is_null() {
                    return 0;
                }
                if !unsafe { _PyAST_GetDocString((*s).v.FunctionDef.body) }.is_null() {
                    unsafe {
                        (*new_ste).__slate_bits_1.__set_ste_has_docstring(1u32);
                    }
                }
                if symtable_visit_annotations(
                    st,
                    s,
                    unsafe { (*s).v.FunctionDef.args },
                    unsafe { (*s).v.FunctionDef.returns },
                    new_ste,
                ) == 0
                {
                    Py_DECREF__2!(new_ste);
                    return 0;
                }
                if symtable_enter_existing_block(st, new_ste, true) == 0 {
                    // add_to_children
                    Py_DECREF__2!(new_ste);
                    return 0;
                }
                Py_DECREF__2!(new_ste);
                loop {
                    if symtable_visit_arguments(st, unsafe { (*s).v.FunctionDef.args }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 =
                        unsafe { (*s).v.FunctionDef.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if symtable_exit_block(st) == 0 {
                    return 0;
                }
                if asdl_seq_LEN__6!(unsafe { (*s).v.FunctionDef.type_params }) > 0 {
                    if symtable_exit_block(st) == 0 {
                        return 0;
                    }
                }
            }
            3 => {
                if symtable_add_def(st, unsafe { (*s).v.ClassDef.name }, 2i32, unsafe {
                    *{
                        __slate_compound_25 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*s).lineno,
                            end_lineno: (*s).end_lineno,
                            col_offset: (*s).col_offset,
                            end_col_offset: (*s).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_25)
                    }
                }) == 0
                {
                    return 0;
                }
                if !unsafe { (*s).v.ClassDef.decorator_list }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*s).v.ClassDef.decorator_list };
                        while i < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                let tmp: *mut _object = unsafe { (*st).st_private };
                if asdl_seq_LEN__6!(unsafe { (*s).v.ClassDef.type_params }) > 0 {
                    if symtable_enter_type_param_block(
                        st,
                        unsafe { (*s).v.ClassDef.name },
                        (unsafe { (*s).v.ClassDef.type_params }) as *mut (),
                        0i32,
                        0i32,
                        unsafe { (*s).kind },
                        unsafe {
                            *{
                                __slate_compound_30 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*s).lineno,
                                    end_lineno: (*s).end_lineno,
                                    col_offset: (*s).col_offset,
                                    end_col_offset: (*s).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_30)
                            }
                        },
                    ) == 0
                    {
                        return 0;
                    }
                    unsafe {
                        (*st).st_private = (*s).v.ClassDef.name;
                    }
                    unsafe {
                        (*(*st).st_cur).ste_mangled_names =
                            PySet_New(core::ptr::null_mut::<_object>());
                    }
                    if unsafe { (*(*st).st_cur).ste_mangled_names }.is_null() {
                        return 0;
                    }
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_155 =
                            unsafe { (*s).v.ClassDef.type_params };
                        while i < asdl_seq_LEN__6!(seq) {
                            let elt: *mut _type_param = asdl_seq_GET__7!(seq, i);
                            if symtable_visit_type_param(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*s).v.ClassDef.bases };
                    while i < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if symtable_visit_expr(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if check_keywords(st, unsafe { (*s).v.ClassDef.keywords }) == 0 {
                    return 0;
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_111 =
                        unsafe { (*s).v.ClassDef.keywords };
                    while i < asdl_seq_LEN__7!(seq) {
                        let elt: *mut _keyword = asdl_seq_GET__4!(seq, i);
                        if symtable_visit_keyword(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if symtable_enter_block(
                    st,
                    unsafe { (*s).v.ClassDef.name },
                    1u32,
                    s as *mut (),
                    unsafe {
                        *{
                            __slate_compound_43 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_43)
                        }
                    },
                ) == 0
                {
                    return 0;
                }
                unsafe {
                    (*st).st_private = (*s).v.ClassDef.name;
                }
                if asdl_seq_LEN__6!(unsafe { (*s).v.ClassDef.type_params }) > 0 {
                    if symtable_add_def(
                        st,
                        unsafe {
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py___type_params__
                                    ._ascii
                                    .ob_base
                            )
                        },
                        2i32,
                        unsafe {
                            *{
                                __slate_compound_44 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*s).lineno,
                                    end_lineno: (*s).end_lineno,
                                    col_offset: (*s).col_offset,
                                    end_col_offset: (*s).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_44)
                            }
                        },
                    ) == 0
                    {
                        return 0;
                    }
                    if symtable_add_def(
                        st,
                        unsafe {
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .literals
                                    ._py_type_params
                                    ._ascii
                                    .ob_base
                            )
                        },
                        16i32,
                        unsafe {
                            *{
                                __slate_compound_45 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*s).lineno,
                                    end_lineno: (*s).end_lineno,
                                    col_offset: (*s).col_offset,
                                    end_col_offset: (*s).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_45)
                            }
                        },
                    ) == 0
                    {
                        return 0;
                    }
                }
                if !unsafe { _PyAST_GetDocString((*s).v.ClassDef.body) }.is_null() {
                    unsafe {
                        (*(*st).st_cur).__slate_bits_1.__set_ste_has_docstring(1u32);
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.ClassDef.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if symtable_exit_block(st) == 0 {
                    return 0;
                }
                if asdl_seq_LEN__6!(unsafe { (*s).v.ClassDef.type_params }) > 0 {
                    if symtable_exit_block(st) == 0 {
                        return 0;
                    }
                }
                unsafe {
                    (*st).st_private = tmp;
                }
            }
            7 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.TypeAlias.name }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                let name: *mut _object = unsafe { (*(*s).v.TypeAlias.name).v.Name.id };
                let is_in_class: i32 = (((unsafe { (*(*st).st_cur).ste_type }) as u32) == 1) as i32;
                let is_generic: i32 =
                    (asdl_seq_LEN__6!(unsafe { (*s).v.TypeAlias.type_params }) > 0) as i32;
                if is_generic != 0 {
                    if symtable_enter_type_param_block(
                        st,
                        name,
                        (unsafe { (*s).v.TypeAlias.type_params }) as *mut (),
                        0i32,
                        0i32,
                        unsafe { (*s).kind },
                        unsafe {
                            *{
                                __slate_compound_51 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*s).lineno,
                                    end_lineno: (*s).end_lineno,
                                    col_offset: (*s).col_offset,
                                    end_col_offset: (*s).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_51)
                            }
                        },
                    ) == 0
                    {
                        return 0;
                    }
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_155 =
                            unsafe { (*s).v.TypeAlias.type_params };
                        while i < asdl_seq_LEN__6!(seq) {
                            let elt: *mut _type_param = asdl_seq_GET__7!(seq, i);
                            if symtable_visit_type_param(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if symtable_enter_block(st, name, 4u32, s as *mut (), unsafe {
                    *{
                        __slate_compound_56 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*s).lineno,
                            end_lineno: (*s).end_lineno,
                            col_offset: (*s).col_offset,
                            end_col_offset: (*s).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_56)
                    }
                }) == 0
                {
                    return 0;
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_can_see_class_scope(is_in_class as u32);
                }
                let __v57: bool = is_in_class != 0
                    && symtable_add_def(
                        st,
                        unsafe {
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py___classdict__
                                    ._ascii
                                    .ob_base
                            )
                        },
                        16i32,
                        unsafe {
                            *{
                                __slate_compound_58 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*(*s).v.TypeAlias.value).lineno,
                                    end_lineno: (*(*s).v.TypeAlias.value).end_lineno,
                                    col_offset: (*(*s).v.TypeAlias.value).col_offset,
                                    end_col_offset: (*(*s).v.TypeAlias.value).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_58)
                            }
                        },
                    ) == 0;
                if __v57 {
                    return 0;
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.TypeAlias.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                if symtable_exit_block(st) == 0 {
                    return 0;
                }
                if is_generic != 0 {
                    if symtable_exit_block(st) == 0 {
                        return 0;
                    }
                }
            }
            4 => {
                if !unsafe { (*s).v.Return.value }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*s).v.Return.value }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                    unsafe {
                        (*(*st).st_cur).__slate_bits_1.__set_ste_returns_value(1u32);
                    }
                }
            }
            5 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*s).v.Delete.targets };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            6 => {
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*s).v.Assign.targets };
                    while i < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if symtable_visit_expr(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.Assign.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
            }
            9 => {
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_0
                        .__set_ste_annotations_used(1u32);
                }
                if ((unsafe { (*(*s).v.AnnAssign.target).kind }) as u32) == 26 {
                    let e_name: *mut _expr = unsafe { (*s).v.AnnAssign.target };
                    let cur: i64 = symtable_lookup(st, unsafe { (*e_name).v.Name.id });
                    if cur < 0 {
                        return 0;
                    }
                    if cur & 9i64 != 0
                        && (unsafe { (*(*st).st_cur).ste_symbols }) != unsafe { (*st).st_global }
                        && (unsafe { (*s).v.AnnAssign.simple }) != 0
                    {
                        unsafe {
                            PyErr_Format(
                                PyExc_SyntaxError,
                                (if cur & 1 != 0 {
                                    c"annotated name '%U' can't be global".as_ptr() as *mut i8
                                } else {
                                    c"annotated name '%U' can't be nonlocal".as_ptr() as *mut i8
                                }) as *const i8,
                                (*e_name).v.Name.id,
                            )
                        };
                        unsafe {
                            PyErr_RangedSyntaxLocationObject(
                                (*st).st_filename,
                                (*{
                                    __slate_compound_70 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*s).lineno,
                                        end_lineno: (*s).end_lineno,
                                        col_offset: (*s).col_offset,
                                        end_col_offset: (*s).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_70)
                                })
                                .lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_71 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*s).lineno,
                                            end_lineno: (*s).end_lineno,
                                            col_offset: (*s).col_offset,
                                            end_col_offset: (*s).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_71)
                                    })
                                    .col_offset
                                }
                                .wrapping_add(1),
                                (*{
                                    __slate_compound_72 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*s).lineno,
                                        end_lineno: (*s).end_lineno,
                                        col_offset: (*s).col_offset,
                                        end_col_offset: (*s).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_72)
                                })
                                .end_lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_73 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*s).lineno,
                                            end_lineno: (*s).end_lineno,
                                            col_offset: (*s).col_offset,
                                            end_col_offset: (*s).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_73)
                                    })
                                    .end_col_offset
                                }
                                .wrapping_add(1),
                            )
                        };
                        return 0;
                    }
                    let __v74: bool = (unsafe { (*s).v.AnnAssign.simple }) != 0
                        && symtable_add_def(st, unsafe { (*e_name).v.Name.id }, 258i32, unsafe {
                            *{
                                __slate_compound_75 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*e_name).lineno,
                                    end_lineno: (*e_name).end_lineno,
                                    col_offset: (*e_name).col_offset,
                                    end_col_offset: (*e_name).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_75)
                            }
                        }) == 0;
                    if __v74 {
                        return 0;
                    } else {
                        let __v76: bool = !unsafe { (*s).v.AnnAssign.value }.is_null()
                            && symtable_add_def(st, unsafe { (*e_name).v.Name.id }, 2i32, unsafe {
                                *{
                                    __slate_compound_77 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*e_name).lineno,
                                        end_lineno: (*e_name).end_lineno,
                                        col_offset: (*e_name).col_offset,
                                        end_col_offset: (*e_name).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_77)
                                }
                            }) == 0;
                        if __v76 {
                            return 0;
                        }
                    }
                } else {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*s).v.AnnAssign.target }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
                if symtable_visit_annotation(
                    st,
                    unsafe { (*s).v.AnnAssign.annotation },
                    ((unsafe { (*(*st).st_cur).ste_id }) as u64).wrapping_add(1) as *mut (),
                ) == 0
                {
                    return 0;
                }
                if !unsafe { (*s).v.AnnAssign.value }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*s).v.AnnAssign.value }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
            }
            8 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.AugAssign.target }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.AugAssign.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
            }
            10 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.For.target }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.For.iter }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.For.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*s).v.For.orelse }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.For.orelse };
                        while i < asdl_seq_LEN__2!(seq) {
                            let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                            if symtable_visit_stmt(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            12 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.While.test }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.While.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*s).v.While.orelse }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_67 =
                            unsafe { (*s).v.While.orelse };
                        while i < asdl_seq_LEN__2!(seq) {
                            let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                            if symtable_visit_stmt(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            13 => {
                // XXX if 0: and lookup_yield() hacks
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.If.test }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.If.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*s).v.If.orelse }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.If.orelse };
                        while i < asdl_seq_LEN__2!(seq) {
                            let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                            if symtable_visit_stmt(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            16 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.Match.subject }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_132 = unsafe { (*s).v.Match.cases };
                    while i < asdl_seq_LEN__14!(seq) {
                        let elt: *mut _match_case = asdl_seq_GET__13!(seq, i);
                        if symtable_visit_match_case(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            17 => {
                if !unsafe { (*s).v.Raise.exc }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*s).v.Raise.exc }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                    if !unsafe { (*s).v.Raise.cause }.is_null() {
                        loop {
                            if symtable_visit_expr(st, unsafe { (*s).v.Raise.cause }) == 0 {
                                return 0;
                            }
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            18 => {
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.Try.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_89 = unsafe { (*s).v.Try.handlers };
                    while i < asdl_seq_LEN__8!(seq) {
                        let elt: *mut _excepthandler = asdl_seq_GET__10!(seq, i);
                        if symtable_visit_excepthandler(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.Try.orelse };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.Try.finalbody };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            19 => {
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.TryStar.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_89 =
                        unsafe { (*s).v.TryStar.handlers };
                    while i < asdl_seq_LEN__8!(seq) {
                        let elt: *mut _excepthandler = asdl_seq_GET__10!(seq, i);
                        if symtable_visit_excepthandler(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.TryStar.orelse };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 =
                        unsafe { (*s).v.TryStar.finalbody };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            20 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.Assert.test }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*s).v.Assert.msg }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*s).v.Assert.msg }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
            }
            21 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_118 = unsafe { (*s).v.Import.names };
                while i < asdl_seq_LEN__11!(seq) {
                    let elt: *mut _alias = asdl_seq_GET__9!(seq, i);
                    if symtable_visit_alias(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            22 => {
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_118 =
                        unsafe { (*s).v.ImportFrom.names };
                    while i < asdl_seq_LEN__11!(seq) {
                        let elt: *mut _alias = asdl_seq_GET__9!(seq, i);
                        if symtable_visit_alias(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if check_import_from(st, s) == 0 {
                    return 0;
                }
            }
            23 => {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_asdl_h_40 = unsafe { (*s).v.Global.names };
                '__slate_break_165: while i < asdl_seq_LEN__12!(seq) {
                    let name: *mut _object = asdl_seq_GET__12!(seq, i);
                    let cur: i64 = symtable_lookup(st, name);
                    if cur < 0 {
                        return 0;
                    }
                    if cur & 278i64 != 0 {
                        let mut msg: *const i8 = unsafe { core::mem::zeroed() };
                        if cur & 4i64 != 0 {
                            msg = c"name '%U' is parameter and global".as_ptr();
                        } else {
                            if cur & 16i64 != 0 {
                                msg = c"name '%U' is used prior to global declaration".as_ptr();
                            } else {
                                if cur & 256i64 != 0 {
                                    msg = c"annotated name '%U' can't be global".as_ptr();
                                } else {
                                    // DEF_LOCAL
                                    msg = c"name '%U' is assigned to before global declaration"
                                        .as_ptr();
                                }
                            }
                        }
                        unsafe { PyErr_Format(PyExc_SyntaxError, msg, name) };
                        unsafe {
                            PyErr_RangedSyntaxLocationObject(
                                (*st).st_filename,
                                (*{
                                    __slate_compound_159 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*s).lineno,
                                        end_lineno: (*s).end_lineno,
                                        col_offset: (*s).col_offset,
                                        end_col_offset: (*s).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_159)
                                })
                                .lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_160 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*s).lineno,
                                            end_lineno: (*s).end_lineno,
                                            col_offset: (*s).col_offset,
                                            end_col_offset: (*s).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_160)
                                    })
                                    .col_offset
                                }
                                .wrapping_add(1),
                                (*{
                                    __slate_compound_161 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*s).lineno,
                                        end_lineno: (*s).end_lineno,
                                        col_offset: (*s).col_offset,
                                        end_col_offset: (*s).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_161)
                                })
                                .end_lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_162 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*s).lineno,
                                            end_lineno: (*s).end_lineno,
                                            col_offset: (*s).col_offset,
                                            end_col_offset: (*s).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_162)
                                    })
                                    .end_col_offset
                                }
                                .wrapping_add(1),
                            )
                        };
                        return 0;
                    }
                    if symtable_add_def(st, name, 1i32, unsafe {
                        *{
                            __slate_compound_163 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_163)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                    if symtable_record_directive(st, name, unsafe {
                        *{
                            __slate_compound_164 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_164)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
            }
            24 => {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_asdl_h_40 = unsafe { (*s).v.Nonlocal.names };
                '__slate_break_174: while i < asdl_seq_LEN__12!(seq) {
                    let name: *mut _object = asdl_seq_GET__12!(seq, i);
                    let cur: i64 = symtable_lookup(st, name);
                    if cur < 0 {
                        return 0;
                    }
                    if cur & 278i64 != 0 {
                        let mut msg: *const i8 = unsafe { core::mem::zeroed() };
                        if cur & 4i64 != 0 {
                            msg = c"name '%U' is parameter and nonlocal".as_ptr();
                        } else {
                            if cur & 16i64 != 0 {
                                msg = c"name '%U' is used prior to nonlocal declaration".as_ptr();
                            } else {
                                if cur & 256i64 != 0 {
                                    msg = c"annotated name '%U' can't be nonlocal".as_ptr();
                                } else {
                                    // DEF_LOCAL
                                    msg = c"name '%U' is assigned to before nonlocal declaration"
                                        .as_ptr();
                                }
                            }
                        }
                        unsafe { PyErr_Format(PyExc_SyntaxError, msg, name) };
                        unsafe {
                            PyErr_RangedSyntaxLocationObject(
                                (*st).st_filename,
                                (*{
                                    __slate_compound_168 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*s).lineno,
                                        end_lineno: (*s).end_lineno,
                                        col_offset: (*s).col_offset,
                                        end_col_offset: (*s).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_168)
                                })
                                .lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_169 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*s).lineno,
                                            end_lineno: (*s).end_lineno,
                                            col_offset: (*s).col_offset,
                                            end_col_offset: (*s).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_169)
                                    })
                                    .col_offset
                                }
                                .wrapping_add(1),
                                (*{
                                    __slate_compound_170 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*s).lineno,
                                        end_lineno: (*s).end_lineno,
                                        col_offset: (*s).col_offset,
                                        end_col_offset: (*s).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_170)
                                })
                                .end_lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_171 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*s).lineno,
                                            end_lineno: (*s).end_lineno,
                                            col_offset: (*s).col_offset,
                                            end_col_offset: (*s).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_171)
                                    })
                                    .end_col_offset
                                }
                                .wrapping_add(1),
                            )
                        };
                        return 0;
                    }
                    if symtable_add_def(st, name, 8i32, unsafe {
                        *{
                            __slate_compound_172 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_172)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                    if symtable_record_directive(st, name, unsafe {
                        *{
                            __slate_compound_173 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_173)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
            }
            25 => loop {
                if symtable_visit_expr(st, unsafe { (*s).v.Expr.value }) == 0 {
                    return 0;
                }
                if true {
                    break;
                }
            },
            26..=28 => {
                break '__slate_break_24; // nothing to do here
            }
            14 => {
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_125 = unsafe { (*s).v.With.items };
                    while i < asdl_seq_LEN__10!(seq) {
                        let elt: *mut _withitem = asdl_seq_GET__11!(seq, i);
                        if symtable_visit_withitem(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.With.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            2 => {
                if symtable_add_def(st, unsafe { (*s).v.AsyncFunctionDef.name }, 2i32, unsafe {
                    *{
                        __slate_compound_186 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*s).lineno,
                            end_lineno: (*s).end_lineno,
                            col_offset: (*s).col_offset,
                            end_col_offset: (*s).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_186)
                    }
                }) == 0
                {
                    return 0;
                }
                if !unsafe { (*(*s).v.AsyncFunctionDef.args).defaults }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*(*s).v.AsyncFunctionDef.args).defaults };
                        while i < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*(*s).v.AsyncFunctionDef.args).kw_defaults }.is_null() {
                    loop {
                        let mut i: i32 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*(*s).v.AsyncFunctionDef.args).kw_defaults };
                        while (i as i64) < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if !elt.is_null() {
                                if symtable_visit_expr(st, elt) == 0 {
                                    return 0;
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*s).v.AsyncFunctionDef.decorator_list }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*s).v.AsyncFunctionDef.decorator_list };
                        while i < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if asdl_seq_LEN__6!(unsafe { (*s).v.AsyncFunctionDef.type_params }) > 0 {
                    if symtable_enter_type_param_block(
                        st,
                        unsafe { (*s).v.AsyncFunctionDef.name },
                        (unsafe { (*s).v.AsyncFunctionDef.type_params }) as *mut (),
                        !unsafe { (*(*s).v.AsyncFunctionDef.args).defaults }.is_null() as i32,
                        has_kwonlydefaults(
                            unsafe { (*(*s).v.AsyncFunctionDef.args).kwonlyargs },
                            unsafe { (*(*s).v.AsyncFunctionDef.args).kw_defaults },
                        ),
                        unsafe { (*s).kind },
                        unsafe {
                            *{
                                __slate_compound_199 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*s).lineno,
                                    end_lineno: (*s).end_lineno,
                                    col_offset: (*s).col_offset,
                                    end_col_offset: (*s).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_199)
                            }
                        },
                    ) == 0
                    {
                        return 0;
                    }
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_155 =
                            unsafe { (*s).v.AsyncFunctionDef.type_params };
                        while i < asdl_seq_LEN__6!(seq) {
                            let elt: *mut _type_param = asdl_seq_GET__7!(seq, i);
                            if symtable_visit_type_param(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                let new_ste: *mut _symtable_entry = ste_new(
                    st,
                    unsafe { (*s).v.FunctionDef.name },
                    0u32,
                    s as *mut (),
                    unsafe {
                        *{
                            __slate_compound_204 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_204)
                        }
                    },
                );
                if new_ste.is_null() {
                    return 0;
                }
                if !unsafe { _PyAST_GetDocString((*s).v.AsyncFunctionDef.body) }.is_null() {
                    unsafe {
                        (*new_ste).__slate_bits_1.__set_ste_has_docstring(1u32);
                    }
                }
                if symtable_visit_annotations(
                    st,
                    s,
                    unsafe { (*s).v.AsyncFunctionDef.args },
                    unsafe { (*s).v.AsyncFunctionDef.returns },
                    new_ste,
                ) == 0
                {
                    Py_DECREF__2!(new_ste);
                    return 0;
                }
                if symtable_enter_existing_block(st, new_ste, true) == 0 {
                    // add_to_children
                    Py_DECREF__2!(new_ste);
                    return 0;
                }
                Py_DECREF__2!(new_ste);
                unsafe {
                    (*(*st).st_cur).__slate_bits_0.__set_ste_coroutine(1u32);
                }
                loop {
                    if symtable_visit_arguments(st, unsafe { (*s).v.AsyncFunctionDef.args }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 =
                        unsafe { (*s).v.AsyncFunctionDef.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if symtable_exit_block(st) == 0 {
                    return 0;
                }
                if asdl_seq_LEN__6!(unsafe { (*s).v.AsyncFunctionDef.type_params }) > 0 {
                    if symtable_exit_block(st) == 0 {
                        return 0;
                    }
                }
            }
            15 => {
                maybe_set_ste_coroutine_for_module(st, s);
                if symtable_raise_if_not_coroutine(
                    st,
                    c"'async with' outside async function".as_ptr(),
                    unsafe {
                        *{
                            __slate_compound_210 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_210)
                        }
                    },
                ) == 0
                {
                    return 0;
                }
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_125 =
                        unsafe { (*s).v.AsyncWith.items };
                    while i < asdl_seq_LEN__10!(seq) {
                        let elt: *mut _withitem = asdl_seq_GET__11!(seq, i);
                        if symtable_visit_withitem(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.AsyncWith.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            11 => {
                maybe_set_ste_coroutine_for_module(st, s);
                if symtable_raise_if_not_coroutine(
                    st,
                    c"'async for' outside async function".as_ptr(),
                    unsafe {
                        *{
                            __slate_compound_219 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*s).lineno,
                                end_lineno: (*s).end_lineno,
                                col_offset: (*s).col_offset,
                                end_col_offset: (*s).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_219)
                        }
                    },
                ) == 0
                {
                    return 0;
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.AsyncFor.target }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*s).v.AsyncFor.iter }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                let in_conditional_block: i32 = (unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_conditional_block()
                }) as i32;
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(1u32);
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*s).v.AsyncFor.body };
                    while i < asdl_seq_LEN__2!(seq) {
                        let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                        if symtable_visit_stmt(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*s).v.AsyncFor.orelse }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_67 =
                            unsafe { (*s).v.AsyncFor.orelse };
                        while i < asdl_seq_LEN__2!(seq) {
                            let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
                            if symtable_visit_stmt(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__set_ste_in_conditional_block(in_conditional_block as u32);
                }
            }
            _ => {}
        }
    }
    LEAVE_RECURSIVE!();
    1
}

fn symtable_extend_namedexpr_scope(st: *mut symtable, e: *mut _expr) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_5: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_6: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_7: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_8: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_9: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_10: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_11: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_13: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_14: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_15: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_16: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let target_name: *mut _object = unsafe { (*e).v.Name.id };
    let mut i: i64 = 0;
    let size: i64 = PyList_GET_SIZE!(unsafe { (*st).st_stack });
    // Iterate over the stack in reverse and add to the nearest adequate scope
    i = size.wrapping_sub(1);
    '__slate_break_4: while i >= 0 {
        let ste: *mut _symtable_entry =
            PyList_GET_ITEM!(unsafe { (*st).st_stack }, i) as *mut _symtable_entry;
        // If we find a comprehension scope, check for a target
        // binding conflict with iteration variables, otherwise skip it
        if ((unsafe { (*ste).ste_comprehension }) as u32) != 0 {
            let target_in_scope: i64 = symtable_lookup_entry(st, ste, target_name);
            if target_in_scope < 0 {
                return 0;
            }
            if target_in_scope & 512i64 != 0 && target_in_scope & 2 != 0 {
                unsafe {
                    PyErr_Format(PyExc_SyntaxError, c"assignment expression cannot rebind comprehension iteration variable '%U'".as_ptr(), target_name)
                };
                unsafe {
                    PyErr_RangedSyntaxLocationObject(
                        (*st).st_filename,
                        (*{
                            __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_0)
                        })
                        .lineno,
                        unsafe {
                            (*{
                                __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*e).lineno,
                                    end_lineno: (*e).end_lineno,
                                    col_offset: (*e).col_offset,
                                    end_col_offset: (*e).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_1)
                            })
                            .col_offset
                        }
                        .wrapping_add(1),
                        (*{
                            __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_2)
                        })
                        .end_lineno,
                        unsafe {
                            (*{
                                __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*e).lineno,
                                    end_lineno: (*e).end_lineno,
                                    col_offset: (*e).col_offset,
                                    end_col_offset: (*e).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_3)
                            })
                            .end_col_offset
                        }
                        .wrapping_add(1),
                    )
                };
                return 0;
            }
        } else {
            // If we find a FunctionBlock entry, add as GLOBAL/LOCAL or NONLOCAL/LOCAL
            if ((unsafe { (*ste).ste_type }) as u32) == 0 {
                let target_in_scope: i64 = symtable_lookup_entry(st, ste, target_name);
                if target_in_scope < 0 {
                    return 0;
                }
                if target_in_scope & 1 != 0 {
                    if symtable_add_def(st, target_name, 1i32, unsafe {
                        *{
                            __slate_compound_5 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_5)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                } else {
                    if symtable_add_def(st, target_name, 8i32, unsafe {
                        *{
                            __slate_compound_6 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_6)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                }
                if symtable_record_directive(st, target_name, unsafe {
                    *{
                        __slate_compound_7 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_7)
                    }
                }) == 0
                {
                    return 0;
                }
                return symtable_add_def_helper(st, target_name, 2i32, ste, unsafe {
                    *{
                        __slate_compound_8 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_8)
                    }
                });
            }
            // If we find a ModuleBlock entry, add as GLOBAL
            if ((unsafe { (*ste).ste_type }) as u32) == 2 {
                if symtable_add_def(st, target_name, 1i32, unsafe {
                    *{
                        __slate_compound_9 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_9)
                    }
                }) == 0
                {
                    return 0;
                }
                if symtable_record_directive(st, target_name, unsafe {
                    *{
                        __slate_compound_10 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_10)
                    }
                }) == 0
                {
                    return 0;
                }
                return symtable_add_def_helper(st, target_name, 1i32, ste, unsafe {
                    *{
                        __slate_compound_11 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_11)
                    }
                });
            }
            // Disallow usage in ClassBlock and type scopes
            if ((unsafe { (*ste).ste_type }) as u32) == 1
                || ((unsafe { (*ste).ste_type }) as u32) == 5
                || ((unsafe { (*ste).ste_type }) as u32) == 4
                || ((unsafe { (*ste).ste_type }) as u32) == 6
            {
                match (unsafe { (*ste).ste_type }) as u32 {
                    1 => {
                        unsafe {
                            PyErr_Format(PyExc_SyntaxError, c"assignment expression within a comprehension cannot be used in a class body".as_ptr())
                        };
                    }
                    5 => {
                        unsafe {
                            PyErr_Format(PyExc_SyntaxError, c"assignment expression within a comprehension cannot be used within the definition of a generic".as_ptr())
                        };
                    }
                    4 => {
                        unsafe {
                            PyErr_Format(PyExc_SyntaxError, c"assignment expression within a comprehension cannot be used in a type alias".as_ptr())
                        };
                    }
                    6 => {
                        unsafe {
                            PyErr_Format(PyExc_SyntaxError, c"assignment expression within a comprehension cannot be used in a TypeVar bound".as_ptr())
                        };
                    }
                    _ => {
                        Py_UNREACHABLE!();
                    }
                }
                unsafe {
                    PyErr_RangedSyntaxLocationObject(
                        (*st).st_filename,
                        (*{
                            __slate_compound_13 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_13)
                        })
                        .lineno,
                        unsafe {
                            (*{
                                __slate_compound_14 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*e).lineno,
                                    end_lineno: (*e).end_lineno,
                                    col_offset: (*e).col_offset,
                                    end_col_offset: (*e).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_14)
                            })
                            .col_offset
                        }
                        .wrapping_add(1),
                        (*{
                            __slate_compound_15 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_15)
                        })
                        .end_lineno,
                        unsafe {
                            (*{
                                __slate_compound_16 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*e).lineno,
                                    end_lineno: (*e).end_lineno,
                                    col_offset: (*e).col_offset,
                                    end_col_offset: (*e).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_16)
                            })
                            .end_col_offset
                        }
                        .wrapping_add(1),
                    )
                };
                return 0;
            }
        }
        i = i.wrapping_sub(1);
    }
    // We should always find either a function-like block, ModuleBlock or ClassBlock
    // and should never fall to this case
    Py_UNREACHABLE!();
    0
}

fn symtable_handle_namedexpr(st: *mut symtable, e: *mut _expr) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    if (unsafe { (*(*st).st_cur).ste_comp_iter_expr }) > 0 {
        // Assignment isn't allowed in a comprehension iterable expression
        unsafe {
            PyErr_Format(
                PyExc_SyntaxError,
                c"assignment expression cannot be used in a comprehension iterable expression"
                    .as_ptr(),
            )
        };
        unsafe {
            PyErr_RangedSyntaxLocationObject(
                (*st).st_filename,
                (*{
                    __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                })
                .lineno,
                unsafe {
                    (*{
                        __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    })
                    .col_offset
                }
                .wrapping_add(1),
                (*{
                    __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                })
                .end_lineno,
                unsafe {
                    (*{
                        __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_3)
                    })
                    .end_col_offset
                }
                .wrapping_add(1),
            )
        };
        return 0;
    }
    if ((unsafe { (*(*st).st_cur).ste_comprehension }) as u32) != 0 {
        // Inside a comprehension body, so find the right target scope
        if symtable_extend_namedexpr_scope(st, unsafe { (*e).v.NamedExpr.target }) == 0 {
            return 0;
        }
    }
    loop {
        if symtable_visit_expr(st, unsafe { (*e).v.NamedExpr.value }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    loop {
        if symtable_visit_expr(st, unsafe { (*e).v.NamedExpr.target }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    1
}

fn symtable_visit_expr(st: *mut symtable, e: *mut _expr) -> i32 {
    let mut __slate_compound_16: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_36: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_37: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_38: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_39: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_40: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_41: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_42: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_43: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_71: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_79: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_82: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    ENTER_RECURSIVE!();
    '__slate_break_0: {
        match (unsafe { (*e).kind }) as u32 {
            2 => {
                if symtable_raise_if_annotation_block(st, c"named expression".as_ptr(), e) == 0 {
                    return 0;
                }
                if symtable_handle_namedexpr(st, e) == 0 {
                    return 0;
                }
            }
            1 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.BoolOp.values };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            3 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.BinOp.left }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.BinOp.right }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
            }
            4 => loop {
                if symtable_visit_expr(st, unsafe { (*e).v.UnaryOp.operand }) == 0 {
                    return 0;
                }
                if true {
                    break;
                }
            },
            5 => {
                if !unsafe { (*(*e).v.Lambda.args).defaults }.is_null() {
                    loop {
                        let mut i: i64 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*(*e).v.Lambda.args).defaults };
                        while i < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*(*e).v.Lambda.args).kw_defaults }.is_null() {
                    loop {
                        let mut i: i32 = 0;
                        let seq: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*(*e).v.Lambda.args).kw_defaults };
                        while (i as i64) < asdl_seq_LEN!(seq) {
                            let elt: *mut _expr = asdl_seq_GET!(seq, i);
                            if !elt.is_null() {
                                if symtable_visit_expr(st, elt) == 0 {
                                    return 0;
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        if true {
                            break;
                        }
                    }
                }
                if symtable_enter_block(
                    st,
                    unsafe {
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py_lambda
                                ._ascii
                                .ob_base
                        )
                    },
                    0u32,
                    e as *mut (),
                    unsafe {
                        *{
                            __slate_compound_16 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_16)
                        }
                    },
                ) == 0
                {
                    return 0;
                }
                loop {
                    if symtable_visit_arguments(st, unsafe { (*e).v.Lambda.args }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Lambda.body }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                if symtable_exit_block(st) == 0 {
                    return 0;
                }
            }
            6 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.IfExp.test }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.IfExp.body }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.IfExp.orelse }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
            }
            7 => {
                loop {
                    let mut i: i32 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.Dict.keys };
                    while (i as i64) < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if !elt.is_null() {
                            if symtable_visit_expr(st, elt) == 0 {
                                return 0;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.Dict.values };
                    while i < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if symtable_visit_expr(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
            }
            8 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.Set.elts };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            12 => {
                if symtable_visit_genexp(st, e) == 0 {
                    return 0;
                }
            }
            9 => {
                if symtable_visit_listcomp(st, e) == 0 {
                    return 0;
                }
            }
            10 => {
                if symtable_visit_setcomp(st, e) == 0 {
                    return 0;
                }
            }
            11 => {
                if symtable_visit_dictcomp(st, e) == 0 {
                    return 0;
                }
            }
            14 => {
                if symtable_raise_if_annotation_block(st, c"yield expression".as_ptr(), e) == 0 {
                    return 0;
                }
                if !unsafe { (*e).v.Yield.value }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*e).v.Yield.value }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
                unsafe {
                    (*(*st).st_cur).__slate_bits_0.__set_ste_generator(1u32);
                }
                if ((unsafe { (*(*st).st_cur).ste_comprehension }) as u32) != 0 {
                    return symtable_raise_if_comprehension_block(st, e);
                }
            }
            15 => {
                if symtable_raise_if_annotation_block(st, c"yield expression".as_ptr(), e) == 0 {
                    return 0;
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.YieldFrom.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur).__slate_bits_0.__set_ste_generator(1u32);
                }
                if ((unsafe { (*(*st).st_cur).ste_comprehension }) as u32) != 0 {
                    return symtable_raise_if_comprehension_block(st, e);
                }
            }
            13 => {
                if symtable_raise_if_annotation_block(st, c"await expression".as_ptr(), e) == 0 {
                    return 0;
                }
                if !allows_top_level_await(st) {
                    if _PyST_IsFunctionLike(unsafe { (*st).st_cur }) == 0 {
                        unsafe {
                            PyErr_SetString(PyExc_SyntaxError, c"'await' outside function".as_ptr())
                        };
                        unsafe {
                            PyErr_RangedSyntaxLocationObject(
                                (*st).st_filename,
                                (*{
                                    __slate_compound_36 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*e).lineno,
                                        end_lineno: (*e).end_lineno,
                                        col_offset: (*e).col_offset,
                                        end_col_offset: (*e).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_36)
                                })
                                .lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_37 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*e).lineno,
                                            end_lineno: (*e).end_lineno,
                                            col_offset: (*e).col_offset,
                                            end_col_offset: (*e).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_37)
                                    })
                                    .col_offset
                                }
                                .wrapping_add(1),
                                (*{
                                    __slate_compound_38 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*e).lineno,
                                        end_lineno: (*e).end_lineno,
                                        col_offset: (*e).col_offset,
                                        end_col_offset: (*e).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_38)
                                })
                                .end_lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_39 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*e).lineno,
                                            end_lineno: (*e).end_lineno,
                                            col_offset: (*e).col_offset,
                                            end_col_offset: (*e).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_39)
                                    })
                                    .end_col_offset
                                }
                                .wrapping_add(1),
                            )
                        };
                        return 0;
                    }
                    if !IS_ASYNC_DEF!(st)
                        && ((unsafe { (*(*st).st_cur).ste_comprehension }) as u32) == 0
                    {
                        unsafe {
                            PyErr_SetString(
                                PyExc_SyntaxError,
                                c"'await' outside async function".as_ptr(),
                            )
                        };
                        unsafe {
                            PyErr_RangedSyntaxLocationObject(
                                (*st).st_filename,
                                (*{
                                    __slate_compound_40 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*e).lineno,
                                        end_lineno: (*e).end_lineno,
                                        col_offset: (*e).col_offset,
                                        end_col_offset: (*e).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_40)
                                })
                                .lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_41 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*e).lineno,
                                            end_lineno: (*e).end_lineno,
                                            col_offset: (*e).col_offset,
                                            end_col_offset: (*e).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_41)
                                    })
                                    .col_offset
                                }
                                .wrapping_add(1),
                                (*{
                                    __slate_compound_42 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*e).lineno,
                                        end_lineno: (*e).end_lineno,
                                        col_offset: (*e).col_offset,
                                        end_col_offset: (*e).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_42)
                                })
                                .end_lineno,
                                unsafe {
                                    (*{
                                        __slate_compound_43 = __SlateRecord_pycore_symtable_h_51 {
                                            lineno: (*e).lineno,
                                            end_lineno: (*e).end_lineno,
                                            col_offset: (*e).col_offset,
                                            end_col_offset: (*e).end_col_offset,
                                        };
                                        core::ptr::addr_of_mut!(__slate_compound_43)
                                    })
                                    .end_col_offset
                                }
                                .wrapping_add(1),
                            )
                        };
                        return 0;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Await.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                unsafe {
                    (*(*st).st_cur).__slate_bits_0.__set_ste_coroutine(1u32);
                }
            }
            16 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Compare.left }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 =
                        unsafe { (*e).v.Compare.comparators };
                    while i < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if symtable_visit_expr(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
            }
            17 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Call.func }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.Call.args };
                    while i < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if symtable_visit_expr(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if check_keywords(st, unsafe { (*e).v.Call.keywords }) == 0 {
                    return 0;
                }
                loop {
                    let mut i: i32 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_111 = unsafe { (*e).v.Call.keywords };
                    while (i as i64) < asdl_seq_LEN__7!(seq) {
                        let elt: *mut _keyword = asdl_seq_GET__4!(seq, i);
                        if !elt.is_null() {
                            if symtable_visit_keyword(st, elt) == 0 {
                                return 0;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
            }
            18 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.FormattedValue.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*e).v.FormattedValue.format_spec }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*e).v.FormattedValue.format_spec })
                            == 0
                        {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
            }
            19 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Interpolation.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*e).v.Interpolation.format_spec }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*e).v.Interpolation.format_spec }) == 0
                        {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
            }
            20 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.JoinedStr.values };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            21 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.TemplateStr.values };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            22 => {
                break '__slate_break_0; // Nothing to do here.
                // The following exprs can be assignment targets.
            }
            23 => {
                if check_name(
                    st,
                    unsafe { (*e).v.Attribute.attr },
                    unsafe {
                        *{
                            __slate_compound_71 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*e).lineno,
                                end_lineno: (*e).end_lineno,
                                col_offset: (*e).col_offset,
                                end_col_offset: (*e).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_71)
                        }
                    },
                    unsafe { (*e).v.Attribute.ctx },
                ) == 0
                {
                    return 0;
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Attribute.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
            }
            24 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Subscript.value }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    if symtable_visit_expr(st, unsafe { (*e).v.Subscript.slice }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
            }
            25 => loop {
                if symtable_visit_expr(st, unsafe { (*e).v.Starred.value }) == 0 {
                    return 0;
                }
                if true {
                    break;
                }
            },
            29 => {
                if !unsafe { (*e).v.Slice.lower }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*e).v.Slice.lower }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*e).v.Slice.upper }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*e).v.Slice.upper }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*e).v.Slice.step }.is_null() {
                    loop {
                        if symtable_visit_expr(st, unsafe { (*e).v.Slice.step }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
            }
            26 => {
                if ((unsafe {
                    (*(*st).st_cur)
                        .__slate_bits_1
                        .__get_ste_in_unevaluated_annotation()
                }) as i32)
                    == 0
                {
                    if symtable_add_def_ctx(
                        st,
                        unsafe { (*e).v.Name.id },
                        if ((unsafe { (*e).v.Name.ctx }) as u32) == 1 {
                            16i32
                        } else {
                            2i32
                        },
                        unsafe {
                            *{
                                __slate_compound_79 = __SlateRecord_pycore_symtable_h_51 {
                                    lineno: (*e).lineno,
                                    end_lineno: (*e).end_lineno,
                                    col_offset: (*e).col_offset,
                                    end_col_offset: (*e).end_col_offset,
                                };
                                core::ptr::addr_of_mut!(__slate_compound_79)
                            }
                        },
                        unsafe { (*e).v.Name.ctx },
                    ) == 0
                    {
                        return 0;
                    }
                    // Special-case super: it counts as a use of __class__
                    let __v80: bool = ((unsafe { (*e).v.Name.ctx }) as u32) == 1
                        && _PyST_IsFunctionLike(unsafe { (*st).st_cur }) != 0;
                    let __v81: bool = __v80
                        && (unsafe {
                            _PyUnicode_EqualToASCIIString((*e).v.Name.id, c"super".as_ptr())
                        }) != 0;
                    if __v81 {
                        if symtable_add_def(
                            st,
                            unsafe {
                                core::ptr::addr_of_mut!(
                                    _PyRuntime
                                        .static_objects
                                        .singletons
                                        .strings
                                        .identifiers
                                        ._py___class__
                                        ._ascii
                                        .ob_base
                                )
                            },
                            16i32,
                            unsafe {
                                *{
                                    __slate_compound_82 = __SlateRecord_pycore_symtable_h_51 {
                                        lineno: (*e).lineno,
                                        end_lineno: (*e).end_lineno,
                                        col_offset: (*e).col_offset,
                                        end_col_offset: (*e).end_col_offset,
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_82)
                                }
                            },
                        ) == 0
                        {
                            return 0;
                        }
                    }
                }
                break '__slate_break_0;
                // child nodes of List and Tuple will have expr_context set
            }
            27 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.List.elts };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            28 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*e).v.Tuple.elts };
                while i < asdl_seq_LEN!(seq) {
                    let elt: *mut _expr = asdl_seq_GET!(seq, i);
                    if symtable_visit_expr(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            _ => {}
        }
    }
    LEAVE_RECURSIVE!();
    1
}

fn symtable_visit_type_param_bound_or_default(
    st: *mut symtable,
    e: *mut _expr,
    name: *mut _object,
    tp: *mut _type_param,
    ste_scope_info: *const i8,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_6: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    if (unsafe {
        _PyUnicode_Equal(
            name,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___classdict__
                    ._ascii
                    .ob_base
            ),
        )
    }) != 0
    {
        let error_msg: *mut _object = unsafe {
            PyUnicode_FromFormat(
                c"reserved name '%U' cannot be used for type parameter".as_ptr(),
                name,
            )
        };
        if error_msg.is_null() {
            return 0;
        }
        unsafe { PyErr_SetObject(PyExc_SyntaxError, error_msg) };
        Py_DECREF!(error_msg);
        unsafe {
            PyErr_RangedSyntaxLocationObject(
                (*st).st_filename,
                (*{
                    __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*tp).lineno,
                        end_lineno: (*tp).end_lineno,
                        col_offset: (*tp).col_offset,
                        end_col_offset: (*tp).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                })
                .lineno,
                unsafe {
                    (*{
                        __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*tp).lineno,
                            end_lineno: (*tp).end_lineno,
                            col_offset: (*tp).col_offset,
                            end_col_offset: (*tp).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    })
                    .col_offset
                }
                .wrapping_add(1),
                (*{
                    __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*tp).lineno,
                        end_lineno: (*tp).end_lineno,
                        col_offset: (*tp).col_offset,
                        end_col_offset: (*tp).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                })
                .end_lineno,
                unsafe {
                    (*{
                        __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*tp).lineno,
                            end_lineno: (*tp).end_lineno,
                            col_offset: (*tp).col_offset,
                            end_col_offset: (*tp).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_3)
                    })
                    .end_col_offset
                }
                .wrapping_add(1),
            )
        };
        return 0;
    }
    if !e.is_null() {
        let is_in_class: i32 = (unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__get_ste_can_see_class_scope()
        }) as i32;
        if symtable_enter_block(st, name, 6u32, tp as *mut (), unsafe {
            *{
                __slate_compound_4 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*e).lineno,
                    end_lineno: (*e).end_lineno,
                    col_offset: (*e).col_offset,
                    end_col_offset: (*e).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_4)
            }
        }) == 0
        {
            return 0;
        }
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__set_ste_can_see_class_scope(is_in_class as u32);
        }
        let __v5: bool = is_in_class != 0
            && symtable_add_def(
                st,
                unsafe {
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py___classdict__
                            ._ascii
                            .ob_base
                    )
                },
                16i32,
                unsafe {
                    *{
                        __slate_compound_6 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_6)
                    }
                },
            ) == 0;
        if __v5 {
            return 0;
        }
        unsafe {
            (*(*st).st_cur).ste_scope_info = ste_scope_info;
        }
        loop {
            if symtable_visit_expr(st, e) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
        if symtable_exit_block(st) == 0 {
            return 0;
        }
    }
    1
}

fn symtable_visit_type_param(st: *mut symtable, tp: *mut _type_param) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    ENTER_RECURSIVE!();
    match (unsafe { (*tp).kind }) as u32 {
        1 => {
            if symtable_add_def(st, unsafe { (*tp).v.TypeVar.name }, 1026i32, unsafe {
                *{
                    __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*tp).lineno,
                        end_lineno: (*tp).end_lineno,
                        col_offset: (*tp).col_offset,
                        end_col_offset: (*tp).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            }) == 0
            {
                return 0;
            }
            let mut ste_scope_info: *const i8 = core::ptr::null::<i8>();
            let bound: *mut _expr = unsafe { (*tp).v.TypeVar.bound };
            if !bound.is_null() {
                ste_scope_info = (if ((unsafe { (*bound).kind }) as u32) == 28 {
                    c"a TypeVar constraint".as_ptr() as *mut i8
                } else {
                    c"a TypeVar bound".as_ptr() as *mut i8
                }) as *const i8;
            }
            // We must use a different key for the bound and default. The obvious choice would be to
            //
            // use the .bound and .default_value pointers, but that fails when the expression immediately
            //
            // inside the bound or default is a comprehension: we would reuse the same key for
            //
            // the comprehension scope. Therefore, use the address + 1 as the second key.
            //
            // The only requirement for the key is that it is unique and it matches the logic in
            //
            // compile.c where the scope is retrieved.
            if symtable_visit_type_param_bound_or_default(
                st,
                unsafe { (*tp).v.TypeVar.bound },
                unsafe { (*tp).v.TypeVar.name },
                tp,
                ste_scope_info,
            ) == 0
            {
                return 0;
            }
            if symtable_visit_type_param_bound_or_default(
                st,
                unsafe { (*tp).v.TypeVar.default_value },
                unsafe { (*tp).v.TypeVar.name },
                (tp as u64).wrapping_add(1) as *mut _type_param,
                c"a TypeVar default".as_ptr(),
            ) == 0
            {
                return 0;
            }
        }
        3 => {
            if symtable_add_def(st, unsafe { (*tp).v.TypeVarTuple.name }, 1026i32, unsafe {
                *{
                    __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*tp).lineno,
                        end_lineno: (*tp).end_lineno,
                        col_offset: (*tp).col_offset,
                        end_col_offset: (*tp).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            }) == 0
            {
                return 0;
            }
            if symtable_visit_type_param_bound_or_default(
                st,
                unsafe { (*tp).v.TypeVarTuple.default_value },
                unsafe { (*tp).v.TypeVarTuple.name },
                tp,
                c"a TypeVarTuple default".as_ptr(),
            ) == 0
            {
                return 0;
            }
        }
        2 => {
            if symtable_add_def(st, unsafe { (*tp).v.ParamSpec.name }, 1026i32, unsafe {
                *{
                    __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*tp).lineno,
                        end_lineno: (*tp).end_lineno,
                        col_offset: (*tp).col_offset,
                        end_col_offset: (*tp).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_3)
                }
            }) == 0
            {
                return 0;
            }
            if symtable_visit_type_param_bound_or_default(
                st,
                unsafe { (*tp).v.ParamSpec.default_value },
                unsafe { (*tp).v.ParamSpec.name },
                tp,
                c"a ParamSpec default".as_ptr(),
            ) == 0
            {
                return 0;
            }
        }
        _ => {}
    }
    LEAVE_RECURSIVE!();
    1
}

fn symtable_visit_pattern(st: *mut symtable, p: *mut _pattern) -> i32 {
    let mut __slate_compound_6: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_15: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_26: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    ENTER_RECURSIVE!();
    '__slate_break_1: {
        match (unsafe { (*p).kind }) as u32 {
            1 => loop {
                if symtable_visit_expr(st, unsafe { (*p).v.MatchValue.value }) == 0 {
                    return 0;
                }
                if true {
                    break;
                }
            },
            2 => {
                break '__slate_break_1; // Nothing to do here.
            }
            3 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_140 =
                    unsafe { (*p).v.MatchSequence.patterns };
                while i < asdl_seq_LEN__5!(seq) {
                    let elt: *mut _pattern = asdl_seq_GET__3!(seq, i);
                    if symtable_visit_pattern(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            6 => {
                if !unsafe { (*p).v.MatchStar.name }.is_null() {
                    if symtable_add_def(st, unsafe { (*p).v.MatchStar.name }, 2i32, unsafe {
                        *{
                            __slate_compound_6 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*p).lineno,
                                end_lineno: (*p).end_lineno,
                                col_offset: (*p).col_offset,
                                end_col_offset: (*p).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_6)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                }
            }
            4 => {
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_74 =
                        unsafe { (*p).v.MatchMapping.keys };
                    while i < asdl_seq_LEN!(seq) {
                        let elt: *mut _expr = asdl_seq_GET!(seq, i);
                        if symtable_visit_expr(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_140 =
                        unsafe { (*p).v.MatchMapping.patterns };
                    while i < asdl_seq_LEN__5!(seq) {
                        let elt: *mut _pattern = asdl_seq_GET__3!(seq, i);
                        if symtable_visit_pattern(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if !unsafe { (*p).v.MatchMapping.rest }.is_null() {
                    if symtable_add_def(st, unsafe { (*p).v.MatchMapping.rest }, 2i32, unsafe {
                        *{
                            __slate_compound_15 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*p).lineno,
                                end_lineno: (*p).end_lineno,
                                col_offset: (*p).col_offset,
                                end_col_offset: (*p).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_15)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                }
            }
            5 => {
                loop {
                    if symtable_visit_expr(st, unsafe { (*p).v.MatchClass.cls }) == 0 {
                        return 0;
                    }
                    if true {
                        break;
                    }
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_140 =
                        unsafe { (*p).v.MatchClass.patterns };
                    while i < asdl_seq_LEN__5!(seq) {
                        let elt: *mut _pattern = asdl_seq_GET__3!(seq, i);
                        if symtable_visit_pattern(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
                if check_kwd_patterns(st, p) == 0 {
                    return 0;
                }
                loop {
                    let mut i: i64 = 0;
                    let seq: *mut __SlateRecord_pycore_ast_h_140 =
                        unsafe { (*p).v.MatchClass.kwd_patterns };
                    while i < asdl_seq_LEN__5!(seq) {
                        let elt: *mut _pattern = asdl_seq_GET__3!(seq, i);
                        if symtable_visit_pattern(st, elt) == 0 {
                            return 0;
                        }
                        i = i.wrapping_add(1);
                    }
                    if true {
                        break;
                    }
                }
            }
            7 => {
                if !unsafe { (*p).v.MatchAs.pattern }.is_null() {
                    loop {
                        if symtable_visit_pattern(st, unsafe { (*p).v.MatchAs.pattern }) == 0 {
                            return 0;
                        }
                        if true {
                            break;
                        }
                    }
                }
                if !unsafe { (*p).v.MatchAs.name }.is_null() {
                    if symtable_add_def(st, unsafe { (*p).v.MatchAs.name }, 2i32, unsafe {
                        *{
                            __slate_compound_26 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*p).lineno,
                                end_lineno: (*p).end_lineno,
                                col_offset: (*p).col_offset,
                                end_col_offset: (*p).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_26)
                        }
                    }) == 0
                    {
                        return 0;
                    }
                }
            }
            8 => loop {
                let mut i: i64 = 0;
                let seq: *mut __SlateRecord_pycore_ast_h_140 = unsafe { (*p).v.MatchOr.patterns };
                while i < asdl_seq_LEN__5!(seq) {
                    let elt: *mut _pattern = asdl_seq_GET__3!(seq, i);
                    if symtable_visit_pattern(st, elt) == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                if true {
                    break;
                }
            },
            _ => {}
        }
    }
    LEAVE_RECURSIVE!();
    1
}

fn symtable_implicit_arg(st: *mut symtable, pos: i32) -> i32 {
    let id: *mut _object = unsafe { PyUnicode_FromFormat(c".%d".as_ptr(), pos) };
    if id.is_null() {
        return 0;
    }
    if symtable_add_def(st, id, 4i32, unsafe { (*(*st).st_cur).ste_loc }) == 0 {
        Py_DECREF!(id);
        return 0;
    }
    Py_DECREF!(id);
    1
}

fn symtable_visit_params(st: *mut symtable, args: *mut __SlateRecord_pycore_ast_h_104) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut i: i64 = 0;
    '__slate_break_1: while i < asdl_seq_LEN__4!(args) {
        let arg: *mut _arg = asdl_seq_GET__6!(args, i);
        if symtable_add_def(st, unsafe { (*arg).arg }, 4i32, unsafe {
            *{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*arg).lineno,
                    end_lineno: (*arg).end_lineno,
                    col_offset: (*arg).col_offset,
                    end_col_offset: (*arg).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        }) == 0
        {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn symtable_visit_annotation(st: *mut symtable, annotation: *mut _expr, key: *mut ()) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    // Annotations in local scopes are not executed and should not affect the symtable
    let is_unevaluated: bool = ((unsafe { (*(*st).st_cur).ste_type }) as u32) == 0;
    // Module-level annotations are always considered conditional because the module
    //
    // may be partially executed.
    if (((unsafe { (*(*st).st_cur).ste_type }) as u32) == 1
        && ((unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__get_ste_in_conditional_block()
        }) as i32)
            != 0
        || ((unsafe { (*(*st).st_cur).ste_type }) as u32) == 2)
        && ((unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__get_ste_has_conditional_annotations()
        }) as i32)
            == 0
    {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__set_ste_has_conditional_annotations(1u32);
        }
        if symtable_add_def(
            st,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py___conditional_annotations__
                        ._ascii
                        .ob_base
                )
            },
            16i32,
            unsafe {
                *{
                    __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*annotation).lineno,
                        end_lineno: (*annotation).end_lineno,
                        col_offset: (*annotation).col_offset,
                        end_col_offset: (*annotation).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            },
        ) == 0
        {
            return 0;
        }
    }
    let parent_ste: *mut _symtable_entry = unsafe { (*st).st_cur };
    if unsafe { (*parent_ste).ste_annotation_block }.is_null() {
        let current_type: u32 = unsafe { (*parent_ste).ste_type };
        if symtable_enter_block(
            st,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py___annotate__
                        ._ascii
                        .ob_base
                )
            },
            3u32,
            key,
            unsafe {
                *{
                    __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*annotation).lineno,
                        end_lineno: (*annotation).end_lineno,
                        col_offset: (*annotation).col_offset,
                        end_col_offset: (*annotation).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            },
        ) == 0
        {
            return 0;
        }
        unsafe {
            (*parent_ste).ste_annotation_block =
                Py_NewRef__2!(unsafe { (*st).st_cur }) as *mut _symtable_entry;
        }
        let future_annotations: i32 = (unsafe { (*(*st).st_future).ff_features }) & 16777216;
        if (current_type as u32) == 1 && future_annotations == 0 {
            unsafe {
                (*(*st).st_cur)
                    .__slate_bits_1
                    .__set_ste_can_see_class_scope(1u32);
            }
            unsafe {
                (*parent_ste).__slate_bits_1.__set_ste_needs_classdict(1u32);
            }
            if symtable_add_def(
                st,
                unsafe {
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py___classdict__
                            ._ascii
                            .ob_base
                    )
                },
                16i32,
                unsafe {
                    *{
                        __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*annotation).lineno,
                            end_lineno: (*annotation).end_lineno,
                            col_offset: (*annotation).col_offset,
                            end_col_offset: (*annotation).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_2)
                    }
                },
            ) == 0
            {
                return 0;
            }
        }
    } else {
        if symtable_enter_existing_block(st, unsafe { (*parent_ste).ste_annotation_block }, false)
            == 0
        {
            return 0;
        }
        // add_to_children
    }
    if is_unevaluated {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__set_ste_in_unevaluated_annotation(1u32);
        }
    }
    let rc: i32 = symtable_visit_expr(st, annotation);
    if is_unevaluated {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__set_ste_in_unevaluated_annotation(0u32);
        }
    }
    if symtable_exit_block(st) == 0 {
        return 0;
    }
    rc
}

fn symtable_visit_argannotations(
    st: *mut symtable,
    args: *mut __SlateRecord_pycore_ast_h_104,
) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__4!(args) {
        let arg: *mut _arg = asdl_seq_GET__6!(args, i);
        if !unsafe { (*arg).annotation }.is_null() {
            unsafe {
                (*(*st).st_cur)
                    .__slate_bits_0
                    .__set_ste_annotations_used(1u32);
            }
            loop {
                if symtable_visit_expr(st, unsafe { (*arg).annotation }) == 0 {
                    return 0;
                }
                if true {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    1
}

fn symtable_visit_annotations(
    st: *mut symtable,
    o: *mut _stmt,
    a: *mut _arguments,
    returns: *mut _expr,
    function_ste: *mut _symtable_entry,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let is_in_class: i32 = (unsafe {
        (*(*st).st_cur)
            .__slate_bits_1
            .__get_ste_can_see_class_scope()
    }) as i32;
    let current_type: u32 = unsafe { (*(*st).st_cur).ste_type };
    if symtable_enter_block(
        st,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___annotate__
                    ._ascii
                    .ob_base
            )
        },
        3u32,
        a as *mut (),
        unsafe {
            *{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*o).lineno,
                    end_lineno: (*o).end_lineno,
                    col_offset: (*o).col_offset,
                    end_col_offset: (*o).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        },
    ) == 0
    {
        return 0;
    }
    if is_in_class != 0 || (current_type as u32) == 1 {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_1
                .__set_ste_can_see_class_scope(1u32);
        }
        if symtable_add_def(
            st,
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py___classdict__
                        ._ascii
                        .ob_base
                )
            },
            16i32,
            unsafe {
                *{
                    __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*o).lineno,
                        end_lineno: (*o).end_lineno,
                        col_offset: (*o).col_offset,
                        end_col_offset: (*o).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            },
        ) == 0
        {
            return 0;
        }
    }
    let __v2: bool = !unsafe { (*a).posonlyargs }.is_null()
        && symtable_visit_argannotations(st, unsafe { (*a).posonlyargs }) == 0;
    if __v2 {
        return 0;
    }
    let __v3: bool = !unsafe { (*a).args }.is_null()
        && symtable_visit_argannotations(st, unsafe { (*a).args }) == 0;
    if __v3 {
        return 0;
    }
    if !unsafe { (*a).vararg }.is_null() && !unsafe { (*(*a).vararg).annotation }.is_null() {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_0
                .__set_ste_annotations_used(1u32);
        }
        loop {
            if symtable_visit_expr(st, unsafe { (*(*a).vararg).annotation }) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    if !unsafe { (*a).kwarg }.is_null() && !unsafe { (*(*a).kwarg).annotation }.is_null() {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_0
                .__set_ste_annotations_used(1u32);
        }
        loop {
            if symtable_visit_expr(st, unsafe { (*(*a).kwarg).annotation }) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    let __v6: bool = !unsafe { (*a).kwonlyargs }.is_null()
        && symtable_visit_argannotations(st, unsafe { (*a).kwonlyargs }) == 0;
    if __v6 {
        return 0;
    }
    if !returns.is_null() {
        unsafe {
            (*(*st).st_cur)
                .__slate_bits_0
                .__set_ste_annotations_used(1u32);
        }
        loop {
            if symtable_visit_expr(st, returns) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    if symtable_exit_block(st) == 0 {
        return 0;
    }
    1
}

fn symtable_visit_arguments(st: *mut symtable, a: *mut _arguments) -> i32 {
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    // skip default arguments inside function block
    // XXX should ast be different?
    let __v0: bool = !unsafe { (*a).posonlyargs }.is_null()
        && symtable_visit_params(st, unsafe { (*a).posonlyargs }) == 0;
    if __v0 {
        return 0;
    }
    let __v1: bool =
        !unsafe { (*a).args }.is_null() && symtable_visit_params(st, unsafe { (*a).args }) == 0;
    if __v1 {
        return 0;
    }
    let __v2: bool = !unsafe { (*a).kwonlyargs }.is_null()
        && symtable_visit_params(st, unsafe { (*a).kwonlyargs }) == 0;
    if __v2 {
        return 0;
    }
    if !unsafe { (*a).vararg }.is_null() {
        if symtable_add_def(st, unsafe { (*(*a).vararg).arg }, 4i32, unsafe {
            *{
                __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*(*a).vararg).lineno,
                    end_lineno: (*(*a).vararg).end_lineno,
                    col_offset: (*(*a).vararg).col_offset,
                    end_col_offset: (*(*a).vararg).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_3)
            }
        }) == 0
        {
            return 0;
        }
        unsafe {
            (*(*st).st_cur).__slate_bits_1.__set_ste_varargs(1u32);
        }
    }
    if !unsafe { (*a).kwarg }.is_null() {
        if symtable_add_def(st, unsafe { (*(*a).kwarg).arg }, 4i32, unsafe {
            *{
                __slate_compound_4 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*(*a).kwarg).lineno,
                    end_lineno: (*(*a).kwarg).end_lineno,
                    col_offset: (*(*a).kwarg).col_offset,
                    end_col_offset: (*(*a).kwarg).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_4)
            }
        }) == 0
        {
            return 0;
        }
        unsafe {
            (*(*st).st_cur).__slate_bits_1.__set_ste_varkeywords(1u32);
        }
    }
    1
}

fn symtable_visit_excepthandler(st: *mut symtable, eh: *mut _excepthandler) -> i32 {
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    if !unsafe { (*eh).v.ExceptHandler.r#type }.is_null() {
        loop {
            if symtable_visit_expr(st, unsafe { (*eh).v.ExceptHandler.r#type }) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    if !unsafe { (*eh).v.ExceptHandler.name }.is_null() {
        if symtable_add_def(st, unsafe { (*eh).v.ExceptHandler.name }, 2i32, unsafe {
            *{
                __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*eh).lineno,
                    end_lineno: (*eh).end_lineno,
                    col_offset: (*eh).col_offset,
                    end_col_offset: (*eh).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            }
        }) == 0
        {
            return 0;
        }
    }
    loop {
        let mut i: i64 = 0;
        let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*eh).v.ExceptHandler.body };
        while i < asdl_seq_LEN__2!(seq) {
            let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
            if symtable_visit_stmt(st, elt) == 0 {
                return 0;
            }
            i = i.wrapping_add(1);
        }
        if true {
            break;
        }
    }
    1
}

fn symtable_visit_withitem(st: *mut symtable, item: *mut _withitem) -> i32 {
    loop {
        if symtable_visit_expr(st, unsafe { (*item).context_expr }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    if !unsafe { (*item).optional_vars }.is_null() {
        loop {
            if symtable_visit_expr(st, unsafe { (*item).optional_vars }) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    1
}

fn symtable_visit_match_case(st: *mut symtable, m: *mut _match_case) -> i32 {
    loop {
        if symtable_visit_pattern(st, unsafe { (*m).pattern }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    if !unsafe { (*m).guard }.is_null() {
        loop {
            if symtable_visit_expr(st, unsafe { (*m).guard }) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    loop {
        let mut i: i64 = 0;
        let seq: *mut __SlateRecord_pycore_ast_h_67 = unsafe { (*m).body };
        while i < asdl_seq_LEN__2!(seq) {
            let elt: *mut _stmt = asdl_seq_GET__2!(seq, i);
            if symtable_visit_stmt(st, elt) == 0 {
                return 0;
            }
            i = i.wrapping_add(1);
        }
        if true {
            break;
        }
    }
    1
}

fn symtable_visit_alias(st: *mut symtable, a: *mut _alias) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    // Compute store_name, the name actually bound by the import
    // operation.  It is different than a->name when a->name is a
    // dotted package name (e.g. spam.eggs)
    let mut store_name: *mut _object = unsafe { core::mem::zeroed() };
    let name: *mut _object = if unsafe { (*a).asname }.is_null() {
        unsafe { (*a).name }
    } else {
        unsafe { (*a).asname }
    };
    let dot: i64 =
        unsafe { PyUnicode_FindChar(name, 46u32, 0i64, PyUnicode_GET_LENGTH!(name), 1i32) };
    if dot != (1i32.wrapping_neg() as i64) {
        store_name = unsafe { PyUnicode_Substring(name, 0i64, dot) };
        if store_name.is_null() {
            return 0;
        }
    } else {
        store_name = Py_NewRef!(name);
    }
    if (unsafe { _PyUnicode_EqualToASCIIString(name, c"*".as_ptr()) }) == 0 {
        let r: i32 = symtable_add_def(st, store_name, 128i32, unsafe {
            *{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*a).lineno,
                    end_lineno: (*a).end_lineno,
                    col_offset: (*a).col_offset,
                    end_col_offset: (*a).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        });
        Py_DECREF!(store_name);
        return r;
    } else {
        if ((unsafe { (*(*st).st_cur).ste_type }) as u32) != 2 {
            unsafe {
                PyErr_SetString(
                    PyExc_SyntaxError,
                    c"import * only allowed at module level".as_ptr(),
                )
            };
            unsafe {
                PyErr_RangedSyntaxLocationObject(
                    (*st).st_filename,
                    (*{
                        __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*a).lineno,
                            end_lineno: (*a).end_lineno,
                            col_offset: (*a).col_offset,
                            end_col_offset: (*a).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    })
                    .lineno,
                    unsafe {
                        (*{
                            __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*a).lineno,
                                end_lineno: (*a).end_lineno,
                                col_offset: (*a).col_offset,
                                end_col_offset: (*a).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_2)
                        })
                        .col_offset
                    }
                    .wrapping_add(1),
                    (*{
                        __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*a).lineno,
                            end_lineno: (*a).end_lineno,
                            col_offset: (*a).col_offset,
                            end_col_offset: (*a).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_3)
                    })
                    .end_lineno,
                    unsafe {
                        (*{
                            __slate_compound_4 = __SlateRecord_pycore_symtable_h_51 {
                                lineno: (*a).lineno,
                                end_lineno: (*a).end_lineno,
                                col_offset: (*a).col_offset,
                                end_col_offset: (*a).end_col_offset,
                            };
                            core::ptr::addr_of_mut!(__slate_compound_4)
                        })
                        .end_col_offset
                    }
                    .wrapping_add(1),
                )
            };
            Py_DECREF!(store_name);
            return 0;
        }
        Py_DECREF!(store_name);
        return 1;
    }
}

fn symtable_visit_comprehension(st: *mut symtable, lc: *mut _comprehension) -> i32 {
    unsafe {
        (*(*st).st_cur)
            .__slate_bits_1
            .__set_ste_comp_iter_target(1u32);
    }
    loop {
        if symtable_visit_expr(st, unsafe { (*lc).target }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    unsafe {
        (*(*st).st_cur)
            .__slate_bits_1
            .__set_ste_comp_iter_target(0u32);
    }
    let __v1: *mut _symtable_entry = unsafe { (*st).st_cur };
    let __v2: i32 = unsafe { (*__v1).ste_comp_iter_expr };
    unsafe {
        (*__v1).ste_comp_iter_expr = __v2.wrapping_add(1);
    }
    loop {
        if symtable_visit_expr(st, unsafe { (*lc).iter }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    let __v5: *mut _symtable_entry = unsafe { (*st).st_cur };
    let __v6: i32 = unsafe { (*__v5).ste_comp_iter_expr };
    unsafe {
        (*__v5).ste_comp_iter_expr = __v6.wrapping_sub(1);
    }
    loop {
        let mut i: i64 = 0;
        let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*lc).ifs };
        while i < asdl_seq_LEN!(seq) {
            let elt: *mut _expr = asdl_seq_GET!(seq, i);
            if symtable_visit_expr(st, elt) == 0 {
                return 0;
            }
            i = i.wrapping_add(1);
        }
        if true {
            break;
        }
    }
    if (unsafe { (*lc).is_async }) != 0 {
        unsafe {
            (*(*st).st_cur).__slate_bits_0.__set_ste_coroutine(1u32);
        }
    }
    1
}

fn symtable_visit_keyword(st: *mut symtable, k: *mut _keyword) -> i32 {
    loop {
        if symtable_visit_expr(st, unsafe { (*k).value }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    1
}

fn symtable_handle_comprehension(
    st: *mut symtable,
    e: *mut _expr,
    scope_name: *mut _object,
    generators: *mut __SlateRecord_pycore_ast_h_81,
    elt: *mut _expr,
    value: *mut _expr,
) -> i32 {
    let mut __slate_compound_8: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_22: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_23: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_24: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_25: __SlateRecord_pycore_symtable_h_51 =
        unsafe { core::mem::zeroed() };
    let is_generator: i32 = (((unsafe { (*e).kind }) as u32) == 12) as i32;
    let outermost: *mut _comprehension = asdl_seq_GET__5!(generators, 0i32);
    // Outermost iterator is evaluated in current scope
    let __v0: *mut _symtable_entry = unsafe { (*st).st_cur };
    let __v1: i32 = unsafe { (*__v0).ste_comp_iter_expr };
    unsafe {
        (*__v0).ste_comp_iter_expr = __v1.wrapping_add(1);
    }
    loop {
        if symtable_visit_expr(st, unsafe { (*outermost).iter }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    let __v4: *mut _symtable_entry = unsafe { (*st).st_cur };
    let __v5: i32 = unsafe { (*__v4).ste_comp_iter_expr };
    unsafe {
        (*__v4).ste_comp_iter_expr = __v5.wrapping_sub(1);
    }
    // Create comprehension scope for the rest
    let __v7: bool = scope_name.is_null()
        || symtable_enter_block(st, scope_name, 0u32, e as *mut (), unsafe {
            *{
                __slate_compound_8 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*e).lineno,
                    end_lineno: (*e).end_lineno,
                    col_offset: (*e).col_offset,
                    end_col_offset: (*e).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_8)
            }
        }) == 0;
    if __v7 {
        return 0;
    }
    match (unsafe { (*e).kind }) as u32 {
        9 => unsafe {
            (*(*st).st_cur).ste_comprehension = 1;
        },
        10 => unsafe {
            (*(*st).st_cur).ste_comprehension = 3;
        },
        11 => unsafe {
            (*(*st).st_cur).ste_comprehension = 2;
        },
        _ => unsafe {
            (*(*st).st_cur).ste_comprehension = 4;
        },
    }
    if (unsafe { (*outermost).is_async }) != 0 {
        unsafe {
            (*(*st).st_cur).__slate_bits_0.__set_ste_coroutine(1u32);
        }
    }
    // Outermost iter is received as an argument
    if symtable_implicit_arg(st, 0i32) == 0 {
        symtable_exit_block(st);
        return 0;
    }
    // Visit iteration variable target, and mark them as such
    unsafe {
        (*(*st).st_cur)
            .__slate_bits_1
            .__set_ste_comp_iter_target(1u32);
    }
    loop {
        if symtable_visit_expr(st, unsafe { (*outermost).target }) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    unsafe {
        (*(*st).st_cur)
            .__slate_bits_1
            .__set_ste_comp_iter_target(0u32);
    }
    // Visit the rest of the comprehension body
    loop {
        let mut i: i64 = 0;
        let seq: *mut __SlateRecord_pycore_ast_h_74 = unsafe { (*outermost).ifs };
        while i < asdl_seq_LEN!(seq) {
            let elt: *mut _expr = asdl_seq_GET!(seq, i);
            if symtable_visit_expr(st, elt) == 0 {
                return 0;
            }
            i = i.wrapping_add(1);
        }
        if true {
            break;
        }
    }
    loop {
        let mut i: i64 = 1;
        let seq: *mut __SlateRecord_pycore_ast_h_81 = generators;
        while i < asdl_seq_LEN__9!(seq) {
            let elt: *mut _comprehension = asdl_seq_GET__5!(seq, i);
            if symtable_visit_comprehension(st, elt) == 0 {
                return 0;
            }
            i = i.wrapping_add(1);
        }
        if true {
            break;
        }
    }
    if !value.is_null() {
        loop {
            if symtable_visit_expr(st, value) == 0 {
                return 0;
            }
            if true {
                break;
            }
        }
    }
    loop {
        if symtable_visit_expr(st, elt) == 0 {
            return 0;
        }
        if true {
            break;
        }
    }
    unsafe {
        (*(*st).st_cur)
            .__slate_bits_0
            .__set_ste_generator(is_generator as u32);
    }
    let is_async: i32 = (((unsafe { (*(*st).st_cur).__slate_bits_0.__get_ste_coroutine() }) as i32)
        != 0
        && is_generator == 0) as i32;
    if symtable_exit_block(st) == 0 {
        return 0;
    }
    let __v21: bool = is_async != 0
        && !IS_ASYNC_DEF!(st)
        && ((unsafe { (*(*st).st_cur).ste_comprehension }) as u32) == 0
        && !allows_top_level_await(st);
    if __v21 {
        unsafe {
            PyErr_SetString(
                PyExc_SyntaxError,
                c"asynchronous comprehension outside of an asynchronous function".as_ptr(),
            )
        };
        unsafe {
            PyErr_RangedSyntaxLocationObject(
                (*st).st_filename,
                (*{
                    __slate_compound_22 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_22)
                })
                .lineno,
                unsafe {
                    (*{
                        __slate_compound_23 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_23)
                    })
                    .col_offset
                }
                .wrapping_add(1),
                (*{
                    __slate_compound_24 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_24)
                })
                .end_lineno,
                unsafe {
                    (*{
                        __slate_compound_25 = __SlateRecord_pycore_symtable_h_51 {
                            lineno: (*e).lineno,
                            end_lineno: (*e).end_lineno,
                            col_offset: (*e).col_offset,
                            end_col_offset: (*e).end_col_offset,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_25)
                    })
                    .end_col_offset
                }
                .wrapping_add(1),
            )
        };
        return 0;
    }
    if is_async != 0 {
        unsafe {
            (*(*st).st_cur).__slate_bits_0.__set_ste_coroutine(1u32);
        }
    }
    1
}

fn symtable_visit_genexp(st: *mut symtable, e: *mut _expr) -> i32 {
    symtable_handle_comprehension(
        st,
        e,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_genexpr
                    ._ascii
                    .ob_base
            )
        },
        unsafe { (*e).v.GeneratorExp.generators },
        unsafe { (*e).v.GeneratorExp.elt },
        core::ptr::null_mut::<_expr>(),
    )
}

fn symtable_visit_listcomp(st: *mut symtable, e: *mut _expr) -> i32 {
    symtable_handle_comprehension(
        st,
        e,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_listcomp
                    ._ascii
                    .ob_base
            )
        },
        unsafe { (*e).v.ListComp.generators },
        unsafe { (*e).v.ListComp.elt },
        core::ptr::null_mut::<_expr>(),
    )
}

fn symtable_visit_setcomp(st: *mut symtable, e: *mut _expr) -> i32 {
    symtable_handle_comprehension(
        st,
        e,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_setcomp
                    ._ascii
                    .ob_base
            )
        },
        unsafe { (*e).v.SetComp.generators },
        unsafe { (*e).v.SetComp.elt },
        core::ptr::null_mut::<_expr>(),
    )
}

fn symtable_visit_dictcomp(st: *mut symtable, e: *mut _expr) -> i32 {
    symtable_handle_comprehension(
        st,
        e,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_dictcomp
                    ._ascii
                    .ob_base
            )
        },
        unsafe { (*e).v.DictComp.generators },
        unsafe { (*e).v.DictComp.key },
        unsafe { (*e).v.DictComp.value },
    )
}

fn symtable_raise_if_annotation_block(st: *mut symtable, name: *const i8, e: *mut _expr) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let r#type: u32 = unsafe { (*(*st).st_cur).ste_type };
    if (r#type as u32) == 3 {
        unsafe {
            PyErr_Format(
                PyExc_SyntaxError,
                c"%s cannot be used within an annotation".as_ptr(),
                name,
            )
        };
    } else {
        if (r#type as u32) == 6 {
            let info: *const i8 = unsafe { (*(*st).st_cur).ste_scope_info };
            // e.g., info == "a ParamSpec default"
            unsafe {
                PyErr_Format(
                    PyExc_SyntaxError,
                    c"%s cannot be used within %s".as_ptr(),
                    name,
                    info,
                )
            };
        } else {
            if (r#type as u32) == 4 {
                // for now, we do not have any extra information
                unsafe {
                    PyErr_Format(
                        PyExc_SyntaxError,
                        c"%s cannot be used within a type alias".as_ptr(),
                        name,
                    )
                };
            } else {
                if (r#type as u32) == 5 {
                    // for now, we do not have any extra information
                    unsafe {
                        PyErr_Format(
                            PyExc_SyntaxError,
                            c"%s cannot be used within the definition of a generic".as_ptr(),
                            name,
                        )
                    };
                } else {
                    return 1;
                }
            }
        }
    }
    unsafe {
        PyErr_RangedSyntaxLocationObject(
            (*st).st_filename,
            (*{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*e).lineno,
                    end_lineno: (*e).end_lineno,
                    col_offset: (*e).col_offset,
                    end_col_offset: (*e).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            })
            .lineno,
            unsafe {
                (*{
                    __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                })
                .col_offset
            }
            .wrapping_add(1),
            (*{
                __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*e).lineno,
                    end_lineno: (*e).end_lineno,
                    col_offset: (*e).col_offset,
                    end_col_offset: (*e).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            })
            .end_lineno,
            unsafe {
                (*{
                    __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_3)
                })
                .end_col_offset
            }
            .wrapping_add(1),
        )
    };
    0
}

fn symtable_raise_if_comprehension_block(st: *mut symtable, e: *mut _expr) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    let r#type: u32 = unsafe { (*(*st).st_cur).ste_comprehension };
    unsafe {
        PyErr_SetString(
            PyExc_SyntaxError,
            (if (r#type as u32) == 1 {
                c"'yield' inside list comprehension".as_ptr() as *mut i8
            } else {
                if (r#type as u32) == 3 {
                    c"'yield' inside set comprehension".as_ptr() as *mut i8
                } else {
                    if (r#type as u32) == 2 {
                        c"'yield' inside dict comprehension".as_ptr() as *mut i8
                    } else {
                        c"'yield' inside generator expression".as_ptr() as *mut i8
                    }
                }
            }) as *const i8,
        )
    };
    unsafe {
        PyErr_RangedSyntaxLocationObject(
            (*st).st_filename,
            (*{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*e).lineno,
                    end_lineno: (*e).end_lineno,
                    col_offset: (*e).col_offset,
                    end_col_offset: (*e).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            })
            .lineno,
            unsafe {
                (*{
                    __slate_compound_1 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                })
                .col_offset
            }
            .wrapping_add(1),
            (*{
                __slate_compound_2 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*e).lineno,
                    end_lineno: (*e).end_lineno,
                    col_offset: (*e).col_offset,
                    end_col_offset: (*e).end_col_offset,
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            })
            .end_lineno,
            unsafe {
                (*{
                    __slate_compound_3 = __SlateRecord_pycore_symtable_h_51 {
                        lineno: (*e).lineno,
                        end_lineno: (*e).end_lineno,
                        col_offset: (*e).col_offset,
                        end_col_offset: (*e).end_col_offset,
                    };
                    core::ptr::addr_of_mut!(__slate_compound_3)
                })
                .end_col_offset
            }
            .wrapping_add(1),
        )
    };
    0
}

fn symtable_raise_if_not_coroutine(
    st: *mut symtable,
    msg: *const i8,
    mut loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    if ((unsafe { (*(*st).st_cur).__slate_bits_0.__get_ste_coroutine() }) as i32) == 0 {
        unsafe { PyErr_SetString(PyExc_SyntaxError, msg) };
        SET_ERROR_LOCATION!(unsafe { (*st).st_filename }, loc);
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_SymtableStringObjectFlags(
    str: *const i8,
    filename: *mut _object,
    start: i32,
    flags: *mut __SlateRecord_compile_h_30,
) -> *mut symtable {
    let arena: *mut _arena = unsafe { _PyArena_New() };
    if arena.is_null() {
        return core::ptr::null_mut::<symtable>();
    }
    let r#mod: *mut _mod = unsafe { _PyParser_ASTFromString(str, filename, start, flags, arena) };
    if r#mod.is_null() {
        unsafe { _PyArena_Free(arena) };
        return core::ptr::null_mut::<symtable>();
    }
    let mut future: __SlateRecord_pycore_symtable_h_67 = unsafe { core::mem::zeroed() };
    if (unsafe { _PyFuture_FromAST(r#mod, filename, core::ptr::addr_of_mut!(future)) }) == 0 {
        unsafe { _PyArena_Free(arena) };
        return core::ptr::null_mut::<symtable>();
    }
    future.ff_features |= unsafe { (*flags).cf_flags };
    let st: *mut symtable = _PySymtable_Build(r#mod, filename, core::ptr::addr_of_mut!(future));
    unsafe { _PyArena_Free(arena) };
    st
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_MaybeMangle(
    privateobj: *mut _object,
    ste: *mut _symtable_entry,
    name: *mut _object,
) -> *mut _object {
    // Special case for type parameter blocks around generic classes:
    // we want to mangle type parameter names (so a type param with a private
    // name can be used inside the class body), but we don't want to mangle
    // any other names that appear within the type parameter scope.
    if !unsafe { (*ste).ste_mangled_names }.is_null() {
        let result: i32 = unsafe { PySet_Contains((*ste).ste_mangled_names, name) };
        if result < 0 {
            return core::ptr::null_mut::<_object>();
        }
        if result == 0 {
            return Py_NewRef!(name);
        }
    }
    _Py_Mangle(privateobj, name)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_Mangle(privateobj: *mut _object, ident: *mut _object) -> *mut _object {
    let mut writer: *mut PyUnicodeWriter = unsafe { core::mem::zeroed() };
    // Strip leading underscores from class name
    let mut ipriv: u64 = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    // Don't mangle __id__ or names with dots.
    //
    // The only time a name with a dot can occur is when
    // we are compiling an import statement that has a
    // package name.
    //
    // TODO(jhylton): Decide whether we want to support
    // mangling of the module name, e.g. __M.X.
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut plen: u64 = unsafe { core::mem::zeroed() };
    let mut nlen: u64 = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut __v9: bool = unsafe { core::mem::zeroed() };
    // Name mangling: __private becomes _classname__private.
    // This is independent from how the name is used.
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if privateobj.is_null() {
            __v10 = true;
        } else {
            __v10 = PyUnicode_Check!(privateobj) == 0;
        }
        if __v10 {
            __v9 = true;
        } else {
            __v9 = PyUnicode_READ_CHAR__2!(ident, 0i32) != 95;
        }
        if __v9 {
            __v8 = true;
        } else {
            __v8 = PyUnicode_READ_CHAR__2!(ident, 1i32) != 95;
        }
        if __v8 {
            return Py_NewRef!(ident);
        } else {
            nlen = PyUnicode_GET_LENGTH!(ident) as u64;
            plen = PyUnicode_GET_LENGTH!(privateobj) as u64;
            if PyUnicode_READ_CHAR__2!(ident, nlen.wrapping_sub(1)) == 95 {
                __v5 = PyUnicode_READ_CHAR__2!(ident, nlen.wrapping_sub(2)) == 95;
            } else {
                __v5 = false;
            }
            if __v5 {
                __v4 = true;
            } else {
                __v4 = PyUnicode_FindChar(ident, 46u32, 0i64, nlen as i64, 1i32)
                    != (1i32.wrapping_neg() as i64);
            }
            if __v4 {
                return Py_NewRef!(ident);
            } else {
                ipriv = 0;
                while PyUnicode_READ_CHAR__2!(privateobj, ipriv) == 95 {
                    ipriv = ipriv.wrapping_add(1);
                }
                if ipriv == plen {
                    return Py_NewRef!(ident);
                } else {
                    if nlen.wrapping_add(plen.wrapping_sub(ipriv)) >= 9223372036854775806u64 {
                        PyErr_SetString(
                            PyExc_OverflowError,
                            c"private identifier too large to be mangled".as_ptr(),
                        );
                        return core::ptr::null_mut::<_object>();
                    } else {
                        writer = PyUnicodeWriter_Create(
                            1u64.wrapping_add(nlen)
                                .wrapping_add(plen.wrapping_sub(ipriv))
                                as i64,
                        );
                        if writer.is_null() {
                            return core::ptr::null_mut::<_object>();
                        } else {
                            // ident = "_" + priv[ipriv:] + ident
                            if PyUnicodeWriter_WriteChar(writer, 95u32) >= 0 {
                                if PyUnicodeWriter_WriteSubstring(
                                    writer,
                                    privateobj,
                                    ipriv as i64,
                                    plen as i64,
                                ) >= 0
                                {
                                    if PyUnicodeWriter_WriteStr(writer, ident) >= 0 {
                                        return PyUnicodeWriter_Finish(writer);
                                    }
                                }
                            }
                            PyUnicodeWriter_Discard(writer);
                            return core::ptr::null_mut::<_object>();
                        }
                    }
                }
            }
        }
    }
    // Don't mangle __whatever__
    // Don't mangle if class is just underscores
}
