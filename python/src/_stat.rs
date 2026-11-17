//! stat.h interface
//!
//! The module defines all S_IF*, S_I*, UF_*, SF_* and ST_* constants to
//! sensible default values as well as defines S_IS*() macros in order to keep
//! backward compatibility with the old stat.py module.
//!
//! New constants and macros such as S_IFDOOR / S_ISDOOR() are always defined
//! as int 0.
//!
//! NOTE: POSIX only defines the values of the S_I* permission bits.
//! Define macros for inline documentation.
//! PyObject_HEAD defines the initial segment of every PyObject.
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
// Signed variants of the above
macro_rules! PyLong_Check__3 {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE__3!($op), 16777216u64)
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! Py_ARRAY_LENGTH__22 {
    () => {
        (core::mem::size_of::<[*const i8; 10]>() as u64)
            / (core::mem::size_of::<*const i8>() as u64)
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_TYPE__3 {
    ($ob:expr) => {
        _Py_TYPE($ob)
    };
}

macro_rules! S_ISDOOR {
    () => {
        0i32
    };
}

macro_rules! S_ISPORT {
    () => {
        0i32
    };
}

macro_rules! S_ISWHT {
    () => {
        0i32
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
    };
}

/// Macro to use the more powerful/dangerous C-style cast even in C++.
macro_rules! _Py_CAST__2 {
    ($expr:expr) => {
        $expr
    };
}

unsafe extern "C" {
    static mut PyExc_OverflowError: *mut _object;
    fn PyType_GetFlags(arg0: *mut _typeobject) -> u64;
    fn _Py_DecRef(arg0: *mut _object);
    fn PyUnicode_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyLong_FromUnsignedLong(arg0: u64) -> *mut _object;
    fn PyLong_AsUnsignedLong(arg0: *mut _object) -> u64;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn PyModule_AddIntConstant(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn PyNumber_Index(arg0: *mut _object) -> *mut _object;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _object {
    __slate_anon_0: __SlateRecord_object_h_122,
    ob_type: *mut _typeobject,
}

// Cast argument to PyVarObject* type.
fn _Py_TYPE(ob: *mut _object) -> *mut _typeobject {
    unsafe { (*ob).ob_type }
}

fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let mut flags: u64 = 0;
    // PyTypeObject is opaque in the limited C API
    flags = unsafe { PyType_GetFlags(r#type) };
    (flags & feature != 0) as i32
}

fn Py_DECREF(op: *mut _object) {
    unsafe { _Py_DecRef(op) };
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _typeobject {}

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
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

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

// Need limited C API version 3.13 for PyModule_Add() on Windows
// Py_GIL_DISABLED
// From Python's stat.py
// S_IFXXX constants (file types)
//
// Only the names are defined by POSIX but not their value. All common file
// types seems to have the same numeric value on all platforms, though.
//
// fileutils.h guarantees S_IFMT, S_IFDIR, S_IFCHR, S_IFREG and S_IFLNK
// S_ISXXX()
// fileutils.h defines S_ISDIR(), S_ISREG() and S_ISCHR()
// S_I* file permission
//
// The permission bit value are defined by POSIX standards.
// what is S_ENFMT?
// Names for file flags
fn _PyLong_AsMode_t(mut op: *mut _object) -> u32 {
    let mut value: u64 = 0;
    let mut mode: u32 = 0;
    if PyLong_Check__3!(op) != 0 {
        value = unsafe { PyLong_AsUnsignedLong(op) };
    } else {
        op = unsafe { PyNumber_Index(op) };
        if op.is_null() {
            return 1i32.wrapping_neg() as u32;
        }
        value = unsafe { PyLong_AsUnsignedLong(op) };
        Py_DECREF!(op);
    }
    let __v0: bool =
        value == (1i32.wrapping_neg() as u64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return 1i32.wrapping_neg() as u32;
    }
    mode = value as u32;
    if (mode as u64) != value {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"mode out of range".as_ptr()) };
        return 1i32.wrapping_neg() as u32;
    }
    mode
}

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISDIR")]
extern "C-unwind" fn stat_S_ISDIR(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 16384) as i64) }
}

static stat_S_ISDIR_doc: __SlateAlign16<[i8; 64]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 68i8, 73i8, 82i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 100i8, 105i8, 114i8, 101i8,
    99i8, 116i8, 111i8, 114i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISCHR")]
extern "C-unwind" fn stat_S_ISCHR(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 8192) as i64) }
}

static stat_S_ISCHR_doc: __SlateAlign16<[i8; 84]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 67i8, 72i8, 82i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 99i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 97i8, 108i8, 32i8,
    100i8, 101i8, 118i8, 105i8, 99i8, 101i8, 32i8, 102i8, 105i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISBLK")]
