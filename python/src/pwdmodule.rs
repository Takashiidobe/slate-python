//! UNIX password file access module
//! Define macros for inline documentation.
//! PyObject_HEAD defines the initial segment of every PyObject.
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
// Signed variants of the above
macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyUnicode_Check__2 {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE__3!($op), 268435456u64)
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

macro_rules! Py_NewRef__3 {
    ($obj:expr) => {
        _Py_NewRef($obj)
    };
}

macro_rules! Py_TYPE__3 {
    ($ob:expr) => {
        _Py_TYPE($ob)
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

unsafe extern "C" {
    static mut PyExc_KeyError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    fn PyMem_RawRealloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyMem_RawFree(arg0: *mut ());
    fn PyType_GetFlags(arg0: *mut _typeobject) -> u64;
    fn Py_GetConstantBorrowed(arg0: u32) -> *mut _object;
    fn _Py_IncRef(arg0: *mut _object);
    fn _Py_DecRef(arg0: *mut _object);
    fn PyBytes_AsStringAndSize(arg0: *mut _object, arg1: *mut *mut i8, arg2: *mut i64) -> i32;
    fn PyUnicode_DecodeFSDefault(arg0: *const i8) -> *mut _object;
    fn PyUnicode_EncodeFSDefault(arg0: *mut _object) -> *mut _object;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyModule_GetState(arg0: *mut _object) -> *mut ();
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn PyStructSequence_NewType(arg0: *mut PyStructSequence_Desc) -> *mut _typeobject;
    fn PyStructSequence_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyStructSequence_SetItem(arg0: *mut _object, arg1: i64, arg2: *mut _object);
    fn PyModule_AddType(arg0: *mut _object, arg1: *mut _typeobject) -> i32;
    fn PyEval_SaveThread() -> *mut _ts;
    fn PyEval_RestoreThread(arg0: *mut _ts);
    fn _PyLong_FromUid(arg0: u32) -> *mut _object;
    fn _PyLong_FromGid(arg0: u32) -> *mut _object;
    fn _Py_Uid_Converter(arg0: *mut _object, arg1: *mut u32) -> i32;
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

#[inline(always)]
fn Py_INCREF(op: *mut _object) {
    // Stable ABI implements Py_INCREF() as a function call on limited C API
    //
    // version 3.12 and newer, and on Python built in debug mode. _Py_IncRef()
    //
    // was added to Python 3.10.0a7, use Py_IncRef() on older Python versions.
    //
    // Py_IncRef() accepts NULL whereas _Py_IncRef() doesn't.
    unsafe { _Py_IncRef(op) };
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _typeobject {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ts {}

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

fn Py_DECREF(op: *mut _object) {
    unsafe { _Py_DecRef(op) };
}

fn Py_XDECREF(op: *mut _object) {
    if !op.is_null() {
        Py_DECREF!(op);
    }
}

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF(obj);
    obj
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

#[repr(C)]
#[derive(Clone, Copy)]
struct PyStructSequence_Field {
    name: *const i8,
    doc: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyStructSequence_Desc {
    name: *const i8,
    doc: *const i8,
    fields: *mut PyStructSequence_Field,
    n_in_sequence: i32,
}

static pwd_getpwuid__doc__: __SlateAlign16<[i8; 155]> = __SlateAlign16([
    103i8, 101i8, 116i8, 112i8, 119i8, 117i8, 105i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 117i8, 105i8, 100i8, 111i8, 98i8, 106i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 112i8, 97i8, 115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8, 116i8,
    97i8, 98i8, 97i8, 115i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 121i8, 32i8, 102i8, 111i8,
    114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 110i8, 117i8,
    109i8, 101i8, 114i8, 105i8, 99i8, 32i8, 117i8, 115i8, 101i8, 114i8, 32i8, 73i8, 68i8, 46i8,
    10i8, 10i8, 83i8, 101i8, 101i8, 32i8, 96i8, 104i8, 101i8, 108i8, 112i8, 40i8, 112i8, 119i8,
    100i8, 41i8, 96i8, 32i8, 102i8, 111i8, 114i8, 32i8, 109i8, 111i8, 114i8, 101i8, 32i8, 111i8,
    110i8, 32i8, 112i8, 97i8, 115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8, 116i8,
    97i8, 98i8, 97i8, 115i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 105i8, 101i8, 115i8, 46i8,
    0i8,
]);

static pwd_getpwnam__doc__: __SlateAlign16<[i8; 147]> = __SlateAlign16([
    103i8, 101i8, 116i8, 112i8, 119i8, 110i8, 97i8, 109i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8,
    97i8, 115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 98i8, 97i8,
    115i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 121i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 117i8, 115i8, 101i8, 114i8, 32i8,
    110i8, 97i8, 109i8, 101i8, 46i8, 10i8, 10i8, 83i8, 101i8, 101i8, 32i8, 96i8, 104i8, 101i8,
    108i8, 112i8, 40i8, 112i8, 119i8, 100i8, 41i8, 96i8, 32i8, 102i8, 111i8, 114i8, 32i8, 109i8,
    111i8, 114i8, 101i8, 32i8, 111i8, 110i8, 32i8, 112i8, 97i8, 115i8, 115i8, 119i8, 111i8, 114i8,
    100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 98i8, 97i8, 115i8, 101i8, 32i8, 101i8, 110i8, 116i8,
    114i8, 105i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwd_getpwnam")]
extern "C-unwind" fn pwd_getpwnam(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check__2!(arg) == 0 {
            PyErr_Format(
                PyExc_TypeError,
                c"getpwnam() argument must be str, not %T".as_ptr(),
                arg,
            );
        } else {
            let name: *mut _object = arg;
            return_value = pwd_getpwnam_impl(module, name);
        }
        return return_value;
    }
}

static pwd_getpwall__doc__: __SlateAlign16<[i8; 157]> = __SlateAlign16([
    103i8, 101i8, 116i8, 112i8, 119i8, 97i8, 108i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 111i8, 102i8, 32i8, 97i8,
    108i8, 108i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8, 112i8,
    97i8, 115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 98i8, 97i8,
    115i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 105i8, 101i8, 115i8, 44i8, 32i8, 105i8, 110i8,
    32i8, 97i8, 114i8, 98i8, 105i8, 116i8, 114i8, 97i8, 114i8, 121i8, 32i8, 111i8, 114i8, 100i8,
    101i8, 114i8, 46i8, 10i8, 10i8, 83i8, 101i8, 101i8, 32i8, 104i8, 101i8, 108i8, 112i8, 40i8,
    112i8, 119i8, 100i8, 41i8, 32i8, 102i8, 111i8, 114i8, 32i8, 109i8, 111i8, 114i8, 101i8, 32i8,
    111i8, 110i8, 32i8, 112i8, 97i8, 115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8,
    116i8, 97i8, 98i8, 97i8, 115i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 105i8, 101i8, 115i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwd_getpwall")]
extern "C-unwind" fn pwd_getpwall(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    pwd_getpwall_impl(module)
}

use slate_support::endpwent;
use slate_support::getpwent;
use slate_support::getpwnam_r;
use slate_support::getpwuid_r;
use slate_support::setpwent;
use slate_support::sysconf;

// Need limited C API version 3.13 for PyMem_RawRealloc()
// Py_GIL_DISABLED
// ERANGE
// getpwuid()
// sysconf()
// [clinic input]
// module pwd
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=60f628ef356b97b6]
static mut struct_pwd_type_fields: __SlateAlign16<[PyStructSequence_Field; 8]> = __SlateAlign16([
    PyStructSequence_Field {
        name: (b"pw_name\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"user name\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"pw_passwd\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"password\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"pw_uid\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"user id\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"pw_gid\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"group id\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"pw_gecos\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"real name\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"pw_dir\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"home directory\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"pw_shell\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"shell program\0".as_ptr() as *mut i8) as *const i8,
    },
    {
        let mut __t0: PyStructSequence_Field = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

static struct_passwd__doc__: __SlateAlign16<[i8; 220]> = __SlateAlign16([
    112i8, 119i8, 100i8, 46i8, 115i8, 116i8, 114i8, 117i8, 99i8, 116i8, 95i8, 112i8, 97i8, 115i8,
    115i8, 119i8, 100i8, 58i8, 32i8, 82i8, 101i8, 115i8, 117i8, 108i8, 116i8, 115i8, 32i8, 102i8,
    114i8, 111i8, 109i8, 32i8, 103i8, 101i8, 116i8, 112i8, 119i8, 42i8, 40i8, 41i8, 32i8, 114i8,
    111i8, 117i8, 116i8, 105i8, 110i8, 101i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 109i8, 97i8, 121i8, 32i8, 98i8, 101i8,
    32i8, 97i8, 99i8, 99i8, 101i8, 115i8, 115i8, 101i8, 100i8, 32i8, 101i8, 105i8, 116i8, 104i8,
    101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8,
    111i8, 102i8, 10i8, 32i8, 32i8, 40i8, 112i8, 119i8, 95i8, 110i8, 97i8, 109i8, 101i8, 44i8,
    112i8, 119i8, 95i8, 112i8, 97i8, 115i8, 115i8, 119i8, 100i8, 44i8, 112i8, 119i8, 95i8, 117i8,
    105i8, 100i8, 44i8, 112i8, 119i8, 95i8, 103i8, 105i8, 100i8, 44i8, 112i8, 119i8, 95i8, 103i8,
    101i8, 99i8, 111i8, 115i8, 44i8, 112i8, 119i8, 95i8, 100i8, 105i8, 114i8, 44i8, 112i8, 119i8,
    95i8, 115i8, 104i8, 101i8, 108i8, 108i8, 41i8, 10i8, 111i8, 114i8, 32i8, 118i8, 105i8, 97i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 97i8, 116i8,
    116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 32i8, 97i8, 115i8, 32i8, 110i8, 97i8,
    109i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 98i8, 111i8,
    118i8, 101i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 46i8, 0i8,
]);

static mut struct_pwd_type_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"pwd.struct_passwd\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(struct_passwd__doc__.0) as *const i8 };
    __t0.fields =
        unsafe { core::ptr::addr_of_mut!(struct_pwd_type_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 7i32;
    __t0
};

static pwd__doc__: __SlateAlign16<[i8; 421]> = __SlateAlign16([
    84i8, 104i8, 105i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 112i8, 114i8,
    111i8, 118i8, 105i8, 100i8, 101i8, 115i8, 32i8, 97i8, 99i8, 99i8, 101i8, 115i8, 115i8, 32i8,
    116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 85i8, 110i8, 105i8, 120i8, 32i8, 112i8, 97i8,
    115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 98i8, 97i8, 115i8,
    101i8, 46i8, 10i8, 73i8, 116i8, 32i8, 105i8, 115i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 110i8, 32i8, 97i8, 108i8, 108i8, 32i8, 85i8, 110i8,
    105i8, 120i8, 32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 115i8, 46i8, 10i8, 10i8,
    80i8, 97i8, 115i8, 115i8, 119i8, 111i8, 114i8, 100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 98i8,
    97i8, 115i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 105i8, 101i8, 115i8, 32i8, 97i8, 114i8,
    101i8, 32i8, 114i8, 101i8, 112i8, 111i8, 114i8, 116i8, 101i8, 100i8, 32i8, 97i8, 115i8, 32i8,
    55i8, 45i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8,
    105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 111i8, 108i8, 108i8,
    111i8, 119i8, 105i8, 110i8, 103i8, 10i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 102i8, 114i8,
    111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 97i8, 115i8, 115i8, 119i8, 111i8, 114i8,
    100i8, 32i8, 100i8, 97i8, 116i8, 97i8, 98i8, 97i8, 115i8, 101i8, 32i8, 40i8, 115i8, 101i8,
    101i8, 32i8, 96i8, 60i8, 112i8, 119i8, 100i8, 46i8, 104i8, 62i8, 39i8, 41i8, 44i8, 32i8, 105i8,
    110i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 58i8, 10i8, 112i8, 119i8, 95i8, 110i8, 97i8,
    109i8, 101i8, 44i8, 32i8, 112i8, 119i8, 95i8, 112i8, 97i8, 115i8, 115i8, 119i8, 100i8, 44i8,
    32i8, 112i8, 119i8, 95i8, 117i8, 105i8, 100i8, 44i8, 32i8, 112i8, 119i8, 95i8, 103i8, 105i8,
    100i8, 44i8, 32i8, 112i8, 119i8, 95i8, 103i8, 101i8, 99i8, 111i8, 115i8, 44i8, 32i8, 112i8,
    119i8, 95i8, 100i8, 105i8, 114i8, 44i8, 32i8, 112i8, 119i8, 95i8, 115i8, 104i8, 101i8, 108i8,
    108i8, 46i8, 10i8, 84i8, 104i8, 101i8, 32i8, 117i8, 105i8, 100i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 103i8, 105i8, 100i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 97i8, 114i8, 101i8,
    32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 44i8, 32i8, 97i8, 108i8, 108i8,
    32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 115i8, 116i8,
    114i8, 105i8, 110i8, 103i8, 115i8, 46i8, 32i8, 65i8, 110i8, 10i8, 101i8, 120i8, 99i8, 101i8,
    112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 114i8, 97i8, 105i8, 115i8, 101i8,
    100i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 110i8, 116i8, 114i8, 121i8,
    32i8, 97i8, 115i8, 107i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 99i8, 97i8, 110i8,
    110i8, 111i8, 116i8, 32i8, 98i8, 101i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 46i8, 0i8,
]);

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pwdmodule_c_60 {
    StructPwdType: *mut _typeobject,
}

fn get_pwd_state(module: *mut _object) -> *mut __SlateRecord_pwdmodule_c_60 {
    let state: *mut () = unsafe { PyModule_GetState(module) };
    state as *mut __SlateRecord_pwdmodule_c_60
}

static mut pwdmodule: PyModuleDef = PyModuleDef {
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
    m_name: (b"pwd\0".as_ptr() as *mut i8) as *const i8,
    m_doc: unsafe { core::ptr::addr_of!(pwd__doc__.0) as *const i8 },
    m_size: (core::mem::size_of::<__SlateRecord_pwdmodule_c_60>() as u64) as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(pwd_methods.0) as *mut PyMethodDef },
    m_slots: unsafe { core::ptr::addr_of_mut!(pwdmodule_slots.0) as *mut PyModuleDef_Slot },
    m_traverse: Some(pwdmodule_traverse),
    m_clear: Some(pwdmodule_clear),
    m_free: Some(pwdmodule_free),
};

fn mkpwent(module: *mut _object, p: *mut slate_support::passwd) -> *mut _object {
    let mut __v2: *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v6: *mut _object = unsafe { core::mem::zeroed() };
    let mut item_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v10: *mut _object = unsafe { core::mem::zeroed() };
    let mut item_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut item_3: *mut _object = unsafe { core::mem::zeroed() };
    let mut item_4: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v20: *mut _object = unsafe { core::mem::zeroed() };
    let mut item_5: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v24: *mut _object = unsafe { core::mem::zeroed() };
    let mut item_6: *mut _object = unsafe { core::mem::zeroed() };
    let mut setIndex: i32 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        v = PyStructSequence_New((*get_pwd_state(module)).StructPwdType);
        if v.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            setIndex = 0;
            '__join_0: {
                loop {
                    if !unsafe { (*p).pw_name }.is_null() {
                        __v24 = PyUnicode_DecodeFSDefault((*p).pw_name as *const i8);
                    } else {
                        __v24 = Py_NewRef__3!(Py_GetConstantBorrowed(0u32));
                    }
                    item_6 = __v24;
                    if item_6.is_null() {
                        break '__join_0;
                    } else {
                        let __v23: i32 = setIndex;
                        setIndex = __v23.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v23 as i64, item_6);
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    if !unsafe { (*p).pw_passwd }.is_null() {
                        __v20 = PyUnicode_DecodeFSDefault((*p).pw_passwd as *const i8);
                    } else {
                        __v20 = Py_NewRef__3!(Py_GetConstantBorrowed(0u32));
                    }
                    item_5 = __v20;
                    if item_5.is_null() {
                        break '__join_0;
                    } else {
                        let __v19: i32 = setIndex;
                        setIndex = __v19.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v19 as i64, item_5);
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    item_4 = _PyLong_FromUid((*p).pw_uid);
                    if item_4.is_null() {
                        break '__join_0;
                    } else {
                        let __v16: i32 = setIndex;
                        setIndex = __v16.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v16 as i64, item_4);
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    item_3 = _PyLong_FromGid((*p).pw_gid);
                    if item_3.is_null() {
                        break '__join_0;
                    } else {
                        let __v13: i32 = setIndex;
                        setIndex = __v13.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v13 as i64, item_3);
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    if !unsafe { (*p).pw_gecos }.is_null() {
                        __v10 = PyUnicode_DecodeFSDefault((*p).pw_gecos as *const i8);
                    } else {
                        __v10 = Py_NewRef__3!(Py_GetConstantBorrowed(0u32));
                    }
                    item_2 = __v10;
                    if item_2.is_null() {
                        break '__join_0;
                    } else {
                        let __v9: i32 = setIndex;
                        setIndex = __v9.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v9 as i64, item_2);
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    if !unsafe { (*p).pw_dir }.is_null() {
                        __v6 = PyUnicode_DecodeFSDefault((*p).pw_dir as *const i8);
                    } else {
                        __v6 = Py_NewRef__3!(Py_GetConstantBorrowed(0u32));
                    }
                    item_1 = __v6;
                    if item_1.is_null() {
                        break '__join_0;
                    } else {
                        let __v5: i32 = setIndex;
                        setIndex = __v5.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v5 as i64, item_1);
                        if true {
                            break;
                        }
                    }
                }
                loop {
                    if !unsafe { (*p).pw_shell }.is_null() {
                        __v2 = PyUnicode_DecodeFSDefault((*p).pw_shell as *const i8);
                    } else {
                        __v2 = Py_NewRef__3!(Py_GetConstantBorrowed(0u32));
                    }
                    item = __v2;
                    if item.is_null() {
                        break '__join_0;
                    } else {
                        let __v1: i32 = setIndex;
                        setIndex = __v1.wrapping_add(1);
                        PyStructSequence_SetItem(v, __v1 as i64, item);
                        if true {
                            break;
                        }
                    }
                }
                return v;
            }
            Py_DECREF!(v);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// [clinic input]
// pwd.getpwuid
//
//     uidobj: object
//     /
//
// Return the password database entry for the given numeric user ID.
//
// See `help(pwd)` for more on password database entries.
// [clinic start generated code]
#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwd_getpwuid")]
extern "C-unwind" fn pwd_getpwuid(module: *mut _object, uidobj: *mut _object) -> *mut _object {
    let mut retval: *mut _object = core::ptr::null_mut::<_object>();
    let mut uid: u32 = 0;
    let mut nomem: i32 = 0;
    let mut p: *mut slate_support::passwd = unsafe { core::mem::zeroed() };
    let mut buf: *mut i8 = core::ptr::null_mut::<i8>();
    let mut buf2: *mut i8 = core::ptr::null_mut::<i8>();
    if (unsafe { _Py_Uid_Converter(uidobj, core::ptr::addr_of_mut!(uid)) }) == 0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            unsafe { PyErr_Format(PyExc_KeyError, c"getpwuid(): uid not found".as_ptr()) };
        }
        return core::ptr::null_mut::<_object>();
    }
    let mut status: i32 = 0;
    let mut bufsize: i64 = 0;
    // Note: 'pwd' will be used via pointer 'p' on getpwuid_r success.
    let mut pwd: slate_support::passwd = unsafe { core::mem::zeroed() };
    let _save: *mut _ts = unsafe { PyEval_SaveThread() };
    bufsize = unsafe { sysconf((slate_support::_SC_GETPW_R_SIZE_MAX as i32) as i32) as i64 };
    if bufsize == (1i32.wrapping_neg() as i64) {
        bufsize = 1024;
    }
    while true {
        buf2 = (unsafe { PyMem_RawRealloc(buf as *mut (), bufsize as u64) }) as *mut i8;
        if buf2.is_null() {
            p = core::ptr::null_mut::<slate_support::passwd>();
            nomem = 1;
            break;
        }
        buf = buf2;
        status = unsafe {
            getpwuid_r(
                uid as u32,
                core::ptr::addr_of_mut!(pwd) as *mut slate_support::passwd,
                buf as *mut core::ffi::c_char,
                (bufsize as u64) as usize,
                core::ptr::addr_of_mut!(p) as *mut *mut slate_support::passwd,
            ) as i32
        };
        if status != 0 {
            p = core::ptr::null_mut::<slate_support::passwd>();
        }
        if !p.is_null() || status != (slate_support::ERANGE as i32) {
            break;
        }
        if bufsize > 4611686018427387903i64 {
            nomem = 1;
            break;
        }
        bufsize <<= 1i32;
    }
    unsafe { PyEval_RestoreThread(_save) };
    if p.is_null() {
        unsafe { PyMem_RawFree(buf as *mut ()) };
        if nomem == 1 {
            return unsafe { PyErr_NoMemory() };
        }
        let uid_obj: *mut _object = unsafe { _PyLong_FromUid(uid) };
        if uid_obj.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        unsafe {
            PyErr_Format(
                PyExc_KeyError,
                c"getpwuid(): uid not found: %S".as_ptr(),
                uid_obj,
            )
        };
        Py_DECREF!(uid_obj);
        return core::ptr::null_mut::<_object>();
    }
    retval = mkpwent(module, p);
    unsafe { PyMem_RawFree(buf as *mut ()) };
    retval
}

// [clinic input]
// pwd.getpwnam
//
//     name: unicode
//     /
//
// Return the password database entry for the given user name.
//
// See `help(pwd)` for more on password database entries.
// [clinic start generated code]
// [clinic end generated code: output=c4ee1d4d429b86c4 input=ae64d507a1c6d3e8]
/// [clinic end generated code: output=359ce1ddeb7a824f input=a6aeb5e3447fb9e0]
fn pwd_getpwnam_impl(module: *mut _object, name: *mut _object) -> *mut _object {
    let mut _save: *mut _ts = unsafe { core::mem::zeroed() };
    // Note: 'pwd' will be used via pointer 'p' on getpwnam_r success.
    let mut pwd: core::mem::MaybeUninit<slate_support::passwd> = core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut slate_support::passwd =
        core::ptr::addr_of_mut!(pwd) as *mut slate_support::passwd;
    let mut bufsize: i64 = unsafe { core::mem::zeroed() };
    let mut status: i32 = unsafe { core::mem::zeroed() };
    let mut retval: *mut _object = unsafe { core::mem::zeroed() };
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut p: *mut slate_support::passwd = unsafe { core::mem::zeroed() };
    let mut nomem: i32 = unsafe { core::mem::zeroed() };
    let mut name_chars: *mut i8 = unsafe { core::mem::zeroed() };
    let mut buf2: *mut i8 = unsafe { core::mem::zeroed() };
    let mut buf: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        buf = core::ptr::null_mut::<i8>();
        buf2 = core::ptr::null_mut::<i8>();
        nomem = 0;
        retval = core::ptr::null_mut::<_object>();
        let __v6: *mut _object = PyUnicode_EncodeFSDefault(name);
        bytes = __v6;
        if __v6.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            // check for embedded null bytes
            if PyBytes_AsStringAndSize(
                bytes,
                core::ptr::addr_of_mut!(name_chars),
                core::ptr::null_mut::<i64>(),
            ) != 1i32.wrapping_neg()
            {
                _save = PyEval_SaveThread();
                bufsize = sysconf((slate_support::_SC_GETPW_R_SIZE_MAX as i32) as i32) as i64;
                if bufsize == (1i32.wrapping_neg() as i64) {
                    bufsize = 1024;
                }
                '__join_5: {
                    '__join_13: {
                        loop {
                            if true {
                                buf2 = PyMem_RawRealloc(buf as *mut (), bufsize as u64) as *mut i8;
                                if buf2.is_null() {
                                    break '__join_13;
                                } else {
                                    buf = buf2;
                                    status = getpwnam_r(
                                        (name_chars as *const i8) as *const core::ffi::c_char,
                                        core::ptr::addr_of_mut!(*__slate_slot_3)
                                            as *mut slate_support::passwd,
                                        buf as *mut core::ffi::c_char,
                                        (bufsize as u64) as usize,
                                        core::ptr::addr_of_mut!(p)
                                            as *mut *mut slate_support::passwd,
                                    ) as i32;
                                    if status != 0 {
                                        p = core::ptr::null_mut::<slate_support::passwd>();
                                    }
                                    if !p.is_null() || status != (slate_support::ERANGE as i32) {
                                        break '__join_5;
                                    } else {
                                        if bufsize > 4611686018427387903i64 {
                                            break;
                                        } else {
                                            bufsize <<= 1i32;
                                        }
                                    }
                                }
                            } else {
                                break '__join_5;
                            }
                        }
                        nomem = 1;
                        break '__join_5;
                    }
                    p = core::ptr::null_mut::<slate_support::passwd>();
                    nomem = 1;
                }
                PyEval_RestoreThread(_save);
                if p.is_null() {
                    if nomem == 1 {
                        PyErr_NoMemory();
                    } else {
                        PyErr_Format(
                            PyExc_KeyError,
                            c"getpwnam(): name not found: %R".as_ptr(),
                            name,
                        );
                    }
                } else {
                    retval = mkpwent(module, p);
                }
            }
            PyMem_RawFree(buf as *mut ());
            Py_DECREF!(bytes);
            return retval;
        }
    }
}

// [clinic input]
// pwd.getpwall
//
// Return a list of all available password database entries, in arbitrary order.
//
// See help(pwd) for more on password database entries.
// [clinic start generated code]
/// [clinic end generated code: output=4853d2f5a0afac8a input=d7ecebfd90219b85]
fn pwd_getpwall_impl(module: *mut _object) -> *mut _object {
    let __v0: *mut _object = unsafe { PyList_New(0i64) };
    let d: *mut _object = __v0;
    if __v0.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { setpwent() };
    loop {
        let __v2: *mut slate_support::passwd = unsafe { getpwent() as *mut slate_support::passwd };
        let p: *mut slate_support::passwd = __v2;
        if __v2.is_null() {
            break;
        }
        let v: *mut _object = mkpwent(module, p);
        let __v1: bool = v.is_null() || (unsafe { PyList_Append(d, v) }) != 0;
        if __v1 {
            Py_XDECREF(v);
            Py_DECREF!(d);
            unsafe { endpwent() };
            return core::ptr::null_mut::<_object>();
        }
        Py_DECREF!(v);
    }
    unsafe { endpwent() };
    d
}

/// sentinel
static mut pwd_methods: __SlateAlign16<[PyMethodDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"getpwuid\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(pwd_getpwuid);
        __t0.ml_flags = 8i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(pwd_getpwuid__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"getpwnam\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(pwd_getpwnam);
        __t1.ml_flags = 8i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(pwd_getpwnam__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"getpwall\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(pwd_getpwall);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(pwd_getpwall__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = core::ptr::null::<i8>();
        __t3.ml_meth = None;
        __t3
    },
]);

#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwdmodule_exec")]
extern "C-unwind" fn pwdmodule_exec(module: *mut _object) -> i32 {
    let state: *mut __SlateRecord_pwdmodule_c_60 = get_pwd_state(module);
    unsafe {
        (*state).StructPwdType =
            PyStructSequence_NewType(core::ptr::addr_of_mut!(struct_pwd_type_desc));
    }
    if unsafe { (*state).StructPwdType }.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddType(module, (*state).StructPwdType) }) < 0 {
        return 1i32.wrapping_neg();
    }
    0
}

static mut pwdmodule_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(pwdmodule_exec),
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

#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwdmodule_traverse")]
extern "C-unwind" fn pwdmodule_traverse(
    m: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    Py_VISIT__2!(unsafe { (*get_pwd_state(m)).StructPwdType }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwdmodule_clear")]
extern "C-unwind" fn pwdmodule_clear(m: *mut _object) -> i32 {
    Py_CLEAR__3!((*get_pwd_state(m)).StructPwdType);
    0
}

#[unsafe(link_section = ".text.slate_distinct.pwdmodule.pwdmodule_free")]
extern "C-unwind" fn pwdmodule_free(m: *mut ()) {
    pwdmodule_clear(m as *mut _object);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.pwdmodule.PyInit_pwd")]
extern "C-unwind" fn PyInit_pwd() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(pwdmodule)) }
}
