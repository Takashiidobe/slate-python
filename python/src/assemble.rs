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
//! PyUnicode_KIND(): Return one of the PyUnicode_*_KIND values defined above.
//!
//!
//!
//! gh-89653: Converting this macro to a static inline function would introduce
//!
//! new compiler warnings on "kind < PyUnicode_KIND(str)" (compare signed and
//!
//! unsigned numbers) where kind type is an int or on
//!
//! "unsigned int kind = PyUnicode_KIND(str)" (cast signed to unsigned).
//! PyException_HEAD defines the initial segment of every exception class.
//! Cast argument to PyListObject* type.
//! A code object, the __code__ attribute
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! write_location_entry_start()
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
//! Note that these all fit within a byte, as do combinations.
//! Instruction opcodes for compiled code
// Signed variants of the above
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// NULL or a tuple
// NULL or a dict
// NULL or a tuple of cell objects
// Initial exit temperature.
// Must be larger than ADAPTIVE_COOLDOWN_VALUE,
// otherwise when a side exit warms up we may construct
// a new trace before the Tier 1 code has properly re-specialized.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! IS_PSEUDO_INSTR {
    ($OP:expr) => {
        ($OP) == 261
            || ($OP) == 266
            || ($OP) == 256
            || ($OP) == 257
            || ($OP) == 260
            || ($OP) == 258
            || ($OP) == 259
            || ($OP) == 264
            || ($OP) == 263
            || ($OP) == 265
            || ($OP) == 262
            || false
    };
}

macro_rules! IS_SCOPE_EXIT_OPCODE {
    ($opcode:expr) => {
        ($opcode) == 35 || ($opcode) == 104 || ($opcode) == 105
    };
}

/// opcodes that must be last in the basicblock
macro_rules! IS_TERMINATOR_OPCODE {
    ($opcode:expr) => {
        OPCODE_HAS_JUMP!($opcode) != 0 || IS_SCOPE_EXIT_OPCODE!($opcode)
    };
}

