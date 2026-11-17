//! <RCS keywords>
//!
//! C++ Library
//!
//! Copyright 1992-1994, David Gottner
//!
//!                    All Rights Reserved
//!
//! Permission to use, copy, modify, and distribute this software and its
//! documentation for any purpose and without fee is hereby granted,
//! provided that the above copyright notice, this permission notice and
//! the following disclaimer notice appear unmodified in all copies.
//!
//! I DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING ALL
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS.  IN NO EVENT SHALL I
//! BE LIABLE FOR ANY SPECIAL, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
//! DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA, OR PROFITS, WHETHER
//! IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT
//! OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
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
unsafe extern "C" {
    fn wcscmp(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcschr(arg0: *const i32, arg1: i32) -> *mut i32;
}

unsafe extern "C" {
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_getopt_h_18 {
    name: *const i32,
    has_arg: i32,
    val: i32,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

// Modified to support --help and --version, as well as /? on Windows
// by Georg Brandl.
/// generate error messages
#[unsafe(no_mangle)]
static mut _PyOS_opterr: i32 = 1;

/// index into argv array
#[unsafe(no_mangle)]
static mut _PyOS_optind: i64 = 1;

/// optional argument
#[unsafe(no_mangle)]
static mut _PyOS_optarg: *const i32 = core::ptr::null::<i32>();

static mut __slate_str50: [i32; 1] = [0];

static mut opt_ptr: *const i32 =
    (unsafe { core::ptr::addr_of_mut!(__slate_str50) as *mut i32 }) as *const i32;

// Python command line short and long options
static mut __slate_str51: [i32; 22] = [
    99, 104, 101, 99, 107, 45, 104, 97, 115, 104, 45, 98, 97, 115, 101, 100, 45, 112, 121, 99, 115,
    0,
];

static mut __slate_str52: [i32; 9] = [104, 101, 108, 112, 45, 97, 108, 108, 0];

static mut __slate_str53: [i32; 9] = [104, 101, 108, 112, 45, 101, 110, 118, 0];

static mut __slate_str54: [i32; 14] = [
    104, 101, 108, 112, 45, 120, 111, 112, 116, 105, 111, 110, 115, 0,
];

/// name, has_arg, val (used in switch in initconfig.c)
/// sentinel
static longopts: __SlateAlign16<[__SlateRecord_pycore_getopt_h_18; 5]> = __SlateAlign16([
    __SlateRecord_pycore_getopt_h_18 {
        name: (unsafe { core::ptr::addr_of_mut!(__slate_str51) as *mut i32 }) as *const i32,
        has_arg: 1i32,
        val: 0i32,
    },
    __SlateRecord_pycore_getopt_h_18 {
        name: (unsafe { core::ptr::addr_of_mut!(__slate_str52) as *mut i32 }) as *const i32,
        has_arg: 0i32,
        val: 1i32,
    },
    __SlateRecord_pycore_getopt_h_18 {
        name: (unsafe { core::ptr::addr_of_mut!(__slate_str53) as *mut i32 }) as *const i32,
        has_arg: 0i32,
        val: 2i32,
    },
    __SlateRecord_pycore_getopt_h_18 {
        name: (unsafe { core::ptr::addr_of_mut!(__slate_str54) as *mut i32 }) as *const i32,
        has_arg: 0i32,
        val: 3i32,
    },
    __SlateRecord_pycore_getopt_h_18 {
        name: core::ptr::null::<i32>(),
        has_arg: 0i32,
        val: 1i32.wrapping_neg(),
    },
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyOS_ResetGetOpt() {
    unsafe {
        _PyOS_opterr = 1;
    }
    unsafe {
        _PyOS_optind = 1;
    }
    unsafe {
        _PyOS_optarg = core::ptr::null::<i32>();
    }
    unsafe {
        opt_ptr = (core::ptr::addr_of_mut!(__slate_str55) as *mut i32) as *const i32;
    }
}

static mut __slate_str55: [i32; 1] = [0];

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyOS_GetOpt(argc: i64, argv: *const *mut i32, longindex: *mut i32) -> i32 {
    let mut option: i32 = 0;
    if (unsafe { *opt_ptr }) == 0 {
        if (unsafe { _PyOS_optind }) >= argc {
            return 1i32.wrapping_neg();
        } else {
            if (unsafe {
                *unsafe { *argv.wrapping_offset(_PyOS_optind as isize) }.wrapping_offset(0 as isize)
            }) != 45
                || (unsafe {
                    *unsafe { *argv.wrapping_offset(_PyOS_optind as isize) }
                        .wrapping_offset(1 as isize)
                }) == 0
            {
                return 1i32.wrapping_neg();
            } else {
                if (unsafe {
                    wcscmp(
                        *argv.wrapping_offset(_PyOS_optind as isize) as *const i32,
                        (core::ptr::addr_of_mut!(__slate_str56) as *mut i32) as *const i32,
                    )
                }) == 0
                {
                    let __v0: i64 = unsafe { _PyOS_optind };
                    unsafe {
                        _PyOS_optind = __v0.wrapping_add(1);
                    }
                    return 1i32.wrapping_neg();
                } else {
                    if (unsafe {
                        wcscmp(
                            *argv.wrapping_offset(_PyOS_optind as isize) as *const i32,
                            (core::ptr::addr_of_mut!(__slate_str57) as *mut i32) as *const i32,
                        )
                    }) == 0
                    {
                        let __v2: i64 = unsafe { _PyOS_optind };
                        unsafe {
                            _PyOS_optind = __v2.wrapping_add(1);
                        }
                        return 104;
                    } else {
                        if (unsafe {
                            wcscmp(
                                *argv.wrapping_offset(_PyOS_optind as isize) as *const i32,
                                (core::ptr::addr_of_mut!(__slate_str58) as *mut i32) as *const i32,
                            )
                        }) == 0
                        {
                            let __v4: i64 = unsafe { _PyOS_optind };
                            unsafe {
                                _PyOS_optind = __v4.wrapping_add(1);
                            }
                            return 86;
                        }
                    }
                }
            }
        }
        // lone dash
        let __v6: i64 = unsafe { _PyOS_optind };
        unsafe {
            _PyOS_optind = __v6.wrapping_add(1);
        }
        unsafe {
            opt_ptr = unsafe { *argv.wrapping_offset(__v6 as isize) }.wrapping_offset(1 as isize)
                as *const i32;
        }
    }
    let __v8: *const i32 = unsafe { opt_ptr };
    unsafe {
        opt_ptr = __v8.wrapping_offset(1 as isize);
    }
    let __v10: i32 = unsafe { *__v8 };
    option = __v10;
    if __v10 == 0 {
        return 1i32.wrapping_neg();
    }
    if option == 45 {
        // Parse long option.
        if (unsafe { *opt_ptr }) == 0 {
            if (unsafe { _PyOS_opterr }) != 0 {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c"Expected long option\n".as_ptr(),
                    )
                };
                unsafe { fflush(core::ptr::null_mut()) };
            }
            return 1i32.wrapping_neg();
        }
        unsafe {
            *longindex = 0;
        }
        let mut opt: *const __SlateRecord_pycore_getopt_h_18 = unsafe { core::mem::zeroed() };
        opt = unsafe { core::ptr::addr_of!(longopts.0) as *const __SlateRecord_pycore_getopt_h_18 }
            .wrapping_offset((unsafe { *longindex }) as isize);
        while !unsafe { (*opt).name }.is_null() {
            if (unsafe { wcscmp((*opt).name, opt_ptr) }) == 0 {
                break;
            }
            let __v13: i32 = unsafe { *longindex };
            let __v14: i32 = __v13.wrapping_add(1);
            unsafe {
                *longindex = __v14;
            }
            opt = unsafe {
                core::ptr::addr_of!(longopts.0) as *const __SlateRecord_pycore_getopt_h_18
            }
            .wrapping_offset(__v14 as isize);
        }
        if unsafe { (*opt).name }.is_null() {
            if (unsafe { _PyOS_opterr }) != 0 {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c"Unknown option: %ls\n".as_ptr(),
                        *argv.wrapping_offset(unsafe { _PyOS_optind }.wrapping_sub(1) as isize),
                    )
                };
                unsafe { fflush(core::ptr::null_mut()) };
            }
            return 95;
        }
        unsafe {
            opt_ptr = (core::ptr::addr_of_mut!(__slate_str62) as *mut i32) as *const i32;
        }
        if (unsafe { (*opt).has_arg }) == 0 {
            return unsafe { (*opt).val };
        }
        if (unsafe { _PyOS_optind }) >= argc {
            if (unsafe { _PyOS_opterr }) != 0 {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c"Argument expected for the %ls options\n".as_ptr(),
                        *argv.wrapping_offset(unsafe { _PyOS_optind }.wrapping_sub(1) as isize),
                    )
                };
                unsafe { fflush(core::ptr::null_mut()) };
            }
            return 95;
        }
        let __v15: i64 = unsafe { _PyOS_optind };
        unsafe {
            _PyOS_optind = __v15.wrapping_add(1);
        }
        unsafe {
            _PyOS_optarg = *argv.wrapping_offset(__v15 as isize) as *const i32;
        }
        return unsafe { (*opt).val };
    }
    let __v17: *mut i32 = unsafe {
        wcschr(
            (core::ptr::addr_of_mut!(__slate_str64) as *mut i32) as *const i32,
            option,
        )
    };
    let ptr: *mut i32 = __v17;
    if __v17.is_null() {
        if (unsafe { _PyOS_opterr }) != 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c"Unknown option: -%c\n".as_ptr(),
                    (option as i8) as i32,
                )
            };
            unsafe { fflush(core::ptr::null_mut()) };
        }
        return 95;
    }
    if (unsafe { *ptr.wrapping_offset(1 as isize) }) == 58 {
        if (unsafe { *opt_ptr }) != 0 {
            unsafe {
                _PyOS_optarg = opt_ptr;
            }
            unsafe {
                opt_ptr = (core::ptr::addr_of_mut!(__slate_str66) as *mut i32) as *const i32;
            }
        } else {
            if (unsafe { _PyOS_optind }) >= argc {
                if (unsafe { _PyOS_opterr }) != 0 {
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                    unsafe {
                        slate_support::fprintf(
                            slate_support::stderr(),
                            c"Argument expected for the -%c option\n".as_ptr(),
                            (option as i8) as i32,
                        )
                    };
                    unsafe { fflush(core::ptr::null_mut()) };
                }
                return 95;
            }
            let __v18: i64 = unsafe { _PyOS_optind };
            unsafe {
                _PyOS_optind = __v18.wrapping_add(1);
            }
            unsafe {
                _PyOS_optarg = *argv.wrapping_offset(__v18 as isize) as *const i32;
            }
        }
    }
    option
}

static mut __slate_str56: [i32; 3] = [45, 45, 0];

static mut __slate_str57: [i32; 7] = [45, 45, 104, 101, 108, 112, 0];

static mut __slate_str58: [i32; 10] = [45, 45, 118, 101, 114, 115, 105, 111, 110, 0];

static mut __slate_str62: [i32; 1] = [0];

static mut __slate_str64: [i32; 28] = [
    98, 66, 99, 58, 100, 69, 104, 105, 73, 109, 58, 79, 80, 113, 82, 115, 83, 116, 117, 118, 86,
    87, 58, 120, 88, 58, 63, 0,
];

static mut __slate_str66: [i32; 1] = [0];