extern "C-unwind" fn stat_S_ISBLK(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 24576) as i64) }
}

static stat_S_ISBLK_doc: __SlateAlign16<[i8; 80]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 66i8, 76i8, 75i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 98i8, 108i8, 111i8, 99i8,
    107i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 97i8, 108i8, 32i8, 100i8, 101i8, 118i8, 105i8,
    99i8, 101i8, 32i8, 102i8, 105i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISREG")]
extern "C-unwind" fn stat_S_ISREG(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 32768) as i64) }
}

static stat_S_ISREG_doc: __SlateAlign16<[i8; 67]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 82i8, 69i8, 71i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 114i8, 101i8, 103i8, 117i8,
    108i8, 97i8, 114i8, 32i8, 102i8, 105i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISFIFO")]
extern "C-unwind" fn stat_S_ISFIFO(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 4096) as i64) }
}

static stat_S_ISFIFO_doc: __SlateAlign16<[i8; 73]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 70i8, 73i8, 70i8, 79i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 70i8, 73i8, 70i8, 79i8,
    32i8, 40i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 112i8, 105i8, 112i8, 101i8, 41i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISLNK")]
extern "C-unwind" fn stat_S_ISLNK(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 40960) as i64) }
}

static stat_S_ISLNK_doc: __SlateAlign16<[i8; 68]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 76i8, 78i8, 75i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 115i8, 121i8, 109i8, 98i8,
    111i8, 108i8, 105i8, 99i8, 32i8, 108i8, 105i8, 110i8, 107i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISSOCK")]
extern "C-unwind" fn stat_S_ISSOCK(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong((mode & 61440 == 49152) as i64) }
}

static stat_S_ISSOCK_doc: __SlateAlign16<[i8; 62]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 83i8, 79i8, 67i8, 75i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 115i8, 111i8, 99i8,
    107i8, 101i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISDOOR")]
extern "C-unwind" fn stat_S_ISDOOR(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong(0i64) }
}

static stat_S_ISDOOR_doc: __SlateAlign16<[i8; 60]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 68i8, 79i8, 79i8, 82i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 100i8, 111i8, 111i8,
    114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISPORT")]
extern "C-unwind" fn stat_S_ISPORT(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong(0i64) }
}

static stat_S_ISPORT_doc: __SlateAlign16<[i8; 67]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 80i8, 79i8, 82i8, 84i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 110i8, 32i8, 101i8, 118i8,
    101i8, 110i8, 116i8, 32i8, 112i8, 111i8, 114i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_ISWHT")]
extern "C-unwind" fn stat_S_ISWHT(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong(0i64) }
}

static stat_S_ISWHT_doc: __SlateAlign16<[i8; 63]> = __SlateAlign16([
    83i8, 95i8, 73i8, 83i8, 87i8, 72i8, 84i8, 40i8, 109i8, 111i8, 100i8, 101i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 119i8, 104i8, 105i8, 116i8,
    101i8, 111i8, 117i8, 116i8, 46i8, 0i8,
]);

static stat_S_IMODE_doc: __SlateAlign16<[i8; 69]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 111i8, 114i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 39i8, 115i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 116i8, 104i8, 97i8, 116i8,
    32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 115i8, 101i8, 116i8, 32i8, 98i8, 121i8, 32i8,
    111i8, 115i8, 46i8, 99i8, 104i8, 109i8, 111i8, 100i8, 40i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_IMODE")]
extern "C-unwind" fn stat_S_IMODE(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromUnsignedLong((mode & 4095) as u64) }
}

static stat_S_IFMT_doc: __SlateAlign16<[i8; 68]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 111i8, 114i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 39i8, 115i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 116i8, 104i8, 97i8, 116i8,
    32i8, 100i8, 101i8, 115i8, 99i8, 114i8, 105i8, 98i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 116i8, 121i8, 112i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_S_IFMT")]
extern "C-unwind" fn stat_S_IFMT(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromUnsignedLong((mode & 61440) as u64) }
}

// file type chars according to
// http://en.wikibooks.org/wiki/C_Programming/POSIX_Reference/sys/stat.h
fn filetype(mode: u32) -> i8 {
    // common cases first
    if mode & 61440 == 32768 {
        return 45;
    }
    if mode & 61440 == 16384 {
        return 100;
    }
    if mode & 61440 == 40960 {
        return 108;
    }
    // special files
    if mode & 61440 == 24576 {
        return 98;
    }
    if mode & 61440 == 8192 {
        return 99;
    }
    if mode & 61440 == 4096 {
        return 112;
    }
    if mode & 61440 == 49152 {
        return 115;
    }
    // non-standard types
    if S_ISDOOR!() != 0 {
        return 68;
    }
    if S_ISPORT!() != 0 {
        return 80;
    }
    if S_ISWHT!() != 0 {
        return 119;
    }
    // unknown
    63
}

