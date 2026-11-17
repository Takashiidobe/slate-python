//! strtol() and strtoul(), renamed to avoid conflicts.
//!
//!
//!
//! API:
//!
//!
//!
//! - PyOS_strtol(): convert string to C long integer.
//!
//! - PyOS_strtoul(): convert string to C unsigned long integer.
//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
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
//! If we change this, we need to change the default value in the
//! signature of gc.collect.
//! Total tool ids available
//! _PyGenObject_HEAD defines the initial segment of generator
//! and coroutine objects.
//! These would be in pycore_long.h if it weren't for an include cycle.
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// PyObject_VAR_HEAD defines the initial segment of all variable-size
// container objects.  These end with a declaration of an array with 1
// element, but enough space is malloc'ed so that the array actually
// has room for ob_size elements.  Note that ob_size is an element count,
// not necessarily a byte count.
// Cast argument to PyObject* type.
// Cast argument to PyVarObject* type.
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
// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
    };
}

macro_rules! Py_ISSPACE {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 8
    };
}

unsafe extern "C" {
    static mut _Py_ctype_table: [u32; 256];
    static mut _PyLong_DigitValue: [u8; 256];
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

// _PyLong_DigitValue
// strtol and strtoul, renamed to avoid conflicts
// errno
/// Static overflow check values for bases 2 through 36.
/// smallmax[base] is the largest unsigned long i such that
/// i * base doesn't overflow unsigned long.
/// bases 0 and 1 are invalid
static smallmax: __SlateAlign16<[u64; 37]> = __SlateAlign16([
    0u64,
    0u64,
    9223372036854775807u64,
    6148914691236517205u64,
    4611686018427387903u64,
    3689348814741910323u64,
    3074457345618258602u64,
    2635249153387078802u64,
    2305843009213693951u64,
    2049638230412172401u64,
    1844674407370955161u64,
    1676976733973595601u64,
    1537228672809129301u64,
    1418980313362273201u64,
    1317624576693539401u64,
    1229782938247303441u64,
    1152921504606846975u64,
    1085102592571150095u64,
    1024819115206086200u64,
    970881267037344821u64,
    922337203685477580u64,
    878416384462359600u64,
    838488366986797800u64,
    802032351030850070u64,
    768614336404564650u64,
    737869762948382064u64,
    709490156681136600u64,
    683212743470724133u64,
    658812288346769700u64,
    636094623231363848u64,
    614891469123651720u64,
    595056260442243600u64,
    576460752303423487u64,
    558992244657865200u64,
    542551296285575047u64,
    527049830677415760u64,
    512409557603043100u64,
]);

/// maximum digits that can't ever overflow for bases 2 through 36,
/// calculated by [int(math.floor(math.log(2**32, i))) for i in range(2, 37)].
/// Note that this is pessimistic if sizeof(long) > 4.
/// [int(math.floor(math.log(2**64, i))) for i in range(2, 37)]
///  0 -  9
/// 10 - 19
/// 20 - 29
/// 30 - 36
static digitlimit: __SlateAlign16<[i32; 37]> = __SlateAlign16([
    0i32, 0i32, 64i32, 40i32, 32i32, 27i32, 24i32, 22i32, 21i32, 20i32, 19i32, 18i32, 17i32, 17i32,
    16i32, 16i32, 16i32, 15i32, 15i32, 15i32, 14i32, 14i32, 14i32, 14i32, 13i32, 13i32, 13i32,
    13i32, 13i32, 13i32, 13i32, 12i32, 12i32, 12i32, 12i32, 12i32, 12i32,
]);

/// strtoul
///         This is a general purpose routine for converting
///         an ascii string to an integer in an arbitrary base.
///         Leading white space is ignored.  If 'base' is zero
///         it looks for a leading 0b, 0o or 0x to tell which
///         base.  If these are absent it defaults to 10.
///         Base must be 0 or between 2 and 36 (inclusive).
///         If 'ptr' is non-NULL it will contain a pointer to
///         the end of the scan.
///         Errors due to bad pointers will probably result in
///         exceptions - we don't check for them.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyOS_strtoul(mut str: *const i8, ptr: *mut *mut i8, mut base: i32) -> u64 {
    let mut temp_result: u64 = unsafe { core::mem::zeroed() };
    // required digits to overflow
    let mut ovlimit: i32 = unsafe { core::mem::zeroed() };
    // current input character
    let mut c: i32 = unsafe { core::mem::zeroed() };
    // return value of the function
    let mut result: u64 = unsafe { core::mem::zeroed() };
    unsafe {
        result = 0;
        // skip leading white space
        while *str != 0 && Py_ISSPACE!(unsafe { *str }) != 0 {
            let __v37: *const i8 = str;
            str = __v37.wrapping_offset(1 as isize);
        }
        // check for leading 0b, 0o or 0x for auto-base or base 16
        let __t0: i32 = base;
        if __t0 == 0 {
            if (*str as i32) == 48 {
                let __v35: *const i8 = str;
                str = __v35.wrapping_offset(1 as isize);
                if (*str as i32) == 120 || (*str as i32) == 88 {
                    // there must be at least one digit after 0x
                    if (_PyLong_DigitValue[(Py_CHARMASK!(unsafe {
                        *str.wrapping_offset(1 as isize)
                    }) as i32) as usize] as i32)
                        >= 16
                    {
                        if !ptr.is_null() {
                            *ptr = str as *mut i8;
                        }
                        return 0;
                    } else {
                        let __v25: *const i8 = str;
                        str = __v25.wrapping_offset(1 as isize);
                        base = 16;
                    }
                } else {
                    if (*str as i32) == 111 || (*str as i32) == 79 {
                        // there must be at least one digit after 0o
                        if (_PyLong_DigitValue[(Py_CHARMASK!(unsafe {
                            *str.wrapping_offset(1 as isize)
                        }) as i32) as usize] as i32)
                            >= 8
                        {
                            if !ptr.is_null() {
                                *ptr = str as *mut i8;
                            }
                            return 0;
                        } else {
                            let __v27: *const i8 = str;
                            str = __v27.wrapping_offset(1 as isize);
                            base = 8;
                        }
                    } else {
                        if (*str as i32) == 98 || (*str as i32) == 66 {
                            // there must be at least one digit after 0b
                            if (_PyLong_DigitValue[(Py_CHARMASK!(unsafe {
                                *str.wrapping_offset(1 as isize)
                            }) as i32) as usize] as i32)
                                >= 2
                            {
                                if !ptr.is_null() {
                                    *ptr = str as *mut i8;
                                }
                                return 0;
                            } else {
                                let __v29: *const i8 = str;
                                str = __v29.wrapping_offset(1 as isize);
                                base = 2;
                            }
                        } else {
                            // skip all zeroes...
                            while (*str as i32) == 48 {
                                let __v33: *const i8 = str;
                                str = __v33.wrapping_offset(1 as isize);
                            }
                            while Py_ISSPACE!(unsafe { *str }) != 0 {
                                let __v31: *const i8 = str;
                                str = __v31.wrapping_offset(1 as isize);
                            }
                            if !ptr.is_null() {
                                *ptr = str as *mut i8;
                            }
                            return 0;
                        }
                    }
                }
            } else {
                base = 10;
            }
        // look for leading 0b, 0o or 0x
        } else {
            if __t0 == 16 {
                if (*str as i32) == 48 {
                    let __v23: *const i8 = str;
                    str = __v23.wrapping_offset(1 as isize);
                    if (*str as i32) == 120 || (*str as i32) == 88 {
                        // there must be at least one digit after 0x
                        if (_PyLong_DigitValue[(Py_CHARMASK!(unsafe {
                            *str.wrapping_offset(1 as isize)
                        }) as i32) as usize] as i32)
                            >= 16
                        {
                            if !ptr.is_null() {
                                *ptr = str as *mut i8;
                            }
                            return 0;
                        } else {
                            let __v21: *const i8 = str;
                            str = __v21.wrapping_offset(1 as isize);
                        }
                    }
                }
            } else {
                if __t0 == 8 {
                    if (*str as i32) == 48 {
                        let __v19: *const i8 = str;
                        str = __v19.wrapping_offset(1 as isize);
                        if (*str as i32) == 111 || (*str as i32) == 79 {
                            // there must be at least one digit after 0o
                            if (_PyLong_DigitValue[(Py_CHARMASK!(unsafe {
                                *str.wrapping_offset(1 as isize)
                            }) as i32) as usize] as i32)
                                >= 8
                            {
                                if !ptr.is_null() {
                                    *ptr = str as *mut i8;
                                }
                                return 0;
                            } else {
                                let __v17: *const i8 = str;
                                str = __v17.wrapping_offset(1 as isize);
                            }
                        }
                    }
                } else {
                    if __t0 == 2 {
                        if (*str as i32) == 48 {
                            let __v15: *const i8 = str;
                            str = __v15.wrapping_offset(1 as isize);
                            if (*str as i32) == 98 || (*str as i32) == 66 {
                                // there must be at least one digit after 0b
                                if (_PyLong_DigitValue[(Py_CHARMASK!(unsafe {
                                    *str.wrapping_offset(1 as isize)
                                }) as i32)
                                    as usize] as i32)
                                    >= 2
                                {
                                    if !ptr.is_null() {
                                        *ptr = str as *mut i8;
                                    }
                                    return 0;
                                } else {
                                    let __v13: *const i8 = str;
                                    str = __v13.wrapping_offset(1 as isize);
                                }
                            }
                        }
                    }
                }
            }
        }
        // catch silly bases
        if base < 2 || base > 36 {
            if !ptr.is_null() {
                *ptr = str as *mut i8;
            }
            return 0;
        } else {
            // skip leading zeroes
            while (*str as i32) == 48 {
                let __v11: *const i8 = str;
                str = __v11.wrapping_offset(1 as isize);
            }
            // base is guaranteed to be in [2, 36] at this point
            ovlimit = digitlimit.0[base as usize];
            // do the conversion until non-digit character encountered
            '__join_5: {
                loop {
                    let __v2: i32 =
                        _PyLong_DigitValue[(Py_CHARMASK!(unsafe { *str }) as i32) as usize] as i32;
                    c = __v2;
                    if __v2 < base {
                        if ovlimit > 0 {
                            result = result.wrapping_mul(base as u64).wrapping_add(c as u64);
                        } else {
                            // requires overflow check
                            if ovlimit < 0 {
                                break '__join_5;
                            } else {
                                // guaranteed overflow
                                // there could be an overflow
                                //
                                // check overflow just from shifting
                                if result > smallmax.0[base as usize] {
                                    break '__join_5;
                                } else {
                                    result = result.wrapping_mul(base as u64);
                                    // check overflow from the digit's value
                                    temp_result = result.wrapping_add(c as u64);
                                    if temp_result < result {
                                        break '__join_5;
                                    } else {
                                        result = temp_result;
                                    }
                                }
                            }
                        }
                        // no overflow check required
                        let __v6: *const i8 = str;
                        str = __v6.wrapping_offset(1 as isize);
                        ovlimit = ovlimit.wrapping_sub(1);
                    } else {
                        break;
                    }
                }
                // set pointer to point to the last character scanned
                if !ptr.is_null() {
                    *ptr = str as *mut i8;
                }
                return result;
            }
            if !ptr.is_null() {
                // spool through remaining digit characters
                while (_PyLong_DigitValue[(Py_CHARMASK!(unsafe { *str }) as i32) as usize] as i32)
                    < base
                {
                    let __v1: *const i8 = str;
                    str = __v1.wrapping_offset(1 as isize);
                }
                *ptr = str as *mut i8;
            }
            *slate_support::errno_location() = slate_support::ERANGE as i32;
            return 1i32.wrapping_neg() as u64;
        }
    }
    // even with explicit base, skip leading 0? prefix
}

