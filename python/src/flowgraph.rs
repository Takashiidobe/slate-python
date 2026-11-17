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
//! A code object, the __code__ attribute
//! To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//!
//! defined in this macro:
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! _Py_CArray_EnsureCapacity
//! A soft limit for stack use, to avoid excessive
//! memory use for large constants, etc.
//!
//! The value 30 is plucked out of thin air.
//! Code that could use more stack than this is
//! rare, so the exact value is unimportant.
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! If we change this, we need to change the default value in the
//! signature of gc.collect.
//! Total tool ids available
//! _PyGenObject_HEAD defines the initial segment of generator
//! and coroutine objects.
//! These would be in pycore_long.h if it weren't for an include cycle.
//! Long value tag bits:
//! 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
//! 2: Set to 1 for the small ints
//! 3+ Unsigned digit count
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// NULL or a tuple
// NULL or a dict
// NULL or a tuple of cell objects
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
// Max static block nesting within a function
// Values used in the oparg for RESUME
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
// The size of the Bigint freelist
// The size of the cached powers of 5 array
// Must be a power of 2
// For now we hard-code this to a value for which we are confident
// all the static builtin types will fit (for all builds).
// Must be a power of 2
// Length of array of slotdef pointers used to store slots with the
// same __name__.  There should be at most MAX_EQUIV-1 slotdef entries with
// the same __name__, for any __name__. Since that's a static property, it is
// appropriate to declare fixed-size arrays for this.
// Count of all local monitoring events
// Count of all "real" monitoring events (not derived from other events)
// Count of all  monitoring events
// List of weak reference.
// Name of the generator.
// Qualified name of the generator.
// The frame
macro_rules! BB_HAS_FALLTHROUGH {
    ($B:expr) => {
        basicblock_nofallthrough(($B) as *const _PyCfgBasicblock) == 0
    };
}

macro_rules! HAS_TARGET {
    ($opcode:expr) => {
        OPCODE_HAS_JUMP!($opcode) != 0 || IS_BLOCK_PUSH_OPCODE!($opcode)
    };
}

macro_rules! INSTR_SET_LOC {
    ($I:expr, $LOC:expr) => {
        loop {
            let _instr__ptr_: *mut _PyCfgInstruction = $I;
            unsafe {
                (*_instr__ptr_).i_loc = $LOC;
            }
            if true {
                break;
            }
        }
    };
}

/// No args
macro_rules! INSTR_SET_OP0 {
    ($I:expr, $OP:expr) => {
        loop {
            let _instr__ptr_: *mut _PyCfgInstruction = $I;
            unsafe {
                (*_instr__ptr_).i_opcode = $OP;
            }
            unsafe {
                (*_instr__ptr_).i_oparg = 0;
            }
            if true {
                break;
            }
        }
    };
}

