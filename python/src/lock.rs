//! Lock implementation
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
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! _Py_UNLOCKED is defined as 0 and _Py_LOCKED as 1 in Include/cpython/lock.h
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
// PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//
// on a lock (see PyThread_acquire_lock_timed() below).
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
macro_rules! Py_FatalError__19 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"unlock_once".as_ptr(), ($message) as *const i8) };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_FatalError__33 {
    ($message:expr) => {
        unsafe {
            _Py_FatalErrorFunc(
                c"_PyRecursiveMutex_Unlock".as_ptr(),
                ($message) as *const i8,
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_FatalError__43 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"_PyRawMutex_UnlockSlow".as_ptr(), ($message) as *const i8) };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_FatalError__48 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"PyMutex_Unlock".as_ptr(), ($message) as *const i8) };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_UNREACHABLE {
    () => {
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! SEQLOCK_IS_UPDATING {
    ($sequence:expr) => {
        ($sequence) & 1
    };
}

unsafe extern "C" {
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn PyTime_MonotonicRaw(arg0: *mut i64) -> i32;
    fn Py_MakePendingCalls() -> i32;
    fn _PyParkingLot_Park(
        arg0: *const (),
        arg1: *const (),
        arg2: u64,
        arg3: i64,
        arg4: *mut (),
        arg5: i32,
    ) -> i32;
    fn _PyParkingLot_Unpark(
        arg0: *const (),
        arg1: Option<unsafe extern "C-unwind" fn(*mut (), *mut (), i32)>,
        arg2: *mut (),
    );
    fn _PyParkingLot_UnparkAll(arg0: *const ());
    fn PyThread_get_thread_ident_ex() -> u64;
    fn _PySemaphore_Wait(arg0: *mut _PySemaphore, arg1: i64, arg2: i32) -> i32;
    fn _PySemaphore_Wakeup(arg0: *mut _PySemaphore);
    fn _PySemaphore_Init(arg0: *mut _PySemaphore);
    fn _PySemaphore_Destroy(arg0: *mut _PySemaphore);
    fn _PyTime_Add(arg0: i64, arg1: i64) -> i64;
    fn _PyDeadline_Get(arg0: i64) -> i64;
    fn __builtin_unreachable();
}

fn _Py_atomic_add_uintptr(obj: *mut u64, value: u64) -> u64 {
    unsafe {
        let __t1 = value;
        let __t0 = core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .fetch_add(__t1, core::sync::atomic::Ordering::SeqCst);
        __t0
    }
}

fn _Py_atomic_compare_exchange_uint8(obj: *mut u8, expected: &mut u8, desired: u8) -> i32 {
    let __v0: bool = unsafe {
        let __t0 = expected as *mut u8;
        let __t1 = desired;
        let __t2 = core::sync::atomic::AtomicU8::from_ptr(obj as *mut u8).compare_exchange(
            *__t0,
            __t1,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        );
        match __t2 {
            Ok(_) => true,
            Err(__t3) => {
                *__t0 = __t3;
                false
            }
        }
    };
    __v0 as i32
}

fn _Py_atomic_compare_exchange_uint32(obj: *mut u32, expected: &mut u32, desired: u32) -> i32 {
    let __v0: bool = unsafe {
        let __t0 = expected as *mut u32;
        let __t1 = desired;
        let __t2 = core::sync::atomic::AtomicU32::from_ptr(obj as *mut u32).compare_exchange(
            *__t0,
            __t1,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        );
        match __t2 {
            Ok(_) => true,
            Err(__t3) => {
                *__t0 = __t3;
                false
            }
        }
    };
    __v0 as i32
}

fn _Py_atomic_compare_exchange_uintptr(obj: *mut u64, expected: &mut u64, desired: u64) -> i32 {
    let __v0: bool = unsafe {
        let __t0 = expected as *mut u64;
        let __t1 = desired;
        let __t2 = core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64).compare_exchange(
            *__t0,
            __t1,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        );
        match __t2 {
            Ok(_) => true,
            Err(__t3) => {
                *__t0 = __t3;
                false
            }
        }
    };
    __v0 as i32
}

fn _Py_atomic_exchange_uint8(obj: *mut u8, value: u8) -> u8 {
    unsafe {
        let __t1 = value;
        let __t0 = core::sync::atomic::AtomicU8::from_ptr(obj as *mut u8)
            .swap(__t1, core::sync::atomic::Ordering::SeqCst);
        __t0
    }
}

fn _Py_atomic_exchange_uintptr(obj: *mut u64, value: u64) -> u64 {
    unsafe {
        let __t1 = value;
        let __t0 = core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .swap(__t1, core::sync::atomic::Ordering::SeqCst);
        __t0
    }
}

fn _Py_atomic_load_uint8(obj: *const u8) -> u8 {
    unsafe {
        core::sync::atomic::AtomicU8::from_ptr(obj as *mut u8)
            .load(core::sync::atomic::Ordering::SeqCst)
    }
}

fn _Py_atomic_load_uintptr(obj: *const u64) -> u64 {
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .load(core::sync::atomic::Ordering::SeqCst)
    }
}