// Checking for overflow in PyOS_strtol is a PITA; see comments
// about PY_ABS_LONG_MIN in longobject.c.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyOS_strtol(mut str: *const i8, ptr: *mut *mut i8, base: i32) -> i64 {
    let mut result: i64 = 0;
    let mut uresult: u64 = 0;
    let mut sign: i8 = 0;
    while (unsafe { *str }) != 0 && Py_ISSPACE!(unsafe { *str }) != 0 {
        let __v0: *const i8 = str;
        str = __v0.wrapping_offset(1 as isize);
    }
    sign = unsafe { *str };
    if (sign as i32) == 43 || (sign as i32) == 45 {
        let __v3: *const i8 = str;
        str = __v3.wrapping_offset(1 as isize);
    }
    uresult = PyOS_strtoul(str, ptr, base);
    if uresult <= 9223372036854775807 {
        result = uresult as i64;
        if (sign as i32) == 45 {
            result = result.wrapping_neg();
        }
    } else {
        if (sign as i32) == 45
            && uresult
                == 0u64.wrapping_sub(9223372036854775807i64.wrapping_neg().wrapping_sub(1) as u64)
        {
            result = 9223372036854775807i64.wrapping_neg().wrapping_sub(1);
        } else {
            unsafe {
                *slate_support::errno_location() = slate_support::ERANGE as i32;
            }
            result = 9223372036854775807;
        }
    }
    result
}
