//! Errno module
//! Define macros for inline documentation.
//! PyObject_HEAD defines the initial segment of every PyObject.
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
// Signed variants of the above
macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
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

macro_rules! add_errcode {
    ($name:expr, $code:expr, $module_dict:expr, $error_dict:expr) => {{
        loop {
            if _add_errcode($module_dict, $error_dict, ($name) as *const i8, $code) < 0 {
                Py_DECREF!($error_dict);
                return 1i32.wrapping_neg();
            }
            if true {
                break;
            }
        }
    }};
}

unsafe extern "C" {
    fn _Py_DecRef(arg0: *mut _object);
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_GetDict(arg0: *mut _object) -> *mut _object;
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _object {
    __slate_anon_0: __SlateRecord_object_h_122,
    ob_type: *mut _typeobject,
}

// Cast argument to PyVarObject* type.
fn Py_DECREF(op: *mut _object) {
    unsafe { _Py_DecRef(op) };
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
struct PyModuleDef_Base {
    ob_base: _object,
    m_init: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
    m_index: i64,
    m_copy: *mut _object,
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

// m_init
// m_index
// m_copy
#[repr(C)]
#[derive(Clone, Copy)]
struct PyModuleDef_Slot {
    slot: i32,
    value: *mut (),
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

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

// Need limited C API version 3.13 for Py_mod_gil
// Py_GIL_DISABLED
// EPIPE
// Windows socket errors (WSA*)
// Pull in the system error definitions
static mut errno_methods: __SlateAlign16<[PyMethodDef; 1]> = __SlateAlign16([{
    let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
    __t0.ml_name = core::ptr::null::<i8>();
    __t0.ml_meth = None;
    __t0
}]);

// Helper function doing the dictionary inserting
fn _add_errcode(
    module_dict: *mut _object,
    error_dict: *mut _object,
    name_str: *const i8,
    code_int: i32,
) -> i32 {
    let mut ret: i32 = unsafe { core::mem::zeroed() };
    let mut code: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        name = PyUnicode_FromString(name_str);
        if name.is_null() {
            return 1i32.wrapping_neg();
        } else {
            code = PyLong_FromLong(code_int as i64);
            if code.is_null() {
                Py_DECREF!(name);
                return 1i32.wrapping_neg();
            } else {
                ret = 1i32.wrapping_neg();
                // insert in modules dict
                if PyDict_SetItem(module_dict, name, code) >= 0 {
                    // insert in errorcode dict
                    if PyDict_SetItem(error_dict, code, name) >= 0 {
                        ret = 0;
                    }
                }
                Py_DECREF!(name);
                Py_DECREF!(code);
                return ret;
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.errnomodule.errno_exec")]
extern "C-unwind" fn errno_exec(module: *mut _object) -> i32 {
    let module_dict: *mut _object = unsafe { PyModule_GetDict(module) }; // Borrowed ref.
    if module_dict.is_null() {
        return 1i32.wrapping_neg();
    }
    let error_dict: *mut _object = unsafe { PyDict_New() };
    if error_dict.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyDict_SetItemString(module_dict, c"errorcode".as_ptr(), error_dict) }) < 0 {
        Py_DECREF!(error_dict);
        return 1i32.wrapping_neg();
    }
    // Macro so I don't have to edit each and every line below...
    // The names and comments are borrowed from linux/include/errno.h,
    // which should be pretty all-inclusive.  However, the Solaris specific
    // names and comments are borrowed from sys/errno.h in Solaris.
    // MacOSX specific names and comments are borrowed from sys/errno.h in
    // MacOSX.
    add_errcode!(
        c"ENODEV".as_ptr() as *mut i8,
        slate_support::ENODEV as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOCSI".as_ptr() as *mut i8,
        slate_support::ENOCSI as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EHOSTUNREACH".as_ptr() as *mut i8,
        slate_support::EHOSTUNREACH as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOMSG".as_ptr() as *mut i8,
        slate_support::ENOMSG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EUCLEAN".as_ptr() as *mut i8,
        slate_support::EUCLEAN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EL2NSYNC".as_ptr() as *mut i8,
        slate_support::EL2NSYNC as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EL2HLT".as_ptr() as *mut i8,
        slate_support::EL2HLT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENODATA".as_ptr() as *mut i8,
        slate_support::ENODATA as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTBLK".as_ptr() as *mut i8,
        slate_support::ENOTBLK as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOSYS".as_ptr() as *mut i8,
        slate_support::ENOSYS as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EPIPE".as_ptr() as *mut i8,
        slate_support::EPIPE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EINVAL".as_ptr() as *mut i8,
        slate_support::EINVAL as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EOVERFLOW".as_ptr() as *mut i8,
        slate_support::EOVERFLOW as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EADV".as_ptr() as *mut i8,
        slate_support::EADV as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EINTR".as_ptr() as *mut i8,
        slate_support::EINTR as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EUSERS".as_ptr() as *mut i8,
        slate_support::EUSERS as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTEMPTY".as_ptr() as *mut i8,
        slate_support::ENOTEMPTY as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOBUFS".as_ptr() as *mut i8,
        slate_support::ENOBUFS as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EPROTO".as_ptr() as *mut i8,
        slate_support::EPROTO as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EREMOTE".as_ptr() as *mut i8,
        slate_support::EREMOTE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENAVAIL".as_ptr() as *mut i8,
        slate_support::ENAVAIL as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECHILD".as_ptr() as *mut i8,
        slate_support::ECHILD as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELOOP".as_ptr() as *mut i8,
        slate_support::ELOOP as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EXDEV".as_ptr() as *mut i8,
        slate_support::EXDEV as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"E2BIG".as_ptr() as *mut i8,
        slate_support::E2BIG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESRCH".as_ptr() as *mut i8,
        slate_support::ESRCH as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EMSGSIZE".as_ptr() as *mut i8,
        slate_support::EMSGSIZE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EAFNOSUPPORT".as_ptr() as *mut i8,
        slate_support::EAFNOSUPPORT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADR".as_ptr() as *mut i8,
        slate_support::EBADR as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EHOSTDOWN".as_ptr() as *mut i8,
        slate_support::EHOSTDOWN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EPFNOSUPPORT".as_ptr() as *mut i8,
        slate_support::EPFNOSUPPORT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOPROTOOPT".as_ptr() as *mut i8,
        slate_support::ENOPROTOOPT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBUSY".as_ptr() as *mut i8,
        slate_support::EBUSY as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EWOULDBLOCK".as_ptr() as *mut i8,
        11i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADFD".as_ptr() as *mut i8,
        slate_support::EBADFD as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EDOTDOT".as_ptr() as *mut i8,
        slate_support::EDOTDOT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EISCONN".as_ptr() as *mut i8,
        slate_support::EISCONN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOANO".as_ptr() as *mut i8,
        slate_support::ENOANO as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESHUTDOWN".as_ptr() as *mut i8,
        slate_support::ESHUTDOWN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECHRNG".as_ptr() as *mut i8,
        slate_support::ECHRNG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELIBBAD".as_ptr() as *mut i8,
        slate_support::ELIBBAD as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENONET".as_ptr() as *mut i8,
        slate_support::ENONET as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADE".as_ptr() as *mut i8,
        slate_support::EBADE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADF".as_ptr() as *mut i8,
        slate_support::EBADF as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EMULTIHOP".as_ptr() as *mut i8,
        slate_support::EMULTIHOP as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EIO".as_ptr() as *mut i8,
        slate_support::EIO as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EUNATCH".as_ptr() as *mut i8,
        slate_support::EUNATCH as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EPROTOTYPE".as_ptr() as *mut i8,
        slate_support::EPROTOTYPE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOSPC".as_ptr() as *mut i8,
        slate_support::ENOSPC as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOEXEC".as_ptr() as *mut i8,
        slate_support::ENOEXEC as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EALREADY".as_ptr() as *mut i8,
        slate_support::EALREADY as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENETDOWN".as_ptr() as *mut i8,
        slate_support::ENETDOWN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTNAM".as_ptr() as *mut i8,
        slate_support::ENOTNAM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EACCES".as_ptr() as *mut i8,
        slate_support::EACCES as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELNRNG".as_ptr() as *mut i8,
        slate_support::ELNRNG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EILSEQ".as_ptr() as *mut i8,
        slate_support::EILSEQ as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTDIR".as_ptr() as *mut i8,
        slate_support::ENOTDIR as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTUNIQ".as_ptr() as *mut i8,
        slate_support::ENOTUNIQ as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EPERM".as_ptr() as *mut i8,
        slate_support::EPERM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EDOM".as_ptr() as *mut i8,
        slate_support::EDOM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EXFULL".as_ptr() as *mut i8,
        slate_support::EXFULL as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECONNREFUSED".as_ptr() as *mut i8,
        slate_support::ECONNREFUSED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EISDIR".as_ptr() as *mut i8,
        slate_support::EISDIR as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EPROTONOSUPPORT".as_ptr() as *mut i8,
        slate_support::EPROTONOSUPPORT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EROFS".as_ptr() as *mut i8,
        slate_support::EROFS as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EADDRNOTAVAIL".as_ptr() as *mut i8,
        slate_support::EADDRNOTAVAIL as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EIDRM".as_ptr() as *mut i8,
        slate_support::EIDRM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECOMM".as_ptr() as *mut i8,
        slate_support::ECOMM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESRMNT".as_ptr() as *mut i8,
        slate_support::ESRMNT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EREMOTEIO".as_ptr() as *mut i8,
        slate_support::EREMOTEIO as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EL3RST".as_ptr() as *mut i8,
        slate_support::EL3RST as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADMSG".as_ptr() as *mut i8,
        slate_support::EBADMSG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENFILE".as_ptr() as *mut i8,
        slate_support::ENFILE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELIBMAX".as_ptr() as *mut i8,
        slate_support::ELIBMAX as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESPIPE".as_ptr() as *mut i8,
        slate_support::ESPIPE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOLINK".as_ptr() as *mut i8,
        slate_support::ENOLINK as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENETRESET".as_ptr() as *mut i8,
        slate_support::ENETRESET as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ETIMEDOUT".as_ptr() as *mut i8,
        slate_support::ETIMEDOUT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOENT".as_ptr() as *mut i8,
        slate_support::ENOENT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EEXIST".as_ptr() as *mut i8,
        slate_support::EEXIST as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EDQUOT".as_ptr() as *mut i8,
        slate_support::EDQUOT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOSTR".as_ptr() as *mut i8,
        slate_support::ENOSTR as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADSLT".as_ptr() as *mut i8,
        slate_support::EBADSLT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBADRQC".as_ptr() as *mut i8,
        slate_support::EBADRQC as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELIBACC".as_ptr() as *mut i8,
        slate_support::ELIBACC as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EFAULT".as_ptr() as *mut i8,
        slate_support::EFAULT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EFBIG".as_ptr() as *mut i8,
        slate_support::EFBIG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EDEADLK".as_ptr() as *mut i8,
        slate_support::EDEADLK as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTCONN".as_ptr() as *mut i8,
        slate_support::ENOTCONN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EDESTADDRREQ".as_ptr() as *mut i8,
        slate_support::EDESTADDRREQ as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELIBSCN".as_ptr() as *mut i8,
        slate_support::ELIBSCN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOLCK".as_ptr() as *mut i8,
        slate_support::ENOLCK as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EISNAM".as_ptr() as *mut i8,
        slate_support::EISNAM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECONNABORTED".as_ptr() as *mut i8,
        slate_support::ECONNABORTED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENETUNREACH".as_ptr() as *mut i8,
        slate_support::ENETUNREACH as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESTALE".as_ptr() as *mut i8,
        slate_support::ESTALE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOSR".as_ptr() as *mut i8,
        slate_support::ENOSR as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOMEM".as_ptr() as *mut i8,
        slate_support::ENOMEM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTSOCK".as_ptr() as *mut i8,
        slate_support::ENOTSOCK as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESTRPIPE".as_ptr() as *mut i8,
        slate_support::ESTRPIPE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EMLINK".as_ptr() as *mut i8,
        slate_support::EMLINK as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ERANGE".as_ptr() as *mut i8,
        slate_support::ERANGE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ELIBEXEC".as_ptr() as *mut i8,
        slate_support::ELIBEXEC as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EL3HLT".as_ptr() as *mut i8,
        slate_support::EL3HLT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECONNRESET".as_ptr() as *mut i8,
        slate_support::ECONNRESET as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EADDRINUSE".as_ptr() as *mut i8,
        slate_support::EADDRINUSE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EOPNOTSUPP".as_ptr() as *mut i8,
        slate_support::EOPNOTSUPP as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EREMCHG".as_ptr() as *mut i8,
        slate_support::EREMCHG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EAGAIN".as_ptr() as *mut i8,
        slate_support::EAGAIN as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENAMETOOLONG".as_ptr() as *mut i8,
        slate_support::ENAMETOOLONG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTTY".as_ptr() as *mut i8,
        slate_support::ENOTTY as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ERESTART".as_ptr() as *mut i8,
        slate_support::ERESTART as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ESOCKTNOSUPPORT".as_ptr() as *mut i8,
        slate_support::ESOCKTNOSUPPORT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ETIME".as_ptr() as *mut i8,
        slate_support::ETIME as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EBFONT".as_ptr() as *mut i8,
        slate_support::EBFONT as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EDEADLOCK".as_ptr() as *mut i8,
        35i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ETOOMANYREFS".as_ptr() as *mut i8,
        slate_support::ETOOMANYREFS as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EMFILE".as_ptr() as *mut i8,
        slate_support::EMFILE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ETXTBSY".as_ptr() as *mut i8,
        slate_support::ETXTBSY as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EINPROGRESS".as_ptr() as *mut i8,
        slate_support::EINPROGRESS as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENXIO".as_ptr() as *mut i8,
        slate_support::ENXIO as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOPKG".as_ptr() as *mut i8,
        slate_support::ENOPKG as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOMEDIUM".as_ptr() as *mut i8,
        slate_support::ENOMEDIUM as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EMEDIUMTYPE".as_ptr() as *mut i8,
        slate_support::EMEDIUMTYPE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ECANCELED".as_ptr() as *mut i8,
        slate_support::ECANCELED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOKEY".as_ptr() as *mut i8,
        slate_support::ENOKEY as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EHWPOISON".as_ptr() as *mut i8,
        slate_support::EHWPOISON as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EKEYEXPIRED".as_ptr() as *mut i8,
        slate_support::EKEYEXPIRED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EKEYREVOKED".as_ptr() as *mut i8,
        slate_support::EKEYREVOKED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EKEYREJECTED".as_ptr() as *mut i8,
        slate_support::EKEYREJECTED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EOWNERDEAD".as_ptr() as *mut i8,
        slate_support::EOWNERDEAD as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTRECOVERABLE".as_ptr() as *mut i8,
        slate_support::ENOTRECOVERABLE as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ERFKILL".as_ptr() as *mut i8,
        slate_support::ERFKILL as i32,
        module_dict,
        error_dict
    );
    // Solaris-specific errnos
    add_errcode!(
        c"ECANCELED".as_ptr() as *mut i8,
        slate_support::ECANCELED as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTSUP".as_ptr() as *mut i8,
        95i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"EOWNERDEAD".as_ptr() as *mut i8,
        slate_support::EOWNERDEAD as i32,
        module_dict,
        error_dict
    );
    add_errcode!(
        c"ENOTRECOVERABLE".as_ptr() as *mut i8,
        slate_support::ENOTRECOVERABLE as i32,
        module_dict,
        error_dict
    );
    // MacOSX specific errnos
    Py_DECREF!(error_dict);
    0
}

static mut errno_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(errno_exec),
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

static errno__doc__: __SlateAlign16<[i8; 508]> = __SlateAlign16([
    84i8, 104i8, 105i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 109i8, 97i8,
    107i8, 101i8, 115i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    115i8, 116i8, 97i8, 110i8, 100i8, 97i8, 114i8, 100i8, 32i8, 101i8, 114i8, 114i8, 110i8, 111i8,
    32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8, 32i8, 115i8, 121i8, 109i8, 98i8, 111i8, 108i8,
    115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8,
    111i8, 102i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 115i8, 121i8, 109i8, 98i8, 111i8, 108i8,
    32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 114i8, 114i8, 101i8, 115i8,
    112i8, 111i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8,
    101i8, 114i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 10i8, 101i8, 46i8, 103i8, 46i8,
    44i8, 32i8, 111i8, 110i8, 32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 115i8, 121i8, 115i8, 116i8,
    101i8, 109i8, 115i8, 44i8, 32i8, 101i8, 114i8, 114i8, 110i8, 111i8, 46i8, 69i8, 78i8, 79i8,
    69i8, 78i8, 84i8, 32i8, 101i8, 113i8, 117i8, 97i8, 108i8, 115i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8, 50i8, 46i8, 10i8, 10i8, 84i8,
    104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8,
    101i8, 114i8, 114i8, 110i8, 111i8, 46i8, 101i8, 114i8, 114i8, 111i8, 114i8, 99i8, 111i8, 100i8,
    101i8, 32i8, 109i8, 97i8, 112i8, 115i8, 32i8, 110i8, 117i8, 109i8, 101i8, 114i8, 105i8, 99i8,
    32i8, 99i8, 111i8, 100i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 115i8, 121i8, 109i8, 98i8,
    111i8, 108i8, 32i8, 110i8, 97i8, 109i8, 101i8, 115i8, 44i8, 10i8, 101i8, 46i8, 103i8, 46i8,
    44i8, 32i8, 101i8, 114i8, 114i8, 110i8, 111i8, 46i8, 101i8, 114i8, 114i8, 111i8, 114i8, 99i8,
    111i8, 100i8, 101i8, 91i8, 50i8, 93i8, 32i8, 99i8, 111i8, 117i8, 108i8, 100i8, 32i8, 98i8,
    101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 39i8,
    69i8, 78i8, 79i8, 69i8, 78i8, 84i8, 39i8, 46i8, 10i8, 10i8, 83i8, 121i8, 109i8, 98i8, 111i8,
    108i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 97i8, 114i8, 101i8, 32i8, 110i8, 111i8,
    116i8, 32i8, 114i8, 101i8, 108i8, 101i8, 118i8, 97i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8,
    103i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8, 32i8, 97i8, 114i8, 101i8, 32i8, 110i8,
    111i8, 116i8, 32i8, 100i8, 101i8, 102i8, 105i8, 110i8, 101i8, 100i8, 46i8, 10i8, 10i8, 84i8,
    111i8, 32i8, 109i8, 97i8, 112i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 32i8, 99i8, 111i8,
    100i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 32i8, 109i8,
    101i8, 115i8, 115i8, 97i8, 103i8, 101i8, 115i8, 44i8, 32i8, 117i8, 115i8, 101i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 115i8,
    46i8, 115i8, 116i8, 114i8, 101i8, 114i8, 114i8, 111i8, 114i8, 40i8, 41i8, 44i8, 10i8, 101i8,
    46i8, 103i8, 46i8, 32i8, 111i8, 115i8, 46i8, 115i8, 116i8, 114i8, 101i8, 114i8, 114i8, 111i8,
    114i8, 40i8, 50i8, 41i8, 32i8, 99i8, 111i8, 117i8, 108i8, 100i8, 32i8, 114i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 39i8, 78i8, 111i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 102i8,
    105i8, 108i8, 101i8, 32i8, 111i8, 114i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8,
    114i8, 121i8, 39i8, 46i8, 0i8,
]);

static mut errnomodule: PyModuleDef = {
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
    __t0.m_name = (b"errno\0".as_ptr() as *mut i8) as *const i8;
    __t0.m_doc = unsafe { core::ptr::addr_of!(errno__doc__.0) as *const i8 };
    __t0.m_size = 0i64;
    __t0.m_methods = unsafe { core::ptr::addr_of_mut!(errno_methods.0) as *mut PyMethodDef };
    __t0.m_slots = unsafe { core::ptr::addr_of_mut!(errno_slots.0) as *mut PyModuleDef_Slot };
    __t0
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.errnomodule.PyInit_errno")]
extern "C-unwind" fn PyInit_errno() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(errnomodule)) }
}