fn _Py_atomic_load_uint8_relaxed(obj: *const u8) -> u8 {
    unsafe {
        core::sync::atomic::AtomicU8::from_ptr(obj as *mut u8)
            .load(core::sync::atomic::Ordering::Relaxed)
    }
}

fn _Py_atomic_load_uint32_relaxed(obj: *const u32) -> u32 {
    unsafe {
        core::sync::atomic::AtomicU32::from_ptr(obj as *mut u32)
            .load(core::sync::atomic::Ordering::Relaxed)
    }
}

fn _Py_atomic_load_uintptr_relaxed(obj: *const u64) -> u64 {
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .load(core::sync::atomic::Ordering::Relaxed)
    }
}

fn _Py_atomic_load_ullong_relaxed(obj: *const u64) -> u64 {
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .load(core::sync::atomic::Ordering::Relaxed)
    }
}

fn _Py_atomic_store_uint8(obj: *mut u8, value: u8) {
    unsafe {
        core::sync::atomic::AtomicU8::from_ptr(obj as *mut u8)
            .store(value, core::sync::atomic::Ordering::SeqCst)
    };
}

use slate_support::sched_yield;

fn _Py_atomic_store_uint32(obj: *mut u32, value: u32) {
    unsafe {
        core::sync::atomic::AtomicU32::from_ptr(obj as *mut u32)
            .store(value, core::sync::atomic::Ordering::SeqCst)
    };
}

fn _Py_atomic_store_ullong_relaxed(obj: *mut u64, value: u64) {
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .store(value, core::sync::atomic::Ordering::Relaxed)
    };
}

fn _Py_atomic_load_uint32_acquire(obj: *const u32) -> u32 {
    unsafe {
        core::sync::atomic::AtomicU32::from_ptr(obj as *mut u32)
            .load(core::sync::atomic::Ordering::Acquire)
    }
}

fn _Py_atomic_fence_acquire() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
}

fn _Py_atomic_fence_release() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMutex {
    _bits: u8,
}

fn _PyMutex_Lock(m: *mut PyMutex) {
    let mut expected: u8 = 0;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        1u8,
    ) == 0
    {
        PyMutex_Lock(m);
    }
}

fn _PyMutex_Unlock(m: *mut PyMutex) {
    let mut expected: u8 = 1;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        0u8,
    ) == 0
    {
        PyMutex_Unlock(m);
    }
}