/// One arg
macro_rules! INSTR_SET_OP1 {
    ($I:expr, $OP:expr, $ARG:expr) => {
        loop {
            let _instr__ptr_: *mut _PyCfgInstruction = $I;
            unsafe {
                (*_instr__ptr_).i_opcode = $OP;
            }
            unsafe {
                (*_instr__ptr_).i_oparg = $ARG;
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! IS_BLOCK_PUSH_OPCODE {
    ($opcode:expr) => {
        ($opcode) == 264 || ($opcode) == 265 || ($opcode) == 263
    };
}

macro_rules! IS_CONDITIONAL_JUMP_OPCODE__2 {
    ($opcode:expr) => {
        ($opcode) == 100 || ($opcode) == 103 || ($opcode) == 101 || ($opcode) == 102
    };
}

macro_rules! IS_GENERATOR {
    ($CF:expr) => {
        ($CF) & (32 | 128 | 512)
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

macro_rules! IS_UNCONDITIONAL_JUMP_OPCODE {
    ($opcode:expr) => {
        ($opcode) == 257
            || ($opcode) == 260
            || ($opcode) == 77
            || ($opcode) == 75
            || ($opcode) == 76
    };
}

macro_rules! OPCODE_HAS_CONST {
    ($OP:expr) => {
        ((unsafe { _PyOpcode_opcode_metadata[($OP) as usize].flags }) as i32) & 2
    };
}

macro_rules! OPCODE_HAS_EVAL_BREAK {
    ($OP:expr) => {
        ((unsafe { _PyOpcode_opcode_metadata[($OP) as usize].flags }) as i32) & 64
    };
}

macro_rules! OPCODE_HAS_JUMP {
    ($OP:expr) => {
        ((unsafe { _PyOpcode_opcode_metadata[($OP) as usize].flags }) as i32) & 8
    };
}

macro_rules! PyBool_Check {
    ($x:expr) => {
        Py_IS_TYPE!($x, unsafe { core::ptr::addr_of_mut!(PyBool_Type) })
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

macro_rules! PyMem_New__3 {
    ($type:ty, $n:expr) => {{
        let __v: *mut $type =
            if (($n) as u64) > 9223372036854775807 / (core::mem::size_of::<$type>() as u64) {
                core::ptr::null_mut::<i32>()
            } else {
                (unsafe {
                    PyMem_Malloc((($n) as u64).wrapping_mul(core::mem::size_of::<$type>() as u64))
                }) as *mut $type
            };
        __v
    }};
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

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyUnicode_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 268435456u64)
    };
}

macro_rules! PyUnicode_GET_LENGTH {
    ($op:expr) => {
        PyUnicode_GET_LENGTH(_PyObject_CAST!($op))
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

macro_rules! Py_IsNone {
    ($x:expr) => {
        Py_Is!($x, unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
    };
}

macro_rules! Py_Is {
    ($x:expr, $y:expr) => {
        ($x) == $y
    };
}

/// Maximum value between x and y
macro_rules! Py_MAX__3 {
    ($x:expr, $y:expr) => {
        if (($x) as i64) > $y { ($x) as i64 } else { $y }
    };
}

/// Maximum value between x and y
macro_rules! Py_MAX {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $x } else { $y }
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

macro_rules! RETURN_IF_ERROR__6 {
    ($X:expr) => {{
        if ($X) == 1i32.wrapping_neg() {
            return 1i32.wrapping_neg() != 0;
        }
    }};
}

macro_rules! RETURN_IF_ERROR__2 {
    ($X:expr) => {{
        if ($X) == 1i32.wrapping_neg() {
            return 1i32.wrapping_neg();
        }
    }};
}

macro_rules! STORES_TO {
    ($instr:expr) => {
        if (unsafe { $instr.i_opcode }) == 112 || (unsafe { $instr.i_opcode }) == 266 {
            unsafe { $instr.i_oparg }
        } else {
            1i32.wrapping_neg()
        }
    };
}

/// This list is pretty small, since it's only okay to reorder opcodes that:
///
/// - can't affect control flow (like jumping or raising exceptions)
///
/// - can't invoke arbitrary code (besides finalizers)
///
/// - only touch the TOS (and pop it when finished)
macro_rules! SWAPPABLE {
    ($opcode:expr) => {
        ($opcode) == 112 || ($opcode) == 266 || ($opcode) == 31
    };
}

macro_rules! _PY_IS_SMALL_INT {
    ($val:expr) => {
        (5i32.wrapping_neg() as i64) <= $val && ($val) < 257
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
    };
}

macro_rules! _PyInstructionSequence_Check {
    ($v:expr) => {
        Py_IS_TYPE!($v, unsafe {
            core::ptr::addr_of_mut!(_PyInstructionSequence_Type)
        })
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
    static mut PyLong_Type: _typeobject;
    static mut PyBool_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyExc_KeyboardInterrupt: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _PyInstructionSequence_Type: _typeobject;
    static mut _PyOpcode_opcode_metadata: [opcode_metadata; 267];
    static mut _PyOpcode_Deopt: [u8; 256];
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Calloc(arg0: u64, arg1: u64) -> *mut ();
    fn PyMem_Realloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyMem_RawCalloc(arg0: u64, arg1: u64) -> *mut ();
    fn PyMem_RawFree(arg0: *mut ());
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsLongAndOverflow(arg0: *mut _object, arg1: *mut i32) -> i64;
    fn PyLong_AsSize_t(arg0: *mut _object) -> u64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn _PyLong_NumBits(arg0: *mut _object) -> i64;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyList_SetItem(arg0: *mut _object, arg1: i64, arg2: *mut _object) -> i32;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyList_SetSlice(arg0: *mut _object, arg1: i64, arg2: i64, arg3: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_GetItemRef(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyFrozenSet_New(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetItem(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Add(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Subtract(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Multiply(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_FloorDivide(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_TrueDivide(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Remainder(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Power(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyNumber_Negative(arg0: *mut _object) -> *mut _object;
    fn PyNumber_Positive(arg0: *mut _object) -> *mut _object;
    fn PyNumber_Invert(arg0: *mut _object) -> *mut _object;
    fn PyNumber_Lshift(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Rshift(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_And(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Xor(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyNumber_Or(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _Py_CArray_EnsureCapacity(arg0: *mut __SlateRecord_pycore_c_array_h_20, arg1: i32) -> i32;
    fn _PyInstructionSequence_New() -> *mut _object;
    fn _PyInstructionSequence_UseLabel(arg0: *mut instruction_sequence, arg1: i32) -> i32;
    fn _PyInstructionSequence_Addop(
        arg0: *mut instruction_sequence,
        arg1: i32,
        arg2: i32,
        arg3: __SlateRecord_pycore_symtable_h_51,
    ) -> i32;
    fn _PyInstructionSequence_ApplyLabelMap(arg0: *mut instruction_sequence) -> i32;
    fn PyInstructionSequence_Fini(arg0: *mut instruction_sequence);
    fn _PyCompile_ConstCacheMergeOne(arg0: *mut _object, arg1: *mut *mut _object) -> i32;
    fn _Py_hashtable_new(
        arg0: Option<unsafe extern "C-unwind" fn(*const ()) -> u64>,
        arg1: Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
    ) -> *mut _Py_hashtable_t;
    fn _Py_hashtable_hash_ptr(arg0: *const ()) -> u64;
    fn _Py_hashtable_compare_direct(arg0: *const (), arg1: *const ()) -> i32;
    fn _Py_hashtable_destroy(arg0: *mut _Py_hashtable_t);
    fn _Py_hashtable_set(arg0: *mut _Py_hashtable_t, arg1: *const (), arg2: *mut ()) -> i32;
    fn _PyOpcode_num_popped(arg0: i32, arg1: i32) -> i32;
    fn _PyOpcode_num_pushed(arg0: i32, arg1: i32) -> i32;
    fn __builtin_unreachable();
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _dictkeysobject {}

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
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits89U0,
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

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF!(obj);
    obj
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_h_15 {
    ob_base: __SlateRecord_object_h_172,
    ob_shash: i64,
    ob_sval: [i8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _dictvalues {}

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
fn PyUnicode_GET_LENGTH(op: *mut _object) -> i64 {
    unsafe { (*_PyASCIIObject_CAST!(op)).length }
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
struct __SlateRecord_pycore_c_array_h_20 {
    array: *mut (),
    allocated_entries: i32,
    item_size: u64,
    initial_num_entries: i32,
}

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
    pub struct __SlateBits89U0 {
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
    pub struct __SlateBits143U0 {
        #[bits(1)]
        pub b_preserve_lasti: u32,
        #[bits(1)]
        pub b_visited: u32,
        #[bits(1)]
        pub b_except_handler: u32,
        #[bits(1)]
        pub b_cold: u32,
        #[bits(1)]
        pub b_warm: u32,
        #[bits(3, access = na)]
        pub __slate_pad_5: u8,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_instruction_sequence_h_55 {
    id: i32,
}

use slate_support::memcpy;
use slate_support::memset;

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

fn _Py_hashtable_get_entry(
    ht: *mut _Py_hashtable_t,
    key: *const (),
) -> *mut __SlateRecord_pycore_hashtable_h_35 {
    unsafe { unsafe { (*ht).get_entry_func }.unwrap()(ht, key) }
}

fn _PyLong_IsZero(op: *const _longobject) -> bool {
    (unsafe { (*op).long_value.lv_tag }) & 3 == 1
}

fn _PyLong_IsPositive(op: *const _longobject) -> bool {
    (unsafe { (*op).long_value.lv_tag }) & 3 == 0
}

#[repr(C)]
#[derive(Clone, Copy)]
struct opcode_metadata {
    valid_entry: u8,
    instr_format: u8,
    flags: u16,
}

// _PyMem_IsPtrFreed()
// _PY_IS_SMALL_INT()
// _Py_hashtable_t
// OPCODE_HAS_ARG, etc
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCfgInstruction {
    i_opcode: i32,
    i_oparg: i32,
    i_loc: __SlateRecord_pycore_symtable_h_51,
    /// target block (if jump instruction)
    i_target: *mut _PyCfgBasicblock,
    /// target block when exception is raised
    i_except: *mut _PyCfgBasicblock,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCfgBasicblock {
    /// Each basicblock in a compilation unit is linked via b_list in the
    /// reverse order that the block are allocated.  b_list points to the next
    /// block in this list, not to be confused with b_next, which is next by
    /// control flow.
    b_list: *mut _PyCfgBasicblock,
    /// The label of this block if it is a jump target, -1 otherwise
    b_label: __SlateRecord_pycore_instruction_sequence_h_55,
    /// Exception stack at start of block, used by assembler to create the exception handling table
    b_exceptstack: *mut _PyCfgExceptStack,
    /// pointer to an array of instructions, initially NULL
    b_instr: *mut _PyCfgInstruction,
    /// If b_next is non-NULL, it is a pointer to the next
    /// block reached by normal control flow.
    b_next: *mut _PyCfgBasicblock,
    /// number of instructions used
    b_iused: i32,
    /// length of instruction array (b_instr)
    b_ialloc: i32,
    /// Used by add_checks_for_loads_of_unknown_variables
    b_unsafe_locals_mask: u64,
    /// Number of predecessors that a block has.
    b_predecessors: i32,
    /// depth of stack upon entry of block, computed by stackdepth()
    b_startdepth: i32,
    /// Basic block is an exception handler that preserves lasti
    __slate_bits_0: __slate_bits::__SlateBits143U0,
    // Used by compiler passes to mark whether they have visited a basic block.
    // b_except_handler is used by the cold-detection algorithm to mark exception targets
    // b_cold is true if this block is not perf critical (like an exception handler)
    // b_warm is used by the cold-detection algorithm to mark blocks which are definitely not cold
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCfgBuilder {
    /// The entryblock, at which control flow begins. All blocks of the
    /// CFG are reachable through the b_next links
    g_entryblock: *mut _PyCfgBasicblock,
    /// Pointer to the most recently allocated block.  By following
    /// b_list links, you can reach all allocated blocks.
    g_block_list: *mut _PyCfgBasicblock,
    /// pointer to the block currently being constructed
    g_curblock: *mut _PyCfgBasicblock,
    /// label for the next instruction to be placed
    g_current_label: __SlateRecord_pycore_instruction_sequence_h_55,
}

fn is_block_push(i: *mut _PyCfgInstruction) -> i32 {
    IS_BLOCK_PUSH_OPCODE!(unsafe { (*i).i_opcode }) as i32
}

fn is_jump(i: *mut _PyCfgInstruction) -> i32 {
    OPCODE_HAS_JUMP!(unsafe { (*i).i_opcode })
}

// Blocks
/// Returns the offset of the next instruction in the current block's
/// b_instr array.  Resizes the b_instr as necessary.
/// Returns -1 on failure.
fn basicblock_next_instr(b: *mut _PyCfgBasicblock) -> i32 {
    let mut array: __SlateRecord_pycore_c_array_h_20 = {
        let mut __t0: __SlateRecord_pycore_c_array_h_20 = unsafe { core::mem::zeroed() };
        __t0.array = (unsafe { (*b).b_instr }) as *mut ();
        __t0.allocated_entries = unsafe { (*b).b_ialloc };
        __t0.item_size = core::mem::size_of::<_PyCfgInstruction>() as u64;
        __t0.initial_num_entries = 16i32;
        __t0
    };
    RETURN_IF_ERROR__2!(unsafe {
        _Py_CArray_EnsureCapacity(
            core::ptr::addr_of_mut!(array),
            unsafe { (*b).b_iused }.wrapping_add(1),
        )
    });
    unsafe {
        (*b).b_instr = array.array as *mut _PyCfgInstruction;
    }
    unsafe {
        (*b).b_ialloc = array.allocated_entries;
    }
    let __v1: i32 = unsafe { (*b).b_iused };
    unsafe {
        (*b).b_iused = __v1.wrapping_add(1);
    }
    __v1
}

fn basicblock_last_instr(b: *const _PyCfgBasicblock) -> *mut _PyCfgInstruction {
    if (unsafe { (*b).b_iused }) > 0 {
        return unsafe { (*b).b_instr }
            .wrapping_offset(unsafe { (*b).b_iused }.wrapping_sub(1) as isize);
    }
    core::ptr::null_mut::<_PyCfgInstruction>()
}

// Allocate a new block and return a pointer to it.
// Returns NULL on error.
fn cfg_builder_new_block(g: *mut _PyCfgBuilder) -> *mut _PyCfgBasicblock {
    let mut __slate_compound_0: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let b: *mut _PyCfgBasicblock =
        (unsafe { PyMem_Calloc(1u64, core::mem::size_of::<_PyCfgBasicblock>() as u64) })
            as *mut _PyCfgBasicblock;
    if b.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_PyCfgBasicblock>();
    }
    // Extend the singly linked list of blocks with new block.
    unsafe {
        (*b).b_list = (*g).g_block_list;
    }
    unsafe {
        (*g).g_block_list = b;
    }
    unsafe {
        (*b).b_label = *{
            __slate_compound_0 = __SlateRecord_pycore_instruction_sequence_h_55 {
                id: 1i32.wrapping_neg(),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    b
}

fn basicblock_addop(
    b: *mut _PyCfgBasicblock,
    opcode: i32,
    oparg: i32,
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    let off: i32 = basicblock_next_instr(b);
    if off < 0 {
        return 1i32.wrapping_neg();
    }
    let i: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(off as isize);
    unsafe {
        (*i).i_opcode = opcode;
    }
    unsafe {
        (*i).i_oparg = oparg;
    }
    unsafe {
        (*i).i_target = core::ptr::null_mut::<_PyCfgBasicblock>();
    }
    unsafe {
        (*i).i_loc = loc;
    }
    0
}

fn basicblock_add_jump(
    b: *mut _PyCfgBasicblock,
    opcode: i32,
    target: *mut _PyCfgBasicblock,
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    let mut last: *mut _PyCfgInstruction = basicblock_last_instr(b as *const _PyCfgBasicblock);
    let __v0: bool = !last.is_null() && is_jump(last) != 0;
    if __v0 {
        return 1i32.wrapping_neg();
    }
    RETURN_IF_ERROR__2!(basicblock_addop(
        b,
        opcode,
        unsafe { (*target).b_label.id },
        loc
    ));
    last = basicblock_last_instr(b as *const _PyCfgBasicblock);
    unsafe {
        (*last).i_target = target;
    }
    0
}

fn basicblock_append_instructions(to: *mut _PyCfgBasicblock, from: *mut _PyCfgBasicblock) -> i32 {
    let mut i: i32 = 0;
    while i < unsafe { (*from).b_iused } {
        let n: i32 = basicblock_next_instr(to);
        if n < 0 {
            return 1i32.wrapping_neg();
        }
        unsafe {
            *unsafe { (*to).b_instr }.wrapping_offset(n as isize) =
                *unsafe { (*from).b_instr }.wrapping_offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    0
}

fn basicblock_nofallthrough(b: *const _PyCfgBasicblock) -> i32 {
    let last: *mut _PyCfgInstruction = basicblock_last_instr(b);
    (!last.is_null()
        && (IS_SCOPE_EXIT_OPCODE!(unsafe { (*last).i_opcode })
            || IS_UNCONDITIONAL_JUMP_OPCODE!(unsafe { (*last).i_opcode }))) as i32
}

fn copy_basicblock(g: *mut _PyCfgBuilder, block: *mut _PyCfgBasicblock) -> *mut _PyCfgBasicblock {
    // Cannot copy a block if it has a fallthrough, since
    // a block can only have one fallthrough predecessor.
    let result: *mut _PyCfgBasicblock = cfg_builder_new_block(g);
    if result.is_null() {
        return core::ptr::null_mut::<_PyCfgBasicblock>();
    }
    if basicblock_append_instructions(result, block) < 0 {
        return core::ptr::null_mut::<_PyCfgBasicblock>();
    }
    result
}

fn basicblock_insert_instruction(
    block: *mut _PyCfgBasicblock,
    pos: i32,
    instr: *mut _PyCfgInstruction,
) -> i32 {
    RETURN_IF_ERROR__2!(basicblock_next_instr(block));
    let mut i: i32 = unsafe { (*block).b_iused }.wrapping_sub(1);
    while i > pos {
        unsafe {
            *unsafe { (*block).b_instr }.wrapping_offset(i as isize) =
                *unsafe { (*block).b_instr }.wrapping_offset(i.wrapping_sub(1) as isize);
        }
        i = i.wrapping_sub(1);
    }
    unsafe {
        *unsafe { (*block).b_instr }.wrapping_offset(pos as isize) = *instr;
    }
    0
}

// CFG construction and modification
/// For debugging purposes only
fn cfg_builder_use_next_block(
    g: *mut _PyCfgBuilder,
    block: *mut _PyCfgBasicblock,
) -> *mut _PyCfgBasicblock {
    unsafe {
        (*(*g).g_curblock).b_next = block;
    }
    unsafe {
        (*g).g_curblock = block;
    }
    block
}

fn basicblock_exits_scope(b: *const _PyCfgBasicblock) -> i32 {
    let last: *mut _PyCfgInstruction = basicblock_last_instr(b);
    (!last.is_null() && IS_SCOPE_EXIT_OPCODE!(unsafe { (*last).i_opcode })) as i32
}

fn basicblock_has_eval_break(b: *const _PyCfgBasicblock) -> i32 {
    let mut i: i32 = 0;
    while i < unsafe { (*b).b_iused } {
        if OPCODE_HAS_EVAL_BREAK!(unsafe {
            (*unsafe { (*b).b_instr }.wrapping_offset(i as isize)).i_opcode
        }) != 0
        {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}

fn cfg_builder_current_block_is_terminated(g: *mut _PyCfgBuilder) -> bool {
    let mut __slate_compound_0: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let last: *mut _PyCfgInstruction =
        basicblock_last_instr((unsafe { (*g).g_curblock }) as *const _PyCfgBasicblock);
    if !last.is_null() && IS_TERMINATOR_OPCODE!(unsafe { (*last).i_opcode }) {
        return true;
    }
    if (unsafe { (*g).g_current_label.id })
        != unsafe {
            (*{
                __slate_compound_0 = __SlateRecord_pycore_instruction_sequence_h_55 {
                    id: 1i32.wrapping_neg(),
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            })
            .id
        }
    {
        if !last.is_null()
            || (unsafe { (*(*g).g_curblock).b_label.id })
                != unsafe {
                    (*{
                        __slate_compound_1 = __SlateRecord_pycore_instruction_sequence_h_55 {
                            id: 1i32.wrapping_neg(),
                        };
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    })
                    .id
                }
        {
            return true;
        } else {
            // current block is empty, label it
            unsafe {
                (*(*g).g_curblock).b_label = (*g).g_current_label;
            }
            unsafe {
                (*g).g_current_label = *{
                    __slate_compound_2 = __SlateRecord_pycore_instruction_sequence_h_55 {
                        id: 1i32.wrapping_neg(),
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                };
            }
        }
    }
    false
}

fn cfg_builder_maybe_start_new_block(g: *mut _PyCfgBuilder) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    if cfg_builder_current_block_is_terminated(g) {
        let b: *mut _PyCfgBasicblock = cfg_builder_new_block(g);
        if b.is_null() {
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*b).b_label = (*g).g_current_label;
        }
        unsafe {
            (*g).g_current_label = *{
                __slate_compound_0 = __SlateRecord_pycore_instruction_sequence_h_55 {
                    id: 1i32.wrapping_neg(),
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
        cfg_builder_use_next_block(g, b);
    }
    0
}

fn init_cfg_builder(g: *mut _PyCfgBuilder) -> i32 {
    let mut __slate_compound_1: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    unsafe {
        (*g).g_block_list = core::ptr::null_mut::<_PyCfgBasicblock>();
    }
    let block: *mut _PyCfgBasicblock = cfg_builder_new_block(g);
    if block.is_null() {
        return 1i32.wrapping_neg();
    }
    let __v0: *mut _PyCfgBasicblock = block;
    unsafe {
        (*g).g_entryblock = __v0;
    }
    unsafe {
        (*g).g_curblock = __v0;
    }
    unsafe {
        (*g).g_current_label = *{
            __slate_compound_1 = __SlateRecord_pycore_instruction_sequence_h_55 {
                id: 1i32.wrapping_neg(),
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        };
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfgBuilder_New() -> *mut _PyCfgBuilder {
    let g: *mut _PyCfgBuilder =
        (unsafe { PyMem_Malloc(core::mem::size_of::<_PyCfgBuilder>() as u64) })
            as *mut _PyCfgBuilder;
    if g.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_PyCfgBuilder>();
    }
    unsafe {
        core::ptr::write_bytes(
            ((g as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<_PyCfgBuilder>() as u64) as usize) as usize,
        )
    };
    if init_cfg_builder(g) < 0 {
        unsafe { PyMem_Free(g as *mut ()) };
        return core::ptr::null_mut::<_PyCfgBuilder>();
    }
    g
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfgBuilder_Free(g: *mut _PyCfgBuilder) {
    if g.is_null() {
        return;
    }
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_block_list };
    while !b.is_null() {
        if !unsafe { (*b).b_instr }.is_null() {
            unsafe { PyMem_Free((*b).b_instr as *mut ()) };
        }
        let next: *mut _PyCfgBasicblock = unsafe { (*b).b_list };
        unsafe { PyMem_Free(b as *mut ()) };
        b = next;
    }
    unsafe { PyMem_Free(g as *mut ()) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfgBuilder_CheckSize(g: *mut _PyCfgBuilder) -> i32 {
    let mut nblocks: i32 = 0;
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_block_list };
    while !b.is_null() {
        nblocks = nblocks.wrapping_add(1);
        b = unsafe { (*b).b_list };
    }
    if (nblocks as u64)
        > 18446744073709551615 / (core::mem::size_of::<*mut _PyCfgBasicblock>() as u64)
    {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfgBuilder_UseLabel(
    g: *mut _PyCfgBuilder,
    lbl: __SlateRecord_pycore_instruction_sequence_h_55,
) -> i32 {
    unsafe {
        (*g).g_current_label = lbl;
    }
    cfg_builder_maybe_start_new_block(g)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfgBuilder_Addop(
    g: *mut _PyCfgBuilder,
    opcode: i32,
    oparg: i32,
    loc: __SlateRecord_pycore_symtable_h_51,
) -> i32 {
    RETURN_IF_ERROR__2!(cfg_builder_maybe_start_new_block(g));
    basicblock_addop(unsafe { (*g).g_curblock }, opcode, oparg, loc)
}

fn next_nonempty_block(mut b: *mut _PyCfgBasicblock) -> *mut _PyCfgBasicblock {
    while !b.is_null() && (unsafe { (*b).b_iused }) == 0 {
        b = unsafe { (*b).b_next };
    }
    b
}

// debugging helpers
// CFG preprocessing (jump targets and exceptions)
fn normalize_jumps_in_block(g: *mut _PyCfgBuilder, b: *mut _PyCfgBasicblock) -> i32 {
    let last: *mut _PyCfgInstruction = basicblock_last_instr(b as *const _PyCfgBasicblock);
    if last.is_null() || !IS_CONDITIONAL_JUMP_OPCODE__2!(unsafe { (*last).i_opcode }) {
        return 0;
    }
    let is_forward: bool =
        ((unsafe { (*(*last).i_target).__slate_bits_0.__get_b_visited() }) as i32) == 0;
    if is_forward {
        RETURN_IF_ERROR__2!(basicblock_addop(b, 28i32, 0i32, unsafe { (*last).i_loc }));
        return 0;
    }
    let mut reversed_opcode: i32 = 0;
    match unsafe { (*last).i_opcode } {
        102 => {
            reversed_opcode = 101;
        }
        101 => {
            reversed_opcode = 102;
        }
        100 => {
            reversed_opcode = 103;
        }
        103 => {
            reversed_opcode = 100;
        }
        _ => {}
    }
    // transform 'conditional jump T' to
    // 'reversed_jump b_next' followed by 'jump_backwards T'
    let target: *mut _PyCfgBasicblock = unsafe { (*last).i_target };
    let backwards_jump: *mut _PyCfgBasicblock = cfg_builder_new_block(g);
    if backwards_jump.is_null() {
        return 1i32.wrapping_neg();
    }
    RETURN_IF_ERROR__2!(basicblock_addop(backwards_jump, 28i32, 0i32, unsafe {
        (*last).i_loc
    }));
    RETURN_IF_ERROR__2!(basicblock_add_jump(
        backwards_jump,
        257i32,
        target,
        unsafe { (*last).i_loc }
    ));
    unsafe {
        (*backwards_jump).b_startdepth = (*target).b_startdepth;
    }
    unsafe {
        (*last).i_opcode = reversed_opcode;
    }
    unsafe {
        (*last).i_target = (*b).b_next;
    }
    unsafe {
        let __t0 = ((*b).__slate_bits_0.__get_b_cold() as i32) as u32;
        (*backwards_jump).__slate_bits_0.__set_b_cold(__t0);
    }
    unsafe {
        (*backwards_jump).b_next = (*b).b_next;
    }
    unsafe {
        (*b).b_next = backwards_jump;
    }
    0
}

fn normalize_jumps(g: *mut _PyCfgBuilder) -> i32 {
    let entryblock: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            unsafe {
                (*b).__slate_bits_0.__set_b_visited(0u32);
            }
            b = unsafe { (*b).b_next };
        }
    }
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !b.is_null() {
        unsafe {
            (*b).__slate_bits_0.__set_b_visited(1u32);
        }
        RETURN_IF_ERROR__2!(normalize_jumps_in_block(g, b));
        b = unsafe { (*b).b_next };
    }
    0
}

fn check_cfg(g: *mut _PyCfgBuilder) -> i32 {
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        // Raise SystemError if jump or exit is not last instruction in the block.
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let opcode: i32 =
                unsafe { (*unsafe { (*b).b_instr }.wrapping_offset(i as isize)).i_opcode };
            if IS_TERMINATOR_OPCODE!(opcode) {
                if i != unsafe { (*b).b_iused }.wrapping_sub(1) {
                    unsafe {
                        PyErr_SetString(
                            PyExc_SystemError,
                            c"malformed control flow graph.".as_ptr(),
                        )
                    };
                    return 1i32.wrapping_neg();
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    0
}

fn get_max_label(mut b: *mut _PyCfgBasicblock) -> i32 {
    let mut lbl: i32 = 1i32.wrapping_neg();
    while !b.is_null() {
        if (unsafe { (*b).b_label.id }) > lbl {
            lbl = unsafe { (*b).b_label.id };
        }
        b = unsafe { (*b).b_next };
    }
    lbl
}

/// Calculate the actual jump target from the target_label
fn translate_jump_labels_to_targets(entryblock: *mut _PyCfgBasicblock) -> i32 {
    let max_label: i32 = get_max_label(entryblock);
    let mapsize: u64 = (core::mem::size_of::<*mut _PyCfgBasicblock>() as u64)
        .wrapping_mul(max_label.wrapping_add(1) as u64);
    let label2block: *mut *mut _PyCfgBasicblock =
        (unsafe { PyMem_Malloc(mapsize) }) as *mut *mut _PyCfgBasicblock;
    if label2block.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    unsafe {
        core::ptr::write_bytes(
            ((label2block as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            (mapsize as usize) as usize,
        )
    };
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            if (unsafe { (*b).b_label.id }) >= 0 {
                unsafe {
                    *label2block.wrapping_offset((*b).b_label.id as isize) = b;
                }
            }
            b = unsafe { (*b).b_next };
        }
    }
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !b.is_null() {
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if HAS_TARGET!(unsafe { (*instr).i_opcode }) {
                let lbl: i32 = unsafe { (*instr).i_oparg };
                unsafe {
                    (*instr).i_target = *label2block.wrapping_offset(lbl as isize);
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    unsafe { PyMem_Free(label2block as *mut ()) };
    0
}

fn mark_except_handlers(mut b: *mut _PyCfgBasicblock) -> i32 {
    while !b.is_null() {
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if is_block_push(instr) != 0 {
                unsafe {
                    (*(*instr).i_target)
                        .__slate_bits_0
                        .__set_b_except_handler(1u32);
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    0
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCfgExceptStack {
    handlers: [*mut _PyCfgBasicblock; 23],
    depth: i32,
}

fn push_except_block(
    stack: *mut _PyCfgExceptStack,
    setup: *mut _PyCfgInstruction,
) -> *mut _PyCfgBasicblock {
    let opcode: i32 = unsafe { (*setup).i_opcode };
    let target: *mut _PyCfgBasicblock = unsafe { (*setup).i_target };
    if opcode == 265 || opcode == 263 {
        unsafe {
            (*target).__slate_bits_0.__set_b_preserve_lasti(1u32);
        }
    }
    let __v1: i32 = unsafe { (*stack).depth };
    let __v2: i32 = __v1.wrapping_add(1);
    unsafe {
        (*stack).depth = __v2;
    }
    unsafe {
        *((*stack).handlers.as_mut_ptr() as *mut *mut _PyCfgBasicblock)
            .wrapping_offset(__v2 as isize) = target;
    }
    target
}

fn pop_except_block(stack: *mut _PyCfgExceptStack) -> *mut _PyCfgBasicblock {
    let __v1: i32 = unsafe { (*stack).depth };
    let __v2: i32 = __v1.wrapping_sub(1);
    unsafe {
        (*stack).depth = __v2;
    }
    unsafe {
        *((*stack).handlers.as_mut_ptr() as *mut *mut _PyCfgBasicblock)
            .wrapping_offset(__v2 as isize)
    }
}

fn except_stack_top(stack: *mut _PyCfgExceptStack) -> *mut _PyCfgBasicblock {
    unsafe {
        *((*stack).handlers.as_mut_ptr() as *mut *mut _PyCfgBasicblock)
            .wrapping_offset((*stack).depth as isize)
    }
}

fn make_except_stack() -> *mut _PyCfgExceptStack {
    let new: *mut _PyCfgExceptStack =
        (unsafe { PyMem_Malloc(core::mem::size_of::<_PyCfgExceptStack>() as u64) })
            as *mut _PyCfgExceptStack;
    if new.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_PyCfgExceptStack>();
    }
    unsafe {
        (*new).depth = 0;
    }
    unsafe {
        *((*new).handlers.as_mut_ptr() as *mut *mut _PyCfgBasicblock).wrapping_offset(0 as isize) =
            core::ptr::null_mut::<_PyCfgBasicblock>();
    }
    new
}

fn copy_except_stack(stack: *mut _PyCfgExceptStack) -> *mut _PyCfgExceptStack {
    let copy: *mut _PyCfgExceptStack =
        (unsafe { PyMem_Malloc(core::mem::size_of::<_PyCfgExceptStack>() as u64) })
            as *mut _PyCfgExceptStack;
    if copy.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_PyCfgExceptStack>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((stack as *const ()) as *const core::ffi::c_void) as *const u8,
            ((copy as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<_PyCfgExceptStack>() as u64) as usize) as usize,
        )
    };
    copy
}

fn make_cfg_traversal_stack(mut b: *mut _PyCfgBasicblock) -> *mut *mut _PyCfgBasicblock {
    let mut nblocks: i32 = 0;
    while !b.is_null() {
        unsafe {
            (*b).__slate_bits_0.__set_b_visited(0u32);
        }
        nblocks = nblocks.wrapping_add(1);
        b = unsafe { (*b).b_next };
    }
    let stack: *mut *mut _PyCfgBasicblock = (unsafe {
        PyMem_Malloc(
            (core::mem::size_of::<*mut _PyCfgBasicblock>() as u64).wrapping_mul(nblocks as u64),
        )
    }) as *mut *mut _PyCfgBasicblock;
    if stack.is_null() {
        unsafe { PyErr_NoMemory() };
    }
    stack
}

/// Compute the stack effects of opcode with argument oparg.
///
/// Some opcodes have different stack effect when jump to the target and
/// when not jump. The 'jump' parameter specifies the case:
///
/// * 0 -- when not jump
/// * 1 -- when jump
/// * -1 -- maximal
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_flowgraph_c_764 {
    /// The stack effect of the instruction.
    net: i32,
}

fn get_stack_effects(
    opcode: i32,
    oparg: i32,
    jump: i32,
    effects: &mut __SlateRecord_flowgraph_c_764,
) -> i32 {
    if opcode < 0 {
        return 1i32.wrapping_neg();
    }
    if opcode <= 254 && ((unsafe { _PyOpcode_Deopt[opcode as usize] }) as i32) != opcode {
        // Specialized instructions are not supported.
        return 1i32.wrapping_neg();
    }
    let popped: i32 = unsafe { _PyOpcode_num_popped(opcode, oparg) };
    let pushed: i32 = unsafe { _PyOpcode_num_pushed(opcode, oparg) };
    if popped < 0 || pushed < 0 {
        return 1i32.wrapping_neg();
    }
    if IS_BLOCK_PUSH_OPCODE!(opcode) && jump == 0 {
        unsafe {
            (*(effects as *mut __SlateRecord_flowgraph_c_764)).net = 0;
        }
        return 0;
    }
    unsafe {
        (*(effects as *mut __SlateRecord_flowgraph_c_764)).net = pushed.wrapping_sub(popped);
    }
    0
}

fn stackdepth_push(
    sp: &mut *mut *mut _PyCfgBasicblock,
    b: *mut _PyCfgBasicblock,
    depth: i32,
) -> i32 {
    if !((unsafe { (*b).b_startdepth }) < 0 || (unsafe { (*b).b_startdepth }) == depth) {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Invalid CFG, inconsistent stackdepth".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*b).b_startdepth }) < depth && (unsafe { (*b).b_startdepth }) < 100 {
        unsafe {
            (*b).b_startdepth = depth;
        }
        let __v1: *mut *mut _PyCfgBasicblock = unsafe { *sp };
        unsafe {
            *sp = __v1.wrapping_offset(1 as isize);
        }
        unsafe {
            *__v1 = b;
        }
    }
    0
}

/// Find the flow path that needs the largest stack.  We assume that
/// cycles in the flow graph have no net effect on the stack depth.
fn calculate_stackdepth(g: *mut _PyCfgBuilder) -> i32 {
    let mut new_depth: i32 = unsafe { core::mem::zeroed() };
    let mut effects: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_764> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_flowgraph_c_764 =
        core::ptr::addr_of_mut!(effects) as *mut __SlateRecord_flowgraph_c_764;
    let mut instr: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut next: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut depth: i32 = unsafe { core::mem::zeroed() };
    let mut b: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut sp: *mut *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut maxdepth: i32 = unsafe { core::mem::zeroed() };
    let mut stackdepth: i32 = unsafe { core::mem::zeroed() };
    let mut stack: *mut *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut b_1: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut entryblock: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    unsafe {
        entryblock = (*g).g_entryblock;
        b_1 = entryblock;
        while !b_1.is_null() {
            (*b_1).b_startdepth = 2147483647i32.wrapping_neg().wrapping_sub(1);
            b_1 = (*b_1).b_next;
        }
        stack = make_cfg_traversal_stack(entryblock);
        if stack.is_null() {
            return 1i32.wrapping_neg();
        } else {
            '__join_0: {
                stackdepth = 1i32.wrapping_neg();
                maxdepth = 0;
                sp = stack;
                if stackdepth_push(&mut (*core::ptr::addr_of_mut!(sp)), entryblock, 0i32) >= 0 {
                    '__join_15: {
                        '__join_13: {
                            '__join_10: {
                                while sp != stack {
                                    let __v10: *mut *mut _PyCfgBasicblock = sp;
                                    let __v9: *mut *mut _PyCfgBasicblock =
                                        __v10.wrapping_offset(-(1 as isize));
                                    sp = __v9;
                                    b = *__v9;
                                    depth = (*b).b_startdepth;
                                    next = (*b).b_next;
                                    i = 0;
                                    '__join_4: {
                                        loop {
                                            if i < (*b).b_iused {
                                                instr = unsafe { (*b).b_instr }
                                                    .wrapping_offset(i as isize);
                                                if get_stack_effects(
                                                    (*instr).i_opcode,
                                                    (*instr).i_oparg,
                                                    0i32,
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_4
                                                    )),
                                                ) < 0
                                                {
                                                    break '__join_15;
                                                } else {
                                                    new_depth =
                                                        depth.wrapping_add((*__slate_slot_4).net);
                                                    if new_depth < 0 {
                                                        break '__join_13;
                                                    } else {
                                                        maxdepth = Py_MAX!(maxdepth, depth);
                                                        if HAS_TARGET!(unsafe { (*instr).i_opcode })
                                                            && (*instr).i_opcode != 68
                                                        {
                                                            if get_stack_effects(
                                                                (*instr).i_opcode,
                                                                (*instr).i_oparg,
                                                                1i32,
                                                                &mut (*core::ptr::addr_of_mut!(
                                                                    *__slate_slot_4
                                                                )),
                                                            ) < 0
                                                            {
                                                                break '__join_10;
                                                            } else {
                                                                let target_depth: i32 = depth
                                                                    .wrapping_add(
                                                                        (*__slate_slot_4).net,
                                                                    );
                                                                // invalid code or bug in stackdepth()
                                                                maxdepth = Py_MAX!(maxdepth, depth);
                                                                if stackdepth_push(
                                                                    &mut (*core::ptr::addr_of_mut!(
                                                                        sp
                                                                    )),
                                                                    (*instr).i_target,
                                                                    target_depth,
                                                                ) < 0
                                                                {
                                                                    break '__join_0;
                                                                }
                                                            }
                                                        }
                                                        depth = new_depth;
                                                        if IS_UNCONDITIONAL_JUMP_OPCODE!(unsafe {
                                                            (*instr).i_opcode
                                                        }) || IS_SCOPE_EXIT_OPCODE!(unsafe {
                                                            (*instr).i_opcode
                                                        }) {
                                                            break;
                                                        } else {
                                                            i = i.wrapping_add(1);
                                                        }
                                                    }
                                                }
                                            } else {
                                                break '__join_4;
                                            }
                                        }
                                        // remaining code is dead
                                        next = core::ptr::null_mut::<_PyCfgBasicblock>();
                                    }
                                    if !next.is_null() {
                                        if stackdepth_push(
                                            &mut (*core::ptr::addr_of_mut!(sp)),
                                            next,
                                            depth,
                                        ) < 0
                                        {
                                            break '__join_0;
                                        }
                                    }
                                }
                                stackdepth = maxdepth;
                                break '__join_0;
                            }
                            PyErr_Format(
                                PyExc_SystemError,
                                c"Invalid stack effect for opcode=%d, arg=%i".as_ptr(),
                                (*instr).i_opcode,
                                (*instr).i_oparg,
                            );
                            break '__join_0;
                        }
                        PyErr_Format(PyExc_ValueError, c"Invalid CFG, stack underflow".as_ptr());
                        break '__join_0;
                    }
                    PyErr_Format(
                        PyExc_SystemError,
                        c"Invalid stack effect for opcode=%d, arg=%i".as_ptr(),
                        (*instr).i_opcode,
                        (*instr).i_oparg,
                    );
                }
            }
            PyMem_Free(stack as *mut ());
            return stackdepth;
        }
    }
}

fn label_exception_targets(entryblock: *mut _PyCfgBasicblock) -> i32 {
    let mut copy: *mut _PyCfgExceptStack = unsafe { core::mem::zeroed() };
    let mut copy_1: *mut _PyCfgExceptStack = unsafe { core::mem::zeroed() };
    let mut instr: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut last_yield_except_depth: i32 = unsafe { core::mem::zeroed() };
    let mut b: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut handler: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut todo: *mut *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut except_stack: *mut _PyCfgExceptStack = unsafe { core::mem::zeroed() };
    let mut todo_stack: *mut *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    unsafe {
        todo_stack = make_cfg_traversal_stack(entryblock);
        if todo_stack.is_null() {
            return 1i32.wrapping_neg();
        } else {
            except_stack = make_except_stack();
            if except_stack.is_null() {
                PyMem_Free(todo_stack as *mut ());
                PyErr_NoMemory();
                return 1i32.wrapping_neg();
            } else {
                (*except_stack).depth = 0;
                *todo_stack.wrapping_offset(0 as isize) = entryblock;
                (*entryblock).__slate_bits_0.__set_b_visited(1u32);
                (*entryblock).b_exceptstack = except_stack;
                todo = todo_stack.wrapping_offset(1 as isize);
                handler = core::ptr::null_mut::<_PyCfgBasicblock>();
                '__join_0: {
                    while todo > todo_stack {
                        let __v18: *mut *mut _PyCfgBasicblock = todo;
                        todo = __v18.wrapping_offset(-(1 as isize));
                        b = *todo.wrapping_offset(0 as isize);
                        except_stack = (*b).b_exceptstack;
                        (*b).b_exceptstack = core::ptr::null_mut::<_PyCfgExceptStack>();
                        handler = except_stack_top(except_stack);
                        last_yield_except_depth = 1i32.wrapping_neg();
                        i = 0;
                        while i < (*b).b_iused {
                            instr = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
                            if is_block_push(instr) != 0 {
                                if ((*(*instr).i_target).__slate_bits_0.__get_b_visited() as i32)
                                    == 0
                                {
                                    copy = copy_except_stack(except_stack);
                                    if copy.is_null() {
                                        break '__join_0;
                                    } else {
                                        (*(*instr).i_target).b_exceptstack = copy;
                                        *todo.wrapping_offset(0 as isize) = (*instr).i_target;
                                        (*(*instr).i_target).__slate_bits_0.__set_b_visited(1u32);
                                        let __v5: *mut *mut _PyCfgBasicblock = todo;
                                        todo = __v5.wrapping_offset(1 as isize);
                                    }
                                }
                                handler = push_except_block(except_stack, instr);
                            } else {
                                if (*instr).i_opcode == 262 {
                                    handler = pop_except_block(except_stack);
                                    INSTR_SET_OP0!(instr, 27i32);
                                } else {
                                    if is_jump(instr) != 0 {
                                        (*instr).i_except = handler;
                                        if ((*(*instr).i_target).__slate_bits_0.__get_b_visited()
                                            as i32)
                                            == 0
                                        {
                                            if BB_HAS_FALLTHROUGH!(b) {
                                                copy_1 = copy_except_stack(except_stack);
                                                if copy_1.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    (*(*instr).i_target).b_exceptstack = copy_1;
                                                }
                                            } else {
                                                (*(*instr).i_target).b_exceptstack = except_stack;
                                                except_stack =
                                                    core::ptr::null_mut::<_PyCfgExceptStack>();
                                            }
                                            *todo.wrapping_offset(0 as isize) = (*instr).i_target;
                                            (*(*instr).i_target)
                                                .__slate_bits_0
                                                .__set_b_visited(1u32);
                                            let __v8: *mut *mut _PyCfgBasicblock = todo;
                                            todo = __v8.wrapping_offset(1 as isize);
                                        }
                                    } else {
                                        if (*instr).i_opcode == 120 {
                                            (*instr).i_except = handler;
                                            last_yield_except_depth = (*except_stack).depth;
                                        } else {
                                            if (*instr).i_opcode == 128 {
                                                (*instr).i_except = handler;
                                                if (*instr).i_oparg != 0 {
                                                    if last_yield_except_depth == 1 {
                                                        (*instr).i_oparg = (*instr).i_oparg | 4;
                                                    }
                                                    last_yield_except_depth = 1i32.wrapping_neg();
                                                }
                                            } else {
                                                (*instr).i_except = handler;
                                            }
                                        }
                                    }
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        if BB_HAS_FALLTHROUGH!(b)
                            && ((*(*b).b_next).__slate_bits_0.__get_b_visited() as i32) == 0
                        {
                            (*(*b).b_next).b_exceptstack = except_stack;
                            *todo.wrapping_offset(0 as isize) = (*b).b_next;
                            (*(*b).b_next).__slate_bits_0.__set_b_visited(1u32);
                            let __v1: *mut *mut _PyCfgBasicblock = todo;
                            todo = __v1.wrapping_offset(1 as isize);
                        } else {
                            if !except_stack.is_null() {
                                PyMem_Free(except_stack as *mut ());
                            }
                        }
                    }
                    PyMem_Free(todo_stack as *mut ());
                    return 0;
                }
                PyMem_Free(todo_stack as *mut ());
                PyMem_Free(except_stack as *mut ());
                return 1i32.wrapping_neg();
            }
        }
    }
}

// CFG optimizations
fn remove_unreachable(entryblock: *mut _PyCfgBasicblock) -> i32 {
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            unsafe {
                (*b).b_predecessors = 0;
            }
            b = unsafe { (*b).b_next };
        }
    }
    let stack: *mut *mut _PyCfgBasicblock = make_cfg_traversal_stack(entryblock);
    if stack.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut sp: *mut *mut _PyCfgBasicblock = stack;
    unsafe {
        (*entryblock).b_predecessors = 1;
    }
    let __v1: *mut *mut _PyCfgBasicblock = sp;
    sp = __v1.wrapping_offset(1 as isize);
    unsafe {
        *__v1 = entryblock;
    }
    unsafe {
        (*entryblock).__slate_bits_0.__set_b_visited(1u32);
    }
    while sp > stack {
        let __v3: *mut *mut _PyCfgBasicblock = sp;
        let __v4: *mut *mut _PyCfgBasicblock = __v3.wrapping_offset(-(1 as isize));
        sp = __v4;
        let b: *mut _PyCfgBasicblock = unsafe { *__v4 };
        let __v5: bool = !unsafe { (*b).b_next }.is_null() && BB_HAS_FALLTHROUGH!(b);
        if __v5 {
            if ((unsafe { (*(*b).b_next).__slate_bits_0.__get_b_visited() }) as i32) == 0 {
                let __v6: *mut *mut _PyCfgBasicblock = sp;
                sp = __v6.wrapping_offset(1 as isize);
                unsafe {
                    *__v6 = (*b).b_next;
                }
                unsafe {
                    (*(*b).b_next).__slate_bits_0.__set_b_visited(1u32);
                }
            }
            let __v8: *mut _PyCfgBasicblock = unsafe { (*b).b_next };
            let __v9: i32 = unsafe { (*__v8).b_predecessors };
            unsafe {
                (*__v8).b_predecessors = __v9.wrapping_add(1);
            }
        }
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            let __v11: bool = is_jump(instr) != 0 || is_block_push(instr) != 0;
            if __v11 {
                let target: *mut _PyCfgBasicblock = unsafe { (*instr).i_target };
                if ((unsafe { (*target).__slate_bits_0.__get_b_visited() }) as i32) == 0 {
                    let __v12: *mut *mut _PyCfgBasicblock = sp;
                    sp = __v12.wrapping_offset(1 as isize);
                    unsafe {
                        *__v12 = target;
                    }
                    unsafe {
                        (*target).__slate_bits_0.__set_b_visited(1u32);
                    }
                }
                let __v15: i32 = unsafe { (*target).b_predecessors };
                unsafe {
                    (*target).b_predecessors = __v15.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    unsafe { PyMem_Free(stack as *mut ()) };
    // Delete unreachable instructions
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !b.is_null() {
        if (unsafe { (*b).b_predecessors }) == 0 {
            unsafe {
                (*b).b_iused = 0;
            }
            unsafe {
                (*b).__slate_bits_0.__set_b_except_handler(0u32);
            }
        }
        b = unsafe { (*b).b_next };
    }
    0
}

fn basicblock_remove_redundant_nops(bb: *mut _PyCfgBasicblock) -> i32 {
    // Remove NOPs when legal to do so.
    let mut dest: i32 = 0;
    let mut prev_lineno: i32 = 1i32.wrapping_neg();
    let mut src: i32 = 0;
    while src < unsafe { (*bb).b_iused } {
        '__slate_continue_0: {
            let lineno: i32 = unsafe {
                (*unsafe { (*bb).b_instr }.wrapping_offset(src as isize))
                    .i_loc
                    .lineno
            };
            if (unsafe { (*unsafe { (*bb).b_instr }.wrapping_offset(src as isize)).i_opcode }) == 27
            {
                // Eliminate no-op if it doesn't have a line number
                if lineno < 0 {
                    break '__slate_continue_0;
                }
                // or, if the previous instruction had the same line number.
                if prev_lineno == lineno {
                    break '__slate_continue_0;
                }
                // or, if the next instruction has same line number or no line number
                if src < unsafe { (*bb).b_iused }.wrapping_sub(1) {
                    let next_lineno: i32 = unsafe {
                        (*unsafe { (*bb).b_instr }.wrapping_offset(src.wrapping_add(1) as isize))
                            .i_loc
                            .lineno
                    };
                    if next_lineno == lineno {
                        break '__slate_continue_0;
                    }
                    if next_lineno < 0 {
                        unsafe {
                            (*unsafe { (*bb).b_instr }
                                .wrapping_offset(src.wrapping_add(1) as isize))
                            .i_loc =
                                (*unsafe { (*bb).b_instr }.wrapping_offset(src as isize)).i_loc;
                        }
                        break '__slate_continue_0;
                    }
                } else {
                    let next: *mut _PyCfgBasicblock = next_nonempty_block(unsafe { (*bb).b_next });
                    // or if last instruction in BB and next BB has same line number
                    if !next.is_null() {
                        let mut next_loc: __SlateRecord_pycore_symtable_h_51 =
                            unsafe { NO_LOCATION };
                        let mut next_i: i32 = 0;
                        while next_i < unsafe { (*next).b_iused } {
                            let instr: *mut _PyCfgInstruction =
                                unsafe { (*next).b_instr }.wrapping_offset(next_i as isize);
                            if (unsafe { (*instr).i_opcode }) == 27
                                && (unsafe { (*instr).i_loc.lineno }) < 0
                            {
                                // Skip over NOPs without a location, they will be removed
                            } else {
                                next_loc = unsafe { (*instr).i_loc };
                                break;
                            }
                            next_i = next_i.wrapping_add(1);
                        }
                        if lineno == next_loc.lineno {
                            break '__slate_continue_0;
                        }
                    }
                }
            }
            if dest != src {
                unsafe {
                    *unsafe { (*bb).b_instr }.wrapping_offset(dest as isize) =
                        *unsafe { (*bb).b_instr }.wrapping_offset(src as isize);
                }
            }
            dest = dest.wrapping_add(1);
            prev_lineno = lineno;
        }
        src = src.wrapping_add(1);
    }
    let num_removed: i32 = unsafe { (*bb).b_iused }.wrapping_sub(dest);
    unsafe {
        (*bb).b_iused = dest;
    }
    num_removed
}

fn remove_redundant_nops(g: *mut _PyCfgBuilder) -> i32 {
    let mut changes: i32 = 0;
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        let change: i32 = basicblock_remove_redundant_nops(b);
        RETURN_IF_ERROR__2!(change);
        changes = changes.wrapping_add(change);
        b = unsafe { (*b).b_next };
    }
    changes
}

fn remove_redundant_nops_and_pairs(entryblock: *mut _PyCfgBasicblock) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let mut done: bool = false;
    '__slate_break_9: while !done {
        done = true;
        let mut prev_instr: *mut _PyCfgInstruction = core::ptr::null_mut::<_PyCfgInstruction>();
        let mut instr: *mut _PyCfgInstruction = core::ptr::null_mut::<_PyCfgInstruction>();
        let mut b: *mut _PyCfgBasicblock = entryblock;
        '__slate_break_8: while !b.is_null() {
            RETURN_IF_ERROR__2!(basicblock_remove_redundant_nops(b));
            if (unsafe { (*b).b_label.id })
                != unsafe {
                    (*{
                        __slate_compound_0 = __SlateRecord_pycore_instruction_sequence_h_55 {
                            id: 1i32.wrapping_neg(),
                        };
                        core::ptr::addr_of_mut!(__slate_compound_0)
                    })
                    .id
                }
            {
                // this block is a jump target, forget instr
                instr = core::ptr::null_mut::<_PyCfgInstruction>();
            }
            let mut i: i32 = 0;
            while i < unsafe { (*b).b_iused } {
                prev_instr = instr;
                instr = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
                let prev_opcode: i32 = if !prev_instr.is_null() {
                    unsafe { (*prev_instr).i_opcode }
                } else {
                    0
                };
                let prev_oparg: i32 = if !prev_instr.is_null() {
                    unsafe { (*prev_instr).i_oparg }
                } else {
                    0
                };
                let opcode: i32 = unsafe { (*instr).i_opcode };
                let mut is_redundant_pair: bool = false;
                if opcode == 31 {
                    if prev_opcode == 82 || prev_opcode == 94 {
                        is_redundant_pair = true;
                    } else {
                        if prev_opcode == 59 && prev_oparg == 1 {
                            is_redundant_pair = true;
                        }
                    }
                }
                if is_redundant_pair {
                    INSTR_SET_OP0!(prev_instr, 27i32);
                    INSTR_SET_OP0!(instr, 27i32);
                    done = false;
                }
                i = i.wrapping_add(1);
            }
            let __v6: bool = !instr.is_null() && is_jump(instr) != 0;
            let __v7: bool = __v6 || !BB_HAS_FALLTHROUGH!(b);
            if __v7 {
                instr = core::ptr::null_mut::<_PyCfgInstruction>();
            }
            b = unsafe { (*b).b_next };
        }
    }
    0
}

fn remove_redundant_jumps(g: *mut _PyCfgBuilder) -> i32 {
    // If a non-empty block ends with a jump instruction, check if the next
    // non-empty block reached through normal flow control is the target
    // of that jump. If it is, then the jump instruction is redundant and
    // can be deleted.
    //
    // Return the number of changes applied, or -1 on error.
    let mut changes: i32 = 0;
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        let last: *mut _PyCfgInstruction = basicblock_last_instr(b as *const _PyCfgBasicblock);
        if !last.is_null() {
            if IS_UNCONDITIONAL_JUMP_OPCODE!(unsafe { (*last).i_opcode }) {
                let jump_target: *mut _PyCfgBasicblock =
                    next_nonempty_block(unsafe { (*last).i_target });
                if jump_target.is_null() {
                    unsafe {
                        PyErr_SetString(PyExc_SystemError, c"jump with NULL target".as_ptr())
                    };
                    return 1i32.wrapping_neg();
                }
                let next: *mut _PyCfgBasicblock = next_nonempty_block(unsafe { (*b).b_next });
                if jump_target == next {
                    changes = changes.wrapping_add(1);
                    INSTR_SET_OP0!(last, 27i32);
                }
            }
        }
        b = unsafe { (*b).b_next };
    }
    changes
}

fn basicblock_has_no_lineno(b: *mut _PyCfgBasicblock) -> bool {
    let mut i: i32 = 0;
    while i < unsafe { (*b).b_iused } {
        if (unsafe {
            (*unsafe { (*b).b_instr }.wrapping_offset(i as isize))
                .i_loc
                .lineno
        }) >= 0
        {
            return false;
        }
        i = i.wrapping_add(1);
    }
    true
}

// Maximum size of basic block that should be copied in optimizer
/// If this block ends with an unconditional jump to a small exit block or
/// a block that has no line numbers (and no fallthrough), then
/// remove the jump and extend this block with the target.
/// Returns 1 if extended, 0 if no change, and -1 on error.
fn basicblock_inline_small_or_no_lineno_blocks(bb: *mut _PyCfgBasicblock) -> i32 {
    let mut last: *mut _PyCfgInstruction = basicblock_last_instr(bb as *const _PyCfgBasicblock);
    if last.is_null() {
        return 0;
    }
    if !IS_UNCONDITIONAL_JUMP_OPCODE!(unsafe { (*last).i_opcode }) {
        return 0;
    }
    let target: *mut _PyCfgBasicblock = unsafe { (*last).i_target };
    let small_exit_block: bool = basicblock_exits_scope(target as *const _PyCfgBasicblock) != 0
        && (unsafe { (*target).b_iused }) <= 4;
    let mut no_lineno_no_fallthrough: bool = false;
    no_lineno_no_fallthrough = basicblock_has_no_lineno(target) && !BB_HAS_FALLTHROUGH!(target);
    if small_exit_block || no_lineno_no_fallthrough {
        let removed_jump_opcode: i32 = unsafe { (*last).i_opcode };
        INSTR_SET_OP0!(last, 27i32);
        RETURN_IF_ERROR__2!(basicblock_append_instructions(bb, target));
        if no_lineno_no_fallthrough {
            last = basicblock_last_instr(bb as *const _PyCfgBasicblock);
            if IS_UNCONDITIONAL_JUMP_OPCODE!(unsafe { (*last).i_opcode })
                && removed_jump_opcode == 257
            {
                // Make sure we don't lose eval breaker checks
                unsafe {
                    (*last).i_opcode = 257;
                }
            }
        }
        let __v3: i32 = unsafe { (*target).b_predecessors };
        unsafe {
            (*target).b_predecessors = __v3.wrapping_sub(1);
        }
        return 1;
    }
    0
}

fn inline_small_or_no_lineno_blocks(entryblock: *mut _PyCfgBasicblock) -> i32 {
    let mut changes: bool = false;
    loop {
        changes = false;
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            let res: i32 = basicblock_inline_small_or_no_lineno_blocks(b);
            RETURN_IF_ERROR__2!(res);
            if res != 0 {
                changes = true;
            }
            b = unsafe { (*b).b_next };
        }
        if !changes {
            break;
        }
    }
    // every change removes a jump, ensuring convergence
    changes as i32
}

/// Attempt to eliminate jumps to jumps by updating inst to jump to
///
/// target->i_target using the provided opcode. Return whether or not the
///
/// optimization was successful.
fn jump_thread(
    bb: *mut _PyCfgBasicblock,
    inst: *mut _PyCfgInstruction,
    target: *mut _PyCfgInstruction,
    opcode: i32,
) -> bool {
    // bpo-45773: If inst->i_target == target->i_target, then nothing actually
    //
    // changes (and we fall into an infinite loop):
    if (unsafe { (*inst).i_target }) != unsafe { (*target).i_target } {
        // Change inst to NOP and append a jump to target->i_target. The
        // NOP will be removed later if it's not needed for the lineno.
        INSTR_SET_OP0!(inst, 27i32);
        RETURN_IF_ERROR__6!(basicblock_add_jump(
            bb,
            opcode,
            unsafe { (*target).i_target },
            unsafe { (*target).i_loc }
        ));
        return true;
    }
    false
}

fn loads_const(opcode: i32) -> i32 {
    (OPCODE_HAS_CONST!(opcode) != 0 || opcode == 94) as i32
}

/// Returns new reference
fn get_const_value(opcode: i32, oparg: i32, co_consts: *mut _object) -> *mut _object {
    let mut constant: *mut _object = core::ptr::null_mut::<_object>();
    if opcode == 82 {
        let n: i64 = PyList_GET_SIZE!(co_consts);
        if oparg < 0 || (oparg as i64) >= n {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"LOAD_CONST index %d is out of range for consts (len=%zd)".as_ptr(),
                    oparg,
                    n,
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        constant = PyList_GET_ITEM!(co_consts, oparg);
    }
    if opcode == 94 {
        return unsafe { PyLong_FromLong(oparg as i64) };
    }
    if constant.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_SystemError,
                c"Internal error: failed to get value of a constant".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(constant)
}

/// Steals a reference to newconst.
fn add_const(
    mut newconst: *mut _object,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    if (unsafe { _PyCompile_ConstCacheMergeOne(const_cache, core::ptr::addr_of_mut!(newconst)) })
        < 0
    {
        Py_DECREF!(newconst);
        return 1i32.wrapping_neg();
    }
    let entry: *mut __SlateRecord_pycore_hashtable_h_35 =
        _Py_hashtable_get_entry(consts_index, newconst as *const ());
    if !entry.is_null() {
        Py_DECREF!(newconst);
        return ((unsafe { (*entry).value }) as u64) as i32;
    }
    let index: i64 = PyList_GET_SIZE!(consts);
    if (index as u64) >= 2147483646u64 {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"too many constants".as_ptr()) };
        Py_DECREF!(newconst);
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyList_Append(consts, newconst) }) != 0 {
        Py_DECREF!(newconst);
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        _Py_hashtable_set(
            consts_index,
            newconst as *const (),
            (index as u64) as *mut (),
        )
    }) < 0
    {
        unsafe {
            PyList_SetSlice(
                consts,
                index,
                index.wrapping_add(1),
                core::ptr::null_mut::<_object>(),
            )
        };
        Py_DECREF!(newconst);
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(newconst);
    index as i32
}

/// Traverse the instructions of the basic block backwards from index "start", skipping over NOPs.
/// Try to collect "size" number of consecutive instructions that load constants into the array "instrs".
/// Caller must make sure that length of "instrs" is sufficient to fit in at least "size" instructions.
///
/// Return boolean indicating whether "size" such instructions were found.
fn get_const_loading_instrs(
    bb: *mut _PyCfgBasicblock,
    mut start: i32,
    instrs: *mut *mut _PyCfgInstruction,
    mut size: i32,
) -> bool {
    while start >= 0 && size > 0 {
        let instr: *mut _PyCfgInstruction =
            unsafe { (*bb).b_instr }.wrapping_offset(start as isize);
        if (unsafe { (*instr).i_opcode }) != 27 {
            if loads_const(unsafe { (*instr).i_opcode }) == 0 {
                return false;
            }
            let __v2: i32 = size.wrapping_sub(1);
            size = __v2;
            unsafe {
                *instrs.wrapping_offset(__v2 as isize) = instr;
            }
        }
        start = start.wrapping_sub(1);
    }
    size == 0
}

/// Change every instruction in "instrs" NOP and set its location to NO_LOCATION.
/// Caller must make sure "instrs" has at least "size" elements.
fn nop_out(instrs: *mut *mut _PyCfgInstruction, size: i32) {
    for i in 0..size {
        let instr: *mut _PyCfgInstruction = unsafe { *instrs.wrapping_offset(i as isize) };
        INSTR_SET_OP0!(instr, 27i32);
        INSTR_SET_LOC!(instr, unsafe { NO_LOCATION });
    }
}

/// Does not steal reference to "newconst".
/// Return 1 if changed instruction to LOAD_SMALL_INT.
/// Return 0 if could not change instruction to LOAD_SMALL_INT.
/// Return -1 on error.
fn maybe_instr_make_load_smallint(
    instr: *mut _PyCfgInstruction,
    newconst: *mut _object,
    consts: *mut _object,
    const_cache: *mut _object,
) -> i32 {
    if PyLong_CheckExact!(newconst) != 0 {
        let mut overflow: i32 = 0;
        let val: i64 =
            unsafe { PyLong_AsLongAndOverflow(newconst, core::ptr::addr_of_mut!(overflow)) };
        let __v0: bool =
            val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return 1i32.wrapping_neg();
        }
        if overflow == 0 && _PY_IS_SMALL_INT!(val) && 0 <= val && val <= 255 {
            INSTR_SET_OP1!(instr, 94i32, val as i32);
            return 1;
        }
    }
    0
}

/// Steals reference to "newconst"
fn instr_make_load_const(
    instr: *mut _PyCfgInstruction,
    newconst: *mut _object,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let res: i32 = maybe_instr_make_load_smallint(instr, newconst, consts, const_cache);
    if res < 0 {
        Py_DECREF!(newconst);
        return 1i32.wrapping_neg();
    }
    if res > 0 {
        return 0;
    }
    let oparg: i32 = add_const(newconst, consts, const_cache, consts_index);
    RETURN_IF_ERROR__2!(oparg);
    INSTR_SET_OP1!(instr, 82i32, oparg);
    0
}

/// Replace LOAD_CONST c1, LOAD_CONST c2 ... LOAD_CONST cn, BUILD_TUPLE n
/// with    LOAD_CONST (c1, c2, ... cn).
/// The consts table must still be in list form so that the
/// new constant (c1, c2, ... cn) can be appended.
/// Called with codestr pointing to the first LOAD_CONST.
fn fold_tuple_of_constants(
    bb: *mut _PyCfgBasicblock,
    i: i32,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    // Pre-conditions
    let instr: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
    let seq_size: i32 = unsafe { (*instr).i_oparg };
    if seq_size > 30 {
        return 0;
    }
    let mut const_instrs: __SlateAlign16<[*mut _PyCfgInstruction; 30]> =
        __SlateAlign16([0 as *mut _PyCfgInstruction; 30]);
    if !get_const_loading_instrs(
        bb,
        i.wrapping_sub(1),
        const_instrs.0.as_mut_ptr() as *mut *mut _PyCfgInstruction,
        seq_size,
    ) {
        // not a const sequence
        return 0;
    }
    let const_tuple: *mut _object = unsafe { PyTuple_New(seq_size as i64) };
    if const_tuple.is_null() {
        return 1i32.wrapping_neg();
    }
    for i in 0..seq_size {
        let inst: *mut _PyCfgInstruction = const_instrs.0[i as usize];
        let element: *mut _object = get_const_value(
            unsafe { (*inst).i_opcode },
            unsafe { (*inst).i_oparg },
            consts,
        );
        if element.is_null() {
            Py_DECREF!(const_tuple);
            return 1i32.wrapping_neg();
        }
        PyTuple_SET_ITEM!(const_tuple, i, element);
    }
    nop_out(
        const_instrs.0.as_mut_ptr() as *mut *mut _PyCfgInstruction,
        seq_size,
    );
    instr_make_load_const(instr, const_tuple, consts, const_cache, consts_index)
}

/// Replace:
///  BUILD_LIST/BUILD_SET 0
///  LOAD_CONST c1
///  LIST_APPEND/SET_ADD 1
///  LOAD_CONST c2
///  LIST_APPEND/SET_ADD 1
///  ...
///  LOAD_CONST cN
///  LIST_APPEND/SET_ADD 1
///  [CALL_INTRINSIC_1 INTRINSIC_LIST_TO_TUPLE]   <-- optional
/// with:
///  LOAD_CONST (c1, c2, ... cN)
/// The instruction at `i` is either the LIST_TO_TUPLE intrinsic (so the
/// immediately preceding non-NOP instruction is expected to be a
/// LIST_APPEND, and only the BUILD_LIST/LIST_APPEND form is considered),
/// or the trailing LIST_APPEND or SET_ADD itself, in which case the
/// matching BUILD_LIST/BUILD_SET start is selected from its opcode, and
/// for sets the result is wrapped in a frozenset.
fn fold_constant_seq_into_load_const(
    bb: *mut _PyCfgBasicblock,
    i: i32,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let target: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
    let expected_append: bool = (unsafe { (*target).i_opcode }) == 53;
    let append_op: i32 = if expected_append {
        78
    } else {
        unsafe { (*target).i_opcode }
    };
    let build_op: i32 = if append_op == 78 { 46 } else { 48 };
    let mut consts_found: i32 = 0;
    // Walking backward from `i`, we expect LIST_APPEND/SET_ADD and
    // LOAD_CONST to alternate. If `i` is the trailing LIST_TO_TUPLE
    // intrinsic, the next instruction back is an APPEND. If `i` is the
    // trailing APPEND itself, the next instruction back is a LOAD_CONST.
    let mut expect_append: bool = expected_append;
    let mut pos: i32 = i.wrapping_sub(1);
    while pos >= 0 {
        let mut instr: *mut _PyCfgInstruction =
            unsafe { (*bb).b_instr }.wrapping_offset(pos as isize);
        let opcode: i32 = unsafe { (*instr).i_opcode };
        let oparg: i32 = unsafe { (*instr).i_oparg };
        if opcode != 27 {
            if opcode == build_op && oparg == 0 {
                if !expect_append {
                    // Not a sequence start.
                    return 0;
                }
                // Sequence start, we are done.
                let mut newconst: *mut _object = unsafe { PyTuple_New(consts_found as i64) };
                if newconst.is_null() {
                    return 1i32.wrapping_neg();
                }
                let newpos_start: i32 = if expected_append {
                    i.wrapping_sub(1)
                } else {
                    i
                };
                let mut newpos: i32 = newpos_start;
                while newpos >= pos {
                    instr = unsafe { (*bb).b_instr }.wrapping_offset(newpos as isize);
                    if (unsafe { (*instr).i_opcode }) != 27 {
                        if loads_const(unsafe { (*instr).i_opcode }) != 0 {
                            let constant: *mut _object = get_const_value(
                                unsafe { (*instr).i_opcode },
                                unsafe { (*instr).i_oparg },
                                consts,
                            );
                            if constant.is_null() {
                                Py_DECREF!(newconst);
                                return 1i32.wrapping_neg();
                            }
                            let __v3: i32 = consts_found.wrapping_sub(1);
                            consts_found = __v3;
                            PyTuple_SET_ITEM(
                                _PyObject_CAST!(newconst),
                                __v3 as i64,
                                _PyObject_CAST!(constant),
                            );
                        }
                        nop_out(core::ptr::addr_of_mut!(instr), 1i32);
                    }
                    newpos = newpos.wrapping_sub(1);
                }
                if build_op == 48 {
                    let frozen: *mut _object = unsafe { PyFrozenSet_New(newconst) };
                    Py_DECREF!(newconst);
                    if frozen.is_null() {
                        return 1i32.wrapping_neg();
                    }
                    newconst = frozen;
                }
                return instr_make_load_const(target, newconst, consts, const_cache, consts_index);
            }
            if expect_append {
                if opcode != append_op || oparg != 1 {
                    return 0;
                }
            } else {
                if loads_const(opcode) == 0 {
                    return 0;
                }
                consts_found = consts_found.wrapping_add(1);
            }
            expect_append = !expect_append;
        }
        pos = pos.wrapping_sub(1);
    }
    // Did not find sequence start.
    0
}

/// Optimize lists and sets for:
///     1. "for" loop, comprehension or "in"/"not in" tests:
///            Change literal list or set of constants into constant
///            tuple or frozenset respectively. Change list of
///            non-constants into tuple.
///     2. Constant literal lists/set with length >= MIN_CONST_SEQUENCE_SIZE:
///            Replace LOAD_CONST c1, LOAD_CONST c2 ... LOAD_CONST cN, BUILD_LIST N
///            with BUILD_LIST 0, LOAD_CONST (c1, c2, ... cN), LIST_EXTEND 1,
///            or BUILD_SET & SET_UPDATE respectively.
fn optimize_lists_and_sets(
    bb: *mut _PyCfgBasicblock,
    i: i32,
    nextop: i32,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let instr: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
    let contains_or_iter: bool = nextop == 16 || nextop == 57;
    let seq_size: i32 = unsafe { (*instr).i_oparg };
    if seq_size > 30 || seq_size < 3 && !contains_or_iter {
        return 0;
    }
    let mut const_instrs: __SlateAlign16<[*mut _PyCfgInstruction; 30]> =
        __SlateAlign16([0 as *mut _PyCfgInstruction; 30]);
    if !get_const_loading_instrs(
        bb,
        i.wrapping_sub(1),
        const_instrs.0.as_mut_ptr() as *mut *mut _PyCfgInstruction,
        seq_size,
    ) {
        // not a const sequence
        if contains_or_iter && (unsafe { (*instr).i_opcode }) == 46 {
            // iterate over a tuple instead of list
            INSTR_SET_OP1!(instr, 51i32, unsafe { (*instr).i_oparg });
        }
        return 0;
    }
    let mut const_result: *mut _object = unsafe { PyTuple_New(seq_size as i64) };
    if const_result.is_null() {
        return 1i32.wrapping_neg();
    }
    for i in 0..seq_size {
        let inst: *mut _PyCfgInstruction = const_instrs.0[i as usize];
        let element: *mut _object = get_const_value(
            unsafe { (*inst).i_opcode },
            unsafe { (*inst).i_oparg },
            consts,
        );
        if element.is_null() {
            Py_DECREF!(const_result);
            return 1i32.wrapping_neg();
        }
        PyTuple_SET_ITEM!(const_result, i, element);
    }
    if (unsafe { (*instr).i_opcode }) == 48 {
        let frozenset: *mut _object = unsafe { PyFrozenSet_New(const_result) };
        if frozenset.is_null() {
            Py_DECREF!(const_result);
            return 1i32.wrapping_neg();
        }
        Py_SETREF!(const_result, frozenset);
    }
    let index: i32 = add_const(const_result, consts, const_cache, consts_index);
    RETURN_IF_ERROR__2!(index);
    nop_out(
        const_instrs.0.as_mut_ptr() as *mut *mut _PyCfgInstruction,
        seq_size,
    );
    if contains_or_iter {
        INSTR_SET_OP1!(instr, 82i32, index);
    } else {
        INSTR_SET_LOC!(
            unsafe { (*bb).b_instr }.wrapping_offset(i.wrapping_sub(2) as isize),
            unsafe { (*instr).i_loc }
        );
        INSTR_SET_OP1!(
            unsafe { (*bb).b_instr }.wrapping_offset(i.wrapping_sub(2) as isize),
            unsafe { (*instr).i_opcode },
            0i32
        );
        INSTR_SET_OP1!(
            unsafe { (*bb).b_instr }.wrapping_offset(i.wrapping_sub(1) as isize),
            82i32,
            index
        );
        INSTR_SET_OP1!(
            unsafe { (*bb).b_instr }.wrapping_offset(i as isize),
            if (unsafe { (*instr).i_opcode }) == 46 {
                79i32
            } else {
                109i32
            },
            1i32
        );
    }
    0
}

/// Check whether the total number of items in the (possibly nested) collection obj exceeds
/// limit. Return a negative number if it does, and a non-negative number otherwise.
/// Used to avoid creating constants which are slow to hash.
fn const_folding_check_complexity(obj: *mut _object, mut limit: i64) -> i64 {
    if PyTuple_Check!(obj) != 0 {
        let mut i: i64 = 0;
        let __v0: i64 = PyTuple_GET_SIZE!(obj);
        limit = limit.wrapping_sub(__v0);
        loop {
            let __v3: bool = limit >= 0 && i < PyTuple_GET_SIZE!(obj);
            if !__v3 {
                break;
            }
            limit = const_folding_check_complexity(PyTuple_GET_ITEM!(obj, i), limit);
            if limit < 0 {
                return limit;
            }
            i = i.wrapping_add(1);
        }
    }
    limit
}

// bits
// items
// characters
// including nested collections
fn const_folding_safe_multiply(v: *mut _object, w: *mut _object) -> *mut _object {
    let __v0: bool = PyLong_Check!(v) != 0 && PyLong_Check!(w) != 0;
    let __v1: bool = __v0 && !_PyLong_IsZero(v as *const _longobject);
    let __v2: bool = __v1 && !_PyLong_IsZero(w as *const _longobject);
    if __v2 {
        let vbits: i64 = unsafe { _PyLong_NumBits(v) };
        let wbits: i64 = unsafe { _PyLong_NumBits(w) };
        if vbits.wrapping_add(wbits) > 128 {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        let __v3: bool = PyLong_Check!(v) != 0 && PyTuple_Check!(w) != 0;
        if __v3 {
            let size: i64 = PyTuple_GET_SIZE!(w);
            if size != 0 {
                let n: i64 = unsafe { PyLong_AsLong(v) };
                if n < 0 || n > 256 / size {
                    return core::ptr::null_mut::<_object>();
                }
                let __v4: bool = n != 0 && const_folding_check_complexity(w, 1024 / n) < 0;
                if __v4 {
                    return core::ptr::null_mut::<_object>();
                }
            }
        } else {
            let __v5: bool =
                PyLong_Check!(v) != 0 && (PyUnicode_Check!(w) != 0 || PyBytes_Check!(w) != 0);
            if __v5 {
                let mut size: i64 = 0;
                let __v7: i64 = if PyUnicode_Check!(w) != 0 {
                    PyUnicode_GET_LENGTH!(w)
                } else {
                    PyBytes_GET_SIZE!(w)
                };
                size = __v7;
                if size != 0 {
                    let n: i64 = unsafe { PyLong_AsLong(v) };
                    if n < 0 || n > 4096 / size {
                        return core::ptr::null_mut::<_object>();
                    }
                }
            } else {
                let __v8: bool = if PyLong_Check!(w) != 0 {
                    let __v9: bool = PyTuple_Check!(v) != 0 || PyUnicode_Check!(v) != 0;
                    let __v10: bool = __v9 || PyBytes_Check!(v) != 0;
                    __v10
                } else {
                    false
                };
                if __v8 {
                    return const_folding_safe_multiply(w, v);
                }
            }
        }
    }
    unsafe { PyNumber_Multiply(v, w) }
}

fn const_folding_safe_power(v: *mut _object, w: *mut _object) -> *mut _object {
    let __v0: bool = PyLong_Check!(v) != 0 && PyLong_Check!(w) != 0;
    let __v1: bool = __v0 && !_PyLong_IsZero(v as *const _longobject);
    let __v2: bool = __v1 && _PyLong_IsPositive(w as *const _longobject);
    if __v2 {
        let vbits: i64 = unsafe { _PyLong_NumBits(v) };
        let wbits: u64 = unsafe { PyLong_AsSize_t(w) };
        if wbits == (1i32.wrapping_neg() as u64) {
            return core::ptr::null_mut::<_object>();
        }
        if (vbits as u64) > 128 / wbits {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { PyNumber_Power(v, w, core::ptr::addr_of_mut!(_Py_NoneStruct)) }
}

fn const_folding_safe_lshift(v: *mut _object, w: *mut _object) -> *mut _object {
    let __v0: bool = PyLong_Check!(v) != 0 && PyLong_Check!(w) != 0;
    let __v1: bool = __v0 && !_PyLong_IsZero(v as *const _longobject);
    let __v2: bool = __v1 && !_PyLong_IsZero(w as *const _longobject);
    if __v2 {
        let vbits: i64 = unsafe { _PyLong_NumBits(v) };
        let wbits: u64 = unsafe { PyLong_AsSize_t(w) };
        if wbits == (1i32.wrapping_neg() as u64) {
            return core::ptr::null_mut::<_object>();
        }
        if wbits > 128 || (vbits as u64) > 128u64.wrapping_sub(wbits) {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { PyNumber_Lshift(v, w) }
}

fn const_folding_safe_mod(v: *mut _object, w: *mut _object) -> *mut _object {
    let __v0: bool = PyUnicode_Check!(v) != 0 || PyBytes_Check!(v) != 0;
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyNumber_Remainder(v, w) }
}

fn eval_const_binop(left: *mut _object, op: i32, right: *mut _object) -> *mut _object {
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    '__slate_break_0: {
        match op {
            0 => {
                result = unsafe { PyNumber_Add(left, right) };
            }
            10 => {
                result = unsafe { PyNumber_Subtract(left, right) };
            }
            5 => {
                result = const_folding_safe_multiply(left, right);
            }
            11 => {
                result = unsafe { PyNumber_TrueDivide(left, right) };
            }
            2 => {
                result = unsafe { PyNumber_FloorDivide(left, right) };
            }
            6 => {
                result = const_folding_safe_mod(left, right);
            }
            8 => {
                result = const_folding_safe_power(left, right);
            }
            3 => {
                result = const_folding_safe_lshift(left, right);
            }
            9 => {
                result = unsafe { PyNumber_Rshift(left, right) };
            }
            7 => {
                result = unsafe { PyNumber_Or(left, right) };
            }
            12 => {
                result = unsafe { PyNumber_Xor(left, right) };
            }
            1 => {
                result = unsafe { PyNumber_And(left, right) };
            }
            26 => {
                result = unsafe { PyObject_GetItem(left, right) };
            }
            4 => {
                break '__slate_break_0; // No builtin constants implement matrix multiplication
            }
            _ => {
                Py_UNREACHABLE!();
            }
        }
    }
    result
}

fn fold_const_binop(
    bb: *mut _PyCfgBasicblock,
    i: i32,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let binop: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
    let mut operands_instrs: __SlateAlign16<[*mut _PyCfgInstruction; 2]> =
        __SlateAlign16([0 as *mut _PyCfgInstruction; 2]);
    if !get_const_loading_instrs(
        bb,
        i.wrapping_sub(1),
        operands_instrs.0.as_mut_ptr() as *mut *mut _PyCfgInstruction,
        2i32,
    ) {
        // not a const sequence
        return 0;
    }
    let lhs_instr: *mut _PyCfgInstruction = operands_instrs.0[0];
    let lhs: *mut _object = get_const_value(
        unsafe { (*lhs_instr).i_opcode },
        unsafe { (*lhs_instr).i_oparg },
        consts,
    );
    if lhs.is_null() {
        return 1i32.wrapping_neg();
    }
    let rhs_instr: *mut _PyCfgInstruction = operands_instrs.0[1];
    let rhs: *mut _object = get_const_value(
        unsafe { (*rhs_instr).i_opcode },
        unsafe { (*rhs_instr).i_oparg },
        consts,
    );
    if rhs.is_null() {
        Py_DECREF!(lhs);
        return 1i32.wrapping_neg();
    }
    let newconst: *mut _object = eval_const_binop(lhs, unsafe { (*binop).i_oparg }, rhs);
    Py_DECREF!(lhs);
    Py_DECREF!(rhs);
    if newconst.is_null() {
        if (unsafe { PyErr_ExceptionMatches(PyExc_KeyboardInterrupt) }) != 0 {
            return 1i32.wrapping_neg();
        }
        unsafe { PyErr_Clear() };
        return 0;
    }
    nop_out(
        operands_instrs.0.as_mut_ptr() as *mut *mut _PyCfgInstruction,
        2i32,
    );
    instr_make_load_const(binop, newconst, consts, const_cache, consts_index)
}

fn eval_const_unaryop(operand: *mut _object, opcode: i32, oparg: i32) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    match opcode {
        41 => {
            result = unsafe { PyNumber_Negative(operand) };
        }
        40 => {
            if PyBool_Check!(operand) != 0 {
                return core::ptr::null_mut::<_object>();
            }
            // XXX: This should be removed once the ~bool depreciation expires.
            result = unsafe { PyNumber_Invert(operand) };
        }
        42 => {
            let r: i32 = unsafe { PyObject_IsTrue(operand) };
            if r < 0 {
                return core::ptr::null_mut::<_object>();
            }
            result = unsafe { PyBool_FromLong((r == 0) as i64) };
        }
        53 => {
            if oparg != 5 {
                Py_UNREACHABLE!();
            }
            result = unsafe { PyNumber_Positive(operand) };
        }
        _ => {
            Py_UNREACHABLE!();
        }
    }
    result
}

fn fold_const_unaryop(
    bb: *mut _PyCfgBasicblock,
    i: i32,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let unaryop: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
    let mut operand_instr: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    if !get_const_loading_instrs(
        bb,
        i.wrapping_sub(1),
        core::ptr::addr_of_mut!(operand_instr),
        1i32,
    ) {
        // not a const
        return 0;
    }
    let operand: *mut _object = get_const_value(
        unsafe { (*operand_instr).i_opcode },
        unsafe { (*operand_instr).i_oparg },
        consts,
    );
    if operand.is_null() {
        return 1i32.wrapping_neg();
    }
    let newconst: *mut _object =
        eval_const_unaryop(operand, unsafe { (*unaryop).i_opcode }, unsafe {
            (*unaryop).i_oparg
        });
    Py_DECREF!(operand);
    if newconst.is_null() {
        if (unsafe { PyErr_ExceptionMatches(PyExc_KeyboardInterrupt) }) != 0 {
            return 1i32.wrapping_neg();
        }
        unsafe { PyErr_Clear() };
        return 0;
    }
    if (unsafe { (*unaryop).i_opcode }) == 42 {}
    nop_out(core::ptr::addr_of_mut!(operand_instr), 1i32);
    instr_make_load_const(unaryop, newconst, consts, const_cache, consts_index)
}

/// Replace an arbitrary run of SWAPs and NOPs with an optimal one that has the
///
/// same effect.
fn swaptimize(block: *mut _PyCfgBasicblock, ix: &mut i32) -> i32 {
    // NOTE: "./python -m test test_patma" serves as a good, quick stress test
    //
    // for this function. Make sure to blow away cached *.pyc files first!
    let instructions: *mut _PyCfgInstruction =
        unsafe { (*block).b_instr }.wrapping_offset((unsafe { *(ix as *mut i32) }) as isize);
    // Find the length of the current sequence of SWAPs and NOPs, and record the
    //
    // maximum depth of the stack manipulations:
    let mut depth: i32 = unsafe { (*instructions.wrapping_offset(0 as isize)).i_oparg };
    let mut len: i32 = 0;
    let mut more: i32 = 0;
    let limit: i32 = unsafe { (*block).b_iused }.wrapping_sub(unsafe { *(ix as *mut i32) });
    loop {
        let __v2: i32 = len.wrapping_add(1);
        len = __v2;
        if __v2 >= limit {
            break;
        }
        let opcode: i32 = unsafe { (*instructions.wrapping_offset(len as isize)).i_opcode };
        if opcode == 117 {
            depth = Py_MAX!(depth, unsafe {
                (*instructions.wrapping_offset(len as isize)).i_oparg
            });
            more = 1;
        } else {
            if opcode != 27 {
                break;
            }
        }
    }
    // It's already optimal if there's only one SWAP:
    if more == 0 {
        return 0;
    }
    // Create an array with elements {0, 1, 2, ..., depth - 1}:
    let stack: *mut i32 =
        (unsafe { PyMem_Malloc((depth as u64).wrapping_mul(core::mem::size_of::<i32>() as u64)) })
            as *mut i32;
    if stack.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    for i in 0..depth {
        unsafe {
            *stack.wrapping_offset(i as isize) = i;
        }
    }
    // Simulate the combined effect of these instructions by "running" them on
    //
    // our "stack":
    for i in 0..len {
        if (unsafe { (*instructions.wrapping_offset(i as isize)).i_opcode }) == 117 {
            let oparg: i32 = unsafe { (*instructions.wrapping_offset(i as isize)).i_oparg };
            let top: i32 = unsafe { *stack.wrapping_offset(0 as isize) };
            // SWAPs are 1-indexed:
            unsafe {
                *stack.wrapping_offset(0 as isize) =
                    *stack.wrapping_offset(oparg.wrapping_sub(1) as isize);
            }
            unsafe {
                *stack.wrapping_offset(oparg.wrapping_sub(1) as isize) = top;
            }
        }
    }
    // Now we can begin! Our approach here is based on a solution to a closely
    //
    // related problem (https://cs.stackexchange.com/a/13938). It's easiest to
    //
    // think of this algorithm as determining the steps needed to efficiently
    //
    // "un-shuffle" our stack. By performing the moves in *reverse* order,
    //
    // though, we can efficiently *shuffle* it! For this reason, we will be
    //
    // replacing instructions starting from the *end* of the run. Since the
    //
    // solution is optimal, we don't need to worry about running out of space:
    let mut current: i32 = len.wrapping_sub(1);
    for i in 0..depth {
        // Skip items that have already been visited, or just happen to be in
        //
        // the correct location:
        if (unsafe { *stack.wrapping_offset(i as isize) }) == 1i32.wrapping_neg()
            || (unsafe { *stack.wrapping_offset(i as isize) }) == i
        {
            continue;
        }
        // Okay, we've found an item that hasn't been visited. It forms a cycle
        //
        // with other items; traversing the cycle and swapping each item with
        //
        // the next will put them all in the correct place. The weird
        //
        // loop-and-a-half is necessary to insert 0 into every cycle, since we
        //
        // can only swap from that position:
        let mut j: i32 = i;
        while true {
            // Skip the actual swap if our item is zero, since swapping the top
            //
            // item with itself is pointless:
            if j != 0 {
                // SWAPs are 1-indexed:
                unsafe {
                    (*instructions.wrapping_offset(current as isize)).i_opcode = 117;
                }
                let __v6: i32 = current;
                current = __v6.wrapping_sub(1);
                unsafe {
                    (*instructions.wrapping_offset(__v6 as isize)).i_oparg = j.wrapping_add(1);
                }
            }
            if (unsafe { *stack.wrapping_offset(j as isize) }) == 1i32.wrapping_neg() {
                // Completed the cycle:
                break;
            }
            let next_j: i32 = unsafe { *stack.wrapping_offset(j as isize) };
            unsafe {
                *stack.wrapping_offset(j as isize) = 1i32.wrapping_neg();
            }
            j = next_j;
        }
    }
    // NOP out any unused instructions:
    while 0 <= current {
        loop {
            let __v9: i32 = current;
            current = __v9.wrapping_sub(1);
            let _instr__ptr_: *mut _PyCfgInstruction = instructions.wrapping_offset(__v9 as isize);
            unsafe {
                (*_instr__ptr_).i_opcode = 27;
            }
            unsafe {
                (*_instr__ptr_).i_oparg = 0;
            }
            if true {
                break;
            }
        }
    }
    unsafe { PyMem_Free(stack as *mut ()) };
    let __v14: i32 = unsafe { *ix };
    unsafe {
        *ix = __v14.wrapping_add(len.wrapping_sub(1));
    }
    0
}

fn next_swappable_instruction(block: *mut _PyCfgBasicblock, mut i: i32, lineno: i32) -> i32 {
    loop {
        let __v2: i32 = i.wrapping_add(1);
        i = __v2;
        if __v2 >= unsafe { (*block).b_iused } {
            break;
        }
        let instruction: *mut _PyCfgInstruction =
            unsafe { (*block).b_instr }.wrapping_offset(i as isize);
        if 0 <= lineno && (unsafe { (*instruction).i_loc.lineno }) != lineno {
            // Optimizing across this instruction could cause user-visible
            //
            // changes in the names bound between line tracing events!
            return 1i32.wrapping_neg();
        }
        if (unsafe { (*instruction).i_opcode }) != 27 {
            if SWAPPABLE!(unsafe { (*instruction).i_opcode }) {
                return i;
            }
            return 1i32.wrapping_neg();
        }
    }
    1i32.wrapping_neg()
}

/// Attempt to apply SWAPs statically by swapping *instructions* rather than
///
/// stack items. For example, we can replace SWAP(2), POP_TOP, STORE_FAST(42)
///
/// with the more efficient NOP, STORE_FAST(42), POP_TOP.
fn apply_static_swaps(block: *mut _PyCfgBasicblock, mut i: i32) {
    // SWAPs are to our left, and potential swaperands are to our right:
    while 0 <= i {
        '__slate_continue_0: {
            let swap: *mut _PyCfgInstruction =
                unsafe { (*block).b_instr }.wrapping_offset(i as isize);
            if (unsafe { (*swap).i_opcode }) != 117 {
                if (unsafe { (*swap).i_opcode }) == 27 || SWAPPABLE!(unsafe { (*swap).i_opcode }) {
                    // Nope, but we know how to handle these. Keep looking:
                    break '__slate_continue_0;
                }
                // We can't reason about what this instruction does. Bail:
                return;
            }
            let j: i32 = next_swappable_instruction(block, i, 1i32.wrapping_neg());
            if j < 0 {
                return;
            }
            let mut k: i32 = j;
            let lineno: i32 = unsafe {
                (*unsafe { (*block).b_instr }.wrapping_offset(j as isize))
                    .i_loc
                    .lineno
            };
            let mut count: i32 = unsafe { (*swap).i_oparg }.wrapping_sub(1);
            while 0 < count {
                k = next_swappable_instruction(block, k, lineno);
                if k < 0 {
                    return;
                }
                count = count.wrapping_sub(1);
            }
            // The reordering is not safe if the two instructions to be swapped
            //
            // store to the same location, or if any intervening instruction stores
            //
            // to the same location as either of them.
            let store_j: i32 = STORES_TO!(*unsafe { (*block).b_instr }.wrapping_offset(j as isize));
            let store_k: i32 = STORES_TO!(*unsafe { (*block).b_instr }.wrapping_offset(k as isize));
            if store_j >= 0 || store_k >= 0 {
                if store_j == store_k {
                    return;
                }
                for idx in j.wrapping_add(1)..k {
                    let store_idx: i32 =
                        STORES_TO!(*unsafe { (*block).b_instr }.wrapping_offset(idx as isize));
                    if store_idx >= 0 && (store_idx == store_j || store_idx == store_k) {
                        return;
                    }
                }
            }
            // Success!
            INSTR_SET_OP0!(swap, 27i32);
            let temp: _PyCfgInstruction =
                unsafe { *unsafe { (*block).b_instr }.wrapping_offset(j as isize) };
            unsafe {
                *unsafe { (*block).b_instr }.wrapping_offset(j as isize) =
                    *unsafe { (*block).b_instr }.wrapping_offset(k as isize);
            }
            unsafe {
                *unsafe { (*block).b_instr }.wrapping_offset(k as isize) = temp;
            }
        }
        i = i.wrapping_sub(1);
    }
}

fn basicblock_optimize_load_const(
    const_cache: *mut _object,
    bb: *mut _PyCfgBasicblock,
    consts: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let mut opcode: i32 = 0;
    let mut oparg: i32 = 0;
    let mut i: i32 = 0;
    while i < unsafe { (*bb).b_iused } {
        let inst: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
        if (unsafe { (*inst).i_opcode }) == 82 {
            let constant: *mut _object = get_const_value(
                unsafe { (*inst).i_opcode },
                unsafe { (*inst).i_oparg },
                consts,
            );
            if constant.is_null() {
                return 1i32.wrapping_neg();
            }
            let res: i32 = maybe_instr_make_load_smallint(inst, constant, consts, const_cache);
            Py_DECREF!(constant);
            if res < 0 {
                return 1i32.wrapping_neg();
            }
        }
        let is_copy_of_load_const: bool = opcode == 82
            && (unsafe { (*inst).i_opcode }) == 59
            && (unsafe { (*inst).i_oparg }) == 1;
        if !is_copy_of_load_const {
            opcode = unsafe { (*inst).i_opcode };
            oparg = unsafe { (*inst).i_oparg };
        }
        if !(opcode != 82 && opcode != 94) {
            let nextop: i32 = if i.wrapping_add(1) < unsafe { (*bb).b_iused } {
                unsafe {
                    (*unsafe { (*bb).b_instr }.wrapping_offset(i.wrapping_add(1) as isize)).i_opcode
                }
            } else {
                0
            };
            '__slate_break_3: {
                match nextop {
                    100 | 103 | 258 | 259 => {
                        // Remove LOAD_CONST const; conditional jump
                        let cnt: *mut _object = get_const_value(opcode, oparg, consts);
                        if cnt.is_null() {
                            return 1i32.wrapping_neg();
                        }
                        let is_true: i32 = unsafe { PyObject_IsTrue(cnt) };
                        Py_DECREF!(cnt);
                        if is_true == 1i32.wrapping_neg() {
                            return 1i32.wrapping_neg();
                        }
                        if PyCompile_OpcodeStackEffect(nextop, 0i32) == 1i32.wrapping_neg() {
                            // POP_JUMP_IF_FALSE or POP_JUMP_IF_TRUE
                            INSTR_SET_OP0!(inst, 27i32);
                        }
                        let jump_if_true: i32 = (nextop == 103 || nextop == 259) as i32;
                        if is_true == jump_if_true {
                            unsafe {
                                (*unsafe { (*bb).b_instr }
                                    .wrapping_offset(i.wrapping_add(1) as isize))
                                .i_opcode = 257;
                            }
                        } else {
                            INSTR_SET_OP0!(
                                unsafe { (*bb).b_instr }
                                    .wrapping_offset(i.wrapping_add(1) as isize),
                                27i32
                            );
                        }
                    }
                    74 => {
                        // Fold to POP_JUMP_IF_NONE:
                        //
                        // - LOAD_CONST(None) IS_OP(0) POP_JUMP_IF_TRUE
                        //
                        // - LOAD_CONST(None) IS_OP(1) POP_JUMP_IF_FALSE
                        //
                        // - LOAD_CONST(None) IS_OP(0) TO_BOOL POP_JUMP_IF_TRUE
                        //
                        // - LOAD_CONST(None) IS_OP(1) TO_BOOL POP_JUMP_IF_FALSE
                        //
                        // Fold to POP_JUMP_IF_NOT_NONE:
                        //
                        // - LOAD_CONST(None) IS_OP(0) POP_JUMP_IF_FALSE
                        //
                        // - LOAD_CONST(None) IS_OP(1) POP_JUMP_IF_TRUE
                        //
                        // - LOAD_CONST(None) IS_OP(0) TO_BOOL POP_JUMP_IF_FALSE
                        //
                        // - LOAD_CONST(None) IS_OP(1) TO_BOOL POP_JUMP_IF_TRUE
                        let cnt: *mut _object = get_const_value(opcode, oparg, consts);
                        if cnt.is_null() {
                            return 1i32.wrapping_neg();
                        }
                        if !Py_IsNone!(cnt) {
                            Py_DECREF!(cnt);
                        } else {
                            if !((unsafe { (*bb).b_iused }) <= i.wrapping_add(2)) {
                                let is_instr: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }
                                    .wrapping_offset(i.wrapping_add(1) as isize);
                                let mut jump_instr: *mut _PyCfgInstruction =
                                    unsafe { (*bb).b_instr }
                                        .wrapping_offset(i.wrapping_add(2) as isize);
                                // Get rid of TO_BOOL regardless:
                                if (unsafe { (*jump_instr).i_opcode }) == 39 {
                                    INSTR_SET_OP0!(jump_instr, 27i32);
                                    if (unsafe { (*bb).b_iused }) <= i.wrapping_add(3) {
                                        break '__slate_break_3;
                                    }
                                    jump_instr = unsafe { (*bb).b_instr }
                                        .wrapping_offset(i.wrapping_add(3) as isize);
                                }
                                let mut invert: bool = (unsafe { (*is_instr).i_oparg }) != 0;
                                if (unsafe { (*jump_instr).i_opcode }) == 100 {
                                    invert = !invert;
                                } else {
                                    if (unsafe { (*jump_instr).i_opcode }) != 103 {
                                        break '__slate_break_3;
                                    }
                                }
                                INSTR_SET_OP0!(inst, 27i32);
                                INSTR_SET_OP0!(is_instr, 27i32);
                                unsafe {
                                    (*jump_instr).i_opcode = if invert { 102 } else { 101 };
                                }
                            }
                        }
                    }
                    39 => {
                        let mut cnt: *mut _object = get_const_value(opcode, oparg, consts);
                        if cnt.is_null() {
                            return 1i32.wrapping_neg();
                        }
                        let is_true: i32 = unsafe { PyObject_IsTrue(cnt) };
                        Py_DECREF!(cnt);
                        if is_true == 1i32.wrapping_neg() {
                            return 1i32.wrapping_neg();
                        }
                        cnt = unsafe { PyBool_FromLong(is_true as i64) };
                        let index: i32 = add_const(cnt, consts, const_cache, consts_index);
                        if index < 0 {
                            return 1i32.wrapping_neg();
                        }
                        INSTR_SET_OP0!(inst, 27i32);
                        INSTR_SET_OP1!(
                            unsafe { (*bb).b_instr }.wrapping_offset(i.wrapping_add(1) as isize),
                            82i32,
                            index
                        );
                    }
                    _ => {}
                }
            }
        }
        i = i.wrapping_add(1);
    }
    0
}

fn optimize_load_const(
    const_cache: *mut _object,
    g: *mut _PyCfgBuilder,
    consts: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        RETURN_IF_ERROR__2!(basicblock_optimize_load_const(
            const_cache,
            b,
            consts,
            consts_index
        ));
        b = unsafe { (*b).b_next };
    }
    0
}

fn optimize_basic_block(
    const_cache: *mut _object,
    bb: *mut _PyCfgBasicblock,
    consts: *mut _object,
    consts_index: *mut _Py_hashtable_t,
) -> i32 {
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut nextop: i32 = unsafe { core::mem::zeroed() };
    let mut oparg: i32 = unsafe { core::mem::zeroed() };
    let mut opcode: i32 = unsafe { core::mem::zeroed() };
    let mut target: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    let mut inst_1: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    let mut i_1: i32 = unsafe { core::mem::zeroed() };
    let mut nop: core::mem::MaybeUninit<_PyCfgInstruction> = core::mem::MaybeUninit::uninit();
    let __slate_slot_38: *mut _PyCfgInstruction =
        core::ptr::addr_of_mut!(nop) as *mut _PyCfgInstruction;
    unsafe {
        INSTR_SET_OP0!(core::ptr::addr_of_mut!(*__slate_slot_38), 27i32);
        i_1 = 0;
        while i_1 < (*bb).b_iused {
            inst_1 = unsafe { (*bb).b_instr }.wrapping_offset(i_1 as isize);
            opcode = (*inst_1).i_opcode;
            oparg = (*inst_1).i_oparg;
            if HAS_TARGET!(opcode) {
                target = unsafe { (*(*inst_1).i_target).b_instr }.wrapping_offset(0 as isize);
            } else {
                target = core::ptr::addr_of_mut!(*__slate_slot_38);
            }
            '__join_9: {
                '__join_55: {
                    '__join_54: {
                        '__join_42: {
                            '__join_29: {
                                '__join_18: {
                                    nextop = if i_1.wrapping_add(1) < (*bb).b_iused {
                                        (*unsafe { (*bb).b_instr }
                                            .wrapping_offset(i_1.wrapping_add(1) as isize))
                                        .i_opcode
                                    } else {
                                        0
                                    };
                                    let __t7: i32 = opcode;
                                    if __t7 == 51 {
                                        '__join_56: {
                                            if nextop == 119
                                                && oparg
                                                    == (*unsafe { (*bb).b_instr }.wrapping_offset(
                                                        i_1.wrapping_add(1) as isize,
                                                    ))
                                                    .i_oparg
                                            {
                                                let __t6: i32 = oparg;
                                                if __t6 == 1 {
                                                    INSTR_SET_OP0!(inst_1, 27i32);
                                                    INSTR_SET_OP0!(
                                                        unsafe { (*bb).b_instr }.wrapping_offset(
                                                            i_1.wrapping_add(1) as isize
                                                        ),
                                                        27i32
                                                    );
                                                    break '__join_9;
                                                } else {
                                                    if __t6 != 2 {
                                                        if __t6 != 3 {
                                                            break '__join_56;
                                                        }
                                                    }
                                                }
                                                INSTR_SET_OP0!(inst_1, 27i32);
                                                (*unsafe { (*bb).b_instr }.wrapping_offset(
                                                    i_1.wrapping_add(1) as isize,
                                                ))
                                                .i_opcode = 117;
                                                break '__join_9;
                                            }
                                        }
                                        RETURN_IF_ERROR__2!(fold_tuple_of_constants(
                                            bb,
                                            i_1,
                                            consts,
                                            const_cache,
                                            consts_index
                                        ));
                                        break '__join_9;
                                    } else {
                                        if __t7 == 46 {
                                            break '__join_55;
                                        } else {
                                            if __t7 == 48 {
                                                break '__join_55;
                                            } else {
                                                if __t7 == 102 {
                                                    break '__join_54;
                                                } else {
                                                    if __t7 == 101 {
                                                        break '__join_54;
                                                    } else {
                                                        if __t7 == 100 {
                                                            let __t4: i32 = (*target).i_opcode;
                                                            if __t4 == 257 {
                                                                let __v28: bool = jump_thread(
                                                                    bb, inst_1, target, 100i32,
                                                                );
                                                                i_1 =
                                                                    i_1.wrapping_sub(__v28 as i32);
                                                                break '__join_9;
                                                            } else {
                                                                break '__join_9;
                                                            }
                                                        } else {
                                                            if __t7 == 103 {
                                                                let __t3: i32 = (*target).i_opcode;
                                                                if __t3 == 257 {
                                                                    let __v25: bool = jump_thread(
                                                                        bb, inst_1, target, 103i32,
                                                                    );
                                                                    i_1 = i_1
                                                                        .wrapping_sub(__v25 as i32);
                                                                    break '__join_9;
                                                                } else {
                                                                    break '__join_9;
                                                                }
                                                            } else {
                                                                if __t7 == 258 {
                                                                    let __t2: i32 =
                                                                        (*target).i_opcode;
                                                                    if __t2 != 257 {
                                                                        if __t2 != 258 {
                                                                            if __t2 == 259 {
                                                                                // No need to check for loops here, a block's b_next
                                                                                //
                                                                                // cannot point to itself.
                                                                                (*inst_1)
                                                                                    .i_target =
                                                                                    (*(*inst_1)
                                                                                        .i_target)
                                                                                        .b_next;
                                                                                i_1 = i_1
                                                                                    .wrapping_sub(
                                                                                        1,
                                                                                    );
                                                                                break '__join_9;
                                                                            } else {
                                                                                break '__join_9;
                                                                            }
                                                                        }
                                                                    }
                                                                    let __v22: bool = jump_thread(
                                                                        bb, inst_1, target, 258i32,
                                                                    );
                                                                    i_1 = i_1
                                                                        .wrapping_sub(__v22 as i32);
                                                                    break '__join_9;
                                                                } else {
                                                                    if __t7 == 259 {
                                                                        let __t1: i32 =
                                                                            (*target).i_opcode;
                                                                        if __t1 != 257 {
                                                                            if __t1 != 259 {
                                                                                if __t1 == 258 {
                                                                                    // No need to check for loops here, a block's b_next
                                                                                    //
                                                                                    // cannot point to itself.
                                                                                    (*inst_1).i_target = (*(*inst_1).i_target).b_next;
                                                                                    i_1 = i_1.wrapping_sub(1);
                                                                                    break '__join_9;
                                                                                } else {
                                                                                    break '__join_9;
                                                                                }
                                                                            }
                                                                        }
                                                                        let __v17: bool =
                                                                            jump_thread(
                                                                                bb, inst_1, target,
                                                                                259i32,
                                                                            );
                                                                        i_1 = i_1.wrapping_sub(
                                                                            __v17 as i32,
                                                                        );
                                                                        break '__join_9;
                                                                    } else {
                                                                        if __t7 == 257 {
                                                                            break '__join_42;
                                                                        } else {
                                                                            if __t7 == 260 {
                                                                                break '__join_42;
                                                                            } else {
                                                                                if __t7 == 70 {
                                                                                    if (*target)
                                                                                        .i_opcode
                                                                                        == 257
                                                                                    {
                                                                                        // This will not work now because the jump (at target) could
                                                                                        // be forward or backward and FOR_ITER only jumps forward. We
                                                                                        // can re-enable this if ever we implement a backward version
                                                                                        // of FOR_ITER.
                                                                                        //
                                                                                        // i -= jump_thread(bb, inst, target, FOR_ITER);
                                                                                        break '__join_9;
                                                                                    } else {
                                                                                        break '__join_9;
                                                                                    }
                                                                                } else {
                                                                                    if __t7 == 112 {
                                                                                        if opcode == nextop && oparg == (*unsafe { (*bb).b_instr }.wrapping_offset(i_1.wrapping_add(1) as isize)).i_oparg && (*unsafe { (*bb).b_instr }.wrapping_offset(i_1 as isize)).i_loc.lineno == (*unsafe { (*bb).b_instr }.wrapping_offset(i_1.wrapping_add(1) as isize)).i_loc.lineno {
(*unsafe { (*bb).b_instr }.wrapping_offset(i_1 as isize)).i_opcode = 31;
(*unsafe { (*bb).b_instr }.wrapping_offset(i_1 as isize)).i_oparg = 0;
break '__join_9;
} else {
break '__join_9;
}
                                                                                    } else {
                                                                                        if __t7
                                                                                            == 117
                                                                                        {
                                                                                            if oparg
                                                                                                == 1
                                                                                            {
                                                                                                INSTR_SET_OP0!(inst_1, 27i32);
                                                                                                break '__join_9;
                                                                                            } else {
                                                                                                break '__join_9;
                                                                                            }
                                                                                        } else {
                                                                                            if __t7 == 92 {
if nextop == 33 && oparg & 1 == 0 {
INSTR_SET_OP1!(inst_1, 92i32, oparg | 1);
INSTR_SET_OP0!(unsafe { (*bb).b_instr }.wrapping_offset(i_1.wrapping_add(1) as isize), 27i32);
break '__join_9;
} else {
break '__join_9;
}
} else {
if __t7 == 56 {
if nextop == 39 {
INSTR_SET_OP0!(inst_1, 27i32);
INSTR_SET_OP1!(unsafe { (*bb).b_instr }.wrapping_offset(i_1.wrapping_add(1) as isize), 56i32, oparg | 16);
break '__join_9;
} else {
break '__join_9;
}
} else {
if __t7 == 57 {
break '__join_29;
} else {
if __t7 == 74 {
break '__join_29;
} else {
if __t7 == 39 {
if nextop == 39 {
INSTR_SET_OP0!(inst_1, 27i32);
break '__join_9;
} else {
break '__join_9;
}
} else {
if __t7 == 42 {
if nextop == 39 {
INSTR_SET_OP0!(inst_1, 27i32);
INSTR_SET_OP0!(unsafe { (*bb).b_instr }.wrapping_offset(i_1.wrapping_add(1) as isize), 42i32);
break '__join_9;
} else {
if nextop == 42 {
INSTR_SET_OP0!(inst_1, 27i32);
INSTR_SET_OP0!(unsafe { (*bb).b_instr }.wrapping_offset(i_1.wrapping_add(1) as isize), 27i32);
break '__join_9;
} else {
break '__join_18;
}
}
} else {
if __t7 == 40 {
break '__join_18;
} else {
if __t7 == 41 {
break '__join_18;
} else {
if __t7 == 53 {
if oparg == 6 {
RETURN_IF_ERROR__2!(fold_constant_seq_into_load_const(bb, i_1, consts, const_cache, consts_index));
if (*inst_1).i_opcode == 53 && nextop == 16 {
INSTR_SET_OP0!(inst_1, 27i32);
break '__join_9;
} else {
break '__join_9;
}
} else {
if oparg == 5 {
RETURN_IF_ERROR__2!(fold_const_unaryop(bb, i_1, consts, const_cache, consts_index));
break '__join_9;
} else {
break '__join_9;
}
}
} else {
if __t7 != 78 {
if __t7 != 107 {
if __t7 == 44 {
RETURN_IF_ERROR__2!(fold_const_binop(bb, i_1, consts, const_cache, consts_index));
break '__join_9;
} else {
break '__join_9;
}
}
}
}
}
}
}
}
}
}
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if oparg == 1 && (nextop == 16 || nextop == 57) {
                                        RETURN_IF_ERROR__2!(fold_constant_seq_into_load_const(
                                            bb,
                                            i_1,
                                            consts,
                                            const_cache,
                                            consts_index
                                        ));
                                        break '__join_9;
                                    } else {
                                        break '__join_9;
                                    }
                                }
                                RETURN_IF_ERROR__2!(fold_const_unaryop(
                                    bb,
                                    i_1,
                                    consts,
                                    const_cache,
                                    consts_index
                                ));
                                break '__join_9;
                            }
                            if nextop == 39 {
                                INSTR_SET_OP0!(inst_1, 27i32);
                                INSTR_SET_OP1!(
                                    unsafe { (*bb).b_instr }
                                        .wrapping_offset(i_1.wrapping_add(1) as isize),
                                    opcode,
                                    oparg
                                );
                                break '__join_9;
                            } else {
                                if nextop == 42 {
                                    INSTR_SET_OP0!(inst_1, 27i32);
                                    let inverted: i32 = oparg ^ 1;
                                    INSTR_SET_OP1!(
                                        unsafe { (*bb).b_instr }
                                            .wrapping_offset(i_1.wrapping_add(1) as isize),
                                        opcode,
                                        inverted
                                    );
                                    break '__join_9;
                                } else {
                                    break '__join_9;
                                }
                            }
                        }
                        let __t0: i32 = (*target).i_opcode;
                        if __t0 == 257 {
                            let __v12: bool = jump_thread(bb, inst_1, target, 257i32);
                            i_1 = i_1.wrapping_sub(__v12 as i32);
                            break '__join_9;
                        } else {
                            if __t0 == 260 {
                                let __v9: bool = jump_thread(bb, inst_1, target, opcode);
                                i_1 = i_1.wrapping_sub(__v9 as i32);
                                break '__join_9;
                            } else {
                                break '__join_9;
                            }
                        }
                    }
                    let __t5: i32 = (*target).i_opcode;
                    if __t5 == 257 {
                        let __v31: bool = jump_thread(bb, inst_1, target, (*inst_1).i_opcode);
                        i_1 = i_1.wrapping_sub(__v31 as i32);
                        break '__join_9;
                    } else {
                        break '__join_9;
                    }
                }
                RETURN_IF_ERROR__2!(optimize_lists_and_sets(
                    bb,
                    i_1,
                    nextop,
                    consts,
                    const_cache,
                    consts_index
                ));
            }
            i_1 = i_1.wrapping_add(1);
        }
        i = 0;
        while i < (*bb).b_iused {
            let inst: *mut _PyCfgInstruction = unsafe { (*bb).b_instr }.wrapping_offset(i as isize);
            if (*inst).i_opcode == 117 {
                if swaptimize(bb, &mut (*core::ptr::addr_of_mut!(i))) < 0 {
                    return 1i32.wrapping_neg();
                } else {
                    apply_static_swaps(bb, i);
                }
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
    // Try to fold tuples of constants.
    // Skip over BUILD_TUPLE(1) UNPACK_SEQUENCE(1).
    // Replace BUILD_TUPLE(2) UNPACK_SEQUENCE(2) with SWAP(2).
    // Replace BUILD_TUPLE(3) UNPACK_SEQUENCE(3) with SWAP(3).
}

fn remove_redundant_nops_and_jumps(g: *mut _PyCfgBuilder) -> i32 {
    let mut removed_nops: i32 = 0;
    let mut removed_jumps: i32 = 0;
    loop {
        // Convergence is guaranteed because the number of
        // redundant jumps and nops only decreases.
        removed_nops = remove_redundant_nops(g);
        RETURN_IF_ERROR__2!(removed_nops);
        removed_jumps = remove_redundant_jumps(g);
        RETURN_IF_ERROR__2!(removed_jumps);
        if removed_nops.wrapping_add(removed_jumps) <= 0 {
            break;
        }
    }
    0
}

/// Perform optimizations on a control flow graph.
/// The consts object should still be in list form to allow new constants
/// to be appended.
///
/// Code trasnformations that reduce code size initially fill the gaps with
/// NOPs.  Later those NOPs are removed.
fn optimize_cfg(
    g: *mut _PyCfgBuilder,
    consts: *mut _object,
    const_cache: *mut _object,
    consts_index: *mut _Py_hashtable_t,
    firstlineno: i32,
) -> i32 {
    RETURN_IF_ERROR__2!(check_cfg(g));
    RETURN_IF_ERROR__2!(inline_small_or_no_lineno_blocks(unsafe {
        (*g).g_entryblock
    }));
    RETURN_IF_ERROR__2!(remove_unreachable(unsafe { (*g).g_entryblock }));
    RETURN_IF_ERROR__2!(resolve_line_numbers(g, firstlineno));
    RETURN_IF_ERROR__2!(optimize_load_const(const_cache, g, consts, consts_index));
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        RETURN_IF_ERROR__2!(optimize_basic_block(const_cache, b, consts, consts_index));
        b = unsafe { (*b).b_next };
    }
    RETURN_IF_ERROR__2!(remove_redundant_nops_and_pairs(unsafe {
        (*g).g_entryblock
    }));
    RETURN_IF_ERROR__2!(remove_unreachable(unsafe { (*g).g_entryblock }));
    RETURN_IF_ERROR__2!(remove_redundant_nops_and_jumps(g));
    0
}

fn make_super_instruction(
    inst1: *mut _PyCfgInstruction,
    inst2: *mut _PyCfgInstruction,
    super_op: i32,
) {
    let line1: i32 = unsafe { (*inst1).i_loc.lineno };
    let line2: i32 = unsafe { (*inst2).i_loc.lineno };
    // Skip if instructions are on different lines
    if line1 >= 0 && line2 >= 0 && line1 != line2 {
        return;
    }
    if (unsafe { (*inst1).i_oparg }) >= 16 || (unsafe { (*inst2).i_oparg }) >= 16 {
        return;
    }
    INSTR_SET_OP1!(
        inst1,
        super_op,
        (unsafe { (*inst1).i_oparg }) << 4i32 | unsafe { (*inst2).i_oparg }
    );
    INSTR_SET_OP0!(inst2, 27i32);
}

fn insert_superinstructions(g: *mut _PyCfgBuilder) -> i32 {
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let inst: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            let nextop: i32 = if i.wrapping_add(1) < unsafe { (*b).b_iused } {
                unsafe {
                    (*unsafe { (*b).b_instr }.wrapping_offset(i.wrapping_add(1) as isize)).i_opcode
                }
            } else {
                0
            };
            match unsafe { (*inst).i_opcode } {
                84 => {
                    if nextop == 84 {
                        make_super_instruction(
                            inst,
                            unsafe { (*b).b_instr }.wrapping_offset(i.wrapping_add(1) as isize),
                            89i32,
                        );
                    }
                }
                112 => match nextop {
                    84 => {
                        make_super_instruction(
                            inst,
                            unsafe { (*b).b_instr }.wrapping_offset(i.wrapping_add(1) as isize),
                            113i32,
                        );
                    }
                    112 => {
                        make_super_instruction(
                            inst,
                            unsafe { (*b).b_instr }.wrapping_offset(i.wrapping_add(1) as isize),
                            114i32,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    let res: i32 = remove_redundant_nops(g);
    res
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_flowgraph_c_2656 {
    /// Index of instruction that produced the reference or DUMMY_INSTR.
    instr: i32,
    /// The local to which the reference refers or NOT_LOCAL.
    local: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_flowgraph_c_2662 {
    refs: *mut __SlateRecord_flowgraph_c_2656,
    size: i64,
    capacity: i64,
}

fn ref_stack_push(
    stack: *mut __SlateRecord_flowgraph_c_2662,
    r: __SlateRecord_flowgraph_c_2656,
) -> i32 {
    if (unsafe { (*stack).size }) == unsafe { (*stack).capacity } {
        let new_cap: i64 = Py_MAX__3!(32i32, unsafe { (*stack).capacity }.wrapping_mul(2));
        let refs: *mut __SlateRecord_flowgraph_c_2656 = (unsafe {
            PyMem_Realloc(
                (*stack).refs as *mut (),
                (core::mem::size_of::<__SlateRecord_flowgraph_c_2656>() as u64)
                    .wrapping_mul(new_cap as u64),
            )
        })
            as *mut __SlateRecord_flowgraph_c_2656;
        if refs.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*stack).refs = refs;
        }
        unsafe {
            (*stack).capacity = new_cap;
        }
    }
    unsafe {
        *unsafe { (*stack).refs }.wrapping_offset((*stack).size as isize) = r;
    }
    let __v1: i64 = unsafe { (*stack).size };
    unsafe {
        (*stack).size = __v1.wrapping_add(1);
    }
    0
}

fn ref_stack_pop(stack: &mut __SlateRecord_flowgraph_c_2662) -> __SlateRecord_flowgraph_c_2656 {
    let __v1: i64 = unsafe { (*stack).size };
    unsafe {
        (*stack).size = __v1.wrapping_sub(1);
    }
    let r: __SlateRecord_flowgraph_c_2656 = unsafe {
        *unsafe { (*(stack as *mut __SlateRecord_flowgraph_c_2662)).refs }
            .wrapping_offset((*(stack as *mut __SlateRecord_flowgraph_c_2662)).size as isize)
    };
    return r;
}

fn ref_stack_swap_top(stack: &__SlateRecord_flowgraph_c_2662, off: i64) {
    let idx: i64 =
        unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).size }.wrapping_sub(off);
    let tmp: __SlateRecord_flowgraph_c_2656 = unsafe {
        *unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).refs }
            .wrapping_offset(idx as isize)
    };
    unsafe {
        *unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).refs }
            .wrapping_offset(idx as isize) =
            *unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).refs }.wrapping_offset(
                unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).size }.wrapping_sub(1)
                    as isize,
            );
    }
    unsafe {
        *unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).refs }.wrapping_offset(
            unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).size }.wrapping_sub(1)
                as isize,
        ) = tmp;
    }
}

fn ref_stack_at(
    stack: &__SlateRecord_flowgraph_c_2662,
    idx: i64,
) -> __SlateRecord_flowgraph_c_2656 {
    return unsafe {
        *unsafe { (*(stack as *const __SlateRecord_flowgraph_c_2662)).refs }
            .wrapping_offset(idx as isize)
    };
}

fn ref_stack_clear(stack: &mut __SlateRecord_flowgraph_c_2662) {
    unsafe {
        (*(stack as *mut __SlateRecord_flowgraph_c_2662)).size = 0;
    }
}

fn ref_stack_fini(stack: &mut __SlateRecord_flowgraph_c_2662) {
    if !unsafe { (*(stack as *mut __SlateRecord_flowgraph_c_2662)).refs }.is_null() {
        unsafe { PyMem_Free((*(stack as *mut __SlateRecord_flowgraph_c_2662)).refs as *mut ()) };
    }
    unsafe {
        (*(stack as *mut __SlateRecord_flowgraph_c_2662)).refs =
            core::ptr::null_mut::<__SlateRecord_flowgraph_c_2656>();
    }
    unsafe {
        (*(stack as *mut __SlateRecord_flowgraph_c_2662)).capacity = 0;
    }
    unsafe {
        (*(stack as *mut __SlateRecord_flowgraph_c_2662)).size = 0;
    }
}

fn kill_local(instr_flags: *mut u8, refs: &__SlateRecord_flowgraph_c_2662, local: i32) {
    let mut i: i64 = 0;
    while i < unsafe { (*(refs as *const __SlateRecord_flowgraph_c_2662)).size } {
        let r: __SlateRecord_flowgraph_c_2656 = ref_stack_at(refs, i);
        if r.local == local {
            let __v0: *mut u8 = instr_flags.wrapping_offset(r.instr as isize);
            unsafe {
                *__v0 = ((*__v0 as i32) | 1) as u8;
            }
        }
        i = i.wrapping_add(1);
    }
}

fn store_local(
    instr_flags: *mut u8,
    refs: &__SlateRecord_flowgraph_c_2662,
    local: i32,
    r: __SlateRecord_flowgraph_c_2656,
) {
    kill_local(instr_flags, refs, local);
    if r.instr != 1i32.wrapping_neg() {
        let __v0: *mut u8 = instr_flags.wrapping_offset(r.instr as isize);
        unsafe {
            *__v0 = ((*__v0 as i32) | 2) as u8;
        }
    }
}

fn load_fast_push_block(
    sp: &mut *mut *mut _PyCfgBasicblock,
    target: *mut _PyCfgBasicblock,
    start_depth: i64,
) {
    if ((unsafe { (*target).__slate_bits_0.__get_b_visited() }) as i32) == 0 {
        unsafe {
            (*target).__slate_bits_0.__set_b_visited(1u32);
        }
        let __v1: *mut *mut _PyCfgBasicblock = unsafe { *sp };
        unsafe {
            *sp = __v1.wrapping_offset(1 as isize);
        }
        unsafe {
            *__v1 = target;
        }
    }
}

/// Strength reduce LOAD_FAST{_LOAD_FAST} instructions into faster variants that
/// load borrowed references onto the operand stack.
///
/// This is only safe when we can prove that the reference in the frame outlives
/// the borrowed reference produced by the instruction. We make this tractable
/// by enforcing the following lifetimes:
///
/// 1. Borrowed references loaded onto the operand stack live until the end of
///    the instruction that consumes them from the stack. Any borrowed
///    references that would escape into the heap (e.g. into frame objects or
///    generators) are converted into new, strong references.
///
/// 2. Locals live until they are either killed by an instruction
///    (e.g. STORE_FAST) or the frame is unwound. Any local that is overwritten
///    via `f_locals` is added to a tuple owned by the frame object.
///
/// To simplify the problem of detecting which supporting references in the
/// frame are killed by instructions that overwrite locals, we only allow
/// borrowed references to be stored as a local in the frame if they were passed
/// as an argument. {RETURN,YIELD}_VALUE convert borrowed references into new,
/// strong references.
///
/// Using the above, we can optimize any LOAD_FAST{_LOAD_FAST} instructions
/// that meet the following criteria:
///
/// 1. The produced reference must be consumed from the stack before the
///    supporting reference in the frame is killed.
///
/// 2. The produced reference cannot be stored as a local.
///
/// We use abstract interpretation to identify instructions that meet these
/// criteria. For each basic block, we simulate the effect the bytecode has on a
/// stack of abstract references and note any instructions that violate the
/// criteria above. Once we've processed all the instructions in a block, any
/// non-violating LOAD_FAST{_LOAD_FAST} can be optimized.
fn optimize_load_fast(g: *mut _PyCfgBuilder) -> i32 {
    let mut __slate_compound_63: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_64: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_65: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_66: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_67: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_68: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_69: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_70: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_71: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_72: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_73: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_74: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_75: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_76: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_77: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_78: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_79: __SlateRecord_flowgraph_c_2656 = unsafe { core::mem::zeroed() };
    let mut instr: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    // Optimize instructions
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut r: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(r) as *mut __SlateRecord_flowgraph_c_2656;
    // Mark instructions that produce values that are on the stack at the
    //
    // end of the basic block
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut j: i32 = unsafe { core::mem::zeroed() };
    // Block push opcodes only affect the stack when jumping
    //
    // to the target.
    let mut j_1: i32 = unsafe { core::mem::zeroed() };
    let mut num_pushed: i32 = unsafe { core::mem::zeroed() };
    let mut num_popped: i32 = unsafe { core::mem::zeroed() };
    let mut tos: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_22: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(tos) as *mut __SlateRecord_flowgraph_c_2656;
    let mut self_: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_23: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(self_) as *mut __SlateRecord_flowgraph_c_2656;
    let mut tos: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_24: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(tos) as *mut __SlateRecord_flowgraph_c_2656;
    let mut i_2: i32 = unsafe { core::mem::zeroed() };
    let mut net_popped: i32 = unsafe { core::mem::zeroed() };
    let mut i_3: i32 = unsafe { core::mem::zeroed() };
    let mut net_pushed: i32 = unsafe { core::mem::zeroed() };
    let mut r_1: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_37: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(r_1) as *mut __SlateRecord_flowgraph_c_2656;
    // STORE_FAST
    let mut r_2: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_39: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(r_2) as *mut __SlateRecord_flowgraph_c_2656;
    // STORE_FAST
    let mut r_3: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_40: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(r_3) as *mut __SlateRecord_flowgraph_c_2656;
    let mut r: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2656> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_41: *mut __SlateRecord_flowgraph_c_2656 =
        core::ptr::addr_of_mut!(r) as *mut __SlateRecord_flowgraph_c_2656;
    let mut oparg: i32 = unsafe { core::mem::zeroed() };
    let mut opcode: i32 = unsafe { core::mem::zeroed() };
    let mut instr_1: *mut _PyCfgInstruction = unsafe { core::mem::zeroed() };
    let mut i_4: i32 = unsafe { core::mem::zeroed() };
    let mut i_5: i32 = unsafe { core::mem::zeroed() };
    let mut sp: *mut *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut blocks: *mut *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut b: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut entryblock: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut max_instrs: i32 = unsafe { core::mem::zeroed() };
    let mut refs: core::mem::MaybeUninit<__SlateRecord_flowgraph_c_2662> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_61: *mut __SlateRecord_flowgraph_c_2662 =
        core::ptr::addr_of_mut!(refs) as *mut __SlateRecord_flowgraph_c_2662;
    let mut status: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        core::ptr::write(__slate_slot_61, {
            let mut __t2: __SlateRecord_flowgraph_c_2662 = core::mem::zeroed();
            __t2.refs = core::ptr::null_mut::<__SlateRecord_flowgraph_c_2656>();
            __t2
        });
        max_instrs = 0;
        entryblock = (*g).g_entryblock;
        b = entryblock;
        while !b.is_null() {
            max_instrs = Py_MAX!(max_instrs, unsafe { (*b).b_iused });
            b = (*b).b_next;
        }
        let instr_flags_size: u64 =
            (max_instrs as u64).wrapping_mul(core::mem::size_of::<u8>() as u64);
        let instr_flags: *mut u8 = PyMem_Malloc(instr_flags_size) as *mut u8;
        if instr_flags.is_null() {
            PyErr_NoMemory();
            return 1i32.wrapping_neg();
        } else {
            '__join_0: {
                blocks = make_cfg_traversal_stack(entryblock);
                if blocks.is_null() {
                    status = 1i32.wrapping_neg();
                } else {
                    sp = blocks;
                    *sp = entryblock;
                    let __v53: *mut *mut _PyCfgBasicblock = sp;
                    sp = __v53.wrapping_offset(1 as isize);
                    (*entryblock).b_startdepth = 0;
                    (*entryblock).__slate_bits_0.__set_b_visited(1u32);
                    '__join_103: {
                        '__join_95: {
                            '__join_91: {
                                '__join_88: {
                                    '__join_85: {
                                        '__join_80: {
                                            '__join_75: {
                                                '__join_70: {
                                                    '__join_61: {
                                                        '__join_57: {
                                                            '__join_53: {
                                                                '__join_48: {
                                                                    '__join_43: {
                                                                        '__join_39: {
                                                                            '__join_36: {
                                                                                '__join_32: {
                                                                                    '__join_22: {
                                                                                        '__loop_2: while sp != blocks {
let __v50: *mut *mut _PyCfgBasicblock = sp;
let __v49: *mut *mut _PyCfgBasicblock = __v50.wrapping_offset(-(1 as isize));
sp = __v49;
let block: *mut _PyCfgBasicblock = *__v49;
// Reset per-block state.
core::ptr::write_bytes(((instr_flags as *mut ()) as *mut core::ffi::c_void) as *mut u8, (0i32 as i32) as u8, (((*block).b_iused as u64).wrapping_mul(core::mem::size_of::<u8>() as u64) as usize) as usize);
// Reset the stack of refs. We don't track references on the stack
//
// across basic blocks, but the bytecode will expect their
//
// presence. Add dummy references as necessary.
ref_stack_clear(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
i_5 = 0;
while i_5 < (*block).b_startdepth {
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_79 = __SlateRecord_flowgraph_c_2656 { instr: 1i32.wrapping_neg(), local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_79)
}) < 0 {
break '__join_103;
} else {
if true {
break;
}
}
}
i_5 = i_5.wrapping_add(1);
}
i_4 = 0;
while i_4 < (*block).b_iused {
'__join_18: {
'__join_72: {
'__join_66: {
'__join_63: {
'__join_51: {
instr_1 = unsafe { (*block).b_instr }.wrapping_offset(i_4 as isize);
opcode = (*instr_1).i_opcode;
oparg = (*instr_1).i_oparg;
let __t1: i32 = opcode;
if __t1 == 63 {
kill_local(instr_flags, &(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg);
break '__join_18;
} else {
if __t1 == 84 {
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_78 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: oparg };
core::ptr::addr_of_mut!(__slate_compound_78)
}) < 0 {
break '__join_95;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 85 {
kill_local(instr_flags, &(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg);
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_77 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: oparg };
core::ptr::addr_of_mut!(__slate_compound_77)
}) < 0 {
break '__join_91;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 89 {
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_76 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: oparg >> 4i32 };
core::ptr::addr_of_mut!(__slate_compound_76)
}) < 0 {
break '__join_88;
} else {
if true {
break;
}
}
}
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_75 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: oparg & 15 };
core::ptr::addr_of_mut!(__slate_compound_75)
}) < 0 {
break '__join_85;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 112 {
core::ptr::write(__slate_slot_41, ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61))));
store_local(instr_flags, &(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg, *__slate_slot_41);
break '__join_18;
} else {
if __t1 == 113 {
core::ptr::write(__slate_slot_40, ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61))));
store_local(instr_flags, &(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg >> 4i32, *__slate_slot_40);
// LOAD_FAST
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_74 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: oparg & 15 };
core::ptr::addr_of_mut!(__slate_compound_74)
}) < 0 {
break '__join_80;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 114 {
core::ptr::write(__slate_slot_39, ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61))));
store_local(instr_flags, &(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg >> 4i32, *__slate_slot_39);
// STORE_FAST
*__slate_slot_39 = ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
store_local(instr_flags, &(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg & 15, *__slate_slot_39);
break '__join_18;
} else {
if __t1 == 59 {
let idx: i64 = (*__slate_slot_61).size.wrapping_sub(oparg as i64);
core::ptr::write(__slate_slot_37, ref_stack_at(&(*core::ptr::addr_of_mut!(*__slate_slot_61)), idx));
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_73 = __SlateRecord_flowgraph_c_2656 { instr: (*__slate_slot_37).instr, local: (*__slate_slot_37).local };
core::ptr::addr_of_mut!(__slate_compound_73)
}) < 0 {
break '__join_75;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 117 {
ref_stack_swap_top(&(*core::ptr::addr_of_mut!(*__slate_slot_61)), oparg as i64);
break '__join_18;
} else {
if __t1 == 12 {
break '__join_72;
} else {
if __t1 == 15 {
break '__join_72;
} else {
if __t1 == 18 {
break '__join_72;
} else {
if __t1 == 19 {
break '__join_72;
} else {
if __t1 == 72 {
break '__join_72;
} else {
if __t1 == 24 {
break '__join_72;
} else {
if __t1 == 25 {
break '__join_72;
} else {
if __t1 == 26 {
break '__join_72;
} else {
if __t1 == 43 {
break '__join_72;
} else {
if __t1 == 66 {
break '__join_66;
} else {
if __t1 == 67 {
break '__join_66;
} else {
if __t1 == 78 {
break '__join_66;
} else {
if __t1 == 79 {
break '__join_66;
} else {
if __t1 == 98 {
break '__join_66;
} else {
if __t1 == 105 {
break '__join_66;
} else {
if __t1 == 107 {
break '__join_66;
} else {
if __t1 == 109 {
break '__join_66;
} else {
if __t1 == 10 {
break '__join_63;
} else {
if __t1 == 108 {
break '__join_63;
} else {
if __t1 == 6 {
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_70 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_70)
}) < 0 {
break '__join_57;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 70 {
load_fast_push_block(&mut (*core::ptr::addr_of_mut!(sp)), (*instr_1).i_target, (*__slate_slot_61).size.wrapping_add(1));
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_69 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_69)
}) < 0 {
break '__join_53;
} else {
if true {
break '__join_18;
}
}
}
} else {
if __t1 == 80 {
break '__join_51;
} else {
if __t1 == 96 {
break '__join_51;
} else {
if __t1 != 95 {
if __t1 != 32 {
if __t1 == 106 {
load_fast_push_block(&mut (*core::ptr::addr_of_mut!(sp)), (*instr_1).i_target, (*__slate_slot_61).size);
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_64 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_64)
}) < 0 {
break '__join_32;
} else {
if true {
break '__join_18;
}
}
}
} else {
num_popped = _PyOpcode_num_popped(opcode, oparg);
num_pushed = _PyOpcode_num_pushed(opcode, oparg);
if HAS_TARGET!(unsafe { (*instr_1).i_opcode }) {
load_fast_push_block(&mut (*core::ptr::addr_of_mut!(sp)), (*instr_1).i_target, (*__slate_slot_61).size.wrapping_sub(num_popped as i64).wrapping_add(num_pushed as i64));
}
if !IS_BLOCK_PUSH_OPCODE!(unsafe { (*instr_1).i_opcode }) {
j_1 = 0;
while j_1 < num_popped {
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
j_1 = j_1.wrapping_add(1);
}
j = 0;
loop {
if j < num_pushed {
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_63 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_63)
}) < 0 {
break '__join_22;
} else {
if true {
break;
}
}
}
j = j.wrapping_add(1);
} else {
break '__join_18;
}
}
} else {
break '__join_18;
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
core::ptr::write(__slate_slot_22, ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61))));
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_66 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_66)
}) < 0 {
break '__join_39;
} else {
if true {
break;
}
}
}
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_65 = __SlateRecord_flowgraph_c_2656 { instr: (*__slate_slot_22).instr, local: (*__slate_slot_22).local };
core::ptr::addr_of_mut!(__slate_compound_65)
}) < 0 {
break '__join_36;
} else {
if true {
break '__join_18;
}
}
}
}
core::ptr::write(__slate_slot_23, ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61))));
if opcode == 96 {
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
}
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_68 = __SlateRecord_flowgraph_c_2656 { instr: i_4, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_68)
}) < 0 {
break '__join_48;
} else {
if true {
break;
}
}
}
if oparg & 1 != 0 {
// A method call; conservatively assume that self is pushed
//
// back onto the stack
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_67 = __SlateRecord_flowgraph_c_2656 { instr: (*__slate_slot_23).instr, local: (*__slate_slot_23).local };
core::ptr::addr_of_mut!(__slate_compound_67)
}) < 0 {
break '__join_43;
} else {
if true {
break '__join_18;
}
}
}
} else {
break '__join_18;
}
}
core::ptr::write(__slate_slot_24, ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61))));
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_71 = __SlateRecord_flowgraph_c_2656 { instr: (*__slate_slot_24).instr, local: (*__slate_slot_24).local };
core::ptr::addr_of_mut!(__slate_compound_71)
}) < 0 {
break '__join_61;
} else {
if true {
break '__join_18;
}
}
}
}
let num_popped: i32 = _PyOpcode_num_popped(opcode, oparg);
let num_pushed: i32 = _PyOpcode_num_pushed(opcode, oparg);
net_popped = num_popped.wrapping_sub(num_pushed);
i_2 = 0;
loop {
if i_2 < net_popped {
ref_stack_pop(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
i_2 = i_2.wrapping_add(1);
} else {
break '__join_18;
}
}
}
let num_popped: i32 = _PyOpcode_num_popped(opcode, oparg);
let num_pushed: i32 = _PyOpcode_num_pushed(opcode, oparg);
net_pushed = num_pushed.wrapping_sub(num_popped);
i_3 = 0;
while i_3 < net_pushed {
loop {
if ref_stack_push(core::ptr::addr_of_mut!(*__slate_slot_61), *{
__slate_compound_72 = __SlateRecord_flowgraph_c_2656 { instr: i_3, local: 1i32.wrapping_neg() };
core::ptr::addr_of_mut!(__slate_compound_72)
}) < 0 {
break '__join_70;
} else {
if true {
break;
}
}
}
i_3 = i_3.wrapping_add(1);
}
}
i_4 = i_4.wrapping_add(1);
}
// Push fallthrough block
let term: *mut _PyCfgInstruction = basicblock_last_instr(block as *const _PyCfgBasicblock);
if !term.is_null() && !unsafe { (*block).b_next }.is_null() && !(IS_UNCONDITIONAL_JUMP_OPCODE!(unsafe { (*term).i_opcode }) || IS_SCOPE_EXIT_OPCODE!(unsafe { (*term).i_opcode })) {
load_fast_push_block(&mut (*core::ptr::addr_of_mut!(sp)), (*block).b_next, (*__slate_slot_61).size);
}
i_1 = 0;
while i_1 < (*__slate_slot_61).size {
core::ptr::write(__slate_slot_9, ref_stack_at(&(*core::ptr::addr_of_mut!(*__slate_slot_61)), i_1));
if (*__slate_slot_9).instr != 1i32.wrapping_neg() {
let __v8: *mut u8 = instr_flags.wrapping_offset((*__slate_slot_9).instr as isize);
*__v8 = ((*__v8 as i32) | 4) as u8;
}
i_1 = i_1.wrapping_add(1);
}
i = 0;
loop {
if i < (*block).b_iused {
if *instr_flags.wrapping_offset(i as isize) == 0 {
instr = unsafe { (*block).b_instr }.wrapping_offset(i as isize);
let __t0: i32 = (*instr).i_opcode;
if __t0 == 84 {
(*instr).i_opcode = 86;
} else {
if __t0 == 89 {
(*instr).i_opcode = 87;
}
}
}
i = i.wrapping_add(1);
} else {
continue '__loop_2;
}
}
}
                                                                                        status = 0;
                                                                                        break '__join_0;
                                                                                    }
                                                                                    status = 1i32.wrapping_neg();
                                                                                    break '__join_0;
                                                                                }
                                                                                status = 1i32
                                                                                    .wrapping_neg();
                                                                                break '__join_0;
                                                                            }
                                                                            status =
                                                                                1i32.wrapping_neg();
                                                                            break '__join_0;
                                                                        }
                                                                        status =
                                                                            1i32.wrapping_neg();
                                                                        break '__join_0;
                                                                    }
                                                                    status = 1i32.wrapping_neg();
                                                                    break '__join_0;
                                                                }
                                                                status = 1i32.wrapping_neg();
                                                                break '__join_0;
                                                            }
                                                            status = 1i32.wrapping_neg();
                                                            break '__join_0;
                                                        }
                                                        status = 1i32.wrapping_neg();
                                                        break '__join_0;
                                                    }
                                                    status = 1i32.wrapping_neg();
                                                    break '__join_0;
                                                }
                                                status = 1i32.wrapping_neg();
                                                break '__join_0;
                                            }
                                            status = 1i32.wrapping_neg();
                                            break '__join_0;
                                        }
                                        status = 1i32.wrapping_neg();
                                        break '__join_0;
                                    }
                                    status = 1i32.wrapping_neg();
                                    break '__join_0;
                                }
                                status = 1i32.wrapping_neg();
                                break '__join_0;
                            }
                            status = 1i32.wrapping_neg();
                            break '__join_0;
                        }
                        status = 1i32.wrapping_neg();
                        break '__join_0;
                    }
                    status = 1i32.wrapping_neg();
                }
            }
            ref_stack_fini(&mut (*core::ptr::addr_of_mut!(*__slate_slot_61)));
            PyMem_Free(instr_flags as *mut ());
            PyMem_Free(blocks as *mut ());
            return status;
        }
    }
    // Opcodes that load and store locals
    // Opcodes that shuffle values on the stack
    // We treat opcodes that do not consume all of their inputs on
    //
    // a case by case basis, as we have no generic way of knowing
    //
    // how many inputs should be left on the stack.
    // Opcodes that consume no inputs
    // Opcodes that consume some inputs and push no new values
    // Opcodes that consume some inputs and push new values
    // Opcodes that consume all of their inputs
}