fn fileperm(mode: u32, buf: *mut i8) {
    unsafe {
        *buf.wrapping_offset(0 as isize) = (if mode & 256 != 0 { 114i32 } else { 45i32 }) as i8;
    }
    unsafe {
        *buf.wrapping_offset(1 as isize) = (if mode & 128 != 0 { 119i32 } else { 45i32 }) as i8;
    }
    if mode & 2048 != 0 {
        unsafe {
            *buf.wrapping_offset(2 as isize) = (if mode & 64 != 0 { 115i32 } else { 83i32 }) as i8;
        }
    } else {
        unsafe {
            *buf.wrapping_offset(2 as isize) = (if mode & 64 != 0 { 120i32 } else { 45i32 }) as i8;
        }
    }
    unsafe {
        *buf.wrapping_offset(3 as isize) = (if mode & 32u32 != 0 { 114i32 } else { 45i32 }) as i8;
    }
    unsafe {
        *buf.wrapping_offset(4 as isize) = (if mode & 16u32 != 0 { 119i32 } else { 45i32 }) as i8;
    }
    if mode & 1024 != 0 {
        unsafe {
            *buf.wrapping_offset(5 as isize) =
                (if mode & 8u32 != 0 { 115i32 } else { 83i32 }) as i8;
        }
    } else {
        unsafe {
            *buf.wrapping_offset(5 as isize) =
                (if mode & 8u32 != 0 { 120i32 } else { 45i32 }) as i8;
        }
    }
    unsafe {
        *buf.wrapping_offset(6 as isize) = (if mode & 4u32 != 0 { 114i32 } else { 45i32 }) as i8;
    }
    unsafe {
        *buf.wrapping_offset(7 as isize) = (if mode & 2u32 != 0 { 119i32 } else { 45i32 }) as i8;
    }
    if mode & 512 != 0 {
        unsafe {
            *buf.wrapping_offset(8 as isize) =
                (if mode & 1u32 != 0 { 116i32 } else { 84i32 }) as i8;
        }
    } else {
        unsafe {
            *buf.wrapping_offset(8 as isize) =
                (if mode & 1u32 != 0 { 120i32 } else { 45i32 }) as i8;
        }
    }
}

