//! Set of hash utility functions to help maintaining the invariant that
//!  if a==b then hash(a)==hash(b)
//!
//! All the utility functions (_Py_Hash*()) return "-1" to signify an error.
//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
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
//! Cast argument to PyTupleObject* type.
//! Cast argument to PyListObject* type.
//! A code object, the __code__ attribute
//! To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//!
//! defined in this macro:
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
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
macro_rules! HALF_ROUND {
    ($a:expr, $b:expr, $c:expr, $d:expr, $s:expr, $t:expr) => {
        let __v: u64 = $a;
        let __v2: u64 = __v.wrapping_add($b);
        $a = __v2;
        let __v3: u64 = $c;
        let __v4: u64 = __v3.wrapping_add($d);
        $c = __v4;
        $b = ROTATE!($b, $s) ^ $a;
        $d = ROTATE!($d, $t) ^ $c;
        $a = ROTATE!($a, 32i32);
    };
}

macro_rules! ROTATE {
    ($x:expr, $b:expr) => {
        ($x) << $b | ($x) >> 64i32.wrapping_sub($b)
    };
}

macro_rules! SINGLE_ROUND {
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
        HALF_ROUND!($v0, $v1, $v2, $v3, 13i32, 16i32);
        HALF_ROUND!($v2, $v1, $v0, $v3, 17i32, 21i32);
    };
}

macro_rules! _le64toh {
    ($x:expr) => {
        ($x) as u64
    };
}

unsafe extern "C" {
    fn frexp(arg0: f64, arg1: *mut i32) -> f64;
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

// Cast argument to PyObject* type.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_172 {
    ob_base: _object,
    ob_size: i64,
}