/// helper functions for add_checks_for_loads_of_unknown_variables
fn maybe_push(b: *mut _PyCfgBasicblock, unsafe_mask: u64, sp: &mut *mut *mut _PyCfgBasicblock) {
    // Push b if the unsafe mask is giving us any new information.
    //
    // To avoid overflowing the stack, only allow each block once.
    //
    // Use b->b_visited=1 to mean that b is currently on the stack.
    let both: u64 = (unsafe { (*b).b_unsafe_locals_mask }) | unsafe_mask;
    if (unsafe { (*b).b_unsafe_locals_mask }) != both {
        unsafe {
            (*b).b_unsafe_locals_mask = both;
        }
        // More work left to do.
        if ((unsafe { (*b).__slate_bits_0.__get_b_visited() }) as i32) == 0 {
            // not on the stack, so push it.
            let __v1: *mut *mut _PyCfgBasicblock = unsafe { *sp };
            unsafe {
                *sp = __v1.wrapping_offset(1 as isize);
            }
            unsafe {
                *__v1 = b;
            }
            unsafe {
                (*b).__slate_bits_0.__set_b_visited(1u32);
            }
        }
    }
}

fn scan_block_for_locals(b: *mut _PyCfgBasicblock, sp: &mut *mut *mut _PyCfgBasicblock) {
    // bit i is set if local i is potentially uninitialized
    let mut unsafe_mask: u64 = unsafe { (*b).b_unsafe_locals_mask };
    let mut i: i32 = 0;
    while i < unsafe { (*b).b_iused } {
        let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
        if !unsafe { (*instr).i_except }.is_null() {
            maybe_push(unsafe { (*instr).i_except }, unsafe_mask, sp);
        }
        if (unsafe { (*instr).i_oparg }) < 64 {
            let bit: u64 = 1u64 << unsafe { (*instr).i_oparg };
            match unsafe { (*instr).i_opcode } {
                63 | 85 | 266 => {
                    unsafe_mask |= bit;
                }
                112 => {
                    let __v5: u64 = unsafe_mask & !bit;
                    unsafe_mask = __v5;
                }
                88 => {
                    let __v7: u64 = unsafe_mask & !bit;
                    unsafe_mask = __v7; // If this doesn't raise, then the local is defined.
                }
                84 => {
                    if unsafe_mask & bit != 0 {
                        unsafe {
                            (*instr).i_opcode = 88;
                        }
                    }
                    let __v9: u64 = unsafe_mask & !bit;
                    unsafe_mask = __v9;
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
    }
    let __v12: bool = !unsafe { (*b).b_next }.is_null() && BB_HAS_FALLTHROUGH!(b);
    if __v12 {
        maybe_push(unsafe { (*b).b_next }, unsafe_mask, sp);
    }
    let last: *mut _PyCfgInstruction = basicblock_last_instr(b as *const _PyCfgBasicblock);
    let __v13: bool = !last.is_null() && is_jump(last) != 0;
    if __v13 {
        maybe_push(unsafe { (*last).i_target }, unsafe_mask, sp);
    }
}

fn fast_scan_many_locals(mut b: *mut _PyCfgBasicblock, nlocals: i32) -> i32 {
    let states: *mut i64 = (unsafe {
        PyMem_Calloc(
            nlocals.wrapping_sub(64) as u64,
            core::mem::size_of::<i64>() as u64,
        )
    }) as *mut i64;
    if states.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    let mut blocknum: i64 = 0;
    // state[i - 64] == blocknum if local i is guaranteed to
    //
    // be initialized, i.e., if it has had a previous LOAD_FAST or
    //
    // STORE_FAST within that basicblock (not followed by
    //
    // DELETE_FAST/LOAD_FAST_AND_CLEAR/STORE_FAST_MAYBE_NULL).
    while !b.is_null() {
        blocknum = blocknum.wrapping_add(1);
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            let arg: i32 = unsafe { (*instr).i_oparg };
            if arg >= 64 {
                '__slate_break_3: {
                    match unsafe { (*instr).i_opcode } {
                        63 | 85 | 266 => unsafe {
                            *states.wrapping_offset(arg.wrapping_sub(64) as isize) =
                                blocknum.wrapping_sub(1);
                        },
                        112 => unsafe {
                            *states.wrapping_offset(arg.wrapping_sub(64) as isize) = blocknum;
                        },
                        84 => {
                            if (unsafe { *states.wrapping_offset(arg.wrapping_sub(64) as isize) })
                                != blocknum
                            {
                                unsafe {
                                    (*instr).i_opcode = 88;
                                }
                            }
                            unsafe {
                                *states.wrapping_offset(arg.wrapping_sub(64) as isize) = blocknum;
                            }
                            break '__slate_break_3;
                            Py_UNREACHABLE!();
                        }
                        _ => {}
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    unsafe { PyMem_Free(states as *mut ()) };
    0
}

fn remove_unused_consts(entryblock: *mut _PyCfgBasicblock, consts: *mut _object) -> i32 {
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut b: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut i_2: i64 = unsafe { core::mem::zeroed() };
    let mut i_3: i64 = unsafe { core::mem::zeroed() };
    let mut i_4: i64 = unsafe { core::mem::zeroed() };
    // now index_map[i] == i if consts[i] is used, -1 otherwise
    //
    // condense consts
    let mut n_used_consts: i64 = unsafe { core::mem::zeroed() };
    let mut i_5: i32 = unsafe { core::mem::zeroed() };
    // mark used consts
    let mut b_1: *mut _PyCfgBasicblock = unsafe { core::mem::zeroed() };
    let mut i_6: i64 = unsafe { core::mem::zeroed() };
    let mut err: i32 = unsafe { core::mem::zeroed() };
    let mut reverse_index_map: *mut i64 = unsafe { core::mem::zeroed() };
    let mut index_map: *mut i64 = unsafe { core::mem::zeroed() };
    let mut nconsts: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        nconsts = PyList_GET_SIZE!(consts);
        if nconsts == 0 {
            return 0;
        } else {
            index_map = core::ptr::null_mut::<i64>();
            reverse_index_map = core::ptr::null_mut::<i64>();
            err = 1i32.wrapping_neg();
            index_map =
                PyMem_Malloc((nconsts as u64).wrapping_mul(core::mem::size_of::<i64>() as u64))
                    as *mut i64;
            if index_map.is_null() {
                PyErr_NoMemory();
            } else {
                i_6 = 1;
                while i_6 < nconsts {
                    *index_map.wrapping_offset(i_6 as isize) = 1i32.wrapping_neg() as i64;
                    i_6 = i_6.wrapping_add(1);
                }
                // The first constant may be docstring; keep it always.
                *index_map.wrapping_offset(0 as isize) = 0;
                b_1 = entryblock;
                while !b_1.is_null() {
                    i_5 = 0;
                    while i_5 < (*b_1).b_iused {
                        let opcode: i32 =
                            (*unsafe { (*b_1).b_instr }.wrapping_offset(i_5 as isize)).i_opcode;
                        if OPCODE_HAS_CONST!(opcode) != 0 {
                            let index: i32 =
                                (*unsafe { (*b_1).b_instr }.wrapping_offset(i_5 as isize)).i_oparg;
                            *index_map.wrapping_offset(index as isize) = index as i64;
                        }
                        i_5 = i_5.wrapping_add(1);
                    }
                    b_1 = (*b_1).b_next;
                }
                n_used_consts = 0;
                i_4 = 0;
                while i_4 < nconsts {
                    if *index_map.wrapping_offset(i_4 as isize) != (1i32.wrapping_neg() as i64) {
                        let __v20: i64 = n_used_consts;
                        n_used_consts = __v20.wrapping_add(1);
                        *index_map.wrapping_offset(__v20 as isize) =
                            *index_map.wrapping_offset(i_4 as isize);
                    }
                    i_4 = i_4.wrapping_add(1);
                }
                if n_used_consts == nconsts {
                    // nothing to do
                    err = 0;
                } else {
                    // move all used consts to the beginning of the consts list
                    i_3 = 0;
                    while i_3 < n_used_consts {
                        let old_index: i64 = *index_map.wrapping_offset(i_3 as isize);
                        if i_3 != old_index {
                            let value: *mut _object = PyList_GET_ITEM!(consts, unsafe {
                                *index_map.wrapping_offset(i_3 as isize)
                            });
                            PyList_SetItem(consts, i_3, Py_NewRef!(value));
                        }
                        i_3 = i_3.wrapping_add(1);
                    }
                    // truncate the consts list at its new size
                    if PyList_SetSlice(
                        consts,
                        n_used_consts,
                        nconsts,
                        core::ptr::null_mut::<_object>(),
                    ) >= 0
                    {
                        // adjust const indices in the bytecode
                        reverse_index_map = PyMem_Malloc(
                            (nconsts as u64).wrapping_mul(core::mem::size_of::<i64>() as u64),
                        ) as *mut i64;
                        if reverse_index_map.is_null() {
                            PyErr_NoMemory();
                        } else {
                            i_2 = 0;
                            while i_2 < nconsts {
                                *reverse_index_map.wrapping_offset(i_2 as isize) =
                                    1i32.wrapping_neg() as i64;
                                i_2 = i_2.wrapping_add(1);
                            }
                            i_1 = 0;
                            while i_1 < n_used_consts {
                                *reverse_index_map.wrapping_offset(
                                    *index_map.wrapping_offset(i_1 as isize) as isize,
                                ) = i_1;
                                i_1 = i_1.wrapping_add(1);
                            }
                            b = entryblock;
                            while !b.is_null() {
                                i = 0;
                                while i < (*b).b_iused {
                                    let opcode: i32 = (*unsafe { (*b).b_instr }
                                        .wrapping_offset(i as isize))
                                    .i_opcode;
                                    if OPCODE_HAS_CONST!(opcode) != 0 {
                                        let index: i32 = (*unsafe { (*b).b_instr }
                                            .wrapping_offset(i as isize))
                                        .i_oparg;
                                        (*unsafe { (*b).b_instr }.wrapping_offset(i as isize))
                                            .i_oparg = *reverse_index_map
                                            .wrapping_offset(index as isize)
                                            as i32;
                                    }
                                    i = i.wrapping_add(1);
                                }
                                b = (*b).b_next;
                            }
                            err = 0;
                        }
                    }
                }
            }
            PyMem_Free(index_map as *mut ());
            PyMem_Free(reverse_index_map as *mut ());
            return err;
        }
    }
    // nothing to do
}

fn add_checks_for_loads_of_uninitialized_variables(
    entryblock: *mut _PyCfgBasicblock,
    mut nlocals: i32,
    nparams: i32,
) -> i32 {
    if nlocals == 0 {
        return 0;
    }
    if nlocals > 64 {
        // To avoid O(nlocals**2) compilation, locals beyond the first
        //
        // 64 are only analyzed one basicblock at a time: initialization
        //
        // info is not passed between basicblocks.
        if fast_scan_many_locals(entryblock, nlocals) < 0 {
            return 1i32.wrapping_neg();
        }
        nlocals = 64;
    }
    let stack: *mut *mut _PyCfgBasicblock = make_cfg_traversal_stack(entryblock);
    if stack.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut sp: *mut *mut _PyCfgBasicblock = stack;
    // First origin of being uninitialized:
    //
    // The non-parameter locals in the entry block.
    let mut start_mask: u64 = 0;
    for i in nparams..nlocals {
        start_mask |= 1u64 << i;
    }
    maybe_push(entryblock, start_mask, unsafe {
        &mut (*core::ptr::addr_of_mut!(sp))
    });
    // Second origin of being uninitialized:
    //
    // There could be DELETE_FAST somewhere, so
    //
    // be sure to scan each basicblock at least once.
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            scan_block_for_locals(b, unsafe { &mut (*core::ptr::addr_of_mut!(sp)) });
            b = unsafe { (*b).b_next };
        }
    }
    // Now propagate the uncertainty from the origins we found: Use
    //
    // LOAD_FAST_CHECK for any LOAD_FAST where the local could be undefined.
    while sp > stack {
        let __v4: *mut *mut _PyCfgBasicblock = sp;
        let __v5: *mut *mut _PyCfgBasicblock = __v4.wrapping_offset(-(1 as isize));
        sp = __v5;
        let b: *mut _PyCfgBasicblock = unsafe { *__v5 };
        // mark as no longer on stack
        unsafe {
            (*b).__slate_bits_0.__set_b_visited(0u32);
        }
        scan_block_for_locals(b, unsafe { &mut (*core::ptr::addr_of_mut!(sp)) });
    }
    unsafe { PyMem_Free(stack as *mut ()) };
    0
}

fn mark_warm(entryblock: *mut _PyCfgBasicblock) -> i32 {
    let stack: *mut *mut _PyCfgBasicblock = make_cfg_traversal_stack(entryblock);
    if stack.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut sp: *mut *mut _PyCfgBasicblock = stack;
    let __v0: *mut *mut _PyCfgBasicblock = sp;
    sp = __v0.wrapping_offset(1 as isize);
    unsafe {
        *__v0 = entryblock;
    }
    unsafe {
        (*entryblock).__slate_bits_0.__set_b_visited(1u32);
    }
    while sp > stack {
        let __v2: *mut *mut _PyCfgBasicblock = sp;
        let __v3: *mut *mut _PyCfgBasicblock = __v2.wrapping_offset(-(1 as isize));
        sp = __v3;
        let b: *mut _PyCfgBasicblock = unsafe { *__v3 };
        unsafe {
            (*b).__slate_bits_0.__set_b_warm(1u32);
        }
        let next: *mut _PyCfgBasicblock = unsafe { (*b).b_next };
        let __v4: bool = !next.is_null() && BB_HAS_FALLTHROUGH!(b);
        if __v4 && ((unsafe { (*next).__slate_bits_0.__get_b_visited() }) as i32) == 0 {
            let __v5: *mut *mut _PyCfgBasicblock = sp;
            sp = __v5.wrapping_offset(1 as isize);
            unsafe {
                *__v5 = next;
            }
            unsafe {
                (*next).__slate_bits_0.__set_b_visited(1u32);
            }
        }
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if is_jump(instr) != 0
                && ((unsafe { (*(*instr).i_target).__slate_bits_0.__get_b_visited() }) as i32) == 0
            {
                let __v7: *mut *mut _PyCfgBasicblock = sp;
                sp = __v7.wrapping_offset(1 as isize);
                unsafe {
                    *__v7 = (*instr).i_target;
                }
                unsafe {
                    (*(*instr).i_target).__slate_bits_0.__set_b_visited(1u32);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    unsafe { PyMem_Free(stack as *mut ()) };
    0
}

fn mark_cold(entryblock: *mut _PyCfgBasicblock) -> i32 {
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            b = unsafe { (*b).b_next };
        }
    }
    if mark_warm(entryblock) < 0 {
        return 1i32.wrapping_neg();
    }
    let stack: *mut *mut _PyCfgBasicblock = make_cfg_traversal_stack(entryblock);
    if stack.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut sp: *mut *mut _PyCfgBasicblock = stack;
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            if ((unsafe { (*b).__slate_bits_0.__get_b_except_handler() }) as i32) != 0 {
                let __v1: *mut *mut _PyCfgBasicblock = sp;
                sp = __v1.wrapping_offset(1 as isize);
                unsafe {
                    *__v1 = b;
                }
                unsafe {
                    (*b).__slate_bits_0.__set_b_visited(1u32);
                }
            }
            b = unsafe { (*b).b_next };
        }
    }
    while sp > stack {
        let __v4: *mut *mut _PyCfgBasicblock = sp;
        let __v5: *mut *mut _PyCfgBasicblock = __v4.wrapping_offset(-(1 as isize));
        sp = __v5;
        let b: *mut _PyCfgBasicblock = unsafe { *__v5 };
        unsafe {
            (*b).__slate_bits_0.__set_b_cold(1u32);
        }
        let next: *mut _PyCfgBasicblock = unsafe { (*b).b_next };
        let __v6: bool = !next.is_null() && BB_HAS_FALLTHROUGH!(b);
        if __v6 {
            if ((unsafe { (*next).__slate_bits_0.__get_b_warm() }) as i32) == 0
                && ((unsafe { (*next).__slate_bits_0.__get_b_visited() }) as i32) == 0
            {
                let __v7: *mut *mut _PyCfgBasicblock = sp;
                sp = __v7.wrapping_offset(1 as isize);
                unsafe {
                    *__v7 = next;
                }
                unsafe {
                    (*next).__slate_bits_0.__set_b_visited(1u32);
                }
            }
        }
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if is_jump(instr) != 0 {
                let target: *mut _PyCfgBasicblock =
                    unsafe { (*unsafe { (*b).b_instr }.wrapping_offset(i as isize)).i_target };
                if ((unsafe { (*target).__slate_bits_0.__get_b_warm() }) as i32) == 0
                    && ((unsafe { (*target).__slate_bits_0.__get_b_visited() }) as i32) == 0
                {
                    let __v9: *mut *mut _PyCfgBasicblock = sp;
                    sp = __v9.wrapping_offset(1 as isize);
                    unsafe {
                        *__v9 = target;
                    }
                    unsafe {
                        (*target).__slate_bits_0.__set_b_visited(1u32);
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    unsafe { PyMem_Free(stack as *mut ()) };
    0
}

fn push_cold_blocks_to_end(g: *mut _PyCfgBuilder) -> i32 {
    let mut __slate_compound_1: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let entryblock: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    if unsafe { (*entryblock).b_next }.is_null() {
        // single basicblock, no need to reorder
        return 0;
    }
    RETURN_IF_ERROR__2!(mark_cold(entryblock));
    let mut next_lbl: i32 = get_max_label(unsafe { (*g).g_entryblock }).wrapping_add(1);
    // If we have a cold block with fallthrough to a warm block, add
    //
    // an explicit jump instead of fallthrough
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        '__slate_break_4: while !b.is_null() {
            let __v0: bool = ((unsafe { (*b).__slate_bits_0.__get_b_cold() }) as i32) != 0
                && BB_HAS_FALLTHROUGH!(b);
            if __v0
                && !unsafe { (*b).b_next }.is_null()
                && ((unsafe { (*(*b).b_next).__slate_bits_0.__get_b_warm() }) as i32) != 0
            {
                let explicit_jump: *mut _PyCfgBasicblock = cfg_builder_new_block(g);
                if explicit_jump.is_null() {
                    return 1i32.wrapping_neg();
                }
                if (unsafe { (*(*b).b_next).b_label.id })
                    == unsafe {
                        (*{
                            __slate_compound_1 = __SlateRecord_pycore_instruction_sequence_h_55 {
                                id: 1i32.wrapping_neg(),
                            };
                            core::ptr::addr_of_mut!(__slate_compound_1)
                        })
                        .id
                    }
                {
                    let __v2: i32 = next_lbl;
                    next_lbl = __v2.wrapping_add(1);
                    unsafe {
                        (*(*b).b_next).b_label.id = __v2;
                    }
                }
                basicblock_addop(
                    explicit_jump,
                    260i32,
                    unsafe { (*(*b).b_next).b_label.id },
                    unsafe { NO_LOCATION },
                );
                unsafe {
                    (*explicit_jump).__slate_bits_0.__set_b_cold(1u32);
                }
                unsafe {
                    (*explicit_jump).b_next = (*b).b_next;
                }
                unsafe {
                    (*explicit_jump).b_predecessors = 1;
                }
                unsafe {
                    (*b).b_next = explicit_jump;
                }
                // set target
                let last: *mut _PyCfgInstruction =
                    basicblock_last_instr(explicit_jump as *const _PyCfgBasicblock);
                unsafe {
                    (*last).i_target = (*explicit_jump).b_next;
                }
            }
            b = unsafe { (*b).b_next };
        }
    }
    // First block can't be cold
    let mut cold_blocks: *mut _PyCfgBasicblock = core::ptr::null_mut::<_PyCfgBasicblock>();
    let mut cold_blocks_tail: *mut _PyCfgBasicblock = core::ptr::null_mut::<_PyCfgBasicblock>();
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !unsafe { (*b).b_next }.is_null() {
        while !unsafe { (*b).b_next }.is_null()
            && ((unsafe { (*(*b).b_next).__slate_bits_0.__get_b_cold() }) as i32) == 0
        {
            b = unsafe { (*b).b_next };
        }
        if unsafe { (*b).b_next }.is_null() {
            // no more cold blocks
            break;
        }
        // b->b_next is the beginning of a cold streak
        let mut b_end: *mut _PyCfgBasicblock = unsafe { (*b).b_next };
        while !unsafe { (*b_end).b_next }.is_null()
            && ((unsafe { (*(*b_end).b_next).__slate_bits_0.__get_b_cold() }) as i32) != 0
        {
            b_end = unsafe { (*b_end).b_next };
        }
        // b_end is the end of the cold streak
        if cold_blocks.is_null() {
            cold_blocks = unsafe { (*b).b_next };
        } else {
            unsafe {
                (*cold_blocks_tail).b_next = (*b).b_next;
            }
        }
        cold_blocks_tail = b_end;
        unsafe {
            (*b).b_next = (*b_end).b_next;
        }
        unsafe {
            (*b_end).b_next = core::ptr::null_mut::<_PyCfgBasicblock>();
        }
    }
    unsafe {
        (*b).b_next = cold_blocks;
    }
    if !cold_blocks.is_null() {
        RETURN_IF_ERROR__2!(remove_redundant_nops_and_jumps(g));
    }
    0
}

fn convert_pseudo_conditional_jumps(g: *mut _PyCfgBuilder) -> i32 {
    let entryblock: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !b.is_null() {
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if (unsafe { (*instr).i_opcode }) == 258 || (unsafe { (*instr).i_opcode }) == 259 {
                unsafe {
                    (*instr).i_opcode = if (*instr).i_opcode == 258 { 100 } else { 103 };
                }
                let loc: __SlateRecord_pycore_symtable_h_51 = unsafe { (*instr).i_loc };
                let except: *mut _PyCfgBasicblock = unsafe { (*instr).i_except };
                let mut copy: _PyCfgInstruction = _PyCfgInstruction {
                    i_opcode: 59,
                    i_oparg: 1,
                    i_loc: loc,
                    i_target: core::ptr::null_mut::<_PyCfgBasicblock>(),
                    i_except: except,
                };
                let __v0: i32 = i;
                i = __v0.wrapping_add(1);
                if basicblock_insert_instruction(b, __v0, core::ptr::addr_of_mut!(copy))
                    == 1i32.wrapping_neg()
                {
                    return 1i32.wrapping_neg();
                }
                let mut to_bool: _PyCfgInstruction = _PyCfgInstruction {
                    i_opcode: 39,
                    i_oparg: 0,
                    i_loc: loc,
                    i_target: core::ptr::null_mut::<_PyCfgBasicblock>(),
                    i_except: except,
                };
                let __v2: i32 = i;
                i = __v2.wrapping_add(1);
                if basicblock_insert_instruction(b, __v2, core::ptr::addr_of_mut!(to_bool))
                    == 1i32.wrapping_neg()
                {
                    return 1i32.wrapping_neg();
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    0
}

fn convert_pseudo_ops(g: *mut _PyCfgBuilder) -> i32 {
    let entryblock: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !b.is_null() {
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if is_block_push(instr) != 0 {
                INSTR_SET_OP0!(instr, 27i32);
            } else {
                if (unsafe { (*instr).i_opcode }) == 261 {
                    unsafe {
                        (*instr).i_opcode = 84;
                    }
                } else {
                    if (unsafe { (*instr).i_opcode }) == 266 {
                        unsafe {
                            (*instr).i_opcode = 112;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    remove_redundant_nops_and_jumps(g)
}

fn is_exit_or_eval_check_without_lineno(b: *mut _PyCfgBasicblock) -> bool {
    let __v0: bool = basicblock_exits_scope(b as *const _PyCfgBasicblock) != 0
        || basicblock_has_eval_break(b as *const _PyCfgBasicblock) != 0;
    if __v0 {
        return basicblock_has_no_lineno(b);
    } else {
        return false;
    }
}

/// PEP 626 mandates that the f_lineno of a frame is correct
/// after a frame terminates. It would be prohibitively expensive
/// to continuously update the f_lineno field at runtime,
/// so we make sure that all exiting instruction (raises and returns)
/// have a valid line number, allowing us to compute f_lineno lazily.
/// We can do this by duplicating the exit blocks without line number
/// so that none have more than one predecessor. We can then safely
/// copy the line number from the sole predecessor block.
fn duplicate_exits_without_lineno(g: *mut _PyCfgBuilder) -> i32 {
    let mut next_lbl: i32 = get_max_label(unsafe { (*g).g_entryblock }).wrapping_add(1);
    // Copy all exit blocks without line number that are targets of a jump.
    let entryblock: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    {
        let mut b: *mut _PyCfgBasicblock = entryblock;
        while !b.is_null() {
            let last: *mut _PyCfgInstruction = basicblock_last_instr(b as *const _PyCfgBasicblock);
            if !last.is_null() {
                if is_jump(last) != 0 {
                    let target: *mut _PyCfgBasicblock =
                        next_nonempty_block(unsafe { (*last).i_target });
                    if is_exit_or_eval_check_without_lineno(target)
                        && (unsafe { (*target).b_predecessors }) > 1
                    {
                        let new_target: *mut _PyCfgBasicblock = copy_basicblock(g, target);
                        if new_target.is_null() {
                            return 1i32.wrapping_neg();
                        }
                        unsafe {
                            (*unsafe { (*new_target).b_instr }.wrapping_offset(0 as isize)).i_loc =
                                (*last).i_loc;
                        }
                        unsafe {
                            (*last).i_target = new_target;
                        }
                        let __v2: i32 = unsafe { (*target).b_predecessors };
                        unsafe {
                            (*target).b_predecessors = __v2.wrapping_sub(1);
                        }
                        unsafe {
                            (*new_target).b_predecessors = 1;
                        }
                        unsafe {
                            (*new_target).b_next = (*target).b_next;
                        }
                        let __v4: i32 = next_lbl;
                        next_lbl = __v4.wrapping_add(1);
                        unsafe {
                            (*new_target).b_label.id = __v4;
                        }
                        unsafe {
                            (*target).b_next = new_target;
                        }
                    }
                }
            }
            b = unsafe { (*b).b_next };
        }
    }
    // Any remaining reachable exit blocks without line number can only be reached by
    // fall through, and thus can only have a single predecessor
    let mut b: *mut _PyCfgBasicblock = entryblock;
    while !b.is_null() {
        if BB_HAS_FALLTHROUGH!(b)
            && !unsafe { (*b).b_next }.is_null()
            && (unsafe { (*b).b_iused }) > 0
        {
            if is_exit_or_eval_check_without_lineno(unsafe { (*b).b_next }) {
                let last: *mut _PyCfgInstruction =
                    basicblock_last_instr(b as *const _PyCfgBasicblock);
                unsafe {
                    (*unsafe { (*(*b).b_next).b_instr }.wrapping_offset(0 as isize)).i_loc =
                        (*last).i_loc;
                }
            }
        }
        b = unsafe { (*b).b_next };
    }
    0
}

/// If an instruction has no line number, but it's predecessor in the BB does,
/// then copy the line number. If a successor block has no line number, and only
/// one predecessor, then inherit the line number.
/// This ensures that all exit blocks (with one predecessor) receive a line number.
/// Also reduces the size of the line number table,
/// but has no impact on the generated line number events.
fn propagate_line_numbers(mut b: *mut _PyCfgBasicblock) {
    while !b.is_null() {
        let last: *mut _PyCfgInstruction = basicblock_last_instr(b as *const _PyCfgBasicblock);
        if !last.is_null() {
            let mut prev_location: __SlateRecord_pycore_symtable_h_51 = unsafe { NO_LOCATION };
            let mut i: i32 = 0;
            while i < unsafe { (*b).b_iused } {
                if (unsafe {
                    (*unsafe { (*b).b_instr }.wrapping_offset(i as isize))
                        .i_loc
                        .lineno
                }) == unsafe { NO_LOCATION.lineno }
                {
                    unsafe {
                        (*unsafe { (*b).b_instr }.wrapping_offset(i as isize)).i_loc =
                            prev_location;
                    }
                } else {
                    prev_location =
                        unsafe { (*unsafe { (*b).b_instr }.wrapping_offset(i as isize)).i_loc };
                }
                i = i.wrapping_add(1);
            }
            if BB_HAS_FALLTHROUGH!(b) && (unsafe { (*(*b).b_next).b_predecessors }) == 1 {
                if (unsafe { (*(*b).b_next).b_iused }) > 0 {
                    if (unsafe {
                        (*unsafe { (*(*b).b_next).b_instr }.wrapping_offset(0 as isize))
                            .i_loc
                            .lineno
                    }) == unsafe { NO_LOCATION.lineno }
                    {
                        unsafe {
                            (*unsafe { (*(*b).b_next).b_instr }.wrapping_offset(0 as isize))
                                .i_loc = prev_location;
                        }
                    }
                }
            }
            if is_jump(last) != 0 {
                let target: *mut _PyCfgBasicblock = unsafe { (*last).i_target };
                if (unsafe { (*target).b_predecessors }) == 1 {
                    if (unsafe {
                        (*unsafe { (*target).b_instr }.wrapping_offset(0 as isize))
                            .i_loc
                            .lineno
                    }) == unsafe { NO_LOCATION.lineno }
                    {
                        unsafe {
                            (*unsafe { (*target).b_instr }.wrapping_offset(0 as isize)).i_loc =
                                prev_location;
                        }
                    }
                }
            }
        }
        b = unsafe { (*b).b_next };
    }
}

fn resolve_line_numbers(g: *mut _PyCfgBuilder, firstlineno: i32) -> i32 {
    RETURN_IF_ERROR__2!(duplicate_exits_without_lineno(g));
    propagate_line_numbers(unsafe { (*g).g_entryblock });
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfg_OptimizeCodeUnit(
    g: *mut _PyCfgBuilder,
    consts: *mut _object,
    const_cache: *mut _object,
    nlocals: i32,
    nparams: i32,
    firstlineno: i32,
) -> i32 {
    // Preprocessing
    //
    // Map labels to targets and mark exception handlers
    RETURN_IF_ERROR__2!(translate_jump_labels_to_targets(unsafe {
        (*g).g_entryblock
    }));
    RETURN_IF_ERROR__2!(mark_except_handlers(unsafe { (*g).g_entryblock }));
    RETURN_IF_ERROR__2!(label_exception_targets(unsafe { (*g).g_entryblock }));
    // Optimization
    let consts_index: *mut _Py_hashtable_t = unsafe {
        _Py_hashtable_new(
            unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const ()) -> u64>,
                >(_Py_hashtable_hash_ptr as *const ())
            },
            unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
                >(_Py_hashtable_compare_direct as *const ())
            },
        )
    };
    if consts_index.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    let mut i: i64 = 0;
    while i < PyList_GET_SIZE!(consts) {
        let item: *mut _object = PyList_GET_ITEM!(consts, i);
        if _Py_hashtable_get_entry(consts_index, item as *const ()).is_null() {
            if (unsafe {
                _Py_hashtable_set(consts_index, item as *const (), (i as u64) as *mut ())
            }) < 0
            {
                unsafe { _Py_hashtable_destroy(consts_index) };
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg();
            }
        }
        i = i.wrapping_add(1);
    }
    let ret: i32 = optimize_cfg(g, consts, const_cache, consts_index, firstlineno);
    unsafe { _Py_hashtable_destroy(consts_index) };
    RETURN_IF_ERROR__2!(ret);
    RETURN_IF_ERROR__2!(remove_unused_consts(unsafe { (*g).g_entryblock }, consts));
    RETURN_IF_ERROR__2!(add_checks_for_loads_of_uninitialized_variables(
        unsafe { (*g).g_entryblock },
        nlocals,
        nparams
    ));
    RETURN_IF_ERROR__2!(insert_superinstructions(g));
    RETURN_IF_ERROR__2!(push_cold_blocks_to_end(g));
    RETURN_IF_ERROR__2!(resolve_line_numbers(g, firstlineno));
    // temporarily remove assert. See https://github.com/python/cpython/issues/125845
    //
    // assert(all_exits_have_lineno(g->g_entryblock));
    0
}

fn build_cellfixedoffsets(umd: *mut __SlateRecord_pycore_compile_h_78) -> *mut i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut oldindex: i32 = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut argoffset: i32 = unsafe { core::mem::zeroed() };
    let mut varindex: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut cellindex: *mut _object = unsafe { core::mem::zeroed() };
    let mut varname: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut fixed: *mut i32 = unsafe { core::mem::zeroed() };
    let mut noffsets: i32 = unsafe { core::mem::zeroed() };
    let mut nlocals: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        nlocals = PyDict_GET_SIZE!(unsafe { (*umd).u_varnames }) as i32;
        let ncellvars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_cellvars }) as i32;
        let nfreevars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_freevars }) as i32;
        noffsets = ncellvars.wrapping_add(nfreevars);
        fixed = PyMem_New__3!(i32, noffsets);
        if fixed.is_null() {
            PyErr_NoMemory();
            return core::ptr::null_mut::<i32>();
        } else {
            i = 0;
            while i < noffsets {
                *fixed.wrapping_offset(i as isize) = nlocals.wrapping_add(i);
                i = i.wrapping_add(1);
            }
            pos = 0;
            '__join_0: {
                while PyDict_Next(
                    (*umd).u_cellvars,
                    core::ptr::addr_of_mut!(pos),
                    core::ptr::addr_of_mut!(varname),
                    core::ptr::addr_of_mut!(cellindex),
                ) != 0
                {
                    if PyDict_GetItemRef(
                        (*umd).u_varnames,
                        varname,
                        core::ptr::addr_of_mut!(varindex),
                    ) < 0
                    {
                        break '__join_0;
                    } else {
                        if !varindex.is_null() {
                            argoffset = PyLong_AsInt(varindex);
                            Py_DECREF!(varindex);
                            if argoffset == 1i32.wrapping_neg() {
                                __v2 = !PyErr_Occurred().is_null();
                            } else {
                                __v2 = false;
                            }
                            if __v2 {
                                break '__join_0;
                            } else {
                                oldindex = PyLong_AsInt(cellindex);
                                if oldindex == 1i32.wrapping_neg() {
                                    __v0 = !PyErr_Occurred().is_null();
                                } else {
                                    __v0 = false;
                                }
                                if __v0 {
                                    break '__join_0;
                                } else {
                                    *fixed.wrapping_offset(oldindex as isize) = argoffset;
                                }
                            }
                        }
                    }
                }
                return fixed;
            }
            PyMem_Free(fixed as *mut ());
            return core::ptr::null_mut::<i32>();
        }
    }
}

fn insert_prefix_instructions(
    umd: *mut __SlateRecord_pycore_compile_h_78,
    entryblock: *mut _PyCfgBasicblock,
    fixed: *mut i32,
    nfreevars: i32,
    code_flags: i32,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_symtable_h_51 = unsafe { core::mem::zeroed() };
    // Add the generator prefix instructions.
    if IS_GENERATOR!(code_flags) != 0 {
        // Note that RETURN_GENERATOR + POP_TOP have a net stack effect
        // of 0. This is because RETURN_GENERATOR pushes an element
        // with _PyFrame_StackPush before switching stacks.
        let loc: __SlateRecord_pycore_symtable_h_51 = unsafe {
            *{
                __slate_compound_0 = __SlateRecord_pycore_symtable_h_51 {
                    lineno: (*umd).u_firstlineno,
                    end_lineno: (*umd).u_firstlineno,
                    col_offset: 1i32.wrapping_neg(),
                    end_col_offset: 1i32.wrapping_neg(),
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
        let mut make_gen: _PyCfgInstruction = _PyCfgInstruction {
            i_opcode: 34,
            i_oparg: 0,
            i_loc: loc,
            i_target: core::ptr::null_mut::<_PyCfgBasicblock>(),
            i_except: core::ptr::null_mut::<_PyCfgBasicblock>(),
        };
        RETURN_IF_ERROR__2!(basicblock_insert_instruction(
            entryblock,
            0i32,
            core::ptr::addr_of_mut!(make_gen)
        ));
        let mut pop_top: _PyCfgInstruction = _PyCfgInstruction {
            i_opcode: 31,
            i_oparg: 0,
            i_loc: loc,
            i_target: core::ptr::null_mut::<_PyCfgBasicblock>(),
            i_except: core::ptr::null_mut::<_PyCfgBasicblock>(),
        };
        RETURN_IF_ERROR__2!(basicblock_insert_instruction(
            entryblock,
            1i32,
            core::ptr::addr_of_mut!(pop_top)
        ));
    }
    // Set up cells for any variable that escapes, to be put in a closure.
    let ncellvars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_cellvars }) as i32;
    if ncellvars != 0 {
        // umd->u_cellvars has the cells out of order so we sort them
        //
        // before adding the MAKE_CELL instructions.  Note that we
        //
        // adjust for arg cells, which come first.
        let nvars: i32 =
            ncellvars.wrapping_add(PyDict_GET_SIZE!(unsafe { (*umd).u_varnames }) as i32);
        let sorted: *mut i32 =
            (unsafe { PyMem_RawCalloc(nvars as u64, core::mem::size_of::<i32>() as u64) })
                as *mut i32;
        if sorted.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
        for i in 0..ncellvars {
            unsafe {
                *sorted.wrapping_offset(*fixed.wrapping_offset(i as isize) as isize) =
                    i.wrapping_add(1);
            }
        }
        let mut i: i32 = 0;
        let mut ncellsused: i32 = 0;
        while ncellsused < ncellvars {
            let oldindex: i32 = unsafe { *sorted.wrapping_offset(i as isize) }.wrapping_sub(1);
            if oldindex != 1i32.wrapping_neg() {
                let mut make_cell: _PyCfgInstruction = _PyCfgInstruction {
                    i_opcode: 97,
                    i_oparg: oldindex,
                    i_loc: unsafe { NO_LOCATION },
                    i_target: core::ptr::null_mut::<_PyCfgBasicblock>(),
                    i_except: core::ptr::null_mut::<_PyCfgBasicblock>(),
                }; // This will get fixed in offset_derefs().
                if basicblock_insert_instruction(
                    entryblock,
                    ncellsused,
                    core::ptr::addr_of_mut!(make_cell),
                ) < 0
                {
                    unsafe { PyMem_RawFree(sorted as *mut ()) };
                    return 1i32.wrapping_neg();
                }
                ncellsused = ncellsused.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        unsafe { PyMem_RawFree(sorted as *mut ()) };
    }
    if nfreevars != 0 {
        let mut copy_frees: _PyCfgInstruction = _PyCfgInstruction {
            i_opcode: 60,
            i_oparg: nfreevars,
            i_loc: unsafe { NO_LOCATION },
            i_target: core::ptr::null_mut::<_PyCfgBasicblock>(),
            i_except: core::ptr::null_mut::<_PyCfgBasicblock>(),
        };
        RETURN_IF_ERROR__2!(basicblock_insert_instruction(
            entryblock,
            0i32,
            core::ptr::addr_of_mut!(copy_frees)
        ));
    }
    0
}

fn fix_cell_offsets(
    umd: *mut __SlateRecord_pycore_compile_h_78,
    mut b: *mut _PyCfgBasicblock,
    fixedmap: *mut i32,
) -> i32 {
    let nlocals: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_varnames }) as i32;
    let ncellvars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_cellvars }) as i32;
    let nfreevars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_freevars }) as i32;
    let noffsets: i32 = ncellvars.wrapping_add(nfreevars);
    // First deal with duplicates (arg cells).
    let mut numdropped: i32 = 0;
    for i in 0..noffsets {
        if (unsafe { *fixedmap.wrapping_offset(i as isize) }) == i.wrapping_add(nlocals) {
            let __v0: *mut i32 = fixedmap.wrapping_offset(i as isize);
            let __v1: i32 = unsafe { *__v0 };
            unsafe {
                *__v0 = __v1.wrapping_sub(numdropped);
            }
        } else {
            // It was a duplicate (cell/arg).
            numdropped = numdropped.wrapping_add(1);
        }
    }
    // Then update offsets, either relative to locals or by cell2arg.
    while !b.is_null() {
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let inst: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            // This is called before extended args are generated.
            let oldoffset: i32 = unsafe { (*inst).i_oparg };
            match unsafe { (*inst).i_opcode } {
                62 | 83 | 90 | 97 | 111 | 261 => unsafe {
                    (*inst).i_oparg = *fixedmap.wrapping_offset(oldoffset as isize);
                },
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    numdropped
}

fn prepare_localsplus(
    umd: *mut __SlateRecord_pycore_compile_h_78,
    g: *mut _PyCfgBuilder,
    code_flags: i32,
) -> i32 {
    let nlocals: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_varnames }) as i32;
    let ncellvars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_cellvars }) as i32;
    let nfreevars: i32 = PyDict_GET_SIZE!(unsafe { (*umd).u_freevars }) as i32;
    let mut nlocalsplus: i32 = nlocals.wrapping_add(ncellvars).wrapping_add(nfreevars);
    let mut cellfixedoffsets: *mut i32 = build_cellfixedoffsets(umd);
    if cellfixedoffsets.is_null() {
        return 1i32.wrapping_neg();
    }
    // This must be called before fix_cell_offsets().
    if insert_prefix_instructions(
        umd,
        unsafe { (*g).g_entryblock },
        cellfixedoffsets,
        nfreevars,
        code_flags,
    ) != 0
    {
        unsafe { PyMem_Free(cellfixedoffsets as *mut ()) };
        return 1i32.wrapping_neg();
    }
    let numdropped: i32 = fix_cell_offsets(umd, unsafe { (*g).g_entryblock }, cellfixedoffsets);
    unsafe { PyMem_Free(cellfixedoffsets as *mut ()) }; // At this point we're done with it.
    cellfixedoffsets = core::ptr::null_mut::<i32>();
    if numdropped < 0 {
        return 1i32.wrapping_neg();
    }
    nlocalsplus = nlocalsplus.wrapping_sub(numdropped);
    nlocalsplus
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfg_FromInstructionSequence(
    seq: *mut instruction_sequence,
) -> *mut _PyCfgBuilder {
    let mut oparg: i32 = unsafe { core::mem::zeroed() };
    let mut opcode: i32 = unsafe { core::mem::zeroed() };
    let mut lbl_: core::mem::MaybeUninit<__SlateRecord_pycore_instruction_sequence_h_55> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut __SlateRecord_pycore_instruction_sequence_h_55 =
        core::ptr::addr_of_mut!(lbl_) as *mut __SlateRecord_pycore_instruction_sequence_h_55;
    let mut j: i32 = unsafe { core::mem::zeroed() };
    let mut instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
        unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut offset: i32 = unsafe { core::mem::zeroed() };
    let mut instr_1: *mut __SlateRecord_pycore_instruction_sequence_h_30 =
        unsafe { core::mem::zeroed() };
    let mut i_1: i32 = unsafe { core::mem::zeroed() };
    let mut i_2: i32 = unsafe { core::mem::zeroed() };
    let mut g: *mut _PyCfgBuilder = unsafe { core::mem::zeroed() };
    unsafe {
        if _PyInstructionSequence_ApplyLabelMap(seq) < 0 {
            return core::ptr::null_mut::<_PyCfgBuilder>();
        } else {
            g = _PyCfgBuilder_New();
            if g.is_null() {
                return core::ptr::null_mut::<_PyCfgBuilder>();
            } else {
                i_2 = 0;
                while i_2 < (*seq).s_used {
                    (*unsafe { (*seq).s_instrs }.wrapping_offset(i_2 as isize)).i_target = 0;
                    i_2 = i_2.wrapping_add(1);
                }
                i_1 = 0;
                while i_1 < (*seq).s_used {
                    instr_1 = unsafe { (*seq).s_instrs }.wrapping_offset(i_1 as isize);
                    if HAS_TARGET!(unsafe { (*instr_1).i_opcode }) {
                        (*unsafe { (*seq).s_instrs }
                            .wrapping_offset((*instr_1).i_oparg as isize))
                        .i_target = 1;
                    }
                    i_1 = i_1.wrapping_add(1);
                }
                offset = 0;
                i = 0;
                '__join_0: {
                    while i < (*seq).s_used {
                        instr = unsafe { (*seq).s_instrs }.wrapping_offset(i as isize);
                        if (*instr).i_opcode == 256 {
                            if !unsafe { (*seq).s_annotations_code }.is_null() {
                                j = 0;
                                while j < (*(*seq).s_annotations_code).s_used {
                                    let ann_instr: *mut __SlateRecord_pycore_instruction_sequence_h_30 = unsafe { (*(*seq).s_annotations_code).s_instrs }.wrapping_offset(j as isize);
                                    if _PyCfgBuilder_Addop(
                                        g,
                                        (*ann_instr).i_opcode,
                                        (*ann_instr).i_oparg,
                                        (*ann_instr).i_loc,
                                    ) < 0
                                    {
                                        break '__join_0;
                                    } else {
                                        j = j.wrapping_add(1);
                                    }
                                }
                                offset = offset.wrapping_add(
                                    unsafe { (*(*seq).s_annotations_code).s_used }.wrapping_sub(1),
                                );
                            } else {
                                offset = offset.wrapping_sub(1);
                            }
                        } else {
                            if (*instr).i_target != 0 {
                                core::ptr::write(
                                    __slate_slot_6,
                                    __SlateRecord_pycore_instruction_sequence_h_55 {
                                        id: i.wrapping_add(offset),
                                    },
                                );
                                if _PyCfgBuilder_UseLabel(g, *__slate_slot_6) < 0 {
                                    break '__join_0;
                                }
                            }
                            opcode = (*instr).i_opcode;
                            oparg = (*instr).i_oparg;
                            if HAS_TARGET!(opcode) {
                                oparg = oparg.wrapping_add(offset);
                            }
                            if _PyCfgBuilder_Addop(g, opcode, oparg, (*instr).i_loc) < 0 {
                                break '__join_0;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    if _PyCfgBuilder_CheckSize(g) >= 0 {
                        return g;
                    }
                }
                _PyCfgBuilder_Free(g);
                return core::ptr::null_mut::<_PyCfgBuilder>();
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfg_ToInstructionSequence(
    g: *mut _PyCfgBuilder,
    seq: *mut instruction_sequence,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_instruction_sequence_h_55 =
        unsafe { core::mem::zeroed() };
    let mut lbl: i32 = 0;
    {
        let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
        '__slate_break_3: while !b.is_null() {
            unsafe {
                (*b).b_label = *{
                    __slate_compound_0 = __SlateRecord_pycore_instruction_sequence_h_55 { id: lbl };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                };
            }
            lbl = lbl.wrapping_add(1);
            b = unsafe { (*b).b_next };
        }
    }
    let mut b: *mut _PyCfgBasicblock = unsafe { (*g).g_entryblock };
    while !b.is_null() {
        RETURN_IF_ERROR__2!(unsafe { _PyInstructionSequence_UseLabel(seq, (*b).b_label.id) });
        let mut i: i32 = 0;
        while i < unsafe { (*b).b_iused } {
            let instr: *mut _PyCfgInstruction = unsafe { (*b).b_instr }.wrapping_offset(i as isize);
            if HAS_TARGET!(unsafe { (*instr).i_opcode }) {
                // Set oparg to the label id (it will later be mapped to an offset)
                unsafe {
                    (*instr).i_oparg = (*(*instr).i_target).b_label.id;
                }
            }
            RETURN_IF_ERROR__2!(unsafe {
                _PyInstructionSequence_Addop(
                    seq,
                    (*instr).i_opcode,
                    (*instr).i_oparg,
                    (*instr).i_loc,
                )
            });
            let hi: *mut __SlateRecord_pycore_instruction_sequence_h_19 = unsafe {
                core::ptr::addr_of_mut!(
                    (*unsafe { (*seq).s_instrs }
                        .wrapping_offset(unsafe { (*seq).s_used }.wrapping_sub(1) as isize))
                    .i_except_handler_info
                )
            };
            if !unsafe { (*instr).i_except }.is_null() {
                unsafe {
                    (*hi).h_label = (*(*instr).i_except).b_label.id;
                }
                unsafe {
                    (*hi).h_startdepth = (*(*instr).i_except).b_startdepth;
                }
                unsafe {
                    (*hi).h_preserve_lasti =
                        (*(*instr).i_except).__slate_bits_0.__get_b_preserve_lasti() as i32;
                }
            } else {
                unsafe {
                    (*hi).h_label = 1i32.wrapping_neg();
                }
            }
            i = i.wrapping_add(1);
        }
        b = unsafe { (*b).b_next };
    }
    if (unsafe { _PyInstructionSequence_ApplyLabelMap(seq) }) < 0 {
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfg_OptimizedCfgToInstructionSequence(
    g: *mut _PyCfgBuilder,
    umd: *mut __SlateRecord_pycore_compile_h_78,
    code_flags: i32,
    stackdepth: *mut i32,
    nlocalsplus: *mut i32,
    seq: *mut instruction_sequence,
) -> i32 {
    RETURN_IF_ERROR__2!(convert_pseudo_conditional_jumps(g));
    unsafe {
        *stackdepth = calculate_stackdepth(g);
    }
    if (unsafe { *stackdepth }) < 0 {
        return 1i32.wrapping_neg();
    }
    // prepare_localsplus adds instructions for generators that push
    // and pop an item on the stack. This assertion makes sure there
    // is space on the stack for that.
    // It should always be true, because a generator must have at
    // least one expression or call to INTRINSIC_STOPITERATION_ERROR,
    // which requires stackspace.
    unsafe {
        *nlocalsplus = prepare_localsplus(umd, g, code_flags);
    }
    if (unsafe { *nlocalsplus }) < 0 {
        return 1i32.wrapping_neg();
    }
    RETURN_IF_ERROR__2!(convert_pseudo_ops(g));
    // Order of basic blocks must have been determined by now
    RETURN_IF_ERROR__2!(normalize_jumps(g));
    // Can't modify the bytecode after inserting instructions that produce
    // borrowed references.
    RETURN_IF_ERROR__2!(optimize_load_fast(g));
    // Can't modify the bytecode after computing jump offsets.
    if _PyCfg_ToInstructionSequence(g, seq) < 0 {
        return 1i32.wrapping_neg();
    }
    0
}

/// This is used by _PyCompile_Assemble to fill in the jump and exception
/// targets in a synthetic CFG (which is not the output of the builtin compiler).
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCfg_JumpLabelsToTargets(g: *mut _PyCfgBuilder) -> i32 {
    RETURN_IF_ERROR__2!(translate_jump_labels_to_targets(unsafe {
        (*g).g_entryblock
    }));
    RETURN_IF_ERROR__2!(label_exception_targets(unsafe { (*g).g_entryblock }));
    0
}

// Exported API functions
#[unsafe(no_mangle)]
extern "C-unwind" fn PyCompile_OpcodeStackEffectWithJump(
    opcode: i32,
    oparg: i32,
    jump: i32,
) -> i32 {
    let mut effs: __SlateRecord_flowgraph_c_764 = unsafe { core::mem::zeroed() };
    if get_stack_effects(opcode, oparg, jump, unsafe {
        &mut (*core::ptr::addr_of_mut!(effs))
    }) < 0
    {
        return 2147483647;
    }
    effs.net
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCompile_OpcodeStackEffect(opcode: i32, oparg: i32) -> i32 {
    let mut effs: __SlateRecord_flowgraph_c_764 = unsafe { core::mem::zeroed() };
    if get_stack_effects(opcode, oparg, 1i32.wrapping_neg(), unsafe {
        &mut (*core::ptr::addr_of_mut!(effs))
    }) < 0
    {
        return 2147483647;
    }
    effs.net
}

// Access to compiler optimizations for unit tests.
//
// _PyCompile_OptimizeCfg takes an instruction list, constructs
// a CFG, optimizes it and converts back to an instruction list.
fn cfg_to_instruction_sequence(g: *mut _PyCfgBuilder) -> *mut _object {
    let seq: *mut instruction_sequence =
        (unsafe { _PyInstructionSequence_New() }) as *mut instruction_sequence;
    if seq.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if _PyCfg_ToInstructionSequence(g, seq) < 0 {
        unsafe { PyInstructionSequence_Fini(seq) };
        return core::ptr::null_mut::<_object>();
    }
    seq as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCompile_OptimizeCfg(
    seq: *mut _object,
    consts: *mut _object,
    nlocals: i32,
) -> *mut _object {
    let mut g: *mut _PyCfgBuilder = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut const_cache: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if _PyInstructionSequence_Check!(seq) == 0 {
            PyErr_SetString(
                PyExc_ValueError,
                c"expected an instruction sequence".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            if PyList_Check!(consts) == 0 {
                PyErr_SetString(PyExc_TypeError, c"consts must be a list".as_ptr());
                return core::ptr::null_mut::<_object>();
            } else {
                const_cache = PyDict_New();
                if const_cache.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    res = core::ptr::null_mut::<_object>();
                    g = _PyCfg_FromInstructionSequence(seq as *mut instruction_sequence);
                    if !g.is_null() {
                        let nparams: i32 = 0;
                        let firstlineno: i32 = 1;
                        if _PyCfg_OptimizeCodeUnit(
                            g,
                            consts,
                            const_cache,
                            nlocals,
                            nparams,
                            firstlineno,
                        ) >= 0
                        {
                            if calculate_stackdepth(g) != 1i32.wrapping_neg() {
                                if optimize_load_fast(g) == 0 {
                                    res = cfg_to_instruction_sequence(g);
                                }
                            }
                        }
                    }
                    Py_DECREF!(const_cache);
                    _PyCfgBuilder_Free(g);
                    return res;
                }
            }
        }
    }
}