macro_rules! OPCODE_HAS_JUMP {
    ($OP:expr) => {
        ((unsafe { _PyOpcode_opcode_metadata[($OP) as usize].flags }) as i32) & 8
    };
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
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

macro_rules! PyTuple_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
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

macro_rules! RETURN_IF_ERROR__3 {
    ($X:expr) => {{
        if ($X) < 0 {
            return 1i32.wrapping_neg();
        }
    }};
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
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
    static mut _PyOpcode_opcode_metadata: [opcode_metadata; 267];
    static mut _PyOpcode_Caches: [u8; 256];
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _PyBytes_Resize(arg0: *mut *mut _object, arg1: i64) -> i32;
    fn PyErr_Occurred() -> *mut _object;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyList_AsTuple(arg0: *mut _object) -> *mut _object;
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_Contains(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn _PyCode_Validate(arg0: *mut _PyCodeConstructor) -> i32;
    fn _PyCode_New(arg0: *mut _PyCodeConstructor) -> *mut PyCodeObject;
    fn _PyInstructionSequence_ApplyLabelMap(arg0: *mut instruction_sequence) -> i32;
    fn _PyCompile_ConstCacheMergeOne(arg0: *mut _object, arg1: *mut *mut _object) -> i32;
    fn __builtin_unreachable();
    fn _Py_set_localsplus_info(
        arg0: i32,
        arg1: *mut _object,
        arg2: u8,
        arg3: *mut _object,
        arg4: *mut _object,
    );
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _dictkeysobject {}

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
struct _dictvalues {}

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
struct _PyExecutorObject {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCoMonitoringData {}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_tupleobject_h_13 {
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct __SlateRecord_assemble_c_494 {
    count: i32,
    kind: u8,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

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

use slate_support::memset;

// pos-only, pos-or-kw, varargs
// kw-only, pos-or-kw, varkwargs
// varargs, varkwargs
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCodeConstructor {
    filename: *mut _object,
    name: *mut _object,
    qualname: *mut _object,
    flags: i32,
    code: *mut _object,
    firstlineno: i32,
    linetable: *mut _object,
    consts: *mut _object,
    names: *mut _object,
    localsplusnames: *mut _object,
    localspluskinds: *mut _object,
    argcount: i32,
    posonlyargcount: i32,
    kwonlyargcount: i32,
    stacksize: i32,
    exceptiontable: *mut _object,
}

fn write_varint(mut ptr: *mut u8, mut val: u32) -> i32 {
    let mut written: i32 = 1;
    while val >= 64 {
        let __v0: *mut u8 = ptr;
        ptr = __v0.wrapping_offset(1 as isize);
        unsafe {
            *__v0 = (64 | val & 63) as u8;
        }
        val >>= 6i32;
        written = written.wrapping_add(1);
    }
    unsafe {
        *ptr = val as u8;
    }
    written
}

fn write_signed_varint(ptr: *mut u8, val: i32) -> i32 {
    // (unsigned int)(-val) has an undefined behavior for INT_MIN
    let uval: u32 = if val < 0 {
        0u32.wrapping_sub(val as u32) << 1i32 | 1
    } else {
        (val as u32) << 1i32
    };
    write_varint(ptr, uval)
}

fn write_location_entry_start(ptr: *mut u8, code: i32, length: i32) -> i32 {
    unsafe {
        *ptr =
            (128 | (((code << 3i32) as u8) as i32) | ((length.wrapping_sub(1) as u8) as i32)) as u8;
    }
    1
}

// A value of 1 means that we attempt to specialize the *second* time each
//
// instruction is executed. Executing twice is a much better indicator of
//
// "hotness" than executing once, but additional warmup delays only prevent
//
// specialization. Most types stabilize by the second execution, too:
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
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_symtable_h_51 {
    lineno: i32,
    end_lineno: i32,
    col_offset: i32,
    end_col_offset: i32,
}

static NO_LOCATION: __SlateRecord_pycore_symtable_h_51 = __SlateRecord_pycore_symtable_h_51 {
    lineno: 1i32.wrapping_neg(),
    end_lineno: 1i32.wrapping_neg(),
    col_offset: 1i32.wrapping_neg(),
    end_col_offset: 1i32.wrapping_neg(),
};

static NEXT_LOCATION: __SlateRecord_pycore_symtable_h_51 = __SlateRecord_pycore_symtable_h_51 {
    lineno: 2i32.wrapping_neg(),
    end_lineno: 2i32.wrapping_neg(),
    col_offset: 2i32.wrapping_neg(),
    end_col_offset: 2i32.wrapping_neg(),
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_instruction_sequence_h_19 {
    h_label: i32,
    h_startdepth: i32,
    h_preserve_lasti: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_instruction_sequence_h_30 {
    i_opcode: i32,
    i_oparg: i32,
    i_loc: __SlateRecord_pycore_symtable_h_51,
    i_except_handler_info: __SlateRecord_pycore_instruction_sequence_h_19,
    i_target: i32,
    i_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct instruction_sequence {
    ob_base: _object,
    s_instrs: *mut __SlateRecord_pycore_instruction_sequence_h_30,
    s_allocated: i32,
    s_used: i32,
    s_next_free_label: i32,
    s_labelmap: *mut i32,
    s_labelmap_size: i32,
    s_nested: *mut _object,
    s_annotations_code: *mut instruction_sequence,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_compile_h_78 {
    u_name: *mut _object,
    u_qualname: *mut _object,
    u_consts: *mut _object,
    u_names: *mut _object,
    u_varnames: *mut _object,
    u_cellvars: *mut _object,
    u_freevars: *mut _object,
    u_fasthidden: *mut _object,
    u_argcount: i64,
    u_posonlyargcount: i64,
    u_kwonlyargcount: i64,
    u_firstlineno: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct opcode_metadata {
    valid_entry: u8,
    instr_format: u8,
    flags: u16,
}

// IS_BACKWARDS_JUMP_OPCODE
// is_pseudo_target, _PyOpcode_Caches
// _Py_SourceLocation
fn same_location(
    a: __SlateRecord_pycore_symtable_h_51,
    b: __SlateRecord_pycore_symtable_h_51,
) -> bool {
    a.lineno == b.lineno
        && a.end_lineno == b.end_lineno
        && a.col_offset == b.col_offset
        && a.end_col_offset == b.end_col_offset
}

fn instr_size(instr: *mut __SlateRecord_pycore_instruction_sequence_h_30) -> i32 {
    let opcode: i32 = unsafe { (*instr).i_opcode };
    let oparg: i32 = unsafe { (*instr).i_oparg };
    let extended_args: i32 = ((16777215 < oparg) as i32)
        .wrapping_add((65535 < oparg) as i32)
        .wrapping_add((255 < oparg) as i32);
    let caches: i32 = (unsafe { _PyOpcode_Caches[opcode as usize] }) as i32;
    extended_args.wrapping_add(1).wrapping_add(caches)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct assembler {
    /// bytes containing bytecode
    a_bytecode: *mut _object,
    /// offset into bytecode
    a_offset: i32,
    /// bytes containing exception table
    a_except_table: *mut _object,
    /// offset into exception table
    a_except_table_off: i32,
    /// Location Info
    /// lineno of last emitted instruction
    a_lineno: i32,
    /// bytes containing location info
    a_linetable: *mut _object,
    /// offset of last written location info frame
    a_location_off: i32,
}

fn assemble_init(a: *mut assembler, firstlineno: i32) -> i32 {
    unsafe {
        core::ptr::write_bytes(
            ((a as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<assembler>() as u64) as usize) as usize,
        );
        (*a).a_lineno = firstlineno;
        (*a).a_linetable = core::ptr::null_mut::<_object>();
        (*a).a_location_off = 0;
        (*a).a_except_table = core::ptr::null_mut::<_object>();
        (*a).a_bytecode = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 128i64);
        if !unsafe { (*a).a_bytecode }.is_null() {
            (*a).a_linetable = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 32i64);
            if !unsafe { (*a).a_linetable }.is_null() {
                (*a).a_except_table = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 16i64);
                if !unsafe { (*a).a_except_table }.is_null() {
                    return 0;
                }
            }
        }
        Py_CLEAR!((*a).a_bytecode);
        Py_CLEAR!((*a).a_linetable);
        Py_CLEAR!((*a).a_except_table);
        return 1i32.wrapping_neg();
    }
}

fn assemble_free(a: *mut assembler) {
    Py_XDECREF!(unsafe { (*a).a_bytecode });
    Py_XDECREF!(unsafe { (*a).a_linetable });
    Py_XDECREF!(unsafe { (*a).a_except_table });
}

fn write_except_byte(a: *mut assembler, byte: i32) {
    let p: *mut u8 = PyBytes_AS_STRING!(unsafe { (*a).a_except_table }) as *mut u8;
    let __v1: i32 = unsafe { (*a).a_except_table_off };
    unsafe {
        (*a).a_except_table_off = __v1.wrapping_add(1);
    }
    unsafe {
        *p.wrapping_offset(__v1 as isize) = byte as u8;
    }
}

fn assemble_emit_exception_table_item(a: *mut assembler, value: i32, mut msb: i32) {
    if value >= 16777216i32 {
        write_except_byte(a, value >> 24i32 | 64 | msb);
        msb = 0;
    }
    if value >= 262144i32 {
        write_except_byte(a, value >> 18i32 & 63 | 64 | msb);
        msb = 0;
    }
    if value >= 4096i32 {
        write_except_byte(a, value >> 12i32 & 63 | 64 | msb);
        msb = 0;
    }
    if value >= 64i32 {
        write_except_byte(a, value >> 6i32 & 63 | 64 | msb);
        msb = 0;
    }
    write_except_byte(a, value & 63 | msb);
}

// See InternalDocs/exception_handling.md for details of layout
fn assemble_emit_exception_table_entry(
    a: *mut assembler,
    start: i32,
    end: i32,
    target: i32,
    handler: *mut __SlateRecord_pycore_instruction_sequence_h_19,
) -> i32 {
    let len: i64 = PyBytes_GET_SIZE!(unsafe { (*a).a_except_table });
    if (unsafe { (*a).a_except_table_off }.wrapping_add(20) as i64) >= len {
        RETURN_IF_ERROR__3!(unsafe {
            _PyBytes_Resize(
                core::ptr::addr_of_mut!((*a).a_except_table),
                len.wrapping_mul(2),
            )
        });
    }
    let size: i32 = end.wrapping_sub(start);
    let mut depth: i32 = unsafe { (*handler).h_startdepth }.wrapping_sub(1);
    if (unsafe { (*handler).h_preserve_lasti }) > 0 {
        depth = depth.wrapping_sub(1);
    }
    let depth_lasti: i32 = depth << 1i32 | unsafe { (*handler).h_preserve_lasti };
    assemble_emit_exception_table_item(a, start, 128i32);
    assemble_emit_exception_table_item(a, size, 0i32);
    assemble_emit_exception_table_item(a, target, 0i32);
    assemble_emit_exception_table_item(a, depth_lasti, 0i32);
    0
}

fn assemble_exception_table(a: *mut assembler, instrs: *mut instruction_sequence) -> i32 {
    let mut ioffset: i32 = 0;
    let mut handler: __SlateRecord_pycore_instruction_sequence_h_19 =
        unsafe { core::mem::zeroed() };
    handler.h_label = 1i32.wrapping_neg();
    handler.h_startdepth = 1i32.wrapping_neg();
    handler.h_preserve_lasti = 1i32.wrapping_neg();
    let mut start: i32 = 1i32.wrapping_neg();
    let mut i: i32 = 0;
    while i < unsafe { (*instrs).s_used } {
        let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
            unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
        if (unsafe { (*instr).i_except_handler_info.h_label }) != handler.h_label {
            if handler.h_label >= 0 {
                let handler_offset: i32 = unsafe {
                    (*unsafe { (*instrs).s_instrs }.wrapping_offset(handler.h_label as isize))
                        .i_offset
                };
                RETURN_IF_ERROR__3!(assemble_emit_exception_table_entry(
                    a,
                    start,
                    ioffset,
                    handler_offset,
                    core::ptr::addr_of_mut!(handler)
                ));
            }
            start = ioffset;
            handler = unsafe { (*instr).i_except_handler_info };
        }
        let __v0: i32 = instr_size(instr);
        ioffset = ioffset.wrapping_add(__v0);
        i = i.wrapping_add(1);
    }
    if handler.h_label >= 0 {
        let handler_offset: i32 = unsafe {
            (*unsafe { (*instrs).s_instrs }.wrapping_offset(handler.h_label as isize)).i_offset
        };
        RETURN_IF_ERROR__3!(assemble_emit_exception_table_entry(
            a,
            start,
            ioffset,
            handler_offset,
            core::ptr::addr_of_mut!(handler)
        ));
    }
    0
}

// Code location emitting code. See locations.md for a description of the format.
fn write_location_byte(a: *mut assembler, val: i32) {
    unsafe {
        *PyBytes_AS_STRING!(unsafe { (*a).a_linetable })
            .wrapping_offset((*a).a_location_off as isize) = (val & 255) as i8;
    }
    let __v1: i32 = unsafe { (*a).a_location_off };
    unsafe {
        (*a).a_location_off = __v1.wrapping_add(1);
    }
}

fn location_pointer(a: *mut assembler) -> *mut u8 {
    (PyBytes_AS_STRING!(unsafe { (*a).a_linetable }) as *mut u8)
        .wrapping_offset((unsafe { (*a).a_location_off }) as isize)
}

fn write_location_first_byte(a: *mut assembler, code: i32, length: i32) {
    let __v0: i32 = write_location_entry_start(location_pointer(a), code, length);
    let __v2: i32 = unsafe { (*a).a_location_off };
    unsafe {
        (*a).a_location_off = __v2.wrapping_add(__v0);
    }
}

fn write_location_varint(a: *mut assembler, val: u32) {
    let ptr: *mut u8 = location_pointer(a);
    let __v0: i32 = write_varint(ptr, val);
    let __v2: i32 = unsafe { (*a).a_location_off };
    unsafe {
        (*a).a_location_off = __v2.wrapping_add(__v0);
    }
}

fn write_location_signed_varint(a: *mut assembler, val: i32) {
    let ptr: *mut u8 = location_pointer(a);
    let __v0: i32 = write_signed_varint(ptr, val);
    let __v2: i32 = unsafe { (*a).a_location_off };
    unsafe {
        (*a).a_location_off = __v2.wrapping_add(__v0);
    }
}

fn write_location_info_short_form(a: *mut assembler, length: i32, column: i32, end_column: i32) {
    let column_low_bits: i32 = column & 7;
    let column_group: i32 = column >> 3i32;
    write_location_first_byte(a, 0i32.wrapping_add(column_group), length);
    write_location_byte(a, column_low_bits << 4i32 | end_column.wrapping_sub(column));
}

fn write_location_info_oneline_form(
    a: *mut assembler,
    length: i32,
    line_delta: i32,
    column: i32,
    end_column: i32,
) {
    write_location_first_byte(a, 10i32.wrapping_add(line_delta), length);
    write_location_byte(a, column);
    write_location_byte(a, end_column);
}

fn write_location_info_long_form(
    a: *mut assembler,
    mut loc: __SlateRecord_pycore_symtable_h_51,
    length: i32,
) {
    write_location_first_byte(a, 14i32, length);
    write_location_signed_varint(a, loc.lineno.wrapping_sub(unsafe { (*a).a_lineno }));
    write_location_varint(a, loc.end_lineno.wrapping_sub(loc.lineno) as u32);
    write_location_varint(a, loc.col_offset.wrapping_add(1) as u32);
    write_location_varint(a, loc.end_col_offset.wrapping_add(1) as u32);
}

fn write_location_info_none(a: *mut assembler, length: i32) {
    write_location_first_byte(a, 15i32, length);
}

fn write_location_info_no_column(a: *mut assembler, length: i32, line_delta: i32) {
    write_location_first_byte(a, 13i32, length);
    write_location_signed_varint(a, line_delta);
}

// 1 + 6 + 6 + 6 + 6
fn write_location_info_entry(
    a: *mut assembler,
    mut loc: __SlateRecord_pycore_symtable_h_51,
    isize: i32,
) -> i32 {
    let len: i64 = PyBytes_GET_SIZE!(unsafe { (*a).a_linetable });
    if (unsafe { (*a).a_location_off }.wrapping_add(25) as i64) >= len {
        RETURN_IF_ERROR__3!(unsafe {
            _PyBytes_Resize(
                core::ptr::addr_of_mut!((*a).a_linetable),
                len.wrapping_mul(2),
            )
        });
    }
    if loc.lineno == unsafe { NO_LOCATION.lineno } {
        write_location_info_none(a, isize);
        return 0;
    }
    let line_delta: i32 = loc.lineno.wrapping_sub(unsafe { (*a).a_lineno });
    let column: i32 = loc.col_offset;
    let end_column: i32 = loc.end_col_offset;
    if column < 0 || end_column < 0 {
        if loc.end_lineno == loc.lineno || loc.end_lineno < 0 {
            write_location_info_no_column(a, isize, line_delta);
            unsafe {
                (*a).a_lineno = loc.lineno;
            }
            return 0;
        }
    } else {
        if loc.end_lineno == loc.lineno {
            if line_delta == 0
                && column < 80
                && end_column.wrapping_sub(column) < 16
                && end_column >= column
            {
                write_location_info_short_form(a, isize, column, end_column);
                return 0;
            }
            if line_delta >= 0 && line_delta < 3 && column < 128 && end_column < 128 {
                write_location_info_oneline_form(a, isize, line_delta, column, end_column);
                unsafe {
                    (*a).a_lineno = loc.lineno;
                }
                return 0;
            }
        }
    }
    write_location_info_long_form(a, loc, isize);
    unsafe {
        (*a).a_lineno = loc.lineno;
    }
    0
}

fn assemble_emit_location(
    a: *mut assembler,
    loc: __SlateRecord_pycore_symtable_h_51,
    mut isize: i32,
) -> i32 {
    if isize == 0 {
        return 0;
    }
    while isize > 8 {
        RETURN_IF_ERROR__3!(write_location_info_entry(a, loc, 8i32));
        isize = isize.wrapping_sub(8);
    }
    write_location_info_entry(a, loc, isize)
}

fn assemble_location_info(
    a: *mut assembler,
    instrs: *mut instruction_sequence,
    firstlineno: i32,
) -> i32 {
    unsafe {
        (*a).a_lineno = firstlineno;
    }
    let mut loc: __SlateRecord_pycore_symtable_h_51 = unsafe { NO_LOCATION };
    {
        let mut i: i32 = unsafe { (*instrs).s_used }.wrapping_sub(1);
        while i >= 0 {
            let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
                unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
            if same_location(unsafe { (*instr).i_loc }, unsafe { NEXT_LOCATION }) {
                if IS_TERMINATOR_OPCODE!(unsafe { (*instr).i_opcode }) {
                    unsafe {
                        (*instr).i_loc = NO_LOCATION;
                    }
                } else {
                    unsafe {
                        (*instr).i_loc = (*instr.wrapping_offset(1 as isize)).i_loc;
                    }
                }
            }
            i = i.wrapping_sub(1);
        }
    }
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < unsafe { (*instrs).s_used } {
        let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
            unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
        if !same_location(loc, unsafe { (*instr).i_loc }) {
            RETURN_IF_ERROR__3!(assemble_emit_location(a, loc, size));
            loc = unsafe { (*instr).i_loc };
            size = 0;
        }
        let __v3: i32 = instr_size(instr);
        size = size.wrapping_add(__v3);
        i = i.wrapping_add(1);
    }
    RETURN_IF_ERROR__3!(assemble_emit_location(a, loc, size));
    0
}

fn write_instr(
    mut codestr: *mut __SlateRecord_pycore_structs_h_32,
    instr: *mut __SlateRecord_pycore_instruction_sequence_h_30,
    ilen: i32,
) {
    let opcode: i32 = unsafe { (*instr).i_opcode };
    let oparg: i32 = unsafe { (*instr).i_oparg };
    let mut caches: i32 = (unsafe { _PyOpcode_Caches[opcode as usize] }) as i32;
    match ilen.wrapping_sub(caches) {
        4 => {
            unsafe {
                (*codestr).op.code = 69;
            }
            unsafe {
                (*codestr).op.arg = (oparg >> 24i32 & 255) as u8;
            }
            let __v1: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            codestr = __v1.wrapping_offset(1 as isize);
            unsafe {
                (*codestr).op.code = 69;
            }
            unsafe {
                (*codestr).op.arg = (oparg >> 16i32 & 255) as u8;
            }
            let _v13: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            let _v14: *mut __SlateRecord_pycore_structs_h_32 = _v13.wrapping_offset(1 as isize);
            codestr = _v14;
            unsafe {
                (*codestr).op.code = 69;
            }
            unsafe {
                (*codestr).op.arg = (oparg >> 8i32 & 255) as u8;
            }
            let _v15: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            let _v16: *mut __SlateRecord_pycore_structs_h_32 = _v15.wrapping_offset(1 as isize);
            codestr = _v16;
            unsafe {
                (*codestr).op.code = opcode as u8;
            }
            unsafe {
                (*codestr).op.arg = (oparg & 255) as u8;
            }
            let _v17: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            let _v18: *mut __SlateRecord_pycore_structs_h_32 = _v17.wrapping_offset(1 as isize);
            codestr = _v18;
        }
        3 => {
            unsafe {
                (*codestr).op.code = 69;
            }
            unsafe {
                (*codestr).op.arg = (oparg >> 16i32 & 255) as u8;
            }
            let __v3: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            codestr = __v3.wrapping_offset(1 as isize);
            unsafe {
                (*codestr).op.code = 69;
            }
            unsafe {
                (*codestr).op.arg = (oparg >> 8i32 & 255) as u8;
            }
            let _v19: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            let _v20: *mut __SlateRecord_pycore_structs_h_32 = _v19.wrapping_offset(1 as isize);
            codestr = _v20;
            unsafe {
                (*codestr).op.code = opcode as u8;
            }
            unsafe {
                (*codestr).op.arg = (oparg & 255) as u8;
            }
            let _v21: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            let _v22: *mut __SlateRecord_pycore_structs_h_32 = _v21.wrapping_offset(1 as isize);
            codestr = _v22;
        }
        2 => {
            unsafe {
                (*codestr).op.code = 69;
            }
            unsafe {
                (*codestr).op.arg = (oparg >> 8i32 & 255) as u8;
            }
            let __v5: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            codestr = __v5.wrapping_offset(1 as isize);
            unsafe {
                (*codestr).op.code = opcode as u8;
            }
            unsafe {
                (*codestr).op.arg = (oparg & 255) as u8;
            }
            let _v23: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            let _v24: *mut __SlateRecord_pycore_structs_h_32 = _v23.wrapping_offset(1 as isize);
            codestr = _v24;
        }
        1 => {
            unsafe {
                (*codestr).op.code = opcode as u8;
            }
            unsafe {
                (*codestr).op.arg = (oparg & 255) as u8;
            }
            let __v7: *mut __SlateRecord_pycore_structs_h_32 = codestr;
            codestr = __v7.wrapping_offset(1 as isize);
        }
        _ => {
            Py_UNREACHABLE!();
        }
    }
    loop {
        let __v11: i32 = caches;
        caches = __v11.wrapping_sub(1);
        if __v11 == 0 {
            break;
        }
        unsafe {
            (*codestr).op.code = 0;
        }
        unsafe {
            (*codestr).op.arg = 0;
        }
        let __v9: *mut __SlateRecord_pycore_structs_h_32 = codestr;
        codestr = __v9.wrapping_offset(1 as isize);
    }
}

// assemble_emit_instr()
// Extend the bytecode with a new instruction.
// Update lnotab if necessary.
fn assemble_emit_instr(
    a: *mut assembler,
    instr: *mut __SlateRecord_pycore_instruction_sequence_h_30,
) -> i32 {
    let len: i64 = PyBytes_GET_SIZE!(unsafe { (*a).a_bytecode });
    let size: i32 = instr_size(instr);
    if (unsafe { (*a).a_offset }.wrapping_add(size) as i64)
        >= len / (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i64)
    {
        if len > 9223372036854775807 / 2 {
            return 1i32.wrapping_neg();
        }
        RETURN_IF_ERROR__3!(unsafe {
            _PyBytes_Resize(
                core::ptr::addr_of_mut!((*a).a_bytecode),
                len.wrapping_mul(2),
            )
        });
    }
    let code: *mut __SlateRecord_pycore_structs_h_32 =
        (PyBytes_AS_STRING!(unsafe { (*a).a_bytecode }) as *mut __SlateRecord_pycore_structs_h_32)
            .wrapping_offset((unsafe { (*a).a_offset }) as isize);
    let __v1: i32 = unsafe { (*a).a_offset };
    unsafe {
        (*a).a_offset = __v1.wrapping_add(size);
    }
    write_instr(code, instr, size);
    0
}

fn assemble_emit(
    a: *mut assembler,
    instrs: *mut instruction_sequence,
    first_lineno: i32,
    const_cache: *mut _object,
) -> i32 {
    RETURN_IF_ERROR__3!(assemble_init(a, first_lineno));
    let mut i: i32 = 0;
    while i < unsafe { (*instrs).s_used } {
        let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
            unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
        RETURN_IF_ERROR__3!(assemble_emit_instr(a, instr));
        i = i.wrapping_add(1);
    }
    RETURN_IF_ERROR__3!(assemble_location_info(a, instrs, unsafe { (*a).a_lineno }));
    RETURN_IF_ERROR__3!(assemble_exception_table(a, instrs));
    RETURN_IF_ERROR__3!(unsafe {
        _PyBytes_Resize(
            core::ptr::addr_of_mut!((*a).a_except_table),
            (*a).a_except_table_off as i64,
        )
    });
    RETURN_IF_ERROR__3!(unsafe {
        _PyCompile_ConstCacheMergeOne(const_cache, core::ptr::addr_of_mut!((*a).a_except_table))
    });
    RETURN_IF_ERROR__3!(unsafe {
        _PyBytes_Resize(
            core::ptr::addr_of_mut!((*a).a_linetable),
            (*a).a_location_off as i64,
        )
    });
    RETURN_IF_ERROR__3!(unsafe {
        _PyCompile_ConstCacheMergeOne(const_cache, core::ptr::addr_of_mut!((*a).a_linetable))
    });
    RETURN_IF_ERROR__3!(unsafe {
        _PyBytes_Resize(
            core::ptr::addr_of_mut!((*a).a_bytecode),
            ((*a).a_offset as u64)
                .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64)
                as i64,
        )
    });
    RETURN_IF_ERROR__3!(unsafe {
        _PyCompile_ConstCacheMergeOne(const_cache, core::ptr::addr_of_mut!((*a).a_bytecode))
    });
    0
}

fn dict_keys_inorder(dict: *mut _object, offset: i64) -> *mut _object {
    let mut k: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = 0;
    let size: i64 = PyDict_GET_SIZE!(dict);
    let tuple: *mut _object = unsafe { PyTuple_New(size) };
    if tuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    while (unsafe {
        PyDict_Next(
            dict,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(k),
            core::ptr::addr_of_mut!(v),
        )
    }) != 0
    {
        let i: i64 = unsafe { PyLong_AsSsize_t(v) };
        let __v0: bool =
            i == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            Py_DECREF!(tuple);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(tuple, i.wrapping_sub(offset), Py_NewRef!(k));
    }
    tuple
}

// This is in codeobject.c.
fn compute_localsplus_info(
    umd: *mut __SlateRecord_pycore_compile_h_78,
    nlocalsplus: i32,
    flags: i32,
    names: *mut _object,
    kinds: *mut _object,
) -> i32 {
    let mut k: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = 0;
    // Set the locals kinds.  Arg vars fill the first portion of the list.
    let argvarkinds: __SlateAlign16<[__SlateRecord_assemble_c_494; 6]> = __SlateAlign16([
        {
            let mut __t0: __SlateRecord_assemble_c_494 = unsafe { core::mem::zeroed() };
            __t0.count = (unsafe { (*umd).u_posonlyargcount }) as i32;
            __t0.kind = 2u8;
            __t0
        },
        {
            let mut __t1: __SlateRecord_assemble_c_494 = unsafe { core::mem::zeroed() };
            __t1.count = (unsafe { (*umd).u_argcount }) as i32;
            __t1.kind = 6u8;
            __t1
        },
        {
            let mut __t2: __SlateRecord_assemble_c_494 = unsafe { core::mem::zeroed() };
            __t2.count = (unsafe { (*umd).u_kwonlyargcount }) as i32;
            __t2.kind = 4u8;
            __t2
        },
        {
            let mut __t3: __SlateRecord_assemble_c_494 = unsafe { core::mem::zeroed() };
            __t3.count = (flags & 4 != 0) as i32;
            __t3.kind = 10u8;
            __t3
        },
        {
            let mut __t4: __SlateRecord_assemble_c_494 = unsafe { core::mem::zeroed() };
            __t4.count = (flags & 8 != 0) as i32;
            __t4.kind = 12u8;
            __t4
        },
        {
            let mut __t5: __SlateRecord_assemble_c_494 = unsafe { core::mem::zeroed() };
            __t5.count = 1i32.wrapping_neg();
            __t5.kind = 0u8;
            __t5
        },
    ]);
    // the remaining local vars
    let mut max: i32 = 0;
    for i in 0i32..6i32 {
        max = if argvarkinds.0[i as usize].count < 0 {
            2147483647
        } else {
            max.wrapping_add(argvarkinds.0[i as usize].count)
        };
        loop {
            let __v5: bool = pos < (max as i64)
                && (unsafe {
                    PyDict_Next(
                        (*umd).u_varnames,
                        core::ptr::addr_of_mut!(pos),
                        core::ptr::addr_of_mut!(k),
                        core::ptr::addr_of_mut!(v),
                    )
                }) != 0;
            if !__v5 {
                break;
            }
            let offset: i32 = unsafe { PyLong_AsInt(v) };
            let __v0: bool =
                offset == 1i32.wrapping_neg() && !unsafe { PyErr_Occurred() }.is_null();
            if __v0 {
                return 1i32.wrapping_neg();
            }
            let mut kind: u8 = (32 | (argvarkinds.0[i as usize].kind as i32)) as u8;
            let mut has_key: i32 = unsafe { PyDict_Contains((*umd).u_fasthidden, k) };
            RETURN_IF_ERROR__3!(has_key);
            if has_key != 0 {
                kind = ((kind as i32) | 16) as u8;
            }
            has_key = unsafe { PyDict_Contains((*umd).u_cellvars, k) };
            RETURN_IF_ERROR__3!(has_key);
            if has_key != 0 {
                kind = ((kind as i32) | 64) as u8;
            }
            unsafe { _Py_set_localsplus_info(offset, k, kind, names, kinds) };
        }
    }
    let nlocals: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_varnames }) as i32;
    // This counter mirrors the fix done in fix_cell_offsets().
    let mut numdropped: i32 = 0;
    let mut cellvar_offset: i32 = 1i32.wrapping_neg();
    pos = 0;
    while (unsafe {
        PyDict_Next(
            (*umd).u_cellvars,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(k),
            core::ptr::addr_of_mut!(v),
        )
    }) != 0
    {
        let has_name: i32 = unsafe { PyDict_Contains((*umd).u_varnames, k) };
        RETURN_IF_ERROR__3!(has_name);
        if has_name != 0 {
            // Skip cells that are already covered by locals.
            numdropped = numdropped.wrapping_add(1);
        } else {
            cellvar_offset = unsafe { PyLong_AsInt(v) };
            let __v11: bool =
                cellvar_offset == 1i32.wrapping_neg() && !unsafe { PyErr_Occurred() }.is_null();
            if __v11 {
                return 1i32.wrapping_neg();
            }
            cellvar_offset = cellvar_offset.wrapping_add(nlocals.wrapping_sub(numdropped));
            unsafe { _Py_set_localsplus_info(cellvar_offset, k, 64u8, names, kinds) };
        }
    }
    pos = 0;
    while (unsafe {
        PyDict_Next(
            (*umd).u_freevars,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(k),
            core::ptr::addr_of_mut!(v),
        )
    }) != 0
    {
        let mut offset: i32 = unsafe { PyLong_AsInt(v) };
        let __v14: bool = offset == 1i32.wrapping_neg() && !unsafe { PyErr_Occurred() }.is_null();
        if __v14 {
            return 1i32.wrapping_neg();
        }
        offset = offset.wrapping_add(nlocals.wrapping_sub(numdropped));
        // XXX If the assertion below fails it is most likely because a freevar
        // was added to u_freevars with the wrong index due to not taking into
        // account cellvars already present, see gh-128632.
        unsafe { _Py_set_localsplus_info(offset, k, 128u8, names, kinds) };
    }
    0
}

fn makecode(
    umd: *mut __SlateRecord_pycore_compile_h_78,
    a: &assembler,
    const_cache: *mut _object,
    constslist: *mut _object,
    maxdepth: i32,
    nlocalsplus: i32,
    code_flags: i32,
    filename: *mut _object,
) -> *mut PyCodeObject {
    let mut con: core::mem::MaybeUninit<_PyCodeConstructor> = core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut _PyCodeConstructor =
        core::ptr::addr_of_mut!(con) as *mut _PyCodeConstructor;
    let mut kwonlyargcount: i32 = unsafe { core::mem::zeroed() };
    let mut posorkwargcount: i32 = unsafe { core::mem::zeroed() };
    let mut posonlyargcount: i32 = unsafe { core::mem::zeroed() };
    let mut localspluskinds: *mut _object = unsafe { core::mem::zeroed() };
    let mut localsplusnames: *mut _object = unsafe { core::mem::zeroed() };
    let mut consts: *mut _object = unsafe { core::mem::zeroed() };
    let mut names: *mut _object = unsafe { core::mem::zeroed() };
    let mut co: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    unsafe {
        co = core::ptr::null_mut::<PyCodeObject>();
        names = core::ptr::null_mut::<_object>();
        consts = core::ptr::null_mut::<_object>();
        localsplusnames = core::ptr::null_mut::<_object>();
        localspluskinds = core::ptr::null_mut::<_object>();
        names = dict_keys_inorder((*umd).u_names, 0i64);
        if !names.is_null() {
            if _PyCompile_ConstCacheMergeOne(const_cache, core::ptr::addr_of_mut!(names)) >= 0 {
                consts = PyList_AsTuple(constslist); // PyCode_New requires a tuple
                if !consts.is_null() {
                    if _PyCompile_ConstCacheMergeOne(const_cache, core::ptr::addr_of_mut!(consts))
                        >= 0
                    {
                        posonlyargcount = (*umd).u_posonlyargcount as i32;
                        posorkwargcount = (*umd).u_argcount as i32;
                        kwonlyargcount = (*umd).u_kwonlyargcount as i32;
                        localsplusnames = PyTuple_New(nlocalsplus as i64);
                        if !localsplusnames.is_null() {
                            localspluskinds = PyBytes_FromStringAndSize(
                                core::ptr::null::<i8>(),
                                nlocalsplus as i64,
                            );
                            if !localspluskinds.is_null() {
                                if compute_localsplus_info(
                                    umd,
                                    nlocalsplus,
                                    code_flags,
                                    localsplusnames,
                                    localspluskinds,
                                ) != 1i32.wrapping_neg()
                                {
                                    core::ptr::write(__slate_slot_0, {
                                        let mut __t0: _PyCodeConstructor = core::mem::zeroed();
                                        __t0.filename = filename;
                                        __t0.name = (*umd).u_name;
                                        __t0.qualname = if !unsafe { (*umd).u_qualname }.is_null() {
                                            (*umd).u_qualname
                                        } else {
                                            (*umd).u_name
                                        };
                                        __t0.flags = code_flags;
                                        __t0.code = (*(a as *const assembler)).a_bytecode;
                                        __t0.firstlineno = (*umd).u_firstlineno;
                                        __t0.linetable = (*(a as *const assembler)).a_linetable;
                                        __t0.consts = consts;
                                        __t0.names = names;
                                        __t0.localsplusnames = localsplusnames;
                                        __t0.localspluskinds = localspluskinds;
                                        __t0.argcount =
                                            posonlyargcount.wrapping_add(posorkwargcount);
                                        __t0.posonlyargcount = posonlyargcount;
                                        __t0.kwonlyargcount = kwonlyargcount;
                                        __t0.stacksize = maxdepth;
                                        __t0.exceptiontable =
                                            (*(a as *const assembler)).a_except_table;
                                        __t0
                                    });
                                    if _PyCode_Validate(core::ptr::addr_of_mut!(*__slate_slot_0))
                                        >= 0
                                    {
                                        if _PyCompile_ConstCacheMergeOne(
                                            const_cache,
                                            core::ptr::addr_of_mut!(localsplusnames),
                                        ) >= 0
                                        {
                                            (*__slate_slot_0).localsplusnames = localsplusnames;
                                            co = _PyCode_New(core::ptr::addr_of_mut!(
                                                *__slate_slot_0
                                            ));
                                            if co.is_null() {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(names);
        Py_XDECREF!(consts);
        Py_XDECREF!(localsplusnames);
        Py_XDECREF!(localspluskinds);
        return co;
    }
}

// The offset (in code units) of the END_SEND from the SEND in the `yield from` sequence.
fn resolve_jump_offsets(instrs: *mut instruction_sequence) -> i32 {
    // Compute the size of each instruction and fixup jump args.
    // Replace instruction index with position in bytecode.
    {
        let mut i: i32 = 0;
        while i < unsafe { (*instrs).s_used } {
            let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
                unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
            if OPCODE_HAS_JUMP!(unsafe { (*instr).i_opcode }) != 0 {
                unsafe {
                    (*instr).i_target = (*instr).i_oparg;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let mut extended_arg_recompile: i32 = 0;
    loop {
        let mut totsize: i32 = 0;
        {
            let mut i: i32 = 0;
            while i < unsafe { (*instrs).s_used } {
                let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
                    unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
                unsafe {
                    (*instr).i_offset = totsize;
                }
                let isize: i32 = instr_size(instr);
                totsize = totsize.wrapping_add(isize);
                i = i.wrapping_add(1);
            }
        }
        extended_arg_recompile = 0;
        let mut offset: i32 = 0;
        let mut i: i32 = 0;
        while i < unsafe { (*instrs).s_used } {
            let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
                unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
            let isize: i32 = instr_size(instr);
            // jump offsets are computed relative to
            // the instruction pointer after fetching
            // the jump instruction.
            offset = offset.wrapping_add(isize);
            if OPCODE_HAS_JUMP!(unsafe { (*instr).i_opcode }) != 0 {
                let target: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
                    unsafe { (*instrs).s_instrs }
                        .wrapping_offset((unsafe { (*instr).i_target }) as isize);
                unsafe {
                    (*instr).i_oparg = (*target).i_offset;
                }
                if (unsafe { (*instr).i_opcode }) == 68 {
                    // sys.monitoring needs to be able to find the matching END_SEND
                    //
                    // but the target is the SEND, so we adjust it here.
                    unsafe {
                        (*instr).i_oparg = offset.wrapping_sub((*instr).i_oparg).wrapping_sub(5);
                    }
                } else {
                    if (unsafe { (*instr).i_oparg }) < offset {
                        unsafe {
                            (*instr).i_oparg = offset.wrapping_sub((*instr).i_oparg);
                        }
                    } else {
                        unsafe {
                            (*instr).i_oparg = unsafe { (*instr).i_oparg }.wrapping_sub(offset);
                        }
                    }
                }
                if instr_size(instr) != isize {
                    extended_arg_recompile = 1;
                }
            }
            i = i.wrapping_add(1);
        }
        // XXX: This is an awful hack that could hurt performance, but
        // on the bright side it should work until we come up
        // with a better solution.
        //
        // The issue is that in the first loop instr_size() is
        // called, and it requires i_oparg be set appropriately.
        // There is a bootstrap problem because i_oparg is
        // calculated in the second loop above.
        //
        // So we loop until we stop seeing new EXTENDED_ARGs.
        // The only EXTENDED_ARGs that could be popping up are
        // ones in jump instructions.  So this should converge
        // fairly quickly.
        if extended_arg_recompile == 0 {
            break;
        }
    }
    0
}

fn resolve_unconditional_jumps(instrs: *mut instruction_sequence) -> i32 {
    // Resolve directions of unconditional jumps
    let mut i: i32 = 0;
    while i < unsafe { (*instrs).s_used } {
        let instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
            unsafe { (*instrs).s_instrs }.wrapping_offset(i as isize);
        let is_forward: bool = (unsafe { (*instr).i_oparg }) > i;
        match unsafe { (*instr).i_opcode } {
            257 => unsafe {
                (*instr).i_opcode = if is_forward { 77 } else { 75 };
            },
            260 => unsafe {
                (*instr).i_opcode = if is_forward { 77 } else { 76 };
            },
            _ => {
                if OPCODE_HAS_JUMP!(unsafe { (*instr).i_opcode }) != 0
                    && IS_PSEUDO_INSTR!(unsafe { (*instr).i_opcode })
                {
                    Py_UNREACHABLE!();
                }
            }
        }
        i = i.wrapping_add(1);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyAssemble_MakeCodeObject(
    umd: *mut __SlateRecord_pycore_compile_h_78,
    const_cache: *mut _object,
    consts: *mut _object,
    maxdepth: i32,
    instrs: *mut instruction_sequence,
    nlocalsplus: i32,
    code_flags: i32,
    filename: *mut _object,
) -> *mut PyCodeObject {
    if (unsafe { _PyInstructionSequence_ApplyLabelMap(instrs) }) < 0 {
        return core::ptr::null_mut::<PyCodeObject>();
    }
    if resolve_unconditional_jumps(instrs) < 0 {
        return core::ptr::null_mut::<PyCodeObject>();
    }
    if resolve_jump_offsets(instrs) < 0 {
        return core::ptr::null_mut::<PyCodeObject>();
    }
    let mut co: *mut PyCodeObject = core::ptr::null_mut::<PyCodeObject>();
    let mut a: assembler = unsafe { core::mem::zeroed() };
    let res: i32 = assemble_emit(
        core::ptr::addr_of_mut!(a),
        instrs,
        unsafe { (*umd).u_firstlineno },
        const_cache,
    );
    if res == 0 {
        co = makecode(
            umd,
            unsafe { &(*core::ptr::addr_of_mut!(a)) },
            const_cache,
            consts,
            maxdepth,
            nlocalsplus,
            code_flags,
            filename,
        );
    }
    assemble_free(core::ptr::addr_of_mut!(a));
    co
}