static stat_filemode_doc: __SlateAlign16<[i8; 59]> = __SlateAlign16([
    67i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 32i8, 97i8, 32i8, 102i8, 105i8, 108i8, 101i8,
    39i8, 115i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 115i8,
    116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8,
    111i8, 114i8, 109i8, 32i8, 39i8, 45i8, 114i8, 119i8, 120i8, 114i8, 119i8, 120i8, 114i8, 119i8,
    120i8, 39i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_filemode")]
extern "C-unwind" fn stat_filemode(self_: *mut _object, omode: *mut _object) -> *mut _object {
    let mut buf: [i8; 10] = [0i8; 10];
    let mode: u32 = _PyLong_AsMode_t(omode);
    let __v0: bool = mode == (1i32.wrapping_neg() as u32) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    buf[0] = filetype(mode);
    fileperm(
        mode,
        (buf.as_mut_ptr() as *mut i8).wrapping_offset(1 as isize),
    );
    unsafe { PyUnicode_FromStringAndSize(buf.as_mut_ptr() as *const i8, 10i64) }
}

/// sentinel
static mut stat_methods: __SlateAlign16<[PyMethodDef; 14]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"S_ISDIR\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(stat_S_ISDIR);
        __t0.ml_flags = 8i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISDIR_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"S_ISCHR\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(stat_S_ISCHR);
        __t1.ml_flags = 8i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISCHR_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"S_ISBLK\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(stat_S_ISBLK);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISBLK_doc.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"S_ISREG\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(stat_S_ISREG);
        __t3.ml_flags = 8i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISREG_doc.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"S_ISFIFO\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(stat_S_ISFIFO);
        __t4.ml_flags = 8i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISFIFO_doc.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"S_ISLNK\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(stat_S_ISLNK);
        __t5.ml_flags = 8i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISLNK_doc.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"S_ISSOCK\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(stat_S_ISSOCK);
        __t6.ml_flags = 8i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISSOCK_doc.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"S_ISDOOR\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(stat_S_ISDOOR);
        __t7.ml_flags = 8i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISDOOR_doc.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"S_ISPORT\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(stat_S_ISPORT);
        __t8.ml_flags = 8i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISPORT_doc.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"S_ISWHT\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(stat_S_ISWHT);
        __t9.ml_flags = 8i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(stat_S_ISWHT_doc.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"S_IMODE\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(stat_S_IMODE);
        __t10.ml_flags = 8i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(stat_S_IMODE_doc.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"S_IFMT\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(stat_S_IFMT);
        __t11.ml_flags = 8i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(stat_S_IFMT_doc.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"filemode\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(stat_filemode);
        __t12.ml_flags = 8i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(stat_filemode_doc.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = core::ptr::null::<i8>();
        __t13.ml_meth = None;
        __t13
    },
]);

static module_doc: __SlateAlign16<[i8; 1908]> = __SlateAlign16([
    83i8, 95i8, 73i8, 70i8, 77i8, 84i8, 95i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 116i8,
    121i8, 112i8, 101i8, 32i8, 98i8, 105i8, 116i8, 115i8, 10i8, 83i8, 95i8, 73i8, 70i8, 68i8, 73i8,
    82i8, 58i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8, 114i8, 121i8, 10i8, 83i8,
    95i8, 73i8, 70i8, 67i8, 72i8, 82i8, 58i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8,
    101i8, 114i8, 32i8, 100i8, 101i8, 118i8, 105i8, 99i8, 101i8, 10i8, 83i8, 95i8, 73i8, 70i8,
    66i8, 76i8, 75i8, 58i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 32i8, 100i8, 101i8, 118i8,
    105i8, 99i8, 101i8, 10i8, 83i8, 95i8, 73i8, 70i8, 82i8, 69i8, 71i8, 58i8, 32i8, 114i8, 101i8,
    103i8, 117i8, 108i8, 97i8, 114i8, 32i8, 102i8, 105i8, 108i8, 101i8, 10i8, 83i8, 95i8, 73i8,
    70i8, 73i8, 70i8, 79i8, 58i8, 32i8, 102i8, 105i8, 102i8, 111i8, 32i8, 40i8, 110i8, 97i8, 109i8,
    101i8, 100i8, 32i8, 112i8, 105i8, 112i8, 101i8, 41i8, 10i8, 83i8, 95i8, 73i8, 70i8, 76i8, 78i8,
    75i8, 58i8, 32i8, 115i8, 121i8, 109i8, 98i8, 111i8, 108i8, 105i8, 99i8, 32i8, 108i8, 105i8,
    110i8, 107i8, 10i8, 83i8, 95i8, 73i8, 70i8, 83i8, 79i8, 67i8, 75i8, 58i8, 32i8, 115i8, 111i8,
    99i8, 107i8, 101i8, 116i8, 32i8, 102i8, 105i8, 108i8, 101i8, 10i8, 83i8, 95i8, 73i8, 70i8,
    68i8, 79i8, 79i8, 82i8, 58i8, 32i8, 100i8, 111i8, 111i8, 114i8, 10i8, 83i8, 95i8, 73i8, 70i8,
    80i8, 79i8, 82i8, 84i8, 58i8, 32i8, 101i8, 118i8, 101i8, 110i8, 116i8, 32i8, 112i8, 111i8,
    114i8, 116i8, 10i8, 83i8, 95i8, 73i8, 70i8, 87i8, 72i8, 84i8, 58i8, 32i8, 119i8, 104i8, 105i8,
    116i8, 101i8, 111i8, 117i8, 116i8, 10i8, 10i8, 83i8, 95i8, 73i8, 83i8, 85i8, 73i8, 68i8, 58i8,
    32i8, 115i8, 101i8, 116i8, 32i8, 85i8, 73i8, 68i8, 32i8, 98i8, 105i8, 116i8, 10i8, 83i8, 95i8,
    73i8, 83i8, 71i8, 73i8, 68i8, 58i8, 32i8, 115i8, 101i8, 116i8, 32i8, 71i8, 73i8, 68i8, 32i8,
    98i8, 105i8, 116i8, 10i8, 83i8, 95i8, 69i8, 78i8, 70i8, 77i8, 84i8, 58i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 32i8, 108i8, 111i8, 99i8, 107i8, 105i8, 110i8, 103i8, 32i8, 101i8, 110i8, 102i8,
    111i8, 114i8, 99i8, 101i8, 109i8, 101i8, 110i8, 116i8, 10i8, 83i8, 95i8, 73i8, 83i8, 86i8,
    84i8, 88i8, 58i8, 32i8, 115i8, 116i8, 105i8, 99i8, 107i8, 121i8, 32i8, 98i8, 105i8, 116i8,
    10i8, 83i8, 95i8, 73i8, 82i8, 69i8, 65i8, 68i8, 58i8, 32i8, 85i8, 110i8, 105i8, 120i8, 32i8,
    86i8, 55i8, 32i8, 115i8, 121i8, 110i8, 111i8, 110i8, 121i8, 109i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 83i8, 95i8, 73i8, 82i8, 85i8, 83i8, 82i8, 10i8, 83i8, 95i8, 73i8, 87i8, 82i8, 73i8, 84i8,
    69i8, 58i8, 32i8, 85i8, 110i8, 105i8, 120i8, 32i8, 86i8, 55i8, 32i8, 115i8, 121i8, 110i8,
    111i8, 110i8, 121i8, 109i8, 32i8, 102i8, 111i8, 114i8, 32i8, 83i8, 95i8, 73i8, 87i8, 85i8,
    83i8, 82i8, 10i8, 83i8, 95i8, 73i8, 69i8, 88i8, 69i8, 67i8, 58i8, 32i8, 85i8, 110i8, 105i8,
    120i8, 32i8, 86i8, 55i8, 32i8, 115i8, 121i8, 110i8, 111i8, 110i8, 121i8, 109i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 83i8, 95i8, 73i8, 88i8, 85i8, 83i8, 82i8, 10i8, 83i8, 95i8, 73i8, 82i8,
    87i8, 88i8, 85i8, 58i8, 32i8, 109i8, 97i8, 115i8, 107i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    111i8, 119i8, 110i8, 101i8, 114i8, 32i8, 112i8, 101i8, 114i8, 109i8, 105i8, 115i8, 115i8,
    105i8, 111i8, 110i8, 115i8, 10i8, 83i8, 95i8, 73i8, 82i8, 85i8, 83i8, 82i8, 58i8, 32i8, 114i8,
    101i8, 97i8, 100i8, 32i8, 98i8, 121i8, 32i8, 111i8, 119i8, 110i8, 101i8, 114i8, 10i8, 83i8,
    95i8, 73i8, 87i8, 85i8, 83i8, 82i8, 58i8, 32i8, 119i8, 114i8, 105i8, 116i8, 101i8, 32i8, 98i8,
    121i8, 32i8, 111i8, 119i8, 110i8, 101i8, 114i8, 10i8, 83i8, 95i8, 73i8, 88i8, 85i8, 83i8, 82i8,
    58i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 101i8, 32i8, 98i8, 121i8, 32i8, 111i8,
    119i8, 110i8, 101i8, 114i8, 10i8, 83i8, 95i8, 73i8, 82i8, 87i8, 88i8, 71i8, 58i8, 32i8, 109i8,
    97i8, 115i8, 107i8, 32i8, 102i8, 111i8, 114i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8, 32i8,
    112i8, 101i8, 114i8, 109i8, 105i8, 115i8, 115i8, 105i8, 111i8, 110i8, 115i8, 10i8, 83i8, 95i8,
    73i8, 82i8, 71i8, 82i8, 80i8, 58i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 98i8, 121i8, 32i8,
    103i8, 114i8, 111i8, 117i8, 112i8, 10i8, 83i8, 95i8, 73i8, 87i8, 71i8, 82i8, 80i8, 58i8, 32i8,
    119i8, 114i8, 105i8, 116i8, 101i8, 32i8, 98i8, 121i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8,
    10i8, 83i8, 95i8, 73i8, 88i8, 71i8, 82i8, 80i8, 58i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8,
    116i8, 101i8, 32i8, 98i8, 121i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8, 10i8, 83i8, 95i8,
    73i8, 82i8, 87i8, 88i8, 79i8, 58i8, 32i8, 109i8, 97i8, 115i8, 107i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 115i8, 32i8, 40i8, 110i8, 111i8, 116i8, 32i8, 105i8,
    110i8, 32i8, 103i8, 114i8, 111i8, 117i8, 112i8, 41i8, 32i8, 112i8, 101i8, 114i8, 109i8, 105i8,
    115i8, 115i8, 105i8, 111i8, 110i8, 115i8, 10i8, 83i8, 95i8, 73i8, 82i8, 79i8, 84i8, 72i8, 58i8,
    32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 98i8, 121i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8,
    115i8, 10i8, 83i8, 95i8, 73i8, 87i8, 79i8, 84i8, 72i8, 58i8, 32i8, 119i8, 114i8, 105i8, 116i8,
    101i8, 32i8, 98i8, 121i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 115i8, 10i8, 83i8, 95i8,
    73i8, 88i8, 79i8, 84i8, 72i8, 58i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 101i8, 32i8,
    98i8, 121i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 115i8, 10i8, 10i8, 85i8, 70i8, 95i8,
    83i8, 69i8, 84i8, 84i8, 65i8, 66i8, 76i8, 69i8, 58i8, 32i8, 109i8, 97i8, 115i8, 107i8, 32i8,
    111i8, 102i8, 32i8, 111i8, 119i8, 110i8, 101i8, 114i8, 32i8, 99i8, 104i8, 97i8, 110i8, 103i8,
    101i8, 97i8, 98i8, 108i8, 101i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 10i8, 85i8, 70i8,
    95i8, 78i8, 79i8, 68i8, 85i8, 77i8, 80i8, 58i8, 32i8, 100i8, 111i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 100i8, 117i8, 109i8, 112i8, 32i8, 102i8, 105i8, 108i8, 101i8, 10i8, 85i8, 70i8, 95i8,
    73i8, 77i8, 77i8, 85i8, 84i8, 65i8, 66i8, 76i8, 69i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8,
    32i8, 109i8, 97i8, 121i8, 32i8, 110i8, 111i8, 116i8, 32i8, 98i8, 101i8, 32i8, 99i8, 104i8,
    97i8, 110i8, 103i8, 101i8, 100i8, 10i8, 85i8, 70i8, 95i8, 65i8, 80i8, 80i8, 69i8, 78i8, 68i8,
    58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 109i8, 97i8, 121i8, 32i8, 111i8, 110i8, 108i8,
    121i8, 32i8, 98i8, 101i8, 32i8, 97i8, 112i8, 112i8, 101i8, 110i8, 100i8, 101i8, 100i8, 32i8,
    116i8, 111i8, 10i8, 85i8, 70i8, 95i8, 79i8, 80i8, 65i8, 81i8, 85i8, 69i8, 58i8, 32i8, 100i8,
    105i8, 114i8, 101i8, 99i8, 116i8, 111i8, 114i8, 121i8, 32i8, 105i8, 115i8, 32i8, 111i8, 112i8,
    97i8, 113i8, 117i8, 101i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 118i8, 105i8, 101i8, 119i8,
    101i8, 100i8, 32i8, 116i8, 104i8, 114i8, 111i8, 117i8, 103i8, 104i8, 32i8, 97i8, 32i8, 117i8,
    110i8, 105i8, 111i8, 110i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 10i8, 85i8, 70i8, 95i8,
    78i8, 79i8, 85i8, 78i8, 76i8, 73i8, 78i8, 75i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8,
    109i8, 97i8, 121i8, 32i8, 110i8, 111i8, 116i8, 32i8, 98i8, 101i8, 32i8, 114i8, 101i8, 110i8,
    97i8, 109i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8, 100i8, 101i8, 108i8, 101i8, 116i8, 101i8,
    100i8, 10i8, 85i8, 70i8, 95i8, 67i8, 79i8, 77i8, 80i8, 82i8, 69i8, 83i8, 83i8, 69i8, 68i8,
    58i8, 32i8, 109i8, 97i8, 99i8, 79i8, 83i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 105i8,
    115i8, 32i8, 104i8, 102i8, 115i8, 45i8, 99i8, 111i8, 109i8, 112i8, 114i8, 101i8, 115i8, 115i8,
    101i8, 100i8, 10i8, 85i8, 70i8, 95i8, 84i8, 82i8, 65i8, 67i8, 75i8, 69i8, 68i8, 58i8, 32i8,
    117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 100i8, 101i8, 97i8, 108i8, 105i8,
    110i8, 103i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 100i8, 111i8, 99i8, 117i8, 109i8, 101i8,
    110i8, 116i8, 32i8, 73i8, 68i8, 115i8, 10i8, 85i8, 70i8, 95i8, 68i8, 65i8, 84i8, 65i8, 86i8,
    65i8, 85i8, 76i8, 84i8, 58i8, 32i8, 101i8, 110i8, 116i8, 105i8, 116i8, 108i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 32i8, 114i8, 101i8, 113i8, 117i8, 105i8, 114i8, 101i8, 100i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 114i8, 101i8, 97i8, 100i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 119i8, 114i8, 105i8, 116i8, 105i8, 110i8, 103i8, 10i8, 85i8, 70i8, 95i8, 72i8, 73i8,
    68i8, 68i8, 69i8, 78i8, 58i8, 32i8, 109i8, 97i8, 99i8, 79i8, 83i8, 58i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 32i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8, 32i8, 110i8, 111i8, 116i8, 32i8,
    98i8, 101i8, 32i8, 100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 101i8, 100i8, 10i8, 83i8,
    70i8, 95i8, 83i8, 69i8, 84i8, 84i8, 65i8, 66i8, 76i8, 69i8, 58i8, 32i8, 109i8, 97i8, 115i8,
    107i8, 32i8, 111i8, 102i8, 32i8, 115i8, 117i8, 112i8, 101i8, 114i8, 32i8, 117i8, 115i8, 101i8,
    114i8, 32i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 97i8, 98i8, 108i8, 101i8, 32i8, 102i8,
    108i8, 97i8, 103i8, 115i8, 10i8, 83i8, 70i8, 95i8, 65i8, 82i8, 67i8, 72i8, 73i8, 86i8, 69i8,
    68i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8,
    32i8, 97i8, 114i8, 99i8, 104i8, 105i8, 118i8, 101i8, 100i8, 10i8, 83i8, 70i8, 95i8, 73i8, 77i8,
    77i8, 85i8, 84i8, 65i8, 66i8, 76i8, 69i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 109i8,
    97i8, 121i8, 32i8, 110i8, 111i8, 116i8, 32i8, 98i8, 101i8, 32i8, 99i8, 104i8, 97i8, 110i8,
    103i8, 101i8, 100i8, 10i8, 83i8, 70i8, 95i8, 65i8, 80i8, 80i8, 69i8, 78i8, 68i8, 58i8, 32i8,
    102i8, 105i8, 108i8, 101i8, 32i8, 109i8, 97i8, 121i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8,
    98i8, 101i8, 32i8, 97i8, 112i8, 112i8, 101i8, 110i8, 100i8, 101i8, 100i8, 32i8, 116i8, 111i8,
    10i8, 83i8, 70i8, 95i8, 82i8, 69i8, 83i8, 84i8, 82i8, 73i8, 67i8, 84i8, 69i8, 68i8, 58i8, 32i8,
    101i8, 110i8, 116i8, 105i8, 116i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 114i8,
    101i8, 113i8, 117i8, 105i8, 114i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 119i8, 114i8,
    105i8, 116i8, 105i8, 110i8, 103i8, 10i8, 83i8, 70i8, 95i8, 78i8, 79i8, 85i8, 78i8, 76i8, 73i8,
    78i8, 75i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 109i8, 97i8, 121i8, 32i8, 110i8,
    111i8, 116i8, 32i8, 98i8, 101i8, 32i8, 114i8, 101i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8,
    111i8, 114i8, 32i8, 100i8, 101i8, 108i8, 101i8, 116i8, 101i8, 100i8, 10i8, 83i8, 70i8, 95i8,
    83i8, 78i8, 65i8, 80i8, 83i8, 72i8, 79i8, 84i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8,
    105i8, 115i8, 32i8, 97i8, 32i8, 115i8, 110i8, 97i8, 112i8, 115i8, 104i8, 111i8, 116i8, 32i8,
    102i8, 105i8, 108i8, 101i8, 10i8, 83i8, 70i8, 95i8, 70i8, 73i8, 82i8, 77i8, 76i8, 73i8, 78i8,
    75i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 32i8, 102i8,
    105i8, 114i8, 109i8, 108i8, 105i8, 110i8, 107i8, 10i8, 83i8, 70i8, 95i8, 68i8, 65i8, 84i8,
    65i8, 76i8, 69i8, 83i8, 83i8, 58i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8,
    97i8, 32i8, 100i8, 97i8, 116i8, 97i8, 108i8, 101i8, 115i8, 115i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 10i8, 10i8, 79i8, 110i8, 32i8, 109i8, 97i8, 99i8, 79i8, 83i8, 58i8, 10i8,
    83i8, 70i8, 95i8, 83i8, 85i8, 80i8, 80i8, 79i8, 82i8, 84i8, 69i8, 68i8, 58i8, 32i8, 109i8,
    97i8, 115i8, 107i8, 32i8, 111i8, 102i8, 32i8, 115i8, 117i8, 112i8, 101i8, 114i8, 32i8, 117i8,
    115i8, 101i8, 114i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 101i8, 100i8, 32i8,
    102i8, 108i8, 97i8, 103i8, 115i8, 10i8, 83i8, 70i8, 95i8, 83i8, 89i8, 78i8, 84i8, 72i8, 69i8,
    84i8, 73i8, 67i8, 58i8, 32i8, 109i8, 97i8, 115i8, 107i8, 32i8, 111i8, 102i8, 32i8, 114i8,
    101i8, 97i8, 100i8, 45i8, 111i8, 110i8, 108i8, 121i8, 32i8, 115i8, 121i8, 110i8, 116i8, 104i8,
    101i8, 116i8, 105i8, 99i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 10i8, 10i8, 83i8, 84i8,
    95i8, 77i8, 79i8, 68i8, 69i8, 10i8, 83i8, 84i8, 95i8, 73i8, 78i8, 79i8, 10i8, 83i8, 84i8, 95i8,
    68i8, 69i8, 86i8, 10i8, 83i8, 84i8, 95i8, 78i8, 76i8, 73i8, 78i8, 75i8, 10i8, 83i8, 84i8, 95i8,
    85i8, 73i8, 68i8, 10i8, 83i8, 84i8, 95i8, 71i8, 73i8, 68i8, 10i8, 83i8, 84i8, 95i8, 83i8, 73i8,
    90i8, 69i8, 10i8, 83i8, 84i8, 95i8, 65i8, 84i8, 73i8, 77i8, 69i8, 10i8, 83i8, 84i8, 95i8, 77i8,
    84i8, 73i8, 77i8, 69i8, 10i8, 83i8, 84i8, 95i8, 67i8, 84i8, 73i8, 77i8, 69i8, 10i8, 10i8, 70i8,
    73i8, 76i8, 69i8, 95i8, 65i8, 84i8, 84i8, 82i8, 73i8, 66i8, 85i8, 84i8, 69i8, 95i8, 42i8, 58i8,
    32i8, 87i8, 105i8, 110i8, 100i8, 111i8, 119i8, 115i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8,
    97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 32i8, 99i8, 111i8, 110i8, 115i8,
    116i8, 97i8, 110i8, 116i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 40i8, 111i8, 110i8, 108i8, 121i8,
    32i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 32i8, 111i8, 110i8, 32i8, 87i8, 105i8,
    110i8, 100i8, 111i8, 119i8, 115i8, 41i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._stat.stat_exec")]
extern "C-unwind" fn stat_exec(module: *mut _object) -> i32 {
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFDIR".as_ptr(), 16384i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFCHR".as_ptr(), 8192i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFBLK".as_ptr(), 24576i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFREG".as_ptr(), 32768i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFIFO".as_ptr(), 4096i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFLNK".as_ptr(), 40960i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFSOCK".as_ptr(), 49152i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFDOOR".as_ptr(), 0i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFPORT".as_ptr(), 0i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IFWHT".as_ptr(), 0i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_ISUID".as_ptr(), 2048i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_ISGID".as_ptr(), 1024i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_ISVTX".as_ptr(), 512i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_ENFMT".as_ptr(), 1024i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IREAD".as_ptr(), 256i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IWRITE".as_ptr(), 128i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IEXEC".as_ptr(), 64i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IRWXU".as_ptr(), 448i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IRUSR".as_ptr(), 256i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IWUSR".as_ptr(), 128i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IXUSR".as_ptr(), 64i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IRWXG".as_ptr(), 56i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IRGRP".as_ptr(), 32i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IWGRP".as_ptr(), 16i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IXGRP".as_ptr(), 8i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IRWXO".as_ptr(), 7i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IROTH".as_ptr(), 4i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IWOTH".as_ptr(), 2i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"S_IXOTH".as_ptr(), 1i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_SETTABLE".as_ptr(), 65535i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_NODUMP".as_ptr(), 1i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_IMMUTABLE".as_ptr(), 2i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_APPEND".as_ptr(), 4i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_OPAQUE".as_ptr(), 8i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_NOUNLINK".as_ptr(), 16i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_COMPRESSED".as_ptr(), 32i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_TRACKED".as_ptr(), 64i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_DATAVAULT".as_ptr(), 128i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"UF_HIDDEN".as_ptr(), 32768i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_SETTABLE".as_ptr(), 4294901760i64) }) < 0
        {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_ARCHIVED".as_ptr(), 65536i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_IMMUTABLE".as_ptr(), 131072i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_APPEND".as_ptr(), 262144i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_NOUNLINK".as_ptr(), 1048576i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_SNAPSHOT".as_ptr(), 2097152i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_FIRMLINK".as_ptr(), 8388608i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"SF_DATALESS".as_ptr(), 1073741824i64) }) < 0
        {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    let st_constants: __SlateAlign16<[*const i8; 10]> = __SlateAlign16([
        c"ST_MODE".as_ptr(),
        c"ST_INO".as_ptr(),
        c"ST_DEV".as_ptr(),
        c"ST_NLINK".as_ptr(),
        c"ST_UID".as_ptr(),
        c"ST_GID".as_ptr(),
        c"ST_SIZE".as_ptr(),
        c"ST_ATIME".as_ptr(),
        c"ST_MTIME".as_ptr(),
        c"ST_CTIME".as_ptr(),
    ]);
    let mut i: i32 = 0;
    while i < (Py_ARRAY_LENGTH__22!() as i32) {
        if (unsafe { PyModule_AddIntConstant(module, st_constants.0[i as usize], i as i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        i = i.wrapping_add(1);
    }
    0
}

static mut stat_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(stat_exec),
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

static mut statmodule: PyModuleDef = {
    let mut __t0: PyModuleDef = unsafe { core::mem::zeroed() };
    __t0.m_base = PyModuleDef_Base {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        m_init: None,
        m_index: 0i64,
        m_copy: core::ptr::null_mut::<_object>(),
    };
    __t0.m_name = (b"_stat\0".as_ptr() as *mut i8) as *const i8;
    __t0.m_doc = unsafe { core::ptr::addr_of!(module_doc.0) as *const i8 };
    __t0.m_size = 0i64;
    __t0.m_methods = unsafe { core::ptr::addr_of_mut!(stat_methods.0) as *mut PyMethodDef };
    __t0.m_slots = unsafe { core::ptr::addr_of_mut!(stat_slots.0) as *mut PyModuleDef_Slot };
    __t0
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct._stat.PyInit__stat")]
extern "C-unwind" fn PyInit__stat() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(statmodule)) }
}