fn _PyMutex_IsLocked(m: *mut PyMutex) -> i32 {
    ((_Py_atomic_load_uint8((unsafe { core::ptr::addr_of_mut!((*m)._bits) }) as *const u8) as i32)
        & 1
        != 0) as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_modsupport_h_9 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_69 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_99 {
    v: u64,
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
struct __SlateRecord_pycore_lock_h_210 {
    sequence: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_semaphore_h_39 {
    __size: [i8; 32],
    __align: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PySemaphore {
    platform_sem: __SlateRecord_semaphore_h_39,
}

// _PyTime_Add()
// sched_yield()
/// If a thread waits on a lock for longer than TIME_TO_BE_FAIR_NS (1 ms), then
///
/// the unlocking thread directly hands off ownership of the lock. This avoids
///
/// starvation.
static TIME_TO_BE_FAIR_NS: i64 = 1000000;

/// Spin for a bit before parking the thread. This is only enabled for
///
/// `--disable-gil` builds because it is unlikely to be helpful if the GIL is
///
/// enabled.
static MAX_SPIN_COUNT: i32 = 0;

#[repr(C)]
#[derive(Clone, Copy)]
struct mutex_entry {
    /// The time after which the unlocking thread should hand off lock ownership
    ///
    /// directly to the waiting thread. Written by the waiting thread.
    time_to_be_fair: i64,
    /// Set to 1 if the lock was handed off. Written by the unlocking thread.
    handed_off: i32,
}

fn _Py_yield() {
    unsafe { sched_yield() as i32 };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMutex_LockTimed(m: *mut PyMutex, mut timeout: i64, flags: u32) -> u32 {
    let mut v: u8 = _Py_atomic_load_uint8_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*m)._bits) }) as *const u8,
    );
    if (v as i32) & 1 == 0 {
        if _Py_atomic_compare_exchange_uint8(
            unsafe { core::ptr::addr_of_mut!((*m)._bits) },
            unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
            ((v as i32) | 1) as u8,
        ) != 0
        {
            return 1;
        }
    }
    if timeout == 0 {
        return 0;
    }
    let mut now: i64 = 0;
    // silently ignore error: cannot report error to the caller
    unsafe { PyTime_MonotonicRaw(core::ptr::addr_of_mut!(now)) };
    let mut endtime: i64 = 0;
    if timeout > 0 {
        endtime = unsafe { _PyTime_Add(now, timeout) };
    }
    let mut entry: mutex_entry = {
        let mut __t0: mutex_entry = unsafe { core::mem::zeroed() };
        __t0.time_to_be_fair = now.wrapping_add(unsafe { TIME_TO_BE_FAIR_NS });
        __t0.handed_off = 0i32;
        __t0
    };
    let mut spin_count: i64 = 0;
    loop {
        '__slate_continue_0: {
            if (v as i32) & 1 == 0 {
                // The lock is unlocked. Try to grab it.
                if _Py_atomic_compare_exchange_uint8(
                    unsafe { core::ptr::addr_of_mut!((*m)._bits) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                    ((v as i32) | 1) as u8,
                ) != 0
                {
                    return 1;
                }
            } else {
                if (v as i32) & 2 == 0 && spin_count < ((unsafe { MAX_SPIN_COUNT }) as i64) {
                    // Spin for a bit.
                    _Py_yield();
                    spin_count = spin_count.wrapping_add(1);
                } else {
                    if timeout == 0 {
                        return 0;
                    }
                    let mut newv: u8 = v;
                    if (v as i32) & 2 == 0 {
                        // We are the first waiter. Set the _Py_HAS_PARKED flag.
                        newv = ((v as i32) | 2) as u8;
                        if _Py_atomic_compare_exchange_uint8(
                            unsafe { core::ptr::addr_of_mut!((*m)._bits) },
                            unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                            newv,
                        ) == 0
                        {
                            break '__slate_continue_0;
                        }
                    }
                    let ret: i32 = unsafe {
                        _PyParkingLot_Park(
                            core::ptr::addr_of_mut!((*m)._bits) as *const (),
                            core::ptr::addr_of_mut!(newv) as *const (),
                            core::mem::size_of::<u8>() as u64,
                            timeout,
                            core::ptr::addr_of_mut!(entry) as *mut (),
                            ((flags as u32) & 1 != 0) as i32,
                        )
                    };
                    if ret == 0 {
                        if entry.handed_off != 0 {
                            // We own the lock now.
                            return 1;
                        }
                    } else {
                        if ret == -3 && (flags as u32) & 2 != 0 {
                            if (unsafe { Py_MakePendingCalls() }) < 0 {
                                return 2;
                            }
                        } else {
                            if ret == -2 {
                                return 0;
                            }
                        }
                    }
                    if timeout > 0 {
                        timeout = unsafe { _PyDeadline_Get(endtime) };
                        if timeout <= 0 {
                            // Avoid negative values because those mean block forever.
                            timeout = 0;
                        }
                    }
                    v = _Py_atomic_load_uint8_relaxed(
                        (unsafe { core::ptr::addr_of_mut!((*m)._bits) }) as *const u8,
                    );
                }
            }
        }
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(link_section = ".text.slate_distinct.lock.mutex_unpark")]
extern "C-unwind" fn mutex_unpark(arg: *mut (), park_arg: *mut (), has_more_waiters: i32) {
    let entry: *mut mutex_entry = park_arg as *mut mutex_entry;
    let mut v: u8 = 0;
    if !entry.is_null() {
        let mut now: i64 = 0;
        // silently ignore error: cannot report error to the caller
        unsafe { PyTime_MonotonicRaw(core::ptr::addr_of_mut!(now)) };
        let should_be_fair: i32 = (now > unsafe { (*entry).time_to_be_fair }) as i32;
        unsafe {
            (*entry).handed_off = should_be_fair;
        }
        if should_be_fair != 0 {
            v = ((v as i32) | 1) as u8;
        }
        if has_more_waiters != 0 {
            v = ((v as i32) | 2) as u8;
        }
    }
    _Py_atomic_store_uint8(
        unsafe { core::ptr::addr_of_mut!((*(arg as *mut PyMutex))._bits) },
        v,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMutex_TryUnlock(m: *mut PyMutex) -> i32 {
    let mut v: u8 =
        _Py_atomic_load_uint8((unsafe { core::ptr::addr_of_mut!((*m)._bits) }) as *const u8);
    loop {
        if (v as i32) & 1 == 0 {
            // error: the mutex is not locked
            return 1i32.wrapping_neg();
        } else {
            if (v as i32) & 2 != 0 {
                // wake up a single thread
                unsafe {
                    _PyParkingLot_Unpark(
                        core::ptr::addr_of_mut!((*m)._bits) as *const (),
                        Some(mutex_unpark),
                        m as *mut (),
                    )
                };
                return 0;
            } else {
                if _Py_atomic_compare_exchange_uint8(
                    unsafe { core::ptr::addr_of_mut!((*m)._bits) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                    0u8,
                ) != 0
                {
                    // fast-path: no waiters
                    return 0;
                }
            }
        }
    }
    unsafe { core::mem::zeroed() }
}

/// _PyRawMutex stores a linked list of `struct raw_mutex_entry`, one for each
///
/// thread waiting on the mutex, directly in the mutex itself.
#[repr(C)]
#[derive(Clone, Copy)]
struct raw_mutex_entry {
    next: *mut raw_mutex_entry,
    sema: _PySemaphore,
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRawMutex_LockSlow(m: *mut __SlateRecord_pycore_lock_h_99) {
    let mut waiter: raw_mutex_entry = unsafe { core::mem::zeroed() };
    unsafe { _PySemaphore_Init(core::ptr::addr_of_mut!(waiter.sema)) };
    let mut v: u64 =
        _Py_atomic_load_uintptr((unsafe { core::ptr::addr_of_mut!((*m).v) }) as *const u64);
    loop {
        if v & 1 == 0 {
            // Unlocked: try to grab it (even if it has a waiter).
            if _Py_atomic_compare_exchange_uintptr(
                unsafe { core::ptr::addr_of_mut!((*m).v) },
                unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                v | 1,
            ) != 0
            {
                break;
            }
        } else {
            // Locked: try to add ourselves as a waiter.
            waiter.next = (v & 18446744073709551614u64) as *mut raw_mutex_entry;
            let desired: u64 = (core::ptr::addr_of_mut!(waiter) as u64) | 1;
            if _Py_atomic_compare_exchange_uintptr(
                unsafe { core::ptr::addr_of_mut!((*m).v) },
                unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                desired,
            ) != 0
            {
                // Wait for us to be woken up. Note that we still have to lock the
                //
                // mutex ourselves: it is NOT handed off to us.
                //
                //
                //
                // Loop until we observe an actual wakeup. A return of Py_PARK_INTR
                //
                // could otherwise let us exit _PySemaphore_Wait and destroy
                //
                // `waiter.sema` while _PyRawMutex_UnlockSlow's matching
                //
                // _PySemaphore_Wakeup is still pending, since the unlocker has
                //
                // already CAS-removed us from the waiter list without any handshake.
                let mut res: i32 = 0;
                loop {
                    res = unsafe {
                        _PySemaphore_Wait(
                            core::ptr::addr_of_mut!(waiter.sema),
                            1i32.wrapping_neg() as i64,
                            0i32,
                        )
                    }; // detach=
                    if res == 0 {
                        break;
                    }
                }
            }
        }
    }
    unsafe { _PySemaphore_Destroy(core::ptr::addr_of_mut!(waiter.sema)) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRawMutex_UnlockSlow(m: *mut __SlateRecord_pycore_lock_h_99) {
    let mut v: u64 =
        _Py_atomic_load_uintptr((unsafe { core::ptr::addr_of_mut!((*m).v) }) as *const u64);
    loop {
        if v & 1 == 0 {
            Py_FatalError__43!(c"unlocking mutex that is not locked".as_ptr() as *mut i8);
        }
        let waiter: *mut raw_mutex_entry = (v & 18446744073709551614u64) as *mut raw_mutex_entry;
        if !waiter.is_null() {
            let next_waiter: u64 = (unsafe { (*waiter).next }) as u64;
            if _Py_atomic_compare_exchange_uintptr(
                unsafe { core::ptr::addr_of_mut!((*m).v) },
                unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                next_waiter,
            ) != 0
            {
                unsafe { _PySemaphore_Wakeup(core::ptr::addr_of_mut!((*waiter).sema)) };
                return;
            }
        } else {
            if _Py_atomic_compare_exchange_uintptr(
                unsafe { core::ptr::addr_of_mut!((*m).v) },
                unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                0u64,
            ) != 0
            {
                return;
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyEvent_IsSet(evt: *mut __SlateRecord_pycore_lock_h_69) -> i32 {
    let v: u8 = _Py_atomic_load_uint8((unsafe { core::ptr::addr_of_mut!((*evt).v) }) as *const u8);
    ((v as i32) == 1) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyEvent_Notify(evt: *mut __SlateRecord_pycore_lock_h_69) {
    let v: u64 =
        _Py_atomic_exchange_uint8(unsafe { core::ptr::addr_of_mut!((*evt).v) }, 1u8) as u64;
    if v == 0 {
        // no waiters
        return;
    } else {
        if v == 1 {
            // event already set
            return;
        } else {
            unsafe { _PyParkingLot_UnparkAll(core::ptr::addr_of_mut!((*evt).v) as *const ()) };
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyEvent_Wait(evt: *mut __SlateRecord_pycore_lock_h_69) {
    while PyEvent_WaitTimed(evt, 1i32.wrapping_neg() as i64, 1i32) == 0 {}
    // detach=
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyEvent_WaitTimed(
    evt: *mut __SlateRecord_pycore_lock_h_69,
    timeout_ns: i64,
    detach: i32,
) -> i32 {
    loop {
        '__slate_continue_0: {
            let mut v: u8 =
                _Py_atomic_load_uint8((unsafe { core::ptr::addr_of_mut!((*evt).v) }) as *const u8);
            if (v as i32) == 1 {
                // event already set
                return 1;
            }
            if (v as i32) == 0 {
                if _Py_atomic_compare_exchange_uint8(
                    unsafe { core::ptr::addr_of_mut!((*evt).v) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                    2u8,
                ) == 0
                {
                    break '__slate_continue_0;
                }
            }
            let mut expected: u8 = 2;
            unsafe {
                _PyParkingLot_Park(
                    core::ptr::addr_of_mut!((*evt).v) as *const (),
                    core::ptr::addr_of_mut!(expected) as *const (),
                    core::mem::size_of::<u8>() as u64,
                    timeout_ns,
                    core::ptr::null_mut::<()>(),
                    detach,
                )
            };
            return ((_Py_atomic_load_uint8(
                (unsafe { core::ptr::addr_of_mut!((*evt).v) }) as *const u8,
            ) as i32)
                == 1) as i32;
        }
    }
    unsafe { core::mem::zeroed() }
}

fn unlock_once(o: *mut __SlateRecord_modsupport_h_9, res: i32) -> i32 {
    // On success (res=0), we set the state to _Py_ONCE_INITIALIZED.
    //
    // On failure (res=-1), we reset the state to _Py_UNLOCKED.
    let mut new_value: u8 = 0;
    match res {
        -1 => {
            new_value = 0;
        }
        0 => {
            new_value = 4;
        }
        _ => {
            Py_FatalError__19!(c"invalid result from _PyOnceFlag_CallOnce".as_ptr() as *mut i8);
            Py_UNREACHABLE!();
        }
    }
    let old_value: u8 =
        _Py_atomic_exchange_uint8(unsafe { core::ptr::addr_of_mut!((*o).v) }, new_value);
    if (old_value as i32) & 2 != 0 {
        // wake up anyone waiting on the once flag
        unsafe { _PyParkingLot_UnparkAll(core::ptr::addr_of_mut!((*o).v) as *const ()) };
    }
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyOnceFlag_CallOnceSlow(
    flag: *mut __SlateRecord_modsupport_h_9,
    r#fn: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let mut v: u8 =
        _Py_atomic_load_uint8((unsafe { core::ptr::addr_of_mut!((*flag).v) }) as *const u8);
    loop {
        '__slate_continue_0: {
            if (v as i32) == 0 {
                if _Py_atomic_compare_exchange_uint8(
                    unsafe { core::ptr::addr_of_mut!((*flag).v) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                    1u8,
                ) == 0
                {
                    break '__slate_continue_0;
                }
                let res: i32 = unsafe { r#fn.unwrap()(arg) };
                return unlock_once(flag, res);
            }
            if (v as i32) == 4 {
                return 0;
            }
            // The once flag is initializing (locked).
            if (v as i32) & 2 == 0 {
                // We are the first waiter. Set the _Py_HAS_PARKED flag.
                let new_value: u8 = ((v as i32) | 2) as u8;
                if _Py_atomic_compare_exchange_uint8(
                    unsafe { core::ptr::addr_of_mut!((*flag).v) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(v)) },
                    new_value,
                ) == 0
                {
                    break '__slate_continue_0;
                }
                v = new_value;
            }
            // Wait for initialization to finish.
            unsafe {
                _PyParkingLot_Park(
                    core::ptr::addr_of_mut!((*flag).v) as *const (),
                    core::ptr::addr_of_mut!(v) as *const (),
                    core::mem::size_of::<u8>() as u64,
                    1i32.wrapping_neg() as i64,
                    core::ptr::null_mut::<()>(),
                    1i32,
                )
            };
            v = _Py_atomic_load_uint8((unsafe { core::ptr::addr_of_mut!((*flag).v) }) as *const u8);
        }
    }
    unsafe { core::mem::zeroed() }
}

fn recursive_mutex_is_owned_by(m: *mut __SlateRecord_pycore_lock_h_153, tid: u64) -> i32 {
    (_Py_atomic_load_ullong_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*m).thread) }) as *const u64,
    ) == tid) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRecursiveMutex_IsLockedByCurrentThread(
    m: *mut __SlateRecord_pycore_lock_h_153,
) -> i32 {
    recursive_mutex_is_owned_by(m, unsafe { PyThread_get_thread_ident_ex() })
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRecursiveMutex_Lock(m: *mut __SlateRecord_pycore_lock_h_153) {
    let thread: u64 = unsafe { PyThread_get_thread_ident_ex() };
    if recursive_mutex_is_owned_by(m, thread) != 0 {
        let __v1: u64 = unsafe { (*m).level };
        unsafe {
            (*m).level = __v1.wrapping_add(1);
        }
        return;
    }
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!((*m).mutex) });
    _Py_atomic_store_ullong_relaxed(unsafe { core::ptr::addr_of_mut!((*m).thread) }, thread);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRecursiveMutex_LockTimed(
    m: *mut __SlateRecord_pycore_lock_h_153,
    timeout: i64,
    flags: u32,
) -> u32 {
    let thread: u64 = unsafe { PyThread_get_thread_ident_ex() };
    if recursive_mutex_is_owned_by(m, thread) != 0 {
        let __v1: u64 = unsafe { (*m).level };
        unsafe {
            (*m).level = __v1.wrapping_add(1);
        }
        return 1;
    }
    let s: u32 = _PyMutex_LockTimed(
        unsafe { core::ptr::addr_of_mut!((*m).mutex) },
        timeout,
        flags,
    );
    if (s as u32) == 1 {
        _Py_atomic_store_ullong_relaxed(unsafe { core::ptr::addr_of_mut!((*m).thread) }, thread);
    }
    s
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRecursiveMutex_Unlock(m: *mut __SlateRecord_pycore_lock_h_153) {
    if _PyRecursiveMutex_TryUnlock(m) < 0 {
        Py_FatalError__33!(
            c"unlocking a recursive mutex that is not owned by the current thread".as_ptr()
                as *mut i8
        );
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRecursiveMutex_TryUnlock(m: *mut __SlateRecord_pycore_lock_h_153) -> i32 {
    let thread: u64 = unsafe { PyThread_get_thread_ident_ex() };
    if recursive_mutex_is_owned_by(m, thread) == 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*m).level }) > 0 {
        let __v1: u64 = unsafe { (*m).level };
        unsafe {
            (*m).level = __v1.wrapping_sub(1);
        }
        return 0;
    }
    _Py_atomic_store_ullong_relaxed(unsafe { core::ptr::addr_of_mut!((*m).thread) }, 0u64);
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*m).mutex) });
    0
}

fn rwmutex_set_parked_and_wait(
    rwmutex: *mut __SlateRecord_pycore_lock_h_189,
    mut bits: u64,
) -> u64 {
    // Set _Py_HAS_PARKED and wait until we are woken up.
    if bits & 2 == 0 {
        let newval: u64 = bits | 2;
        if _Py_atomic_compare_exchange_uintptr(
            unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) },
            unsafe { &mut (*core::ptr::addr_of_mut!(bits)) },
            newval,
        ) == 0
        {
            return bits;
        }
        bits = newval;
    }
    unsafe {
        _PyParkingLot_Park(
            core::ptr::addr_of_mut!((*rwmutex).bits) as *const (),
            core::ptr::addr_of_mut!(bits) as *const (),
            core::mem::size_of::<u64>() as u64,
            1i32.wrapping_neg() as i64,
            core::ptr::null_mut::<()>(),
            1i32,
        )
    };
    _Py_atomic_load_uintptr_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) }) as *const u64,
    )
}

/// The number of readers holding the lock
fn rwmutex_reader_count(bits: u64) -> u64 {
    bits >> 2i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRWMutex_RLock(rwmutex: *mut __SlateRecord_pycore_lock_h_189) {
    let mut bits: u64 = _Py_atomic_load_uintptr_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) }) as *const u64,
    );
    loop {
        if bits & 1 != 0 {
            // A writer already holds the lock.
            bits = rwmutex_set_parked_and_wait(rwmutex, bits);
        } else {
            if bits & 2 != 0 {
                // Reader(s) hold the lock (or just gave up the lock), but there is
                //
                // at least one waiting writer. We can't grab the lock because we
                //
                // don't want to starve the writer. Instead, we park ourselves and
                //
                // wait for the writer to eventually wake us up.
                bits = rwmutex_set_parked_and_wait(rwmutex, bits);
            } else {
                // The lock is unlocked or read-locked. Try to grab it.
                let newval: u64 = bits.wrapping_add(4u64);
                if _Py_atomic_compare_exchange_uintptr(
                    unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(bits)) },
                    newval,
                ) != 0
                {
                    return;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRWMutex_RUnlock(rwmutex: *mut __SlateRecord_pycore_lock_h_189) {
    let mut bits: u64 = _Py_atomic_add_uintptr(
        unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) },
        4i32.wrapping_neg() as u64,
    );
    bits = bits.wrapping_sub(4u64);
    if rwmutex_reader_count(bits) == 0 && bits & 2 != 0 {
        unsafe { _PyParkingLot_UnparkAll(core::ptr::addr_of_mut!((*rwmutex).bits) as *const ()) };
        return;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRWMutex_Lock(rwmutex: *mut __SlateRecord_pycore_lock_h_189) {
    let mut bits: u64 = _Py_atomic_load_uintptr_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) }) as *const u64,
    );
    loop {
        '__slate_continue_0: {
            // If there are no active readers and it's not already write-locked,
            //
            // then we can grab the lock.
            if bits & 18446744073709551613u64 == 0 {
                if _Py_atomic_compare_exchange_uintptr(
                    unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) },
                    unsafe { &mut (*core::ptr::addr_of_mut!(bits)) },
                    bits | 1,
                ) == 0
                {
                    break '__slate_continue_0;
                }
                return;
            }
            // Otherwise, we have to wait.
            bits = rwmutex_set_parked_and_wait(rwmutex, bits);
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyRWMutex_Unlock(rwmutex: *mut __SlateRecord_pycore_lock_h_189) {
    let old_bits: u64 =
        _Py_atomic_exchange_uintptr(unsafe { core::ptr::addr_of_mut!((*rwmutex).bits) }, 0u64);
    if old_bits & 2 != 0 {
        unsafe { _PyParkingLot_UnparkAll(core::ptr::addr_of_mut!((*rwmutex).bits) as *const ()) };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySeqLock_LockWrite(seqlock: *mut __SlateRecord_pycore_lock_h_210) {
    // lock by moving to an odd sequence number
    let mut prev: u32 = _Py_atomic_load_uint32_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
    );
    while true {
        if SEQLOCK_IS_UPDATING!(prev) != 0 {
            // Someone else is currently updating the cache
            _Py_yield();
            prev = _Py_atomic_load_uint32_relaxed(
                (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
            );
        } else {
            if _Py_atomic_compare_exchange_uint32(
                unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) },
                unsafe { &mut (*core::ptr::addr_of_mut!(prev)) },
                prev.wrapping_add(1),
            ) != 0
            {
                // We've locked the cache
                _Py_atomic_fence_release();
                break;
            } else {
                _Py_yield();
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySeqLock_AbandonWrite(seqlock: *mut __SlateRecord_pycore_lock_h_210) {
    let new_seq: u32 = _Py_atomic_load_uint32_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
    )
    .wrapping_sub(1);
    _Py_atomic_store_uint32(
        unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) },
        new_seq,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySeqLock_UnlockWrite(seqlock: *mut __SlateRecord_pycore_lock_h_210) {
    let new_seq: u32 = _Py_atomic_load_uint32_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
    )
    .wrapping_add(1);
    _Py_atomic_store_uint32(
        unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) },
        new_seq,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySeqLock_BeginRead(seqlock: *mut __SlateRecord_pycore_lock_h_210) -> u32 {
    let mut sequence: u32 = _Py_atomic_load_uint32_acquire(
        (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
    );
    while SEQLOCK_IS_UPDATING!(sequence) != 0 {
        _Py_yield();
        sequence = _Py_atomic_load_uint32_acquire(
            (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
        );
    }
    sequence
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySeqLock_EndRead(
    seqlock: *mut __SlateRecord_pycore_lock_h_210,
    previous: u32,
) -> i32 {
    // gh-121368: We need an explicit acquire fence here to ensure that
    //
    // this load of the sequence number is not reordered before any loads
    //
    // within the read lock.
    _Py_atomic_fence_acquire();
    if _Py_atomic_load_uint32_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*seqlock).sequence) }) as *const u32,
    ) == previous
    {
        return 1;
    }
    _Py_yield();
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySeqLock_AfterFork(seqlock: *mut __SlateRecord_pycore_lock_h_210) -> i32 {
    // Synchronize again and validate that the entry hasn't been updated
    //
    // while we were readying the values.
    if SEQLOCK_IS_UPDATING!(unsafe { (*seqlock).sequence }) != 0 {
        unsafe {
            (*seqlock).sequence = 0;
        }
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMutex_Lock(m: *mut PyMutex) {
    _PyMutex_LockTimed(m, 1i32.wrapping_neg() as i64, 1u32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMutex_Unlock(m: *mut PyMutex) {
    if _PyMutex_TryUnlock(m) < 0 {
        Py_FatalError__48!(c"unlocking mutex that is not locked".as_ptr() as *mut i8);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMutex_IsLocked(m: *mut PyMutex) -> i32 {
    _PyMutex_IsLocked(m)
}