// Cast argument to PyVarObject* type.
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyhash_h_39 {
    hash: Option<unsafe extern "C-unwind" fn(*const (), i64) -> i64>,
    name: *const i8,
    hash_bits: i32,
    seed_bits: i32,
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

fn _Py_HashPointerRaw(ptr: *const ()) -> i64 {
    let mut x: u64 = ptr as u64;
    while false {}
    // Bottom 3 or 4 bits are likely to be 0; rotate x by 4 to the right
    //
    // to avoid excessive hash collisions for dicts and sets.
    x = x >> 4i32
        | x << 8u64
            .wrapping_mul(core::mem::size_of::<u64>() as u64)
            .wrapping_sub(4);
    while false {}
    x as i64
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
union __SlateRecord_pycore_pyhash_h_66 {
    uc: [u8; 24],
    fnv: __SlateRecord_pycore_pyhash_h_66_2,
    siphash: __SlateRecord_pycore_pyhash_h_66_3,
    djbx33a: __SlateRecord_pycore_pyhash_h_66_4,
    expat: __SlateRecord_pycore_pyhash_h_66_5,
}

// _Py_HashSecret_t
#[unsafe(no_mangle)]
static mut _Py_HashSecret: __SlateRecord_pycore_pyhash_h_66 = __SlateRecord_pycore_pyhash_h_66 {
    uc: {
        let mut __t0: [u8; 24] = unsafe { core::mem::zeroed() };
        __t0[0] = 0u8;
        __t0
    },
};

use slate_support::memcpy;

static mut PyHash_Func: __SlateRecord_pyhash_h_39 = __SlateRecord_pyhash_h_39 {
    hash: Some(pysiphash),
    name: (b"siphash13\0".as_ptr() as *mut i8) as *const i8,
    hash_bits: 64i32,
    seed_bits: 128i32,
};

// Count Py_HashBuffer() calls
// For numeric types, the hash of a number x is based on the reduction
// of x modulo the prime P = 2**_PyHASH_BITS - 1.  It's designed so that
// hash(x) == hash(y) whenever x and y are numerically equal, even if
// x and y have different types.
//
// A quick summary of the hashing strategy:
//
// (1) First define the 'reduction of x modulo P' for any rational
// number x; this is a standard extension of the usual notion of
// reduction modulo P for integers.  If x == p/q (written in lowest
// terms), the reduction is interpreted as the reduction of p times
// the inverse of the reduction of q, all modulo P; if q is exactly
// divisible by P then define the reduction to be infinity.  So we've
// got a well-defined map
//
//    reduce : { rational numbers } -> { 0, 1, 2, ..., P-1, infinity }.
//
// (2) Now for a rational number x, define hash(x) by:
//
//    reduce(x)   if x >= 0
//    -reduce(-x) if x < 0
//
// If the result of the reduction is infinity (this is impossible for
// integers, floats and Decimals) then use the predefined hash value
// _PyHASH_INF for x >= 0, or -_PyHASH_INF for x < 0, instead.
// _PyHASH_INF and -_PyHASH_INF are also used for the
// hashes of float and Decimal infinities.
//
// NaNs hash with a pointer hash.  Having distinct hash values prevents
// catastrophic pileups from distinct NaN instances which used to always
// have the same hash value but would compare unequal.
//
// A selling point for the above strategy is that it makes it possible
// to compute hashes of decimal and binary floating-point numbers
// efficiently, even if the exponent of the binary or decimal number
// is large.  The key point is that
//
//    reduce(x * y) == reduce(x) * reduce(y) (modulo _PyHASH_MODULUS)
//
// provided that {reduce(x), reduce(y)} != {0, infinity}.  The reduction of a
// binary or decimal float is never infinity, since the denominator is a power
// of 2 (for binary) or a divisor of a power of 10 (for decimal).  So we have,
// for nonnegative x,
//
//    reduce(x * 2**e) == reduce(x) * reduce(2**e) % _PyHASH_MODULUS
//
//    reduce(x * 10**e) == reduce(x) * reduce(10**e) % _PyHASH_MODULUS
//
// and reduce(10**e) can be computed efficiently by the usual modular
// exponentiation algorithm.  For reduce(2**e) it's even better: since
// P is of the form 2**n-1, reduce(2**e) is 2**(e mod n), and multiplication
// by 2**(e mod n) modulo 2**n-1 just amounts to a rotation of bits.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_HashDouble(inst: *mut _object, v: f64) -> i64 {
    let mut e: i32 = 0;
    let mut sign: i32 = 1;
    let mut m: f64 = 0 as f64;
    // process 28 bits at a time;  this should work well both for binary
    // and hexadecimal floating point.
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    if !v.is_finite() {
        if (if v.is_infinite() {
            if v.is_sign_negative() { -1i32 } else { 1i32 }
        } else {
            0i32
        }) != 0
        {
            return (if v > (0i32 as f64) {
                314159i32
            } else {
                314159i32.wrapping_neg()
            }) as i64;
        } else {
            return PyObject_GenericHash(inst);
        }
    }
    m = unsafe { frexp(v, core::ptr::addr_of_mut!(e)) };
    if m < (0i32 as f64) {
        sign = 1i32.wrapping_neg();
        m = -m;
    }
    while m != 0.0f64 {
        x = x << 28i32 & 2305843009213693951u64 | x >> 33i32;
        let __v1: f64 = m * 268435456.0f64;
        m = __v1; // 2**28
        e = e.wrapping_sub(28);
        y = m as u64; // pull out integer part
        m -= y as f64;
        x = x.wrapping_add(y);
        if x >= 2305843009213693951u64 {
            x = x.wrapping_sub(2305843009213693951u64);
        }
    }
    // adjust for the exponent;  first reduce it modulo _PyHASH_BITS
    e = if e >= 0 {
        e % 61
    } else {
        60i32.wrapping_sub(1i32.wrapping_neg().wrapping_sub(e) % 61)
    };
    x = x << e & 2305843009213693951u64 | x >> 61i32.wrapping_sub(e);
    x = x.wrapping_mul(sign as u64);
    if x == (1i32.wrapping_neg() as u64) {
        x = 2i32.wrapping_neg() as u64;
    }
    x as i64
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_HashPointer(ptr: *const ()) -> i64 {
    let mut hash: i64 = _Py_HashPointerRaw(ptr);
    if hash == (1i32.wrapping_neg() as i64) {
        hash = 2i32.wrapping_neg() as i64;
    }
    hash
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.pyhash.PyObject_GenericHash")]
extern "C-unwind" fn PyObject_GenericHash(obj: *mut _object) -> i64 {
    Py_HashPointer(obj as *const ())
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_HashBuffer(ptr: *const (), len: i64) -> i64 {
    // We make the hash of the empty string be 0, rather than using
    // (prefix ^ suffix), since this slightly obfuscates the hash secret
    if len == 0 {
        return 0;
    }
    let x: i64 = unsafe { unsafe { PyHash_Func.hash }.unwrap()(ptr, len) };
    if x == (1i32.wrapping_neg() as i64) {
        return 2i32.wrapping_neg() as i64;
    }
    x
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyHash_Fini() {}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyHash_GetFuncDef() -> *mut __SlateRecord_pyhash_h_39 {
    unsafe { core::ptr::addr_of_mut!(PyHash_Func) }
}

// Optimized memcpy() for Windows
// <MIT License>
// Copyright (c) 2013  Marek Majkowski <marek@popcount.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
// </MIT License>
//
// Original location:
//    https://github.com/majek/csiphash/
//
// Solution inspired by code from:
//    Samuel Neves (supercop/crypto_auth/siphash24/little)
//    djb (supercop/crypto_auth/siphash24/little2)
//    Jean-Philippe Aumasson (https://131002.net/siphash/siphash24.c)
//
// Modified for Python by Christian Heimes:
//    - C89 / MSVC compatibility
//    - _rotl64() on Windows
//    - letoh64() fallback
// byte swap little endian to host endian
// Endian conversion not only ensures that the hash function returns the same
// value on all platforms. It is also required to for a good dispersion of
// the hash values' least significant bits.
fn siphash13(k0: u64, k1: u64, src: *const (), mut src_sz: i64) -> u64 {
    let mut b: u64 = (src_sz as u64) << 56i32;
    let mut r#in: *const u8 = src as *const u8;
    let mut v0: u64 = k0 ^ 8317987319222330741;
    let mut v1: u64 = k1 ^ 7237128888997146477;
    let mut v2: u64 = k0 ^ 7816392313619706465;
    let mut v3: u64 = k1 ^ 8387220255154660723;
    let mut t: u64 = 0;
    while src_sz >= 8 {
        let mut mi: u64 = 0;
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((r#in as *const ()) as *const core::ffi::c_void) as *const u8,
                ((core::ptr::addr_of_mut!(mi) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((core::mem::size_of::<u64>() as u64) as usize) as usize,
            )
        };
        mi = _le64toh!(mi);
        let __v0: *const u8 = r#in;
        r#in = __v0.wrapping_offset(core::mem::size_of::<u64>() as isize);
        src_sz = (src_sz as u64).wrapping_sub(core::mem::size_of::<u64>() as u64) as i64;
        v3 ^= mi;
        SINGLE_ROUND!(v0, v1, v2, v3);
        v0 ^= mi;
    }
    let pt: *mut u8 = core::ptr::addr_of_mut!(t) as *mut u8;
    match src_sz {
        7 => {
            unsafe {
                *pt.wrapping_offset(6 as isize) = *r#in.wrapping_offset(6 as isize);
            }
            unsafe {
                *pt.wrapping_offset(5 as isize) = *r#in.wrapping_offset(5 as isize);
            }
            unsafe {
                *pt.wrapping_offset(4 as isize) = *r#in.wrapping_offset(4 as isize);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((r#in as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((pt as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<u32>() as u64) as usize) as usize,
                )
            };
        }
        6 => {
            unsafe {
                *pt.wrapping_offset(5 as isize) = *r#in.wrapping_offset(5 as isize);
            }
            unsafe {
                *pt.wrapping_offset(4 as isize) = *r#in.wrapping_offset(4 as isize);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((r#in as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((pt as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<u32>() as u64) as usize) as usize,
                )
            };
        }
        5 => {
            unsafe {
                *pt.wrapping_offset(4 as isize) = *r#in.wrapping_offset(4 as isize);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((r#in as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((pt as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<u32>() as u64) as usize) as usize,
                )
            };
        }
        4 => {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((r#in as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((pt as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<u32>() as u64) as usize) as usize,
                )
            };
        }
        3 => {
            unsafe {
                *pt.wrapping_offset(2 as isize) = *r#in.wrapping_offset(2 as isize);
            }
            unsafe {
                *pt.wrapping_offset(1 as isize) = *r#in.wrapping_offset(1 as isize);
            }
            unsafe {
                *pt.wrapping_offset(0 as isize) = *r#in.wrapping_offset(0 as isize);
            }
        }
        2 => {
            unsafe {
                *pt.wrapping_offset(1 as isize) = *r#in.wrapping_offset(1 as isize);
            }
            unsafe {
                *pt.wrapping_offset(0 as isize) = *r#in.wrapping_offset(0 as isize);
            }
        }
        1 => unsafe {
            *pt.wrapping_offset(0 as isize) = *r#in.wrapping_offset(0 as isize);
        },
        _ => {}
    }
    let __v18: u64 = b;
    b = __v18 | _le64toh!(t);
    v3 ^= b;
    SINGLE_ROUND!(v0, v1, v2, v3);
    v0 ^= b;
    v2 ^= 255;
    SINGLE_ROUND!(v0, v1, v2, v3);
    SINGLE_ROUND!(v0, v1, v2, v3);
    SINGLE_ROUND!(v0, v1, v2, v3);
    // modified
    t = v0 ^ v1 ^ (v2 ^ v3);
    t
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_KeyedHash(key: u64, src: *const (), src_sz: i64) -> u64 {
    siphash13(key, 0u64, src, src_sz)
}

#[unsafe(link_section = ".text.slate_distinct.pyhash.pysiphash")]
extern "C-unwind" fn pysiphash(src: *const (), src_sz: i64) -> i64 {
    siphash13(
        _le64toh!(unsafe { _Py_HashSecret.siphash.k0 }),
        _le64toh!(unsafe { _Py_HashSecret.siphash.k1 }),
        src,
        src_sz,
    ) as i64
}
