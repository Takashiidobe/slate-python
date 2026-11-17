//! Python's malloc wrappers (see pymem.h)
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
//! Cast argument to PyListObject* type.
//! A code object, the __code__ attribute
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! Special bytes broadcast into debug memory blocks at appropriate times.
//! Strings of these are unlikely to be valid addresses, floats, ints or
//! 7-bit ASCII.
//!
//! - PYMEM_CLEANBYTE: clean (newly allocated) memory
//! - PYMEM_DEADBYTE dead (newly freed) memory
//! - PYMEM_FORBIDDENBYTE: untouchable bytes at each end of a block
//!
//! Byte patterns 0xCB, 0xDB and 0xFB have been replaced with 0xCD, 0xDD and
//! 0xFD to use the same values as Windows CRT debug malloc() and free().
//! If modified, _PyMem_IsPtrFreed() should be updated as well.
//! major + 2 digits minor
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Total tool ids available
//! Set if the interpreter share obmalloc runtime state
//! with the main interpreter.
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
//! Fast inlined version of PyType_IS_GC()
//! all bits set
//! 64MiB  (must be at least MI_SEGMENT_ALIGN)
//! Empty page queues for every bin
//! perhaps use 12 for better performance?
//! 40TB (to include huge page areas)
// Signed variants of the above
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//
// on a lock (see PyThread_acquire_lock_timed() below).
macro_rules! AS_UINT {
    ($p:expr) => {
        ($p) as u64
    };
}

macro_rules! HEAD_LOCK {
    ($runtime:expr) => {
        PyMutex_LockFlags(
            unsafe { core::ptr::addr_of_mut!((*$runtime).interpreters.mutex) },
            0u32,
        );
    };
}

macro_rules! HEAD_UNLOCK {
    ($runtime:expr) => {
        _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*$runtime).interpreters.mutex) });
    };
}

/// Return the number of bytes in size class I, as a uint.
macro_rules! INDEX2SIZE {
    ($I:expr) => {
        (($I) as u32).wrapping_add(1) << 4i32
    };
}

macro_rules! LIKELY {
    ($value:expr) => {
        ($value) as i64
    };
}

macro_rules! MAP_BOT_INDEX {
    ($p:expr) => {
        AS_UINT!($p) >> 20i32 & 16383u64
    };
}

macro_rules! MAP_MID_INDEX {
    ($p:expr) => {
        AS_UINT!($p) >> 34i32 & 32767u64
    };
}

macro_rules! MAP_TOP_INDEX {
    ($p:expr) => {
        AS_UINT!($p) >> 49i32 & 32767u64
    };
}

macro_rules! MI_UNUSED_RELEASE {
    ($x:expr) => {
        MI_UNUSED!($x);
    };
}

/// Inlined definitions
macro_rules! MI_UNUSED {
    ($x:expr) => {
        $x;
    };
}

/// Return total number of blocks in pool of size index I, as a uint.
macro_rules! NUMBLOCKS {
    ($I:expr) => {
        (16384u64.wrapping_sub(_Py_SIZE_ROUND_UP!(
            core::mem::size_of::<pool_header>() as u64,
            16i32
        )) as u32)
            / INDEX2SIZE!($I)
    };
}

macro_rules! OBJECT_STAT_INC_COND {
    () => {};
}

macro_rules! OBJECT_STAT_INC {
    () => {};
}

/// Round pointer P down to the closest pool-aligned address <= P, as a poolp
macro_rules! POOL_ADDR {
    ($P:expr) => {
        _Py_ALIGN_DOWN__2!($P, 16384i32) as *mut pool_header
    };
}

macro_rules! PTA {
    ($pools:expr, $x:expr) => {
        (unsafe { $pools.used.as_mut_ptr() as *mut *mut pool_header }
            .wrapping_offset(2i32.wrapping_mul($x) as isize) as *mut u8)
            .wrapping_offset(-(2u64.wrapping_mul(core::mem::size_of::<*mut u8>() as u64) as isize))
            as *mut pool_header
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

/// Minimum value between x and y
macro_rules! Py_MIN {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $y } else { $x }
    };
}

/// Minimum value between x and y
macro_rules! Py_MIN__4 {
    ($x:expr, $y:expr) => {
        if ($x) > (($y) as u64) {
            ($y) as u64
        } else {
            $x
        }
    };
}

macro_rules! UNLIKELY {
    ($value:expr) => {
        ($value) as i64
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
    };
}

/// Round pointer "p" down to the closest "a"-aligned address <= "p".
macro_rules! _Py_ALIGN_DOWN__2 {
    ($p:expr, $a:expr) => {
        ((($p) as u64) & !($a.wrapping_sub(1) as u64)) as *mut ()
    };
}

/// Round pointer "p" up to the closest "a"-aligned address >= "p".
macro_rules! _Py_ALIGN_UP {
    ($p:expr, $a:expr) => {
        ((($p) as u64).wrapping_add($a.wrapping_sub(1) as u64) & !($a.wrapping_sub(1) as u64))
            as *mut ()
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

macro_rules! _Py_CONTAINER_OF {
    ($ptr:expr, $type:ty) => {
        (($ptr) as *mut i8).wrapping_offset(-(core::mem::offset_of!($type, node) as isize))
            as *mut $type
    };
}

macro_rules! _Py_DECREF_IMMORTAL_STAT_INC {
    () => {};
}

macro_rules! _Py_DECREF_STAT_INC {
    () => {};
}

macro_rules! _Py_IsImmortal {
    ($op:expr) => {
        _Py_IsImmortal(_PyObject_CAST!($op))
    };
}

/// Round up size "n" to be a multiple of "a".
macro_rules! _Py_SIZE_ROUND_UP {
    ($n:expr, $a:expr) => {
        (($n) as u64).wrapping_add($a.wrapping_sub(1) as u64) & !($a.wrapping_sub(1) as u64)
    };
}

/// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! _mi_memzero_var {
    ($x:expr) => {
        unsafe {
            core::ptr::write_bytes(
                ((core::ptr::addr_of_mut!($x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                ((core::mem::size_of::<mi_process_info_s>() as u64) as usize) as usize,
            )
        };
    };
}

/// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! _mi_memzero_var__2 {
    ($x:expr) => {
        unsafe {
            core::ptr::write_bytes(
                ((core::ptr::addr_of_mut!($x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                ((core::mem::size_of::<mi_memid_s>() as u64) as usize) as usize,
            )
        };
    };
}

/// Get the struct containing a node.
macro_rules! llist_data {
    ($node:expr, $type:ty) => {
        _Py_CONTAINER_OF!($node, $type)
    };
}

macro_rules! mi_atomic_add_acq_rel {
    ($p:expr, $x:expr) => {{
        let __v: u64 = unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .fetch_add(__t, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
        __v
    }};
}

macro_rules! mi_atomic_add_acq_rel__2 {
    ($p:expr, $x:expr) => {
        unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64)
                .fetch_add(__t, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
    };
}

macro_rules! mi_atomic_add_relaxed {
    ($p:expr, $x:expr) => {
        unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .fetch_add(__t, core::sync::atomic::Ordering::Relaxed);
            __t2
        };
    };
}

macro_rules! mi_atomic_addi64_acq_rel {
    ($p:expr, $i:expr) => {
        mi_atomic_add_acq_rel__2!($p, $i);
    };
}

macro_rules! mi_atomic_and_acq_rel {
    ($p:expr, $x:expr) => {{
        let __v: u64 = unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .fetch_and(__t, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
        __v
    }};
}

macro_rules! mi_atomic_cas_ptr_weak_acq_rel {
    ($p:expr, $exp:expr) => {
        mi_atomic_cas_weak_acq_rel__2!($p, $exp)
    };
}

macro_rules! mi_atomic_cas_ptr_weak_acq_rel__2 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak_acq_rel__3!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_ptr_weak_release {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak_release__4!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_ptr_weak_release__2 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak_release__2!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_strong {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64).compare_exchange(
                *__t,
                __t2,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
        __v
    }};
}

macro_rules! mi_atomic_cas_strong__2 {
    ($p:expr, $expected:expr, $desired:expr) => {
        unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64).compare_exchange(
                *__t,
                __t2,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
    };
}

macro_rules! mi_atomic_cas_strong__3 {
    ($p:expr, $expected:expr, $desired:expr) => {
        unsafe {
            let __t = $expected;
            let __t2 = ($desired) as i64;
            let __t3 = core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64).compare_exchange(
                *__t,
                __t2,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
    };
}

macro_rules! mi_atomic_cas_strong__4 {
    ($p:expr, $expected:expr, $desired:expr) => {
        unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64).compare_exchange(
                *__t,
                __t2,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
    };
}

macro_rules! mi_atomic_cas_strong_acq_rel__3 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_strong__4!($p, $exp, $des);
    };
}

macro_rules! mi_atomic_cas_strong_acq_rel__2 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_strong__2!($p, $exp, $des);
    };
}

macro_rules! mi_atomic_cas_strong_acq_rel__4 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_strong__3!($p, $exp, $des);
    };
}

macro_rules! mi_atomic_cas_strong_acq_rel {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_strong!($p, $exp, $des)
    };
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_block_s)
                .compare_exchange_weak(
                    *__t as *mut mi_block_s,
                    __t2 as *mut mi_block_s,
                    core::sync::atomic::Ordering::Release,
                    core::sync::atomic::Ordering::Relaxed,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4 as *mut mi_block_s;
                    false
                }
            }
        };
        __v
    }};
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak__2 {
    ($p:expr, $expected:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = core::ptr::null_mut::<mi_block_s>();
            let __t3 = core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_block_s)
                .compare_exchange_weak(
                    *__t as *mut mi_block_s,
                    __t2 as *mut mi_block_s,
                    core::sync::atomic::Ordering::AcqRel,
                    core::sync::atomic::Ordering::Acquire,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4 as *mut mi_block_s;
                    false
                }
            }
        };
        __v
    }};
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak__3 {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .compare_exchange_weak(
                    *__t,
                    __t2,
                    core::sync::atomic::Ordering::Release,
                    core::sync::atomic::Ordering::Relaxed,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
        __v
    }};
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak__4 {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_thread_data_s)
                .compare_exchange_weak(
                    *__t as *mut mi_thread_data_s,
                    __t2 as *mut mi_thread_data_s,
                    core::sync::atomic::Ordering::AcqRel,
                    core::sync::atomic::Ordering::Acquire,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4 as *mut mi_thread_data_s;
                    false
                }
            }
        };
        __v
    }};
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak__5 {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64)
                .compare_exchange_weak(
                    *__t,
                    __t2,
                    core::sync::atomic::Ordering::Release,
                    core::sync::atomic::Ordering::Relaxed,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
        __v
    }};
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak__6 {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_segment_s)
                .compare_exchange_weak(
                    *__t as *mut mi_segment_s,
                    __t2 as *mut mi_segment_s,
                    core::sync::atomic::Ordering::Release,
                    core::sync::atomic::Ordering::Relaxed,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4 as *mut mi_segment_s;
                    false
                }
            }
        };
        __v
    }};
}

/// Various defines for all used memory orders in mimalloc
macro_rules! mi_atomic_cas_weak__7 {
    ($p:expr, $expected:expr, $desired:expr) => {{
        let __v: bool = unsafe {
            let __t = $expected;
            let __t2 = $desired;
            let __t3 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .compare_exchange_weak(
                    *__t,
                    __t2,
                    core::sync::atomic::Ordering::AcqRel,
                    core::sync::atomic::Ordering::Acquire,
                );
            match __t3 {
                Ok(_) => true,
                Err(__t4) => {
                    *__t = __t4;
                    false
                }
            }
        };
        __v
    }};
}

macro_rules! mi_atomic_cas_weak_acq_rel {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak__7!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_weak_acq_rel__2 {
    ($p:expr, $exp:expr) => {
        mi_atomic_cas_weak__2!($p, $exp)
    };
}

macro_rules! mi_atomic_cas_weak_acq_rel__3 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak__4!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_weak_release__2 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak__6!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_weak_release__3 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak__5!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_weak_release__4 {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_cas_weak_release {
    ($p:expr, $exp:expr, $des:expr) => {
        mi_atomic_cas_weak__3!($p, $exp, $des)
    };
}

macro_rules! mi_atomic_casi64_strong_acq_rel {
    ($p:expr, $e:expr, $d:expr) => {
        mi_atomic_cas_strong_acq_rel__4!($p, $e, $d);
    };
}

macro_rules! mi_atomic_casi64_strong_acq_rel__2 {
    ($p:expr, $e:expr, $d:expr) => {
        mi_atomic_cas_strong_acq_rel__3!($p, $e, $d);
    };
}

macro_rules! mi_atomic_decrement_acq_rel {
    ($p:expr) => {
        mi_atomic_sub_acq_rel!($p, 1u64);
    };
}

macro_rules! mi_atomic_decrement_relaxed {
    ($p:expr) => {
        mi_atomic_sub_relaxed!($p, 1u64);
    };
}

macro_rules! mi_atomic_exchange_acq_rel {
    ($p:expr) => {{
        let __v: *mut mi_thread_data_s = unsafe {
            let __t = core::ptr::null_mut::<mi_thread_data_s>();
            let __t2 = core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_thread_data_s)
                .swap(
                    __t as *mut mi_thread_data_s,
                    core::sync::atomic::Ordering::AcqRel,
                );
            __t2 as *mut mi_thread_data_s
        };
        __v
    }};
}

macro_rules! mi_atomic_exchange_acq_rel__2 {
    ($p:expr) => {{
        let __v: *mut mi_segment_s = unsafe {
            let __t = core::ptr::null_mut::<mi_segment_s>();
            let __t2 = core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_segment_s)
                .swap(
                    __t as *mut mi_segment_s,
                    core::sync::atomic::Ordering::AcqRel,
                );
            __t2 as *mut mi_segment_s
        };
        __v
    }};
}

macro_rules! mi_atomic_exchange_ptr_acq_rel__2 {
    ($p:expr) => {
        mi_atomic_exchange_acq_rel__2!($p)
    };
}

macro_rules! mi_atomic_exchange_ptr_acq_rel {
    ($p:expr) => {
        mi_atomic_exchange_acq_rel!($p)
    };
}

macro_rules! mi_atomic_increment_acq_rel {
    ($p:expr) => {
        mi_atomic_add_acq_rel!($p, 1u64)
    };
}

macro_rules! mi_atomic_increment_relaxed {
    ($p:expr) => {
        mi_atomic_add_relaxed!($p, 1u64);
    };
}

macro_rules! mi_atomic_load_acquire__2 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut ())
                .load(core::sync::atomic::Ordering::Acquire) as *mut ()
        }
    };
}

macro_rules! mi_atomic_load_acquire__3 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_arena_s)
                .load(core::sync::atomic::Ordering::Acquire) as *mut mi_arena_s
        }
    };
}

macro_rules! mi_atomic_load_acquire {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .load(core::sync::atomic::Ordering::Acquire)
        }
    };
}

/// In C++/C11 atomics we have polymorphic atomics so can use the typed `ptr` variants (where `tp` is the type of atomic value)
///
/// We use these macros so we can provide a typed wrapper in MSVC in C compilation mode as well
macro_rules! mi_atomic_load_ptr_acquire {
    ($p:expr) => {
        mi_atomic_load_acquire__3!($p)
    };
}

/// In C++/C11 atomics we have polymorphic atomics so can use the typed `ptr` variants (where `tp` is the type of atomic value)
///
/// We use these macros so we can provide a typed wrapper in MSVC in C compilation mode as well
macro_rules! mi_atomic_load_ptr_acquire__2 {
    ($p:expr) => {
        mi_atomic_load_acquire__2!($p)
    };
}

macro_rules! mi_atomic_load_ptr_relaxed__2 {
    ($p:expr) => {
        mi_atomic_load_relaxed__7!($p)
    };
}

macro_rules! mi_atomic_load_ptr_relaxed__3 {
    ($p:expr) => {
        mi_atomic_load_relaxed__2!($p)
    };
}

macro_rules! mi_atomic_load_ptr_relaxed__4 {
    ($p:expr) => {
        mi_atomic_load_relaxed__3!($p)
    };
}

macro_rules! mi_atomic_load_ptr_relaxed {
    ($p:expr) => {
        mi_atomic_load_relaxed__5!($p)
    };
}

macro_rules! mi_atomic_load_ptr_relaxed__5 {
    ($p:expr) => {
        mi_atomic_load_relaxed__4!($p)
    };
}

macro_rules! mi_atomic_load_relaxed__2 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_block_s)
                .load(core::sync::atomic::Ordering::Relaxed) as *mut mi_block_s
        }
    };
}

macro_rules! mi_atomic_load_relaxed__3 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut ())
                .load(core::sync::atomic::Ordering::Relaxed) as *mut ()
        }
    };
}

macro_rules! mi_atomic_load_relaxed {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .load(core::sync::atomic::Ordering::Relaxed)
        }
    };
}

macro_rules! mi_atomic_load_relaxed__4 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_arena_s)
                .load(core::sync::atomic::Ordering::Relaxed) as *mut mi_arena_s
        }
    };
}

macro_rules! mi_atomic_load_relaxed__5 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_segment_s)
                .load(core::sync::atomic::Ordering::Relaxed) as *mut mi_segment_s
        }
    };
}

macro_rules! mi_atomic_load_relaxed__6 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64)
                .load(core::sync::atomic::Ordering::Relaxed)
        }
    };
}

macro_rules! mi_atomic_load_relaxed__7 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_thread_data_s)
                .load(core::sync::atomic::Ordering::Relaxed) as *mut mi_thread_data_s
        }
    };
}

macro_rules! mi_atomic_loadi64_relaxed {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64)
                .load(core::sync::atomic::Ordering::Relaxed)
        }
    };
}

macro_rules! mi_atomic_or_acq_rel {
    ($p:expr, $x:expr) => {{
        let __v: u64 = unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .fetch_or(__t, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
        __v
    }};
}

macro_rules! mi_atomic_store_ptr_release {
    ($p:expr, $x:expr) => {
        mi_atomic_store_release__5!($p, $x);
    };
}

macro_rules! mi_atomic_store_ptr_release__4 {
    ($p:expr, $x:expr) => {
        mi_atomic_store_release__2!($p, $x);
    };
}

macro_rules! mi_atomic_store_ptr_release__2 {
    ($p:expr) => {
        mi_atomic_store_release__6!($p);
    };
}

macro_rules! mi_atomic_store_ptr_release__5 {
    ($p:expr) => {
        mi_atomic_store_release__3!($p);
    };
}

macro_rules! mi_atomic_store_ptr_release__3 {
    ($p:expr, $x:expr) => {
        mi_atomic_store_release__4!($p, $x);
    };
}

macro_rules! mi_atomic_store_relaxed {
    ($p:expr, $x:expr) => {
        unsafe {
            core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .store($x, core::sync::atomic::Ordering::Relaxed)
        };
    };
}

macro_rules! mi_atomic_store_release {
    ($p:expr, $x:expr) => {
        unsafe {
            core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .store($x, core::sync::atomic::Ordering::Release)
        };
    };
}

macro_rules! mi_atomic_store_release__2 {
    ($p:expr, $x:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_arena_s).store(
                ($x) as *mut mi_arena_s,
                core::sync::atomic::Ordering::Release,
            )
        };
    };
}

macro_rules! mi_atomic_store_release__3 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_arena_s).store(
                core::ptr::null_mut::<mi_arena_s>() as *mut mi_arena_s,
                core::sync::atomic::Ordering::Release,
            )
        };
    };
}

macro_rules! mi_atomic_store_release__4 {
    ($p:expr, $x:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_segment_s).store(
                ($x) as *mut mi_segment_s,
                core::sync::atomic::Ordering::Release,
            )
        };
    };
}

macro_rules! mi_atomic_store_release__5 {
    ($p:expr, $x:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut ())
                .store(($x) as *mut (), core::sync::atomic::Ordering::Release)
        };
    };
}

macro_rules! mi_atomic_store_release__6 {
    ($p:expr) => {
        unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(($p) as *mut *mut mi_segment_s).store(
                core::ptr::null_mut::<mi_segment_s>() as *mut mi_segment_s,
                core::sync::atomic::Ordering::Release,
            )
        };
    };
}

macro_rules! mi_atomic_storei64_release {
    ($p:expr, $x:expr) => {
        unsafe {
            core::sync::atomic::AtomicI64::from_ptr(($p) as *mut i64)
                .store($x, core::sync::atomic::Ordering::Release)
        };
    };
}

macro_rules! mi_atomic_sub_acq_rel {
    ($p:expr, $x:expr) => {
        unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .fetch_sub(__t, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
    };
}

macro_rules! mi_atomic_sub_relaxed {
    ($p:expr, $x:expr) => {
        unsafe {
            let __t = $x;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(($p) as *mut u64)
                .fetch_sub(__t, core::sync::atomic::Ordering::Relaxed);
            __t2
        };
    };
}

macro_rules! mi_heap_malloc_tp {
    ($hp:expr, $tp:ty) => {
        mi_heap_malloc($hp, 3064u64) as *mut $tp
    };
}

macro_rules! mi_heap_stat_counter_increase {
    () => {
        mi_stat_counter_increase!();
    };
}

macro_rules! mi_heap_stat_decrease {
    () => {
        mi_stat_decrease!();
    };
}

macro_rules! mi_heap_stat_increase {
    () => {
        mi_stat_increase!();
    };
}

macro_rules! mi_stat_counter_increase {
    () => {};
}

macro_rules! mi_stat_decrease {
    () => {};
}

macro_rules! mi_stat_increase {
    () => {};
}

macro_rules! mi_track_malloc {
    ($p:expr) => {
        if !$p.is_null() {}
    };
}

unsafe extern "C" {
    static mut environ: *mut *mut i8;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn round(arg0: f64) -> f64;
    fn fflush(arg0: *mut slate_support::FILE) -> i32;
    fn vsnprintf(
        arg0: *mut i8,
        arg1: u64,
        arg2: *const i8,
        arg3: core::ffi::VaList<'static>,
    ) -> i32;
    fn fileno(arg0: *mut slate_support::FILE) -> i32;
    #[link_name = "__isoc23_strtol"]
    fn strtol(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> i64;
    fn atexit(arg0: Option<unsafe extern "C-unwind" fn()>) -> i32;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMutex_Lock(arg0: *mut PyMutex);
    fn PyMutex_Unlock(arg0: *mut PyMutex);
    fn _Py_Dealloc(arg0: *mut _object);
    fn Py_GETENV(arg0: *const i8) -> *mut i8;
    fn PyOS_snprintf(arg0: *mut i8, arg1: u64, arg2: *const i8, ...) -> i32;
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn PyInterpreterState_Head() -> *mut _is;
    fn PyInterpreterState_Next(arg0: *mut _is) -> *mut _is;
    #[link_name = "stat64"]
    fn stat(arg0: *const i8, arg1: *mut stat) -> i32;
    fn _PyMutex_LockTimed(arg0: *mut PyMutex, arg1: i64, arg2: u32) -> u32;
    fn __builtin_expect(arg0: i64, arg1: i64) -> i64;
    fn __builtin_assume_aligned(arg0: *const (), arg1: u64, ...) -> *mut ();
    fn _Py_qsbr_poll(arg0: *mut _qsbr_thread_state, arg1: u64) -> bool;
    fn _Py_FatalErrorFormat(arg0: *const i8, arg1: *const i8, ...);
    fn _PyInterpreterState_HasFeature(arg0: *mut _is, arg1: u64) -> i32;
    fn _PyInterpreterState_Main() -> *mut _is;
    fn _Py_IsMainInterpreterFinalizing(arg0: *mut _is) -> i32;
    fn _PyEval_StopTheWorldAll(arg0: *mut pyruntimestate);
    fn _PyEval_StartTheWorldAll(arg0: *mut pyruntimestate);
    #[link_name = "mmap64"]
    fn mmap(arg0: *mut (), arg1: u64, arg2: i32, arg3: i32, arg4: i32, arg5: i64) -> *mut ();
    fn getrusage(arg0: i32, arg1: *mut rusage) -> i32;
    fn _PyMem_DumpTraceback(arg0: i32, arg1: *const ());
}

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.x86.sse2.pause"]
    fn __slate_intrinsic_cf7232a07549d448();
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

fn _Py_atomic_load_int_relaxed(obj: *const i32) -> i32 {
    unsafe {
        core::sync::atomic::AtomicI32::from_ptr(obj as *mut i32)
            .load(core::sync::atomic::Ordering::Relaxed)
    }
}

fn _Py_atomic_load_ptr_relaxed(obj: *const ()) -> *mut () {
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(obj as *mut *mut ())
            .load(core::sync::atomic::Ordering::Relaxed) as *mut ()
    }
}

fn _Py_atomic_store_int_relaxed(obj: *mut i32, value: i32) {
    unsafe {
        core::sync::atomic::AtomicI32::from_ptr(obj as *mut i32)
            .store(value, core::sync::atomic::Ordering::Relaxed)
    };
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
        unsafe { PyMutex_Lock(m) };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

fn _PyMutex_Unlock(m: *mut PyMutex) {
    let mut expected: u8 = 1;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        0u8,
    ) == 0
    {
        unsafe { PyMutex_Unlock(m) };
    }
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

// Cast argument to PyVarObject* type.
// Track types initialized using _PyStaticType_InitBuiltin().
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

// Define a pair of assertion macros:
// _PyObject_ASSERT_FROM(), _PyObject_ASSERT_WITH_MSG() and _PyObject_ASSERT().
//
// These work like the regular C assert(), in that they will abort the
// process with a message on stderr if the given condition fails to hold,
// but compile away to nothing if NDEBUG is defined.
//
// However, before aborting, Python will also try to call _PyObject_Dump() on
// the given object.  This may be of use when investigating bugs in which a
// particular object is corrupt (e.g. buggy a tp_visit method in an extension
// module breaking the garbage collector), to help locate the broken objects.
//
// The WITH_MSG variant allows you to supply an additional message that Python
// will attempt to print to stderr, after the object dump.
//
// No debugging: compile away the assertions:
#[inline(always)]
fn _Py_IsImmortal(op: *mut _object) -> i32 {
    (_Py_CAST!(i32, unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt
    }) < 0) as i32
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
    __slate_bits_0: __slate_bits::__SlateBits157U0,
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

// Cast argument to PyTupleObject* type.
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
struct _PyExecutorObject {}

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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits201U0,
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
struct _inittab {
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: i32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: slate_support::timespec,
    st_mtim: slate_support::timespec,
    st_ctim: slate_support::timespec,
    __glibc_reserved: [i64; 3],
}

fn PyMutex_LockFlags(m: *mut PyMutex, flags: u32) {
    let mut expected: u8 = 0;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        1u8,
    ) == 0
    {
        unsafe { _PyMutex_LockTimed(m, 1i32.wrapping_neg() as i64, flags) };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_69 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct llist_node {
    next: *mut llist_node,
    prev: *mut llist_node,
}

fn llist_init(head: *mut llist_node) {
    unsafe {
        (*head).next = head;
    }
    unsafe {
        (*head).prev = head;
    }
}

fn llist_empty(head: *mut llist_node) -> i32 {
    ((unsafe { (*head).next }) == head) as i32
}

fn llist_remove(node: *mut llist_node) {
    let prev: *mut llist_node = unsafe { (*node).prev };
    let next: *mut llist_node = unsafe { (*node).next };
    unsafe {
        (*prev).next = next;
    }
    unsafe {
        (*next).prev = prev;
    }
    unsafe {
        (*node).prev = core::ptr::null_mut::<llist_node>();
    }
    unsafe {
        (*node).next = core::ptr::null_mut::<llist_node>();
    }
}

fn llist_concat(head1: *mut llist_node, head2: *mut llist_node) {
    if llist_empty(head2) == 0 {
        unsafe {
            (*(*head1).prev).next = (*head2).next;
        }
        unsafe {
            (*(*head2).next).prev = (*head1).prev;
        }
        unsafe {
            (*head1).prev = (*head2).prev;
        }
        unsafe {
            (*(*head2).prev).next = head1;
        }
        llist_init(head2);
    }
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
struct __SlateRecord_types_h_260 {
    __slate_bits_0: __slate_bits::__SlateBits274U0,
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
struct _Py_freelist {
    freelist: *mut (),
    size: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_types_h_434 {
    os: mi_memid_os_info,
    arena: mi_memid_arena_info,
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

// leads to warnings... __attribute__((cdecl))
#[repr(C)]
#[derive(Clone, Copy)]
struct mi_heap_area_s {
    blocks: *mut (),
    reserved: u64,
    committed: u64,
    used: u64,
    block_size: u64,
    full_block_size: u64,
}

fn mi_atomic_addi64_relaxed(p: *mut i64, add: i64) -> i64 {
    unsafe {
        let __t1 = add;
        let __t0 = core::sync::atomic::AtomicI64::from_ptr(p as *mut i64)
            .fetch_add(__t1, core::sync::atomic::Ordering::Relaxed);
        __t0
    }
}

fn mi_atomic_maxi64_relaxed(p: *mut i64, x: i64) {
    let mut current: i64 = mi_atomic_load_relaxed__6!(p as *mut i64);
    loop {
        let __v0: bool = current < x
            && !mi_atomic_cas_weak_release__3!(p as *mut i64, core::ptr::addr_of_mut!(current), x);
        if !__v0 {
            break;
        }
    }
    // nothing
}

fn mi_atomic_once(once: *mut u64) -> bool {
    if mi_atomic_load_relaxed!(once) != 0 {
        return false;
    }
    // quick test
    let mut expected: u64 = 0;
    return mi_atomic_cas_strong_acq_rel!(once, core::ptr::addr_of_mut!(expected), 1u64); // try to set to 1
}

// Allows only one thread to execute at a time
fn mi_atomic_yield() {
    unsafe { __slate_intrinsic_cf7232a07549d448() };
}

// 64KiB  (32KiB on 32-bit)
// Derived constants
// 1024
// 8KiB on 64-bit
// 128KiB on 64-bit
// 32MiB on 64-bit
// Maximum number of size classes. (spaced exponentially in 12.5% increments)
// Maximum slice offset (15)
// Used as a special value to encode block sizes in 32 bits.
// blocks up to this size are always allocated aligned
// Alignments over MI_ALIGNMENT_MAX are allocated in dedicated huge page segments
#[repr(C)]
#[derive(Clone, Copy)]
struct mi_block_s {
    next: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union mi_page_flags_s {
    full_aligned: u8,
    x: __SlateRecord_types_h_260,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_page_s {
    slice_count: u32,
    slice_offset: u32,
    __slate_bits_0: __slate_bits::__SlateBits277U0,
    debug_offset: u8,
    capacity: u16,
    reserved: u16,
    flags: mi_page_flags_s,
    __slate_bits_1: __slate_bits::__SlateBits277U1,
    free: *mut mi_block_s,
    used: u32,
    xblock_size: u32,
    local_free: *mut mi_block_s,
    xthread_free: u64,
    xheap: u64,
    next: *mut mi_page_s,
    prev: *mut mi_page_s,
    padding: [u64; 1],
}

// 64KiB
#[repr(C)]
#[derive(Clone, Copy)]
struct mi_commit_mask_s {
    mask: [u64; 8],
}

fn mi_memkind_is_os(memkind: u32) -> bool {
    (memkind as u32) >= 3 && (memkind as u32) <= 5
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_memid_os_info {
    base: *mut (),
    alignment: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_memid_arena_info {
    block_index: u64,
    id: i32,
    is_exclusive: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_memid_s {
    mem: __SlateRecord_types_h_434,
    is_pinned: bool,
    initially_committed: bool,
    initially_zero: bool,
    memkind: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_segment_s {
    memid: mi_memid_s,
    allow_decommit: bool,
    allow_purge: bool,
    segment_size: u64,
    purge_expire: i64,
    purge_mask: mi_commit_mask_s,
    commit_mask: mi_commit_mask_s,
    abandoned_next: *mut mi_segment_s,
    next: *mut mi_segment_s,
    abandoned: u64,
    abandoned_visits: u64,
    used: u64,
    cookie: u64,
    segment_slices: u64,
    segment_info_slices: u64,
    kind: u32,
    slice_entries: u64,
    thread_id: u64,
    slices: [mi_page_s; 513],
}

#[repr(C, align(64))]
#[derive(Clone, Copy)]
struct mi_abandoned_pool_s {
    abandoned_visited: *mut mi_segment_s,
    __slate_pad_1: [u8; 56],
    abandoned: u64,
    __slate_pad_3: [u8; 56],
    abandoned_count: u64,
    __slate_pad_5: [u8; 56],
    abandoned_visited_count: u64,
    __slate_pad_7: [u8; 56],
    abandoned_readers: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_page_queue_s {
    first: *mut mi_page_s,
    last: *mut mi_page_s,
    block_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_random_cxt_s {
    input: [u32; 16],
    output: [u32; 16],
    output_available: i32,
    weak: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct code_arena_st {}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_heap_s {
    tld: *mut mi_tld_s,
    pages_free_direct: [*mut mi_page_s; 129],
    pages: [mi_page_queue_s; 75],
    thread_delayed_free: *mut mi_block_s,
    thread_id: u64,
    arena_id: i32,
    cookie: u64,
    keys: [u64; 2],
    random: mi_random_cxt_s,
    page_count: u64,
    page_retired_min: u64,
    page_retired_max: u64,
    next: *mut mi_heap_s,
    no_reclaim: bool,
    tag: u8,
    debug_offset: u8,
    page_use_qsbr: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_stat_count_s {
    allocated: i64,
    freed: i64,
    peak: i64,
    current: i64,
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
struct mi_stat_counter_s {
    total: i64,
    count: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_stats_s {
    segments: mi_stat_count_s,
    pages: mi_stat_count_s,
    reserved: mi_stat_count_s,
    committed: mi_stat_count_s,
    reset: mi_stat_count_s,
    purged: mi_stat_count_s,
    page_committed: mi_stat_count_s,
    segments_abandoned: mi_stat_count_s,
    pages_abandoned: mi_stat_count_s,
    threads: mi_stat_count_s,
    normal: mi_stat_count_s,
    huge: mi_stat_count_s,
    large: mi_stat_count_s,
    malloc: mi_stat_count_s,
    segments_cache: mi_stat_count_s,
    pages_extended: mi_stat_counter_s,
    mmap_calls: mi_stat_counter_s,
    commit_calls: mi_stat_counter_s,
    reset_calls: mi_stat_counter_s,
    purge_calls: mi_stat_counter_s,
    page_no_retire: mi_stat_counter_s,
    searches: mi_stat_counter_s,
    normal_count: mi_stat_counter_s,
    huge_count: mi_stat_counter_s,
    large_count: mi_stat_counter_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_span_queue_s {
    first: *mut mi_page_s,
    last: *mut mi_page_s,
    slice_count: u64,
}

// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
#[repr(C)]
#[derive(Clone, Copy)]
struct mi_os_tld_s {
    region_idx: u64,
    stats: *mut mi_stats_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_segments_tld_s {
    spans: [mi_span_queue_s; 36],
    count: u64,
    peak_count: u64,
    current_size: u64,
    peak_size: u64,
    stats: *mut mi_stats_s,
    os: *mut mi_os_tld_s,
    abandoned: *mut mi_abandoned_pool_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_tld_s {
    heartbeat: u64,
    recurse: bool,
    heap_backing: *mut mi_heap_s,
    heaps: *mut mi_heap_s,
    segments: mi_segments_tld_s,
    os: mi_os_tld_s,
    stats: mi_stats_s,
}

#[unsafe(no_mangle)]
static _mi_page_empty: __SlateAlign64<mi_page_s> = __SlateAlign64(unsafe { core::mem::zeroed() });

#[unsafe(no_mangle)]
static mut _mi_abandoned_default: mi_abandoned_pool_s = unsafe { core::mem::zeroed() };

fn _mi_is_power_of_two(x: u64) -> bool {
    x & x.wrapping_sub(1) == 0
}

fn _mi_is_aligned(p: *mut (), alignment: u64) -> bool {
    (p as u64) % alignment == 0
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

fn _mi_align_up(sz: u64, alignment: u64) -> u64 {
    let mask: u64 = alignment.wrapping_sub(1);
    if alignment & mask == 0 {
        // power of two?
        return sz.wrapping_add(mask) & !mask;
    } else {
        return (sz.wrapping_add(mask) / alignment).wrapping_mul(alignment);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
}

fn _mi_align_down(sz: u64, alignment: u64) -> u64 {
    let mask: u64 = alignment.wrapping_sub(1);
    if alignment & mask == 0 {
        // power of two?
        return sz & !mask;
    } else {
        return (sz / alignment).wrapping_mul(alignment);
    }
}

fn _mi_divide_up(size: u64, divider: u64) -> u64 {
    if divider == 0 {
        size
    } else {
        size.wrapping_add(divider).wrapping_sub(1) / divider
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
}

fn _mi_wsize_from_size(size: u64) -> u64 {
    size.wrapping_add(core::mem::size_of::<u64>() as u64)
        .wrapping_sub(1)
        / (core::mem::size_of::<u64>() as u64)
}

fn mi_mul_overflow(count: u64, size: u64, mut total: &mut u64) -> bool {
    {
        let __t0: i128 = count as i128;
        let __t1: i128 = size as i128;
        let __t2: i128 = __t0.wrapping_mul(__t1);
        unsafe {
            *(total as *mut u64) = __t2 as u64;
        }
        __t0.checked_mul(__t1).is_none() || ((__t2 as u64) as i128) != __t2
    }
}

fn mi_count_size_overflow(count: u64, size: u64, total: &mut u64) -> bool {
    if count == 1 {
        // quick check for the case where count is one (common for C++ allocators)
        unsafe {
            *(total as *mut u64) = size;
        }
        return false;
    } else {
        if (!(!mi_mul_overflow(count, size, total)) as i64) != 0 {
            unsafe {
                *(total as *mut u64) = 18446744073709551615;
            }
            return true;
        } else {
            return false;
        }
    }
}

fn mi_heap_is_backing(heap: *const mi_heap_s) -> bool {
    (unsafe { (*(*heap).tld).heap_backing }) == (heap as *mut mi_heap_s)
}

fn mi_heap_is_initialized(heap: *mut mi_heap_s) -> bool {
    heap != ((unsafe { core::ptr::addr_of!(_mi_heap_empty.0) }) as *mut mi_heap_s)
}

fn _mi_ptr_cookie(p: *const ()) -> u64 {
    (p as u64) ^ unsafe { _mi_heap_main.cookie }
}

fn _mi_heap_get_free_small_page(heap: *mut mi_heap_s, size: u64) -> *mut mi_page_s {
    let idx: u64 = _mi_wsize_from_size(size);
    unsafe {
        *((*heap).pages_free_direct.as_mut_ptr() as *mut *mut mi_page_s)
            .wrapping_offset(idx as isize)
    }
}

fn _mi_ptr_segment(p: *const ()) -> *mut mi_segment_s {
    ((p as u64).wrapping_sub(1) & 18446744073675997184u64) as *mut mi_segment_s
}

fn mi_slice_to_page(s: *mut mi_page_s) -> *mut mi_page_s {
    s
}

fn mi_page_to_slice(p: *mut mi_page_s) -> *mut mi_page_s {
    p
}

fn _mi_page_segment(page: *const mi_page_s) -> *mut mi_segment_s {
    let segment: *mut mi_segment_s = _mi_ptr_segment(page as *const ());
    segment
}

fn mi_slice_first(slice: *const mi_page_s) -> *mut mi_page_s {
    let start: *mut mi_page_s = (slice as *mut u8)
        .wrapping_offset(-((unsafe { (*slice).slice_offset }) as isize))
        as *mut mi_page_s;
    start
}

fn _mi_segment_page_of(segment: *const mi_segment_s, p: *const ()) -> *mut mi_page_s {
    let diff: i64 = (unsafe { (p as *mut u8).offset_from(segment as *mut u8) }) as i64;
    let idx: u64 = (diff as u64) >> 16i32;
    let slice0: *mut mi_page_s = unsafe { (*segment).slices.as_ptr() as *const mi_page_s }
        .wrapping_offset(idx as isize) as *mut mi_page_s;
    let slice: *mut mi_page_s = mi_slice_first(slice0 as *const mi_page_s); // adjust to the block that holds the page data
    mi_slice_to_page(slice)
}

fn _mi_page_start(
    segment: *const mi_segment_s,
    page: *const mi_page_s,
    page_size: *mut u64,
) -> *mut u8 {
    _mi_segment_page_start(segment, page, page_size)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
}

fn _mi_ptr_page(p: *mut ()) -> *mut mi_page_s {
    _mi_segment_page_of(
        _mi_ptr_segment(p as *const ()) as *const mi_segment_s,
        p as *const (),
    )
}

fn mi_page_block_size(page: *const mi_page_s) -> u64 {
    let bsize: u64 = (unsafe { (*page).xblock_size }) as u64;
    if bsize < 2147483648u64 {
        return bsize;
    } else {
        let mut psize: u64 = 0;
        _mi_segment_page_start(
            _mi_page_segment(page) as *const mi_segment_s,
            page,
            core::ptr::addr_of_mut!(psize),
        );
        return psize;
    }
}

fn mi_page_usable_block_size(page: *const mi_page_s) -> u64 {
    mi_page_block_size(page).wrapping_sub(0)
}

fn mi_segment_size(segment: *mut mi_segment_s) -> u64 {
    unsafe { (*segment).segment_slices }.wrapping_mul(65536u64)
}

fn mi_page_thread_free(page: *const mi_page_s) -> *mut mi_block_s {
    (mi_atomic_load_relaxed!(unsafe {
        core::ptr::addr_of_mut!((*(page as *mut mi_page_s)).xthread_free)
    }) & 18446744073709551612u64) as *mut mi_block_s
}

fn mi_page_heap(page: *const mi_page_s) -> *mut mi_heap_s {
    mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*(page as *mut mi_page_s)).xheap) })
        as *mut mi_heap_s
}

fn mi_page_set_heap(page: *mut mi_page_s, heap: *mut mi_heap_s) {
    mi_atomic_store_release!(
        unsafe { core::ptr::addr_of_mut!((*page).xheap) },
        heap as u64
    );
}

fn mi_tf_block(tf: u64) -> *mut mi_block_s {
    (tf & 18446744073709551612u64) as *mut mi_block_s
}

fn mi_tf_delayed(tf: u64) -> u32 {
    (tf & 3) as u32
}

fn mi_tf_make(block: *mut mi_block_s, delayed: u32) -> u64 {
    (block as u64) | ((delayed as u32) as u64)
}

fn mi_tf_set_delayed(tf: u64, delayed: u32) -> u64 {
    mi_tf_make(mi_tf_block(tf), delayed)
}

fn mi_tf_set_block(tf: u64, block: *mut mi_block_s) -> u64 {
    mi_tf_make(block, mi_tf_delayed(tf))
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

fn mi_page_all_free(page: *const mi_page_s) -> bool {
    (unsafe { (*page).used }) == 0
}

fn mi_page_has_any_available(page: *const mi_page_s) -> bool {
    (unsafe { (*page).used }) < (((unsafe { (*page).reserved }) as i32) as u32)
        || !mi_page_thread_free(page).is_null()
}

fn mi_page_immediate_available(page: *const mi_page_s) -> bool {
    !unsafe { (*page).free }.is_null()
}

fn mi_page_queue(heap: *const mi_heap_s, size: u64) -> *mut mi_page_queue_s {
    unsafe { (*(heap as *mut mi_heap_s)).pages.as_mut_ptr() as *mut mi_page_queue_s }
        .wrapping_offset((_mi_bin(size) as i32) as isize)
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

fn mi_page_is_in_full(page: *const mi_page_s) -> bool {
    ((unsafe { (*page).flags.x.__slate_bits_0.__get_in_full() }) as i32) != 0
}

fn mi_page_set_in_full(page: *mut mi_page_s, in_full: bool) {
    unsafe {
        (*page).flags.x.__slate_bits_0.__set_in_full(in_full as u8);
    }
}

fn mi_page_has_aligned(page: *const mi_page_s) -> bool {
    ((unsafe { (*page).flags.x.__slate_bits_0.__get_has_aligned() }) as i32) != 0
}

fn mi_page_set_has_aligned(page: *mut mi_page_s, has_aligned: bool) {
    unsafe {
        (*page)
            .flags
            .x
            .__slate_bits_0
            .__set_has_aligned(has_aligned as u8);
    }
}

fn mi_block_nextx(null: *const (), block: *const mi_block_s, keys: *const u64) -> *mut mi_block_s {
    MI_UNUSED!(keys);
    MI_UNUSED!(null);
    let next: *mut mi_block_s =
        mi_atomic_load_relaxed!((unsafe { core::ptr::addr_of!((*block).next) }) as *mut u64)
            as *mut mi_block_s;
    next
}

fn mi_block_set_nextx(
    null: *const (),
    block: *mut mi_block_s,
    next: *const mi_block_s,
    keys: *const u64,
) {
    MI_UNUSED!(keys);
    MI_UNUSED!(null);
    mi_atomic_store_relaxed!(
        unsafe { core::ptr::addr_of_mut!((*block).next) },
        next as u64
    );
}

fn mi_block_next(page: *const mi_page_s, block: *const mi_block_s) -> *mut mi_block_s {
    MI_UNUSED!(page);
    mi_block_nextx(page as *const (), block, core::ptr::null::<u64>())
}

fn mi_block_set_next(page: *const mi_page_s, block: *mut mi_block_s, next: *const mi_block_s) {
    MI_UNUSED!(page);
    mi_block_set_nextx(page as *const (), block, next, core::ptr::null::<u64>());
}

fn mi_commit_mask_create_empty(cm: *mut mi_commit_mask_s) {
    for i in 0..8u64 {
        unsafe {
            *((*cm).mask.as_mut_ptr() as *mut u64).wrapping_offset(i as isize) = 0;
        }
    }
}

fn mi_commit_mask_create_full(cm: &mut mi_commit_mask_s) {
    for i in 0..8u64 {
        unsafe {
            *((*(cm as *mut mi_commit_mask_s)).mask.as_mut_ptr() as *mut u64)
                .wrapping_offset(i as isize) = !0;
        }
    }
}

fn mi_commit_mask_is_empty(cm: *const mi_commit_mask_s) -> bool {
    for i in 0..8u64 {
        if (unsafe { *((*cm).mask.as_ptr() as *const u64).wrapping_offset(i as isize) }) != 0 {
            return false;
        }
    }
    true
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
}

fn mi_commit_mask_is_full(cm: *const mi_commit_mask_s) -> bool {
    for i in 0..8u64 {
        if (unsafe { *((*cm).mask.as_ptr() as *const u64).wrapping_offset(i as isize) }) != !0 {
            return false;
        }
    }
    true
}

fn _mi_memid_create(memkind: u32) -> mi_memid_s {
    let mut memid: mi_memid_s = unsafe { core::mem::zeroed() };
    _mi_memzero_var__2!(memid);
    memid.memkind = memkind;
    return memid;
}

fn _mi_memid_none() -> mi_memid_s {
    return _mi_memid_create(0u32);
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

fn _mi_memid_create_os(committed: bool, is_zero: bool, is_large: bool) -> mi_memid_s {
    let mut memid: mi_memid_s = _mi_memid_create(3u32);
    memid.initially_committed = committed;
    memid.initially_zero = is_zero;
    memid.is_pinned = is_large;
    return memid;
}

fn _mi_random_shuffle(mut x: u64) -> u64 {
    if x == 0 {
        x = 17;
    }
    // ensure we don't get stuck in generating zeros
    // by Sebastiano Vigna, see: <http://xoshiro.di.unimi.it/splitmix64.c>
    x ^= x >> 30i32;
    x = x.wrapping_mul(13787848793156543929);
    x ^= x >> 27i32;
    x = x.wrapping_mul(10723151780598845931);
    x ^= x >> 31i32;
    x
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
}

#[unsafe(no_mangle)]
static mut _mi_numa_node_count: u64 = unsafe { core::mem::zeroed() };

fn _mi_os_numa_node(tld: *mut mi_os_tld_s) -> i32 {
    if mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(_mi_numa_node_count) }) == 1 {
        return 0;
    } else {
        return _mi_os_numa_node_get(tld);
    }
}

fn _mi_os_numa_node_count() -> u64 {
    let count: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(_mi_numa_node_count) });
    if count > 0 {
        return count;
    } else {
        return _mi_os_numa_node_count_get();
    }
}

fn mi_clz(x: u64) -> u64 {
    if x == 0 {
        return 64u64;
    }
    (x.leading_zeros() as i32) as u64
}

fn mi_ctz(x: u64) -> u64 {
    if x == 0 {
        return 64u64;
    }
    (x.trailing_zeros() as i32) as u64
}

fn mi_bsr(x: u64) -> u64 {
    let __v0: u64 = if x == 0 {
        64u64
    } else {
        63u64.wrapping_sub(mi_clz(x))
    };
    __v0
}

fn _mi_memcpy(dst: *mut (), src: *const (), n: u64) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            (src as *const core::ffi::c_void) as *const u8,
            (dst as *mut core::ffi::c_void) as *mut u8,
            (n as usize) as usize,
        )
    };
}

fn _mi_memzero(dst: *mut (), n: u64) {
    unsafe {
        core::ptr::write_bytes(
            (dst as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            (n as usize) as usize,
        )
    };
}

fn _mi_memcpy_aligned(dst: *mut (), src: *const (), n: u64) {
    let adst: *mut () = dst as *mut ();
    let asrc: *const () = src as *const ();
    _mi_memcpy(adst, asrc, n);
}

fn _mi_memzero_aligned(dst: *mut (), n: u64) {
    let adst: *mut () = dst as *mut ();
    _mi_memzero(adst, n);
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _pending_call {
    func: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
    flags: i32,
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
struct trampoline_api_st {
    init_state: Option<unsafe extern "C-unwind" fn() -> *mut ()>,
    write_state: Option<unsafe extern "C-unwind" fn(*mut (), *const (), u32, *mut PyCodeObject)>,
    free_state: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    state: *mut (),
    code_padding: i64,
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
struct _ceval_runtime_state {
    perf: __SlateRecord_pycore_interp_structs_h_81,
    pending_mainthread: _pending_calls,
    unused_sys_trace_profile_mutex: PyMutex,
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
struct atexit_state {
    ll_callbacks: *mut atexit_callback,
    callbacks: *mut _object,
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

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_40 {
    ru_maxrss: i64,
    __ru_maxrss_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_47 {
    ru_ixrss: i64,
    __ru_ixrss_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_53 {
    ru_idrss: i64,
    __ru_idrss_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_59 {
    ru_isrss: i64,
    __ru_isrss_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_66 {
    ru_minflt: i64,
    __ru_minflt_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_72 {
    ru_majflt: i64,
    __ru_majflt_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_78 {
    ru_nswap: i64,
    __ru_nswap_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_85 {
    ru_inblock: i64,
    __ru_inblock_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_91 {
    ru_oublock: i64,
    __ru_oublock_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_97 {
    ru_msgsnd: i64,
    __ru_msgsnd_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_103 {
    ru_msgrcv: i64,
    __ru_msgrcv_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_109 {
    ru_nsignals: i64,
    __ru_nsignals_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_117 {
    ru_nvcsw: i64,
    __ru_nvcsw_word: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_struct_rusage_h_124 {
    ru_nivcsw: i64,
    __ru_nivcsw_word: i64,
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

#[repr(C, align(64))]
struct __SlateAlign64<T>(T);

unsafe impl<T> Sync for __SlateAlign64<T> {}

mod __slate_bits {
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
    pub struct __SlateBits277U0 {
        #[bits(1)]
        pub is_committed: u8,
        #[bits(1)]
        pub is_zero_init: u8,
        #[bits(1)]
        pub use_qsbr: u8,
        #[bits(4)]
        pub tag: u8,
        #[bits(1, access = na)]
        pub __slate_pad_4: u8,
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
    pub struct __SlateBits274U0 {
        #[bits(1)]
        pub in_full: u8,
        #[bits(1)]
        pub has_aligned: u8,
        #[bits(6, access = na)]
        pub __slate_pad_2: u8,
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
    pub struct __SlateBits277U1 {
        #[bits(1)]
        pub free_is_zero: u8,
        #[bits(7)]
        pub retire_expire: u8,
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
    pub struct __SlateBits157U0 {
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
    pub struct __SlateBits201U0 {
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

fn _Py_IsMainInterpreter(interp: *mut _is) -> i32 {
    (interp == unsafe { _PyInterpreterState_Main() }) as i32
}

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

// Max size threshold below which malloc requests are considered to be
// small enough in order to use preallocated memory pools. You can tune
// this value according to your application behaviour and memory needs.
//
// Note: a size threshold of 512 guarantees that newly created dictionaries
// will be allocated from preallocated memory pools on 64-bit.
//
// The following invariants must hold:
//      1) ALIGNMENT <= SMALL_REQUEST_THRESHOLD <= 512
//      2) SMALL_REQUEST_THRESHOLD is evenly divisible by ALIGNMENT
//
// Although not required, for better performance and space efficiency,
// it is recommended that SMALL_REQUEST_THRESHOLD is set to a power of 2.
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

// size class of newly cached pools
#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_pools {
    used: [*mut pool_header; 64],
}

// How many arena_objects do we initially allocate?
// 16 = can allocate 16 arenas = 16 * ARENA_SIZE = 4MB before growing the
// `arenas` vector.
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
struct _obmalloc_usage {
    arena_map_root: arena_map_top,
    arena_map_mid_count: i32,
    arena_map_bot_count: i32,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124 {
    si_signo: i32,
    si_errno: i32,
    si_code: i32,
    __pad0: i32,
    _sifields: __SlateRecord_siginfo_t_h_124_2,
}

use slate_support::abort;
use slate_support::calloc;
use slate_support::clock_gettime;
use slate_support::free;
use slate_support::getenv;
use slate_support::madvise;
use slate_support::malloc;
use slate_support::memchr;
use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::memset;
use slate_support::mprotect;
use slate_support::munmap;
use slate_support::pthread_key_create;
use slate_support::pthread_setspecific;
use slate_support::realloc;
use slate_support::realpath;
use slate_support::snprintf;
use slate_support::strcmp;
use slate_support::strlen;
use slate_support::strstr;
use slate_support::syscall;
use slate_support::sysconf;
use slate_support::wcslen;

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
struct __SlateRecord_pycore_asdl_h_45 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [i32; 1],
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_global_strings {
    literals: __SlateRecord_pycore_global_strings_h_32,
    identifiers: __SlateRecord_pycore_global_strings_h_58,
    ascii: [__SlateRecord_pycore_global_strings_h_815; 128],
    latin1: [__SlateRecord_pycore_global_strings_h_819; 128],
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

fn _PyRuntimeState_GetFinalizing(runtime: *mut pyruntimestate) -> *mut _ts {
    _Py_atomic_load_ptr_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*runtime)._finalizing) }) as *const (),
    ) as *mut _ts
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_os_mem_config_s {
    page_size: u64,
    large_page_size: u64,
    alloc_granularity: u64,
    has_overcommit: bool,
    must_free_whole: bool,
    has_virtual_reserve: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_process_info_s {
    elapsed: i64,
    utime: i64,
    stime: i64,
    current_rss: u64,
    peak_rss: u64,
    current_commit: u64,
    peak_commit: u64,
    page_faults: u64,
}

fn mi_prim_tls_slot(slot: u64) -> *mut () {
    let mut res: *mut () = unsafe { core::mem::zeroed() };
    let mut ofs: u64 = slot.wrapping_mul(core::mem::size_of::<*mut ()>() as u64);
    unsafe {
        let __t0: u64;
        core::arch::asm!("movq %fs:({1:r}), {0:r}", lateout(reg) __t0, in(reg) ofs as *mut *mut (), options(pure, readonly, nostack, att_syntax));
        res = unsafe { core::mem::transmute::<u64, *mut ()>(__t0 as u64) };
    }
    // x86_64 Linux, BSD uses FS
    res
}

fn _mi_prim_thread_id() -> u64 {
    // in all our other targets, slot 0 is the thread id
    //
    // glibc: https://sourceware.org/git/?p=glibc.git;a=blob_plain;f=sysdeps/x86_64/nptl/tls.h
    //
    // apple: https://github.com/apple/darwin-xnu/blob/main/libsyscall/os/tsd.h#L36
    mi_prim_tls_slot(0u64) as u64
}

fn mi_prim_get_default_heap() -> *mut mi_heap_s {
    unsafe { _mi_heap_default }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_malloc(
    heap: *mut mi_heap_s,
    page: *mut mi_page_s,
    size: u64,
    zero: bool,
) -> *mut () {
    let block: *mut mi_block_s = unsafe { (*page).free };
    if block.is_null() {
        return _mi_malloc_generic(heap, size, zero, 0u64);
    }
    // pop from the free list
    let __v1: u32 = unsafe { (*page).used };
    unsafe {
        (*page).used = __v1.wrapping_add(1);
    }
    unsafe {
        (*page).free = mi_block_next(page as *const mi_page_s, block as *const mi_block_s);
    }
    // allow use of the block internally
    //
    // note: when tracking we need to avoid ever touching the MI_PADDING since
    //
    // that is tracked by valgrind etc. as non-accessible (through the red-zone, see `mimalloc/track.h`)
    // zero the block? note: we need to zero the full block size (issue #63)
    if zero {
        // do not call with zero'ing for huge blocks (see _mi_malloc_generic)
        if ((unsafe { (*page).__slate_bits_1.__get_free_is_zero() }) as i32) != 0 {
            unsafe {
                core::sync::atomic::AtomicU64::from_ptr(
                    core::ptr::addr_of_mut!((*block).next) as *mut u64
                )
                .store(0u64, core::sync::atomic::Ordering::SeqCst)
            };
        } else {
            _mi_memzero_aligned(
                block as *mut (),
                unsafe { (*page).xblock_size }.wrapping_sub(0) as u64,
            );
        }
    }
    block as *mut ()
}

fn mi_heap_malloc_small_zero(heap: *mut mi_heap_s, size: u64, zero: bool) -> *mut () {
    let page: *mut mi_page_s = _mi_heap_get_free_small_page(heap, size.wrapping_add(0));
    let p: *mut () = _mi_page_malloc(heap, page, size.wrapping_add(0), zero);
    mi_track_malloc!(p);
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_malloc_small(heap: *mut mi_heap_s, size: u64) -> *mut () {
    mi_heap_malloc_small_zero(heap, size, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc_small(size: u64) -> *mut () {
    mi_heap_malloc_small(mi_prim_get_default_heap(), size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_malloc_zero_ex(
    heap: *mut mi_heap_s,
    size: u64,
    zero: bool,
    huge_alignment: u64,
) -> *mut () {
    if size <= 128u64.wrapping_mul(core::mem::size_of::<*mut ()>() as u64) {
        return mi_heap_malloc_small_zero(heap, size, zero);
    } else {
        // heaps are thread local
        let p: *mut () = _mi_malloc_generic(heap, size.wrapping_add(0), zero, huge_alignment); // note: size can overflow but it is detected in malloc_generic
        mi_track_malloc!(p);
        return p;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_malloc_zero(heap: *mut mi_heap_s, size: u64, zero: bool) -> *mut () {
    _mi_heap_malloc_zero_ex(heap, size, zero, 0u64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_malloc(heap: *mut mi_heap_s, size: u64) -> *mut () {
    _mi_heap_malloc_zero(heap, size, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc(size: u64) -> *mut () {
    mi_heap_malloc(mi_prim_get_default_heap(), size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_zalloc_small(size: u64) -> *mut () {
    mi_heap_malloc_small_zero(mi_prim_get_default_heap(), size, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_zalloc(heap: *mut mi_heap_s, size: u64) -> *mut () {
    _mi_heap_malloc_zero(heap, size, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_zalloc(size: u64) -> *mut () {
    mi_heap_zalloc(mi_prim_get_default_heap(), size)
}

fn mi_check_is_double_free(page: *const mi_page_s, block: *const mi_block_s) -> bool {
    MI_UNUSED!(page);
    MI_UNUSED!(block);
    false
}

fn mi_page_usable_size_of(page: *const mi_page_s, block: *const mi_block_s) -> u64 {
    MI_UNUSED!(block);
    mi_page_usable_block_size(page)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_padding_shrink(
    page: *const mi_page_s,
    block: *const mi_block_s,
    min_size: u64,
) {
    MI_UNUSED!(page);
    MI_UNUSED!(block);
    MI_UNUSED!(min_size);
}

fn mi_check_padding(page: *const mi_page_s, block: *const mi_block_s) {
    MI_UNUSED!(page);
    MI_UNUSED!(block);
}

fn mi_stat_free(page: *const mi_page_s, block: *const mi_block_s) {
    MI_UNUSED!(page);
    MI_UNUSED!(block);
}

#[inline(never)]
fn _mi_free_block_mt(page: *mut mi_page_s, block: *mut mi_block_s) {
    // The padding check may access the non-thread-owned page for the key values.
    //
    // that is safe as these are constant and the page won't be freed (as the block is not freed yet).
    mi_check_padding(page as *const mi_page_s, block as *const mi_block_s);
    _mi_padding_shrink(page as *const mi_page_s, block as *const mi_block_s, 8u64); // for small size, ensure we can fit the delayed thread pointers without triggering overflow detection
    // huge page segments are always abandoned and can be freed immediately
    let segment: *mut mi_segment_s = _mi_page_segment(page as *const mi_page_s);
    if ((unsafe { (*segment).kind }) as u32) == 1 {
        // huge pages are special as they occupy the entire segment
        //
        // as these are large we reset the memory occupied by the page so it is available to other threads
        //
        // (as the owning thread needs to actually free the memory later).
        _mi_segment_huge_page_reset(segment, page, block);
    }
    // Try to put the block on either the page-local thread free list, or the heap delayed free list.
    let mut tfreex: u64 = 0;
    let mut use_delayed: bool = false;
    let mut tfree: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*page).xthread_free) });
    loop {
        use_delayed = (mi_tf_delayed(tfree) as u32) == 0;
        if use_delayed {
            // unlikely: this only happens on the first concurrent free in a page that is in the full list
            tfreex = mi_tf_set_delayed(tfree, 1u32);
        } else {
            // usual: directly add to page thread_free list
            mi_block_set_next(
                page as *const mi_page_s,
                block,
                mi_tf_block(tfree) as *const mi_block_s,
            );
            tfreex = mi_tf_set_block(tfree, block);
        }
        if !(!mi_atomic_cas_weak_release!(
            unsafe { core::ptr::addr_of_mut!((*page).xthread_free) },
            core::ptr::addr_of_mut!(tfree),
            tfreex
        )) {
            break;
        }
    }
    if use_delayed {
        // racy read on `heap`, but ok because MI_DELAYED_FREEING is set (see `mi_heap_delete` and `mi_heap_collect_abandon`)
        let heap: *mut mi_heap_s =
            mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!((*page).xheap) })
                as *mut mi_heap_s; // mi_page_heap(page);
        if !heap.is_null() {
            // add to the delayed free list of this heap. (do this atomically as the lock only protects heap memory validity)
            let mut dfree: *mut mi_block_s = mi_atomic_load_ptr_relaxed__3!(unsafe {
                core::ptr::addr_of_mut!((*heap).thread_delayed_free)
            });
            loop {
                mi_block_set_nextx(
                    heap as *const (),
                    block,
                    dfree as *const mi_block_s,
                    (unsafe { (*heap).keys.as_mut_ptr() as *mut u64 }) as *const u64,
                );
                if !(!mi_atomic_cas_ptr_weak_release!(
                    unsafe { core::ptr::addr_of_mut!((*heap).thread_delayed_free) },
                    core::ptr::addr_of_mut!(dfree),
                    block
                )) {
                    break;
                }
            }
        }
        // and reset the MI_DELAYED_FREEING flag
        tfree = mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*page).xthread_free) });
        loop {
            tfreex = tfree;
            tfreex = mi_tf_set_delayed(tfree, 2u32);
            if !(!mi_atomic_cas_weak_release!(
                unsafe { core::ptr::addr_of_mut!((*page).xthread_free) },
                core::ptr::addr_of_mut!(tfree),
                tfreex
            )) {
                break;
            }
        }
    }
}

fn _mi_free_block(page: *mut mi_page_s, local: bool, block: *mut mi_block_s) {
    // and push it on the free list
    //
    // const size_t bsize = mi_page_block_size(page);
    if local {
        // owning thread can free a block directly
        if (!(!mi_check_is_double_free(page as *const mi_page_s, block as *const mi_block_s))
            as i64)
            != 0
        {
            return;
        }
        mi_check_padding(page as *const mi_page_s, block as *const mi_block_s);
        mi_block_set_next(
            page as *const mi_page_s,
            block,
            (unsafe { (*page).local_free }) as *const mi_block_s,
        );
        unsafe {
            (*page).local_free = block;
        }
        let __v1: u32 = unsafe { (*page).used };
        unsafe {
            (*page).used = __v1.wrapping_sub(1);
        }
        if (!(!mi_page_all_free(page as *const mi_page_s)) as i64) != 0 {
            _mi_page_retire(page);
        } else {
            if (!(!mi_page_is_in_full(page as *const mi_page_s)) as i64) != 0 {
                _mi_page_unfull(page);
            }
        }
    } else {
        _mi_free_block_mt(page, block);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_ptr_unalign(
    segment: *const mi_segment_s,
    page: *const mi_page_s,
    p: *const (),
) -> *mut mi_block_s {
    let diff: u64 = ((unsafe {
        (p as *mut u8)
            .offset_from(_mi_page_start(segment, page, core::ptr::null_mut::<u64>()) as *mut u8)
    }) as i64) as u64;
    let adjust: u64 = diff % mi_page_block_size(page);
    (p as u64).wrapping_sub(adjust) as *mut mi_block_s
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_free_generic(
    segment: *const mi_segment_s,
    page: *mut mi_page_s,
    is_local: bool,
    p: *mut (),
) {
    let __v0: *mut mi_block_s = if mi_page_has_aligned(page as *const mi_page_s) {
        _mi_page_ptr_unalign(segment, page as *const mi_page_s, p as *const ())
    } else {
        p as *mut mi_block_s
    };
    let block: *mut mi_block_s = __v0;
    mi_stat_free(page as *const mi_page_s, block as *const mi_block_s); // stat_free may access the padding
    _mi_free_block(page, is_local, block);
}

fn mi_checked_ptr_segment(p: *const (), msg: *const i8) -> *mut mi_segment_s {
    MI_UNUSED!(msg);
    let segment: *mut mi_segment_s = _mi_ptr_segment(p);
    segment
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_free(p: *mut ()) {
    if p.is_null() {
        return;
    }
    let segment: *mut mi_segment_s = mi_checked_ptr_segment(p as *const (), c"mi_free".as_ptr());
    let is_local: bool = _mi_prim_thread_id()
        == mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*segment).thread_id) });
    let page: *mut mi_page_s = _mi_segment_page_of(segment as *const mi_segment_s, p as *const ());
    if is_local {
        // thread-local free?
        if ((unsafe { (*page).flags.full_aligned }) as i32) == 0 {
            let block: *mut mi_block_s = p as *mut mi_block_s;
            if (!(!mi_check_is_double_free(page as *const mi_page_s, block as *const mi_block_s))
                as i64)
                != 0
            {
                return;
            }
            mi_check_padding(page as *const mi_page_s, block as *const mi_block_s);
            mi_stat_free(page as *const mi_page_s, block as *const mi_block_s);
            // faster then mi_usable_size as we already know the page and that p is unaligned
            mi_block_set_next(
                page as *const mi_page_s,
                block,
                (unsafe { (*page).local_free }) as *const mi_block_s,
            );
            unsafe {
                (*page).local_free = block;
            }
            let __v1: u32 = unsafe { (*page).used };
            let __v2: u32 = __v1.wrapping_sub(1);
            unsafe {
                (*page).used = __v2;
            }
            if __v2 == 0 {
                // using this expression generates better code than: page->used--; if (mi_page_all_free(page))
                _mi_page_retire(page);
            }
        } else {
            // page is full or contains (inner) aligned blocks; use generic path
            _mi_free_generic(segment as *const mi_segment_s, page, true, p);
        }
    // and it is not a full page (full pages need to move from the full bin), nor has aligned blocks (aligned blocks need to be unaligned)
    } else {
        // not thread-local; use generic path
        _mi_free_generic(segment as *const mi_segment_s, page, false, p);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_free_delayed_block(block: *mut mi_block_s) -> bool {
    // get segment and page
    let segment: *const mi_segment_s = _mi_ptr_segment(block as *const ()) as *const mi_segment_s;
    // The GC traverses heaps of other threads, which can trigger this assert.
    let page: *mut mi_page_s = _mi_segment_page_of(segment, block as *const ());
    // Clear the no-delayed flag so delayed freeing is used again for this page.
    //
    // This must be done before collecting the free lists on this page -- otherwise
    //
    // some blocks may end up in the page `thread_free` list with no blocks in the
    //
    // heap `thread_delayed_free` list which may cause the page to be never freed!
    //
    // (it would only be freed if we happen to scan it in `mi_page_queue_find_free_ex`)
    if !_mi_page_try_use_delayed_free(page, 0u32, false) {
        // dont overwrite never delayed
        return false;
    }
    // collect all other non-local frees to ensure up-to-date `used` count
    _mi_page_free_collect(page, false);
    // and free the block (possibly freeing the page as well since used is updated)
    _mi_free_block(page, true, block);
    true
}

#[inline(never)]
fn mi_page_usable_aligned_size_of(
    segment: *const mi_segment_s,
    page: *const mi_page_s,
    p: *const (),
) -> u64 {
    let block: *const mi_block_s = _mi_page_ptr_unalign(segment, page, p) as *const mi_block_s;
    let size: u64 = mi_page_usable_size_of(page, block);
    let adjust: i64 = (unsafe { (p as *mut u8).offset_from(block as *mut u8) }) as i64;
    size.wrapping_sub(adjust as u64)
}

fn _mi_usable_size(p: *const (), msg: *const i8) -> u64 {
    if p.is_null() {
        return 0;
    }
    let segment: *const mi_segment_s = mi_checked_ptr_segment(p, msg) as *const mi_segment_s;
    let page: *const mi_page_s = _mi_segment_page_of(segment, p) as *const mi_page_s;
    if (!(!(!mi_page_has_aligned(page))) as i64) != 0 {
        let block: *const mi_block_s = p as *const mi_block_s;
        return mi_page_usable_size_of(page, block);
    } else {
        // split out to separate routine for improved code generation
        return mi_page_usable_aligned_size_of(segment, page, p);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_usable_size(p: *const ()) -> u64 {
    _mi_usable_size(p, c"mi_usable_size".as_ptr())
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_free_size(p: *mut (), size: u64) {
    MI_UNUSED_RELEASE!(size);
    mi_free(p);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_free_size_aligned(p: *mut (), size: u64, alignment: u64) {
    MI_UNUSED_RELEASE!(alignment);
    mi_free_size(p, size);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_free_aligned(p: *mut (), alignment: u64) {
    MI_UNUSED_RELEASE!(alignment);
    mi_free(p);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_calloc(heap: *mut mi_heap_s, count: u64, size: u64) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(count, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_zalloc(heap, total)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_calloc(count: u64, size: u64) -> *mut () {
    mi_heap_calloc(mi_prim_get_default_heap(), count, size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_mallocn(heap: *mut mi_heap_s, count: u64, size: u64) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(count, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_malloc(heap, total)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_mallocn(count: u64, size: u64) -> *mut () {
    mi_heap_mallocn(mi_prim_get_default_heap(), count, size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_expand(p: *mut (), newsize: u64) -> *mut () {
    if p.is_null() {
        return core::ptr::null_mut::<()>();
    }
    let size: u64 = _mi_usable_size(p as *const (), c"mi_expand".as_ptr());
    if newsize > size {
        return core::ptr::null_mut::<()>();
    }
    return p; // it fits
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_realloc_zero(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    zero: bool,
) -> *mut () {
    // if p == NULL then behave as malloc.
    //
    // else if size == 0 then reallocate to a zero-sized block (and don't return NULL, just as mi_malloc(0)).
    //
    // (this means that returning NULL always indicates an error, and `p` will not have been freed in that case.)
    let size: u64 = _mi_usable_size(p as *const (), c"mi_realloc".as_ptr()); // also works if p == NULL (with size 0)
    if newsize <= size && newsize >= size / 2 && newsize > 0 {
        // note: newsize must be > 0 or otherwise we return NULL for realloc(NULL,0)
        // todo: do not track as the usable size is still the same in the free; adjust potential padding?
        //
        // mi_track_resize(p,size,newsize)
        //
        // if (newsize < size) { mi_track_mem_noaccess((uint8_t*)p + newsize, size - newsize); }
        return p; // reallocation still fits and not more than 50% waste
    }
    let newp: *mut () = mi_heap_malloc(heap, newsize);
    if !newp.is_null() {
        if zero && newsize > size {
            // also set last word in the previous allocation to zero to ensure any padding is zero-initialized
            let start: u64 = if size >= (core::mem::size_of::<i64>() as u64) {
                size.wrapping_sub(core::mem::size_of::<i64>() as u64)
            } else {
                0
            };
            _mi_memzero(
                (newp as *mut u8).wrapping_offset(start as isize) as *mut (),
                newsize.wrapping_sub(start),
            );
        } else {
            if newsize == 0 {
                unsafe {
                    *(newp as *mut u8).wrapping_offset(0 as isize) = 0;
                }
                // work around for applications that expect zero-reallocation to be zero initialized (issue #725)
            }
        }
        if !p.is_null() {
            let copysize: u64 = if newsize > size { size } else { newsize };
            // _mi_useable_size may be too large for byte precise memory tracking..
            _mi_memcpy(newp, p as *const (), copysize);
            mi_free(p); // only free the original pointer if successful
        }
    }
    newp
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_realloc(heap: *mut mi_heap_s, p: *mut (), newsize: u64) -> *mut () {
    _mi_heap_realloc_zero(heap, p, newsize, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_reallocn(
    heap: *mut mi_heap_s,
    p: *mut (),
    count: u64,
    size: u64,
) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(count, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_realloc(heap, p, total)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_reallocf(heap: *mut mi_heap_s, p: *mut (), newsize: u64) -> *mut () {
    let newp: *mut () = mi_heap_realloc(heap, p, newsize);
    if newp.is_null() && !p.is_null() {
        mi_free(p);
    }
    newp
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_rezalloc(heap: *mut mi_heap_s, p: *mut (), newsize: u64) -> *mut () {
    _mi_heap_realloc_zero(heap, p, newsize, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_recalloc(
    heap: *mut mi_heap_s,
    p: *mut (),
    count: u64,
    size: u64,
) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(count, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_rezalloc(heap, p, total)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_realloc(p: *mut (), newsize: u64) -> *mut () {
    mi_heap_realloc(mi_prim_get_default_heap(), p, newsize)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reallocn(p: *mut (), count: u64, size: u64) -> *mut () {
    mi_heap_reallocn(mi_prim_get_default_heap(), p, count, size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reallocf(p: *mut (), newsize: u64) -> *mut () {
    mi_heap_reallocf(mi_prim_get_default_heap(), p, newsize)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_rezalloc(p: *mut (), newsize: u64) -> *mut () {
    mi_heap_rezalloc(mi_prim_get_default_heap(), p, newsize)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_recalloc(p: *mut (), count: u64, size: u64) -> *mut () {
    mi_heap_recalloc(mi_prim_get_default_heap(), p, count, size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_strdup(heap: *mut mi_heap_s, s: *const i8) -> *mut i8 {
    if s.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    let n: u64 = unsafe { strlen(s as *const core::ffi::c_char) as u64 };
    let t: *mut i8 = mi_heap_malloc(heap, n.wrapping_add(1)) as *mut i8;
    if t.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    _mi_memcpy(t as *mut (), s as *const (), n);
    unsafe {
        *t.wrapping_offset(n as isize) = 0;
    }
    t
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_strdup(s: *const i8) -> *mut i8 {
    mi_heap_strdup(mi_prim_get_default_heap(), s)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_strndup(heap: *mut mi_heap_s, s: *const i8, n: u64) -> *mut i8 {
    if s.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    let end: *const i8 = ((unsafe {
        memchr(
            (s as *const ()) as *const core::ffi::c_void,
            0i32 as i32,
            n as usize,
        ) as *mut ()
    }) as *const ()) as *const i8; // find end of string in the first `n` characters (returns NULL if not found)
    let m: u64 = if !end.is_null() {
        ((unsafe { end.offset_from(s as *const i8) }) as i64) as u64
    } else {
        n
    }; // `m` is the minimum of `n` or the end-of-string
    let t: *mut i8 = mi_heap_malloc(heap, m.wrapping_add(1)) as *mut i8;
    if t.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    _mi_memcpy(t as *mut (), s as *const (), m);
    unsafe {
        *t.wrapping_offset(m as isize) = 0;
    }
    t
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_strndup(s: *const i8, n: u64) -> *mut i8 {
    mi_heap_strndup(mi_prim_get_default_heap(), s, n)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_realpath(
    heap: *mut mi_heap_s,
    fname: *const i8,
    resolved_name: *mut i8,
) -> *mut i8 {
    if !resolved_name.is_null() {
        return unsafe {
            realpath(
                fname as *const core::ffi::c_char,
                resolved_name as *mut core::ffi::c_char,
            ) as *mut i8
        };
    } else {
        let rname: *mut i8 = unsafe {
            realpath(
                fname as *const core::ffi::c_char,
                core::ptr::null_mut::<i8>() as *mut core::ffi::c_char,
            ) as *mut i8
        };
        if rname.is_null() {
            return core::ptr::null_mut::<i8>();
        }
        let result: *mut i8 = mi_heap_strdup(heap, rname as *const i8);
        unsafe { free((rname as *mut ()) as *mut core::ffi::c_void) }; // use regular free! (which may be redirected to our free but that's ok)
        return result;
    }
    //   const size_t n  = mi_path_max();
    //   char* buf = (char*)mi_malloc(n+1);
    //   if (buf == NULL) {
    //     errno = ENOMEM;
    //     return NULL;
    //   }
    //   char* rname  = realpath(fname,buf);
    //   char* result = mi_heap_strndup(heap,rname,n); // ok if `rname==NULL`
    //   mi_free(buf);
    //   return result;
    // }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_realpath(fname: *const i8, resolved_name: *mut i8) -> *mut i8 {
    mi_heap_realpath(mi_prim_get_default_heap(), fname, resolved_name)
}

#[unsafe(no_mangle)]
#[linkage = "weak"]
extern "C-unwind" fn _Py__ZSt15get_new_handlerv() -> Option<unsafe extern "C-unwind" fn()> {
    None
}

fn mi_get_new_handler() -> Option<unsafe extern "C-unwind" fn()> {
    _Py__ZSt15get_new_handlerv()
}

fn mi_try_new_handler(nothrow: bool) -> bool {
    let h: Option<unsafe extern "C-unwind" fn()> = mi_get_new_handler();
    if h == None {
        unsafe {
            _mi_error_message(
                slate_support::ENOMEM as i32,
                c"out of memory in 'new'".as_ptr(),
            )
        };
        if !nothrow {
            unsafe { std::process::abort() }; // cannot throw in plain C, use abort
        }
        return false;
    } else {
        unsafe { h.unwrap()() };
        return true;
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_try_new(heap: *mut mi_heap_s, size: u64, nothrow: bool) -> *mut () {
    let mut p: *mut () = core::ptr::null_mut::<()>();
    loop {
        let __v0: bool = p.is_null() && mi_try_new_handler(nothrow);
        if !__v0 {
            break;
        }
        p = mi_heap_malloc(heap, size);
    }
    p
}

#[inline(never)]
fn mi_try_new(size: u64, nothrow: bool) -> *mut () {
    mi_heap_try_new(mi_prim_get_default_heap(), size, nothrow)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_alloc_new(heap: *mut mi_heap_s, size: u64) -> *mut () {
    let p: *mut () = mi_heap_malloc(heap, size);
    if p.is_null() {
        return mi_heap_try_new(heap, size, false);
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new(size: u64) -> *mut () {
    mi_heap_alloc_new(mi_prim_get_default_heap(), size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_alloc_new_n(heap: *mut mi_heap_s, count: u64, size: u64) -> *mut () {
    let mut total: u64 = 0;
    if (!(!mi_count_size_overflow(count, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    })) as i64)
        != 0
    {
        mi_try_new_handler(false); // on overflow we invoke the try_new_handler once to potentially throw std::bad_alloc
        return core::ptr::null_mut::<()>();
    } else {
        return mi_heap_alloc_new(heap, total);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new_n(count: u64, size: u64) -> *mut () {
    mi_heap_alloc_new_n(mi_prim_get_default_heap(), size, count)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new_nothrow(size: u64) -> *mut () {
    let p: *mut () = mi_malloc(size);
    if p.is_null() {
        return mi_try_new(size, true);
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new_aligned(size: u64, alignment: u64) -> *mut () {
    let mut p: *mut () = unsafe { core::mem::zeroed() };
    loop {
        p = mi_malloc_aligned(size, alignment);
        let __v1: bool = p.is_null() && mi_try_new_handler(false);
        if !__v1 {
            break;
        }
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new_aligned_nothrow(size: u64, alignment: u64) -> *mut () {
    let mut p: *mut () = unsafe { core::mem::zeroed() };
    loop {
        p = mi_malloc_aligned(size, alignment);
        let __v1: bool = p.is_null() && mi_try_new_handler(true);
        if !__v1 {
            break;
        }
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new_realloc(p: *mut (), newsize: u64) -> *mut () {
    let mut q: *mut () = unsafe { core::mem::zeroed() };
    loop {
        q = mi_realloc(p, newsize);
        let __v1: bool = q.is_null() && mi_try_new_handler(false);
        if !__v1 {
            break;
        }
    }
    q
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_new_reallocn(p: *mut (), newcount: u64, size: u64) -> *mut () {
    let mut total: u64 = 0;
    if (!(!mi_count_size_overflow(newcount, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    })) as i64)
        != 0
    {
        mi_try_new_handler(false); // on overflow we invoke the try_new_handler once to potentially throw std::bad_alloc
        return core::ptr::null_mut::<()>();
    } else {
        return mi_new_realloc(p, total);
    }
}

#[inline(never)]
fn mi_heap_malloc_zero_aligned_at_fallback(
    heap: *mut mi_heap_s,
    size: u64,
    alignment: u64,
    offset: u64,
    zero: bool,
) -> *mut () {
    let align_mask: u64 = alignment.wrapping_sub(1); // for any x, `(x & align_mask) == (x % alignment)`
    let padsize: u64 = size.wrapping_add(0);
    // use regular allocation if it is guaranteed to fit the alignment constraints
    if offset == 0 && alignment <= padsize && padsize <= 128u64 && padsize & align_mask == 0 {
        let p: *mut () = _mi_heap_malloc_zero(heap, size, zero);
        return p;
    }
    let mut p: *mut () = unsafe { core::mem::zeroed() };
    let mut oversize: u64 = 0;
    if alignment > 16777216u64 {
        // use OS allocation for very large alignment and allocate inside a huge page (dedicated segment with 1 page)
        //
        // This can support alignments >= MI_SEGMENT_SIZE by ensuring the object can be aligned at a point in the
        //
        // first (and single) page such that the segment info is `MI_SEGMENT_SIZE` bytes before it (so it can be found by aligning the pointer down)
        if offset != 0 {
            // todo: cannot support offset alignment for very large alignments yet
            return core::ptr::null_mut::<()>();
        }
        oversize = if size <= 128u64.wrapping_mul(core::mem::size_of::<*mut ()>() as u64) {
            128u64
                .wrapping_mul(core::mem::size_of::<*mut ()>() as u64)
                .wrapping_add(1)
        } else {
            size
        }; // ensure we use generic malloc path
        p = _mi_heap_malloc_zero_ex(heap, oversize, false, alignment); // the page block size should be large enough to align in the single huge page block
        // zero afterwards as only the area from the aligned_p may be committed!
        if p.is_null() {
            return core::ptr::null_mut::<()>();
        }
    } else {
        // otherwise over-allocate
        oversize = size.wrapping_add(alignment).wrapping_sub(1);
        p = _mi_heap_malloc_zero(heap, oversize, zero);
        if p.is_null() {
            return core::ptr::null_mut::<()>();
        }
    }
    // .. and align within the allocation
    let poffset: u64 = (p as u64).wrapping_add(offset) & align_mask;
    let adjust: u64 = if poffset == 0 {
        0
    } else {
        alignment.wrapping_sub(poffset)
    };
    let aligned_p: *mut () = (p as u64).wrapping_add(adjust) as *mut ();
    if aligned_p != p {
        let page: *mut mi_page_s = _mi_ptr_page(p);
        mi_page_set_has_aligned(page, true);
        _mi_padding_shrink(
            page as *const mi_page_s,
            p as *const mi_block_s,
            adjust.wrapping_add(size),
        );
    }
    // todo: expand padding if overallocated ?
    // now zero the block if needed
    if alignment > 16777216u64 {
        // for the tracker, on huge aligned allocations only from the start of the large block is defined
        if zero {
            _mi_memzero_aligned(aligned_p, mi_usable_size(aligned_p as *const ()));
        }
    }
    aligned_p
}

fn mi_heap_malloc_zero_aligned_at(
    heap: *mut mi_heap_s,
    size: u64,
    alignment: u64,
    offset: u64,
    zero: bool,
) -> *mut () {
    // note: we don't require `size > offset`, we just guarantee that the address at offset is aligned regardless of the allocated size.
    let __v0: bool = alignment == 0 || !_mi_is_power_of_two(alignment);
    if __v0 {
        // require power-of-two (see <https://en.cppreference.com/w/c/memory/aligned_alloc>)
        return core::ptr::null_mut::<()>();
    }
    if size > 9223372036854775807 {
        // we don't allocate more than PTRDIFF_MAX (see <https://sourceware.org/ml/libc-announce/2019/msg00001.html>)
        return core::ptr::null_mut::<()>();
    }
    let align_mask: u64 = alignment.wrapping_sub(1); // for any x, `(x & align_mask) == (x % alignment)`
    let padsize: u64 = size.wrapping_add(0); // note: cannot overflow due to earlier size > PTRDIFF_MAX check
    // try first if there happens to be a small block available with just the right alignment
    if padsize <= 128u64.wrapping_mul(core::mem::size_of::<*mut ()>() as u64)
        && alignment <= padsize
    {
        let page: *mut mi_page_s = _mi_heap_get_free_small_page(heap, padsize);
        let is_aligned: bool =
            ((unsafe { (*page).free }) as u64).wrapping_add(offset) & align_mask == 0;
        if !unsafe { (*page).free }.is_null() && is_aligned {
            let p: *mut () = _mi_page_malloc(heap, page, padsize, zero); // TODO: inline _mi_page_malloc
            mi_track_malloc!(p);
            return p;
        }
    }
    // fallback
    mi_heap_malloc_zero_aligned_at_fallback(heap, size, alignment, offset, zero)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_malloc_aligned_at(
    heap: *mut mi_heap_s,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_malloc_zero_aligned_at(heap, size, alignment, offset, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_malloc_aligned(
    heap: *mut mi_heap_s,
    size: u64,
    alignment: u64,
) -> *mut () {
    let __v0: bool = alignment == 0 || !_mi_is_power_of_two(alignment);
    if __v0 {
        return core::ptr::null_mut::<()>();
    }
    // without padding, any small sized allocation is naturally aligned (see also `_mi_segment_page_start`)
    if _mi_is_power_of_two(size)
        && size >= alignment
        && size <= 128u64.wrapping_mul(core::mem::size_of::<*mut ()>() as u64)
    {
        // fast path for common alignment and size
        return mi_heap_malloc_small(heap, size);
    } else {
        return mi_heap_malloc_aligned_at(heap, size, alignment, 0u64);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_zalloc_aligned_at(
    heap: *mut mi_heap_s,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_malloc_zero_aligned_at(heap, size, alignment, offset, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_zalloc_aligned(
    heap: *mut mi_heap_s,
    size: u64,
    alignment: u64,
) -> *mut () {
    mi_heap_zalloc_aligned_at(heap, size, alignment, 0u64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_calloc_aligned_at(
    heap: *mut mi_heap_s,
    count: u64,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(count, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_zalloc_aligned_at(heap, total, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_calloc_aligned(
    heap: *mut mi_heap_s,
    count: u64,
    size: u64,
    alignment: u64,
) -> *mut () {
    mi_heap_calloc_aligned_at(heap, count, size, alignment, 0u64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc_aligned_at(size: u64, alignment: u64, offset: u64) -> *mut () {
    mi_heap_malloc_aligned_at(mi_prim_get_default_heap(), size, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc_aligned(size: u64, alignment: u64) -> *mut () {
    mi_heap_malloc_aligned(mi_prim_get_default_heap(), size, alignment)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_zalloc_aligned_at(size: u64, alignment: u64, offset: u64) -> *mut () {
    mi_heap_zalloc_aligned_at(mi_prim_get_default_heap(), size, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_zalloc_aligned(size: u64, alignment: u64) -> *mut () {
    mi_heap_zalloc_aligned(mi_prim_get_default_heap(), size, alignment)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_calloc_aligned_at(
    count: u64,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_calloc_aligned_at(mi_prim_get_default_heap(), count, size, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_calloc_aligned(count: u64, size: u64, alignment: u64) -> *mut () {
    mi_heap_calloc_aligned(mi_prim_get_default_heap(), count, size, alignment)
}

fn mi_heap_realloc_zero_aligned_at(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    alignment: u64,
    offset: u64,
    zero: bool,
) -> *mut () {
    if alignment <= (core::mem::size_of::<u64>() as u64) {
        return _mi_heap_realloc_zero(heap, p, newsize, zero);
    }
    if p.is_null() {
        return mi_heap_malloc_zero_aligned_at(heap, newsize, alignment, offset, zero);
    }
    let size: u64 = mi_usable_size(p as *const ());
    if newsize <= size
        && newsize >= size.wrapping_sub(size / 2)
        && (p as u64).wrapping_add(offset) % alignment == 0
    {
        return p; // reallocation still fits, is aligned and not more than 50% waste
    } else {
        // note: we don't zero allocate upfront so we only zero initialize the expanded part
        let newp: *mut () = mi_heap_malloc_aligned_at(heap, newsize, alignment, offset);
        if !newp.is_null() {
            if zero && newsize > size {
                // also set last word in the previous allocation to zero to ensure any padding is zero-initialized
                let start: u64 = if size >= (core::mem::size_of::<i64>() as u64) {
                    size.wrapping_sub(core::mem::size_of::<i64>() as u64)
                } else {
                    0
                };
                _mi_memzero(
                    (newp as *mut u8).wrapping_offset(start as isize) as *mut (),
                    newsize.wrapping_sub(start),
                );
            }
            _mi_memcpy_aligned(
                newp,
                p as *const (),
                if newsize > size { size } else { newsize },
            );
            mi_free(p); // only free if successful
        }
        return newp;
    }
}

fn mi_heap_realloc_zero_aligned(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    alignment: u64,
    zero: bool,
) -> *mut () {
    if alignment <= (core::mem::size_of::<u64>() as u64) {
        return _mi_heap_realloc_zero(heap, p, newsize, zero);
    }
    let offset: u64 = (p as u64) % alignment; // use offset of previous allocation (p can be NULL)
    mi_heap_realloc_zero_aligned_at(heap, p, newsize, alignment, offset, zero)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_realloc_aligned_at(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_realloc_zero_aligned_at(heap, p, newsize, alignment, offset, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_realloc_aligned(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    alignment: u64,
) -> *mut () {
    mi_heap_realloc_zero_aligned(heap, p, newsize, alignment, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_rezalloc_aligned_at(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_realloc_zero_aligned_at(heap, p, newsize, alignment, offset, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_rezalloc_aligned(
    heap: *mut mi_heap_s,
    p: *mut (),
    newsize: u64,
    alignment: u64,
) -> *mut () {
    mi_heap_realloc_zero_aligned(heap, p, newsize, alignment, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_recalloc_aligned_at(
    heap: *mut mi_heap_s,
    p: *mut (),
    newcount: u64,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(newcount, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_rezalloc_aligned_at(heap, p, total, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_recalloc_aligned(
    heap: *mut mi_heap_s,
    p: *mut (),
    newcount: u64,
    size: u64,
    alignment: u64,
) -> *mut () {
    let mut total: u64 = 0;
    if mi_count_size_overflow(newcount, size, unsafe {
        &mut (*core::ptr::addr_of_mut!(total))
    }) {
        return core::ptr::null_mut::<()>();
    }
    mi_heap_rezalloc_aligned(heap, p, total, alignment)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_realloc_aligned_at(
    p: *mut (),
    newsize: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_realloc_aligned_at(mi_prim_get_default_heap(), p, newsize, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_realloc_aligned(p: *mut (), newsize: u64, alignment: u64) -> *mut () {
    mi_heap_realloc_aligned(mi_prim_get_default_heap(), p, newsize, alignment)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_rezalloc_aligned_at(
    p: *mut (),
    newsize: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_rezalloc_aligned_at(mi_prim_get_default_heap(), p, newsize, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_rezalloc_aligned(p: *mut (), newsize: u64, alignment: u64) -> *mut () {
    mi_heap_rezalloc_aligned(mi_prim_get_default_heap(), p, newsize, alignment)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_recalloc_aligned_at(
    p: *mut (),
    newcount: u64,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    mi_heap_recalloc_aligned_at(
        mi_prim_get_default_heap(),
        p,
        newcount,
        size,
        alignment,
        offset,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_recalloc_aligned(
    p: *mut (),
    newcount: u64,
    size: u64,
    alignment: u64,
) -> *mut () {
    mi_heap_recalloc_aligned(mi_prim_get_default_heap(), p, newcount, size, alignment)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc_size(p: *const ()) -> u64 {
    // if (!mi_is_in_heap_region(p)) return 0;
    mi_usable_size(p)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc_usable_size(p: *const ()) -> u64 {
    // if (!mi_is_in_heap_region(p)) return 0;
    mi_usable_size(p)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_malloc_good_size(size: u64) -> u64 {
    mi_good_size(size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_cfree(p: *mut ()) {
    if mi_is_in_heap_region(p as *const ()) {
        mi_free(p);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_posix_memalign(p: *mut *mut (), alignment: u64, size: u64) -> i32 {
    // Note: The spec dictates we should not modify `*p` on an error. (issue#27)
    //
    // <http://man7.org/linux/man-pages/man3/posix_memalign.3.html>
    if p.is_null() {
        return slate_support::EINVAL as i32;
    }
    if alignment % (core::mem::size_of::<*mut ()>() as u64) != 0 {
        return slate_support::EINVAL as i32;
    }
    // natural alignment
    // it is also required that alignment is a power of 2 and > 0; this is checked in `mi_malloc_aligned`
    let __v0: bool = alignment == 0 || !_mi_is_power_of_two(alignment);
    if __v0 {
        return slate_support::EINVAL as i32;
    }
    // not a power of 2
    let q: *mut () = mi_malloc_aligned(size, alignment);
    if q.is_null() && size != 0 {
        return slate_support::ENOMEM as i32;
    }
    unsafe {
        *p = q;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_memalign(alignment: u64, size: u64) -> *mut () {
    let p: *mut () = mi_malloc_aligned(size, alignment);
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_valloc(size: u64) -> *mut () {
    mi_memalign(_mi_os_page_size(), size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_pvalloc(size: u64) -> *mut () {
    let psize: u64 = _mi_os_page_size();
    if size >= 18446744073709551615u64.wrapping_sub(psize) {
        return core::ptr::null_mut::<()>();
    }
    // overflow
    let asize: u64 = _mi_align_up(size, psize);
    mi_malloc_aligned(asize, psize)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_aligned_alloc(alignment: u64, size: u64) -> *mut () {
    // C11 requires the size to be an integral multiple of the alignment, see <https://en.cppreference.com/w/c/memory/aligned_alloc>.
    //
    // unfortunately, it turns out quite some programs pass a size that is not an integral multiple so skip this check..
    //
    // if mi_unlikely((size & (alignment - 1)) != 0) { // C11 requires alignment>0 && integral multiple, see <https://en.cppreference.com/w/c/memory/aligned_alloc>
    //   #if MI_DEBUG > 0
    //   _mi_error_message(EOVERFLOW, "(mi_)aligned_alloc requires the size to be an integral multiple of the alignment (size %zu, alignment %zu)\n", size, alignment);
    //   #endif
    //   return NULL;
    // }
    //
    // C11 also requires alignment to be a power-of-two (and > 0) which is checked in mi_malloc_aligned
    let p: *mut () = mi_malloc_aligned(size, alignment);
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reallocarray(p: *mut (), count: u64, size: u64) -> *mut () {
    // BSD
    let newp: *mut () = mi_reallocn(p, count, size);
    if newp.is_null() {
        unsafe {
            *slate_support::errno_location() = slate_support::ENOMEM as i32;
        }
    }
    newp
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reallocarr(p: *mut (), count: u64, size: u64) -> i32 {
    // NetBSD
    if p.is_null() {
        unsafe {
            *slate_support::errno_location() = slate_support::EINVAL as i32;
        }
        return slate_support::EINVAL as i32;
    }
    let newp: *mut () = mi_reallocarray(unsafe { *(p as *mut *mut ()) }, count, size);
    if newp.is_null() {
        return unsafe { *slate_support::errno_location() };
    }
    unsafe {
        *(p as *mut *mut ()) = newp;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi__expand(p: *mut (), newsize: u64) -> *mut () {
    // Microsoft
    let res: *mut () = mi_expand(p, newsize);
    if res.is_null() {
        unsafe {
            *slate_support::errno_location() = slate_support::ENOMEM as i32;
        }
    }
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_wcsdup(s: *const u16) -> *mut u16 {
    if s.is_null() {
        return core::ptr::null_mut::<u16>();
    }
    let mut len: u64 = 0;
    while ((unsafe { *s.wrapping_offset(len as isize) }) as i32) != 0 {
        len = len.wrapping_add(1);
    }
    let size: u64 = len
        .wrapping_add(1)
        .wrapping_mul(core::mem::size_of::<u16>() as u64);
    let p: *mut u16 = mi_malloc(size) as *mut u16;
    if !p.is_null() {
        _mi_memcpy(p as *mut (), s as *const (), size);
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_mbsdup(s: *const u8) -> *mut u8 {
    mi_strdup(s as *const i8) as *mut u8
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_dupenv_s(buf: *mut *mut i8, size: *mut u64, name: *const i8) -> i32 {
    if buf.is_null() || name.is_null() {
        return slate_support::EINVAL as i32;
    }
    if !size.is_null() {
        unsafe {
            *size = 0;
        }
    }
    let p: *mut i8 = unsafe { getenv(name as *const core::ffi::c_char) as *mut i8 }; // mscver warning 4996
    if p.is_null() {
        unsafe {
            *buf = core::ptr::null_mut::<i8>();
        }
    } else {
        unsafe {
            *buf = mi_strdup(p as *const i8);
        }
        if unsafe { *buf }.is_null() {
            return slate_support::ENOMEM as i32;
        }
        if !size.is_null() {
            unsafe {
                *size = _mi_strlen(p as *const i8);
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_wdupenv_s(buf: *mut *mut u16, size: *mut u64, name: *const u16) -> i32 {
    if buf.is_null() || name.is_null() {
        return slate_support::EINVAL as i32;
    }
    if !size.is_null() {
        unsafe {
            *size = 0;
        }
    }
    // not supported
    unsafe {
        *buf = core::ptr::null_mut::<u16>();
    }
    slate_support::EINVAL as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_aligned_offset_recalloc(
    p: *mut (),
    newcount: u64,
    size: u64,
    alignment: u64,
    offset: u64,
) -> *mut () {
    // Microsoft
    mi_recalloc_aligned_at(p, newcount, size, alignment, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_aligned_recalloc(
    p: *mut (),
    newcount: u64,
    size: u64,
    alignment: u64,
) -> *mut () {
    // Microsoft
    mi_recalloc_aligned(p, newcount, size, alignment)
}

fn mi_bitmap_index_create(idx: u64, bitidx: u64) -> u64 {
    idx.wrapping_mul(64u64).wrapping_add(bitidx)
}

fn mi_bitmap_index_field(bitmap_idx: u64) -> u64 {
    bitmap_idx / 64u64
}

fn mi_bitmap_index_bit_in_field(bitmap_idx: u64) -> u64 {
    bitmap_idx % 64u64
}

fn mi_bitmap_index_bit(bitmap_idx: u64) -> u64 {
    bitmap_idx
}

// 32MiB
// not more than 126 (since we use 7 bits in the memid and an arena index + 1)
#[repr(C)]
#[derive(Clone, Copy)]
struct mi_arena_s {
    id: i32,
    memid: mi_memid_s,
    start: *mut u8,
    block_count: u64,
    field_count: u64,
    meta_size: u64,
    meta_memid: mi_memid_s,
    numa_node: i32,
    exclusive: bool,
    is_large: bool,
    search_idx: u64,
    purge_expire: i64,
    blocks_dirty: *mut u64,
    blocks_committed: *mut u64,
    blocks_purge: *mut u64,
    blocks_inuse: [u64; 1],
}

static mut mi_arenas: __SlateAlign64<[*mut mi_arena_s; 112]> =
    __SlateAlign64(unsafe { core::mem::zeroed() });

static mut mi_arena_count: __SlateAlign64<u64> = __SlateAlign64(unsafe { core::mem::zeroed() });

fn mi_arena_id_index(id: i32) -> u64 {
    (if id <= 0 { 112i32 } else { id.wrapping_sub(1) }) as u64
}

fn mi_arena_id_create(arena_index: u64) -> i32 {
    (arena_index as i32).wrapping_add(1)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_id_none() -> i32 {
    0
}

fn mi_arena_id_is_suitable(arena_id: i32, arena_is_exclusive: bool, req_arena_id: i32) -> bool {
    let __v0: bool = !arena_is_exclusive && req_arena_id == _mi_arena_id_none();
    __v0 || arena_id == req_arena_id
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_memid_is_suitable(memid: mi_memid_s, request_arena_id: i32) -> bool {
    if (memid.memkind as u32) == 6 {
        return mi_arena_id_is_suitable(
            unsafe { memid.mem.arena.id },
            unsafe { memid.mem.arena.is_exclusive },
            request_arena_id,
        );
    } else {
        return mi_arena_id_is_suitable(0i32, false, request_arena_id);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_memid_is_os_allocated(memid: mi_memid_s) -> bool {
    (memid.memkind as u32) == 3
}

fn mi_block_count_of_size(size: u64) -> u64 {
    _mi_divide_up(size, 33554432u64)
}

fn mi_arena_block_size(bcount: u64) -> u64 {
    bcount.wrapping_mul(33554432u64)
}

fn mi_arena_size(arena: *mut mi_arena_s) -> u64 {
    mi_arena_block_size(unsafe { (*arena).block_count })
}

fn mi_memid_create_arena(id: i32, is_exclusive: bool, bitmap_index: u64) -> mi_memid_s {
    let mut memid: mi_memid_s = _mi_memid_create(6u32);
    unsafe {
        memid.mem.arena.id = id;
    }
    unsafe {
        memid.mem.arena.block_index = bitmap_index;
    }
    unsafe {
        memid.mem.arena.is_exclusive = is_exclusive;
    }
    return memid;
}

fn mi_arena_memid_indices(
    memid: mi_memid_s,
    arena_index: &mut u64,
    bitmap_index: &mut u64,
) -> bool {
    unsafe {
        *(arena_index as *mut u64) = mi_arena_id_index(memid.mem.arena.id);
    }
    unsafe {
        *(bitmap_index as *mut u64) = memid.mem.arena.block_index;
    }
    unsafe { memid.mem.arena.is_exclusive }
}

// 8 KiB on 64-bit
static mut mi_arena_static: __SlateAlign16<[u8; 8192]> =
    __SlateAlign16(unsafe { core::mem::zeroed() });

static mut mi_arena_static_top: u64 = unsafe { core::mem::zeroed() };

fn mi_arena_static_zalloc(size: u64, mut alignment: u64, memid: *mut mi_memid_s) -> *mut () {
    unsafe {
        *memid = _mi_memid_none();
    }
    if size == 0 || size > 8192u64 {
        return core::ptr::null_mut::<()>();
    }
    if mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(mi_arena_static_top) })
        .wrapping_add(size)
        > 8192u64
    {
        return core::ptr::null_mut::<()>();
    }
    // try to claim space
    if alignment == 0 {
        alignment = 1;
    }
    let oversize: u64 = size.wrapping_add(alignment).wrapping_sub(1);
    if oversize > 8192u64 {
        return core::ptr::null_mut::<()>();
    }
    let oldtop: u64 = mi_atomic_add_acq_rel!(
        unsafe { core::ptr::addr_of_mut!(mi_arena_static_top) },
        oversize
    );
    let mut top: u64 = oldtop.wrapping_add(oversize);
    if top > 8192u64 {
        // try to roll back, ok if this fails
        mi_atomic_cas_strong_acq_rel__2!(
            unsafe { core::ptr::addr_of_mut!(mi_arena_static_top) },
            core::ptr::addr_of_mut!(top),
            oldtop
        );
        return core::ptr::null_mut::<()>();
    }
    // success
    unsafe {
        *memid = _mi_memid_create(2u32);
    }
    let start: u64 = _mi_align_up(oldtop, alignment);
    let p: *mut u8 = unsafe { core::ptr::addr_of_mut!(mi_arena_static.0) as *mut u8 }
        .wrapping_offset(start as isize);
    _mi_memzero(p as *mut (), size);
    p as *mut ()
}

fn mi_arena_meta_zalloc(size: u64, memid: *mut mi_memid_s, stats: *mut mi_stats_s) -> *mut () {
    unsafe {
        *memid = _mi_memid_none();
    }
    // try static
    let p: *mut () = mi_arena_static_zalloc(size, 16777216u64, memid);
    if !p.is_null() {
        return p;
    }
    // or fall back to the OS
    _mi_os_alloc(size, memid, stats)
}

fn mi_arena_meta_free(p: *mut (), memid: mi_memid_s, size: u64, stats: *mut mi_stats_s) {
    if mi_memkind_is_os(memid.memkind) {
        _mi_os_free(p, size, memid, stats);
    }
}

fn mi_arena_block_start(arena: *mut mi_arena_s, bindex: u64) -> *mut () {
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(core::ptr::addr_of!((*arena).start) as *mut *mut u8)
            .load(core::sync::atomic::Ordering::SeqCst) as *mut u8
    }
    .wrapping_offset(mi_arena_block_size(mi_bitmap_index_bit(bindex)) as isize) as *mut ()
}

fn mi_arena_try_claim(arena: *mut mi_arena_s, blocks: u64, bitmap_idx: *mut u64) -> bool {
    let idx: u64 = 0; // mi_atomic_load_relaxed(&arena->search_idx);  // start from last search; ok to be relaxed as the exact start does not matter
    if _mi_bitmap_try_find_from_claim_across(
        unsafe { (*arena).blocks_inuse.as_mut_ptr() as *mut u64 },
        unsafe { (*arena).field_count },
        idx,
        blocks,
        bitmap_idx,
    ) {
        mi_atomic_store_relaxed!(
            unsafe { core::ptr::addr_of_mut!((*arena).search_idx) },
            mi_bitmap_index_field(unsafe { *bitmap_idx })
        ); // start search from found location next time around
        return true;
    }
    false
}

#[inline(never)]
fn mi_arena_try_alloc_at(
    arena: *mut mi_arena_s,
    arena_index: u64,
    needed_bcount: u64,
    commit: bool,
    memid: *mut mi_memid_s,
    tld: *mut mi_os_tld_s,
) -> *mut () {
    MI_UNUSED!(arena_index);
    let mut bitmap_index: u64 = 0;
    if !mi_arena_try_claim(arena, needed_bcount, core::ptr::addr_of_mut!(bitmap_index)) {
        return core::ptr::null_mut::<()>();
    }
    // claimed it!
    let p: *mut () = mi_arena_block_start(arena, bitmap_index);
    unsafe {
        *memid = mi_memid_create_arena((*arena).id, (*arena).exclusive, bitmap_index);
    }
    unsafe {
        (*memid).is_pinned = (*arena).memid.is_pinned;
    }
    // none of the claimed blocks should be scheduled for a decommit
    if !unsafe { (*arena).blocks_purge }.is_null() {
        // this is thread safe as a potential purge only decommits parts that are not yet claimed as used (in `blocks_inuse`).
        _mi_bitmap_unclaim_across(
            unsafe { (*arena).blocks_purge },
            unsafe { (*arena).field_count },
            needed_bcount,
            bitmap_index,
        );
    }
    // set the dirty bits (todo: no need for an atomic op here?)
    if (unsafe { (*arena).memid.initially_zero }) && !unsafe { (*arena).blocks_dirty }.is_null() {
        unsafe {
            (*memid).initially_zero = _mi_bitmap_claim_across(
                (*arena).blocks_dirty,
                (*arena).field_count,
                needed_bcount,
                bitmap_index,
                core::ptr::null_mut::<bool>(),
            );
        }
    }
    // set commit state
    if unsafe { (*arena).blocks_committed }.is_null() {
        // always committed
        unsafe {
            (*memid).initially_committed = true;
        }
    } else {
        if commit {
            // commit requested, but the range may not be committed as a whole: ensure it is committed now
            unsafe {
                (*memid).initially_committed = true;
            }
            let mut any_uncommitted: bool = false;
            _mi_bitmap_claim_across(
                unsafe { (*arena).blocks_committed },
                unsafe { (*arena).field_count },
                needed_bcount,
                bitmap_index,
                core::ptr::addr_of_mut!(any_uncommitted),
            );
            if any_uncommitted {
                let mut commit_zero: bool = false;
                if !_mi_os_commit(
                    p,
                    mi_arena_block_size(needed_bcount),
                    core::ptr::addr_of_mut!(commit_zero),
                    unsafe { (*tld).stats },
                ) {
                    unsafe {
                        (*memid).initially_committed = false;
                    }
                } else {
                    if commit_zero {
                        unsafe {
                            (*memid).initially_zero = true;
                        }
                    }
                }
            }
        } else {
            // no need to commit, but check if already fully committed
            unsafe {
                (*memid).initially_committed = _mi_bitmap_is_claimed_across(
                    (*arena).blocks_committed,
                    (*arena).field_count,
                    needed_bcount,
                    bitmap_index,
                );
            }
        }
    }
    p
}

fn mi_arena_try_alloc_at_id(
    arena_id: i32,
    match_numa_node: bool,
    numa_node: i32,
    size: u64,
    alignment: u64,
    commit: bool,
    allow_large: bool,
    req_arena_id: i32,
    memid: *mut mi_memid_s,
    tld: *mut mi_os_tld_s,
) -> *mut () {
    MI_UNUSED_RELEASE!(alignment);
    let bcount: u64 = mi_block_count_of_size(size);
    let arena_index: u64 = mi_arena_id_index(arena_id);
    // Check arena suitability
    let arena: *mut mi_arena_s = mi_atomic_load_ptr_acquire!(
        unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
            .wrapping_offset(arena_index as isize)
    );
    if arena.is_null() {
        return core::ptr::null_mut::<()>();
    }
    if !allow_large && unsafe { (*arena).is_large } {
        return core::ptr::null_mut::<()>();
    }
    if !mi_arena_id_is_suitable(
        unsafe { (*arena).id },
        unsafe { (*arena).exclusive },
        req_arena_id,
    ) {
        return core::ptr::null_mut::<()>();
    }
    if req_arena_id == _mi_arena_id_none() {
        // in not specific, check numa affinity
        let numa_suitable: bool = numa_node < 0
            || (unsafe { (*arena).numa_node }) < 0
            || (unsafe { (*arena).numa_node }) == numa_node;
        if match_numa_node {
            if !numa_suitable {
                return core::ptr::null_mut::<()>();
            }
        } else {
            if numa_suitable {
                return core::ptr::null_mut::<()>();
            }
        }
    }
    // try to allocate
    let p: *mut () = mi_arena_try_alloc_at(arena, arena_index, bcount, commit, memid, tld);
    p
}

#[inline(never)]
fn mi_arena_try_alloc(
    numa_node: i32,
    size: u64,
    alignment: u64,
    commit: bool,
    allow_large: bool,
    req_arena_id: i32,
    memid: *mut mi_memid_s,
    tld: *mut mi_os_tld_s,
) -> *mut () {
    MI_UNUSED!(alignment);
    let max_arena: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    if max_arena == 0 {
        return core::ptr::null_mut::<()>();
    }
    if req_arena_id != _mi_arena_id_none() {
        // try a specific arena if requested
        if mi_arena_id_index(req_arena_id) < max_arena {
            let p: *mut () = mi_arena_try_alloc_at_id(
                req_arena_id,
                true,
                numa_node,
                size,
                alignment,
                commit,
                allow_large,
                req_arena_id,
                memid,
                tld,
            );
            if !p.is_null() {
                return p;
            }
        }
    } else {
        // try numa affine allocation
        for i in 0..max_arena {
            let p: *mut () = mi_arena_try_alloc_at_id(
                mi_arena_id_create(i),
                true,
                numa_node,
                size,
                alignment,
                commit,
                allow_large,
                req_arena_id,
                memid,
                tld,
            );
            if !p.is_null() {
                return p;
            }
        }
        // try from another numa node instead..
        if numa_node >= 0 {
            // if numa_node was < 0 (no specific affinity requested), all arena's have been tried already
            for i in 0..max_arena {
                let p: *mut () = mi_arena_try_alloc_at_id(
                    mi_arena_id_create(i),
                    false,
                    numa_node,
                    size,
                    alignment,
                    commit,
                    allow_large,
                    req_arena_id,
                    memid,
                    tld,
                ); // only proceed if not numa local
                if !p.is_null() {
                    return p;
                }
            }
        }
    }
    core::ptr::null_mut::<()>()
}

fn mi_arena_reserve(
    req_size: u64,
    allow_large: bool,
    req_arena_id: i32,
    arena_id: *mut i32,
) -> bool {
    if _mi_preloading() {
        return false;
    }
    // use OS only while pre loading
    if req_arena_id != _mi_arena_id_none() {
        return false;
    }
    let arena_count: u64 =
        mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    if arena_count > 108u64 {
        return false;
    }
    let mut arena_reserve: u64 = mi_option_get_size(23u32);
    if arena_reserve == 0 {
        return false;
    }
    if !_mi_os_has_virtual_reserve() {
        arena_reserve /= 4; // be conservative if virtual reserve is not supported (for some embedded systems for example)
    }
    arena_reserve = _mi_align_up(arena_reserve, 33554432u64);
    if arena_count >= 8 && arena_count <= 128 {
        arena_reserve = (1u64 << arena_count / 8).wrapping_mul(arena_reserve); // scale up the arena sizes exponentially
    }
    if arena_reserve < req_size {
        return false;
    }
    // should be able to at least handle the current allocation size
    // commit eagerly?
    let mut arena_commit: bool = false;
    if mi_option_get(4u32) == 2 {
        arena_commit = _mi_os_has_overcommit();
    } else {
        if mi_option_get(4u32) == 1 {
            arena_commit = true;
        }
    }
    return mi_reserve_os_memory_ex(arena_reserve, arena_commit, allow_large, false, arena_id) == 0; // exclusive
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_alloc_aligned(
    size: u64,
    alignment: u64,
    align_offset: u64,
    commit: bool,
    allow_large: bool,
    req_arena_id: i32,
    memid: *mut mi_memid_s,
    tld: *mut mi_os_tld_s,
) -> *mut () {
    unsafe {
        *memid = _mi_memid_none();
    }
    let numa_node: i32 = _mi_os_numa_node(tld); // current numa node
    // try to allocate in an arena if the alignment is small enough and the object is not too small (as for heap meta data)
    if size >= 16777216u64 && alignment <= 33554432u64 && align_offset == 0 {
        let mut p: *mut () = mi_arena_try_alloc(
            numa_node,
            size,
            alignment,
            commit,
            allow_large,
            req_arena_id,
            memid,
            tld,
        );
        if !p.is_null() {
            return p;
        }
        // otherwise, try to first eagerly reserve a new arena
        if req_arena_id == _mi_arena_id_none() {
            let mut arena_id: i32 = 0;
            if mi_arena_reserve(
                size,
                allow_large,
                req_arena_id,
                core::ptr::addr_of_mut!(arena_id),
            ) {
                // and try allocate in there
                p = mi_arena_try_alloc_at_id(
                    arena_id,
                    true,
                    numa_node,
                    size,
                    alignment,
                    commit,
                    allow_large,
                    req_arena_id,
                    memid,
                    tld,
                );
                if !p.is_null() {
                    return p;
                }
            }
        }
    }
    // if we cannot use OS allocation, return NULL
    let __v0: bool = mi_option_is_enabled(17u32) || req_arena_id != _mi_arena_id_none();
    if __v0 {
        unsafe {
            *slate_support::errno_location() = slate_support::ENOMEM as i32;
        }
        return core::ptr::null_mut::<()>();
    }
    // finally, fall back to the OS
    if align_offset > 0 {
        return _mi_os_alloc_aligned_at_offset(
            size,
            alignment,
            align_offset,
            commit,
            allow_large,
            memid,
            unsafe { (*tld).stats },
        );
    } else {
        return _mi_os_alloc_aligned(size, alignment, commit, allow_large, memid, unsafe {
            (*tld).stats
        });
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_alloc(
    size: u64,
    commit: bool,
    allow_large: bool,
    req_arena_id: i32,
    memid: *mut mi_memid_s,
    tld: *mut mi_os_tld_s,
) -> *mut () {
    _mi_arena_alloc_aligned(
        size,
        33554432u64,
        0u64,
        commit,
        allow_large,
        req_arena_id,
        memid,
        tld,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_arena_area(arena_id: i32, size: *mut u64) -> *mut () {
    if !size.is_null() {
        unsafe {
            *size = 0;
        }
    }
    let arena_index: u64 = mi_arena_id_index(arena_id);
    if arena_index >= 112 {
        return core::ptr::null_mut::<()>();
    }
    let arena: *mut mi_arena_s = mi_atomic_load_ptr_acquire!(
        unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
            .wrapping_offset(arena_index as isize)
    );
    if arena.is_null() {
        return core::ptr::null_mut::<()>();
    }
    if !size.is_null() {
        unsafe {
            *size = mi_arena_block_size((*arena).block_count);
        }
    }
    (unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(core::ptr::addr_of!((*arena).start) as *mut *mut u8)
            .load(core::sync::atomic::Ordering::SeqCst) as *mut u8
    }) as *mut ()
}

fn mi_arena_purge_delay() -> i64 {
    // <0 = no purging allowed, 0=immediate purging, >0=milli-second delay
    mi_option_get(15u32).wrapping_mul(mi_option_get(24u32))
}

fn mi_arena_purge(arena: *mut mi_arena_s, bitmap_idx: u64, blocks: u64, stats: *mut mi_stats_s) {
    let size: u64 = mi_arena_block_size(blocks);
    let p: *mut () = mi_arena_block_start(arena, bitmap_idx);
    let mut needs_recommit: bool = false;
    if _mi_bitmap_is_claimed_across(
        unsafe { (*arena).blocks_committed },
        unsafe { (*arena).field_count },
        blocks,
        bitmap_idx,
    ) {
        // all blocks are committed, we can purge freely
        needs_recommit = _mi_os_purge(p, size, stats);
    } else {
        // some blocks are not committed -- this can happen when a partially committed block is freed
        //
        // in `_mi_arena_free` and it is conservatively marked as uncommitted but still scheduled for a purge
        //
        // we need to ensure we do not try to reset (as that may be invalid for uncommitted memory),
        //
        // and also undo the decommit stats (as it was already adjusted)
        needs_recommit = _mi_os_purge_ex(p, size, false, stats); // allow reset?
        _mi_stat_increase(unsafe { core::ptr::addr_of_mut!((*stats).committed) }, size);
    }
    // clear the purged blocks
    _mi_bitmap_unclaim_across(
        unsafe { (*arena).blocks_purge },
        unsafe { (*arena).field_count },
        blocks,
        bitmap_idx,
    );
    // update committed bitmap
    if needs_recommit {
        _mi_bitmap_unclaim_across(
            unsafe { (*arena).blocks_committed },
            unsafe { (*arena).field_count },
            blocks,
            bitmap_idx,
        );
    }
}

fn mi_arena_schedule_purge(
    arena: *mut mi_arena_s,
    bitmap_idx: u64,
    blocks: u64,
    stats: *mut mi_stats_s,
) {
    let delay: i64 = mi_arena_purge_delay();
    if delay < 0 {
        return;
    }
    // is purging allowed at all?
    if _mi_preloading() || delay == 0 {
        // decommit directly
        mi_arena_purge(arena, bitmap_idx, blocks, stats);
    } else {
        // schedule decommit
        let expire: i64 =
            mi_atomic_loadi64_relaxed!(unsafe { core::ptr::addr_of_mut!((*arena).purge_expire) });
        if expire != 0 {
            mi_atomic_addi64_acq_rel!(
                unsafe { core::ptr::addr_of_mut!((*arena).purge_expire) },
                delay / 10
            ); // add smallish extra delay
        } else {
            mi_atomic_storei64_release!(
                unsafe { core::ptr::addr_of_mut!((*arena).purge_expire) },
                _mi_clock_now().wrapping_add(delay)
            );
        }
        _mi_bitmap_claim_across(
            unsafe { (*arena).blocks_purge },
            unsafe { (*arena).field_count },
            blocks,
            bitmap_idx,
            core::ptr::null_mut::<bool>(),
        );
    }
}

fn mi_arena_purge_range(
    arena: *mut mi_arena_s,
    idx: u64,
    startidx: u64,
    bitlen: u64,
    purge: u64,
    stats: *mut mi_stats_s,
) -> bool {
    let endidx: u64 = startidx.wrapping_add(bitlen);
    let mut bitidx: u64 = startidx;
    let mut all_purged: bool = false;
    while bitidx < endidx {
        // count consecutive ones in the purge mask
        let mut count: u64 = 0;
        while bitidx.wrapping_add(count) < endidx && purge & 1u64 << bitidx.wrapping_add(count) != 0
        {
            count = count.wrapping_add(1);
        }
        if count > 0 {
            // found range to be purged
            let range_idx: u64 = mi_bitmap_index_create(idx, bitidx);
            mi_arena_purge(arena, range_idx, count, stats);
            if count == bitlen {
                all_purged = true;
            }
        }
        bitidx = bitidx.wrapping_add(count.wrapping_add(1)); // +1 to skip the zero bit (or end)
    }
    all_purged
}

fn mi_arena_try_purge(
    arena: *mut mi_arena_s,
    now: i64,
    force: bool,
    stats: *mut mi_stats_s,
) -> bool {
    if (unsafe { (*arena).memid.is_pinned }) || unsafe { (*arena).blocks_purge }.is_null() {
        return false;
    }
    let mut expire: i64 =
        mi_atomic_loadi64_relaxed!(unsafe { core::ptr::addr_of_mut!((*arena).purge_expire) });
    if expire == 0 {
        return false;
    }
    if !force && expire > now {
        return false;
    }
    // reset expire (if not already set concurrently)
    mi_atomic_casi64_strong_acq_rel!(
        unsafe { core::ptr::addr_of_mut!((*arena).purge_expire) },
        core::ptr::addr_of_mut!(expire),
        0i32
    );
    // potential purges scheduled, walk through the bitmap
    let mut any_purged: bool = false;
    let mut full_purge: bool = true;
    let mut i: u64 = 0;
    while i < unsafe { (*arena).field_count } {
        let mut purge: u64 =
            mi_atomic_load_relaxed!(unsafe { (*arena).blocks_purge }.wrapping_offset(i as isize));
        if purge != 0 {
            let mut bitidx: u64 = 0;
            while bitidx < 64u64 {
                // find consecutive range of ones in the purge mask
                let mut bitlen: u64 = 0;
                while bitidx.wrapping_add(bitlen) < 64u64
                    && purge & 1u64 << bitidx.wrapping_add(bitlen) != 0
                {
                    bitlen = bitlen.wrapping_add(1);
                }
                // try to claim the longest range of corresponding in_use bits
                let bitmap_index: u64 = mi_bitmap_index_create(i, bitidx);
                while bitlen > 0 {
                    if _mi_bitmap_try_claim(
                        unsafe { (*arena).blocks_inuse.as_mut_ptr() as *mut u64 },
                        unsafe { (*arena).field_count },
                        bitlen,
                        bitmap_index,
                    ) {
                        break;
                    }
                    bitlen = bitlen.wrapping_sub(1);
                }
                // actual claimed bits at `in_use`
                if bitlen > 0 {
                    // read purge again now that we have the in_use bits
                    purge = mi_atomic_load_acquire!(
                        unsafe { (*arena).blocks_purge }.wrapping_offset(i as isize)
                    );
                    if !mi_arena_purge_range(arena, i, bitidx, bitlen, purge, stats) {
                        full_purge = false;
                    }
                    any_purged = true;
                    // release the claimed `in_use` bits again
                    _mi_bitmap_unclaim(
                        unsafe { (*arena).blocks_inuse.as_mut_ptr() as *mut u64 },
                        unsafe { (*arena).field_count },
                        bitlen,
                        bitmap_index,
                    );
                }
                bitidx = bitidx.wrapping_add(bitlen.wrapping_add(1)); // +1 to skip the zero (or end)
            }
            // while bitidx
        }
        // purge != 0
        i = i.wrapping_add(1);
    }
    // if not fully purged, make sure to purge again in the future
    if !full_purge {
        let delay: i64 = mi_arena_purge_delay();
        let mut expected: i64 = 0;
        mi_atomic_casi64_strong_acq_rel__2!(
            unsafe { core::ptr::addr_of_mut!((*arena).purge_expire) },
            core::ptr::addr_of_mut!(expected),
            _mi_clock_now().wrapping_add(delay)
        );
    }
    any_purged
}

fn mi_arenas_try_purge(force: bool, visit_all: bool, stats: *mut mi_stats_s) {
    let __v0: bool = _mi_preloading() || mi_arena_purge_delay() <= 0;
    if __v0 {
        return;
    }
    // nothing will be scheduled
    let max_arena: u64 =
        mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    if max_arena == 0 {
        return;
    }
    // allow only one thread to purge at a time
    let mut _mi_guard_expected: u64 = 0;
    let mut _mi_guard_once: bool = true;
    loop {
        let __v4: bool = _mi_guard_once
            && mi_atomic_cas_strong_acq_rel!(
                unsafe { core::ptr::addr_of_mut!(purge_guard) },
                core::ptr::addr_of_mut!(_mi_guard_expected),
                1u64
            );
        if !__v4 {
            break;
        }
        let now: i64 = _mi_clock_now();
        let mut max_purge_count: u64 = if visit_all { max_arena } else { 1 };
        for i in 0..max_arena {
            let arena: *mut mi_arena_s = mi_atomic_load_ptr_acquire!(
                unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
                    .wrapping_offset(i as isize)
            );
            if !arena.is_null() {
                if mi_arena_try_purge(arena, now, force, stats) {
                    if max_purge_count <= 1 {
                        break;
                    }
                    max_purge_count = max_purge_count.wrapping_sub(1);
                }
            }
        }
        unsafe {
            core::sync::atomic::AtomicU64::from_ptr(core::ptr::addr_of_mut!(purge_guard) as *mut u64).store(0u64, core::sync::atomic::Ordering::Release)
        };
        _mi_guard_once = false;
    }
}

static mut purge_guard: u64 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_free(
    p: *mut (),
    size: u64,
    committed_size: u64,
    memid: mi_memid_s,
    stats: *mut mi_stats_s,
) {
    if p.is_null() {
        return;
    }
    if size == 0 {
        return;
    }
    let all_committed: bool = committed_size == size;
    if mi_memkind_is_os(memid.memkind) {
        // was a direct OS allocation, pass through
        if !all_committed && committed_size > 0 {
            // if partially committed, adjust the committed stats (as `_mi_os_free` will increase decommit by the full size)
            _mi_stat_decrease(
                unsafe { core::ptr::addr_of_mut!((*stats).committed) },
                committed_size,
            );
        }
        _mi_os_free(p, size, memid, stats);
    } else {
        if (memid.memkind as u32) == 6 {
            // allocated in an arena
            let mut arena_idx: u64 = 0;
            let mut bitmap_idx: u64 = 0;
            mi_arena_memid_indices(
                memid,
                unsafe { &mut (*core::ptr::addr_of_mut!(arena_idx)) },
                unsafe { &mut (*core::ptr::addr_of_mut!(bitmap_idx)) },
            );
            let arena: *mut mi_arena_s = mi_atomic_load_ptr_acquire!(
                unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
                    .wrapping_offset(arena_idx as isize)
            );
            let blocks: u64 = mi_block_count_of_size(size);
            // checks
            if arena.is_null() {
                unsafe {
                    _mi_error_message(
                        slate_support::EINVAL as i32,
                        c"trying to free from non-existent arena: %p, size %zu, memid: 0x%zx\n"
                            .as_ptr(),
                        p,
                        size,
                        memid,
                    )
                };
                return;
            }
            if (unsafe { (*arena).field_count }) <= mi_bitmap_index_field(bitmap_idx) {
                unsafe {
                    _mi_error_message(slate_support::EINVAL as i32, c"trying to free from non-existent arena block: %p, size %zu, memid: 0x%zx\n".as_ptr(), p, size, memid)
                };
                return;
            }
            // need to set all memory to undefined as some parts may still be marked as no_access (like padding etc.)
            // potentially decommit
            if !((unsafe { (*arena).memid.is_pinned })
                || unsafe { (*arena).blocks_committed }.is_null())
            {
                if !all_committed {
                    // mark the entire range as no longer committed (so we recommit the full range when re-using)
                    _mi_bitmap_unclaim_across(
                        unsafe { (*arena).blocks_committed },
                        unsafe { (*arena).field_count },
                        blocks,
                        bitmap_idx,
                    );
                    if committed_size > 0 {
                        // if partially committed, adjust the committed stats (is it will be recommitted when re-using)
                        //
                        // in the delayed purge, we now need to not count a decommit if the range is not marked as committed.
                        _mi_stat_decrease(
                            unsafe { core::ptr::addr_of_mut!((*stats).committed) },
                            committed_size,
                        );
                    }
                    // note: if not all committed, it may be that the purge will reset/decommit the entire range
                    //
                    // that contains already decommitted parts. Since purge consistently uses reset or decommit that
                    //
                    // works (as we should never reset decommitted parts).
                }
                // (delay) purge the entire range
                mi_arena_schedule_purge(arena, bitmap_idx, blocks, stats);
            }
            // and make it available to others again
            let all_inuse: bool = _mi_bitmap_unclaim_across(
                unsafe { (*arena).blocks_inuse.as_mut_ptr() as *mut u64 },
                unsafe { (*arena).field_count },
                blocks,
                bitmap_idx,
            );
            if !all_inuse {
                unsafe {
                    _mi_error_message(
                        slate_support::EAGAIN as i32,
                        c"trying to free an already freed arena block: %p, size %zu\n".as_ptr(),
                        p,
                        size,
                    )
                };
                return;
            }
        }
        // arena was none, external, or static; nothing to do
    }
    // purge expired decommits
    mi_arenas_try_purge(false, false, stats);
}

fn mi_arenas_unsafe_destroy() {
    let max_arena: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    let mut new_max_arena: u64 = 0;
    for i in 0..max_arena {
        let arena: *mut mi_arena_s = mi_atomic_load_ptr_acquire!(
            unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
                .wrapping_offset(i as isize)
        );
        if !arena.is_null() {
            let __v0: bool = !unsafe {
                core::sync::atomic::AtomicPtr::from_ptr(
                    core::ptr::addr_of!((*arena).start) as *mut *mut u8
                )
                .load(core::sync::atomic::Ordering::SeqCst) as *mut u8
            }
            .is_null()
                && mi_memkind_is_os(unsafe { (*arena).memid.memkind });
            if __v0 {
                mi_atomic_store_ptr_release__5!(
                    unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
                        .wrapping_offset(i as isize)
                );
                _mi_os_free(
                    (unsafe {
                        core::sync::atomic::AtomicPtr::from_ptr(
                            core::ptr::addr_of!((*arena).start) as *mut *mut u8
                        )
                        .load(core::sync::atomic::Ordering::SeqCst)
                            as *mut u8
                    }) as *mut (),
                    mi_arena_size(arena),
                    unsafe { (*arena).memid },
                    unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
                );
            } else {
                new_max_arena = i;
            }
            mi_arena_meta_free(
                arena as *mut (),
                unsafe { (*arena).meta_memid },
                unsafe { (*arena).meta_size },
                unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
            );
        }
    }
    // try to lower the max arena.
    let mut expected: u64 = max_arena;
    mi_atomic_cas_strong_acq_rel__2!(
        unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) },
        core::ptr::addr_of_mut!(expected),
        new_max_arena
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_collect(force_purge: bool, stats: *mut mi_stats_s) {
    mi_arenas_try_purge(force_purge, true, stats); // visit all
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_unsafe_destroy_all(stats: *mut mi_stats_s) {
    mi_arenas_unsafe_destroy();
    _mi_arena_collect(true, stats); // force purge
    // purge non-owned arenas
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_arena_contains(p: *const ()) -> bool {
    let max_arena: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    for i in 0..max_arena {
        let arena: *mut mi_arena_s = mi_atomic_load_ptr_acquire!(
            unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
                .wrapping_offset(i as isize)
        );
        let __v0: bool = !arena.is_null()
            && (unsafe {
                core::sync::atomic::AtomicPtr::from_ptr(
                    core::ptr::addr_of!((*arena).start) as *mut *mut u8
                )
                .load(core::sync::atomic::Ordering::SeqCst) as *mut u8
            }) <= (p as *mut u8)
            && unsafe {
                core::sync::atomic::AtomicPtr::from_ptr(
                    core::ptr::addr_of!((*arena).start) as *mut *mut u8
                )
                .load(core::sync::atomic::Ordering::SeqCst) as *mut u8
            }
            .wrapping_offset(mi_arena_block_size(unsafe { (*arena).block_count }) as isize)
                > (p as *mut u8);
        if __v0 {
            return true;
        }
    }
    false
}

fn mi_arena_add(arena: *mut mi_arena_s, arena_id: *mut i32) -> bool {
    if !arena_id.is_null() {
        unsafe {
            *arena_id = 1i32.wrapping_neg();
        }
    }
    let i: u64 = mi_atomic_increment_acq_rel!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    if i >= 112 {
        mi_atomic_decrement_acq_rel!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
        return false;
    }
    unsafe {
        (*arena).id = mi_arena_id_create(i);
    }
    mi_atomic_store_ptr_release__4!(
        unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
            .wrapping_offset(i as isize),
        arena
    );
    if !arena_id.is_null() {
        unsafe {
            *arena_id = (*arena).id;
        }
    }
    true
}

fn mi_manage_os_memory_ex2(
    start: *mut (),
    size: u64,
    is_large: bool,
    numa_node: i32,
    exclusive: bool,
    memid: mi_memid_s,
    arena_id: *mut i32,
) -> bool {
    if !arena_id.is_null() {
        unsafe {
            *arena_id = _mi_arena_id_none();
        }
    }
    if size < 33554432u64 {
        return false;
    }
    let bcount: u64 = size / 33554432u64;
    let fields: u64 = _mi_divide_up(bcount, 64u64);
    let bitmaps: u64 = (if memid.is_pinned { 2i32 } else { 4i32 }) as u64;
    let asize: u64 = 144u64.wrapping_add(bitmaps.wrapping_mul(fields).wrapping_mul(8));
    let mut meta_memid: mi_memid_s = unsafe { core::mem::zeroed() };
    let arena: *mut mi_arena_s =
        mi_arena_meta_zalloc(asize, core::ptr::addr_of_mut!(meta_memid), unsafe {
            core::ptr::addr_of_mut!(_mi_stats_main.0)
        }) as *mut mi_arena_s; // TODO: can we avoid allocating from the OS?
    if arena.is_null() {
        return false;
    }
    // already zero'd due to os_alloc
    //
    // _mi_memzero(arena, asize);
    unsafe {
        (*arena).id = _mi_arena_id_none();
    }
    unsafe {
        (*arena).memid = memid;
    }
    unsafe {
        (*arena).exclusive = exclusive;
    }
    unsafe {
        (*arena).meta_size = asize;
    }
    unsafe {
        (*arena).meta_memid = meta_memid;
    }
    unsafe {
        (*arena).block_count = bcount;
    }
    unsafe {
        (*arena).field_count = fields;
    }
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(
            core::ptr::addr_of_mut!((*arena).start) as *mut *mut u8
        )
        .store(start as *mut u8, core::sync::atomic::Ordering::SeqCst)
    };
    unsafe {
        (*arena).numa_node = numa_node;
    }
    // TODO: or get the current numa node if -1? (now it allows anyone to allocate on -1)
    unsafe {
        (*arena).is_large = is_large;
    }
    unsafe {
        core::sync::atomic::AtomicI64::from_ptr(
            core::ptr::addr_of_mut!((*arena).purge_expire) as *mut i64
        )
        .store(0i64, core::sync::atomic::Ordering::SeqCst)
    };
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(
            core::ptr::addr_of_mut!((*arena).search_idx) as *mut u64
        )
        .store(0u64, core::sync::atomic::Ordering::SeqCst)
    };
    unsafe {
        (*arena).blocks_dirty =
            ((*arena).blocks_inuse.as_mut_ptr() as *mut u64).wrapping_offset(fields as isize);
    }
    // just after inuse bitmap
    unsafe {
        (*arena).blocks_committed = if (*arena).memid.is_pinned {
            core::ptr::null_mut::<u64>()
        } else {
            ((*arena).blocks_inuse.as_mut_ptr() as *mut u64)
                .wrapping_offset(2u64.wrapping_mul(fields) as isize)
        };
    }
    // just after dirty bitmap
    unsafe {
        (*arena).blocks_purge = if (*arena).memid.is_pinned {
            core::ptr::null_mut::<u64>()
        } else {
            ((*arena).blocks_inuse.as_mut_ptr() as *mut u64)
                .wrapping_offset(3u64.wrapping_mul(fields) as isize)
        };
    }
    // just after committed bitmap
    // initialize committed bitmap?
    if !unsafe { (*arena).blocks_committed }.is_null()
        && unsafe { (*arena).memid.initially_committed }
    {
        unsafe {
            core::ptr::write_bytes(
                (((*arena).blocks_committed as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (255i32 as i32) as u8,
                (fields.wrapping_mul(8) as usize) as usize,
            )
        }; // cast to void* to avoid atomic warning
    }
    // and claim leftover blocks if needed (so we never allocate there)
    let post: i64 = fields.wrapping_mul(64u64).wrapping_sub(bcount) as i64;
    if post > 0 {
        // don't use leftover bits at the end
        let postidx: u64 =
            mi_bitmap_index_create(fields.wrapping_sub(1), 64i64.wrapping_sub(post) as u64);
        _mi_bitmap_claim(
            unsafe { (*arena).blocks_inuse.as_mut_ptr() as *mut u64 },
            fields,
            post as u64,
            postidx,
            core::ptr::null_mut::<bool>(),
        );
    }
    mi_arena_add(arena, arena_id)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_manage_os_memory_ex(
    start: *mut (),
    size: u64,
    is_committed: bool,
    is_large: bool,
    is_zero: bool,
    numa_node: i32,
    exclusive: bool,
    arena_id: *mut i32,
) -> bool {
    let mut memid: mi_memid_s = _mi_memid_create(1u32);
    memid.initially_committed = is_committed;
    memid.initially_zero = is_zero;
    memid.is_pinned = is_large;
    mi_manage_os_memory_ex2(start, size, is_large, numa_node, exclusive, memid, arena_id)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reserve_os_memory_ex(
    mut size: u64,
    commit: bool,
    allow_large: bool,
    exclusive: bool,
    arena_id: *mut i32,
) -> i32 {
    if !arena_id.is_null() {
        unsafe {
            *arena_id = _mi_arena_id_none();
        }
    }
    size = _mi_align_up(size, 33554432u64); // at least one block
    let mut memid: mi_memid_s = unsafe { core::mem::zeroed() };
    let start: *mut () = _mi_os_alloc_aligned(
        size,
        33554432u64,
        commit,
        allow_large,
        core::ptr::addr_of_mut!(memid),
        unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
    );
    if start.is_null() {
        return slate_support::ENOMEM as i32;
    }
    let is_large: bool = memid.is_pinned; // todo: use separate is_large field?
    if !mi_manage_os_memory_ex2(
        start,
        size,
        is_large,
        1i32.wrapping_neg(),
        exclusive,
        memid,
        arena_id,
    ) {
        // numa node
        _mi_os_free_ex(start, size, commit, memid, unsafe {
            core::ptr::addr_of_mut!(_mi_stats_main.0)
        });
        unsafe {
            _mi_verbose_message(
                c"failed to reserve %zu k memory\n".as_ptr(),
                _mi_divide_up(size, 1024u64),
            )
        };
        return slate_support::ENOMEM as i32;
    }
    unsafe {
        _mi_verbose_message(
            c"reserved %zu KiB memory%s\n".as_ptr(),
            _mi_divide_up(size, 1024u64),
            if is_large {
                c" (in large os pages)".as_ptr() as *mut i8
            } else {
                c"".as_ptr() as *mut i8
            },
        )
    };
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_manage_os_memory(
    start: *mut (),
    size: u64,
    is_committed: bool,
    is_large: bool,
    is_zero: bool,
    numa_node: i32,
) -> bool {
    return mi_manage_os_memory_ex(
        start,
        size,
        is_committed,
        is_large,
        is_zero,
        numa_node,
        false,
        core::ptr::null_mut::<i32>(),
    ); // exclusive?
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reserve_os_memory(size: u64, commit: bool, allow_large: bool) -> i32 {
    mi_reserve_os_memory_ex(
        size,
        commit,
        allow_large,
        false,
        core::ptr::null_mut::<i32>(),
    )
}

fn mi_debug_show_bitmap(prefix: *const i8, fields: *mut u64, field_count: u64) -> u64 {
    let mut inuse_count: u64 = 0;
    for i in 0..field_count {
        let mut buf: __SlateAlign16<[i8; 65]> = __SlateAlign16([0i8; 65]);
        let field: u64 = mi_atomic_load_relaxed!(fields.wrapping_offset(i as isize));
        for bit in 0..64u64 {
            let inuse: bool = 1u64 << bit & field != 0;
            if inuse {
                inuse_count = inuse_count.wrapping_add(1);
            }
            buf.0[63u64.wrapping_sub(bit) as usize] = (if inuse { 120i32 } else { 46i32 }) as i8;
        }
        buf.0[64i32 as usize] = 0;
        unsafe { _mi_verbose_message(c"%s%s\n".as_ptr(), prefix, buf.0.as_mut_ptr() as *mut i8) };
    }
    inuse_count
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_debug_show_arenas() {
    let max_arenas: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(mi_arena_count.0) });
    for i in 0..max_arenas {
        let arena: *mut mi_arena_s = mi_atomic_load_ptr_relaxed__5!(
            unsafe { core::ptr::addr_of_mut!(mi_arenas.0) as *mut *mut mi_arena_s }
                .wrapping_offset(i as isize)
        );
        if arena.is_null() {
            break;
        }
        let mut inuse_count: u64 = 0;
        unsafe {
            _mi_verbose_message(
                c"arena %zu: %zu blocks with %zu fields\n".as_ptr(),
                i,
                (*arena).block_count,
                (*arena).field_count,
            )
        };
        let __v1: u64 = mi_debug_show_bitmap(
            c"  ".as_ptr(),
            unsafe { (*arena).blocks_inuse.as_mut_ptr() as *mut u64 },
            unsafe { (*arena).field_count },
        );
        inuse_count = inuse_count.wrapping_add(__v1);
        unsafe { _mi_verbose_message(c"  blocks in use ('x'): %zu\n".as_ptr(), inuse_count) };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reserve_huge_os_pages_at_ex(
    pages: u64,
    mut numa_node: i32,
    timeout_msecs: u64,
    exclusive: bool,
    arena_id: *mut i32,
) -> i32 {
    if !arena_id.is_null() {
        unsafe {
            *arena_id = 1i32.wrapping_neg();
        }
    }
    if pages == 0 {
        return 0;
    }
    if numa_node < 1i32.wrapping_neg() {
        numa_node = 1i32.wrapping_neg();
    }
    if numa_node >= 0 {
        numa_node = ((numa_node as u64) % _mi_os_numa_node_count()) as i32;
    }
    let mut hsize: u64 = 0;
    let mut pages_reserved: u64 = 0;
    let mut memid: mi_memid_s = unsafe { core::mem::zeroed() };
    let p: *mut () = _mi_os_alloc_huge_os_pages(
        pages,
        numa_node,
        timeout_msecs as i64,
        core::ptr::addr_of_mut!(pages_reserved),
        core::ptr::addr_of_mut!(hsize),
        core::ptr::addr_of_mut!(memid),
    );
    if p.is_null() || pages_reserved == 0 {
        unsafe { _mi_warning_message(c"failed to reserve %zu GiB huge pages\n".as_ptr(), pages) };
        return slate_support::ENOMEM as i32;
    }
    unsafe {
        _mi_verbose_message(
            c"numa node %i: reserved %zu GiB huge pages (of the %zu GiB requested)\n".as_ptr(),
            numa_node,
            pages_reserved,
            pages,
        )
    };
    if !mi_manage_os_memory_ex2(p, hsize, true, numa_node, exclusive, memid, arena_id) {
        _mi_os_free(p, hsize, memid, unsafe {
            core::ptr::addr_of_mut!(_mi_stats_main.0)
        });
        return slate_support::ENOMEM as i32;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reserve_huge_os_pages_at(
    pages: u64,
    numa_node: i32,
    timeout_msecs: u64,
) -> i32 {
    mi_reserve_huge_os_pages_at_ex(
        pages,
        numa_node,
        timeout_msecs,
        false,
        core::ptr::null_mut::<i32>(),
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reserve_huge_os_pages_interleave(
    mut pages: u64,
    numa_nodes: u64,
    timeout_msecs: u64,
) -> i32 {
    if pages == 0 {
        return 0;
    }
    // pages per numa node
    let mut numa_count: u64 = 0;
    let __v0: u64 = if numa_nodes > 0 {
        numa_nodes
    } else {
        _mi_os_numa_node_count()
    };
    numa_count = __v0;
    if numa_count <= 0 {
        numa_count = 1;
    }
    let pages_per: u64 = pages / numa_count;
    let pages_mod: u64 = pages % numa_count;
    let timeout_per: u64 = if timeout_msecs == 0 {
        0
    } else {
        (timeout_msecs / numa_count).wrapping_add(50)
    };
    // reserve evenly among numa nodes
    let mut numa_node: u64 = 0;
    while numa_node < numa_count && pages > 0 {
        let mut node_pages: u64 = pages_per; // can be 0
        if numa_node < pages_mod {
            node_pages = node_pages.wrapping_add(1);
        }
        let err: i32 = mi_reserve_huge_os_pages_at(node_pages, numa_node as i32, timeout_per);
        if err != 0 {
            return err;
        }
        if pages < node_pages {
            pages = 0;
        } else {
            pages = pages.wrapping_sub(node_pages);
        }
        numa_node = numa_node.wrapping_add(1);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_reserve_huge_os_pages(
    pages: u64,
    max_secs: f64,
    pages_reserved: *mut u64,
) -> i32 {
    MI_UNUSED!(max_secs);
    unsafe {
        _mi_warning_message(c"mi_reserve_huge_os_pages is deprecated: use mi_reserve_huge_os_pages_interleave/at instead\n".as_ptr())
    };
    if !pages_reserved.is_null() {
        unsafe {
            *pages_reserved = 0;
        }
    }
    let err: i32 = mi_reserve_huge_os_pages_interleave(pages, 0u64, (max_secs * 1000.0f64) as u64);
    if err == 0 && !pages_reserved.is_null() {
        unsafe {
            *pages_reserved = pages;
        }
    }
    err
}

fn mi_bitmap_mask_(count: u64, bitidx: u64) -> u64 {
    if count >= 64u64 {
        return !0;
    }
    if count == 0 {
        return 0;
    }
    (1u64 << count).wrapping_sub(1) << bitidx
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_try_find_claim_field(
    bitmap: *mut u64,
    idx: u64,
    count: u64,
    bitmap_idx: *mut u64,
) -> bool {
    let field: *mut u64 = bitmap.wrapping_offset(idx as isize);
    let mut map: u64 = mi_atomic_load_relaxed!(field);
    if map == !0 {
        return false;
    }
    // short cut
    // search for 0-bit sequence of length count
    let mask: u64 = mi_bitmap_mask_(count, 0u64);
    let bitidx_max: u64 = 64u64.wrapping_sub(count);
    let mut bitidx: u64 = mi_ctz(!map); // quickly find the first zero bit if possible
    let mut m: u64 = mask << bitidx; // invariant: m == mask shifted by bitidx
    // scan linearly for a free range of zero bits
    while bitidx <= bitidx_max {
        let mapm: u64 = map & m;
        if mapm == 0 {
            // are the mask bits free at bitidx?
            // no overflow?
            let newmap: u64 = map | m;
            if !mi_atomic_cas_strong_acq_rel!(field, core::ptr::addr_of_mut!(map), newmap) {
                // TODO: use weak cas here?
                // no success, another thread claimed concurrently.. keep going (with updated `map`)
            } else {
                // success, we claimed the bits!
                unsafe {
                    *bitmap_idx = mi_bitmap_index_create(idx, bitidx);
                }
                return true;
            }
        } else {
            // on to the next bit range
            let mut shift: u64 = 0;
            let __v2: u64 = if count == 1 {
                1
            } else {
                64u64.wrapping_sub(mi_clz(mapm)).wrapping_sub(bitidx)
            };
            shift = __v2;
            bitidx = bitidx.wrapping_add(shift);
            m <<= shift;
        }
    }
    // no bits found
    false
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_try_find_from_claim(
    bitmap: *mut u64,
    bitmap_fields: u64,
    mut idx: u64,
    count: u64,
    bitmap_idx: *mut u64,
) -> bool {
    let mut visited: u64 = 0;
    while visited < bitmap_fields {
        if idx >= bitmap_fields {
            idx = 0;
        }
        // wrap
        if _mi_bitmap_try_find_claim_field(bitmap, idx, count, bitmap_idx) {
            return true;
        }
        visited = visited.wrapping_add(1);
        idx = idx.wrapping_add(1);
    }
    false
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_try_find_from_claim_pred(
    bitmap: *mut u64,
    bitmap_fields: u64,
    mut idx: u64,
    count: u64,
    pred_fun: Option<unsafe extern "C-unwind" fn(u64, *mut ()) -> bool>,
    pred_arg: *mut (),
    bitmap_idx: *mut u64,
) -> bool {
    let mut visited: u64 = 0;
    while visited < bitmap_fields {
        if idx >= bitmap_fields {
            idx = 0;
        }
        // wrap
        if _mi_bitmap_try_find_claim_field(bitmap, idx, count, bitmap_idx) {
            let __v0: bool =
                pred_fun == None || unsafe { pred_fun.unwrap()(*bitmap_idx, pred_arg) };
            if __v0 {
                return true;
            }
            // predicate returned false, unclaim and look further
            _mi_bitmap_unclaim(bitmap, bitmap_fields, count, unsafe { *bitmap_idx });
        }
        visited = visited.wrapping_add(1);
        idx = idx.wrapping_add(1);
    }
    false
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_unclaim(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let bitidx: u64 = mi_bitmap_index_bit_in_field(bitmap_idx);
    let mask: u64 = mi_bitmap_mask_(count, bitidx);
    MI_UNUSED!(bitmap_fields);
    // mi_assert_internal((bitmap[idx] & mask) == mask);
    let prev: u64 = mi_atomic_and_acq_rel!(bitmap.wrapping_offset(idx as isize), !mask);
    prev & mask == mask
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_claim(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
    any_zero: *mut bool,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let bitidx: u64 = mi_bitmap_index_bit_in_field(bitmap_idx);
    let mask: u64 = mi_bitmap_mask_(count, bitidx);
    MI_UNUSED!(bitmap_fields);
    // mi_assert_internal(any_zero != NULL || (bitmap[idx] & mask) == 0);
    let prev: u64 = mi_atomic_or_acq_rel!(bitmap.wrapping_offset(idx as isize), mask);
    if !any_zero.is_null() {
        unsafe {
            *any_zero = prev & mask != mask;
        }
    }
    prev & mask == 0
}

fn mi_bitmap_is_claimedx(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
    any_ones: *mut bool,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let bitidx: u64 = mi_bitmap_index_bit_in_field(bitmap_idx);
    let mask: u64 = mi_bitmap_mask_(count, bitidx);
    MI_UNUSED!(bitmap_fields);
    let field: u64 = mi_atomic_load_relaxed!(bitmap.wrapping_offset(idx as isize));
    if !any_ones.is_null() {
        unsafe {
            *any_ones = field & mask != 0;
        }
    }
    field & mask == mask
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_try_claim(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let bitidx: u64 = mi_bitmap_index_bit_in_field(bitmap_idx);
    let mask: u64 = mi_bitmap_mask_(count, bitidx);
    MI_UNUSED!(bitmap_fields);
    let mut expected: u64 = mi_atomic_load_relaxed!(bitmap.wrapping_offset(idx as isize));
    loop {
        if expected & mask != 0 {
            return false;
        }
        if !(!mi_atomic_cas_strong_acq_rel!(
            bitmap.wrapping_offset(idx as isize),
            core::ptr::addr_of_mut!(expected),
            expected | mask
        )) {
            break;
        }
    }
    true
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_is_claimed(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    mi_bitmap_is_claimedx(
        bitmap,
        bitmap_fields,
        count,
        bitmap_idx,
        core::ptr::null_mut::<bool>(),
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_is_any_claimed(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    let mut any_ones: bool = false;
    mi_bitmap_is_claimedx(
        bitmap,
        bitmap_fields,
        count,
        bitmap_idx,
        core::ptr::addr_of_mut!(any_ones),
    );
    any_ones
}

fn mi_bitmap_try_find_claim_field_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    idx: u64,
    count: u64,
    retries: u64,
    bitmap_idx: *mut u64,
) -> bool {
    // initial field
    let mut newmap: u64 = unsafe { core::mem::zeroed() };
    let mut initial_mask: u64 = unsafe { core::mem::zeroed() };
    let mut initial_idx: u64 = unsafe { core::mem::zeroed() };
    let mut initial_field: *mut u64 = unsafe { core::mem::zeroed() };
    let mut final_mask: u64 = unsafe { core::mem::zeroed() };
    // we found a range of contiguous zeros up to the final field; mask contains mask in the final field
    //
    // now try to claim the range atomically
    let mut final_field: *mut u64 = unsafe { core::mem::zeroed() };
    let mut mask_bits: u64 = unsafe { core::mem::zeroed() };
    // mask bits for the final field
    let mut mask: u64 = unsafe { core::mem::zeroed() };
    // scan ahead
    let mut found: u64 = unsafe { core::mem::zeroed() };
    // count of initial zeros starting at idx
    let mut initial: u64 = unsafe { core::mem::zeroed() };
    let mut map: u64 = unsafe { core::mem::zeroed() };
    // check initial trailing zeros
    let mut field: *mut u64 = unsafe { core::mem::zeroed() };
    unsafe {
        field = bitmap.wrapping_offset(idx as isize);
        map = mi_atomic_load_relaxed!(field);
        initial = mi_clz(map);
        if initial == 0 {
            return false;
        } else {
            if initial >= count {
                return _mi_bitmap_try_find_claim_field(bitmap, idx, count, bitmap_idx);
            } else {
                // no need to cross fields (this case won't happen for us)
                if _mi_divide_up(count.wrapping_sub(initial), 64u64)
                    >= bitmap_fields.wrapping_sub(idx)
                {
                    return false;
                } else {
                    // not enough entries
                    found = initial;
                    mask = 0;
                    while found < count {
                        let __v14: *mut u64 = field;
                        field = __v14.wrapping_offset(1 as isize);
                        map = mi_atomic_load_relaxed!(field);
                        mask_bits = if found.wrapping_add(64u64) <= count {
                            64u64
                        } else {
                            count.wrapping_sub(found)
                        };
                        mask = mi_bitmap_mask_(mask_bits, 0u64);
                        if map & mask != 0 {
                            return false;
                        } else {
                            // some part is already claimed
                            found = found.wrapping_add(mask_bits);
                        }
                    }
                    final_field = field;
                    final_mask = mask;
                    initial_field = bitmap.wrapping_offset(idx as isize);
                    initial_idx = 64u64.wrapping_sub(initial);
                    initial_mask = mi_bitmap_mask_(initial, initial_idx);
                    field = initial_field;
                    map = mi_atomic_load_relaxed!(field);
                    '__join_6: {
                        loop {
                            newmap = map | initial_mask;
                            if map & initial_mask != 0 {
                                break '__join_6;
                            } else {
                                if mi_atomic_cas_strong_acq_rel!(
                                    field,
                                    core::ptr::addr_of_mut!(map),
                                    newmap
                                ) {
                                    break;
                                }
                            }
                        }
                        // intermediate fields
                        loop {
                            let __v3: *mut u64 = field;
                            let __v2: *mut u64 = __v3.wrapping_offset(1 as isize);
                            field = __v2;
                            if __v2 < final_field {
                                newmap = !0;
                                map = 0;
                                if !mi_atomic_cas_strong_acq_rel!(
                                    field,
                                    core::ptr::addr_of_mut!(map),
                                    newmap
                                ) {
                                    break '__join_6;
                                }
                            } else {
                                break;
                            }
                        }
                        // final field
                        map = mi_atomic_load_relaxed!(field);
                        loop {
                            newmap = map | final_mask;
                            if map & final_mask != 0 {
                                break '__join_6;
                            } else {
                                if mi_atomic_cas_strong_acq_rel!(
                                    field,
                                    core::ptr::addr_of_mut!(map),
                                    newmap
                                ) {
                                    break;
                                }
                            }
                        }
                        // claimed!
                        *bitmap_idx = mi_bitmap_index_create(idx, initial_idx);
                        return true;
                    }
                    loop {
                        let __v1: *mut u64 = field;
                        let __v0: *mut u64 = __v1.wrapping_offset(-(1 as isize));
                        field = __v0;
                        if __v0 > initial_field {
                            newmap = 0;
                            map = !0;
                            mi_atomic_store_release!(field, newmap);
                        } else {
                            break;
                        }
                    }
                    // roll back intermediate fields
                    //
                    // (we just failed to claim `field` so decrement first)
                    if field == initial_field {
                        // (if we failed on the initial field, `field + 1 == initial_field`)
                        map = mi_atomic_load_relaxed!(field);
                        loop {
                            newmap = map & !initial_mask;
                            if mi_atomic_cas_strong_acq_rel!(
                                field,
                                core::ptr::addr_of_mut!(map),
                                newmap
                            ) {
                                break;
                            }
                        }
                    }
                    // retry? (we make a recursive call instead of goto to be able to use const declarations)
                    if retries <= 2 {
                        return mi_bitmap_try_find_claim_field_across(
                            bitmap,
                            bitmap_fields,
                            idx,
                            count,
                            retries.wrapping_add(1),
                            bitmap_idx,
                        );
                    } else {
                        return false;
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_try_find_from_claim_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    start_field_idx: u64,
    count: u64,
    bitmap_idx: *mut u64,
) -> bool {
    if count <= 2 {
        // we don't bother with crossover fields for small counts
        return _mi_bitmap_try_find_from_claim(
            bitmap,
            bitmap_fields,
            start_field_idx,
            count,
            bitmap_idx,
        );
    }
    // visit the fields
    let mut idx: u64 = start_field_idx;
    let mut visited: u64 = 0;
    while visited < bitmap_fields {
        if idx >= bitmap_fields {
            idx = 0;
        }
        // wrap
        // first try to claim inside a field
        if count <= 64u64 {
            if _mi_bitmap_try_find_claim_field(bitmap, idx, count, bitmap_idx) {
                return true;
            }
        }
        // if that fails, then try to claim across fields
        if mi_bitmap_try_find_claim_field_across(
            bitmap,
            bitmap_fields,
            idx,
            count,
            0u64,
            bitmap_idx,
        ) {
            return true;
        }
        visited = visited.wrapping_add(1);
        idx = idx.wrapping_add(1);
    }
    false
}

fn mi_bitmap_mask_across(
    bitmap_idx: u64,
    bitmap_fields: u64,
    mut count: u64,
    pre_mask: &mut u64,
    mid_mask: &mut u64,
    post_mask: &mut u64,
) -> u64 {
    MI_UNUSED!(bitmap_fields);
    let bitidx: u64 = mi_bitmap_index_bit_in_field(bitmap_idx);
    if bitidx.wrapping_add(count) <= 64u64 {
        unsafe {
            *(pre_mask as *mut u64) = mi_bitmap_mask_(count, bitidx);
        }
        unsafe {
            *(mid_mask as *mut u64) = 0;
        }
        unsafe {
            *(post_mask as *mut u64) = 0;
        }
        return 0;
    } else {
        let pre_bits: u64 = 64u64.wrapping_sub(bitidx);
        unsafe {
            *(pre_mask as *mut u64) = mi_bitmap_mask_(pre_bits, bitidx);
        }
        count = count.wrapping_sub(pre_bits);
        let mid_count: u64 = count / 64u64;
        unsafe {
            *(mid_mask as *mut u64) = !0;
        }
        count %= 64u64;
        let __v4: u64 = if count == 0 {
            0
        } else {
            mi_bitmap_mask_(count, 0u64)
        };
        unsafe {
            *(post_mask as *mut u64) = __v4;
        }
        return mid_count;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_unclaim_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let mut pre_mask: u64 = 0;
    let mut mid_mask: u64 = 0;
    let mut post_mask: u64 = 0;
    let mut mid_count: u64 = mi_bitmap_mask_across(
        bitmap_idx,
        bitmap_fields,
        count,
        unsafe { &mut (*core::ptr::addr_of_mut!(pre_mask)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(mid_mask)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(post_mask)) },
    );
    let mut all_one: bool = true;
    let mut field: *mut u64 = bitmap.wrapping_offset(idx as isize);
    let mut prev: u64 = 0;
    let __v0: *mut u64 = field;
    field = __v0.wrapping_offset(1 as isize);
    prev = unsafe {
        let __t1 = !pre_mask;
        let __t0 = core::sync::atomic::AtomicU64::from_ptr(__v0 as *mut u64)
            .fetch_and(__t1, core::sync::atomic::Ordering::AcqRel);
        __t0
    };
    // clear first part
    if prev & pre_mask != pre_mask {
        all_one = false;
    }
    '__slate_break_8: loop {
        let __v6: u64 = mid_count;
        mid_count = __v6.wrapping_sub(1);
        if __v6 <= 0 {
            break;
        }
        let __v3: *mut u64 = field;
        field = __v3.wrapping_offset(1 as isize);
        prev = unsafe {
            let __t3 = !mid_mask;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(__v3 as *mut u64)
                .fetch_and(__t3, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
        // clear mid part
        if prev & mid_mask != mid_mask {
            all_one = false;
        }
    }
    if post_mask != 0 {
        prev = mi_atomic_and_acq_rel!(field, !post_mask); // clear end part
        if prev & post_mask != post_mask {
            all_one = false;
        }
    }
    all_one
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_claim_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
    pany_zero: *mut bool,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let mut pre_mask: u64 = 0;
    let mut mid_mask: u64 = 0;
    let mut post_mask: u64 = 0;
    let mut mid_count: u64 = mi_bitmap_mask_across(
        bitmap_idx,
        bitmap_fields,
        count,
        unsafe { &mut (*core::ptr::addr_of_mut!(pre_mask)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(mid_mask)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(post_mask)) },
    );
    let mut all_zero: bool = true;
    let mut any_zero: bool = false;
    let mut field: *mut u64 = bitmap.wrapping_offset(idx as isize);
    let mut prev: u64 = 0;
    let __v0: *mut u64 = field;
    field = __v0.wrapping_offset(1 as isize);
    prev = unsafe {
        let __t1 = pre_mask;
        let __t0 = core::sync::atomic::AtomicU64::from_ptr(__v0 as *mut u64)
            .fetch_or(__t1, core::sync::atomic::Ordering::AcqRel);
        __t0
    };
    if prev & pre_mask != 0 {
        all_zero = false;
    }
    if prev & pre_mask != pre_mask {
        any_zero = true;
    }
    '__slate_break_8: loop {
        let __v6: u64 = mid_count;
        mid_count = __v6.wrapping_sub(1);
        if __v6 <= 0 {
            break;
        }
        let __v3: *mut u64 = field;
        field = __v3.wrapping_offset(1 as isize);
        prev = unsafe {
            let __t3 = mid_mask;
            let __t2 = core::sync::atomic::AtomicU64::from_ptr(__v3 as *mut u64)
                .fetch_or(__t3, core::sync::atomic::Ordering::AcqRel);
            __t2
        };
        if prev & mid_mask != 0 {
            all_zero = false;
        }
        if prev & mid_mask != mid_mask {
            any_zero = true;
        }
    }
    if post_mask != 0 {
        prev = mi_atomic_or_acq_rel!(field, post_mask);
        if prev & post_mask != 0 {
            all_zero = false;
        }
        if prev & post_mask != post_mask {
            any_zero = true;
        }
    }
    if !pany_zero.is_null() {
        unsafe {
            *pany_zero = any_zero;
        }
    }
    all_zero
}

fn mi_bitmap_is_claimedx_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
    pany_ones: *mut bool,
) -> bool {
    let idx: u64 = mi_bitmap_index_field(bitmap_idx);
    let mut pre_mask: u64 = 0;
    let mut mid_mask: u64 = 0;
    let mut post_mask: u64 = 0;
    let mut mid_count: u64 = mi_bitmap_mask_across(
        bitmap_idx,
        bitmap_fields,
        count,
        unsafe { &mut (*core::ptr::addr_of_mut!(pre_mask)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(mid_mask)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(post_mask)) },
    );
    let mut all_ones: bool = true;
    let mut any_ones: bool = false;
    let mut field: *mut u64 = bitmap.wrapping_offset(idx as isize);
    let mut prev: u64 = 0;
    let __v0: *mut u64 = field;
    field = __v0.wrapping_offset(1 as isize);
    prev = unsafe {
        core::sync::atomic::AtomicU64::from_ptr(__v0 as *mut u64)
            .load(core::sync::atomic::Ordering::Relaxed)
    };
    if prev & pre_mask != pre_mask {
        all_ones = false;
    }
    if prev & pre_mask != 0 {
        any_ones = true;
    }
    loop {
        let __v4: u64 = mid_count;
        mid_count = __v4.wrapping_sub(1);
        if __v4 <= 0 {
            break;
        }
        let __v2: *mut u64 = field;
        field = __v2.wrapping_offset(1 as isize);
        prev = unsafe {
            core::sync::atomic::AtomicU64::from_ptr(__v2 as *mut u64)
                .load(core::sync::atomic::Ordering::Relaxed)
        };
        if prev & mid_mask != mid_mask {
            all_ones = false;
        }
        if prev & mid_mask != 0 {
            any_ones = true;
        }
    }
    if post_mask != 0 {
        prev = mi_atomic_load_relaxed!(field);
        if prev & post_mask != post_mask {
            all_ones = false;
        }
        if prev & post_mask != 0 {
            any_ones = true;
        }
    }
    if !pany_ones.is_null() {
        unsafe {
            *pany_ones = any_ones;
        }
    }
    all_ones
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_is_claimed_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    mi_bitmap_is_claimedx_across(
        bitmap,
        bitmap_fields,
        count,
        bitmap_idx,
        core::ptr::null_mut::<bool>(),
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bitmap_is_any_claimed_across(
    bitmap: *mut u64,
    bitmap_fields: u64,
    count: u64,
    bitmap_idx: u64,
) -> bool {
    let mut any_ones: bool = false;
    mi_bitmap_is_claimedx_across(
        bitmap,
        bitmap_fields,
        count,
        bitmap_idx,
        core::ptr::addr_of_mut!(any_ones),
    );
    any_ones
}

fn mi_heap_visit_pages(
    heap: *mut mi_heap_s,
    r#fn: Option<
        unsafe extern "C-unwind" fn(
            *mut mi_heap_s,
            *mut mi_page_queue_s,
            *mut mi_page_s,
            *mut (),
            *mut (),
        ) -> bool,
    >,
    arg1: *mut (),
    arg2: *mut (),
) -> bool {
    if heap.is_null() || (unsafe { (*heap).page_count }) == 0 {
        return true;
    }
    // visit all pages
    let mut i: u64 = 0;
    while i <= 74u64 {
        let pq: *mut mi_page_queue_s =
            unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
                .wrapping_offset(i as isize);
        let mut page: *mut mi_page_s = unsafe { (*pq).first };
        while !page.is_null() {
            let next: *mut mi_page_s = unsafe { (*page).next }; // save next in case the page gets removed from the queue
            if !unsafe { r#fn.unwrap()(heap, pq, page, arg1, arg2) } {
                return false;
            }
            page = next; // and continue
        }
        i = i.wrapping_add(1);
    }
    true
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_heap_page_collect")]
extern "C-unwind" fn mi_heap_page_collect(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    page: *mut mi_page_s,
    arg_collect: *mut (),
    arg2: *mut (),
) -> bool {
    MI_UNUSED!(arg2);
    MI_UNUSED!(heap);
    let collect: u32 = unsafe { *(arg_collect as *mut u32) };
    _mi_page_free_collect(page, (collect as u32) >= 1);
    if mi_page_all_free(page as *const mi_page_s) {
        // no more used blocks, free the page.
        //
        // note: this will free retired pages as well.
        let freed: bool = _PyMem_mi_page_maybe_free(page, pq, (collect as u32) >= 1);
        if !freed && (collect as u32) == 2 {
            // _PyMem_mi_page_maybe_free may have moved the page to a different
            //
            // page queue, so we need to re-fetch the correct queue.
            let mut bin: u8 = 0;
            let __v0: u32 = if mi_page_is_in_full(page as *const mi_page_s) {
                74u32
            } else {
                (_mi_bin((unsafe { (*page).xblock_size }) as u64) as i32) as u32
            };
            bin = __v0 as u8;
            _mi_page_abandon(
                page,
                unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
                    .wrapping_offset((bin as i32) as isize),
            );
        }
    } else {
        if (collect as u32) == 2 {
            // still used blocks but the thread is done; abandon the page
            _mi_page_abandon(page, pq);
        }
    }
    return true; // don't break
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_heap_page_never_delayed_free")]
extern "C-unwind" fn mi_heap_page_never_delayed_free(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    page: *mut mi_page_s,
    arg1: *mut (),
    arg2: *mut (),
) -> bool {
    MI_UNUSED!(arg1);
    MI_UNUSED!(arg2);
    MI_UNUSED!(heap);
    MI_UNUSED!(pq);
    _mi_page_use_delayed_free(page, 3u32, false);
    return true; // don't break
}

fn mi_heap_collect_ex(heap: *mut mi_heap_s, mut collect: u32) {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return;
    }
    let force: bool = (collect as u32) >= 1;
    _mi_deferred_free(heap, force);
    // gh-112532: we may be called from a thread that is not the owner of the heap
    let mut is_main_thread: bool = false;
    is_main_thread = _mi_is_main_thread() && (unsafe { (*heap).thread_id }) == _mi_thread_id();
    // note: never reclaim on collect but leave it to threads that need storage to reclaim
    let mut force_main: bool = false;
    let __v2: bool =
        (collect as u32) == 1 && is_main_thread && mi_heap_is_backing(heap as *const mi_heap_s);
    force_main = __v2 && !unsafe { (*heap).no_reclaim };
    if force_main {
        // the main thread is abandoned (end-of-program), try to reclaim all abandoned segments.
        //
        // if all memory is freed by now, all segments should be freed.
        _mi_abandoned_reclaim_all(heap, unsafe {
            core::ptr::addr_of_mut!((*(*heap).tld).segments)
        });
    }
    // if abandoning, mark all pages to no longer add to delayed_free
    if (collect as u32) == 2 {
        mi_heap_visit_pages(
            heap,
            Some(mi_heap_page_never_delayed_free),
            core::ptr::null_mut::<()>(),
            core::ptr::null_mut::<()>(),
        );
    }
    // free all current thread delayed blocks.
    //
    // (if abandoning, after this there are no more thread-delayed references into the pages.)
    _mi_heap_delayed_free_all(heap);
    // collect retired pages
    _mi_heap_collect_retired(heap, force);
    // free pages that were delayed with QSBR
    _PyMem_mi_heap_collect_qsbr(heap);
    // collect all pages owned by this thread
    mi_heap_visit_pages(
        heap,
        Some(mi_heap_page_collect),
        core::ptr::addr_of_mut!(collect) as *mut (),
        core::ptr::null_mut::<()>(),
    );
    // collect abandoned segments (in particular, purge expired parts of segments in the abandoned segment list)
    //
    // note: forced purge can be quite expensive if many threads are created/destroyed so we do not force on abandonment
    _mi_abandoned_collect(heap, (collect as u32) == 1, unsafe {
        core::ptr::addr_of_mut!((*(*heap).tld).segments)
    }); // force?
    // collect segment local caches
    if force {
        _mi_segment_thread_collect(unsafe { core::ptr::addr_of_mut!((*(*heap).tld).segments) });
    }
    // collect regions on program-exit (or shared library unload)
    let __v3: bool = force && is_main_thread && mi_heap_is_backing(heap as *const mi_heap_s);
    if __v3 {
        _mi_thread_data_collect(); // collect thread data cache
        _mi_arena_collect(true, unsafe {
            core::ptr::addr_of_mut!((*(*heap).tld).stats)
        }); // force purge
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_collect_abandon(heap: *mut mi_heap_s) {
    mi_heap_collect_ex(heap, 2u32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_collect(heap: *mut mi_heap_s, force: bool) {
    mi_heap_collect_ex(heap, (if force { 1i32 } else { 0i32 }) as u32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_collect(force: bool) {
    mi_heap_collect(mi_prim_get_default_heap(), force);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_get_default() -> *mut mi_heap_s {
    mi_thread_init();
    mi_prim_get_default_heap()
}

fn mi_heap_is_default(heap: *const mi_heap_s) -> bool {
    heap == (mi_prim_get_default_heap() as *const mi_heap_s)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_get_backing() -> *mut mi_heap_s {
    let heap: *mut mi_heap_s = mi_heap_get_default();
    let bheap: *mut mi_heap_s = unsafe { (*(*heap).tld).heap_backing };
    bheap
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_init_ex(
    heap: *mut mi_heap_s,
    tld: *mut mi_tld_s,
    arena_id: i32,
    no_reclaim: bool,
    tag: u8,
) {
    _mi_memcpy_aligned(
        heap as *mut (),
        (unsafe { core::ptr::addr_of!(_mi_heap_empty.0) }) as *const (),
        3064u64,
    );
    unsafe {
        (*heap).tld = tld;
    }
    unsafe {
        (*heap).thread_id = _mi_thread_id();
    }
    unsafe {
        (*heap).arena_id = arena_id;
    }
    if heap == unsafe { (*tld).heap_backing } {
        _mi_random_init(unsafe { core::ptr::addr_of_mut!((*heap).random) });
    } else {
        _mi_random_split(
            unsafe { core::ptr::addr_of_mut!((*(*tld).heap_backing).random) },
            unsafe { core::ptr::addr_of_mut!((*heap).random) },
        );
    }
    unsafe {
        (*heap).cookie = _mi_heap_random_next(heap) | 1;
    }
    unsafe {
        *((*heap).keys.as_mut_ptr() as *mut u64).wrapping_offset(0 as isize) =
            _mi_heap_random_next(heap);
    }
    unsafe {
        *((*heap).keys.as_mut_ptr() as *mut u64).wrapping_offset(1 as isize) =
            _mi_heap_random_next(heap);
    }
    unsafe {
        (*heap).no_reclaim = no_reclaim;
    }
    unsafe {
        (*heap).tag = tag;
    }
    // push on the thread local heaps list
    unsafe {
        (*heap).next = (*(*heap).tld).heaps;
    }
    unsafe {
        (*(*heap).tld).heaps = heap;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_new_in_arena(arena_id: i32) -> *mut mi_heap_s {
    let bheap: *mut mi_heap_s = mi_heap_get_backing();
    let heap: *mut mi_heap_s = mi_heap_malloc_tp!(bheap, mi_heap_s); // todo: OS allocate in secure mode?
    if heap.is_null() {
        return core::ptr::null_mut::<mi_heap_s>();
    }
    // don't reclaim abandoned pages or otherwise destroy is unsafe
    _mi_heap_init_ex(heap, unsafe { (*bheap).tld }, arena_id, true, 0u8);
    heap
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_new() -> *mut mi_heap_s {
    mi_heap_new_in_arena(_mi_arena_id_none())
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_memid_is_suitable(heap: *mut mi_heap_s, memid: mi_memid_s) -> bool {
    _mi_arena_memid_is_suitable(memid, unsafe { (*heap).arena_id })
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_random_next(heap: *mut mi_heap_s) -> u64 {
    _mi_random_next(unsafe { core::ptr::addr_of_mut!((*heap).random) })
}

fn mi_heap_reset_pages(heap: *mut mi_heap_s) {
    // TODO: copy full empty heap instead?
    unsafe {
        core::ptr::write_bytes(
            ((core::ptr::addr_of_mut!((*heap).pages_free_direct) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<[*mut mi_page_s; 129]>() as u64) as usize) as usize,
        )
    };
    _mi_memcpy_aligned(
        (unsafe { core::ptr::addr_of_mut!((*heap).pages) }) as *mut (),
        (unsafe { core::ptr::addr_of!(_mi_heap_empty.0.pages) }) as *const (),
        core::mem::size_of::<[mi_page_queue_s; 75]>() as u64,
    );
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(
            core::ptr::addr_of_mut!((*heap).thread_delayed_free) as *mut *mut mi_block_s,
        )
        .store(
            core::ptr::null_mut::<mi_block_s>() as *mut mi_block_s,
            core::sync::atomic::Ordering::SeqCst,
        )
    };
    unsafe {
        (*heap).page_count = 0;
    }
}

fn mi_heap_free(heap: *mut mi_heap_s) {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return;
    }
    if mi_heap_is_backing(heap as *const mi_heap_s) {
        return;
    }
    // dont free the backing heap
    // reset default
    if mi_heap_is_default(heap as *const mi_heap_s) {
        _mi_heap_set_default_direct(unsafe { (*(*heap).tld).heap_backing });
    }
    // remove ourselves from the thread local heaps list
    //
    // linear search but we expect the number of heaps to be relatively small
    let mut prev: *mut mi_heap_s = core::ptr::null_mut::<mi_heap_s>();
    let mut curr: *mut mi_heap_s = unsafe { (*(*heap).tld).heaps };
    while curr != heap && !curr.is_null() {
        prev = curr;
        curr = unsafe { (*curr).next };
    }
    if curr == heap {
        if !prev.is_null() {
            unsafe {
                (*prev).next = (*heap).next;
            }
        } else {
            unsafe {
                (*(*heap).tld).heaps = (*heap).next;
            }
        }
    }
    // and free the used memory
    mi_free(heap as *mut ());
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc._mi_heap_page_destroy")]
extern "C-unwind" fn _mi_heap_page_destroy(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    page: *mut mi_page_s,
    arg1: *mut (),
    arg2: *mut (),
) -> bool {
    MI_UNUSED!(arg1);
    MI_UNUSED!(arg2);
    MI_UNUSED!(heap);
    MI_UNUSED!(pq);
    // ensure no more thread_delayed_free will be added
    _mi_page_use_delayed_free(page, 3u32, false);
    // stats
    let bsize: u64 = mi_page_block_size(page as *const mi_page_s);
    if bsize > 131072u64 {
        if bsize <= 16777216u64 {
            mi_heap_stat_decrease!();
        } else {
            mi_heap_stat_decrease!();
        }
    }
    // pretend it is all free now
    unsafe {
        (*page).used = 0;
    }
    // and free the page
    //
    // mi_page_free(page,false);
    unsafe {
        (*page).next = core::ptr::null_mut::<mi_page_s>();
    }
    unsafe {
        (*page).prev = core::ptr::null_mut::<mi_page_s>();
    }
    _mi_segment_page_free(page, false, unsafe {
        core::ptr::addr_of_mut!((*(*heap).tld).segments)
    }); // no force?
    return true; // keep going
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_destroy_pages(heap: *mut mi_heap_s) {
    mi_heap_visit_pages(
        heap,
        Some(_mi_heap_page_destroy),
        core::ptr::null_mut::<()>(),
        core::ptr::null_mut::<()>(),
    );
    mi_heap_reset_pages(heap);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_destroy(heap: *mut mi_heap_s) {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return;
    }
    if !unsafe { (*heap).no_reclaim } {
        // don't free in case it may contain reclaimed pages
        mi_heap_delete(heap);
    } else {
        // track all blocks as freed
        // free all pages
        _mi_heap_destroy_pages(heap);
        mi_heap_free(heap);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_unsafe_destroy_all() {
    let bheap: *mut mi_heap_s = mi_heap_get_backing();
    let mut curr: *mut mi_heap_s = unsafe { (*(*bheap).tld).heaps };
    while !curr.is_null() {
        let next: *mut mi_heap_s = unsafe { (*curr).next };
        if unsafe { (*curr).no_reclaim } {
            mi_heap_destroy(curr);
        } else {
            _mi_heap_destroy_pages(curr);
        }
        curr = next;
    }
}

fn mi_heap_absorb(heap: *mut mi_heap_s, from: *mut mi_heap_s) {
    if from.is_null() || (unsafe { (*from).page_count }) == 0 {
        return;
    }
    // reduce the size of the delayed frees
    _mi_heap_delayed_free_partial(from);
    // transfer all pages by appending the queues; this will set a new heap field
    //
    // so threads may do delayed frees in either heap for a while.
    //
    // note: appending waits for each page to not be in the `MI_DELAYED_FREEING` state
    //
    // so after this only the new heap will get delayed frees
    let mut i: u64 = 0;
    while i <= 74u64 {
        let pq: *mut mi_page_queue_s =
            unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
                .wrapping_offset(i as isize);
        let append: *mut mi_page_queue_s =
            unsafe { (*from).pages.as_mut_ptr() as *mut mi_page_queue_s }
                .wrapping_offset(i as isize);
        let pcount: u64 = _mi_page_queue_append(heap, pq, append);
        let __v1: u64 = unsafe { (*heap).page_count };
        unsafe {
            (*heap).page_count = __v1.wrapping_add(pcount);
        }
        let __v4: u64 = unsafe { (*from).page_count };
        unsafe {
            (*from).page_count = __v4.wrapping_sub(pcount);
        }
        i = i.wrapping_add(1);
    }
    // and do outstanding delayed frees in the `from` heap
    //
    // note: be careful here as the `heap` field in all those pages no longer point to `from`,
    //
    // turns out to be ok as `_mi_heap_delayed_free` only visits the list and calls a
    //
    // the regular `_mi_free_delayed_block` which is safe.
    _mi_heap_delayed_free_all(from);
    // and reset the `from` heap
    mi_heap_reset_pages(from);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_delete(heap: *mut mi_heap_s) {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return;
    }
    if !mi_heap_is_backing(heap as *const mi_heap_s) {
        // transfer still used pages to the backing heap
        mi_heap_absorb(unsafe { (*(*heap).tld).heap_backing }, heap);
    } else {
        // the backing heap abandons its pages
        _mi_heap_collect_abandon(heap);
    }
    mi_heap_free(heap);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_set_default(heap: *mut mi_heap_s) -> *mut mi_heap_s {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return core::ptr::null_mut::<mi_heap_s>();
    }
    let old: *mut mi_heap_s = mi_prim_get_default_heap();
    _mi_heap_set_default_direct(heap);
    old
}

fn mi_heap_of_block(p: *const ()) -> *mut mi_heap_s {
    if p.is_null() {
        return core::ptr::null_mut::<mi_heap_s>();
    }
    let segment: *mut mi_segment_s = _mi_ptr_segment(p);
    let valid: bool = _mi_ptr_cookie(segment as *const ()) == unsafe { (*segment).cookie };
    if !valid {
        return core::ptr::null_mut::<mi_heap_s>();
    }
    mi_page_heap(_mi_segment_page_of(segment as *const mi_segment_s, p) as *const mi_page_s)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_contains_block(heap: *mut mi_heap_s, p: *const ()) -> bool {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return false;
    }
    heap == mi_heap_of_block(p)
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_heap_page_check_owned")]
extern "C-unwind" fn mi_heap_page_check_owned(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    page: *mut mi_page_s,
    p: *mut (),
    vfound: *mut (),
) -> bool {
    MI_UNUSED!(heap);
    MI_UNUSED!(pq);
    let segment: *mut mi_segment_s = _mi_page_segment(page as *const mi_page_s);
    let start: *mut () = _mi_page_start(
        segment as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::null_mut::<u64>(),
    ) as *mut ();
    let end: *mut () = (start as *mut u8).wrapping_offset(
        (((unsafe { (*page).capacity }) as i32) as u64)
            .wrapping_mul(mi_page_block_size(page as *const mi_page_s)) as isize,
    ) as *mut ();
    unsafe {
        *(vfound as *mut bool) = p >= start && p < end;
    }
    return !unsafe { *(vfound as *mut bool) }; // continue if not found
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_check_owned(heap: *mut mi_heap_s, p: *const ()) -> bool {
    let __v0: bool = heap.is_null() || !mi_heap_is_initialized(heap);
    if __v0 {
        return false;
    }
    if (p as u64) & 7u64 != 0 {
        return false;
    }
    // only aligned pointers
    let mut found: bool = false;
    mi_heap_visit_pages(
        heap,
        Some(mi_heap_page_check_owned),
        p as *mut (),
        core::ptr::addr_of_mut!(found) as *mut (),
    );
    found
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_check_owned(p: *const ()) -> bool {
    mi_heap_check_owned(mi_prim_get_default_heap(), p)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_heap_area_ex_s {
    area: mi_heap_area_s,
    page: *mut mi_page_s,
}

fn mi_fast_divisor(divisor: u64, magic: &mut u64, shift: &mut u64) {
    unsafe {
        *(shift as *mut u64) = 64u64.wrapping_sub(mi_clz(divisor.wrapping_sub(1)));
    }
    unsafe {
        *(magic as *mut u64) = (4294967296u64
            .wrapping_mul((1u64 << *(shift as *mut u64)).wrapping_sub(divisor))
            / divisor)
            .wrapping_add(1);
    }
}

fn mi_fast_divide(n: u64, magic: u64, shift: u64) -> u64 {
    (n.wrapping_mul(magic) >> 32i32).wrapping_add(n) >> shift
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_area_visit_blocks(
    area: *const mi_heap_area_s,
    page: *mut mi_page_s,
    visitor: Option<
        unsafe extern "C-unwind" fn(
            *const mi_heap_s,
            *const mi_heap_area_s,
            *mut (),
            u64,
            *mut (),
        ) -> bool,
    >,
    arg: *mut (),
) -> bool {
    if area.is_null() {
        return true;
    }
    if page.is_null() {
        return true;
    }
    if (unsafe { (*page).used }) == 0 {
        return true;
    }
    let bsize: u64 = mi_page_block_size(page as *const mi_page_s);
    let ubsize: u64 = mi_page_usable_block_size(page as *const mi_page_s); // without padding
    let mut psize: u64 = 0;
    let pstart: *mut u8 = _mi_page_start(
        _mi_page_segment(page as *const mi_page_s) as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::addr_of_mut!(psize),
    );
    let heap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
    if ((unsafe { (*page).capacity }) as i32) == 1 {
        // optimize page with one block
        return unsafe {
            visitor.unwrap()(
                heap as *const mi_heap_s,
                area,
                pstart as *mut (),
                ubsize,
                arg,
            )
        };
    }
    if (unsafe { (*page).used }) == (((unsafe { (*page).capacity }) as i32) as u32) {
        // optimize full pages
        let mut block: *mut u8 = pstart;
        let mut i: u64 = 0;
        while i < (((unsafe { (*page).capacity }) as i32) as u64) {
            if !unsafe {
                visitor.unwrap()(
                    heap as *const mi_heap_s,
                    area,
                    block as *mut (),
                    ubsize,
                    arg,
                )
            } {
                return false;
            }
            let __v0: *mut u8 = block;
            block = __v0.wrapping_offset(bsize as isize);
            i = i.wrapping_add(1);
        }
        return true;
    }
    // create a bitmap of free blocks.
    let mut free_map: __SlateAlign16<[u64; 128]> = __SlateAlign16([0u64; 128]);
    let bmapsize: u64 = (((unsafe { (*page).capacity }) as i32)
        .wrapping_add(64i32)
        .wrapping_sub(1)
        / 64i32) as u64;
    unsafe {
        core::ptr::write_bytes(
            ((free_map.0.as_mut_ptr() as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            (bmapsize.wrapping_mul(core::mem::size_of::<u64>() as u64) as usize) as usize,
        )
    };
    if ((unsafe { (*page).capacity }) as i32) % 64i32 != 0 {
        let shift: u64 = (((unsafe { (*page).capacity }) as i32) % 64i32) as u64;
        let mask: u64 = 18446744073709551615u64 << shift;
        free_map.0[bmapsize.wrapping_sub(1) as usize] = mask;
    }
    // fast repeated division by the block size
    let mut magic: u64 = 0;
    let mut shift: u64 = 0;
    mi_fast_divisor(
        bsize,
        unsafe { &mut (*core::ptr::addr_of_mut!(magic)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(shift)) },
    );
    {
        let mut block: *mut mi_block_s = unsafe { (*page).free };
        while !block.is_null() {
            let offset: u64 =
                ((unsafe { (block as *mut u8).offset_from(pstart as *mut u8) }) as i64) as u64;
            let blockidx: u64 = mi_fast_divide(offset, magic, shift);
            let bitidx: u64 = blockidx / 64u64;
            let bit: u64 = blockidx.wrapping_sub(bitidx.wrapping_mul(64u64));
            let __v5: *mut u64 =
                (free_map.0.as_mut_ptr() as *mut u64).wrapping_offset(bitidx as isize);
            unsafe {
                *__v5 = *__v5 | 1u64 << bit;
            }
            block = mi_block_next(page as *const mi_page_s, block as *const mi_block_s);
        }
    }
    // walk through all blocks skipping the free ones
    let mut block: *mut u8 = pstart;
    for i in 0..bmapsize {
        if free_map.0[i as usize] == 0 {
            // every block is in use
            for j in 0..64u64 {
                if !unsafe {
                    visitor.unwrap()(
                        heap as *const mi_heap_s,
                        area,
                        block as *mut (),
                        ubsize,
                        arg,
                    )
                } {
                    return false;
                }
                let __v9: *mut u8 = block;
                block = __v9.wrapping_offset(bsize as isize);
            }
        } else {
            let mut m: u64 = !free_map.0[i as usize];
            while m != 0 {
                let bitidx: u64 = mi_ctz(m);
                if !unsafe {
                    visitor.unwrap()(
                        heap as *const mi_heap_s,
                        area,
                        block.wrapping_offset(bitidx.wrapping_mul(bsize) as isize) as *mut (),
                        ubsize,
                        arg,
                    )
                } {
                    return false;
                }
                m &= m.wrapping_sub(1);
            }
            let __v15: *mut u8 = block;
            block = __v15.wrapping_offset(bsize.wrapping_mul(64u64) as isize);
        }
    }
    true
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_area_init(area: *mut mi_heap_area_s, page: *mut mi_page_s) {
    _mi_page_free_collect(page, true);
    let bsize: u64 = mi_page_block_size(page as *const mi_page_s);
    let ubsize: u64 = mi_page_usable_block_size(page as *const mi_page_s);
    unsafe {
        (*area).reserved = (((*page).reserved as i32) as u64).wrapping_mul(bsize);
    }
    unsafe {
        (*area).committed = (((*page).capacity as i32) as u64).wrapping_mul(bsize);
    }
    unsafe {
        (*area).blocks = _mi_page_start(
            _mi_page_segment(page as *const mi_page_s) as *const mi_segment_s,
            page as *const mi_page_s,
            core::ptr::null_mut::<u64>(),
        ) as *mut ();
    }
    unsafe {
        (*area).used = (*page).used as u64;
    }
    // number of blocks in use (#553)
    unsafe {
        (*area).block_size = ubsize;
    }
    unsafe {
        (*area).full_block_size = bsize;
    }
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_heap_visit_areas_page")]
extern "C-unwind" fn mi_heap_visit_areas_page(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    page: *mut mi_page_s,
    vfun: *mut (),
    arg: *mut (),
) -> bool {
    MI_UNUSED!(heap);
    MI_UNUSED!(pq);
    let fun: Option<
        unsafe extern "C-unwind" fn(*const mi_heap_s, *const mi_heap_area_ex_s, *mut ()) -> bool,
    > = unsafe {
        core::mem::transmute::<
            *mut (),
            Option<
                unsafe extern "C-unwind" fn(
                    *const mi_heap_s,
                    *const mi_heap_area_ex_s,
                    *mut (),
                ) -> bool,
            >,
        >(vfun)
    };
    let mut xarea: mi_heap_area_ex_s = unsafe { core::mem::zeroed() };
    xarea.page = page;
    _mi_heap_area_init(core::ptr::addr_of_mut!(xarea.area), page);
    unsafe {
        fun.unwrap()(
            heap as *const mi_heap_s,
            core::ptr::addr_of_mut!(xarea) as *const mi_heap_area_ex_s,
            arg,
        )
    }
}

fn mi_heap_visit_areas(
    heap: *const mi_heap_s,
    visitor: Option<
        unsafe extern "C-unwind" fn(*const mi_heap_s, *const mi_heap_area_ex_s, *mut ()) -> bool,
    >,
    arg: *mut (),
) -> bool {
    if visitor == None {
        return false;
    }
    return mi_heap_visit_pages(
        heap as *mut mi_heap_s,
        Some(mi_heap_visit_areas_page),
        unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *const mi_heap_s,
                        *const mi_heap_area_ex_s,
                        *mut (),
                    ) -> bool,
                >,
                *mut (),
            >(visitor)
        },
        arg,
    ); // note: function pointer to void* :-{
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_visit_blocks_args_s {
    visit_blocks: bool,
    visitor: Option<
        unsafe extern "C-unwind" fn(
            *const mi_heap_s,
            *const mi_heap_area_s,
            *mut (),
            u64,
            *mut (),
        ) -> bool,
    >,
    arg: *mut (),
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_heap_area_visitor")]
extern "C-unwind" fn mi_heap_area_visitor(
    heap: *const mi_heap_s,
    xarea: *const mi_heap_area_ex_s,
    arg: *mut (),
) -> bool {
    if !unsafe {
        unsafe { (*(arg as *mut mi_visit_blocks_args_s)).visitor }.unwrap()(
            heap,
            core::ptr::addr_of!((*xarea).area),
            core::ptr::null_mut::<()>(),
            (*xarea).area.block_size,
            (*(arg as *mut mi_visit_blocks_args_s)).arg,
        )
    } {
        return false;
    }
    if unsafe { (*(arg as *mut mi_visit_blocks_args_s)).visit_blocks } {
        return _mi_heap_area_visit_blocks(
            unsafe { core::ptr::addr_of!((*xarea).area) },
            unsafe { (*xarea).page },
            unsafe { (*(arg as *mut mi_visit_blocks_args_s)).visitor },
            unsafe { (*(arg as *mut mi_visit_blocks_args_s)).arg },
        );
    } else {
        return true;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_heap_visit_blocks(
    heap: *const mi_heap_s,
    visit_blocks: bool,
    visitor: Option<
        unsafe extern "C-unwind" fn(
            *const mi_heap_s,
            *const mi_heap_area_s,
            *mut (),
            u64,
            *mut (),
        ) -> bool,
    >,
    arg: *mut (),
) -> bool {
    let mut args: mi_visit_blocks_args_s = {
        let mut __t0: mi_visit_blocks_args_s = unsafe { core::mem::zeroed() };
        __t0.visit_blocks = visit_blocks;
        __t0.visitor = visitor;
        __t0.arg = arg;
        __t0
    };
    _mi_heap_delayed_free_partial(heap as *mut mi_heap_s);
    mi_heap_visit_areas(
        heap,
        Some(mi_heap_area_visitor),
        core::ptr::addr_of_mut!(args) as *mut (),
    )
}

// 8
// 16
// 24
// 32
// 40
// 48
// 56
// 64
// 72
// 655360, Huge queue
// Full queue
// Empty slice span queues for every bin
//  8
// 16
// 24
// 32
// 35
#[unsafe(no_mangle)]
static _mi_heap_empty: __SlateAlign64<mi_heap_s> = __SlateAlign64({
    let mut __t0: mi_heap_s = unsafe { core::mem::zeroed() };
    __t0.tld = core::ptr::null_mut::<mi_tld_s>();
    __t0.pages_free_direct = [
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
    ];
    __t0.pages = [
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 2u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 3u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 4u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 5u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 6u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 7u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 8u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 10u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 12u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 14u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 20u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 24u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 28u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 32u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 40u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 48u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 56u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 64u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 80u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 96u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 112u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 128u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 160u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 192u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 224u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 256u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 320u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 384u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 448u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 512u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 640u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 768u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 896u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1024u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1280u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1536u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1792u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 2048u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 2560u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 3072u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 3584u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 4096u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 5120u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 6144u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 7168u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 8192u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 10240u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 12288u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 14336u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16384u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 20480u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 24576u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 28672u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 32768u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 40960u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 49152u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 57344u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 65536u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 81920u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 98304u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 114688u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 131072u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 163840u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 196608u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 229376u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 262144u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 327680u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 393216u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 458752u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 524288u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16385u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16386u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
    ];
    __t0.thread_delayed_free = core::ptr::null_mut::<mi_block_s>();
    __t0.thread_id = 0u64;
    __t0.arena_id = 0i32;
    __t0.cookie = 0u64;
    __t0.keys = [0u64, 0u64];
    __t0.random = {
        let mut __t1: mi_random_cxt_s = unsafe { core::mem::zeroed() };
        __t1.input = {
            let mut __t2: [u32; 16] = unsafe { core::mem::zeroed() };
            __t2[0] = 0u32;
            __t2
        };
        __t1.output = {
            let mut __t3: [u32; 16] = unsafe { core::mem::zeroed() };
            __t3[0] = 0u32;
            __t3
        };
        __t1.output_available = 0i32;
        __t1.weak = 1i32 != 0i32;
        __t1
    };
    __t0.page_count = 0u64;
    __t0.page_retired_min = 74u64;
    __t0.page_retired_max = 0u64;
    __t0.next = core::ptr::null_mut::<mi_heap_s>();
    __t0.no_reclaim = 0i32 != 0i32;
    __t0.tag = 0u8;
    __t0.debug_offset = 0u8;
    __t0
});

static tld_empty: __SlateAlign64<mi_tld_s> = __SlateAlign64({
    let mut __t0: mi_tld_s = unsafe { core::mem::zeroed() };
    __t0.heartbeat = 0u64;
    __t0.recurse = 0i32 != 0i32;
    __t0.heap_backing = core::ptr::null_mut::<mi_heap_s>();
    __t0.heaps = core::ptr::null_mut::<mi_heap_s>();
    __t0.segments = mi_segments_tld_s {
        spans: [
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 1u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 1u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 2u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 3u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 4u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 5u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 6u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 7u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 10u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 12u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 14u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 16u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 20u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 24u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 28u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 32u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 40u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 48u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 56u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 64u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 80u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 96u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 112u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 128u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 160u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 192u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 224u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 256u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 320u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 384u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 448u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 512u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 640u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 768u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 896u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 1024u64,
            },
        ],
        count: 0u64,
        peak_count: 0u64,
        current_size: 0u64,
        peak_size: 0u64,
        stats: ((unsafe { core::ptr::addr_of!(tld_empty.0) }) as *mut u8)
            .wrapping_offset((core::mem::offset_of!(mi_tld_s, stats) as u64) as isize)
            as *mut mi_stats_s,
        os: ((unsafe { core::ptr::addr_of!(tld_empty.0) }) as *mut u8)
            .wrapping_offset((core::mem::offset_of!(mi_tld_s, os) as u64) as isize)
            as *mut mi_os_tld_s,
        abandoned: unsafe { core::ptr::addr_of_mut!(_mi_abandoned_default) },
    };
    __t0.os = mi_os_tld_s {
        region_idx: 0u64,
        stats: ((unsafe { core::ptr::addr_of!(tld_empty.0) }) as *mut u8)
            .wrapping_offset((core::mem::offset_of!(mi_tld_s, stats) as u64) as isize)
            as *mut mi_stats_s,
    };
    __t0.stats = mi_stats_s {
        segments: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        pages: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        reserved: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        committed: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        reset: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        purged: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        page_committed: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        segments_abandoned: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        pages_abandoned: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        threads: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        normal: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        huge: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        large: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        malloc: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        segments_cache: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        pages_extended: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        mmap_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        commit_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        reset_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        purge_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        page_no_retire: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        searches: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        normal_count: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        huge_count: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        large_count: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
    };
    __t0
});

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_thread_id() -> u64 {
    _mi_prim_thread_id()
}

#[thread_local]
#[unsafe(no_mangle)]
static mut _mi_heap_default: *mut mi_heap_s =
    (unsafe { core::ptr::addr_of!(_mi_heap_empty.0) }) as *mut mi_heap_s;

static mut tld_main: mi_tld_s = {
    let mut __t0: mi_tld_s = unsafe { core::mem::zeroed() };
    __t0.heartbeat = 0u64;
    __t0.recurse = 0i32 != 0i32;
    __t0.heap_backing = unsafe { core::ptr::addr_of_mut!(_mi_heap_main) };
    __t0.heaps = unsafe { core::ptr::addr_of_mut!(_mi_heap_main) };
    __t0.segments = mi_segments_tld_s {
        spans: [
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 1u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 1u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 2u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 3u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 4u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 5u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 6u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 7u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 10u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 12u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 14u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 16u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 20u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 24u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 28u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 32u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 40u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 48u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 56u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 64u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 80u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 96u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 112u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 128u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 160u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 192u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 224u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 256u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 320u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 384u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 448u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 512u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 640u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 768u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 896u64,
            },
            mi_span_queue_s {
                first: core::ptr::null_mut::<mi_page_s>(),
                last: core::ptr::null_mut::<mi_page_s>(),
                slice_count: 1024u64,
            },
        ],
        count: 0u64,
        peak_count: 0u64,
        current_size: 0u64,
        peak_size: 0u64,
        stats: unsafe { core::ptr::addr_of_mut!(tld_main.stats) },
        os: unsafe { core::ptr::addr_of_mut!(tld_main.os) },
        abandoned: unsafe { core::ptr::addr_of_mut!(_mi_abandoned_default) },
    };
    __t0.os = mi_os_tld_s {
        region_idx: 0u64,
        stats: unsafe { core::ptr::addr_of_mut!(tld_main.stats) },
    };
    __t0.stats = mi_stats_s {
        segments: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        pages: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        reserved: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        committed: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        reset: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        purged: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        page_committed: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        segments_abandoned: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        pages_abandoned: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        threads: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        normal: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        huge: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        large: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        malloc: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        segments_cache: mi_stat_count_s {
            allocated: 0i64,
            freed: 0i64,
            peak: 0i64,
            current: 0i64,
        },
        pages_extended: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        mmap_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        commit_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        reset_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        purge_calls: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        page_no_retire: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        searches: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        normal_count: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        huge_count: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
        large_count: mi_stat_counter_s {
            total: 0i64,
            count: 0i64,
        },
    };
    __t0
};

#[unsafe(no_mangle)]
static mut _mi_heap_main: mi_heap_s = {
    let mut __t0: mi_heap_s = unsafe { core::mem::zeroed() };
    __t0.tld = unsafe { core::ptr::addr_of_mut!(tld_main) };
    __t0.pages_free_direct = [
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
        (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s,
    ];
    __t0.pages = [
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 2u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 3u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 4u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 5u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 6u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 7u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 8u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 10u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 12u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 14u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 20u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 24u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 28u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 32u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 40u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 48u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 56u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 64u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 80u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 96u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 112u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 128u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 160u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 192u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 224u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 256u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 320u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 384u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 448u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 512u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 640u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 768u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 896u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1024u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1280u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1536u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 1792u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 2048u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 2560u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 3072u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 3584u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 4096u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 5120u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 6144u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 7168u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 8192u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 10240u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 12288u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 14336u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16384u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 20480u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 24576u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 28672u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 32768u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 40960u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 49152u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 57344u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 65536u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 81920u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 98304u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 114688u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 131072u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 163840u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 196608u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 229376u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 262144u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 327680u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 393216u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 458752u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 524288u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16385u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
        mi_page_queue_s {
            first: core::ptr::null_mut::<mi_page_s>(),
            last: core::ptr::null_mut::<mi_page_s>(),
            block_size: 16386u64.wrapping_mul(core::mem::size_of::<u64>() as u64),
        },
    ];
    __t0.thread_delayed_free = core::ptr::null_mut::<mi_block_s>();
    __t0.thread_id = 0u64;
    __t0.arena_id = 0i32;
    __t0.cookie = 0u64;
    __t0.keys = [0u64, 0u64];
    __t0.random = {
        let mut __t1: mi_random_cxt_s = unsafe { core::mem::zeroed() };
        __t1.input = {
            let mut __t2: [u32; 16] = unsafe { core::mem::zeroed() };
            __t2[0] = 2221713035u32;
            __t2
        };
        __t1.output = {
            let mut __t3: [u32; 16] = unsafe { core::mem::zeroed() };
            __t3[0] = 0u32;
            __t3
        };
        __t1.output_available = 0i32;
        __t1.weak = 1i32 != 0i32;
        __t1
    };
    __t0.page_count = 0u64;
    __t0.page_retired_min = 74u64;
    __t0.page_retired_max = 0u64;
    __t0.next = core::ptr::null_mut::<mi_heap_s>();
    __t0.no_reclaim = 0i32 != 0i32;
    __t0
};

#[unsafe(no_mangle)]
static mut _mi_process_is_initialized: bool = 0i32 != 0i32;

#[unsafe(no_mangle)]
static mut _mi_stats_main: __SlateAlign64<mi_stats_s> = __SlateAlign64(mi_stats_s {
    segments: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    pages: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    reserved: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    committed: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    reset: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    purged: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    page_committed: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    segments_abandoned: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    pages_abandoned: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    threads: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    normal: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    huge: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    large: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    malloc: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    segments_cache: mi_stat_count_s {
        allocated: 0i64,
        freed: 0i64,
        peak: 0i64,
        current: 0i64,
    },
    pages_extended: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    mmap_calls: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    commit_calls: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    reset_calls: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    purge_calls: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    page_no_retire: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    searches: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    normal_count: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    huge_count: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
    large_count: mi_stat_counter_s {
        total: 0i64,
        count: 0i64,
    },
});

fn mi_heap_main_init() {
    if (unsafe { _mi_heap_main.cookie }) == 0 {
        unsafe {
            _mi_heap_main.thread_id = _mi_thread_id();
        }
        unsafe {
            _mi_heap_main.cookie = 1;
        }
        _mi_random_init(unsafe { core::ptr::addr_of_mut!(_mi_heap_main.random) });
        unsafe {
            _mi_heap_main.cookie = _mi_heap_random_next(core::ptr::addr_of_mut!(_mi_heap_main));
        }
        unsafe {
            _mi_heap_main.keys[0] = _mi_heap_random_next(core::ptr::addr_of_mut!(_mi_heap_main));
        }
        unsafe {
            _mi_heap_main.keys[1] = _mi_heap_random_next(core::ptr::addr_of_mut!(_mi_heap_main));
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_main_get() -> *mut mi_heap_s {
    mi_heap_main_init();
    unsafe { core::ptr::addr_of_mut!(_mi_heap_main) }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_thread_data_s {
    heap: mi_heap_s,
    tld: mi_tld_s,
    memid: mi_memid_s,
}

static mut td_cache: __SlateAlign16<[*mut mi_thread_data_s; 16]> =
    __SlateAlign16(unsafe { core::mem::zeroed() });

fn mi_thread_data_zalloc() -> *mut mi_thread_data_s {
    // try to find thread metadata in the cache
    let mut is_zero: bool = false;
    let mut td: *mut mi_thread_data_s = core::ptr::null_mut::<mi_thread_data_s>();
    for i in 0i32..16i32 {
        td = mi_atomic_load_ptr_relaxed__2!(
            unsafe { core::ptr::addr_of_mut!(td_cache.0) as *mut *mut mi_thread_data_s }
                .wrapping_offset(i as isize)
        );
        if !td.is_null() {
            // found cached allocation, try use it
            td = mi_atomic_exchange_ptr_acq_rel!(
                unsafe { core::ptr::addr_of_mut!(td_cache.0) as *mut *mut mi_thread_data_s }
                    .wrapping_offset(i as isize)
            );
            if !td.is_null() {
                break;
            }
        }
    }
    // if that fails, allocate as meta data
    if td.is_null() {
        let mut memid: mi_memid_s = unsafe { core::mem::zeroed() };
        td = _mi_os_alloc(4696u64, core::ptr::addr_of_mut!(memid), unsafe {
            core::ptr::addr_of_mut!(_mi_stats_main.0)
        }) as *mut mi_thread_data_s;
        if td.is_null() {
            // if this fails, try once more. (issue #257)
            td = _mi_os_alloc(4696u64, core::ptr::addr_of_mut!(memid), unsafe {
                core::ptr::addr_of_mut!(_mi_stats_main.0)
            }) as *mut mi_thread_data_s;
            if td.is_null() {
                // really out of memory
                unsafe {
                    _mi_error_message(
                        slate_support::ENOMEM as i32,
                        c"unable to allocate thread local heap metadata (%zu bytes)\n".as_ptr(),
                        4696u64,
                    )
                };
            }
        }
        if !td.is_null() {
            unsafe {
                (*td).memid = memid;
            }
            is_zero = memid.initially_zero;
        }
    }
    if !td.is_null() && !is_zero {
        _mi_memzero_aligned(td as *mut (), 4672u64);
    }
    td
}

fn mi_thread_data_free(tdfree: *mut mi_thread_data_s) {
    // try to add the thread metadata to the cache
    for i in 0i32..16i32 {
        let td: *mut mi_thread_data_s = mi_atomic_load_ptr_relaxed__2!(
            unsafe { core::ptr::addr_of_mut!(td_cache.0) as *mut *mut mi_thread_data_s }
                .wrapping_offset(i as isize)
        );
        if td.is_null() {
            let mut expected: *mut mi_thread_data_s = core::ptr::null_mut::<mi_thread_data_s>();
            if mi_atomic_cas_ptr_weak_acq_rel__2!(
                unsafe { core::ptr::addr_of_mut!(td_cache.0) as *mut *mut mi_thread_data_s }
                    .wrapping_offset(i as isize),
                core::ptr::addr_of_mut!(expected),
                tdfree
            ) {
                return;
            }
        }
    }
    // if that fails, just free it directly
    _mi_os_free(
        tdfree as *mut (),
        4696u64,
        unsafe { (*tdfree).memid },
        unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_thread_data_collect() {
    // free all thread metadata from the cache
    for i in 0i32..16i32 {
        let mut td: *mut mi_thread_data_s = mi_atomic_load_ptr_relaxed__2!(
            unsafe { core::ptr::addr_of_mut!(td_cache.0) as *mut *mut mi_thread_data_s }
                .wrapping_offset(i as isize)
        );
        if !td.is_null() {
            td = mi_atomic_exchange_ptr_acq_rel!(
                unsafe { core::ptr::addr_of_mut!(td_cache.0) as *mut *mut mi_thread_data_s }
                    .wrapping_offset(i as isize)
            );
            if !td.is_null() {
                _mi_os_free(td as *mut (), 4696u64, unsafe { (*td).memid }, unsafe {
                    core::ptr::addr_of_mut!(_mi_stats_main.0)
                });
            }
        }
    }
}

fn _mi_heap_init() -> bool {
    if mi_heap_is_initialized(mi_prim_get_default_heap()) {
        return true;
    }
    if _mi_is_main_thread() {
        // mi_assert_internal(_mi_heap_main.thread_id != 0);  // can happen on freeBSD where alloc is called before any initialization
        //
        // the main heap is statically allocated
        mi_heap_main_init();
        _mi_heap_set_default_direct(unsafe { core::ptr::addr_of_mut!(_mi_heap_main) });
    // mi_assert_internal(_mi_heap_default->tld->heap_backing == mi_prim_get_default_heap());
    } else {
        // use `_mi_os_alloc` to allocate directly from the OS
        let td: *mut mi_thread_data_s = mi_thread_data_zalloc();
        if td.is_null() {
            return false;
        }
        _mi_tld_init(unsafe { core::ptr::addr_of_mut!((*td).tld) }, unsafe {
            core::ptr::addr_of_mut!((*td).heap)
        });
        _mi_heap_init_ex(
            unsafe { core::ptr::addr_of_mut!((*td).heap) },
            unsafe { core::ptr::addr_of_mut!((*td).tld) },
            _mi_arena_id_none(),
            false,
            0u8,
        );
        _mi_heap_set_default_direct(unsafe { core::ptr::addr_of_mut!((*td).heap) });
    }
    false
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_tld_init(tld: *mut mi_tld_s, bheap: *mut mi_heap_s) {
    _mi_memcpy_aligned(
        tld as *mut (),
        (unsafe { core::ptr::addr_of!(tld_empty.0) }) as *const (),
        core::mem::size_of::<mi_tld_s>() as u64,
    );
    unsafe {
        (*tld).segments.stats = core::ptr::addr_of_mut!((*tld).stats);
    }
    unsafe {
        (*tld).segments.os = core::ptr::addr_of_mut!((*tld).os);
    }
    unsafe {
        (*tld).segments.abandoned = core::ptr::addr_of_mut!(_mi_abandoned_default);
    }
    unsafe {
        (*tld).os.stats = core::ptr::addr_of_mut!((*tld).stats);
    }
    unsafe {
        (*tld).heap_backing = bheap;
    }
}

fn _mi_heap_done(mut heap: *mut mi_heap_s) -> bool {
    if !mi_heap_is_initialized(heap) {
        return true;
    }
    // reset default heap
    _mi_heap_set_default_direct(if _mi_is_main_thread() {
        unsafe { core::ptr::addr_of_mut!(_mi_heap_main) }
    } else {
        (unsafe { core::ptr::addr_of!(_mi_heap_empty.0) }) as *mut mi_heap_s
    });
    // switch to backing heap
    heap = unsafe { (*(*heap).tld).heap_backing };
    if !mi_heap_is_initialized(heap) {
        return false;
    }
    // delete all non-backing heaps in this thread
    let mut curr: *mut mi_heap_s = unsafe { (*(*heap).tld).heaps };
    while !curr.is_null() {
        let next: *mut mi_heap_s = unsafe { (*curr).next }; // save `next` as `curr` will be freed
        if curr != heap {
            mi_heap_delete(curr);
        }
        curr = next;
    }
    // collect if not the main thread
    if heap != unsafe { core::ptr::addr_of_mut!(_mi_heap_main) } {
        _mi_heap_collect_abandon(heap);
    }
    // merge stats
    _mi_stats_done(unsafe { core::ptr::addr_of_mut!((*(*heap).tld).stats) });
    // free if not the main thread
    if heap != unsafe { core::ptr::addr_of_mut!(_mi_heap_main) } {
        // the following assertion does not always hold for huge segments as those are always treated
        //
        // as abondened: one may allocate it in one thread, but deallocate in another in which case
        //
        // the count can be too large or negative. todo: perhaps not count huge segments? see issue #363
        //
        // mi_assert_internal(heap->tld->segments.count == 0 || heap->thread_id != _mi_thread_id());
        mi_thread_data_free(heap as *mut mi_thread_data_s);
    }
    false
}

fn mi_process_setup_auto_thread_done() {
    // fine if it races
    if unsafe { tls_initialized } {
        return;
    }
    unsafe {
        tls_initialized = true;
    }
    _mi_prim_thread_init_auto_done();
    _mi_heap_set_default_direct(unsafe { core::ptr::addr_of_mut!(_mi_heap_main) });
}

static mut tls_initialized: bool = 0i32 != 0i32;

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_is_main_thread() -> bool {
    (unsafe { _mi_heap_main.thread_id }) == 0
        || (unsafe { _mi_heap_main.thread_id }) == _mi_thread_id()
}

static mut thread_count: u64 = 1;

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_current_thread_count() -> u64 {
    mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(thread_count) })
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_thread_init() {
    // ensure our process has started already
    mi_process_init();
    // initialize the thread local default heap
    //
    // (this will call `_mi_heap_set_default_direct` and thus set the
    //
    //  fiber/pthread key to a non-zero value, ensuring `_mi_thread_done` is called)
    if _mi_heap_init() {
        return;
    }
    // returns true if already initialized
    _mi_stat_increase(
        unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.threads) },
        1u64,
    );
    mi_atomic_increment_relaxed!(unsafe { core::ptr::addr_of_mut!(thread_count) });
    // _mi_verbose_message("thread init: 0x%zx\n", _mi_thread_id());
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_thread_done() {
    _mi_thread_done(core::ptr::null_mut::<mi_heap_s>());
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_thread_done(mut heap: *mut mi_heap_s) {
    // calling with NULL implies using the default heap
    if heap.is_null() {
        heap = mi_prim_get_default_heap();
        if heap.is_null() {
            return;
        }
    }
    // prevent re-entrancy through heap_done/heap_set_default_direct (issue #699)
    if !mi_heap_is_initialized(heap) {
        return;
    }
    // adjust stats
    mi_atomic_decrement_relaxed!(unsafe { core::ptr::addr_of_mut!(thread_count) });
    _mi_stat_decrease(
        unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.threads) },
        1u64,
    );
    // check thread-id as on Windows shutdown with FLS the main (exit) thread may call this on thread-local heaps...
    if (unsafe { (*heap).thread_id }) != _mi_thread_id() {
        return;
    }
    // abandon the thread local heap
    if _mi_heap_done(heap) {
        return;
    }
    // returns true if already ran
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_set_default_direct(heap: *mut mi_heap_s) {
    unsafe {
        _mi_heap_default = heap;
    }
    // ensure the default heap is passed to `_mi_thread_done`
    //
    // setting to a non-NULL value also ensures `mi_thread_done` is called.
    _mi_prim_thread_associate_default_heap(heap);
}

static mut os_preloading: bool = 1i32 != 0i32;

static mut mi_redirected: bool = 0i32 != 0i32;

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_preloading() -> bool {
    unsafe { os_preloading }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_is_redirected() -> bool {
    unsafe { mi_redirected }
}

fn mi_allocator_init(message: *mut *const i8) -> bool {
    if !message.is_null() {
        unsafe {
            *message = core::ptr::null::<i8>();
        }
    }
    true
}

fn mi_allocator_done() {
    // nothing to do
}

fn mi_process_load() {
    mi_heap_main_init();
    unsafe {
        os_preloading = false;
    }
    unsafe { atexit(Some(mi_process_done)) };
    _mi_options_init();
    mi_process_setup_auto_thread_done();
    mi_process_init();
    if unsafe { mi_redirected } {
        unsafe { _mi_verbose_message(c"malloc is redirected.\n".as_ptr()) };
    }
    // show message from the redirector (if present)
    let mut msg: *const i8 = core::ptr::null::<i8>();
    mi_allocator_init(core::ptr::addr_of_mut!(msg));
    let __v0: bool = !msg.is_null() && (mi_option_is_enabled(2u32) || mi_option_is_enabled(0u32));
    if __v0 {
        _mi_fputs(
            None,
            core::ptr::null_mut::<()>(),
            core::ptr::null::<i8>(),
            msg,
        );
    }
    // reseed random
    _mi_random_reinit_if_weak(unsafe { core::ptr::addr_of_mut!(_mi_heap_main.random) });
}

fn mi_detect_cpu_features() {
    // nothing
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_process_init() {
    // ensure we are called once
    mi_heap_main_init(); // vs2017 can dynamically re-initialize _mi_heap_main
    if !mi_atomic_once(unsafe { core::ptr::addr_of_mut!(process_init) }) {
        return;
    }
    unsafe {
        _mi_process_is_initialized = true;
    }
    unsafe { _mi_verbose_message(c"process init: 0x%zx\n".as_ptr(), _mi_thread_id()) };
    mi_process_setup_auto_thread_done();
    mi_detect_cpu_features();
    _mi_os_init();
    mi_heap_main_init();
    unsafe { _mi_verbose_message(c"secure level: %d\n".as_ptr(), 0i32) };
    unsafe { _mi_verbose_message(c"mem tracking: %s\n".as_ptr(), c"none".as_ptr() as *mut i8) };
    mi_thread_init();
    mi_stats_reset(); // only call stat reset *after* thread init (or the heap tld == NULL)
    if mi_option_is_enabled(7u32) {
        let pages: u64 = mi_option_get_clamp(7u32, 0i64, 131072i64) as u64;
        let reserve_at: i64 = mi_option_get(8u32);
        if reserve_at != (1i32.wrapping_neg() as i64) {
            mi_reserve_huge_os_pages_at(pages, reserve_at as i32, pages.wrapping_mul(500));
        } else {
            mi_reserve_huge_os_pages_interleave(pages, 0u64, pages.wrapping_mul(500));
        }
    }
    if mi_option_is_enabled(9u32) {
        let ksize: i64 = mi_option_get(9u32);
        if ksize > 0 {
            mi_reserve_os_memory((ksize as u64).wrapping_mul(1024), true, true); // commit?
            // allow large pages?
        }
    }
}

static mut process_init: u64 = unsafe { core::mem::zeroed() };

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_process_done")]
extern "C-unwind" fn mi_process_done() {
    // only shutdown if we were initialized
    if !unsafe { _mi_process_is_initialized } {
        return;
    }
    // ensure we are called once
    if unsafe { process_done } {
        return;
    }
    unsafe {
        process_done = true;
    }
    // release any thread specific resources and ensure _mi_thread_done is called on all but the main thread
    _mi_prim_thread_done_auto_done();
    // free all memory if possible on process exit. This is not needed for a stand-alone process
    //
    // but should be done if mimalloc is statically linked into another shared library which
    //
    // is repeatedly loaded/unloaded, see issue #281.
    mi_collect(true); // force
    // Forcefully release all retained memory; this can be dangerous in general if overriding regular malloc/free
    //
    // since after process_done there might still be other code running that calls `free` (like at_exit routines,
    //
    // or C-runtime termination code.
    if mi_option_is_enabled(22u32) {
        mi_collect(true); // force
        _mi_heap_unsafe_destroy_all(); // forcefully release all memory held by all heaps (of this thread only!)
        _mi_arena_unsafe_destroy_all(unsafe {
            core::ptr::addr_of_mut!((*(*_mi_heap_main_get()).tld).stats)
        });
    }
    let __v0: bool = mi_option_is_enabled(1u32) || mi_option_is_enabled(2u32);
    if __v0 {
        mi_stats_print(core::ptr::null_mut::<()>());
    }
    mi_allocator_done();
    unsafe { _mi_verbose_message(c"process done: 0x%zx\n".as_ptr(), _mi_heap_main.thread_id) };
    unsafe {
        os_preloading = true;
    }
    // don't call the C runtime anymore
}

static mut process_done: bool = 0i32 != 0i32;

fn _mi_process_init() {
    mi_process_load();
}

static mut mi_max_error_count: i64 = 16;

static mut mi_max_warning_count: i64 = 16;

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_version() -> i32 {
    212
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mi_option_desc_s {
    value: i64,
    init: u32,
    option: u32,
    name: *const i8,
    legacy_name: *const i8,
}

static mut options: __SlateAlign16<[mi_option_desc_s; 26]> = __SlateAlign16([
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 0u32,
        name: (b"show_errors\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 1u32,
        name: (b"show_stats\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 2u32,
        name: (b"verbose\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 1i64,
        init: 0u32,
        option: 3u32,
        name: (b"eager_commit\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 2i64,
        init: 0u32,
        option: 4u32,
        name: (b"arena_eager_commit\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: (b"eager_region_commit\0".as_ptr() as *mut i8) as *const i8,
    },
    mi_option_desc_s {
        value: 1i64,
        init: 0u32,
        option: 5u32,
        name: (b"purge_decommits\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: (b"reset_decommits\0".as_ptr() as *mut i8) as *const i8,
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 6u32,
        name: (b"allow_large_os_pages\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: (b"large_os_pages\0".as_ptr() as *mut i8) as *const i8,
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 7u32,
        name: (b"reserve_huge_os_pages\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 1i32.wrapping_neg() as i64,
        init: 0u32,
        option: 8u32,
        name: (b"reserve_huge_os_pages_at\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 9u32,
        name: (b"reserve_os_memory\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 10u32,
        name: (b"deprecated_segment_cache\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 11u32,
        name: (b"deprecated_page_reset\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 12u32,
        name: (b"abandoned_page_purge\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: (b"abandoned_page_reset\0".as_ptr() as *mut i8) as *const i8,
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 13u32,
        name: (b"deprecated_segment_reset\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 1i64,
        init: 0u32,
        option: 14u32,
        name: (b"eager_commit_delay\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 10i64,
        init: 0u32,
        option: 15u32,
        name: (b"purge_delay\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: (b"reset_delay\0".as_ptr() as *mut i8) as *const i8,
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 16u32,
        name: (b"use_numa_nodes\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 17u32,
        name: (b"limit_os_alloc\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 100i64,
        init: 0u32,
        option: 18u32,
        name: (b"os_tag\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 16i64,
        init: 0u32,
        option: 19u32,
        name: (b"max_errors\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 16i64,
        init: 0u32,
        option: 20u32,
        name: (b"max_warnings\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 8i64,
        init: 0u32,
        option: 21u32,
        name: (b"max_segment_reclaim\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 0i64,
        init: 0u32,
        option: 22u32,
        name: (b"destroy_on_exit\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 1048576i64,
        init: 0u32,
        option: 23u32,
        name: (b"arena_reserve\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 10i64,
        init: 0u32,
        option: 24u32,
        name: (b"arena_purge_mult\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: core::ptr::null::<i8>(),
    },
    mi_option_desc_s {
        value: 1i64,
        init: 0u32,
        option: 25u32,
        name: (b"purge_extend_delay\0".as_ptr() as *mut i8) as *const i8,
        legacy_name: (b"decommit_extend_delay\0".as_ptr() as *mut i8) as *const i8,
    },
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_options_init() {
    // called on process load; should not be called before the CRT is initialized!
    //
    // (e.g. do not call this from process_init as that may run before CRT initialization)
    mi_add_stderr_output(); // now it safe to use stderr for output
    for i in 0i32..26i32 {
        let option: u32 = i as u32;
        let l: i64 = mi_option_get(option);
        MI_UNUSED!(l); // initialize
        // if (option != mi_option_verbose)
        let desc: *mut mi_option_desc_s =
            unsafe { core::ptr::addr_of_mut!(options.0) as *mut mi_option_desc_s }
                .wrapping_offset((option as u32) as isize);
        unsafe { _mi_verbose_message(c"option '%s': %ld\n".as_ptr(), (*desc).name, (*desc).value) };
    }
    unsafe {
        mi_max_error_count = mi_option_get(19u32);
    }
    unsafe {
        mi_max_warning_count = mi_option_get(20u32);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_get(option: u32) -> i64 {
    if (option as u32) < 0 || (option as u32) >= 26 {
        return 0;
    }
    let desc: *mut mi_option_desc_s =
        unsafe { core::ptr::addr_of_mut!(options.0) as *mut mi_option_desc_s }
            .wrapping_offset((option as u32) as isize);
    // index should match the option
    if ((unsafe { (*desc).init }) as u32) == 0 {
        mi_option_init(desc);
    }
    unsafe { (*desc).value }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_get_clamp(option: u32, min: i64, max: i64) -> i64 {
    let x: i64 = mi_option_get(option);
    if x < min {
        min
    } else {
        if x > max { max } else { x }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_get_size(option: u32) -> u64 {
    let x: i64 = mi_option_get(option);
    if x < 0 {
        0
    } else {
        (x as u64).wrapping_mul(1024)
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_set(option: u32, value: i64) {
    if (option as u32) < 0 || (option as u32) >= 26 {
        return;
    }
    let desc: *mut mi_option_desc_s =
        unsafe { core::ptr::addr_of_mut!(options.0) as *mut mi_option_desc_s }
            .wrapping_offset((option as u32) as isize);
    // index should match the option
    unsafe {
        (*desc).value = value;
    }
    unsafe {
        (*desc).init = 2;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_set_default(option: u32, value: i64) {
    if (option as u32) < 0 || (option as u32) >= 26 {
        return;
    }
    let desc: *mut mi_option_desc_s =
        unsafe { core::ptr::addr_of_mut!(options.0) as *mut mi_option_desc_s }
            .wrapping_offset((option as u32) as isize);
    if ((unsafe { (*desc).init }) as u32) != 2 {
        unsafe {
            (*desc).value = value;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_is_enabled(option: u32) -> bool {
    mi_option_get(option) != 0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_set_enabled(option: u32, enable: bool) {
    mi_option_set(option, (if enable { 1i32 } else { 0i32 }) as i64);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_set_enabled_default(option: u32, enable: bool) {
    mi_option_set_default(option, (if enable { 1i32 } else { 0i32 }) as i64);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_enable(option: u32) {
    mi_option_set_enabled(option, true);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_option_disable(option: u32) {
    mi_option_set_enabled(option, false);
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_out_stderr")]
extern "C-unwind" fn mi_out_stderr(msg: *const i8, arg: *mut ()) {
    MI_UNUSED!(arg);
    if !msg.is_null() && ((unsafe { *msg.wrapping_offset(0 as isize) }) as i32) != 0 {
        _mi_prim_out_stderr(msg);
    }
}

static mut out_buf: __SlateAlign16<[i8; 32769]> = __SlateAlign16(unsafe { core::mem::zeroed() });

static mut out_len: u64 = unsafe { core::mem::zeroed() };

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_out_buf")]
extern "C-unwind" fn mi_out_buf(msg: *const i8, arg: *mut ()) {
    MI_UNUSED!(arg);
    if msg.is_null() {
        return;
    }
    if mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(out_len) }) >= 32768u64 {
        return;
    }
    let mut n: u64 = _mi_strlen(msg);
    if n == 0 {
        return;
    }
    // claim space
    let start: u64 = mi_atomic_add_acq_rel!(unsafe { core::ptr::addr_of_mut!(out_len) }, n);
    if start >= 32768u64 {
        return;
    }
    // check bound
    if start.wrapping_add(n) >= 32768u64 {
        n = 32768u64.wrapping_sub(start).wrapping_sub(1);
    }
    _mi_memcpy(
        unsafe { core::ptr::addr_of_mut!(out_buf.0) as *mut i8 }.wrapping_offset(start as isize)
            as *mut (),
        msg as *const (),
        n,
    );
}

fn mi_out_buf_flush(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    no_more_buf: bool,
    arg: *mut (),
) {
    if out == None {
        return;
    }
    // claim (if `no_more_buf == true`, no more output will be added after this point)
    let mut count: u64 = mi_atomic_add_acq_rel!(
        unsafe { core::ptr::addr_of_mut!(out_len) },
        if no_more_buf { 32768u64 } else { 1u64 }
    );
    // and output the current contents
    if count > 32768u64 {
        count = 32768u64;
    }
    unsafe {
        out_buf.0[count as usize] = 0;
    }
    unsafe {
        out.unwrap()(
            (core::ptr::addr_of_mut!(out_buf.0) as *mut i8) as *const i8,
            arg,
        )
    };
    if !no_more_buf {
        unsafe {
            out_buf.0[count as usize] = 10;
        }
        // if continue with the buffer, insert a newline
    }
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_out_buf_stderr")]
extern "C-unwind" fn mi_out_buf_stderr(msg: *const i8, arg: *mut ()) {
    mi_out_stderr(msg, arg);
    mi_out_buf(msg, arg);
}

static mut mi_out_default: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())> =
    unsafe { core::mem::zeroed() };

static mut mi_out_arg: *mut () = unsafe { core::mem::zeroed() };

fn mi_out_get_default(
    parg: *mut *mut (),
) -> Option<unsafe extern "C-unwind" fn(*const i8, *mut ())> {
    if !parg.is_null() {
        unsafe {
            *parg = mi_atomic_load_ptr_acquire__2!(core::ptr::addr_of_mut!(mi_out_arg));
        }
    }
    let out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())> =
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(mi_out_default)) };
    {
        let __t0: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())> =
            if out == None { Some(mi_out_buf) } else { out };
        __t0
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_register_output(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(mi_out_default), {
            let __t0: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())> = if out == None {
                Some(mi_out_stderr)
            } else {
                out
            };
            __t0
        })
    };
    // stop using the delayed output buffer
    mi_atomic_store_ptr_release!(unsafe { core::ptr::addr_of_mut!(mi_out_arg) }, arg);
    if out != None {
        mi_out_buf_flush(out, true, arg);
    }
    // output all the delayed output now
}

fn mi_add_stderr_output() {
    mi_out_buf_flush(Some(mi_out_stderr), false, core::ptr::null_mut::<()>()); // flush current contents to stderr
    unsafe {
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!(mi_out_default),
            Some(mi_out_buf_stderr),
        )
    }; // and add stderr to the delayed output
}

static mut error_count: u64 = unsafe { core::mem::zeroed() };

static mut warning_count: u64 = unsafe { core::mem::zeroed() };

#[thread_local]
static mut recurse: bool = 0i32 != 0i32;

#[inline(never)]
fn mi_recurse_enter_prim() -> bool {
    if unsafe { recurse } {
        return false;
    }
    unsafe {
        recurse = true;
    }
    true
}

#[inline(never)]
fn mi_recurse_exit_prim() {
    unsafe {
        recurse = false;
    }
}

fn mi_recurse_enter() -> bool {
    mi_recurse_enter_prim()
}

fn mi_recurse_exit() {
    mi_recurse_exit_prim();
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_fputs(
    mut out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    mut arg: *mut (),
    prefix: *const i8,
    message: *const i8,
) {
    if out == None
        || (unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>, *mut ()>(
                out,
            )
        }) == (slate_support::stdout() as *mut ())
        || (unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>, *mut ()>(
                out,
            )
        }) == (slate_support::stderr() as *mut ())
    {
        // TODO: use mi_out_stderr for stderr?
        if !mi_recurse_enter() {
            return;
        }
        out = mi_out_get_default(core::ptr::addr_of_mut!(arg));
        if !prefix.is_null() {
            unsafe { out.unwrap()(prefix, arg) };
        }
        unsafe { out.unwrap()(message, arg) };
        mi_recurse_exit();
    } else {
        if !prefix.is_null() {
            unsafe { out.unwrap()(prefix, arg) };
        }
        unsafe { out.unwrap()(message, arg) };
    }
}

fn mi_vfprintf(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
    prefix: *const i8,
    fmt: *const i8,
    mut args: core::ffi::VaList<'static>,
) {
    let mut buf: __SlateAlign16<[i8; 512]> = __SlateAlign16([0i8; 512]);
    if fmt.is_null() {
        return;
    }
    if !mi_recurse_enter() {
        return;
    }
    unsafe {
        vsnprintf(
            buf.0.as_mut_ptr() as *mut i8,
            (core::mem::size_of::<[i8; 512]>() as u64).wrapping_sub(1),
            fmt,
            args.clone(),
        )
    };
    mi_recurse_exit();
    _mi_fputs(out, arg, prefix, buf.0.as_mut_ptr() as *const i8);
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _mi_fprintf(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
    fmt: *const i8,
    mut __va_args: ...
) {
    let mut args: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    args = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    mi_vfprintf(out, arg, core::ptr::null::<i8>(), fmt, args.clone());
}

fn mi_vfprintf_thread(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
    prefix: *const i8,
    fmt: *const i8,
    mut args: core::ffi::VaList<'static>,
) {
    let __v0: bool = !prefix.is_null() && _mi_strnlen(prefix, 33u64) <= 32;
    let __v1: bool = __v0 && !_mi_is_main_thread();
    if __v1 {
        let mut tprefix: __SlateAlign16<[i8; 64]> = __SlateAlign16([0i8; 64]);
        unsafe {
            snprintf(
                (tprefix.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                (core::mem::size_of::<[i8; 64]>() as u64) as usize,
                c"%sthread 0x%llx: ".as_ptr() as *const core::ffi::c_char,
                prefix,
                _mi_thread_id(),
            ) as i32
        };
        mi_vfprintf(
            out,
            arg,
            tprefix.0.as_mut_ptr() as *const i8,
            fmt,
            args.clone(),
        );
    } else {
        mi_vfprintf(out, arg, prefix, fmt, args.clone());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _mi_trace_message(fmt: *const i8, mut __va_args: ...) {
    if mi_option_get(2u32) <= 1 {
        return;
    }
    // only with verbose level 2 or higher
    let mut args: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    args = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    mi_vfprintf_thread(
        None,
        core::ptr::null_mut::<()>(),
        c"mimalloc: ".as_ptr(),
        fmt,
        args.clone(),
    );
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _mi_verbose_message(fmt: *const i8, mut __va_args: ...) {
    if !mi_option_is_enabled(2u32) {
        return;
    }
    let mut args: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    args = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    mi_vfprintf(
        None,
        core::ptr::null_mut::<()>(),
        c"mimalloc: ".as_ptr(),
        fmt,
        args.clone(),
    );
}

fn mi_show_error_message(fmt: *const i8, mut args: core::ffi::VaList<'static>) {
    if !mi_option_is_enabled(2u32) {
        if !mi_option_is_enabled(0u32) {
            return;
        }
        let __v0: bool = (unsafe { mi_max_error_count }) >= 0
            && (mi_atomic_increment_acq_rel!(unsafe { core::ptr::addr_of_mut!(error_count) })
                as i64)
                > unsafe { mi_max_error_count };
        if __v0 {
            return;
        }
    }
    mi_vfprintf_thread(
        None,
        core::ptr::null_mut::<()>(),
        c"mimalloc: error: ".as_ptr(),
        fmt,
        args.clone(),
    );
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _mi_warning_message(fmt: *const i8, mut __va_args: ...) {
    if !mi_option_is_enabled(2u32) {
        if !mi_option_is_enabled(0u32) {
            return;
        }
        let __v0: bool = (unsafe { mi_max_warning_count }) >= 0
            && (mi_atomic_increment_acq_rel!(unsafe { core::ptr::addr_of_mut!(warning_count) })
                as i64)
                > unsafe { mi_max_warning_count };
        if __v0 {
            return;
        }
    }
    let mut args: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    args = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    mi_vfprintf_thread(
        None,
        core::ptr::null_mut::<()>(),
        c"mimalloc: warning: ".as_ptr(),
        fmt,
        args.clone(),
    );
}

static mut mi_error_handler: Option<unsafe extern "C-unwind" fn(i32, *mut ())> =
    unsafe { core::mem::zeroed() };

static mut mi_error_arg: *mut () = unsafe { core::mem::zeroed() };

fn mi_error_default(err: i32) {
    MI_UNUSED!(err);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_register_error(
    fun: Option<unsafe extern "C-unwind" fn(i32, *mut ())>,
    arg: *mut (),
) {
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(mi_error_handler), fun) }; // can be NULL
    mi_atomic_store_ptr_release!(unsafe { core::ptr::addr_of_mut!(mi_error_arg) }, arg);
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _mi_error_message(err: i32, fmt: *const i8, mut __va_args: ...) {
    // show detailed error message
    let mut args: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    args = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    mi_show_error_message(fmt, args.clone());
    // and call the error handler which may abort (or return normally)
    if (unsafe { core::ptr::read_volatile(core::ptr::addr_of!(mi_error_handler)) }) != None {
        unsafe {
            core::ptr::read_volatile(core::ptr::addr_of!(mi_error_handler)).unwrap()(
                err,
                mi_atomic_load_ptr_acquire__2!(core::ptr::addr_of_mut!(mi_error_arg)),
            )
        };
    } else {
        mi_error_default(err);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_toupper(c: i8) -> i8 {
    if (c as i32) >= 97 && (c as i32) <= 122 {
        return (c as i32).wrapping_sub(97).wrapping_add(65) as i8;
    } else {
        return c;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_strnicmp(mut s: *const i8, mut t: *const i8, mut n: u64) -> i32 {
    if n == 0 {
        return 0;
    }
    while ((unsafe { *s }) as i32) != 0 && ((unsafe { *t }) as i32) != 0 && n > 0 {
        if (_mi_toupper(unsafe { *s }) as i32) != (_mi_toupper(unsafe { *t }) as i32) {
            break;
        }
        let __v1: *const i8 = s;
        s = __v1.wrapping_offset(1 as isize);
        let __v3: *const i8 = t;
        t = __v3.wrapping_offset(1 as isize);
        n = n.wrapping_sub(1);
    }
    if n == 0 {
        0
    } else {
        ((unsafe { *s }) as i32).wrapping_sub((unsafe { *t }) as i32)
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_strlcpy(mut dest: *mut i8, mut src: *const i8, mut dest_size: u64) {
    if dest.is_null() || src.is_null() || dest_size == 0 {
        return;
    }
    // copy until end of src, or when dest is (almost) full
    while ((unsafe { *src }) as i32) != 0 && dest_size > 1 {
        let __v0: *const i8 = src;
        src = __v0.wrapping_offset(1 as isize);
        let __v2: *mut i8 = dest;
        dest = __v2.wrapping_offset(1 as isize);
        unsafe {
            *__v2 = *__v0;
        }
        dest_size = dest_size.wrapping_sub(1);
    }
    // always zero terminate
    unsafe {
        *dest = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_strlcat(mut dest: *mut i8, src: *const i8, mut dest_size: u64) {
    if dest.is_null() || src.is_null() || dest_size == 0 {
        return;
    }
    // find end of string in the dest buffer
    while ((unsafe { *dest }) as i32) != 0 && dest_size > 1 {
        let __v0: *mut i8 = dest;
        dest = __v0.wrapping_offset(1 as isize);
        dest_size = dest_size.wrapping_sub(1);
    }
    // and catenate
    _mi_strlcpy(dest, src, dest_size);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_strlen(s: *const i8) -> u64 {
    if s.is_null() {
        return 0;
    }
    let mut len: u64 = 0;
    while ((unsafe { *s.wrapping_offset(len as isize) }) as i32) != 0 {
        len = len.wrapping_add(1);
    }
    len
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_strnlen(s: *const i8, max_len: u64) -> u64 {
    if s.is_null() {
        return 0;
    }
    let mut len: u64 = 0;
    while ((unsafe { *s.wrapping_offset(len as isize) }) as i32) != 0 && len < max_len {
        len = len.wrapping_add(1);
    }
    len
}

fn mi_getenv(name: *const i8, result: *mut i8, result_size: u64) -> bool {
    if name.is_null() || result.is_null() || result_size < 64 {
        return false;
    }
    _mi_prim_getenv(name, result, result_size)
}

fn mi_option_init(desc: *mut mi_option_desc_s) {
    // Read option value from the environment
    let mut s: __SlateAlign16<[i8; 65]> = __SlateAlign16([0i8; 65]);
    let mut buf: __SlateAlign16<[i8; 65]> = __SlateAlign16([0i8; 65]);
    _mi_strlcpy(
        buf.0.as_mut_ptr() as *mut i8,
        c"mimalloc_".as_ptr(),
        core::mem::size_of::<[i8; 65]>() as u64,
    );
    _mi_strlcat(
        buf.0.as_mut_ptr() as *mut i8,
        unsafe { (*desc).name },
        core::mem::size_of::<[i8; 65]>() as u64,
    );
    let mut found: bool = mi_getenv(
        buf.0.as_mut_ptr() as *const i8,
        s.0.as_mut_ptr() as *mut i8,
        core::mem::size_of::<[i8; 65]>() as u64,
    );
    if !found && !unsafe { (*desc).legacy_name }.is_null() {
        _mi_strlcpy(
            buf.0.as_mut_ptr() as *mut i8,
            c"mimalloc_".as_ptr(),
            core::mem::size_of::<[i8; 65]>() as u64,
        );
        _mi_strlcat(
            buf.0.as_mut_ptr() as *mut i8,
            unsafe { (*desc).legacy_name },
            core::mem::size_of::<[i8; 65]>() as u64,
        );
        found = mi_getenv(
            buf.0.as_mut_ptr() as *const i8,
            s.0.as_mut_ptr() as *mut i8,
            core::mem::size_of::<[i8; 65]>() as u64,
        );
        if found {
            unsafe {
                _mi_warning_message(c"environment option \"mimalloc_%s\" is deprecated -- use \"mimalloc_%s\" instead.\n".as_ptr(), (*desc).legacy_name, (*desc).name)
            };
        }
    }
    if found {
        let len: u64 = _mi_strnlen(
            s.0.as_mut_ptr() as *const i8,
            (core::mem::size_of::<[i8; 65]>() as u64).wrapping_sub(1),
        );
        for i in 0..len {
            buf.0[i as usize] = _mi_toupper(s.0[i as usize]);
        }
        buf.0[len as usize] = 0;
        if (buf.0[0] as i32) == 0
            || !unsafe {
                strstr(
                    c"1;TRUE;YES;ON".as_ptr() as *const core::ffi::c_char,
                    (buf.0.as_mut_ptr() as *const i8) as *const core::ffi::c_char,
                ) as *mut i8
            }
            .is_null()
        {
            unsafe {
                (*desc).value = 1;
            }
            unsafe {
                (*desc).init = 2;
            }
        } else {
            if !unsafe {
                strstr(
                    c"0;FALSE;NO;OFF".as_ptr() as *const core::ffi::c_char,
                    (buf.0.as_mut_ptr() as *const i8) as *const core::ffi::c_char,
                ) as *mut i8
            }
            .is_null()
            {
                unsafe {
                    (*desc).value = 0;
                }
                unsafe {
                    (*desc).init = 2;
                }
            } else {
                let mut end: *mut i8 = buf.0.as_mut_ptr() as *mut i8;
                let mut value: i64 = unsafe {
                    strtol(
                        buf.0.as_mut_ptr() as *const i8,
                        core::ptr::addr_of_mut!(end),
                        10i32,
                    )
                };
                if ((unsafe { (*desc).option }) as u32) == 9
                    || ((unsafe { (*desc).option }) as u32) == 23
                {
                    // this option is interpreted in KiB to prevent overflow of `long`
                    if ((unsafe { *end }) as i32) == 75 {
                        let __v1: *mut i8 = end;
                        end = __v1.wrapping_offset(1 as isize);
                    } else {
                        if ((unsafe { *end }) as i32) == 77 {
                            value = (value as u64).wrapping_mul(1024) as i64;
                            let __v5: *mut i8 = end;
                            end = __v5.wrapping_offset(1 as isize);
                        } else {
                            if ((unsafe { *end }) as i32) == 71 {
                                value = (value as u64).wrapping_mul(1048576u64) as i64;
                                let __v9: *mut i8 = end;
                                end = __v9.wrapping_offset(1 as isize);
                            } else {
                                value = ((value as u64).wrapping_add(1024).wrapping_sub(1) / 1024)
                                    as i64;
                            }
                        }
                    }
                    if ((unsafe { *end.wrapping_offset(0 as isize) }) as i32) == 73
                        && ((unsafe { *end.wrapping_offset(1 as isize) }) as i32) == 66
                    {
                        let __v11: *mut i8 = end;
                        end = __v11.wrapping_offset(2 as isize);
                    } else {
                        if ((unsafe { *end }) as i32) == 66 {
                            let __v13: *mut i8 = end;
                            end = __v13.wrapping_offset(1 as isize);
                        }
                    }
                }
                if ((unsafe { *end }) as i32) == 0 {
                    unsafe {
                        (*desc).value = value;
                    }
                    unsafe {
                        (*desc).init = 2;
                    }
                } else {
                    // set `init` first to avoid recursion through _mi_warning_message on mimalloc_verbose.
                    unsafe {
                        (*desc).init = 1;
                    }
                    if ((unsafe { (*desc).option }) as u32) == 2 && (unsafe { (*desc).value }) == 0
                    {
                        // if the 'mimalloc_verbose' env var has a bogus value we'd never know
                        //
                        // (since the value defaults to 'off') so in that case briefly enable verbose
                        unsafe {
                            (*desc).value = 1;
                        }
                        unsafe {
                            _mi_warning_message(
                                c"environment option mimalloc_%s has an invalid value.\n".as_ptr(),
                                (*desc).name,
                            )
                        };
                        unsafe {
                            (*desc).value = 0;
                        }
                    } else {
                        unsafe {
                            _mi_warning_message(
                                c"environment option mimalloc_%s has an invalid value.\n".as_ptr(),
                                (*desc).name,
                            )
                        };
                    }
                }
            }
        }
    } else {
        if !_mi_preloading() {
            unsafe {
                (*desc).init = 1;
            }
        }
    }
}

static mut mi_os_mem_config: mi_os_mem_config_s = {
    let mut __t0: mi_os_mem_config_s = unsafe { core::mem::zeroed() };
    __t0.page_size = 4096u64;
    __t0.large_page_size = 0u64;
    __t0.alloc_granularity = 4096u64;
    __t0.has_overcommit = 1i32 != 0i32;
    __t0.must_free_whole = 0i32 != 0i32;
    __t0.has_virtual_reserve = 1i32 != 0i32;
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_has_overcommit() -> bool {
    unsafe { mi_os_mem_config.has_overcommit }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_has_virtual_reserve() -> bool {
    unsafe { mi_os_mem_config.has_virtual_reserve }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_page_size() -> u64 {
    unsafe { mi_os_mem_config.page_size }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_large_page_size() -> u64 {
    let __v0: u64 = if (unsafe { mi_os_mem_config.large_page_size }) != 0 {
        unsafe { mi_os_mem_config.large_page_size }
    } else {
        _mi_os_page_size()
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_use_large_page(size: u64, alignment: u64) -> bool {
    // if we have access, check the size and alignment requirements
    let __v0: bool =
        (unsafe { mi_os_mem_config.large_page_size }) == 0 || !mi_option_is_enabled(6u32);
    if __v0 {
        return false;
    }
    size % unsafe { mi_os_mem_config.large_page_size } == 0
        && alignment % unsafe { mi_os_mem_config.large_page_size } == 0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_good_alloc_size(size: u64) -> u64 {
    let mut align_size: u64 = 0;
    if size < 524288u64 {
        align_size = _mi_os_page_size();
    } else {
        if size < 2097152u64 {
            align_size = 65536u64;
        } else {
            if size < 8388608u64 {
                align_size = 262144u64;
            } else {
                if size < 33554432u64 {
                    align_size = 1048576u64;
                } else {
                    align_size = 4194304u64;
                }
            }
        }
    }
    if size >= 18446744073709551615u64.wrapping_sub(align_size) {
        return size;
    }
    // possible overflow?
    _mi_align_up(size, align_size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_init() {
    _mi_prim_mem_init(unsafe { core::ptr::addr_of_mut!(mi_os_mem_config) });
}

fn mi_align_up_ptr(p: *mut (), alignment: u64) -> *mut () {
    _mi_align_up(p as u64, alignment) as *mut ()
}

fn mi_align_down_ptr(p: *mut (), alignment: u64) -> *mut () {
    _mi_align_down(p as u64, alignment) as *mut ()
}

static mut aligned_base: __SlateAlign64<u64> = __SlateAlign64(unsafe { core::mem::zeroed() });

// 2TiB start
// upto 6TiB   (since before win8 there is "only" 8TiB available to processes)
// wrap after 30TiB (area after 32TiB is used for huge OS pages)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._mi_os_get_aligned_hint")]
extern "C-unwind" fn _mi_os_get_aligned_hint(try_alignment: u64, mut size: u64) -> *mut () {
    if try_alignment <= 1 || try_alignment > 33554432u64 {
        return core::ptr::null_mut::<()>();
    }
    size = _mi_align_up(size, 33554432u64);
    if size > 1073741824u64 {
        return core::ptr::null_mut::<()>();
    }
    // guarantee the chance of fixed valid address is at most 1/(MI_HINT_AREA / 1<<30) = 1/4096.
    let mut hint: u64 =
        mi_atomic_add_acq_rel!(unsafe { core::ptr::addr_of_mut!(aligned_base.0) }, size);
    if hint == 0 || hint > 32985348833280u64 {
        // wrap or initialize
        let mut init: u64 = 2199023255552u64;
        let heap: *mut mi_heap_s = mi_prim_get_default_heap();
        // gh-123022: default heap may not be initialized in CPython in background threads
        if mi_heap_is_initialized(heap) {
            let r: u64 = _mi_heap_random_next(heap);
            init = init
                .wrapping_add(33554432u64.wrapping_mul(r >> 17i32 & 1048575) % 4398046511104u64); // (randomly 20 bits)*4MiB == 0 to 4TiB
        }
        let mut expected: u64 = hint.wrapping_add(size);
        mi_atomic_cas_strong_acq_rel__2!(
            unsafe { core::ptr::addr_of_mut!(aligned_base.0) },
            core::ptr::addr_of_mut!(expected),
            init
        );
        hint = mi_atomic_add_acq_rel!(unsafe { core::ptr::addr_of_mut!(aligned_base.0) }, size); // this may still give 0 or > MI_HINT_MAX but that is ok, it is a hint after all
    }
    if hint % try_alignment != 0 {
        return core::ptr::null_mut::<()>();
    }
    hint as *mut ()
}

fn mi_os_prim_free(addr: *mut (), size: u64, still_committed: bool, tld_stats: *mut mi_stats_s) {
    MI_UNUSED!(tld_stats);
    if addr.is_null() || size == 0 {
        return;
    }
    // || _mi_os_is_huge_reserved(addr)
    let err: i32 = _mi_prim_free(addr, size);
    if err != 0 {
        unsafe {
            _mi_warning_message(
                c"unable to free OS memory (error: %d (0x%x), size: 0x%zx bytes, address: %p)\n"
                    .as_ptr(),
                err,
                err,
                size,
                addr,
            )
        };
    }
    let stats: *mut mi_stats_s = unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) };
    if still_committed {
        _mi_stat_decrease(unsafe { core::ptr::addr_of_mut!((*stats).committed) }, size);
    }
    _mi_stat_decrease(unsafe { core::ptr::addr_of_mut!((*stats).reserved) }, size);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_free_ex(
    addr: *mut (),
    size: u64,
    still_committed: bool,
    memid: mi_memid_s,
    tld_stats: *mut mi_stats_s,
) {
    if mi_memkind_is_os(memid.memkind) {
        let mut csize: u64 = _mi_os_good_alloc_size(size);
        let mut base: *mut () = addr;
        // different base? (due to alignment)
        if !unsafe { memid.mem.os.base }.is_null() {
            base = unsafe { memid.mem.os.base };
            let __v0: u64 = csize;
            csize = __v0.wrapping_add(
                ((unsafe { (addr as *mut u8).offset_from(memid.mem.os.base as *mut u8) }) as i64)
                    as u64,
            );
        }
        // free it
        if (memid.memkind as u32) == 4 {
            mi_os_free_huge_os_pages(base, csize, tld_stats);
        } else {
            mi_os_prim_free(base, csize, still_committed, tld_stats);
        }
    }
    // nothing to do
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_free(
    p: *mut (),
    size: u64,
    memid: mi_memid_s,
    tld_stats: *mut mi_stats_s,
) {
    _mi_os_free_ex(p, size, true, memid, tld_stats);
}

fn mi_os_prim_alloc(
    size: u64,
    mut try_alignment: u64,
    commit: bool,
    mut allow_large: bool,
    is_large: *mut bool,
    is_zero: *mut bool,
    stats: *mut mi_stats_s,
) -> *mut () {
    if size == 0 {
        return core::ptr::null_mut::<()>();
    }
    if !commit {
        allow_large = false;
    }
    if try_alignment == 0 {
        try_alignment = 1;
    }
    // avoid 0 to ensure there will be no divide by zero when aligning
    unsafe {
        *is_zero = false;
    }
    let mut p: *mut () = core::ptr::null_mut::<()>();
    let err: i32 = _mi_prim_alloc(
        size,
        try_alignment,
        commit,
        allow_large,
        is_large,
        is_zero,
        core::ptr::addr_of_mut!(p),
    );
    if err != 0 {
        unsafe {
            _mi_warning_message(c"unable to allocate OS memory (error: %d (0x%x), size: 0x%zx bytes, align: 0x%zx, commit: %d, allow large: %d)\n".as_ptr(), err, err, size, try_alignment, commit as i32, allow_large as i32)
        };
    }
    mi_stat_counter_increase!();
    if !p.is_null() {
        _mi_stat_increase(unsafe { core::ptr::addr_of_mut!((*stats).reserved) }, size);
        if commit {
            _mi_stat_increase(unsafe { core::ptr::addr_of_mut!((*stats).committed) }, size);
            // seems needed for asan (or `mimalloc-test-api` fails)
        }
    }
    p
}

fn mi_os_prim_alloc_aligned(
    mut size: u64,
    alignment: u64,
    commit: bool,
    mut allow_large: bool,
    is_large: *mut bool,
    is_zero: *mut bool,
    base: &mut *mut (),
    stats: *mut mi_stats_s,
) -> *mut () {
    if !commit {
        allow_large = false;
    }
    if !(alignment >= _mi_os_page_size() && alignment & alignment.wrapping_sub(1) == 0) {
        return core::ptr::null_mut::<()>();
    }
    size = _mi_align_up(size, _mi_os_page_size());
    // try first with a hint (this will be aligned directly on Win 10+ or BSD)
    let mut p: *mut () = mi_os_prim_alloc(
        size,
        alignment,
        commit,
        allow_large,
        is_large,
        is_zero,
        stats,
    );
    if p.is_null() {
        return core::ptr::null_mut::<()>();
    }
    // aligned already?
    if (p as u64) % alignment == 0 {
        unsafe {
            *(base as *mut *mut ()) = p;
        }
    } else {
        // if not aligned, free it, overallocate, and unmap around it
        //
        // NOTE(sgross): this warning causes issues in Python tests
        //
        // _mi_warning_message("unable to allocate aligned OS memory directly, fall back to over-allocation (size: 0x%zx bytes, address: %p, alignment: 0x%zx, commit: %d)\n", size, p, alignment, commit);
        mi_os_prim_free(p, size, commit, stats);
        if size >= 18446744073709551615u64.wrapping_sub(alignment) {
            return core::ptr::null_mut::<()>();
        }
        // overflow
        let over_size: u64 = size.wrapping_add(alignment);
        if unsafe { mi_os_mem_config.must_free_whole } {
            // win32 virtualAlloc cannot free parts of an allocate block
            // over-allocate uncommitted (virtual) memory
            p = mi_os_prim_alloc(over_size, 1u64, false, false, is_large, is_zero, stats); // alignment
            // commit?
            // allow_large
            if p.is_null() {
                return core::ptr::null_mut::<()>();
            }
            // set p to the aligned part in the full region
            //
            // note: this is dangerous on Windows as VirtualFree needs the actual base pointer
            //
            // this is handled though by having the `base` field in the memid's
            unsafe {
                *(base as *mut *mut ()) = p;
            }
            // remember the base
            p = mi_align_up_ptr(p, alignment);
            // explicitly commit only the aligned part
            if commit {
                _mi_os_commit(p, size, core::ptr::null_mut::<bool>(), stats);
            }
        } else {
            // mmap can free inside an allocation
            // overallocate...
            p = mi_os_prim_alloc(over_size, 1u64, commit, false, is_large, is_zero, stats);
            if p.is_null() {
                return core::ptr::null_mut::<()>();
            }
            // and selectively unmap parts around the over-allocated area. (noop on sbrk)
            let aligned_p: *mut () = mi_align_up_ptr(p, alignment);
            let pre_size: u64 =
                ((unsafe { (aligned_p as *mut u8).offset_from(p as *mut u8) }) as i64) as u64;
            let mid_size: u64 = _mi_align_up(size, _mi_os_page_size());
            let post_size: u64 = over_size.wrapping_sub(pre_size).wrapping_sub(mid_size);
            if pre_size > 0 {
                mi_os_prim_free(p, pre_size, commit, stats);
            }
            if post_size > 0 {
                mi_os_prim_free(
                    (aligned_p as *mut u8).wrapping_offset(mid_size as isize) as *mut (),
                    post_size,
                    commit,
                    stats,
                );
            }
            // we can return the aligned pointer on `mmap` (and sbrk) systems
            p = aligned_p;
            unsafe {
                *(base as *mut *mut ()) = aligned_p;
            }
            // since we freed the pre part, `*base == p`.
        }
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_alloc(
    mut size: u64,
    memid: *mut mi_memid_s,
    tld_stats: *mut mi_stats_s,
) -> *mut () {
    MI_UNUSED!(tld_stats);
    unsafe {
        *memid = _mi_memid_none();
    }
    let stats: *mut mi_stats_s = unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) };
    if size == 0 {
        return core::ptr::null_mut::<()>();
    }
    size = _mi_os_good_alloc_size(size);
    let mut os_is_large: bool = false;
    let mut os_is_zero: bool = false;
    let p: *mut () = mi_os_prim_alloc(
        size,
        0u64,
        true,
        false,
        core::ptr::addr_of_mut!(os_is_large),
        core::ptr::addr_of_mut!(os_is_zero),
        stats,
    );
    if !p.is_null() {
        unsafe {
            *memid = _mi_memid_create_os(true, os_is_zero, os_is_large);
        }
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_alloc_aligned(
    mut size: u64,
    mut alignment: u64,
    commit: bool,
    allow_large: bool,
    memid: *mut mi_memid_s,
    tld_stats: *mut mi_stats_s,
) -> *mut () {
    MI_UNUSED!(Some(_mi_os_get_aligned_hint)); // suppress unused warnings
    MI_UNUSED!(tld_stats);
    unsafe {
        *memid = _mi_memid_none();
    }
    if size == 0 {
        return core::ptr::null_mut::<()>();
    }
    size = _mi_os_good_alloc_size(size);
    alignment = _mi_align_up(alignment, _mi_os_page_size());
    let mut os_is_large: bool = false;
    let mut os_is_zero: bool = false;
    let mut os_base: *mut () = core::ptr::null_mut::<()>();
    let p: *mut () = mi_os_prim_alloc_aligned(
        size,
        alignment,
        commit,
        allow_large,
        core::ptr::addr_of_mut!(os_is_large),
        core::ptr::addr_of_mut!(os_is_zero),
        unsafe { &mut (*core::ptr::addr_of_mut!(os_base)) },
        unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
    ); // tld->stats
    if !p.is_null() {
        unsafe {
            *memid = _mi_memid_create_os(commit, os_is_zero, os_is_large);
        }
        unsafe {
            (*memid).mem.os.base = os_base;
        }
        unsafe {
            (*memid).mem.os.alignment = alignment;
        }
    }
    p
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_alloc_aligned_at_offset(
    size: u64,
    alignment: u64,
    offset: u64,
    commit: bool,
    allow_large: bool,
    memid: *mut mi_memid_s,
    tld_stats: *mut mi_stats_s,
) -> *mut () {
    unsafe {
        *memid = _mi_memid_none();
    }
    if offset > 33554432u64 {
        return core::ptr::null_mut::<()>();
    }
    if offset == 0 {
        // regular aligned allocation
        return _mi_os_alloc_aligned(size, alignment, commit, allow_large, memid, tld_stats);
    } else {
        // overallocate to align at an offset
        let extra: u64 = _mi_align_up(offset, alignment).wrapping_sub(offset);
        let oversize: u64 = size.wrapping_add(extra);
        let start: *mut () =
            _mi_os_alloc_aligned(oversize, alignment, commit, allow_large, memid, tld_stats);
        if start.is_null() {
            return core::ptr::null_mut::<()>();
        }
        let p: *mut () = (start as *mut u8).wrapping_offset(extra as isize) as *mut ();
        // decommit the overallocation at the start
        let __v0: bool = commit && extra > _mi_os_page_size();
        if __v0 {
            _mi_os_decommit(start, extra, tld_stats);
        }
        return p;
    }
}

fn mi_os_page_align_areax(
    conservative: bool,
    addr: *mut (),
    size: u64,
    newsize: *mut u64,
) -> *mut () {
    if !newsize.is_null() {
        unsafe {
            *newsize = 0;
        }
    }
    if size == 0 || addr.is_null() {
        return core::ptr::null_mut::<()>();
    }
    let __v0: *mut () = if conservative {
        mi_align_up_ptr(addr, _mi_os_page_size())
    } else {
        mi_align_down_ptr(addr, _mi_os_page_size())
    };
    // page align conservatively within the range
    let start: *mut () = __v0;
    let __v1: *mut () = if conservative {
        mi_align_down_ptr(
            (addr as *mut u8).wrapping_offset(size as isize) as *mut (),
            _mi_os_page_size(),
        )
    } else {
        mi_align_up_ptr(
            (addr as *mut u8).wrapping_offset(size as isize) as *mut (),
            _mi_os_page_size(),
        )
    };
    let end: *mut () = __v1;
    let diff: i64 = (unsafe { (end as *mut u8).offset_from(start as *mut u8) }) as i64;
    if diff <= 0 {
        return core::ptr::null_mut::<()>();
    }
    if !newsize.is_null() {
        unsafe {
            *newsize = diff as u64;
        }
    }
    start
}

fn mi_os_page_align_area_conservative(addr: *mut (), size: u64, newsize: *mut u64) -> *mut () {
    mi_os_page_align_areax(true, addr, size, newsize)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_commit(
    addr: *mut (),
    size: u64,
    is_zero: *mut bool,
    tld_stats: *mut mi_stats_s,
) -> bool {
    MI_UNUSED!(tld_stats);
    let stats: *mut mi_stats_s = unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) };
    if !is_zero.is_null() {
        unsafe {
            *is_zero = false;
        }
    }
    _mi_stat_increase(unsafe { core::ptr::addr_of_mut!((*stats).committed) }, size); // use size for precise commit vs. decommit
    _mi_stat_counter_increase(
        unsafe { core::ptr::addr_of_mut!((*stats).commit_calls) },
        1u64,
    );
    // page align range
    let mut csize: u64 = 0;
    let start: *mut () = mi_os_page_align_areax(false, addr, size, core::ptr::addr_of_mut!(csize)); // conservative?
    if csize == 0 {
        return true;
    }
    // commit
    let mut os_is_zero: bool = false;
    let err: i32 = _mi_prim_commit(start, csize, core::ptr::addr_of_mut!(os_is_zero));
    if err != 0 {
        unsafe {
            _mi_warning_message(
                c"cannot commit OS memory (error: %d (0x%x), address: %p, size: 0x%zx bytes)\n"
                    .as_ptr(),
                err,
                err,
                start,
                csize,
            )
        };
        return false;
    }
    if os_is_zero && !is_zero.is_null() {
        unsafe {
            *is_zero = true;
        }
    }
    // note: the following seems required for asan (otherwise `mimalloc-test-stress` fails)
    true
}

fn mi_os_decommit_ex(
    addr: *mut (),
    size: u64,
    needs_recommit: *mut bool,
    tld_stats: *mut mi_stats_s,
) -> bool {
    MI_UNUSED!(tld_stats);
    let stats: *mut mi_stats_s = unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) };
    _mi_stat_decrease(unsafe { core::ptr::addr_of_mut!((*stats).committed) }, size);
    // page align
    let mut csize: u64 = 0;
    let start: *mut () =
        mi_os_page_align_area_conservative(addr, size, core::ptr::addr_of_mut!(csize));
    if csize == 0 {
        return true;
    }
    // decommit
    unsafe {
        *needs_recommit = true;
    }
    let err: i32 = _mi_prim_decommit(start, csize, needs_recommit);
    if err != 0 {
        unsafe {
            _mi_warning_message(
                c"cannot decommit OS memory (error: %d (0x%x), address: %p, size: 0x%zx bytes)\n"
                    .as_ptr(),
                err,
                err,
                start,
                csize,
            )
        };
    }
    err == 0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_decommit(addr: *mut (), size: u64, tld_stats: *mut mi_stats_s) -> bool {
    let mut needs_recommit: bool = false;
    mi_os_decommit_ex(
        addr,
        size,
        core::ptr::addr_of_mut!(needs_recommit),
        tld_stats,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_reset(addr: *mut (), size: u64, stats: *mut mi_stats_s) -> bool {
    // page align conservatively within the range
    let mut csize: u64 = 0;
    let start: *mut () =
        mi_os_page_align_area_conservative(addr, size, core::ptr::addr_of_mut!(csize));
    if csize == 0 {
        return true;
    }
    // || _mi_os_is_huge_reserved(addr)
    _mi_stat_increase(unsafe { core::ptr::addr_of_mut!((*stats).reset) }, csize);
    _mi_stat_counter_increase(
        unsafe { core::ptr::addr_of_mut!((*stats).reset_calls) },
        1u64,
    );
    let err: i32 = _mi_prim_reset(start, csize);
    if err != 0 {
        unsafe {
            _mi_warning_message(
                c"cannot reset OS memory (error: %d (0x%x), address: %p, size: 0x%zx bytes)\n"
                    .as_ptr(),
                err,
                err,
                start,
                csize,
            )
        };
    }
    err == 0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_purge_ex(
    p: *mut (),
    size: u64,
    allow_reset: bool,
    stats: *mut mi_stats_s,
) -> bool {
    if mi_option_get(15u32) < 0 {
        return false;
    }
    // is purging allowed?
    _mi_stat_counter_increase(
        unsafe { core::ptr::addr_of_mut!((*stats).purge_calls) },
        1u64,
    );
    _mi_stat_increase(unsafe { core::ptr::addr_of_mut!((*stats).purged) }, size);
    let __v0: bool = mi_option_is_enabled(5u32) && !_mi_preloading();
    if __v0 {
        let mut needs_recommit: bool = true;
        mi_os_decommit_ex(p, size, core::ptr::addr_of_mut!(needs_recommit), stats);
        return needs_recommit;
    } else {
        if allow_reset {
            // this can sometimes be not allowed if the range is not fully committed
            _mi_os_reset(p, size, stats);
        }
        return false; // needs no recommit
    }
    // should decommit?
    // don't decommit during preloading (unsafe)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_purge(p: *mut (), size: u64, stats: *mut mi_stats_s) -> bool {
    _mi_os_purge_ex(p, size, true, stats)
}

fn mi_os_protectx(addr: *mut (), size: u64, protect: bool) -> bool {
    // page align conservatively within the range
    let mut csize: u64 = 0;
    let start: *mut () =
        mi_os_page_align_area_conservative(addr, size, core::ptr::addr_of_mut!(csize));
    if csize == 0 {
        return false;
    }
    // if (_mi_os_is_huge_reserved(addr)) {
    //         _mi_warning_message("cannot mprotect memory allocated in huge OS pages\n");
    // }
    let err: i32 = _mi_prim_protect(start, csize, protect);
    if err != 0 {
        unsafe {
            _mi_warning_message(
                c"cannot %s OS memory (error: %d (0x%x), address: %p, size: 0x%zx bytes)\n"
                    .as_ptr(),
                if protect {
                    c"protect".as_ptr() as *mut i8
                } else {
                    c"unprotect".as_ptr() as *mut i8
                },
                err,
                err,
                start,
                csize,
            )
        };
    }
    err == 0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_protect(addr: *mut (), size: u64) -> bool {
    mi_os_protectx(addr, size, true)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_unprotect(addr: *mut (), size: u64) -> bool {
    mi_os_protectx(addr, size, false)
}

// Support for allocating huge OS pages (1Gib) that are reserved up-front
// and possibly associated with a specific NUMA node. (use `numa_node>=0`)
static mut mi_huge_start: __SlateAlign64<u64> = __SlateAlign64(unsafe { core::mem::zeroed() });

fn mi_os_claim_huge_pages(pages: u64, total_size: *mut u64) -> *mut u8 {
    if !total_size.is_null() {
        unsafe {
            *total_size = 0;
        }
    }
    let size: u64 = pages.wrapping_mul(1073741824u64);
    let mut start: u64 = 0;
    let mut end: u64 = 0;
    let mut huge_start: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(mi_huge_start.0) });
    loop {
        start = huge_start;
        if start == 0 {
            // Initialize the start address after the 32TiB area
            start = 35184372088832u64; // 32TiB virtual start address
            let heap: *mut mi_heap_s = mi_prim_get_default_heap();
            // gh-123022: default heap may not be initialized in CPython in background threads
            if mi_heap_is_initialized(heap) {
                let r: u64 = _mi_heap_random_next(heap);
                start = start.wrapping_add(1073741824u64.wrapping_mul(r >> 17i32 & 4095)); // (randomly 12bits)*1GiB == between 0 to 4TiB
            }
        }
        end = start.wrapping_add(size);
        if !(!mi_atomic_cas_strong_acq_rel!(
            unsafe { core::ptr::addr_of_mut!(mi_huge_start.0) },
            core::ptr::addr_of_mut!(huge_start),
            end
        )) {
            break;
        }
    }
    if !total_size.is_null() {
        unsafe {
            *total_size = size;
        }
    }
    start as *mut u8
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_alloc_huge_os_pages(
    pages: u64,
    numa_node: i32,
    max_msecs: i64,
    pages_reserved: *mut u64,
    psize: *mut u64,
    memid: *mut mi_memid_s,
) -> *mut () {
    unsafe {
        *memid = _mi_memid_none();
    }
    if !psize.is_null() {
        unsafe {
            *psize = 0;
        }
    }
    if !pages_reserved.is_null() {
        unsafe {
            *pages_reserved = 0;
        }
    }
    let mut size: u64 = 0;
    let start: *mut u8 = mi_os_claim_huge_pages(pages, core::ptr::addr_of_mut!(size));
    if start.is_null() {
        return core::ptr::null_mut::<()>();
    }
    // or 32-bit systems
    // Allocate one page at the time but try to place them contiguously
    //
    // We allocate one page at the time to be able to abort if it takes too long
    //
    // or to at least allocate as many as available on the system.
    let start_t: i64 = _mi_clock_start();
    let mut page: u64 = 0;
    let mut all_zero: bool = true;
    while page < pages {
        // allocate a page
        let mut is_zero: bool = false;
        let addr: *mut () =
            start.wrapping_offset(page.wrapping_mul(1073741824u64) as isize) as *mut ();
        let mut p: *mut () = core::ptr::null_mut::<()>();
        let err: i32 = _mi_prim_alloc_huge_os_pages(
            addr,
            1073741824u64,
            numa_node,
            core::ptr::addr_of_mut!(is_zero),
            core::ptr::addr_of_mut!(p),
        );
        if !is_zero {
            all_zero = false;
        }
        if err != 0 {
            unsafe {
                _mi_warning_message(c"unable to allocate huge OS page (error: %d (0x%x), address: %p, size: %zx bytes)\n".as_ptr(), err, err, addr, 1073741824u64)
            };
            break;
        }
        // Did we succeed at a contiguous address?
        if p != addr {
            // no success, issue a warning and break
            if !p.is_null() {
                unsafe {
                    _mi_warning_message(
                        c"could not allocate contiguous huge OS page %zu at %p\n".as_ptr(),
                        page,
                        addr,
                    )
                };
                mi_os_prim_free(p, 1073741824u64, true, unsafe {
                    core::ptr::addr_of_mut!(_mi_stats_main.0)
                });
            }
            break;
        }
        // success, record it
        page = page.wrapping_add(1); // increase before timeout check (see issue #711)
        _mi_stat_increase(
            unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.committed) },
            1073741824u64,
        );
        _mi_stat_increase(
            unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.reserved) },
            1073741824u64,
        );
        // check for timeout
        if max_msecs > 0 {
            let mut elapsed: i64 = _mi_clock_end(start_t);
            if page >= 1 {
                let estimate: i64 =
                    ((elapsed as u64) / page.wrapping_add(1)).wrapping_mul(pages) as i64;
                if estimate > 2i64.wrapping_mul(max_msecs) {
                    // seems like we are going to timeout, break
                    elapsed = max_msecs.wrapping_add(1);
                }
            }
            if elapsed > max_msecs {
                unsafe {
                    _mi_warning_message(
                        c"huge OS page allocation timed out (after allocating %zu page(s))\n"
                            .as_ptr(),
                        page,
                    )
                };
                break;
            }
        }
    }
    if !pages_reserved.is_null() {
        unsafe {
            *pages_reserved = page;
        }
    }
    if !psize.is_null() {
        unsafe {
            *psize = page.wrapping_mul(1073741824u64);
        }
    }
    if page != 0 {
        unsafe {
            *memid = _mi_memid_create_os(true, all_zero, true);
        }
        // is committed
        // is_large
        unsafe {
            (*memid).memkind = 4;
        }
    }
    (if page == 0 {
        core::ptr::null_mut::<u8>()
    } else {
        start
    }) as *mut ()
}

fn mi_os_free_huge_os_pages(p: *mut (), mut size: u64, stats: *mut mi_stats_s) {
    if p.is_null() || size == 0 {
        return;
    }
    let mut base: *mut u8 = p as *mut u8;
    while size >= 1073741824u64 {
        mi_os_prim_free(base as *mut (), 1073741824u64, true, stats);
        size = size.wrapping_sub(1073741824u64);
        let __v2: *mut u8 = base;
        base = __v2.wrapping_offset(1073741824u64 as isize);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_numa_node_count_get() -> u64 {
    let mut count: u64 =
        mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!(_mi_numa_node_count) });
    if count <= 0 {
        let ncount: i64 = mi_option_get(16u32); // given explicitly?
        if ncount > 0 {
            count = ncount as u64;
        } else {
            count = _mi_prim_numa_node_count(); // or detect dynamically
            if count == 0 {
                count = 1;
            }
        }
        mi_atomic_store_release!(
            unsafe { core::ptr::addr_of_mut!(_mi_numa_node_count) },
            count
        ); // save it
        unsafe { _mi_verbose_message(c"using %zd numa regions\n".as_ptr(), count) };
    }
    count
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_os_numa_node_get(tld: *mut mi_os_tld_s) -> i32 {
    MI_UNUSED!(tld);
    let numa_count: u64 = _mi_os_numa_node_count();
    if numa_count <= 1 {
        return 0;
    }
    // optimize on single numa node systems: always node 0
    // never more than the node count and >= 0
    let mut numa_node: u64 = _mi_prim_numa_node();
    if numa_node >= numa_count {
        numa_node %= numa_count;
    }
    numa_node as i32
}

fn mi_page_queue_is_huge(pq: *const mi_page_queue_s) -> bool {
    (unsafe { (*pq).block_size }) == 131072u64.wrapping_add(core::mem::size_of::<u64>() as u64)
}

fn mi_page_queue_is_full(pq: *const mi_page_queue_s) -> bool {
    (unsafe { (*pq).block_size })
        == 131072u64.wrapping_add(2u64.wrapping_mul(core::mem::size_of::<u64>() as u64))
}

fn mi_page_queue_is_special(pq: *const mi_page_queue_s) -> bool {
    (unsafe { (*pq).block_size }) > 131072u64
}

fn mi_bin(size: u64) -> u8 {
    let mut wsize: u64 = _mi_wsize_from_size(size);
    let mut bin: u8 = 0;
    if wsize <= 1 {
        bin = 1;
    } else {
        if wsize <= 8 {
            bin = (wsize.wrapping_add(1) & 18446744073709551614u64) as u8; // round to double word sizes
        } else {
            if wsize > 16384u64 {
                bin = 73;
            } else {
                wsize = wsize.wrapping_sub(1);
                // find the highest bit
                let b: u8 = mi_bsr(wsize) as u8; // note: wsize != 0
                // and use the top 3 bits to determine the bin (~12.5% worst internal fragmentation).
                //
                // - adjust with 3 because we use do not round the first 8 sizes
                //
                //   which each get an exact bin
                bin = ((b as i32) << 2i32)
                    .wrapping_add(((wsize >> (b as i32).wrapping_sub(2) & 3) as u8) as i32)
                    .wrapping_sub(3) as u8;
            }
        }
    }
    bin
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bin(size: u64) -> u8 {
    mi_bin(size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_bin_size(bin: u8) -> u64 {
    unsafe { _mi_heap_empty.0.pages[(bin as i32) as usize].block_size }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_good_size(size: u64) -> u64 {
    if size <= 131072u64 {
        return _mi_bin_size(mi_bin(size));
    } else {
        return _mi_align_up(size, _mi_os_page_size());
    }
}

fn mi_page_queue_of(page: *const mi_page_s) -> *mut mi_page_queue_s {
    let mut bin: u8 = 0;
    let __v0: u32 = if mi_page_is_in_full(page) {
        74u32
    } else {
        (mi_bin((unsafe { (*page).xblock_size }) as u64) as i32) as u32
    };
    bin = __v0 as u8;
    let heap: *mut mi_heap_s = mi_page_heap(page);
    let pq: *mut mi_page_queue_s = unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
        .wrapping_offset((bin as i32) as isize);
    pq
}

fn mi_heap_page_queue_of(heap: *mut mi_heap_s, page: *const mi_page_s) -> *mut mi_page_queue_s {
    let mut bin: u8 = 0;
    let __v0: u32 = if mi_page_is_in_full(page) {
        74u32
    } else {
        (mi_bin((unsafe { (*page).xblock_size }) as u64) as i32) as u32
    };
    bin = __v0 as u8;
    let pq: *mut mi_page_queue_s = unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
        .wrapping_offset((bin as i32) as isize);
    pq
}

fn mi_heap_queue_first_update(heap: *mut mi_heap_s, pq: *const mi_page_queue_s) {
    let size: u64 = unsafe { (*pq).block_size };
    if size > 128u64.wrapping_mul(core::mem::size_of::<*mut ()>() as u64) {
        return;
    }
    let mut page: *mut mi_page_s = unsafe { (*pq).first };
    if unsafe { (*pq).first }.is_null() {
        page = (unsafe { core::ptr::addr_of!(_mi_page_empty.0) }) as *mut mi_page_s;
    }
    // find index in the right direct page array
    let mut start: u64 = 0;
    let idx: u64 = _mi_wsize_from_size(size);
    let pages_free: *mut *mut mi_page_s =
        unsafe { (*heap).pages_free_direct.as_mut_ptr() as *mut *mut mi_page_s };
    if (unsafe { *pages_free.wrapping_offset(idx as isize) }) == page {
        return;
    }
    // already set
    // find start slot
    if idx <= 1 {
        start = 0;
    } else {
        // find previous size; due to minimal alignment upto 3 previous bins may need to be skipped
        let bin: u8 = mi_bin(size);
        let mut prev: *const mi_page_queue_s = pq.wrapping_offset(-(1 as isize));
        while (bin as i32) == (mi_bin(unsafe { (*prev).block_size }) as i32)
            && prev
                > (unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
                    .wrapping_offset(0 as isize) as *const mi_page_queue_s)
        {
            let __v0: *const mi_page_queue_s = prev;
            prev = __v0.wrapping_offset(-(1 as isize));
        }
        start = 1u64.wrapping_add(_mi_wsize_from_size(unsafe { (*prev).block_size }));
        if start > idx {
            start = idx;
        }
    }
    // set size range to the right page
    let mut sz: u64 = start;
    while sz <= idx {
        unsafe {
            *pages_free.wrapping_offset(sz as isize) = page;
        }
        sz = sz.wrapping_add(1);
    }
}

fn mi_page_queue_remove(queue: *mut mi_page_queue_s, page: *mut mi_page_s) {
    let heap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
    if !unsafe { (*page).prev }.is_null() {
        unsafe {
            (*(*page).prev).next = (*page).next;
        }
    }
    if !unsafe { (*page).next }.is_null() {
        unsafe {
            (*(*page).next).prev = (*page).prev;
        }
    }
    if page == unsafe { (*queue).last } {
        unsafe {
            (*queue).last = (*page).prev;
        }
    }
    if page == unsafe { (*queue).first } {
        unsafe {
            (*queue).first = (*page).next;
        }
        // update first
        mi_heap_queue_first_update(heap, queue as *const mi_page_queue_s);
    }
    let __v1: u64 = unsafe { (*heap).page_count };
    unsafe {
        (*heap).page_count = __v1.wrapping_sub(1);
    }
    unsafe {
        (*page).next = core::ptr::null_mut::<mi_page_s>();
    }
    unsafe {
        (*page).prev = core::ptr::null_mut::<mi_page_s>();
    }
    // mi_atomic_store_ptr_release(mi_atomic_cast(void*, &page->heap), NULL);
    mi_page_set_in_full(page, false);
}

fn mi_page_queue_push(heap: *mut mi_heap_s, queue: *mut mi_page_queue_s, page: *mut mi_page_s) {
    mi_page_set_in_full(page, mi_page_queue_is_full(queue as *const mi_page_queue_s));
    // mi_atomic_store_ptr_release(mi_atomic_cast(void*, &page->heap), heap);
    unsafe {
        (*page).next = (*queue).first;
    }
    unsafe {
        (*page).prev = core::ptr::null_mut::<mi_page_s>();
    }
    if !unsafe { (*queue).first }.is_null() {
        unsafe {
            (*(*queue).first).prev = page;
        }
        unsafe {
            (*queue).first = page;
        }
    } else {
        let __v0: *mut mi_page_s = page;
        unsafe {
            (*queue).last = __v0;
        }
        unsafe {
            (*queue).first = __v0;
        }
    }
    // update direct
    mi_heap_queue_first_update(heap, queue as *const mi_page_queue_s);
    let __v2: u64 = unsafe { (*heap).page_count };
    unsafe {
        (*heap).page_count = __v2.wrapping_add(1);
    }
}

fn mi_page_queue_enqueue_from(
    to: *mut mi_page_queue_s,
    from: *mut mi_page_queue_s,
    page: *mut mi_page_s,
) {
    let heap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
    if !unsafe { (*page).prev }.is_null() {
        unsafe {
            (*(*page).prev).next = (*page).next;
        }
    }
    if !unsafe { (*page).next }.is_null() {
        unsafe {
            (*(*page).next).prev = (*page).prev;
        }
    }
    if page == unsafe { (*from).last } {
        unsafe {
            (*from).last = (*page).prev;
        }
    }
    if page == unsafe { (*from).first } {
        unsafe {
            (*from).first = (*page).next;
        }
        // update first
        mi_heap_queue_first_update(heap, from as *const mi_page_queue_s);
    }
    unsafe {
        (*page).prev = (*to).last;
    }
    unsafe {
        (*page).next = core::ptr::null_mut::<mi_page_s>();
    }
    if !unsafe { (*to).last }.is_null() {
        unsafe {
            (*(*to).last).next = page;
        }
        unsafe {
            (*to).last = page;
        }
    } else {
        unsafe {
            (*to).first = page;
        }
        unsafe {
            (*to).last = page;
        }
        mi_heap_queue_first_update(heap, to as *const mi_page_queue_s);
    }
    mi_page_set_in_full(page, mi_page_queue_is_full(to as *const mi_page_queue_s));
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_queue_append(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    append: *mut mi_page_queue_s,
) -> u64 {
    if unsafe { (*append).first }.is_null() {
        return 0;
    }
    // set append pages to new heap and count
    let mut count: u64 = 0;
    let mut page: *mut mi_page_s = unsafe { (*append).first };
    while !page.is_null() {
        // inline `mi_page_set_heap` to avoid wrong assertion during absorption;
        //
        // in this case it is ok to be delayed freeing since both "to" and "from" heap are still alive.
        mi_atomic_store_release!(
            unsafe { core::ptr::addr_of_mut!((*page).xheap) },
            heap as u64
        );
        // set the flag to delayed free (not overriding NEVER_DELAYED_FREE) which has as a
        //
        // side effect that it spins until any DELAYED_FREEING is finished. This ensures
        //
        // that after appending only the new heap will be used for delayed free operations.
        _mi_page_use_delayed_free(page, 0u32, false);
        count = count.wrapping_add(1);
        page = unsafe { (*page).next };
    }
    if unsafe { (*pq).last }.is_null() {
        // take over afresh
        unsafe {
            (*pq).first = (*append).first;
        }
        unsafe {
            (*pq).last = (*append).last;
        }
        mi_heap_queue_first_update(heap, pq as *const mi_page_queue_s);
    } else {
        // append to end
        unsafe {
            (*(*pq).last).next = (*append).first;
        }
        unsafe {
            (*(*append).first).prev = (*pq).last;
        }
        unsafe {
            (*pq).last = (*append).last;
        }
    }
    count
}

fn mi_page_block_at(
    page: *const mi_page_s,
    page_start: *mut (),
    block_size: u64,
    i: u64,
) -> *mut mi_block_s {
    MI_UNUSED!(page);
    (page_start as *mut u8).wrapping_offset(i.wrapping_mul(block_size) as isize) as *mut mi_block_s
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_use_delayed_free(
    page: *mut mi_page_s,
    delay: u32,
    override_never: bool,
) {
    while !_mi_page_try_use_delayed_free(page, delay, override_never) {
        mi_atomic_yield();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_try_use_delayed_free(
    page: *mut mi_page_s,
    delay: u32,
    override_never: bool,
) -> bool {
    let mut tfreex: u64 = 0;
    let mut old_delay: u32 = 0;
    let mut tfree: u64 = 0;
    let mut yield_count: u64 = 0;
    loop {
        tfree = mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!((*page).xthread_free) }); // note: must acquire as we can break/repeat this loop and not do a CAS;
        tfreex = mi_tf_set_delayed(tfree, delay);
        old_delay = mi_tf_delayed(tfree);
        if (old_delay as u32) == 1 {
            if yield_count >= 4 {
                return false;
            }
            // give up after 4 tries
            yield_count = yield_count.wrapping_add(1);
            mi_atomic_yield(); // delay until outstanding MI_DELAYED_FREEING are done.
        // tfree = mi_tf_set_delayed(tfree, MI_NO_DELAYED_FREE); // will cause CAS to busy fail
        } else {
            if (delay as u32) == (old_delay as u32) {
                break; // avoid atomic operation if already equal
            } else {
                if !override_never && (old_delay as u32) == 3 {
                    break; // leave never-delayed flag set
                }
            }
        }
        let __v3: bool = (old_delay as u32) == 1
            || !mi_atomic_cas_weak_release!(
                unsafe { core::ptr::addr_of_mut!((*page).xthread_free) },
                core::ptr::addr_of_mut!(tfree),
                tfreex
            );
        if !__v3 {
            break;
        }
    }
    return true; // success
}

fn _mi_page_thread_free_collect(page: *mut mi_page_s) {
    let mut head: *mut mi_block_s = unsafe { core::mem::zeroed() };
    let mut tfreex: u64 = 0;
    let mut tfree: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*page).xthread_free) });
    loop {
        head = mi_tf_block(tfree);
        tfreex = mi_tf_set_block(tfree, core::ptr::null_mut::<mi_block_s>());
        if !(!mi_atomic_cas_weak_acq_rel!(
            unsafe { core::ptr::addr_of_mut!((*page).xthread_free) },
            core::ptr::addr_of_mut!(tfree),
            tfreex
        )) {
            break;
        }
    }
    // return if the list is empty
    if head.is_null() {
        return;
    }
    // find the tail -- also to get a proper count (without data races)
    let max_count: u32 = (unsafe { (*page).capacity }) as u32; // cannot collect more than capacity
    let mut count: u32 = 1;
    let mut tail: *mut mi_block_s = head;
    loop {
        let __v4: *mut mi_block_s =
            mi_block_next(page as *const mi_page_s, tail as *const mi_block_s);
        let next: *mut mi_block_s = __v4;
        if !(!__v4.is_null() && count <= max_count) {
            break;
        }
        count = count.wrapping_add(1);
        tail = next;
    }
    // if `count > max_count` there was a memory corruption (possibly infinite list due to double multi-threaded free)
    if count > max_count {
        unsafe {
            _mi_error_message(
                slate_support::EFAULT as i32,
                c"corrupted thread-free list\n".as_ptr(),
            )
        };
        return; // the thread-free items cannot be freed
    }
    // and append the current local free list
    mi_block_set_next(
        page as *const mi_page_s,
        tail,
        (unsafe { (*page).local_free }) as *const mi_block_s,
    );
    unsafe {
        (*page).local_free = head;
    }
    // update counts now
    let __v7: u32 = unsafe { (*page).used };
    unsafe {
        (*page).used = __v7.wrapping_sub(count);
    }
    if (unsafe { (*page).used }) == 0 {
        // The page may have had a QSBR goal set from a previous point when it
        //
        // was all-free. That goal is no longer valid because the page was
        //
        // allocated from and then freed again by other threads.
        _PyMem_mi_page_clear_qsbr(page);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_free_collect(page: *mut mi_page_s, force: bool) {
    // collect the thread free list
    let __v0: bool = force || !mi_page_thread_free(page as *const mi_page_s).is_null();
    if __v0 {
        // quick test to avoid an atomic operation
        _mi_page_thread_free_collect(page);
    }
    // and the local free list
    if !unsafe { (*page).local_free }.is_null() {
        if unsafe { (*page).free }.is_null() {
            // usual case
            unsafe {
                (*page).free = (*page).local_free;
            }
            unsafe {
                (*page).local_free = core::ptr::null_mut::<mi_block_s>();
            }
            unsafe {
                (*page).__slate_bits_1.__set_free_is_zero(0u8);
            }
        } else {
            if force {
                // append -- only on shutdown (force) as this is a linear operation
                let mut tail: *mut mi_block_s = unsafe { (*page).local_free };
                loop {
                    let __v1: *mut mi_block_s =
                        mi_block_next(page as *const mi_page_s, tail as *const mi_block_s);
                    let next: *mut mi_block_s = __v1;
                    if __v1.is_null() {
                        break;
                    }
                    tail = next;
                }
                mi_block_set_next(
                    page as *const mi_page_s,
                    tail,
                    (unsafe { (*page).free }) as *const mi_block_s,
                );
                unsafe {
                    (*page).free = (*page).local_free;
                }
                unsafe {
                    (*page).local_free = core::ptr::null_mut::<mi_block_s>();
                }
                unsafe {
                    (*page).__slate_bits_1.__set_free_is_zero(0u8);
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_reclaim(heap: *mut mi_heap_s, page: *mut mi_page_s) {
    // TODO: push on full queue immediately if it is full?
    let pq: *mut mi_page_queue_s = mi_page_queue(
        heap as *const mi_heap_s,
        mi_page_block_size(page as *const mi_page_s),
    );
    mi_page_queue_push(heap, pq, page);
    _PyMem_mi_page_reclaimed(page);
}

fn mi_page_fresh_alloc(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    block_size: u64,
    page_alignment: u64,
) -> *mut mi_page_s {
    let page: *mut mi_page_s = _mi_segment_page_alloc(
        heap,
        block_size,
        page_alignment,
        unsafe { core::ptr::addr_of_mut!((*(*heap).tld).segments) },
        unsafe { core::ptr::addr_of_mut!((*(*heap).tld).os) },
    );
    if page.is_null() {
        // this may be out-of-memory, or an abandoned page was reclaimed (and in our queue)
        return core::ptr::null_mut::<mi_page_s>();
    }
    // a fresh page was found, initialize it
    let mut full_block_size: u64 = 0;
    let __v0: bool = pq.is_null() || mi_page_queue_is_huge(pq as *const mi_page_queue_s);
    let __v1: u64 = if __v0 {
        mi_page_block_size(page as *const mi_page_s)
    } else {
        block_size
    };
    full_block_size = __v1; // see also: mi_segment_huge_page_alloc
    mi_page_init(heap, page, full_block_size, unsafe { (*heap).tld });
    mi_heap_stat_increase!();
    if !pq.is_null() {
        mi_page_queue_push(heap, pq, page);
    }
    page
}

fn mi_page_fresh(heap: *mut mi_heap_s, pq: *mut mi_page_queue_s) -> *mut mi_page_s {
    let page: *mut mi_page_s = mi_page_fresh_alloc(heap, pq, unsafe { (*pq).block_size }, 0u64);
    if page.is_null() {
        return core::ptr::null_mut::<mi_page_s>();
    }
    page
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_delayed_free_all(heap: *mut mi_heap_s) {
    while !_mi_heap_delayed_free_partial(heap) {
        mi_atomic_yield();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_delayed_free_partial(heap: *mut mi_heap_s) -> bool {
    // take over the list (note: no atomic exchange since it is often NULL)
    let mut block: *mut mi_block_s = mi_atomic_load_ptr_relaxed__3!(unsafe {
        core::ptr::addr_of_mut!((*heap).thread_delayed_free)
    });
    loop {
        let __v0: bool = !block.is_null()
            && !mi_atomic_cas_ptr_weak_acq_rel!(
                unsafe { core::ptr::addr_of_mut!((*heap).thread_delayed_free) },
                core::ptr::addr_of_mut!(block)
            );
        if !__v0 {
            break;
        }
    }
    // nothing
    let mut all_freed: bool = true;
    // and free them all
    while !block.is_null() {
        let next: *mut mi_block_s = mi_block_nextx(
            heap as *const (),
            block as *const mi_block_s,
            (unsafe { (*heap).keys.as_mut_ptr() as *mut u64 }) as *const u64,
        );
        // use internal free instead of regular one to keep stats etc correct
        if !_mi_free_delayed_block(block) {
            // we might already start delayed freeing while another thread has not yet
            //
            // reset the delayed_freeing flag; in that case delay it further by reinserting the current block
            //
            // into the delayed free list
            all_freed = false;
            let mut dfree: *mut mi_block_s = mi_atomic_load_ptr_relaxed__3!(unsafe {
                core::ptr::addr_of_mut!((*heap).thread_delayed_free)
            });
            loop {
                mi_block_set_nextx(
                    heap as *const (),
                    block,
                    dfree as *const mi_block_s,
                    (unsafe { (*heap).keys.as_mut_ptr() as *mut u64 }) as *const u64,
                );
                if !(!mi_atomic_cas_ptr_weak_release!(
                    unsafe { core::ptr::addr_of_mut!((*heap).thread_delayed_free) },
                    core::ptr::addr_of_mut!(dfree),
                    block
                )) {
                    break;
                }
            }
        }
        block = next;
    }
    all_freed
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_unfull(page: *mut mi_page_s) {
    if !mi_page_is_in_full(page as *const mi_page_s) {
        return;
    }
    let heap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
    let pqfull: *mut mi_page_queue_s =
        unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
            .wrapping_offset(74u32 as isize);
    mi_page_set_in_full(page, false); // to get the right queue
    let pq: *mut mi_page_queue_s = mi_heap_page_queue_of(heap, page as *const mi_page_s);
    mi_page_set_in_full(page, true);
    mi_page_queue_enqueue_from(pq, pqfull, page);
}

fn mi_page_to_full(page: *mut mi_page_s, pq: *mut mi_page_queue_s) {
    if mi_page_is_in_full(page as *const mi_page_s) {
        return;
    }
    mi_page_queue_enqueue_from(
        unsafe {
            (*mi_page_heap(page as *const mi_page_s)).pages.as_mut_ptr() as *mut mi_page_queue_s
        }
        .wrapping_offset(74u32 as isize),
        pq,
        page,
    );
    _mi_page_free_collect(page, false); // try to collect right away in case another thread freed just before MI_USE_DELAYED_FREE was set
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_abandon(page: *mut mi_page_s, pq: *mut mi_page_queue_s) {
    let pheap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
    // remove from our page list
    let segments_tld: *mut mi_segments_tld_s =
        unsafe { core::ptr::addr_of_mut!((*(*pheap).tld).segments) };
    mi_page_queue_remove(pq, page);
    // page is no longer associated with our heap
    mi_page_set_heap(page, core::ptr::null_mut::<mi_heap_s>());
    // and abandon it
    _mi_segment_page_abandon(page, segments_tld);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_free(page: *mut mi_page_s, pq: *mut mi_page_queue_s, force: bool) {
    // no more aligned blocks in here
    mi_page_set_has_aligned(page, false);
    let heap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
    // remove from the page list
    //
    // (no need to do _mi_heap_delayed_free first as all blocks are already free)
    let segments_tld: *mut mi_segments_tld_s =
        unsafe { core::ptr::addr_of_mut!((*(*heap).tld).segments) };
    mi_page_queue_remove(pq, page);
    // and free it
    mi_page_set_heap(page, core::ptr::null_mut::<mi_heap_s>());
    _mi_segment_page_free(page, force, segments_tld);
}

// Retire parameters
#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_page_retire(page: *mut mi_page_s) {
    mi_page_set_has_aligned(page, false);
    // any previous QSBR goals are no longer valid because we reused the page
    _PyMem_mi_page_clear_qsbr(page);
    // don't retire too often..
    //
    // (or we end up retiring and re-allocating most of the time)
    //
    // NOTE: refine this more: we should not retire if this
    //
    // is the only page left with free blocks. It is not clear
    //
    // how to check this efficiently though...
    //
    // for now, we don't retire if it is the only page left of this size class.
    let pq: *mut mi_page_queue_s = mi_page_queue_of(page as *const mi_page_s);
    let __v0: bool = ((unsafe { (*page).xblock_size }) as u64) <= 131072u64
        && !mi_page_queue_is_special(pq as *const mi_page_queue_s);
    if __v0 {
        // not too large && not full or huge queue?
        if (unsafe { (*pq).last }) == page && (unsafe { (*pq).first }) == page {
            // the only page in the queue?
            mi_stat_counter_increase!();
            unsafe {
                let __t0 = 1i32.wrapping_add(if ((*page).xblock_size as u64) <= 16384u64 {
                    16i32
                } else {
                    4i32
                }) as u8;
                (*page).__slate_bits_1.__set_retire_expire(__t0);
            }
            let heap: *mut mi_heap_s = mi_page_heap(page as *const mi_page_s);
            let index: u64 = ((unsafe {
                pq.offset_from(
                    ((*heap).pages.as_mut_ptr() as *mut mi_page_queue_s) as *mut mi_page_queue_s,
                )
            }) as i64) as u64;
            if index < unsafe { (*heap).page_retired_min } {
                unsafe {
                    (*heap).page_retired_min = index;
                }
            }
            if index > unsafe { (*heap).page_retired_max } {
                unsafe {
                    (*heap).page_retired_max = index;
                }
            }
            return; // don't free after all
        }
    }
    _PyMem_mi_page_maybe_free(page, pq, false);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_heap_collect_retired(heap: *mut mi_heap_s, force: bool) {
    let mut min: u64 = 74u64;
    let mut max: u64 = 0;
    let mut bin: u64 = unsafe { (*heap).page_retired_min };
    while bin <= unsafe { (*heap).page_retired_max } {
        let pq: *mut mi_page_queue_s =
            unsafe { (*heap).pages.as_mut_ptr() as *mut mi_page_queue_s }
                .wrapping_offset(bin as isize);
        let page: *mut mi_page_s = unsafe { (*pq).first };
        if !page.is_null()
            && ((unsafe { (*page).__slate_bits_1.__get_retire_expire() }) as i32) != 0
        {
            if mi_page_all_free(page as *const mi_page_s) {
                let __v1: u8 = unsafe { (*page).__slate_bits_1.__get_retire_expire() };
                unsafe {
                    (*page)
                        .__slate_bits_1
                        .__set_retire_expire((__v1 as i32).wrapping_sub(1) as u8);
                }
                if force || ((unsafe { (*page).__slate_bits_1.__get_retire_expire() }) as i32) == 0
                {
                    _PyMem_mi_page_maybe_free(page, pq, force);
                } else {
                    // keep retired, update min/max
                    if bin < min {
                        min = bin;
                    }
                    if bin > max {
                        max = bin;
                    }
                }
            } else {
                unsafe {
                    (*page).__slate_bits_1.__set_retire_expire(0u8);
                }
            }
        }
        bin = bin.wrapping_add(1);
    }
    unsafe {
        (*heap).page_retired_min = min;
    }
    unsafe {
        (*heap).page_retired_max = max;
    }
}

// at most 64 slices
fn mi_page_free_list_extend_secure(
    heap: *mut mi_heap_s,
    page: *mut mi_page_s,
    bsize: u64,
    extend: u64,
    stats: *mut mi_stats_s,
) {
    MI_UNUSED!(stats);
    let page_area: *mut () = _mi_page_start(
        _mi_page_segment(page as *const mi_page_s) as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::null_mut::<u64>(),
    ) as *mut ();
    // initialize a randomized free list
    //
    // set up `slice_count` slices to alternate between
    let mut shift: u64 = 6;
    while extend >> shift == 0 {
        shift = shift.wrapping_sub(1);
    }
    let slice_count: u64 = 1u64 << shift;
    let slice_extend: u64 = extend / slice_count;
    let mut blocks: __SlateAlign16<[*mut mi_block_s; 64]> =
        __SlateAlign16([0 as *mut mi_block_s; 64]); // current start of the slice
    let mut counts: __SlateAlign16<[u64; 64]> = __SlateAlign16([0u64; 64]); // available objects in the slice
    for i in 0..slice_count {
        blocks.0[i as usize] = mi_page_block_at(
            page as *const mi_page_s,
            page_area,
            bsize,
            (((unsafe { (*page).capacity }) as i32) as u64)
                .wrapping_add(i.wrapping_mul(slice_extend)),
        );
        counts.0[i as usize] = slice_extend;
    }
    let __v4: *mut u64 =
        (counts.0.as_mut_ptr() as *mut u64).wrapping_offset(slice_count.wrapping_sub(1) as isize);
    let __v5: u64 = unsafe { *__v4 };
    unsafe {
        *__v4 = __v5.wrapping_add(extend % slice_count);
    }
    // final slice holds the modulus too (todo: distribute evenly?)
    // and initialize the free list by randomly threading through them
    //
    // set up first element
    let r: u64 = _mi_heap_random_next(heap);
    let mut current: u64 = r % slice_count;
    let __v7: *mut u64 = (counts.0.as_mut_ptr() as *mut u64).wrapping_offset(current as isize);
    let __v8: u64 = unsafe { *__v7 };
    unsafe {
        *__v7 = __v8.wrapping_sub(1);
    }
    let free_start: *mut mi_block_s = blocks.0[current as usize];
    // and iterate through the rest; use `random_shuffle` for performance
    let mut rnd: u64 = _mi_random_shuffle(r | 1); // ensure not 0
    for i in 1..extend {
        // call random_shuffle only every INTPTR_SIZE rounds
        let round: u64 = i % 8u64;
        if round == 0 {
            rnd = _mi_random_shuffle(rnd);
        }
        // select a random next slice index
        let mut next: u64 = rnd >> 8u64.wrapping_mul(round) & slice_count.wrapping_sub(1);
        while counts.0[next as usize] == 0 {
            // ensure it still has space
            next = next.wrapping_add(1);
            if next == slice_count {
                next = 0;
            }
        }
        // and link the current block to it
        let __v13: *mut u64 = (counts.0.as_mut_ptr() as *mut u64).wrapping_offset(next as isize);
        let __v14: u64 = unsafe { *__v13 };
        unsafe {
            *__v13 = __v14.wrapping_sub(1);
        }
        let block: *mut mi_block_s = blocks.0[current as usize];
        blocks.0[current as usize] =
            (block as *mut u8).wrapping_offset(bsize as isize) as *mut mi_block_s; // bump to the following block
        mi_block_set_next(
            page as *const mi_page_s,
            block,
            blocks.0[next as usize] as *const mi_block_s,
        ); // and set next; note: we may have `current == next`
        current = next;
    }
    // prepend to the free list (usually NULL)
    mi_block_set_next(
        page as *const mi_page_s,
        blocks.0[current as usize],
        (unsafe { (*page).free }) as *const mi_block_s,
    ); // end of the list
    unsafe {
        (*page).free = free_start;
    }
}

#[inline(never)]
fn mi_page_free_list_extend(page: *mut mi_page_s, bsize: u64, extend: u64, stats: *mut mi_stats_s) {
    MI_UNUSED!(stats);
    let page_area: *mut () = _mi_page_start(
        _mi_page_segment(page as *const mi_page_s) as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::null_mut::<u64>(),
    ) as *mut ();
    let start: *mut mi_block_s = mi_page_block_at(
        page as *const mi_page_s,
        page_area,
        bsize,
        (unsafe { (*page).capacity }) as u64,
    );
    // initialize a sequential free list
    let last: *mut mi_block_s = mi_page_block_at(
        page as *const mi_page_s,
        page_area,
        bsize,
        (((unsafe { (*page).capacity }) as i32) as u64)
            .wrapping_add(extend)
            .wrapping_sub(1),
    );
    let mut block: *mut mi_block_s = start;
    while block <= last {
        let next: *mut mi_block_s =
            (block as *mut u8).wrapping_offset(bsize as isize) as *mut mi_block_s;
        mi_block_set_next(page as *const mi_page_s, block, next as *const mi_block_s);
        block = next;
    }
    // prepend to free list (usually `NULL`)
    mi_block_set_next(
        page as *const mi_page_s,
        last,
        (unsafe { (*page).free }) as *const mi_block_s,
    );
    unsafe {
        (*page).free = start;
    }
}

// heuristic, one OS page seems to work well.
fn mi_page_extend_free(heap: *mut mi_heap_s, page: *mut mi_page_s, tld: *mut mi_tld_s) {
    MI_UNUSED!(tld);
    if !unsafe { (*page).free }.is_null() {
        return;
    }
    if ((unsafe { (*page).capacity }) as i32) >= ((unsafe { (*page).reserved }) as i32) {
        return;
    }
    let mut page_size: u64 = 0;
    _mi_page_start(
        _mi_page_segment(page as *const mi_page_s) as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::addr_of_mut!(page_size),
    );
    mi_stat_counter_increase!();
    // calculate the extend count
    let bsize: u64 = if (unsafe { (*page).xblock_size }) < 2147483648u32 {
        (unsafe { (*page).xblock_size }) as u64
    } else {
        page_size
    };
    let mut extend: u64 = ((unsafe { (*page).reserved }) as i32)
        .wrapping_sub((unsafe { (*page).capacity }) as i32) as u64;
    let mut max_extend: u64 = (if bsize >= 4096u64 {
        4u32
    } else {
        4096u32 / (bsize as u32)
    }) as u64;
    if max_extend < 4 {
        max_extend = 4;
    }
    if extend > max_extend {
        // ensure we don't touch memory beyond the page to reduce page commit.
        //
        // the `lean` benchmark tests this. Going from 1 to 8 increases rss by 50%.
        extend = max_extend;
    }
    // and append the extend the free list
    if extend < 2 || true {
        // mi_option_is_enabled(mi_option_secure)) {
        mi_page_free_list_extend(page, bsize, extend, unsafe {
            core::ptr::addr_of_mut!((*tld).stats)
        });
    } else {
        mi_page_free_list_extend_secure(heap, page, bsize, extend, unsafe {
            core::ptr::addr_of_mut!((*tld).stats)
        });
    }
    // enable the new free list
    unsafe {
        (*page).capacity = ((*page).capacity as i32).wrapping_add((extend as u16) as i32) as u16;
    }
    mi_stat_increase!();
}

fn mi_page_init(heap: *mut mi_heap_s, page: *mut mi_page_s, block_size: u64, tld: *mut mi_tld_s) {
    let segment: *mut mi_segment_s = _mi_page_segment(page as *const mi_page_s);
    // set fields
    mi_page_set_heap(page, heap);
    unsafe {
        let __t0 = (*heap).tag;
        (*page).__slate_bits_0.__set_tag(__t0);
    }
    unsafe {
        let __t1 = (*heap).page_use_qsbr as u8;
        (*page).__slate_bits_0.__set_use_qsbr(__t1);
    }
    unsafe {
        (*page).debug_offset = (*heap).debug_offset;
    }
    unsafe {
        (*page).xblock_size = if block_size < 2147483648u64 {
            block_size as u32
        } else {
            2147483648u32
        };
    }
    // initialize before _mi_segment_page_start
    let mut page_size: u64 = 0;
    let page_start: *const () = _mi_segment_page_start(
        segment as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::addr_of_mut!(page_size),
    ) as *const ();
    MI_UNUSED!(page_start);
    unsafe {
        (*page).reserved = (page_size / block_size) as u16;
    }
    unsafe {
        let __t2 = ((*page).__slate_bits_0.__get_is_zero_init() as i32) as u8;
        (*page).__slate_bits_1.__set_free_is_zero(__t2);
    }
    // initialize an initial free list
    mi_page_extend_free(heap, page, tld);
}

fn mi_page_queue_find_free_ex(
    heap: *mut mi_heap_s,
    pq: *mut mi_page_queue_s,
    first_try: bool,
) -> *mut mi_page_s {
    // search through the pages in "next fit" order
    let mut page: *mut mi_page_s = unsafe { (*pq).first };
    while !page.is_null() {
        let next: *mut mi_page_s = unsafe { (*page).next }; // remember next
        // 0. collect freed blocks by us and other threads
        _mi_page_free_collect(page, false);
        // 1. if the page contains free blocks, we are done
        if mi_page_immediate_available(page as *const mi_page_s) {
            break; // pick this one
        }
        // 2. Try to extend
        if ((unsafe { (*page).capacity }) as i32) < ((unsafe { (*page).reserved }) as i32) {
            mi_page_extend_free(heap, page, unsafe { (*heap).tld });
            break;
        }
        // 3. If the page is completely full, move it to the `mi_pages_full`
        //
        // queue so we don't visit long-lived pages too often.
        mi_page_to_full(page, pq);
        page = next;
    }
    // for each page
    mi_heap_stat_counter_increase!();
    if page.is_null() {
        _PyMem_mi_heap_collect_qsbr(heap); // some pages might be safe to free now
        _mi_heap_collect_retired(heap, false); // perhaps make a page available?
        page = mi_page_fresh(heap, pq);
        if page.is_null() && first_try {
            // out-of-memory _or_ an abandoned page with free blocks was reclaimed, try once again
            page = mi_page_queue_find_free_ex(heap, pq, false);
        }
    } else {
        unsafe {
            (*page).__slate_bits_1.__set_retire_expire(0u8);
        }
        _PyMem_mi_page_clear_qsbr(page);
    }
    page
}

fn mi_find_free_page(heap: *mut mi_heap_s, size: u64) -> *mut mi_page_s {
    let pq: *mut mi_page_queue_s = mi_page_queue(heap as *const mi_heap_s, size);
    let page: *mut mi_page_s = unsafe { (*pq).first };
    if !page.is_null() {
        _mi_page_free_collect(page, false);
        if mi_page_immediate_available(page as *const mi_page_s) {
            unsafe {
                (*page).__slate_bits_1.__set_retire_expire(0u8);
            }
            _PyMem_mi_page_clear_qsbr(page);
            return page; // fast path
        }
    }
    mi_page_queue_find_free_ex(heap, pq, true)
}

static mut deferred_free: Option<unsafe extern "C-unwind" fn(bool, u64, *mut ())> = None;

static mut deferred_arg: *mut () = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_deferred_free(heap: *mut mi_heap_s, force: bool) {
    let __v0: *mut mi_tld_s = unsafe { (*heap).tld };
    let __v1: u64 = unsafe { (*__v0).heartbeat };
    unsafe {
        (*__v0).heartbeat = __v1.wrapping_add(1);
    }
    if (unsafe { core::ptr::read_volatile(core::ptr::addr_of!(deferred_free)) }) != None
        && !unsafe { (*(*heap).tld).recurse }
    {
        unsafe {
            (*(*heap).tld).recurse = true;
        }
        unsafe {
            core::ptr::read_volatile(core::ptr::addr_of!(deferred_free)).unwrap()(
                force,
                (*(*heap).tld).heartbeat,
                mi_atomic_load_ptr_relaxed__4!(core::ptr::addr_of_mut!(deferred_arg)),
            )
        };
        unsafe {
            (*(*heap).tld).recurse = false;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_register_deferred_free(
    r#fn: Option<unsafe extern "C-unwind" fn(bool, u64, *mut ())>,
    arg: *mut (),
) {
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(deferred_free), r#fn) };
    mi_atomic_store_ptr_release!(unsafe { core::ptr::addr_of_mut!(deferred_arg) }, arg);
}

fn mi_large_huge_page_alloc(
    heap: *mut mi_heap_s,
    size: u64,
    page_alignment: u64,
) -> *mut mi_page_s {
    let block_size: u64 = _mi_os_good_alloc_size(size);
    let is_huge: bool = block_size > 16777216u64 || page_alignment > 0;
    let pq: *mut mi_page_queue_s = mi_page_queue(
        heap as *const mi_heap_s,
        if is_huge { 2147483648u64 } else { block_size },
    ); // not block_size as that can be low if the page_alignment > 0
    let page: *mut mi_page_s = mi_page_fresh_alloc(heap, pq, block_size, page_alignment);
    if !page.is_null() {
        let bsize: u64 = mi_page_usable_block_size(page as *const mi_page_s); // note: not `mi_page_block_size` to account for padding
        if bsize <= 16777216u64 {
            mi_heap_stat_increase!();
            mi_heap_stat_counter_increase!();
        } else {
            mi_heap_stat_increase!();
            mi_heap_stat_counter_increase!();
        }
    }
    page
}

fn mi_find_page(heap: *mut mi_heap_s, size: u64, huge_alignment: u64) -> *mut mi_page_s {
    // huge allocation?
    let req_size: u64 = size.wrapping_sub(0); // correct for padding_size in case of an overflow on `size`
    if req_size > 131072u64 || huge_alignment > 0 {
        if req_size > 9223372036854775807 {
            // we don't allocate more than PTRDIFF_MAX (see <https://sourceware.org/ml/libc-announce/2019/msg00001.html>)
            unsafe {
                _mi_error_message(
                    slate_support::EOVERFLOW as i32,
                    c"allocation request is too large (%zu bytes)\n".as_ptr(),
                    req_size,
                )
            };
            return core::ptr::null_mut::<mi_page_s>();
        } else {
            _PyMem_mi_heap_collect_qsbr(heap);
            return mi_large_huge_page_alloc(heap, size, huge_alignment);
        }
    } else {
        // otherwise find a page with free blocks in our size segregated queues
        return mi_find_free_page(heap, size);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_malloc_generic(
    mut heap: *mut mi_heap_s,
    size: u64,
    zero: bool,
    huge_alignment: u64,
) -> *mut () {
    // initialize if necessary
    if (!(!(!mi_heap_is_initialized(heap))) as i64) != 0 {
        heap = mi_heap_get_default(); // calls mi_thread_init
        if (!(!(!mi_heap_is_initialized(heap))) as i64) != 0 {
            return core::ptr::null_mut::<()>();
        }
    }
    // call potential deferred free routines
    _mi_deferred_free(heap, false);
    // free delayed frees from other threads (but skip contended ones)
    _mi_heap_delayed_free_partial(heap);
    // find (or allocate) a page of the right size
    let mut page: *mut mi_page_s = mi_find_page(heap, size, huge_alignment);
    if page.is_null() {
        // first time out of memory, try to collect and retry the allocation once more
        mi_heap_collect(heap, true); // force
        page = mi_find_page(heap, size, huge_alignment);
    }
    if page.is_null() {
        // out of memory
        let req_size: u64 = size.wrapping_sub(0); // correct for padding_size in case of an overflow on `size`
        unsafe {
            _mi_error_message(
                slate_support::ENOMEM as i32,
                c"unable to allocate memory (%zu bytes)\n".as_ptr(),
                req_size,
            )
        };
        return core::ptr::null_mut::<()>();
    }
    // and try again, this time succeeding! (i.e. this should never recurse through _mi_page_malloc)
    if zero && (unsafe { (*page).xblock_size }) == 0 {
        // note: we cannot call _mi_page_malloc with zeroing for huge blocks; we zero it afterwards in that case.
        let p: *mut () = _mi_page_malloc(heap, page, size, false);
        _mi_memzero_aligned(p, mi_page_usable_block_size(page as *const mi_page_s));
        return p;
    } else {
        return _mi_page_malloc(heap, page, size, zero);
    }
}

fn rotl(x: u32, shift: u32) -> u32 {
    x << shift | x >> 32u32.wrapping_sub(shift)
}

fn qround(x: *mut u32, a: u64, b: u64, c: u64, d: u64) {
    let __v0: *mut u32 = x.wrapping_offset(a as isize);
    let __v1: u32 = unsafe { *__v0 };
    unsafe {
        *__v0 = __v1.wrapping_add(*x.wrapping_offset(b as isize));
    }
    unsafe {
        *x.wrapping_offset(d as isize) = rotl(
            *x.wrapping_offset(d as isize) ^ *x.wrapping_offset(a as isize),
            16u32,
        );
    }
    let __v3: *mut u32 = x.wrapping_offset(c as isize);
    let __v4: u32 = unsafe { *__v3 };
    unsafe {
        *__v3 = __v4.wrapping_add(*x.wrapping_offset(d as isize));
    }
    unsafe {
        *x.wrapping_offset(b as isize) = rotl(
            *x.wrapping_offset(b as isize) ^ *x.wrapping_offset(c as isize),
            12u32,
        );
    }
    let __v6: *mut u32 = x.wrapping_offset(a as isize);
    let __v7: u32 = unsafe { *__v6 };
    unsafe {
        *__v6 = __v7.wrapping_add(*x.wrapping_offset(b as isize));
    }
    unsafe {
        *x.wrapping_offset(d as isize) = rotl(
            *x.wrapping_offset(d as isize) ^ *x.wrapping_offset(a as isize),
            8u32,
        );
    }
    let __v9: *mut u32 = x.wrapping_offset(c as isize);
    let __v10: u32 = unsafe { *__v9 };
    unsafe {
        *__v9 = __v10.wrapping_add(*x.wrapping_offset(d as isize));
    }
    unsafe {
        *x.wrapping_offset(b as isize) = rotl(
            *x.wrapping_offset(b as isize) ^ *x.wrapping_offset(c as isize),
            7u32,
        );
    }
}

fn chacha_block(ctx: *mut mi_random_cxt_s) {
    // scramble into `x`
    let mut x: __SlateAlign16<[u32; 16]> = __SlateAlign16([0u32; 16]);
    for i in 0u64..16u64 {
        x.0[i as usize] =
            unsafe { *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) };
    }
    {
        let mut i: u64 = 0;
        while i < 20 {
            qround(x.0.as_mut_ptr() as *mut u32, 0u64, 4u64, 8u64, 12u64);
            qround(x.0.as_mut_ptr() as *mut u32, 1u64, 5u64, 9u64, 13u64);
            qround(x.0.as_mut_ptr() as *mut u32, 2u64, 6u64, 10u64, 14u64);
            qround(x.0.as_mut_ptr() as *mut u32, 3u64, 7u64, 11u64, 15u64);
            qround(x.0.as_mut_ptr() as *mut u32, 0u64, 5u64, 10u64, 15u64);
            qround(x.0.as_mut_ptr() as *mut u32, 1u64, 6u64, 11u64, 12u64);
            qround(x.0.as_mut_ptr() as *mut u32, 2u64, 7u64, 8u64, 13u64);
            qround(x.0.as_mut_ptr() as *mut u32, 3u64, 4u64, 9u64, 14u64);
            i = i.wrapping_add(2);
        }
    }
    // add scrambled data to the initial state
    for i in 0u64..16u64 {
        unsafe {
            *((*ctx).output.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) = x.0[i as usize]
                .wrapping_add(*((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(i as isize));
        }
    }
    unsafe {
        (*ctx).output_available = 16;
    }
    // increment the counter for the next round
    let __v9: *mut u32 =
        unsafe { (*ctx).input.as_mut_ptr() as *mut u32 }.wrapping_offset(12 as isize);
    let __v10: u32 = unsafe { *__v9 };
    unsafe {
        *__v9 = __v10.wrapping_add(1);
    }
    if (unsafe { *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(12 as isize) }) == 0 {
        let __v12: *mut u32 =
            unsafe { (*ctx).input.as_mut_ptr() as *mut u32 }.wrapping_offset(13 as isize);
        let __v13: u32 = unsafe { *__v12 };
        unsafe {
            *__v12 = __v13.wrapping_add(1);
        }
        if (unsafe { *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(13 as isize) }) == 0 {
            // and keep increasing into the nonce
            let __v15: *mut u32 =
                unsafe { (*ctx).input.as_mut_ptr() as *mut u32 }.wrapping_offset(14 as isize);
            let __v16: u32 = unsafe { *__v15 };
            unsafe {
                *__v15 = __v16.wrapping_add(1);
            }
        }
    }
}

fn chacha_next32(ctx: *mut mi_random_cxt_s) -> u32 {
    if (unsafe { (*ctx).output_available }) <= 0 {
        chacha_block(ctx);
        unsafe {
            (*ctx).output_available = 16;
        }
        // (assign again to suppress static analysis warning)
    }
    let x: u32 = unsafe {
        *((*ctx).output.as_mut_ptr() as *mut u32)
            .wrapping_offset(16i32.wrapping_sub((*ctx).output_available) as isize)
    };
    unsafe {
        *((*ctx).output.as_mut_ptr() as *mut u32)
            .wrapping_offset(16i32.wrapping_sub((*ctx).output_available) as isize) = 0;
    }
    // reset once the data is handed out
    let __v1: i32 = unsafe { (*ctx).output_available };
    unsafe {
        (*ctx).output_available = __v1.wrapping_sub(1);
    }
    x
}

fn read32(p: *const u8, idx32: u64) -> u32 {
    let i: u64 = 4u64.wrapping_mul(idx32);
    ((unsafe { *p.wrapping_offset(i.wrapping_add(0) as isize) }) as u32)
        | ((unsafe { *p.wrapping_offset(i.wrapping_add(1) as isize) }) as u32) << 8i32
        | ((unsafe { *p.wrapping_offset(i.wrapping_add(2) as isize) }) as u32) << 16i32
        | ((unsafe { *p.wrapping_offset(i.wrapping_add(3) as isize) }) as u32) << 24i32
}

fn chacha_init(ctx: *mut mi_random_cxt_s, key: *const u8, nonce: u64) {
    // since we only use chacha for randomness (and not encryption) we
    //
    // do not _need_ to read 32-bit values as little endian but we do anyways
    //
    // just for being compatible :-)
    unsafe {
        core::ptr::write_bytes(
            ((ctx as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<mi_random_cxt_s>() as u64) as usize) as usize,
        )
    };
    for i in 0u64..4u64 {
        let sigma: *const u8 = c"expand 32-byte k".as_ptr() as *const u8;
        unsafe {
            *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) = read32(sigma, i);
        }
    }
    for i in 0u64..8u64 {
        unsafe {
            *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(i.wrapping_add(4) as isize) =
                read32(key, i);
        }
    }
    unsafe {
        *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(12 as isize) = 0;
    }
    unsafe {
        *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(13 as isize) = 0;
    }
    unsafe {
        *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(14 as isize) = nonce as u32;
    }
    unsafe {
        *((*ctx).input.as_mut_ptr() as *mut u32).wrapping_offset(15 as isize) =
            (nonce >> 32i32) as u32;
    }
}

fn chacha_split(ctx: *mut mi_random_cxt_s, nonce: u64, ctx_new: *mut mi_random_cxt_s) {
    unsafe {
        core::ptr::write_bytes(
            ((ctx_new as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<mi_random_cxt_s>() as u64) as usize) as usize,
        )
    };
    _mi_memcpy(
        (unsafe { (*ctx_new).input.as_mut_ptr() as *mut u32 }) as *mut (),
        (unsafe { (*ctx).input.as_mut_ptr() as *mut u32 }) as *const (),
        core::mem::size_of::<[u32; 16]>() as u64,
    );
    unsafe {
        *((*ctx_new).input.as_mut_ptr() as *mut u32).wrapping_offset(12 as isize) = 0;
    }
    unsafe {
        *((*ctx_new).input.as_mut_ptr() as *mut u32).wrapping_offset(13 as isize) = 0;
    }
    unsafe {
        *((*ctx_new).input.as_mut_ptr() as *mut u32).wrapping_offset(14 as isize) = nonce as u32;
    }
    unsafe {
        *((*ctx_new).input.as_mut_ptr() as *mut u32).wrapping_offset(15 as isize) =
            (nonce >> 32i32) as u32;
    }
    // do not reuse nonces!
    chacha_block(ctx_new);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_random_split(ctx: *mut mi_random_cxt_s, ctx_new: *mut mi_random_cxt_s) {
    chacha_split(ctx, ctx_new as u64, ctx_new); // nonce
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_random_next(ctx: *mut mi_random_cxt_s) -> u64 {
    (chacha_next32(ctx) as u64) << 32i32 | (chacha_next32(ctx) as u64)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._mi_os_random_weak")]
extern "C-unwind" fn _mi_os_random_weak(extra_seed: u64) -> u64 {
    let mut x: u64 = ((unsafe {
        core::mem::transmute::<Option<unsafe extern "C-unwind" fn(u64) -> u64>, usize>(Some(
            _mi_os_random_weak,
        ))
    }) as u64)
        ^ extra_seed; // ASLR makes the address random
    let __v0: i64 = _mi_prim_clock_now();
    x ^= __v0 as u64;
    // and do a few randomization steps
    let max: u64 = ((x ^ x >> 17i32) & 15).wrapping_add(1);
    for i in 0..max {
        x = _mi_random_shuffle(x);
    }
    x
}

fn mi_random_init_ex(ctx: *mut mi_random_cxt_s, use_weak: bool) {
    let mut key: __SlateAlign16<[u8; 32]> = __SlateAlign16({
        let mut __t0: [u8; 32] = unsafe { core::mem::zeroed() };
        __t0[0] = 0u8;
        __t0
    });
    let __v0: bool = use_weak
        || !_mi_prim_random_buf(
            key.0.as_mut_ptr() as *mut (),
            core::mem::size_of::<[u8; 32]>() as u64,
        );
    if __v0 {
        // if we fail to get random data from the OS, we fall back to a
        //
        // weak random source based on the current time
        if !use_weak {
            unsafe { _mi_warning_message(c"unable to use secure randomness\n".as_ptr()) };
        }
        let mut x: u64 = _mi_os_random_weak(0u64);
        for i in 0u64..8u64 {
            // key is eight 32-bit words.
            x = _mi_random_shuffle(x);
            unsafe {
                *(key.0.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) = x as u32;
            }
        }
        unsafe {
            (*ctx).weak = true;
        }
    } else {
        unsafe {
            (*ctx).weak = false;
        }
    }
    chacha_init(ctx, key.0.as_mut_ptr() as *const u8, ctx as u64); // nonce
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_random_init(ctx: *mut mi_random_cxt_s) {
    mi_random_init_ex(ctx, false);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_random_init_weak(ctx: *mut mi_random_cxt_s) {
    mi_random_init_ex(ctx, true);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_random_reinit_if_weak(ctx: *mut mi_random_cxt_s) {
    if unsafe { (*ctx).weak } {
        _mi_random_init(ctx);
    }
}

fn mi_commit_mask_all_set(commit: *const mi_commit_mask_s, cm: &mi_commit_mask_s) -> bool {
    for i in 0..8u64 {
        if (unsafe { *((*commit).mask.as_ptr() as *const u64).wrapping_offset(i as isize) })
            & unsafe {
                *((*(cm as *const mi_commit_mask_s)).mask.as_ptr() as *const u64)
                    .wrapping_offset(i as isize)
            }
            != unsafe {
                *((*(cm as *const mi_commit_mask_s)).mask.as_ptr() as *const u64)
                    .wrapping_offset(i as isize)
            }
        {
            return false;
        }
    }
    true
}

fn mi_commit_mask_any_set(commit: *const mi_commit_mask_s, cm: &mi_commit_mask_s) -> bool {
    for i in 0..8u64 {
        if (unsafe { *((*commit).mask.as_ptr() as *const u64).wrapping_offset(i as isize) })
            & unsafe {
                *((*(cm as *const mi_commit_mask_s)).mask.as_ptr() as *const u64)
                    .wrapping_offset(i as isize)
            }
            != 0
        {
            return true;
        }
    }
    false
}

fn mi_commit_mask_create_intersect(
    commit: *const mi_commit_mask_s,
    cm: &mi_commit_mask_s,
    res: &mut mi_commit_mask_s,
) {
    for i in 0..8u64 {
        unsafe {
            *((*(res as *mut mi_commit_mask_s)).mask.as_mut_ptr() as *mut u64)
                .wrapping_offset(i as isize) = *((*commit).mask.as_ptr() as *const u64)
                .wrapping_offset(i as isize)
                & *((*(cm as *const mi_commit_mask_s)).mask.as_ptr() as *const u64)
                    .wrapping_offset(i as isize);
        }
    }
}

fn mi_commit_mask_clear(res: *mut mi_commit_mask_s, cm: &mi_commit_mask_s) {
    for i in 0..8u64 {
        let __v0: *mut u64 =
            unsafe { (*res).mask.as_mut_ptr() as *mut u64 }.wrapping_offset(i as isize);
        let __v1: u64 = unsafe { *__v0 };
        let __v2: u64 = __v1
            & !unsafe {
                *((*(cm as *const mi_commit_mask_s)).mask.as_ptr() as *const u64)
                    .wrapping_offset(i as isize)
            };
        unsafe {
            *__v0 = __v2;
        }
    }
}

fn mi_commit_mask_set(res: *mut mi_commit_mask_s, cm: &mi_commit_mask_s) {
    for i in 0..8u64 {
        let __v0: *mut u64 =
            unsafe { (*res).mask.as_mut_ptr() as *mut u64 }.wrapping_offset(i as isize);
        let __v1: u64 = unsafe { *__v0 };
        let __v2: u64 = __v1
            | unsafe {
                *((*(cm as *const mi_commit_mask_s)).mask.as_ptr() as *const u64)
                    .wrapping_offset(i as isize)
            };
        unsafe {
            *__v0 = __v2;
        }
    }
}

fn mi_commit_mask_create(bitidx: u64, mut bitcount: u64, cm: &mut mi_commit_mask_s) {
    if bitcount == 512u64 {
        mi_commit_mask_create_full(cm);
    } else {
        if bitcount == 0 {
            mi_commit_mask_create_empty(cm as *mut mi_commit_mask_s);
        } else {
            mi_commit_mask_create_empty(cm as *mut mi_commit_mask_s);
            let mut i: u64 = bitidx / 64u64;
            let mut ofs: u64 = bitidx % 64u64;
            while bitcount > 0 {
                let avail: u64 = 64u64.wrapping_sub(ofs);
                let count: u64 = if bitcount > avail { avail } else { bitcount };
                let mask: u64 = if count >= 64u64 {
                    !0
                } else {
                    (1u64 << count).wrapping_sub(1) << ofs
                };
                unsafe {
                    *((*(cm as *mut mi_commit_mask_s)).mask.as_mut_ptr() as *mut u64)
                        .wrapping_offset(i as isize) = mask;
                }
                bitcount = bitcount.wrapping_sub(count);
                ofs = 0;
                i = i.wrapping_add(1);
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_commit_mask_committed_size(
    cm: *const mi_commit_mask_s,
    total: u64,
) -> u64 {
    let mut count: u64 = 0;
    for i in 0..8u64 {
        let mut mask: u64 =
            unsafe { *((*cm).mask.as_ptr() as *const u64).wrapping_offset(i as isize) };
        if !mask == 0 {
            count = count.wrapping_add(64u64);
        } else {
            while mask != 0 {
                // todo: use popcount
                if mask & 1 != 0 {
                    count = count.wrapping_add(1);
                }
                mask >>= 1i32;
            }
        }
    }
    // we use total since for huge segments each commit bit may represent a larger size
    (total / 512u64).wrapping_mul(count)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_commit_mask_next_run(cm: *const mi_commit_mask_s, idx: *mut u64) -> u64 {
    let mut i: u64 = (unsafe { *idx }) / 64u64;
    let mut ofs: u64 = (unsafe { *idx }) % 64u64;
    let mut mask: u64 = 0;
    // find first ones
    while i < 8u64 {
        mask = unsafe { *((*cm).mask.as_ptr() as *const u64).wrapping_offset(i as isize) };
        mask >>= ofs;
        if mask != 0 {
            while mask & 1 == 0 {
                mask >>= 1i32;
                ofs = ofs.wrapping_add(1);
            }
            break;
        }
        i = i.wrapping_add(1);
        ofs = 0;
    }
    if i >= 8u64 {
        // not found
        unsafe {
            *idx = 512u64;
        }
        return 0;
    } else {
        // found, count ones
        let mut count: u64 = 0;
        unsafe {
            *idx = i.wrapping_mul(64u64).wrapping_add(ofs);
        }
        loop {
            loop {
                count = count.wrapping_add(1);
                mask >>= 1i32;
                if mask & 1 != 1 {
                    break;
                }
            }
            if unsafe { *idx }.wrapping_add(count) % 64u64 == 0 {
                i = i.wrapping_add(1);
                if i >= 8u64 {
                    break;
                }
                mask = unsafe { *((*cm).mask.as_ptr() as *const u64).wrapping_offset(i as isize) };
                ofs = 0;
            }
            if mask & 1 != 1 {
                break;
            }
        }
        return count;
    }
}

fn mi_segment_slices_end(segment: *const mi_segment_s) -> *const mi_page_s {
    unsafe { (*segment).slices.as_ptr() as *const mi_page_s }
        .wrapping_offset((unsafe { (*segment).slice_entries }) as isize)
}

fn mi_slice_start(slice: *const mi_page_s) -> *mut u8 {
    let segment: *mut mi_segment_s = _mi_ptr_segment(slice as *const ());
    (segment as *mut u8).wrapping_offset(
        (((unsafe {
            slice
                .offset_from(((*segment).slices.as_mut_ptr() as *mut mi_page_s) as *const mi_page_s)
        }) as i64) as u64)
            .wrapping_mul(65536u64) as isize,
    )
}

fn mi_slice_bin8(mut slice_count: u64) -> u64 {
    if slice_count <= 1 {
        return slice_count;
    }
    slice_count = slice_count.wrapping_sub(1);
    let s: u64 = mi_bsr(slice_count); // slice_count > 1
    if s <= 2 {
        return slice_count.wrapping_add(1);
    }
    let bin: u64 = (s << 2i32 | slice_count >> s.wrapping_sub(2) & 3).wrapping_sub(4);
    bin
}

fn mi_slice_bin(slice_count: u64) -> u64 {
    let bin: u64 = mi_slice_bin8(slice_count);
    bin
}

fn mi_slice_index(slice: *const mi_page_s) -> u64 {
    let segment: *mut mi_segment_s = _mi_ptr_segment(slice as *const ());
    let index: i64 = (unsafe {
        slice.offset_from(((*segment).slices.as_mut_ptr() as *mut mi_page_s) as *const mi_page_s)
    }) as i64;
    index as u64
}

fn mi_span_queue_push(sq: *mut mi_span_queue_s, slice: *mut mi_page_s) {
    // todo: or push to the end?
    unsafe {
        (*slice).prev = core::ptr::null_mut::<mi_page_s>();
    }
    // paranoia
    unsafe {
        (*slice).next = (*sq).first;
    }
    unsafe {
        (*sq).first = slice;
    }
    if !unsafe { (*slice).next }.is_null() {
        unsafe {
            (*(*slice).next).prev = slice;
        }
    } else {
        unsafe {
            (*sq).last = slice;
        }
    }
    unsafe {
        (*slice).xblock_size = 0;
    }
    // free
}

fn mi_span_queue_for(slice_count: u64, tld: *mut mi_segments_tld_s) -> *mut mi_span_queue_s {
    let bin: u64 = mi_slice_bin(slice_count);
    let sq: *mut mi_span_queue_s =
        unsafe { (*tld).spans.as_mut_ptr() as *mut mi_span_queue_s }.wrapping_offset(bin as isize);
    sq
}

fn mi_span_queue_delete(sq: *mut mi_span_queue_s, slice: *mut mi_page_s) {
    // should work too if the queue does not contain slice (which can happen during reclaim)
    if !unsafe { (*slice).prev }.is_null() {
        unsafe {
            (*(*slice).prev).next = (*slice).next;
        }
    }
    if slice == unsafe { (*sq).first } {
        unsafe {
            (*sq).first = (*slice).next;
        }
    }
    if !unsafe { (*slice).next }.is_null() {
        unsafe {
            (*(*slice).next).prev = (*slice).prev;
        }
    }
    if slice == unsafe { (*sq).last } {
        unsafe {
            (*sq).last = (*slice).prev;
        }
    }
    unsafe {
        (*slice).prev = core::ptr::null_mut::<mi_page_s>();
    }
    unsafe {
        (*slice).next = core::ptr::null_mut::<mi_page_s>();
    }
    unsafe {
        (*slice).xblock_size = 1;
    }
    // no more free
}

fn mi_slice_is_used(slice: *const mi_page_s) -> bool {
    (unsafe { (*slice).xblock_size }) > 0
}

fn mi_segment_info_size(segment: *mut mi_segment_s) -> u64 {
    unsafe { (*segment).segment_info_slices }.wrapping_mul(65536u64)
}

fn _mi_segment_page_start_from_slice(
    segment: *const mi_segment_s,
    slice: *const mi_page_s,
    xblock_size: u64,
    page_size: *mut u64,
) -> *mut u8 {
    let idx: i64 = (unsafe {
        slice.offset_from(((*segment).slices.as_ptr() as *const mi_page_s) as *const mi_page_s)
    }) as i64;
    let psize: u64 = ((unsafe { (*slice).slice_count }) as u64).wrapping_mul(65536u64);
    // make the start not OS page aligned for smaller blocks to avoid page/cache effects
    //
    // note: the offset must always be an xblock_size multiple since we assume small allocations
    //
    // are aligned (see `mi_heap_malloc_aligned`).
    let mut start_offset: u64 = 0;
    if xblock_size >= 8u64 {
        if xblock_size <= 64 {
            start_offset = 3u64.wrapping_mul(xblock_size);
        } else {
            if xblock_size <= 512 {
                start_offset = xblock_size;
            }
        }
    }
    if !page_size.is_null() {
        unsafe {
            *page_size = psize.wrapping_sub(start_offset);
        }
    }
    (segment as *mut u8).wrapping_offset(
        (idx as u64)
            .wrapping_mul(65536u64)
            .wrapping_add(start_offset) as isize,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_page_start(
    segment: *const mi_segment_s,
    page: *const mi_page_s,
    page_size: *mut u64,
) -> *mut u8 {
    let slice: *const mi_page_s = mi_page_to_slice(page as *mut mi_page_s) as *const mi_page_s;
    let p: *mut u8 = _mi_segment_page_start_from_slice(
        segment,
        slice,
        (unsafe { (*page).xblock_size }) as u64,
        page_size,
    );
    p
}

fn mi_segment_calculate_slices(
    mut required: u64,
    pre_size: *mut u64,
    info_slices: *mut u64,
) -> u64 {
    let page_size: u64 = _mi_os_page_size();
    let mut isize: u64 = _mi_align_up(41304u64, page_size);
    let mut guardsize: u64 = 0;
    if false {
        // in secure mode, we set up a protected page in between the segment info
        //
        // and the page data (and one at the end of the segment)
        guardsize = page_size;
        if required > 0 {
            required = _mi_align_up(required, 65536u64).wrapping_add(page_size);
        }
    }
    if !pre_size.is_null() {
        unsafe {
            *pre_size = isize;
        }
    }
    isize = _mi_align_up(isize.wrapping_add(guardsize), 65536u64);
    if !info_slices.is_null() {
        unsafe {
            *info_slices = isize / 65536u64;
        }
    }
    let mut segment_size: u64 = 0;
    let __v0: u64 = if required == 0 {
        33554432u64
    } else {
        _mi_align_up(
            required.wrapping_add(isize).wrapping_add(guardsize),
            65536u64,
        )
    };
    segment_size = __v0;
    segment_size / 65536u64
}

fn mi_segments_track_size(segment_size: i64, tld: *mut mi_segments_tld_s) {
    if segment_size >= 0 {
        _mi_stat_increase(
            unsafe { core::ptr::addr_of_mut!((*(*tld).stats).segments) },
            1u64,
        );
    } else {
        _mi_stat_decrease(
            unsafe { core::ptr::addr_of_mut!((*(*tld).stats).segments) },
            1u64,
        );
    }
    let __v1: u64 = unsafe { (*tld).count };
    unsafe {
        (*tld).count = __v1.wrapping_add(
            (if segment_size >= 0 {
                1i32
            } else {
                1i32.wrapping_neg()
            }) as u64,
        );
    }
    if (unsafe { (*tld).count }) > unsafe { (*tld).peak_count } {
        unsafe {
            (*tld).peak_count = (*tld).count;
        }
    }
    let __v4: u64 = unsafe { (*tld).current_size };
    unsafe {
        (*tld).current_size = __v4.wrapping_add(segment_size as u64);
    }
    if (unsafe { (*tld).current_size }) > unsafe { (*tld).peak_size } {
        unsafe {
            (*tld).peak_size = (*tld).current_size;
        }
    }
}

fn mi_segment_os_free(segment: *mut mi_segment_s, tld: *mut mi_segments_tld_s) {
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(
            core::ptr::addr_of_mut!((*segment).thread_id) as *mut u64
        )
        .store(0u64, core::sync::atomic::Ordering::SeqCst)
    };
    _mi_segment_map_freed_at(segment as *const mi_segment_s);
    mi_segments_track_size((mi_segment_size(segment) as i64).wrapping_neg(), tld);
    if false {
        // _mi_os_unprotect(segment, mi_segment_size(segment)); // ensure no more guard pages are set
        //
        // unprotect the guard pages; we cannot just unprotect the whole segment size as part may be decommitted
        let os_pagesize: u64 = _mi_os_page_size();
        _mi_os_unprotect(
            (segment as *mut u8)
                .wrapping_offset(mi_segment_info_size(segment) as isize)
                .wrapping_offset(-(os_pagesize as isize)) as *mut (),
            os_pagesize,
        );
        let end: *mut u8 = (segment as *mut u8)
            .wrapping_offset(mi_segment_size(segment) as isize)
            .wrapping_offset(-(os_pagesize as isize));
        _mi_os_unprotect(end as *mut (), os_pagesize);
    }
    // purge delayed decommits now? (no, leave it to the arena)
    //
    // mi_segment_try_purge(segment,true,tld->stats);
    let size: u64 = mi_segment_size(segment);
    let csize: u64 = _mi_commit_mask_committed_size(
        (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) }) as *const mi_commit_mask_s,
        size,
    );
    _mi_abandoned_await_readers(unsafe { (*tld).abandoned }); // wait until safe to free
    _mi_arena_free(
        segment as *mut (),
        mi_segment_size(segment),
        csize,
        unsafe { (*segment).memid },
        unsafe { (*tld).stats },
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_thread_collect(tld: *mut mi_segments_tld_s) {
    MI_UNUSED!(tld);
    // nothing to do
}

fn mi_segment_commit_mask(
    segment: *mut mi_segment_s,
    conservative: bool,
    p: *mut u8,
    size: u64,
    start_p: &mut *mut u8,
    full_size: &mut u64,
    cm: &mut mi_commit_mask_s,
) {
    mi_commit_mask_create_empty(cm as *mut mi_commit_mask_s);
    if size == 0 || size > 33554432u64 || ((unsafe { (*segment).kind }) as u32) == 1 {
        return;
    }
    let segstart: u64 = mi_segment_info_size(segment);
    let segsize: u64 = mi_segment_size(segment);
    if p >= (segment as *mut u8).wrapping_offset(segsize as isize) {
        return;
    }
    let pstart: u64 = ((unsafe { p.offset_from(segment as *mut u8) }) as i64) as u64;
    let mut start: u64 = 0;
    let mut end: u64 = if conservative {
        // decommit conservative
        start = _mi_align_up(pstart, 65536u64);
        _mi_align_down(pstart.wrapping_add(size), 65536u64)
    } else {
        // commit liberal
        start = _mi_align_down(pstart, 65536u64);
        _mi_align_up(pstart.wrapping_add(size), 65536u64)
    };
    if pstart >= segstart && start < segstart {
        // note: the mask is also calculated for an initial commit of the info area
        start = segstart;
    }
    if end > segsize {
        end = segsize;
    }
    unsafe {
        *(start_p as *mut *mut u8) = (segment as *mut u8).wrapping_offset(start as isize);
    }
    unsafe {
        *(full_size as *mut u64) = if end > start {
            end.wrapping_sub(start)
        } else {
            0
        };
    }
    if (unsafe { *(full_size as *mut u64) }) == 0 {
        return;
    }
    let bitidx: u64 = start / 65536u64;
    let bitcount: u64 = (unsafe { *(full_size as *mut u64) }) / 65536u64; // can be 0
    if bitidx.wrapping_add(bitcount) > 512u64 {
        unsafe {
            _mi_warning_message(c"commit mask overflow: idx=%zu count=%zu start=%zx end=%zx p=0x%p size=%zu fullsize=%zu\n".as_ptr(), bitidx, bitcount, start, end, p, size, *(full_size as *mut u64))
        };
    }
    mi_commit_mask_create(bitidx, bitcount, cm);
}

fn mi_segment_commit(
    segment: *mut mi_segment_s,
    p: *mut u8,
    size: u64,
    stats: *mut mi_stats_s,
) -> bool {
    // commit liberal
    let mut start: *mut u8 = core::ptr::null_mut::<u8>();
    let mut full_size: u64 = 0;
    let mut mask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
    mi_segment_commit_mask(
        segment,
        false,
        p,
        size,
        unsafe { &mut (*core::ptr::addr_of_mut!(start)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(full_size)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(mask)) },
    ); // conservative?
    if mi_commit_mask_is_empty(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)
        || full_size == 0
    {
        return true;
    }
    if !mi_commit_mask_all_set(
        (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) }) as *const mi_commit_mask_s,
        unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
    ) {
        // committing
        let mut is_zero: bool = false;
        let mut cmask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
        mi_commit_mask_create_intersect(
            (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) }) as *const mi_commit_mask_s,
            unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
            unsafe { &mut (*core::ptr::addr_of_mut!(cmask)) },
        );
        _mi_stat_decrease(
            unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.committed) },
            _mi_commit_mask_committed_size(
                core::ptr::addr_of_mut!(cmask) as *const mi_commit_mask_s,
                33554432u64,
            ),
        ); // adjust for overlap
        if !_mi_os_commit(
            start as *mut (),
            full_size,
            core::ptr::addr_of_mut!(is_zero),
            stats,
        ) {
            return false;
        }
        mi_commit_mask_set(
            unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) },
            unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
        );
    }
    // increase purge expiration when using part of delayed purges -- we assume more allocations are coming soon.
    if mi_commit_mask_any_set(
        (unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) }) as *const mi_commit_mask_s,
        unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
    ) {
        unsafe {
            (*segment).purge_expire = _mi_clock_now().wrapping_add(mi_option_get(15u32));
        }
    }
    // always clear any delayed purges in our range (as they are either committed now)
    mi_commit_mask_clear(
        unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) },
        unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
    );
    true
}

fn mi_segment_ensure_committed(
    segment: *mut mi_segment_s,
    p: *mut u8,
    size: u64,
    stats: *mut mi_stats_s,
) -> bool {
    // note: assumes commit_mask is always full for huge segments as otherwise the commit mask bits can overflow
    let __v0: bool = mi_commit_mask_is_full(
        (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) }) as *const mi_commit_mask_s,
    ) && mi_commit_mask_is_empty(
        (unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) }) as *const mi_commit_mask_s,
    );
    if __v0 {
        return true;
    }
    // fully committed
    mi_segment_commit(segment, p, size, stats)
}

fn mi_segment_purge(
    segment: *mut mi_segment_s,
    p: *mut u8,
    size: u64,
    stats: *mut mi_stats_s,
) -> bool {
    if !unsafe { (*segment).allow_purge } {
        return true;
    }
    // purge conservative
    let mut start: *mut u8 = core::ptr::null_mut::<u8>();
    let mut full_size: u64 = 0;
    let mut mask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
    mi_segment_commit_mask(
        segment,
        true,
        p,
        size,
        unsafe { &mut (*core::ptr::addr_of_mut!(start)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(full_size)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(mask)) },
    ); // conservative?
    if mi_commit_mask_is_empty(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)
        || full_size == 0
    {
        return true;
    }
    if mi_commit_mask_any_set(
        (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) }) as *const mi_commit_mask_s,
        unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
    ) {
        // purging
        let decommitted: bool = _mi_os_purge(start as *mut (), full_size, stats); // reset or decommit
        if decommitted {
            let mut cmask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
            mi_commit_mask_create_intersect(
                (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) })
                    as *const mi_commit_mask_s,
                unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
                unsafe { &mut (*core::ptr::addr_of_mut!(cmask)) },
            );
            _mi_stat_increase(
                unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.committed) },
                full_size.wrapping_sub(_mi_commit_mask_committed_size(
                    core::ptr::addr_of_mut!(cmask) as *const mi_commit_mask_s,
                    33554432u64,
                )),
            ); // adjust for double counting
            mi_commit_mask_clear(
                unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) },
                unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
            );
        }
    }
    // always clear any scheduled purges in our range
    mi_commit_mask_clear(
        unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) },
        unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
    );
    true
}

fn mi_segment_schedule_purge(
    segment: *mut mi_segment_s,
    p: *mut u8,
    size: u64,
    stats: *mut mi_stats_s,
) {
    if !unsafe { (*segment).allow_purge } {
        return;
    }
    if mi_option_get(15u32) == 0 {
        mi_segment_purge(segment, p, size, stats);
    } else {
        // register for future purge in the purge mask
        let mut start: *mut u8 = core::ptr::null_mut::<u8>();
        let mut full_size: u64 = 0;
        let mut mask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
        mi_segment_commit_mask(
            segment,
            true,
            p,
            size,
            unsafe { &mut (*core::ptr::addr_of_mut!(start)) },
            unsafe { &mut (*core::ptr::addr_of_mut!(full_size)) },
            unsafe { &mut (*core::ptr::addr_of_mut!(mask)) },
        ); // conservative
        if mi_commit_mask_is_empty(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)
            || full_size == 0
        {
            return;
        }
        // update delayed commit
        let mut cmask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
        mi_commit_mask_create_intersect(
            (unsafe { core::ptr::addr_of_mut!((*segment).commit_mask) }) as *const mi_commit_mask_s,
            unsafe { &(*(core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s)) },
            unsafe { &mut (*core::ptr::addr_of_mut!(cmask)) },
        ); // only purge what is committed; span_free may try to decommit more
        mi_commit_mask_set(
            unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) },
            unsafe { &(*(core::ptr::addr_of_mut!(cmask) as *const mi_commit_mask_s)) },
        );
        let now: i64 = _mi_clock_now();
        if (unsafe { (*segment).purge_expire }) == 0 {
            // no previous purgess, initialize now
            unsafe {
                (*segment).purge_expire = now.wrapping_add(mi_option_get(15u32));
            }
        } else {
            if (unsafe { (*segment).purge_expire }) <= now {
                // previous purge mask already expired
                if unsafe { (*segment).purge_expire }.wrapping_add(mi_option_get(25u32)) <= now {
                    mi_segment_try_purge(segment, true, stats);
                } else {
                    unsafe {
                        (*segment).purge_expire = now.wrapping_add(mi_option_get(25u32));
                    }
                    // (mi_option_get(mi_option_purge_delay) / 8); // wait a tiny bit longer in case there is a series of free's
                }
            } else {
                // previous purge mask is not yet expired, increase the expiration by a bit.
                let __v0: i64 = mi_option_get(25u32);
                let __v2: i64 = unsafe { (*segment).purge_expire };
                unsafe {
                    (*segment).purge_expire = __v2.wrapping_add(__v0);
                }
            }
        }
    }
}

fn mi_segment_try_purge(segment: *mut mi_segment_s, force: bool, stats: *mut mi_stats_s) {
    let __v0: bool = !unsafe { (*segment).allow_purge }
        || mi_commit_mask_is_empty(
            (unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) }) as *const mi_commit_mask_s,
        );
    if __v0 {
        return;
    }
    let now: i64 = _mi_clock_now();
    if !force && now < unsafe { (*segment).purge_expire } {
        return;
    }
    let mut mask: mi_commit_mask_s = unsafe { (*segment).purge_mask };
    unsafe {
        (*segment).purge_expire = 0;
    }
    mi_commit_mask_create_empty(unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) });
    let mut idx: u64 = 0;
    let mut count: u64 = 0;
    loop {
        let __v3: u64 = _mi_commit_mask_next_run(
            core::ptr::addr_of_mut!(mask) as *const mi_commit_mask_s,
            core::ptr::addr_of_mut!(idx),
        );
        count = __v3;
        if __v3 <= 0 {
            break;
        }
        // if found, decommit that sequence
        if count > 0 {
            let p: *mut u8 =
                (segment as *mut u8).wrapping_offset(idx.wrapping_mul(65536u64) as isize);
            let size: u64 = count.wrapping_mul(65536u64);
            mi_segment_purge(segment, p, size, stats);
        }
        idx = idx.wrapping_add(count);
    }
}

fn mi_segment_is_abandoned(segment: *mut mi_segment_s) -> bool {
    (unsafe {
        core::sync::atomic::AtomicU64::from_ptr(
            core::ptr::addr_of!((*segment).thread_id) as *mut u64
        )
        .load(core::sync::atomic::Ordering::SeqCst)
    }) == 0
}

fn mi_segment_span_free(
    segment: *mut mi_segment_s,
    slice_index: u64,
    mut slice_count: u64,
    allow_purge: bool,
    tld: *mut mi_segments_tld_s,
) {
    let __v0: bool = ((unsafe { (*segment).kind }) as u32) == 1 || mi_segment_is_abandoned(segment);
    let __v1: *mut mi_span_queue_s = if __v0 {
        core::ptr::null_mut::<mi_span_queue_s>()
    } else {
        mi_span_queue_for(slice_count, tld)
    };
    let sq: *mut mi_span_queue_s = __v1;
    if slice_count == 0 {
        slice_count = 1;
    }
    // set first and last slice (the intermediates can be undetermined)
    let slice: *mut mi_page_s = unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s }
        .wrapping_offset(slice_index as isize);
    unsafe {
        (*slice).slice_count = slice_count as u32;
    }
    // no overflow?
    unsafe {
        (*slice).slice_offset = 0;
    }
    if slice_count > 1 {
        let last: *mut mi_page_s = unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s }
            .wrapping_offset(slice_index.wrapping_add(slice_count).wrapping_sub(1) as isize);
        unsafe {
            (*last).slice_count = 0;
        }
        unsafe {
            (*last).slice_offset = 80u64.wrapping_mul(slice_count.wrapping_sub(1)) as u32;
        }
        unsafe {
            (*last).xblock_size = 0;
        }
    }
    // perhaps decommit
    if allow_purge {
        mi_segment_schedule_purge(
            segment,
            mi_slice_start(slice as *const mi_page_s),
            slice_count.wrapping_mul(65536u64),
            unsafe { (*tld).stats },
        );
    }
    // and push it on the free page queue (if it was not a huge page)
    if !sq.is_null() {
        mi_span_queue_push(sq, slice);
    } else {
        unsafe {
            (*slice).xblock_size = 0;
        }
    }
    // mark huge page as free anyways
}

fn mi_segment_span_remove_from_queue(slice: *mut mi_page_s, tld: *mut mi_segments_tld_s) {
    let sq: *mut mi_span_queue_s = mi_span_queue_for((unsafe { (*slice).slice_count }) as u64, tld);
    mi_span_queue_delete(sq, slice);
}

fn mi_segment_span_free_coalesce(
    mut slice: *mut mi_page_s,
    tld: *mut mi_segments_tld_s,
) -> *mut mi_page_s {
    let segment: *mut mi_segment_s = _mi_ptr_segment(slice as *const ());
    let is_abandoned: bool = mi_segment_is_abandoned(segment);
    // for huge pages, just mark as free but don't add to the queues
    if ((unsafe { (*segment).kind }) as u32) == 1 {
        // issue #691: segment->used can be 0 if the huge page block was freed while abandoned (reclaim will get here in that case)
        // decreased right after this call in `mi_segment_page_clear`
        unsafe {
            (*slice).xblock_size = 0;
        }
        // mark as free anyways
        // we should mark the last slice `xblock_size=0` now to maintain invariants but we skip it to
        //
        // avoid a possible cache miss (and the segment is about to be freed)
        return slice;
    }
    // otherwise coalesce the span and add to the free span queues
    let mut slice_count: u64 = (unsafe { (*slice).slice_count }) as u64;
    let next: *mut mi_page_s = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize);
    if next < (mi_segment_slices_end(segment as *const mi_segment_s) as *mut mi_page_s)
        && (unsafe { (*next).xblock_size }) == 0
    {
        // free next block -- remove it from free and merge
        slice_count = slice_count.wrapping_add((unsafe { (*next).slice_count }) as u64); // extend
        if !is_abandoned {
            mi_segment_span_remove_from_queue(next, tld);
        }
    }
    if slice > unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s } {
        let prev: *mut mi_page_s =
            mi_slice_first(slice.wrapping_offset(-(1 as isize)) as *const mi_page_s);
        if (unsafe { (*prev).xblock_size }) == 0 {
            // free previous slice -- remove it from free and merge
            slice_count = slice_count.wrapping_add((unsafe { (*prev).slice_count }) as u64);
            if !is_abandoned {
                mi_segment_span_remove_from_queue(prev, tld);
            }
            slice = prev;
        }
    }
    // and add the new free page
    mi_segment_span_free(
        segment,
        mi_slice_index(slice as *const mi_page_s),
        slice_count,
        true,
        tld,
    );
    slice
}

fn mi_segment_span_allocate(
    segment: *mut mi_segment_s,
    slice_index: u64,
    slice_count: u64,
    tld: *mut mi_segments_tld_s,
) -> *mut mi_page_s {
    let slice: *mut mi_page_s = unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s }
        .wrapping_offset(slice_index as isize);
    // commit before changing the slice data
    if !mi_segment_ensure_committed(
        segment,
        _mi_segment_page_start_from_slice(
            segment as *const mi_segment_s,
            slice as *const mi_page_s,
            0u64,
            core::ptr::null_mut::<u64>(),
        ),
        slice_count.wrapping_mul(65536u64),
        unsafe { (*tld).stats },
    ) {
        return core::ptr::null_mut::<mi_page_s>(); // commit failed!
    }
    // convert the slices to a page
    unsafe {
        (*slice).slice_offset = 0;
    }
    unsafe {
        (*slice).slice_count = slice_count as u32;
    }
    let bsize: u64 = slice_count.wrapping_mul(65536u64);
    unsafe {
        (*slice).xblock_size = (if bsize >= 2147483648u64 {
            2147483648u64
        } else {
            bsize
        }) as u32;
    }
    let page: *mut mi_page_s = mi_slice_to_page(slice);
    // set slice back pointers for the first MI_MAX_SLICE_OFFSET entries
    let mut extra: u64 = slice_count.wrapping_sub(1);
    if extra > 255u64 {
        extra = 255u64;
    }
    if slice_index.wrapping_add(extra) >= unsafe { (*segment).slice_entries } {
        extra = unsafe { (*segment).slice_entries }
            .wrapping_sub(slice_index)
            .wrapping_sub(1);
    }
    // huge objects may have more slices than available entries in the segment->slices
    let mut slice_next: *mut mi_page_s = slice.wrapping_offset(1 as isize);
    let mut i: u64 = 1;
    while i <= extra {
        unsafe {
            (*slice_next).slice_offset = 80u64.wrapping_mul(i) as u32;
        }
        unsafe {
            (*slice_next).slice_count = 0;
        }
        unsafe {
            (*slice_next).xblock_size = 1;
        }
        i = i.wrapping_add(1);
        let __v3: *mut mi_page_s = slice_next;
        slice_next = __v3.wrapping_offset(1 as isize);
    }
    // and also for the last one (if not set already) (the last one is needed for coalescing and for large alignments)
    //
    // note: the cast is needed for ubsan since the index can be larger than MI_SLICES_PER_SEGMENT for huge allocations (see #543)
    let mut last: *mut mi_page_s = slice
        .wrapping_offset(slice_count as isize)
        .wrapping_offset(-(1 as isize));
    let end: *mut mi_page_s =
        mi_segment_slices_end(segment as *const mi_segment_s) as *mut mi_page_s;
    if last > end {
        last = end;
    }
    if last > slice {
        unsafe {
            (*last).slice_offset = 80u64
                .wrapping_mul((last.offset_from(slice as *mut mi_page_s) as i64) as u64)
                as u32;
        }
        unsafe {
            (*last).slice_count = 0;
        }
        unsafe {
            (*last).xblock_size = 1;
        }
    }
    // and initialize the page
    unsafe {
        (*page).__slate_bits_0.__set_is_committed(1u8);
    }
    let __v6: u64 = unsafe { (*segment).used };
    unsafe {
        (*segment).used = __v6.wrapping_add(1);
    }
    page
}

fn mi_segment_slice_split(
    segment: *mut mi_segment_s,
    slice: *mut mi_page_s,
    slice_count: u64,
    tld: *mut mi_segments_tld_s,
) {
    // no more in free queue
    if ((unsafe { (*slice).slice_count }) as u64) <= slice_count {
        return;
    }
    let next_index: u64 = mi_slice_index(slice as *const mi_page_s).wrapping_add(slice_count);
    let next_count: u64 = ((unsafe { (*slice).slice_count }) as u64).wrapping_sub(slice_count);
    mi_segment_span_free(segment, next_index, next_count, false, tld); // don't purge left-over part
    unsafe {
        (*slice).slice_count = slice_count as u32;
    }
}

fn mi_segments_page_find_and_allocate(
    mut slice_count: u64,
    req_arena_id: i32,
    tld: *mut mi_segments_tld_s,
) -> *mut mi_page_s {
    // search from best fit up
    let mut sq: *mut mi_span_queue_s = mi_span_queue_for(slice_count, tld);
    if slice_count == 0 {
        slice_count = 1;
    }
    while sq
        <= unsafe { (*tld).spans.as_mut_ptr() as *mut mi_span_queue_s }.wrapping_offset(35 as isize)
    {
        let mut slice: *mut mi_page_s = unsafe { (*sq).first };
        while !slice.is_null() {
            if ((unsafe { (*slice).slice_count }) as u64) >= slice_count {
                // found one
                let segment: *mut mi_segment_s = _mi_ptr_segment(slice as *const ());
                if _mi_arena_memid_is_suitable(unsafe { (*segment).memid }, req_arena_id) {
                    // found a suitable page span
                    mi_span_queue_delete(sq, slice);
                    if ((unsafe { (*slice).slice_count }) as u64) > slice_count {
                        mi_segment_slice_split(segment, slice, slice_count, tld);
                    }
                    let page: *mut mi_page_s = mi_segment_span_allocate(
                        segment,
                        mi_slice_index(slice as *const mi_page_s),
                        (unsafe { (*slice).slice_count }) as u64,
                        tld,
                    );
                    if page.is_null() {
                        // commit failed; return NULL but first restore the slice
                        mi_segment_span_free_coalesce(slice, tld);
                        return core::ptr::null_mut::<mi_page_s>();
                    }
                    return page;
                }
            }
            slice = unsafe { (*slice).next };
        }
        let __v1: *mut mi_span_queue_s = sq;
        sq = __v1.wrapping_offset(1 as isize);
    }
    // could not find a page..
    core::ptr::null_mut::<mi_page_s>()
}

fn mi_segment_os_alloc(
    required: u64,
    page_alignment: u64,
    eager_delayed: bool,
    req_arena_id: i32,
    psegment_slices: &mut u64,
    ppre_size: *mut u64,
    pinfo_slices: *mut u64,
    commit: bool,
    tld: *mut mi_segments_tld_s,
    os_tld: *mut mi_os_tld_s,
) -> *mut mi_segment_s {
    let mut memid: mi_memid_s = unsafe { core::mem::zeroed() };
    let allow_large: bool = !eager_delayed && true; // only allow large OS pages once we are no longer lazy
    let mut align_offset: u64 = 0;
    let mut alignment: u64 = 33554432u64;
    if page_alignment > 0 {
        // mi_assert_internal(huge_page != NULL);
        alignment = page_alignment;
        let info_size: u64 = unsafe { *pinfo_slices }.wrapping_mul(65536u64);
        align_offset = _mi_align_up(info_size, 33554432u64);
        let extra: u64 = align_offset.wrapping_sub(info_size);
        // recalculate due to potential guard pages
        unsafe {
            *(psegment_slices as *mut u64) =
                mi_segment_calculate_slices(required.wrapping_add(extra), ppre_size, pinfo_slices);
        }
        // mi_page_t.slice_count type is uint32_t
        if (unsafe { *(psegment_slices as *mut u64) }) > 4294967295 {
            return core::ptr::null_mut::<mi_segment_s>();
        }
    }
    let segment_size: u64 = unsafe { *(psegment_slices as *mut u64) }.wrapping_mul(65536u64);
    let segment: *mut mi_segment_s = _mi_arena_alloc_aligned(
        segment_size,
        alignment,
        align_offset,
        commit,
        allow_large,
        req_arena_id,
        core::ptr::addr_of_mut!(memid),
        os_tld,
    ) as *mut mi_segment_s;
    if segment.is_null() {
        return core::ptr::null_mut::<mi_segment_s>(); // failed to allocate
    }
    // ensure metadata part of the segment is committed
    let mut commit_mask: mi_commit_mask_s = unsafe { core::mem::zeroed() };
    if memid.initially_committed {
        mi_commit_mask_create_full(unsafe { &mut (*core::ptr::addr_of_mut!(commit_mask)) });
    } else {
        // at least commit the info slices
        let commit_needed: u64 =
            _mi_divide_up(unsafe { *pinfo_slices }.wrapping_mul(65536u64), 65536u64);
        mi_commit_mask_create(0u64, commit_needed, unsafe {
            &mut (*core::ptr::addr_of_mut!(commit_mask))
        });
        if !_mi_os_commit(
            segment as *mut (),
            commit_needed.wrapping_mul(65536u64),
            core::ptr::null_mut::<bool>(),
            unsafe { (*tld).stats },
        ) {
            _mi_arena_free(segment as *mut (), segment_size, 0u64, memid, unsafe {
                (*tld).stats
            });
            return core::ptr::null_mut::<mi_segment_s>();
        }
    }
    unsafe {
        (*segment).memid = memid;
    }
    unsafe {
        (*segment).allow_decommit = !memid.is_pinned;
    }
    let __v0: bool = (unsafe { (*segment).allow_decommit }) && mi_option_get(15u32) >= 0;
    unsafe {
        (*segment).allow_purge = __v0;
    }
    unsafe {
        (*segment).segment_size = segment_size;
    }
    unsafe {
        (*segment).commit_mask = commit_mask;
    }
    unsafe {
        (*segment).purge_expire = 0;
    }
    mi_commit_mask_create_empty(unsafe { core::ptr::addr_of_mut!((*segment).purge_mask) });
    mi_atomic_store_ptr_release__2!(unsafe { core::ptr::addr_of_mut!((*segment).abandoned_next) }); // tsan
    mi_segments_track_size(segment_size as i64, tld);
    _mi_segment_map_allocated_at(segment as *const mi_segment_s);
    segment
}

fn mi_segment_alloc(
    required: u64,
    page_alignment: u64,
    req_arena_id: i32,
    tld: *mut mi_segments_tld_s,
    os_tld: *mut mi_os_tld_s,
    huge_page: *mut *mut mi_page_s,
) -> *mut mi_segment_s {
    // calculate needed sizes first
    let mut info_slices: u64 = 0;
    let mut pre_size: u64 = 0;
    let mut segment_slices: u64 = mi_segment_calculate_slices(
        required,
        core::ptr::addr_of_mut!(pre_size),
        core::ptr::addr_of_mut!(info_slices),
    );
    // mi_page_t.slice_count type is uint32_t
    if segment_slices > 4294967295 {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    // Commit eagerly only if not the first N lazy segments (to reduce impact of many threads that allocate just a little)
    let mut eager_delay: bool = false;
    eager_delay =
        _mi_current_thread_count() > 1 && (unsafe { (*tld).count }) < (mi_option_get(14u32) as u64); // !_mi_os_has_overcommit() &&             // never delay on overcommit systems
    // do not delay for the first N threads
    let mut eager: bool = false;
    eager = !eager_delay && mi_option_is_enabled(3u32);
    let commit: bool = eager || required > 0;
    // Allocate the segment from the OS
    let segment: *mut mi_segment_s = mi_segment_os_alloc(
        required,
        page_alignment,
        eager_delay,
        req_arena_id,
        unsafe { &mut (*core::ptr::addr_of_mut!(segment_slices)) },
        core::ptr::addr_of_mut!(pre_size),
        core::ptr::addr_of_mut!(info_slices),
        commit,
        tld,
        os_tld,
    );
    if segment.is_null() {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    // zero the segment info? -- not always needed as it may be zero initialized from the OS
    if !unsafe { (*segment).memid.initially_zero } {
        let ofs: i64 = 184;
        let prefix: u64 = 264u64.wrapping_sub(ofs as u64);
        let zsize: u64 = prefix.wrapping_add(80u64.wrapping_mul(segment_slices.wrapping_add(1))); // one more
        _mi_memzero(
            (segment as *mut u8).wrapping_offset(ofs as isize) as *mut (),
            zsize,
        );
    }
    // initialize the rest of the segment info
    let slice_entries: u64 = if segment_slices > 512u64 {
        512u64
    } else {
        segment_slices
    };
    unsafe {
        (*segment).segment_slices = segment_slices;
    }
    unsafe {
        (*segment).segment_info_slices = info_slices;
    }
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(
            core::ptr::addr_of_mut!((*segment).thread_id) as *mut u64
        )
        .store(_mi_thread_id(), core::sync::atomic::Ordering::SeqCst)
    };
    unsafe {
        (*segment).cookie = _mi_ptr_cookie(segment as *const ());
    }
    unsafe {
        (*segment).slice_entries = slice_entries;
    }
    unsafe {
        (*segment).kind = (if required == 0 { 0i32 } else { 1i32 }) as u32;
    }
    // _mi_memzero(segment->slices, sizeof(mi_slice_t)*(info_slices+1));
    _mi_stat_increase(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).page_committed) },
        mi_segment_info_size(segment),
    );
    // set up guard pages
    let mut guard_slices: u64 = 0;
    if false {
        // in secure mode, we set up a protected page in between the segment info
        //
        // and the page data, and at the end of the segment.
        let os_pagesize: u64 = _mi_os_page_size();
        _mi_os_protect(
            (segment as *mut u8)
                .wrapping_offset(mi_segment_info_size(segment) as isize)
                .wrapping_offset(-(os_pagesize as isize)) as *mut (),
            os_pagesize,
        );
        let end: *mut u8 = (segment as *mut u8)
            .wrapping_offset(mi_segment_size(segment) as isize)
            .wrapping_offset(-(os_pagesize as isize));
        mi_segment_ensure_committed(segment, end, os_pagesize, unsafe { (*tld).stats });
        _mi_os_protect(end as *mut (), os_pagesize);
        if slice_entries == segment_slices {
            let __v3: u64 = unsafe { (*segment).slice_entries };
            unsafe {
                (*segment).slice_entries = __v3.wrapping_sub(1);
            }
        }
        // don't use the last slice :-(
        guard_slices = 1;
    }
    // reserve first slices for segment info
    let page0: *mut mi_page_s = mi_segment_span_allocate(segment, 0u64, info_slices, tld);
    if page0.is_null() {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    // cannot fail as we always commit in advance
    unsafe {
        (*segment).used = 0;
    }
    // don't count our internal slices towards usage
    // initialize initial free pages
    if ((unsafe { (*segment).kind }) as u32) == 0 {
        // not a huge page
        mi_segment_span_free(
            segment,
            info_slices,
            unsafe { (*segment).slice_entries }.wrapping_sub(info_slices),
            false,
            tld,
        ); // don't purge
    } else {
        unsafe {
            *huge_page = mi_segment_span_allocate(
                segment,
                info_slices,
                segment_slices
                    .wrapping_sub(info_slices)
                    .wrapping_sub(guard_slices),
                tld,
            );
        }
        // cannot fail as we commit in advance
    }
    segment
}

fn mi_segment_free(segment: *mut mi_segment_s, force: bool, tld: *mut mi_segments_tld_s) {
    MI_UNUSED!(force);
    // Remove the free pages
    let mut slice: *mut mi_page_s =
        unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s }.wrapping_offset(0 as isize);
    let end: *const mi_page_s = mi_segment_slices_end(segment as *const mi_segment_s);
    while slice < (end as *mut mi_page_s) {
        // no more used pages ..
        if (unsafe { (*slice).xblock_size }) == 0 && ((unsafe { (*segment).kind }) as u32) != 1 {
            mi_segment_span_remove_from_queue(slice, tld);
        }
        slice = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize);
    }
    // first page is allocated by the segment itself
    // stats
    _mi_stat_decrease(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).page_committed) },
        mi_segment_info_size(segment),
    );
    // return it to the OS
    mi_segment_os_free(segment, tld);
}

fn mi_segment_page_clear(page: *mut mi_page_s, tld: *mut mi_segments_tld_s) -> *mut mi_page_s {
    let segment: *mut mi_segment_s = _mi_ptr_segment(page as *const ());
    let inuse: u64 = (((unsafe { (*page).capacity }) as i32) as u64)
        .wrapping_mul(mi_page_block_size(page as *const mi_page_s));
    _mi_stat_decrease(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).page_committed) },
        inuse,
    );
    _mi_stat_decrease(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).pages) },
        1u64,
    );
    // reset the page memory to reduce memory pressure?
    let __v0: bool = (unsafe { (*segment).allow_decommit }) && mi_option_is_enabled(11u32);
    if __v0 {
        let mut psize: u64 = 0;
        let start: *mut u8 = _mi_page_start(
            segment as *const mi_segment_s,
            page as *const mi_page_s,
            core::ptr::addr_of_mut!(psize),
        );
        _mi_os_reset(start as *mut (), psize, unsafe { (*tld).stats });
    }
    // zero the page data, but not the segment fields
    unsafe {
        (*page).__slate_bits_0.__set_is_zero_init(0u8);
    }
    let ofs: i64 = 10;
    _mi_memzero(
        (page as *mut u8).wrapping_offset(ofs as isize) as *mut (),
        80u64.wrapping_sub(ofs as u64),
    );
    unsafe {
        (*page).xblock_size = 1;
    }
    // and free it
    let slice: *mut mi_page_s = mi_segment_span_free_coalesce(mi_page_to_slice(page), tld);
    let __v2: u64 = unsafe { (*segment).used };
    unsafe {
        (*segment).used = __v2.wrapping_sub(1);
    }
    // cannot assert segment valid as it is called during reclaim
    //
    // mi_assert_expensive(mi_segment_is_valid(segment, tld));
    slice
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_page_free(
    page: *mut mi_page_s,
    force: bool,
    tld: *mut mi_segments_tld_s,
) {
    let segment: *mut mi_segment_s = _mi_page_segment(page as *const mi_page_s);
    // mark it as free now
    mi_segment_page_clear(page, tld);
    if (unsafe { (*segment).used }) == 0 {
        // no more used pages; remove from the free list and free the segment
        mi_segment_free(segment, force, tld);
    } else {
        if (unsafe { (*segment).used }) == unsafe { (*segment).abandoned } {
            // only abandoned pages; remove from free list and abandon
            mi_segment_abandon(segment, tld);
        }
    }
}

// Use the bottom 20-bits (on 64-bit) of the aligned segment pointers
//
// to put in a tag that increments on update to avoid the A-B-A problem.
fn mi_tagged_segment_ptr(ts: u64) -> *mut mi_segment_s {
    (ts & 18446744073675997184u64) as *mut mi_segment_s
}

fn mi_tagged_segment(segment: *mut mi_segment_s, ts: u64) -> u64 {
    let tag: u64 = (ts & 33554431u64).wrapping_add(1) & 33554431u64;
    (segment as u64) | tag
}

fn mi_abandoned_visited_push(pool: *mut mi_abandoned_pool_s, segment: *mut mi_segment_s) {
    let mut anext: *mut mi_segment_s =
        mi_atomic_load_ptr_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_visited) });
    loop {
        mi_atomic_store_ptr_release__3!(
            unsafe { core::ptr::addr_of_mut!((*segment).abandoned_next) },
            anext
        );
        if !(!mi_atomic_cas_ptr_weak_release__2!(
            unsafe { core::ptr::addr_of_mut!((*pool).abandoned_visited) },
            core::ptr::addr_of_mut!(anext),
            segment
        )) {
            break;
        }
    }
    mi_atomic_increment_relaxed!(unsafe {
        core::ptr::addr_of_mut!((*pool).abandoned_visited_count)
    });
}

fn mi_abandoned_visited_revisit(pool: *mut mi_abandoned_pool_s) -> bool {
    // quick check if the visited list is empty
    if mi_atomic_load_ptr_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_visited) })
        .is_null()
    {
        return false;
    }
    // grab the whole visited list
    let first: *mut mi_segment_s = mi_atomic_exchange_ptr_acq_rel__2!(unsafe {
        core::ptr::addr_of_mut!((*pool).abandoned_visited)
    });
    if first.is_null() {
        return false;
    }
    // first try to swap directly if the abandoned list happens to be NULL
    let mut afirst: u64 = 0;
    let mut ts: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned) });
    if mi_tagged_segment_ptr(ts).is_null() {
        let count: u64 = mi_atomic_load_relaxed!(unsafe {
            core::ptr::addr_of_mut!((*pool).abandoned_visited_count)
        });
        afirst = mi_tagged_segment(first, ts);
        if mi_atomic_cas_strong_acq_rel!(
            unsafe { core::ptr::addr_of_mut!((*pool).abandoned) },
            core::ptr::addr_of_mut!(ts),
            afirst
        ) {
            mi_atomic_add_relaxed!(
                unsafe { core::ptr::addr_of_mut!((*pool).abandoned_count) },
                count
            );
            mi_atomic_sub_relaxed!(
                unsafe { core::ptr::addr_of_mut!((*pool).abandoned_visited_count) },
                count
            );
            return true;
        }
    }
    // find the last element of the visited list: O(n)
    let mut last: *mut mi_segment_s = first;
    loop {
        let __v4: *mut mi_segment_s =
            mi_atomic_load_ptr_relaxed!(unsafe { core::ptr::addr_of_mut!((*last).abandoned_next) });
        let next: *mut mi_segment_s = __v4;
        if __v4.is_null() {
            break;
        }
        last = next;
    }
    // and atomically prepend to the abandoned list
    //
    // (no need to increase the readers as we don't access the abandoned segments)
    let mut anext: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned) });
    let mut count: u64 = 0;
    loop {
        count = mi_atomic_load_relaxed!(unsafe {
            core::ptr::addr_of_mut!((*pool).abandoned_visited_count)
        });
        mi_atomic_store_ptr_release__3!(
            unsafe { core::ptr::addr_of_mut!((*last).abandoned_next) },
            mi_tagged_segment_ptr(anext)
        );
        afirst = mi_tagged_segment(first, anext);
        if !(!mi_atomic_cas_weak_release!(
            unsafe { core::ptr::addr_of_mut!((*pool).abandoned) },
            core::ptr::addr_of_mut!(anext),
            afirst
        )) {
            break;
        }
    }
    mi_atomic_add_relaxed!(
        unsafe { core::ptr::addr_of_mut!((*pool).abandoned_count) },
        count
    );
    mi_atomic_sub_relaxed!(
        unsafe { core::ptr::addr_of_mut!((*pool).abandoned_visited_count) },
        count
    );
    true
}

fn mi_abandoned_push(pool: *mut mi_abandoned_pool_s, segment: *mut mi_segment_s) {
    let mut next: u64 = 0;
    let mut ts: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned) });
    loop {
        mi_atomic_store_ptr_release__3!(
            unsafe { core::ptr::addr_of_mut!((*segment).abandoned_next) },
            mi_tagged_segment_ptr(ts)
        );
        next = mi_tagged_segment(segment, ts);
        if !(!mi_atomic_cas_weak_release!(
            unsafe { core::ptr::addr_of_mut!((*pool).abandoned) },
            core::ptr::addr_of_mut!(ts),
            next
        )) {
            break;
        }
    }
    mi_atomic_increment_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_count) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_abandoned_await_readers(pool: *mut mi_abandoned_pool_s) {
    let mut n: u64 = 0;
    loop {
        n = mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_readers) });
        if n != 0 {
            mi_atomic_yield();
        }
        if n == 0 {
            break;
        }
    }
}

fn mi_abandoned_pop(pool: *mut mi_abandoned_pool_s) -> *mut mi_segment_s {
    let mut segment: *mut mi_segment_s = unsafe { core::mem::zeroed() };
    // Check efficiently if it is empty (or if the visited list needs to be moved)
    let mut ts: u64 =
        mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned) });
    segment = mi_tagged_segment_ptr(ts);
    if segment.is_null() {
        if (!(!(!mi_abandoned_visited_revisit(pool))) as i64) != 0 {
            // try to swap in the visited list on NULL
            return core::ptr::null_mut::<mi_segment_s>();
        }
    }
    // Do a pop. We use a reader count to prevent
    //
    // a segment to be decommitted while a read is still pending,
    //
    // and a tagged pointer to prevent A-B-A link corruption.
    //
    // (this is called from `region.c:_mi_mem_free` for example)
    mi_atomic_increment_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_readers) }); // ensure no segment gets decommitted
    let mut next: u64 = 0;
    ts = mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned) });
    loop {
        segment = mi_tagged_segment_ptr(ts);
        if !segment.is_null() {
            let anext: *mut mi_segment_s = mi_atomic_load_ptr_relaxed!(unsafe {
                core::ptr::addr_of_mut!((*segment).abandoned_next)
            });
            next = mi_tagged_segment(anext, ts); // note: reads the segment's `abandoned_next` field so should not be decommitted
        }
        let __v2: bool = !segment.is_null()
            && !mi_atomic_cas_weak_acq_rel!(
                unsafe { core::ptr::addr_of_mut!((*pool).abandoned) },
                core::ptr::addr_of_mut!(ts),
                next
            );
        if !__v2 {
            break;
        }
    }
    mi_atomic_decrement_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_readers) }); // release reader lock
    if !segment.is_null() {
        mi_atomic_store_ptr_release__2!(unsafe {
            core::ptr::addr_of_mut!((*segment).abandoned_next)
        });
        mi_atomic_decrement_relaxed!(unsafe { core::ptr::addr_of_mut!((*pool).abandoned_count) });
    }
    segment
}

fn mi_segment_abandon(segment: *mut mi_segment_s, tld: *mut mi_segments_tld_s) {
    // remove the free pages from the free page queues
    let mut slice: *mut mi_page_s =
        unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s }.wrapping_offset(0 as isize);
    let end: *const mi_page_s = mi_segment_slices_end(segment as *const mi_segment_s);
    while slice < (end as *mut mi_page_s) {
        if (unsafe { (*slice).xblock_size }) == 0 {
            // a free page
            mi_segment_span_remove_from_queue(slice, tld);
            unsafe {
                (*slice).xblock_size = 0;
            }
            // but keep it free
        }
        slice = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize);
    }
    // perform delayed decommits (forcing is much slower on mstress)
    mi_segment_try_purge(segment, mi_option_is_enabled(12u32), unsafe {
        (*tld).stats
    }); // force?
    // all pages in the segment are abandoned; add it to the abandoned list
    _mi_stat_increase(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).segments_abandoned) },
        1u64,
    );
    mi_segments_track_size((mi_segment_size(segment) as i64).wrapping_neg(), tld);
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(
            core::ptr::addr_of_mut!((*segment).thread_id) as *mut u64
        )
        .store(0u64, core::sync::atomic::Ordering::SeqCst)
    };
    mi_atomic_store_ptr_release__2!(unsafe { core::ptr::addr_of_mut!((*segment).abandoned_next) });
    unsafe {
        (*segment).abandoned_visits = 1;
    }
    // from 0 to 1 to signify it is abandoned
    mi_abandoned_push(unsafe { (*tld).abandoned }, segment);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_page_abandon(page: *mut mi_page_s, tld: *mut mi_segments_tld_s) {
    let segment: *mut mi_segment_s = _mi_page_segment(page as *const mi_page_s);
    let __v1: u64 = unsafe { (*segment).abandoned };
    unsafe {
        (*segment).abandoned = __v1.wrapping_add(1);
    }
    _mi_stat_increase(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).pages_abandoned) },
        1u64,
    );
    if (unsafe { (*segment).used }) == unsafe { (*segment).abandoned } {
        // all pages are abandoned, abandon the entire segment
        mi_segment_abandon(segment, tld);
    }
}

fn mi_slices_start_iterate(
    segment: *mut mi_segment_s,
    end: &mut *const mi_page_s,
) -> *mut mi_page_s {
    let mut slice: *mut mi_page_s =
        unsafe { (*segment).slices.as_mut_ptr() as *mut mi_page_s }.wrapping_offset(0 as isize);
    unsafe {
        *(end as *mut *const mi_page_s) = mi_segment_slices_end(segment as *const mi_segment_s);
    }
    // segment allocated page
    slice = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize); // skip the first segment allocated page
    slice
}

fn mi_segment_check_free(
    segment: *mut mi_segment_s,
    slices_needed: u64,
    block_size: u64,
    tld: *mut mi_segments_tld_s,
) -> bool {
    let mut has_page: bool = false;
    // for all slices
    let mut end: *const mi_page_s = unsafe { core::mem::zeroed() };
    let mut slice: *mut mi_page_s =
        mi_slices_start_iterate(segment, unsafe { &mut (*core::ptr::addr_of_mut!(end)) });
    while slice < (end as *mut mi_page_s) {
        if mi_slice_is_used(slice as *const mi_page_s) {
            // used page
            // ensure used count is up to date and collect potential concurrent frees
            let page: *mut mi_page_s = mi_slice_to_page(slice);
            _mi_page_free_collect(page, false);
            let __v0: bool =
                mi_page_all_free(page as *const mi_page_s) && _PyMem_mi_page_is_safe_to_free(page);
            if __v0 {
                // if this page is all free now, free it without adding to any queues (yet)
                _mi_stat_decrease(
                    unsafe { core::ptr::addr_of_mut!((*(*tld).stats).pages_abandoned) },
                    1u64,
                );
                let __v2: u64 = unsafe { (*segment).abandoned };
                unsafe {
                    (*segment).abandoned = __v2.wrapping_sub(1);
                }
                slice = mi_segment_page_clear(page, tld); // re-assign slice due to coalesce!
                if ((unsafe { (*slice).slice_count }) as u64) >= slices_needed {
                    has_page = true;
                }
            } else {
                let __v4: bool = ((unsafe { (*page).xblock_size }) as u64) == block_size
                    && mi_page_has_any_available(page as *const mi_page_s);
                if __v4 {
                    // a page has available free blocks of the right size
                    has_page = true;
                }
            }
        } else {
            // empty span
            if ((unsafe { (*slice).slice_count }) as u64) >= slices_needed {
                has_page = true;
            }
        }
        slice = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize);
    }
    has_page
}

extern "C-unwind" fn __slate_init__mi_process_init() {
    _mi_process_init();
}

#[allow(non_upper_case_globals)]
#[used(compiler)]
#[unsafe(link_section = ".init_array")]
static __SLATE_INIT__mi_process_init: Option<unsafe extern "C-unwind" fn()> =
    Some(__slate_init__mi_process_init);

fn mi_heap_by_tag(heap: *mut mi_heap_s, tag: u8) -> *mut mi_heap_s {
    if ((unsafe { (*heap).tag }) as i32) == (tag as i32) {
        return heap;
    }
    let mut curr: *mut mi_heap_s = unsafe { (*(*heap).tld).heaps };
    while !curr.is_null() {
        if ((unsafe { (*curr).tag }) as i32) == (tag as i32) {
            return curr;
        }
        curr = unsafe { (*curr).next };
    }
    core::ptr::null_mut::<mi_heap_s>()
}

fn mi_segment_reclaim(
    segment: *mut mi_segment_s,
    heap: *mut mi_heap_s,
    requested_block_size: u64,
    right_page_reclaimed: *mut bool,
    tld: *mut mi_segments_tld_s,
) -> *mut mi_segment_s {
    if !right_page_reclaimed.is_null() {
        unsafe {
            *right_page_reclaimed = false;
        }
    }
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(
            core::ptr::addr_of_mut!((*segment).thread_id) as *mut u64
        )
        .store(_mi_thread_id(), core::sync::atomic::Ordering::SeqCst)
    };
    unsafe {
        (*segment).abandoned_visits = 0;
    }
    mi_segments_track_size(mi_segment_size(segment) as i64, tld);
    _mi_stat_decrease(
        unsafe { core::ptr::addr_of_mut!((*(*tld).stats).segments_abandoned) },
        1u64,
    );
    // for all slices
    let mut end: *const mi_page_s = unsafe { core::mem::zeroed() };
    let mut slice: *mut mi_page_s =
        mi_slices_start_iterate(segment, unsafe { &mut (*core::ptr::addr_of_mut!(end)) });
    while slice < (end as *mut mi_page_s) {
        if mi_slice_is_used(slice as *const mi_page_s) {
            // in use: reclaim the page in our heap
            let page: *mut mi_page_s = mi_slice_to_page(slice);
            let target_heap: *mut mi_heap_s = mi_heap_by_tag(
                heap,
                ((unsafe { (*page).__slate_bits_0.__get_tag() }) as i32) as u8,
            );
            _mi_stat_decrease(
                unsafe { core::ptr::addr_of_mut!((*(*tld).stats).pages_abandoned) },
                1u64,
            );
            let __v1: u64 = unsafe { (*segment).abandoned };
            unsafe {
                (*segment).abandoned = __v1.wrapping_sub(1);
            }
            // set the heap again and allow delayed free again
            mi_page_set_heap(page, target_heap);
            _mi_page_use_delayed_free(page, 0u32, true); // override never (after heap is set)
            _mi_page_free_collect(page, false); // ensure used count is up to date
            let __v3: bool =
                mi_page_all_free(page as *const mi_page_s) && _PyMem_mi_page_is_safe_to_free(page);
            if __v3 {
                // if everything free by now, free the page
                slice = mi_segment_page_clear(page, tld); // set slice again due to coalesceing
            } else {
                // otherwise reclaim it into the heap
                _mi_page_reclaim(target_heap, page);
                let __v4: bool = requested_block_size == ((unsafe { (*page).xblock_size }) as u64)
                    && mi_page_has_any_available(page as *const mi_page_s);
                if __v4 && requested_block_size <= 131072u64 && heap == target_heap {
                    if !right_page_reclaimed.is_null() {
                        unsafe {
                            *right_page_reclaimed = true;
                        }
                    }
                }
            }
        } else {
            // the span is free, add it to our page queues
            slice = mi_segment_span_free_coalesce(slice, tld); // set slice again due to coalesceing
        }
        slice = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize);
    }
    if (unsafe { (*segment).used }) == 0 {
        // due to page_clear
        mi_segment_free(segment, false, tld);
        return core::ptr::null_mut::<mi_segment_s>();
    } else {
        return segment;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_abandoned_reclaim_all(heap: *mut mi_heap_s, tld: *mut mi_segments_tld_s) {
    loop {
        let __v0: *mut mi_segment_s = mi_abandoned_pop(unsafe { (*tld).abandoned });
        let segment: *mut mi_segment_s = __v0;
        if __v0.is_null() {
            break;
        }
        mi_segment_reclaim(segment, heap, 0u64, core::ptr::null_mut::<bool>(), tld);
    }
}

fn mi_segment_try_reclaim(
    heap: *mut mi_heap_s,
    needed_slices: u64,
    block_size: u64,
    reclaimed: *mut bool,
    tld: *mut mi_segments_tld_s,
) -> *mut mi_segment_s {
    unsafe {
        *reclaimed = false;
    }
    let mut segment: *mut mi_segment_s = unsafe { core::mem::zeroed() };
    let mut max_tries: i64 = mi_option_get_clamp(21u32, 8i64, 1024i64); // limit the work to bound allocation times
    loop {
        let __v3: i64 = max_tries;
        max_tries = __v3.wrapping_sub(1);
        let __v5: bool = if __v3 > 0 {
            let __v6: *mut mi_segment_s = mi_abandoned_pop(unsafe { (*tld).abandoned });
            segment = __v6;
            !__v6.is_null()
        } else {
            false
        };
        if !__v5 {
            break;
        }
        let __v1: u64 = unsafe { (*segment).abandoned_visits };
        unsafe {
            (*segment).abandoned_visits = __v1.wrapping_add(1);
        }
        // todo: an arena exclusive heap will potentially visit many abandoned unsuitable segments
        //
        // and push them into the visited list and use many tries. Perhaps we can skip non-suitable ones in a better way?
        let is_suitable: bool = _mi_heap_memid_is_suitable(heap, unsafe { (*segment).memid });
        let has_page: bool = mi_segment_check_free(segment, needed_slices, block_size, tld); // try to free up pages (due to concurrent frees)
        if (unsafe { (*segment).used }) == 0 {
            // free the segment (by forced reclaim) to make it available to other threads.
            //
            // note1: we prefer to free a segment as that might lead to reclaiming another
            //
            // segment that is still partially used.
            //
            // note2: we could in principle optimize this by skipping reclaim and directly
            //
            // freeing but that would violate some invariants temporarily)
            mi_segment_reclaim(segment, heap, 0u64, core::ptr::null_mut::<bool>(), tld);
        } else {
            if has_page && is_suitable {
                // found a large enough free span, or a page of the right block_size with free space
                //
                // we return the result of reclaim (which is usually `segment`) as it might free
                //
                // the segment due to concurrent frees (in which case `NULL` is returned).
                return mi_segment_reclaim(segment, heap, block_size, reclaimed, tld);
            } else {
                if (unsafe { (*segment).abandoned_visits }) > 3 && is_suitable {
                    // always reclaim on 3rd visit to limit the abandoned queue length.
                    mi_segment_reclaim(segment, heap, 0u64, core::ptr::null_mut::<bool>(), tld);
                } else {
                    // otherwise, push on the visited list so it gets not looked at too quickly again
                    mi_segment_try_purge(segment, true, unsafe { (*tld).stats }); // force?
                    // force purge if needed as we may not visit soon again
                    mi_abandoned_visited_push(unsafe { (*tld).abandoned }, segment);
                }
            }
        }
    }
    core::ptr::null_mut::<mi_segment_s>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_abandoned_collect(
    heap: *mut mi_heap_s,
    force: bool,
    tld: *mut mi_segments_tld_s,
) {
    let mut segment: *mut mi_segment_s = unsafe { core::mem::zeroed() };
    let pool: *mut mi_abandoned_pool_s = unsafe { (*tld).abandoned };
    let mut max_tries: i32 = if force { 16384i32 } else { 1024 }; // limit latency
    if force {
        mi_abandoned_visited_revisit(pool);
    }
    loop {
        let __v0: i32 = max_tries;
        max_tries = __v0.wrapping_sub(1);
        let __v2: bool = if __v0 > 0 {
            let __v3: *mut mi_segment_s = mi_abandoned_pop(pool);
            segment = __v3;
            !__v3.is_null()
        } else {
            false
        };
        if !__v2 {
            break;
        }
        mi_segment_check_free(segment, 0u64, 0u64, tld); // try to free up pages (due to concurrent frees)
        if (unsafe { (*segment).used }) == 0 {
            // free the segment (by forced reclaim) to make it available to other threads.
            //
            // note: we could in principle optimize this by skipping reclaim and directly
            //
            // freeing but that would violate some invariants temporarily)
            mi_segment_reclaim(segment, heap, 0u64, core::ptr::null_mut::<bool>(), tld);
        } else {
            // otherwise, purge if needed and push on the visited list
            //
            // note: forced purge can be expensive if many threads are destroyed/created as in mstress.
            mi_segment_try_purge(segment, force, unsafe { (*tld).stats });
            mi_abandoned_visited_push(pool, segment);
        }
    }
}

fn mi_segment_reclaim_or_alloc(
    heap: *mut mi_heap_s,
    needed_slices: u64,
    block_size: u64,
    tld: *mut mi_segments_tld_s,
    os_tld: *mut mi_os_tld_s,
) -> *mut mi_segment_s {
    // 1. try to reclaim an abandoned segment
    let mut reclaimed: bool = false;
    let segment: *mut mi_segment_s = mi_segment_try_reclaim(
        heap,
        needed_slices,
        block_size,
        core::ptr::addr_of_mut!(reclaimed),
        tld,
    );
    if reclaimed {
        // reclaimed the right page right into the heap
        return core::ptr::null_mut::<mi_segment_s>(); // pretend out-of-memory as the page will be in the page queue of the heap with available blocks
    } else {
        if !segment.is_null() {
            // reclaimed a segment with a large enough empty span in it
            return segment;
        }
    }
    // 2. otherwise allocate a fresh segment
    mi_segment_alloc(
        0u64,
        0u64,
        unsafe { (*heap).arena_id },
        tld,
        os_tld,
        core::ptr::null_mut::<*mut mi_page_s>(),
    )
}

fn mi_segments_page_alloc(
    heap: *mut mi_heap_s,
    page_kind: u32,
    required: u64,
    block_size: u64,
    tld: *mut mi_segments_tld_s,
    os_tld: *mut mi_os_tld_s,
) -> *mut mi_page_s {
    // find a free page
    let page_size: u64 = _mi_align_up(
        required,
        if required > 524288u64 {
            524288u64
        } else {
            65536u64
        },
    );
    let slices_needed: u64 = page_size / 65536u64;
    let page: *mut mi_page_s =
        mi_segments_page_find_and_allocate(slices_needed, unsafe { (*heap).arena_id }, tld); // (required <= MI_SMALL_SIZE_MAX ? 0 : slices_needed), tld);
    if page.is_null() {
        // no free page, allocate a new segment and try again
        if mi_segment_reclaim_or_alloc(heap, slices_needed, block_size, tld, os_tld).is_null() {
            // OOM or reclaimed a good page in the heap
            return core::ptr::null_mut::<mi_page_s>();
        } else {
            // otherwise try again
            return mi_segments_page_alloc(heap, page_kind, required, block_size, tld, os_tld);
        }
    }
    mi_segment_try_purge(_mi_ptr_segment(page as *const ()), false, unsafe {
        (*tld).stats
    });
    page
}

fn mi_segment_huge_page_alloc(
    size: u64,
    page_alignment: u64,
    req_arena_id: i32,
    tld: *mut mi_segments_tld_s,
    os_tld: *mut mi_os_tld_s,
) -> *mut mi_page_s {
    let mut page: *mut mi_page_s = core::ptr::null_mut::<mi_page_s>();
    let segment: *mut mi_segment_s = mi_segment_alloc(
        size,
        page_alignment,
        req_arena_id,
        tld,
        os_tld,
        core::ptr::addr_of_mut!(page),
    );
    if segment.is_null() || page.is_null() {
        return core::ptr::null_mut::<mi_page_s>();
    }
    // for huge pages we initialize the xblock_size as we may
    //
    // overallocate to accommodate large alignments.
    let mut psize: u64 = 0;
    let start: *mut u8 = _mi_segment_page_start(
        segment as *const mi_segment_s,
        page as *const mi_page_s,
        core::ptr::addr_of_mut!(psize),
    );
    unsafe {
        (*page).xblock_size = if psize > 2147483648u64 {
            2147483648u32
        } else {
            psize as u32
        };
    }
    // decommit the part of the prefix of a page that will not be used; this can be quite large (close to MI_SEGMENT_SIZE)
    if page_alignment > 0 && unsafe { (*segment).allow_decommit } {
        let aligned_p: *mut u8 = _mi_align_up(start as u64, page_alignment) as *mut u8;
        let decommit_start: *mut u8 = start.wrapping_offset(8 as isize); // for the free list
        let decommit_size: i64 =
            (unsafe { aligned_p.offset_from(decommit_start as *mut u8) }) as i64;
        _mi_os_reset(decommit_start as *mut (), decommit_size as u64, unsafe {
            core::ptr::addr_of_mut!(_mi_stats_main.0)
        }); // note: cannot use segment_decommit on huge segments
    }
    page
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_huge_page_reset(
    segment: *mut mi_segment_s,
    page: *mut mi_page_s,
    block: *mut mi_block_s,
) {
    MI_UNUSED!(page);
    // this is called just before the free
    if unsafe { (*segment).allow_decommit } {
        let mut csize: u64 = mi_usable_size(block as *const ());
        if csize > 8 {
            csize = csize.wrapping_sub(8);
            let p: *mut u8 = (block as *mut u8).wrapping_offset(8 as isize);
            _mi_os_reset(p as *mut (), csize, unsafe {
                core::ptr::addr_of_mut!(_mi_stats_main.0)
            }); // note: cannot use segment_decommit on huge segments
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_page_alloc(
    heap: *mut mi_heap_s,
    block_size: u64,
    mut page_alignment: u64,
    tld: *mut mi_segments_tld_s,
    os_tld: *mut mi_os_tld_s,
) -> *mut mi_page_s {
    let mut page: *mut mi_page_s = unsafe { core::mem::zeroed() };
    if page_alignment > 16777216u64 {
        if page_alignment < 33554432u64 {
            page_alignment = 33554432u64;
        }
        page = mi_segment_huge_page_alloc(
            block_size,
            page_alignment,
            unsafe { (*heap).arena_id },
            tld,
            os_tld,
        );
    } else {
        if block_size <= 16384u64 {
            page = mi_segments_page_alloc(heap, 0u32, block_size, block_size, tld, os_tld);
        } else {
            if block_size <= 131072u64 {
                page = mi_segments_page_alloc(heap, 1u32, 524288u64, block_size, tld, os_tld);
            } else {
                if block_size <= 16777216u64 {
                    page = mi_segments_page_alloc(heap, 2u32, block_size, block_size, tld, os_tld);
                } else {
                    page = mi_segment_huge_page_alloc(
                        block_size,
                        page_alignment,
                        unsafe { (*heap).arena_id },
                        tld,
                        os_tld,
                    );
                }
            }
        }
    }
    page
}

fn mi_segment_visit_page(
    segment: *mut mi_segment_s,
    page: *mut mi_page_s,
    visit_blocks: bool,
    visitor: Option<
        unsafe extern "C-unwind" fn(
            *const mi_heap_s,
            *const mi_heap_area_s,
            *mut (),
            u64,
            *mut (),
        ) -> bool,
    >,
    arg: *mut (),
) -> bool {
    let mut area: mi_heap_area_s = unsafe { core::mem::zeroed() };
    _mi_heap_area_init(core::ptr::addr_of_mut!(area), page);
    if !unsafe {
        visitor.unwrap()(
            core::ptr::null::<mi_heap_s>(),
            core::ptr::addr_of_mut!(area) as *const mi_heap_area_s,
            core::ptr::null_mut::<()>(),
            area.block_size,
            arg,
        )
    } {
        return false;
    }
    if visit_blocks {
        return _mi_heap_area_visit_blocks(
            core::ptr::addr_of_mut!(area) as *const mi_heap_area_s,
            page,
            visitor,
            arg,
        );
    } else {
        return true;
    }
}

fn mi_segment_visit_pages(
    segment: *mut mi_segment_s,
    page_tag: u8,
    visit_blocks: bool,
    visitor: Option<
        unsafe extern "C-unwind" fn(
            *const mi_heap_s,
            *const mi_heap_area_s,
            *mut (),
            u64,
            *mut (),
        ) -> bool,
    >,
    arg: *mut (),
) -> bool {
    let mut end: *const mi_page_s = unsafe { core::mem::zeroed() };
    let mut slice: *mut mi_page_s =
        mi_slices_start_iterate(segment, unsafe { &mut (*core::ptr::addr_of_mut!(end)) });
    while slice < (end as *mut mi_page_s) {
        if mi_slice_is_used(slice as *const mi_page_s) {
            let page: *mut mi_page_s = mi_slice_to_page(slice);
            if ((unsafe { (*page).__slate_bits_0.__get_tag() }) as i32) == (page_tag as i32) {
                if !mi_segment_visit_page(segment, page, visit_blocks, visitor, arg) {
                    return false;
                }
            }
        }
        slice = slice.wrapping_offset((unsafe { (*slice).slice_count }) as isize);
    }
    true
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_abandoned_pool_visit_blocks(
    pool: *mut mi_abandoned_pool_s,
    page_tag: u8,
    visit_blocks: bool,
    visitor: Option<
        unsafe extern "C-unwind" fn(
            *const mi_heap_s,
            *const mi_heap_area_s,
            *mut (),
            u64,
            *mut (),
        ) -> bool,
    >,
    arg: *mut (),
) -> bool {
    // Note: this is not safe in any other thread is abandoning or claiming segments from the pool
    let mut segment: *mut mi_segment_s = mi_tagged_segment_ptr(unsafe {
        core::sync::atomic::AtomicU64::from_ptr(core::ptr::addr_of!((*pool).abandoned) as *mut u64)
            .load(core::sync::atomic::Ordering::SeqCst)
    });
    while !segment.is_null() {
        if !mi_segment_visit_pages(segment, page_tag, visit_blocks, visitor, arg) {
            return false;
        }
        segment = unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(
                core::ptr::addr_of!((*segment).abandoned_next) as *mut *mut mi_segment_s
            )
            .load(core::sync::atomic::Ordering::SeqCst) as *mut mi_segment_s
        };
    }
    segment = unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(
            core::ptr::addr_of!((*pool).abandoned_visited) as *mut *mut mi_segment_s
        )
        .load(core::sync::atomic::Ordering::SeqCst) as *mut mi_segment_s
    };
    while !segment.is_null() {
        if !mi_segment_visit_pages(segment, page_tag, visit_blocks, visitor, arg) {
            return false;
        }
        segment = unsafe {
            core::sync::atomic::AtomicPtr::from_ptr(
                core::ptr::addr_of!((*segment).abandoned_next) as *mut *mut mi_segment_s
            )
            .load(core::sync::atomic::Ordering::SeqCst) as *mut mi_segment_s
        };
    }
    true
}

static mut mi_segment_map: __SlateAlign16<[u64; 20481]> =
    __SlateAlign16(unsafe { core::mem::zeroed() });

fn mi_segment_map_index_of(segment: *const mi_segment_s, bitidx: &mut u64) -> u64 {
    // is it aligned on MI_SEGMENT_SIZE?
    if (segment as u64) >= 43980465111040u64 {
        unsafe {
            *(bitidx as *mut u64) = 0;
        }
        return 20480u64;
    } else {
        let segindex: u64 = (segment as u64) / 33554432u64;
        unsafe {
            *(bitidx as *mut u64) = segindex % 64u64;
        }
        let mapindex: u64 = segindex / 64u64;
        return mapindex;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_map_allocated_at(segment: *const mi_segment_s) {
    let mut bitidx: u64 = 0;
    let index: u64 =
        mi_segment_map_index_of(segment, unsafe { &mut (*core::ptr::addr_of_mut!(bitidx)) });
    if index == 20480u64 {
        return;
    }
    let mut mask: u64 = mi_atomic_load_relaxed!(
        unsafe { core::ptr::addr_of_mut!(mi_segment_map.0) as *mut u64 }
            .wrapping_offset(index as isize)
    );
    let mut newmask: u64 = 0;
    loop {
        newmask = mask | 1u64 << bitidx;
        if !(!mi_atomic_cas_weak_release!(
            unsafe { core::ptr::addr_of_mut!(mi_segment_map.0) as *mut u64 }
                .wrapping_offset(index as isize),
            core::ptr::addr_of_mut!(mask),
            newmask
        )) {
            break;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_segment_map_freed_at(segment: *const mi_segment_s) {
    let mut bitidx: u64 = 0;
    let index: u64 =
        mi_segment_map_index_of(segment, unsafe { &mut (*core::ptr::addr_of_mut!(bitidx)) });
    if index == 20480u64 {
        return;
    }
    let mut mask: u64 = mi_atomic_load_relaxed!(
        unsafe { core::ptr::addr_of_mut!(mi_segment_map.0) as *mut u64 }
            .wrapping_offset(index as isize)
    );
    let mut newmask: u64 = 0;
    loop {
        newmask = mask & !(1u64 << bitidx);
        if !(!mi_atomic_cas_weak_release!(
            unsafe { core::ptr::addr_of_mut!(mi_segment_map.0) as *mut u64 }
                .wrapping_offset(index as isize),
            core::ptr::addr_of_mut!(mask),
            newmask
        )) {
            break;
        }
    }
}

fn _mi_segment_of(p: *const ()) -> *mut mi_segment_s {
    if p.is_null() {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    let mut segment: *mut mi_segment_s = _mi_ptr_segment(p);
    let mut bitidx: u64 = 0;
    let index: u64 = mi_segment_map_index_of(segment as *const mi_segment_s, unsafe {
        &mut (*core::ptr::addr_of_mut!(bitidx))
    });
    // fast path: for any pointer to valid small/medium/large object or first MI_SEGMENT_SIZE in huge
    let mask: u64 = mi_atomic_load_relaxed!(
        unsafe { core::ptr::addr_of_mut!(mi_segment_map.0) as *mut u64 }
            .wrapping_offset(index as isize)
    );
    if mask & 1u64 << bitidx != 0 {
        return segment; // yes, allocated by us
    }
    if index == 20480u64 {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    // TODO: maintain max/min allocated range for efficiency for more efficient rejection of invalid pointers?
    // search downwards for the first segment in case it is an interior pointer
    //
    // could be slow but searches in MI_INTPTR_SIZE * MI_SEGMENT_SIZE (512MiB) steps through
    //
    // valid huge objects
    //
    // note: we could maintain a lowest index to speed up the path for invalid pointers?
    let mut lobitidx: u64 = 0;
    let mut loindex: u64 = 0;
    let lobits: u64 = mask & (1u64 << bitidx).wrapping_sub(1);
    if lobits != 0 {
        loindex = index;
        lobitidx = mi_bsr(lobits); // lobits != 0
    } else {
        if index == 0 {
            return core::ptr::null_mut::<mi_segment_s>();
        } else {
            let mut lomask: u64 = mask;
            loindex = index;
            loop {
                loindex = loindex.wrapping_sub(1);
                lomask = mi_atomic_load_relaxed!(
                    unsafe { core::ptr::addr_of_mut!(mi_segment_map.0) as *mut u64 }
                        .wrapping_offset(loindex as isize)
                );
                if !(lomask != 0 && loindex > 0) {
                    break;
                }
            }
            if lomask == 0 {
                return core::ptr::null_mut::<mi_segment_s>();
            }
            lobitidx = mi_bsr(lomask); // lomask != 0
        }
    }
    // take difference as the addresses could be larger than the MAX_ADDRESS space.
    let diff: u64 = index
        .wrapping_sub(loindex)
        .wrapping_mul(64u64)
        .wrapping_add(bitidx)
        .wrapping_sub(lobitidx)
        .wrapping_mul(33554432u64);
    segment = (segment as *mut u8).wrapping_offset(-(diff as isize)) as *mut mi_segment_s;
    if segment.is_null() {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    let cookie_ok: bool = _mi_ptr_cookie(segment as *const ()) == unsafe { (*segment).cookie };
    if !cookie_ok {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    if (segment as *mut u8).wrapping_offset(mi_segment_size(segment) as isize) <= (p as *mut u8) {
        return core::ptr::null_mut::<mi_segment_s>();
    }
    // outside the range
    segment
}

fn mi_is_valid_pointer(p: *const ()) -> bool {
    !_mi_segment_of(p).is_null() || _mi_arena_contains(p)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_is_in_heap_region(p: *const ()) -> bool {
    mi_is_valid_pointer(p)
}

fn mi_is_in_main(stat: *mut ()) -> bool {
    (stat as *mut u8) >= ((unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) }) as *mut u8)
        && (stat as *mut u8)
            < ((unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) }) as *mut u8)
                .wrapping_offset(core::mem::size_of::<mi_stats_s>() as isize)
}

fn mi_stat_update(stat: *mut mi_stat_count_s, amount: i64) {
    if amount == 0 {
        return;
    }
    if mi_is_in_main(stat as *mut ()) {
        // add atomically (for abandoned pages)
        let current: i64 = mi_atomic_addi64_relaxed(
            (unsafe { core::ptr::addr_of_mut!((*stat).current) }) as *mut i64,
            amount,
        );
        mi_atomic_maxi64_relaxed(
            (unsafe { core::ptr::addr_of_mut!((*stat).peak) }) as *mut i64,
            current.wrapping_add(amount),
        );
        if amount > 0 {
            mi_atomic_addi64_relaxed(
                (unsafe { core::ptr::addr_of_mut!((*stat).allocated) }) as *mut i64,
                amount,
            );
        } else {
            mi_atomic_addi64_relaxed(
                (unsafe { core::ptr::addr_of_mut!((*stat).freed) }) as *mut i64,
                amount.wrapping_neg(),
            );
        }
    } else {
        // add thread local
        let __v1: i64 = unsafe { (*stat).current };
        unsafe {
            (*stat).current = __v1.wrapping_add(amount);
        }
        if (unsafe { (*stat).current }) > unsafe { (*stat).peak } {
            unsafe {
                (*stat).peak = (*stat).current;
            }
        }
        if amount > 0 {
            let __v4: i64 = unsafe { (*stat).allocated };
            unsafe {
                (*stat).allocated = __v4.wrapping_add(amount);
            }
        } else {
            let __v7: i64 = unsafe { (*stat).freed };
            unsafe {
                (*stat).freed = __v7.wrapping_add(amount.wrapping_neg());
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_stat_counter_increase(stat: *mut mi_stat_counter_s, amount: u64) {
    if mi_is_in_main(stat as *mut ()) {
        mi_atomic_addi64_relaxed(
            (unsafe { core::ptr::addr_of_mut!((*stat).count) }) as *mut i64,
            1i64,
        );
        mi_atomic_addi64_relaxed(
            (unsafe { core::ptr::addr_of_mut!((*stat).total) }) as *mut i64,
            amount as i64,
        );
    } else {
        let __v1: i64 = unsafe { (*stat).count };
        unsafe {
            (*stat).count = __v1.wrapping_add(1);
        }
        unsafe {
            (*stat).total = ((*stat).total as u64).wrapping_add(amount) as i64;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_stat_increase(stat: *mut mi_stat_count_s, amount: u64) {
    mi_stat_update(stat, amount as i64);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_stat_decrease(stat: *mut mi_stat_count_s, amount: u64) {
    mi_stat_update(stat, (amount as i64).wrapping_neg());
}

fn mi_stat_add(stat: *mut mi_stat_count_s, src: *const mi_stat_count_s, unit: i64) {
    if stat == (src as *mut mi_stat_count_s) {
        return;
    }
    if (unsafe { (*src).allocated }) == 0 && (unsafe { (*src).freed }) == 0 {
        return;
    }
    mi_atomic_addi64_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*stat).allocated) }) as *mut i64,
        unsafe { (*src).allocated }.wrapping_mul(unit),
    );
    mi_atomic_addi64_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*stat).current) }) as *mut i64,
        unsafe { (*src).current }.wrapping_mul(unit),
    );
    mi_atomic_addi64_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*stat).freed) }) as *mut i64,
        unsafe { (*src).freed }.wrapping_mul(unit),
    );
    // peak scores do not work across threads..
    mi_atomic_addi64_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*stat).peak) }) as *mut i64,
        unsafe { (*src).peak }.wrapping_mul(unit),
    );
}

fn mi_stat_counter_add(stat: *mut mi_stat_counter_s, src: *const mi_stat_counter_s, unit: i64) {
    if stat == (src as *mut mi_stat_counter_s) {
        return;
    }
    mi_atomic_addi64_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*stat).total) }) as *mut i64,
        unsafe { (*src).total }.wrapping_mul(unit),
    );
    mi_atomic_addi64_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*stat).count) }) as *mut i64,
        unsafe { (*src).count }.wrapping_mul(unit),
    );
}

fn mi_stats_add(stats: *mut mi_stats_s, src: *const mi_stats_s) {
    if stats == (src as *mut mi_stats_s) {
        return;
    }
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).segments) },
        unsafe { core::ptr::addr_of!((*src).segments) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).pages) },
        unsafe { core::ptr::addr_of!((*src).pages) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).reserved) },
        unsafe { core::ptr::addr_of!((*src).reserved) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).committed) },
        unsafe { core::ptr::addr_of!((*src).committed) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).reset) },
        unsafe { core::ptr::addr_of!((*src).reset) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).purged) },
        unsafe { core::ptr::addr_of!((*src).purged) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).page_committed) },
        unsafe { core::ptr::addr_of!((*src).page_committed) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).pages_abandoned) },
        unsafe { core::ptr::addr_of!((*src).pages_abandoned) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).segments_abandoned) },
        unsafe { core::ptr::addr_of!((*src).segments_abandoned) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).threads) },
        unsafe { core::ptr::addr_of!((*src).threads) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).malloc) },
        unsafe { core::ptr::addr_of!((*src).malloc) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).segments_cache) },
        unsafe { core::ptr::addr_of!((*src).segments_cache) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).normal) },
        unsafe { core::ptr::addr_of!((*src).normal) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).huge) },
        unsafe { core::ptr::addr_of!((*src).huge) },
        1i64,
    );
    mi_stat_add(
        unsafe { core::ptr::addr_of_mut!((*stats).large) },
        unsafe { core::ptr::addr_of!((*src).large) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).pages_extended) },
        unsafe { core::ptr::addr_of!((*src).pages_extended) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).mmap_calls) },
        unsafe { core::ptr::addr_of!((*src).mmap_calls) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).commit_calls) },
        unsafe { core::ptr::addr_of!((*src).commit_calls) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).reset_calls) },
        unsafe { core::ptr::addr_of!((*src).reset_calls) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).purge_calls) },
        unsafe { core::ptr::addr_of!((*src).purge_calls) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).page_no_retire) },
        unsafe { core::ptr::addr_of!((*src).page_no_retire) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).searches) },
        unsafe { core::ptr::addr_of!((*src).searches) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).normal_count) },
        unsafe { core::ptr::addr_of!((*src).normal_count) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).huge_count) },
        unsafe { core::ptr::addr_of!((*src).huge_count) },
        1i64,
    );
    mi_stat_counter_add(
        unsafe { core::ptr::addr_of_mut!((*stats).large_count) },
        unsafe { core::ptr::addr_of!((*src).large_count) },
        1i64,
    );
}

fn mi_printf_amount(
    mut n: i64,
    unit: i64,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
    fmt: *const i8,
) {
    let mut buf: __SlateAlign16<[i8; 32]> = __SlateAlign16([0i8; 32]);
    buf.0[0] = 0;
    let len: i32 = 32;
    let suffix: *const i8 = (if unit <= 0 {
        c" ".as_ptr() as *mut i8
    } else {
        c"B".as_ptr() as *mut i8
    }) as *const i8;
    let base: i64 = (if unit == 0 { 1000i32 } else { 1024i32 }) as i64;
    if unit > 0 {
        n = n.wrapping_mul(unit);
    }
    let pos: i64 = if n < 0 { n.wrapping_neg() } else { n };
    if pos < base {
        if n != 1 || ((unsafe { *suffix.wrapping_offset(0 as isize) }) as i32) != 66 {
            // skip printing 1 B for the unit column
            unsafe {
                snprintf(
                    (buf.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                    (len as u64) as usize,
                    c"%d   %-3s".as_ptr() as *const core::ffi::c_char,
                    n as i32,
                    if n == 0 { c"".as_ptr() } else { suffix },
                ) as i32
            };
        }
    } else {
        let mut divider: i64 = base;
        let mut magnitude: *const i8 = c"K".as_ptr();
        if pos >= divider.wrapping_mul(base) {
            divider = divider.wrapping_mul(base);
            magnitude = c"M".as_ptr();
        }
        if pos >= divider.wrapping_mul(base) {
            divider = divider.wrapping_mul(base);
            magnitude = c"G".as_ptr();
        }
        let tens: i64 = n / (divider / 10);
        let whole: i64 = tens / 10;
        let frac1: i64 = tens % 10;
        let mut unitdesc: [i8; 8] = [0i8; 8];
        unsafe {
            snprintf(
                (unitdesc.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                8u64 as usize,
                c"%s%s%s".as_ptr() as *const core::ffi::c_char,
                magnitude,
                if base == 1024 {
                    c"i".as_ptr() as *mut i8
                } else {
                    c"".as_ptr() as *mut i8
                },
                suffix,
            ) as i32
        };
        unsafe {
            snprintf(
                (buf.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                (len as u64) as usize,
                c"%ld.%ld %-3s".as_ptr() as *const core::ffi::c_char,
                whole,
                if frac1 < 0 {
                    frac1.wrapping_neg()
                } else {
                    frac1
                },
                unitdesc.as_mut_ptr() as *mut i8,
            ) as i32
        };
    }
    unsafe {
        _mi_fprintf(
            out,
            arg,
            if fmt.is_null() { c"%12s".as_ptr() } else { fmt },
            buf.0.as_mut_ptr() as *mut i8,
        )
    };
}

fn mi_print_amount(
    n: i64,
    unit: i64,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    mi_printf_amount(n, unit, out, arg, core::ptr::null::<i8>());
}

fn mi_print_count(
    n: i64,
    unit: i64,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    if unit == 1 {
        unsafe { _mi_fprintf(out, arg, c"%12s".as_ptr(), c" ".as_ptr() as *mut i8) };
    } else {
        mi_print_amount(n, 0i64, out, arg);
    }
}

fn mi_stat_print_ex(
    stat: *const mi_stat_count_s,
    msg: *const i8,
    unit: i64,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
    notok: *const i8,
) {
    unsafe { _mi_fprintf(out, arg, c"%10s:".as_ptr(), msg) };
    if unit > 0 {
        mi_print_amount(unsafe { (*stat).peak }, unit, out, arg);
        mi_print_amount(unsafe { (*stat).allocated }, unit, out, arg);
        mi_print_amount(unsafe { (*stat).freed }, unit, out, arg);
        mi_print_amount(unsafe { (*stat).current }, unit, out, arg);
        mi_print_amount(unit, 1i64, out, arg);
        mi_print_count(unsafe { (*stat).allocated }, unit, out, arg);
        if (unsafe { (*stat).allocated }) > unsafe { (*stat).freed } {
            unsafe { _mi_fprintf(out, arg, c"  ".as_ptr()) };
            unsafe {
                _mi_fprintf(
                    out,
                    arg,
                    if notok.is_null() {
                        c"not all freed".as_ptr()
                    } else {
                        notok
                    },
                )
            };
            unsafe { _mi_fprintf(out, arg, c"\n".as_ptr()) };
        } else {
            unsafe { _mi_fprintf(out, arg, c"  ok\n".as_ptr()) };
        }
    } else {
        if unit < 0 {
            mi_print_amount(
                unsafe { (*stat).peak },
                1i32.wrapping_neg() as i64,
                out,
                arg,
            );
            mi_print_amount(
                unsafe { (*stat).allocated },
                1i32.wrapping_neg() as i64,
                out,
                arg,
            );
            mi_print_amount(
                unsafe { (*stat).freed },
                1i32.wrapping_neg() as i64,
                out,
                arg,
            );
            mi_print_amount(
                unsafe { (*stat).current },
                1i32.wrapping_neg() as i64,
                out,
                arg,
            );
            if unit == (1i32.wrapping_neg() as i64) {
                unsafe { _mi_fprintf(out, arg, c"%24s".as_ptr(), c"".as_ptr() as *mut i8) };
            } else {
                mi_print_amount(unit.wrapping_neg(), 1i64, out, arg);
                mi_print_count(
                    (unsafe { (*stat).allocated }) / unit.wrapping_neg(),
                    0i64,
                    out,
                    arg,
                );
            }
            if (unsafe { (*stat).allocated }) > unsafe { (*stat).freed } {
                unsafe { _mi_fprintf(out, arg, c"  not all freed!\n".as_ptr()) };
            } else {
                unsafe { _mi_fprintf(out, arg, c"  ok\n".as_ptr()) };
            }
        } else {
            mi_print_amount(unsafe { (*stat).peak }, 1i64, out, arg);
            mi_print_amount(unsafe { (*stat).allocated }, 1i64, out, arg);
            unsafe { _mi_fprintf(out, arg, c"%11s".as_ptr(), c" ".as_ptr() as *mut i8) }; // no freed
            mi_print_amount(unsafe { (*stat).current }, 1i64, out, arg);
            unsafe { _mi_fprintf(out, arg, c"\n".as_ptr()) };
        }
    }
}

fn mi_stat_print(
    stat: *const mi_stat_count_s,
    msg: *const i8,
    unit: i64,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    mi_stat_print_ex(stat, msg, unit, out, arg, core::ptr::null::<i8>());
}

fn mi_stat_peak_print(
    stat: *const mi_stat_count_s,
    msg: *const i8,
    unit: i64,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    unsafe { _mi_fprintf(out, arg, c"%10s:".as_ptr(), msg) };
    mi_print_amount(unsafe { (*stat).peak }, unit, out, arg);
    unsafe { _mi_fprintf(out, arg, c"\n".as_ptr()) };
}

fn mi_stat_counter_print(
    stat: *const mi_stat_counter_s,
    msg: *const i8,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    unsafe { _mi_fprintf(out, arg, c"%10s:".as_ptr(), msg) };
    mi_print_amount(
        unsafe { (*stat).total },
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    unsafe { _mi_fprintf(out, arg, c"\n".as_ptr()) };
}

fn mi_stat_counter_print_avg(
    stat: *const mi_stat_counter_s,
    msg: *const i8,
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    let avg_tens: i64 = if (unsafe { (*stat).count }) == 0 {
        0
    } else {
        unsafe { (*stat).total }.wrapping_mul(10) / unsafe { (*stat).count }
    };
    let avg_whole: i64 = avg_tens / 10;
    let avg_frac1: i64 = avg_tens % 10;
    unsafe {
        _mi_fprintf(
            out,
            arg,
            c"%10s: %5ld.%ld avg\n".as_ptr(),
            msg,
            avg_whole,
            avg_frac1,
        )
    };
}

fn mi_print_header(out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>, arg: *mut ()) {
    unsafe {
        _mi_fprintf(
            out,
            arg,
            c"%10s: %11s %11s %11s %11s %11s %11s\n".as_ptr(),
            c"heap stats".as_ptr() as *mut i8,
            c"peak   ".as_ptr() as *mut i8,
            c"total   ".as_ptr() as *mut i8,
            c"freed   ".as_ptr() as *mut i8,
            c"current   ".as_ptr() as *mut i8,
            c"unit   ".as_ptr() as *mut i8,
            c"count   ".as_ptr() as *mut i8,
        )
    };
}

#[repr(C)]
#[derive(Clone, Copy)]
struct buffered_s {
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
    buf: *mut i8,
    used: u64,
    count: u64,
}

fn mi_buffered_flush(buf: *mut buffered_s) {
    unsafe {
        *unsafe { (*buf).buf }.wrapping_offset((*buf).used as isize) = 0;
    }
    _mi_fputs(
        unsafe { (*buf).out },
        unsafe { (*buf).arg },
        core::ptr::null::<i8>(),
        (unsafe { (*buf).buf }) as *const i8,
    );
    unsafe {
        (*buf).used = 0;
    }
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_buffered_out")]
extern "C-unwind" fn mi_buffered_out(msg: *const i8, arg: *mut ()) {
    let buf: *mut buffered_s = arg as *mut buffered_s;
    if msg.is_null() || buf.is_null() {
        return;
    }
    let mut src: *const i8 = msg;
    while ((unsafe { *src }) as i32) != 0 {
        let c: i8 = unsafe { *src };
        if (unsafe { (*buf).used }) >= unsafe { (*buf).count } {
            mi_buffered_flush(buf);
        }
        let __v1: u64 = unsafe { (*buf).used };
        unsafe {
            (*buf).used = __v1.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*buf).buf }.wrapping_offset(__v1 as isize) = c;
        }
        if (c as i32) == 10 {
            mi_buffered_flush(buf);
        }
        let __v4: *const i8 = src;
        src = __v4.wrapping_offset(1 as isize);
    }
}

fn _mi_stats_print(
    stats: *mut mi_stats_s,
    out0: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg0: *mut (),
) {
    // wrap the output function to be line buffered
    let mut buf: __SlateAlign16<[i8; 256]> = __SlateAlign16([0i8; 256]);
    let mut buffer: buffered_s = buffered_s {
        out: out0,
        arg: arg0,
        buf: core::ptr::null_mut::<i8>(),
        used: 0,
        count: 255,
    };
    buffer.buf = buf.0.as_mut_ptr() as *mut i8;
    let out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())> = Some(mi_buffered_out);
    let arg: *mut () = core::ptr::addr_of_mut!(buffer) as *mut ();
    // and print using that
    mi_print_header(out, arg);
    mi_stat_print_ex(
        (unsafe { core::ptr::addr_of_mut!((*stats).reserved) }) as *const mi_stat_count_s,
        c"reserved".as_ptr(),
        1i64,
        out,
        arg,
        c"".as_ptr(),
    );
    mi_stat_print_ex(
        (unsafe { core::ptr::addr_of_mut!((*stats).committed) }) as *const mi_stat_count_s,
        c"committed".as_ptr(),
        1i64,
        out,
        arg,
        c"".as_ptr(),
    );
    mi_stat_peak_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).reset) }) as *const mi_stat_count_s,
        c"reset".as_ptr(),
        1i64,
        out,
        arg,
    );
    mi_stat_peak_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).purged) }) as *const mi_stat_count_s,
        c"purged".as_ptr(),
        1i64,
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).page_committed) }) as *const mi_stat_count_s,
        c"touched".as_ptr(),
        1i64,
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).segments) }) as *const mi_stat_count_s,
        c"segments".as_ptr(),
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).segments_abandoned) }) as *const mi_stat_count_s,
        c"-abandoned".as_ptr(),
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).segments_cache) }) as *const mi_stat_count_s,
        c"-cached".as_ptr(),
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).pages) }) as *const mi_stat_count_s,
        c"pages".as_ptr(),
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).pages_abandoned) }) as *const mi_stat_count_s,
        c"-abandoned".as_ptr(),
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    mi_stat_counter_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).pages_extended) }) as *const mi_stat_counter_s,
        c"-extended".as_ptr(),
        out,
        arg,
    );
    mi_stat_counter_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).page_no_retire) }) as *const mi_stat_counter_s,
        c"-noretire".as_ptr(),
        out,
        arg,
    );
    mi_stat_counter_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).mmap_calls) }) as *const mi_stat_counter_s,
        c"mmaps".as_ptr(),
        out,
        arg,
    );
    mi_stat_counter_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).commit_calls) }) as *const mi_stat_counter_s,
        c"commits".as_ptr(),
        out,
        arg,
    );
    mi_stat_counter_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).reset_calls) }) as *const mi_stat_counter_s,
        c"resets".as_ptr(),
        out,
        arg,
    );
    mi_stat_counter_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).purge_calls) }) as *const mi_stat_counter_s,
        c"purges".as_ptr(),
        out,
        arg,
    );
    mi_stat_print(
        (unsafe { core::ptr::addr_of_mut!((*stats).threads) }) as *const mi_stat_count_s,
        c"threads".as_ptr(),
        1i32.wrapping_neg() as i64,
        out,
        arg,
    );
    mi_stat_counter_print_avg(
        (unsafe { core::ptr::addr_of_mut!((*stats).searches) }) as *const mi_stat_counter_s,
        c"searches".as_ptr(),
        out,
        arg,
    );
    unsafe {
        _mi_fprintf(
            out,
            arg,
            c"%10s: %5zu\n".as_ptr(),
            c"numa nodes".as_ptr() as *mut i8,
            _mi_os_numa_node_count(),
        )
    };
    let mut elapsed: u64 = 0;
    let mut user_time: u64 = 0;
    let mut sys_time: u64 = 0;
    let mut current_rss: u64 = 0;
    let mut peak_rss: u64 = 0;
    let mut current_commit: u64 = 0;
    let mut peak_commit: u64 = 0;
    let mut page_faults: u64 = 0;
    mi_process_info(
        core::ptr::addr_of_mut!(elapsed),
        core::ptr::addr_of_mut!(user_time),
        core::ptr::addr_of_mut!(sys_time),
        core::ptr::addr_of_mut!(current_rss),
        core::ptr::addr_of_mut!(peak_rss),
        core::ptr::addr_of_mut!(current_commit),
        core::ptr::addr_of_mut!(peak_commit),
        core::ptr::addr_of_mut!(page_faults),
    );
    unsafe {
        _mi_fprintf(
            out,
            arg,
            c"%10s: %5ld.%03ld s\n".as_ptr(),
            c"elapsed".as_ptr() as *mut i8,
            elapsed / 1000,
            elapsed % 1000,
        )
    };
    unsafe {
        _mi_fprintf(
            out,
            arg,
            c"%10s: user: %ld.%03ld s, system: %ld.%03ld s, faults: %lu, rss: ".as_ptr(),
            c"process".as_ptr() as *mut i8,
            user_time / 1000,
            user_time % 1000,
            sys_time / 1000,
            sys_time % 1000,
            page_faults,
        )
    };
    mi_printf_amount(peak_rss as i64, 1i64, out, arg, c"%s".as_ptr());
    if peak_commit > 0 {
        unsafe { _mi_fprintf(out, arg, c", commit: ".as_ptr()) };
        mi_printf_amount(peak_commit as i64, 1i64, out, arg, c"%s".as_ptr());
    }
    unsafe { _mi_fprintf(out, arg, c"\n".as_ptr()) };
}

static mut mi_process_start: i64 = unsafe { core::mem::zeroed() };

fn mi_stats_get_default() -> *mut mi_stats_s {
    let heap: *mut mi_heap_s = mi_heap_get_default();
    unsafe { core::ptr::addr_of_mut!((*(*heap).tld).stats) }
}

fn mi_stats_merge_from(stats: *mut mi_stats_s) {
    if stats != unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) } {
        mi_stats_add(
            unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
            stats as *const mi_stats_s,
        );
        unsafe {
            core::ptr::write_bytes(
                ((stats as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                ((core::mem::size_of::<mi_stats_s>() as u64) as usize) as usize,
            )
        };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_stats_reset() {
    let stats: *mut mi_stats_s = mi_stats_get_default();
    if stats != unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) } {
        unsafe {
            core::ptr::write_bytes(
                ((stats as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                ((core::mem::size_of::<mi_stats_s>() as u64) as usize) as usize,
            )
        };
    }
    unsafe {
        core::ptr::write_bytes(
            ((core::ptr::addr_of_mut!(_mi_stats_main.0) as *mut ()) as *mut core::ffi::c_void)
                as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<mi_stats_s>() as u64) as usize) as usize,
        )
    };
    if (unsafe { mi_process_start }) == 0 {
        unsafe {
            mi_process_start = _mi_clock_start();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_stats_merge() {
    mi_stats_merge_from(mi_stats_get_default());
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_stats_done(stats: *mut mi_stats_s) {
    // called from `mi_thread_done`
    mi_stats_merge_from(stats);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_stats_print_out(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    mi_stats_merge_from(mi_stats_get_default());
    _mi_stats_print(
        unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0) },
        out,
        arg,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_stats_print(out: *mut ()) {
    // for compatibility there is an `out` parameter (which can be `stdout` or `stderr`)
    mi_stats_print_out(
        unsafe {
            core::mem::transmute::<*mut (), Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>>(
                out,
            )
        },
        core::ptr::null_mut::<()>(),
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_thread_stats_print_out(
    out: Option<unsafe extern "C-unwind" fn(*const i8, *mut ())>,
    arg: *mut (),
) {
    _mi_stats_print(mi_stats_get_default(), out, arg);
}

static mut mi_clock_diff: i64 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_clock_now() -> i64 {
    _mi_prim_clock_now()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_clock_start() -> i64 {
    if ((unsafe { mi_clock_diff }) as f64) == 0.0f64 {
        let t0: i64 = _mi_clock_now();
        unsafe {
            mi_clock_diff = _mi_clock_now().wrapping_sub(t0);
        }
    }
    _mi_clock_now()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_clock_end(start: i64) -> i64 {
    let end: i64 = _mi_clock_now();
    end.wrapping_sub(start)
        .wrapping_sub(unsafe { mi_clock_diff })
}

#[unsafe(no_mangle)]
extern "C-unwind" fn mi_process_info(
    elapsed_msecs: *mut u64,
    user_msecs: *mut u64,
    system_msecs: *mut u64,
    current_rss: *mut u64,
    peak_rss: *mut u64,
    current_commit: *mut u64,
    peak_commit: *mut u64,
    page_faults: *mut u64,
) {
    let mut pinfo: mi_process_info_s = unsafe { core::mem::zeroed() };
    _mi_memzero_var!(pinfo);
    pinfo.elapsed = _mi_clock_end(unsafe { mi_process_start });
    pinfo.current_commit = mi_atomic_loadi64_relaxed!(
        (unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.committed.current) }) as *mut i64
    ) as u64;
    pinfo.peak_commit = mi_atomic_loadi64_relaxed!(
        (unsafe { core::ptr::addr_of_mut!(_mi_stats_main.0.committed.peak) }) as *mut i64
    ) as u64;
    pinfo.current_rss = pinfo.current_commit;
    pinfo.peak_rss = pinfo.peak_commit;
    pinfo.utime = 0;
    pinfo.stime = 0;
    pinfo.page_faults = 0;
    _mi_prim_process_info(core::ptr::addr_of_mut!(pinfo));
    if !elapsed_msecs.is_null() {
        unsafe {
            *elapsed_msecs = if pinfo.elapsed < 0 {
                0
            } else {
                if pinfo.elapsed < 9223372036854775807 {
                    pinfo.elapsed as u64
                } else {
                    9223372036854775807
                }
            };
        }
    }
    if !user_msecs.is_null() {
        unsafe {
            *user_msecs = if pinfo.utime < 0 {
                0
            } else {
                if pinfo.utime < 9223372036854775807 {
                    pinfo.utime as u64
                } else {
                    9223372036854775807
                }
            };
        }
    }
    if !system_msecs.is_null() {
        unsafe {
            *system_msecs = if pinfo.stime < 0 {
                0
            } else {
                if pinfo.stime < 9223372036854775807 {
                    pinfo.stime as u64
                } else {
                    9223372036854775807
                }
            };
        }
    }
    if !current_rss.is_null() {
        unsafe {
            *current_rss = pinfo.current_rss;
        }
    }
    if !peak_rss.is_null() {
        unsafe {
            *peak_rss = pinfo.peak_rss;
        }
    }
    if !current_commit.is_null() {
        unsafe {
            *current_commit = pinfo.current_commit;
        }
    }
    if !peak_commit.is_null() {
        unsafe {
            *peak_commit = pinfo.peak_commit;
        }
    }
    if !page_faults.is_null() {
        unsafe {
            *page_faults = pinfo.page_faults;
        }
    }
}

fn mi_prim_open(fpath: *const i8, open_flags: i32) -> i32 {
    (unsafe { syscall(2i64 as i64, fpath, open_flags, 0i32) as i64 }) as i32
}

fn mi_prim_read(fd: i32, buf: *mut (), bufsize: u64) -> i64 {
    unsafe { syscall(0i64 as i64, fd, buf, bufsize) as i64 }
}

fn mi_prim_close(fd: i32) -> i32 {
    (unsafe { syscall(3i64 as i64, fd) as i64 }) as i32
}

fn mi_prim_access(fpath: *const i8, mode: i32) -> i32 {
    (unsafe { syscall(21i64 as i64, fpath, mode) as i64 }) as i32
}

fn unix_detect_overcommit() -> bool {
    let mut os_overcommit: bool = true;
    let fd: i32 = mi_prim_open(
        c"/proc/sys/vm/overcommit_memory".as_ptr(),
        slate_support::O_RDONLY as i32,
    );
    if fd >= 0 {
        let mut buf: __SlateAlign16<[i8; 32]> = __SlateAlign16({
            let mut __t0: [i8; 32] = unsafe { core::mem::zeroed() };
            __t0[0] = 0i8;
            __t0
        });
        let nread: i64 = mi_prim_read(
            fd,
            core::ptr::addr_of_mut!(buf.0) as *mut (),
            core::mem::size_of::<[i8; 32]>() as u64,
        );
        mi_prim_close(fd);
        // <https://www.kernel.org/doc/Documentation/vm/overcommit-accounting>
        //
        // 0: heuristic overcommit, 1: always overcommit, 2: never overcommit (ignore NORESERVE)
        if nread >= 1 {
            os_overcommit = (buf.0[0] as i32) == 48 || (buf.0[0] as i32) == 49;
        }
    }
    os_overcommit
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_mem_init(config: *mut mi_os_mem_config_s) {
    let psize: i64 = unsafe { sysconf((slate_support::_SC_PAGESIZE as i32) as i32) as i64 };
    if psize > 0 {
        unsafe {
            (*config).page_size = psize as u64;
        }
        unsafe {
            (*config).alloc_granularity = psize as u64;
        }
    }
    unsafe {
        (*config).large_page_size = 2097152u64;
    }
    // TODO: can we query the OS for this?
    unsafe {
        (*config).has_overcommit = unix_detect_overcommit();
    }
    unsafe {
        (*config).must_free_whole = false;
    }
    // mmap can free in parts
    unsafe {
        (*config).has_virtual_reserve = true;
    }
    // todo: check if this true for NetBSD?  (for anonymous mmap with PROT_NONE)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_free(addr: *mut (), size: u64) -> i32 {
    let err: bool = (unsafe { munmap(addr as *mut core::ffi::c_void, size as usize) as i32 })
        == 1i32.wrapping_neg();
    if err {
        unsafe { *slate_support::errno_location() }
    } else {
        0
    }
}

fn unix_madvise(addr: *mut (), size: u64, advice__0: i32) -> i32 {
    unsafe {
        madvise(
            addr as *mut core::ffi::c_void,
            size as usize,
            advice__0 as i32,
        ) as i32
    }
}

fn unix_mmap_prim(
    addr: *mut (),
    size: u64,
    try_alignment: u64,
    protect_flags: i32,
    flags: i32,
    fd: i32,
) -> *mut () {
    MI_UNUSED!(try_alignment);
    let mut p: *mut () = core::ptr::null_mut::<()>();
    // on 64-bit systems, use the virtual address area after 2TiB for 4MiB aligned allocations
    if addr.is_null() {
        let hint: *mut () = _mi_os_get_aligned_hint(try_alignment, size);
        if !hint.is_null() {
            p = unsafe { mmap(hint, size, protect_flags, flags, fd, 0i64) };
            let __v0: bool =
                p == (1i32.wrapping_neg() as *mut ()) || !_mi_is_aligned(p, try_alignment);
            if __v0 {
                let err: i32 = unsafe { *slate_support::errno_location() };
                unsafe {
                    _mi_verbose_message(c"unable to directly request hinted aligned OS memory (error: %d (0x%x), size: 0x%zx bytes, alignment: 0x%zx, hint address: %p)\n".as_ptr(), err, err, size, try_alignment, hint)
                };
            }
            if p != (1i32.wrapping_neg() as *mut ()) {
                return p;
            }
            // fall back to regular mmap
        }
    }
    // regular mmap
    p = unsafe { mmap(addr, size, protect_flags, flags, fd, 0i64) };
    if p != (1i32.wrapping_neg() as *mut ()) {
        return p;
    }
    // failed to allocate
    core::ptr::null_mut::<()>()
}

fn unix_mmap_fd() -> i32 {
    1i32.wrapping_neg()
}

fn unix_mmap(
    addr: *mut (),
    size: u64,
    try_alignment: u64,
    protect_flags: i32,
    large_only: bool,
    allow_large: bool,
    is_large: *mut bool,
) -> *mut () {
    let mut p: *mut () = core::ptr::null_mut::<()>();
    let fd: i32 = unix_mmap_fd();
    let mut flags: i32 =
        (slate_support::MAP_PRIVATE as i32) | (slate_support::MAP_ANONYMOUS as i32);
    if _mi_os_has_overcommit() {
        let __v1: i32 = flags | (slate_support::MAP_NORESERVE as i32);
        flags = __v1;
    }
    // huge page allocation
    let __v2: bool = large_only || _mi_os_use_large_page(size, try_alignment);
    if __v2 && allow_large {
        // = 0;
        let mut try_ok: u64 =
            mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!(large_page_try_ok) });
        if !large_only && try_ok > 0 {
            // If the OS is not configured for large OS pages, or the user does not have
            //
            // enough permission, the `mmap` will always fail (but it might also fail for other reasons).
            //
            // Therefore, once a large page allocation failed, we don't try again for `large_page_try_ok` times
            //
            // to avoid too many failing calls to mmap.
            mi_atomic_cas_strong_acq_rel__2!(
                unsafe { core::ptr::addr_of_mut!(large_page_try_ok) },
                core::ptr::addr_of_mut!(try_ok),
                try_ok.wrapping_sub(1)
            );
        } else {
            let mut lflags: i32 = flags & !(slate_support::MAP_NORESERVE as i32); // using NORESERVE on huge pages seems to fail on Linux
            let lfd: i32 = fd;
            let __v5: i32 = lflags | (slate_support::MAP_HUGETLB as i32);
            lflags = __v5;
            if size % 1073741824u64 == 0 && unsafe { mi_huge_pages_available } {
                lflags = ((lflags as u32) | 2013265920u32) as i32;
            } else {
                lflags = ((lflags as u32) | 1409286144u32) as i32;
            }
            if large_only || lflags != flags {
                // try large OS page allocation
                unsafe {
                    *is_large = true;
                }
                p = unix_mmap_prim(addr, size, try_alignment, protect_flags, lflags, lfd);
                if p.is_null() && (lflags as u32) & 2013265920u32 != 0 {
                    unsafe {
                        mi_huge_pages_available = false;
                    }
                    // don't try huge 1GiB pages again
                    unsafe {
                        _mi_warning_message(c"unable to allocate huge (1GiB) page, trying large (2MiB) pages instead (errno: %i)\n".as_ptr(), *slate_support::errno_location())
                    };
                    lflags = ((lflags as u32) & 2281701375u32 | 1409286144u32) as i32;
                    p = unix_mmap_prim(addr, size, try_alignment, protect_flags, lflags, lfd);
                }
                if large_only {
                    return p;
                }
                if p.is_null() {
                    mi_atomic_store_release!(
                        unsafe { core::ptr::addr_of_mut!(large_page_try_ok) },
                        8u64
                    ); // on error, don't try again for the next N allocations
                }
            }
        }
    }
    // regular allocation
    if p.is_null() {
        unsafe {
            *is_large = false;
        }
        p = unix_mmap_prim(addr, size, try_alignment, protect_flags, flags, fd);
        if !p.is_null() {
            // Many Linux systems don't allow MAP_HUGETLB but they support instead
            //
            // transparent huge pages (THP). Generally, it is not required to call `madvise` with MADV_HUGE
            //
            // though since properly aligned allocations will already use large pages if available
            //
            // in that case -- in particular for our large regions (in `memory.c`).
            //
            // However, some systems only allow THP if called with explicit `madvise`, so
            //
            // when large OS pages are enabled for mimalloc, we call `madvise` anyways.
            let __v10: bool = allow_large && _mi_os_use_large_page(size, try_alignment);
            if __v10 {
                if unix_madvise(p, size, slate_support::MADV_HUGEPAGE as i32) == 0 {
                    unsafe {
                        *is_large = true;
                    }
                    // possibly
                }
            }
        }
    }
    p
}

static mut large_page_try_ok: u64 = unsafe { core::mem::zeroed() };

static mut mi_huge_pages_available: bool = 1i32 != 0i32;

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_alloc(
    size: u64,
    try_alignment: u64,
    commit: bool,
    allow_large: bool,
    is_large: *mut bool,
    is_zero: *mut bool,
    addr: *mut *mut (),
) -> i32 {
    unsafe {
        *is_zero = true;
    }
    let protect_flags: i32 = if commit {
        (slate_support::PROT_WRITE as i32) | (slate_support::PROT_READ as i32)
    } else {
        slate_support::PROT_NONE as i32
    };
    unsafe {
        *addr = unix_mmap(
            core::ptr::null_mut::<()>(),
            size,
            try_alignment,
            protect_flags,
            false,
            allow_large,
            is_large,
        );
    }
    if !unsafe { *addr }.is_null() {
        0
    } else {
        unsafe { *slate_support::errno_location() }
    }
}

fn unix_mprotect_hint(err: i32) {
    MI_UNUSED!(err);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_commit(start: *mut (), size: u64, is_zero: *mut bool) -> i32 {
    // commit: ensure we can access the area
    //
    // note: we may think that *is_zero can be true since the memory
    //
    // was either from mmap PROT_NONE, or from decommit MADV_DONTNEED, but
    //
    // we sometimes call commit on a range with still partially committed
    //
    // memory and `mprotect` does not zero the range.
    unsafe {
        *is_zero = false;
    }
    let mut err: i32 = unsafe {
        mprotect(
            start as *mut core::ffi::c_void,
            size as usize,
            ((slate_support::PROT_READ as i32) | (slate_support::PROT_WRITE as i32)) as i32,
        ) as i32
    };
    if err != 0 {
        err = unsafe { *slate_support::errno_location() };
        unix_mprotect_hint(err);
    }
    err
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_decommit(
    start: *mut (),
    size: u64,
    needs_recommit: *mut bool,
) -> i32 {
    let mut err: i32 = 0;
    // decommit: use MADV_DONTNEED as it decreases rss immediately (unlike MADV_FREE)
    err = unix_madvise(start, size, slate_support::MADV_DONTNEED as i32);
    unsafe {
        *needs_recommit = false;
    }
    // // decommit: use mmap with MAP_FIXED and PROT_NONE to discard the existing memory (and reduce rss)
    // *needs_recommit = true;
    // const int fd = unix_mmap_fd();
    // void* p = mmap(start, size, PROT_NONE, (MAP_FIXED | MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE), fd, 0);
    // if (p != start) { err = errno; }
    err
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_reset(start: *mut (), size: u64) -> i32 {
    // We try to use `MADV_FREE` as that is the fastest. A drawback though is that it
    //
    // will not reduce the `rss` stats in tools like `top` even though the memory is available
    //
    // to other processes. With the default `MIMALLOC_PURGE_DECOMMITS=1` we ensure that by
    //
    // default `MADV_DONTNEED` is used though.
    let oadvice: i32 = mi_atomic_load_relaxed!(unsafe { core::ptr::addr_of_mut!(advice) }) as i32;
    let mut err: i32 = 0;
    loop {
        let __v0: i32 = unix_madvise(start, size, oadvice);
        err = __v0;
        if !(__v0 != 0
            && (unsafe { *slate_support::errno_location() }) == (slate_support::EAGAIN as i32))
        {
            break;
        }
        unsafe {
            *slate_support::errno_location() = 0;
        }
    }
    if err != 0
        && (unsafe { *slate_support::errno_location() }) == (slate_support::EINVAL as i32)
        && oadvice == (slate_support::MADV_FREE as i32)
    {
        // if MADV_FREE is not supported, fall back to MADV_DONTNEED from now on
        mi_atomic_store_release!(
            unsafe { core::ptr::addr_of_mut!(advice) },
            (slate_support::MADV_DONTNEED as i32) as u64
        );
        err = unix_madvise(start, size, slate_support::MADV_DONTNEED as i32);
    }
    err
}

static mut advice: u64 = 8;

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_protect(start: *mut (), size: u64, protect: bool) -> i32 {
    let mut err: i32 = unsafe {
        mprotect(
            start as *mut core::ffi::c_void,
            size as usize,
            (if protect {
                slate_support::PROT_NONE as i32
            } else {
                (slate_support::PROT_READ as i32) | (slate_support::PROT_WRITE as i32)
            }) as i32,
        ) as i32
    };
    if err != 0 {
        err = unsafe { *slate_support::errno_location() };
    }
    unix_mprotect_hint(err);
    err
}

fn mi_prim_mbind(
    start: *mut (),
    len: u64,
    mode: u64,
    nmask: *const u64,
    maxnode: u64,
    flags: u32,
) -> i64 {
    unsafe { syscall(237i64 as i64, start, len, mode, nmask, maxnode, flags) as i64 }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_alloc_huge_os_pages(
    hint_addr: *mut (),
    size: u64,
    numa_node: i32,
    is_zero: *mut bool,
    addr: *mut *mut (),
) -> i32 {
    let mut is_large: bool = true;
    unsafe {
        *is_zero = true;
    }
    unsafe {
        *addr = unix_mmap(
            hint_addr,
            size,
            33554432u64,
            (slate_support::PROT_READ as i32) | (slate_support::PROT_WRITE as i32),
            true,
            true,
            core::ptr::addr_of_mut!(is_large),
        );
    }
    if !unsafe { *addr }.is_null() && numa_node >= 0 && numa_node < 64i32 {
        // at most 64 nodes
        let mut numa_mask: u64 = 1u64 << numa_node;
        // TODO: does `mbind` work correctly for huge OS pages? should we
        //
        // use `set_mempolicy` before calling mmap instead?
        //
        // see: <https://lkml.org/lkml/2017/2/9/875>
        let mut err: i64 = mi_prim_mbind(
            unsafe { *addr },
            size,
            1u64,
            core::ptr::addr_of_mut!(numa_mask) as *const u64,
            64u64,
            0u32,
        );
        if err != 0 {
            err = (unsafe { *slate_support::errno_location() }) as i64;
            unsafe {
                _mi_warning_message(
                    c"failed to bind huge (1GiB) pages to numa node %d (error: %d (0x%x))\n"
                        .as_ptr(),
                    numa_node,
                    err,
                    err,
                )
            };
        }
    }
    if !unsafe { *addr }.is_null() {
        0
    } else {
        unsafe { *slate_support::errno_location() }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_numa_node() -> u64 {
    let mut node: u64 = 0;
    let mut ncpu: u64 = 0;
    let err: i64 = unsafe {
        syscall(
            309i64 as i64,
            core::ptr::addr_of_mut!(ncpu),
            core::ptr::addr_of_mut!(node),
            core::ptr::null_mut::<()>(),
        ) as i64
    };
    if err != 0 {
        return 0;
    }
    node
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_numa_node_count() -> u64 {
    let mut buf: __SlateAlign16<[i8; 128]> = __SlateAlign16([0i8; 128]);
    let mut node: u32 = 0;
    while node < 256 {
        // enumerate node entries -- todo: it there a more efficient way to do this? (but ensure there is no allocation)
        unsafe {
            snprintf(
                (buf.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
                127u64 as usize,
                c"/sys/devices/system/node/node%u".as_ptr() as *const core::ffi::c_char,
                node.wrapping_add(1),
            ) as i32
        };
        if mi_prim_access(buf.0.as_mut_ptr() as *const i8, slate_support::R_OK as i32) != 0 {
            break;
        }
        node = node.wrapping_add(1);
    }
    node.wrapping_add(1) as u64
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_clock_now() -> i64 {
    let mut t: slate_support::timespec = unsafe { core::mem::zeroed() };
    unsafe {
        clock_gettime(
            (slate_support::CLOCK_MONOTONIC as i32) as i32,
            core::ptr::addr_of_mut!(t) as *mut slate_support::timespec,
        ) as i32
    };
    t.tv_sec
        .wrapping_mul(1000)
        .wrapping_add(t.tv_nsec / 1000000)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct rusage {
    ru_utime: slate_support::timeval,
    ru_stime: slate_support::timeval,
    __slate_anon_2: __SlateRecord_struct_rusage_h_40,
    __slate_anon_3: __SlateRecord_struct_rusage_h_47,
    __slate_anon_4: __SlateRecord_struct_rusage_h_53,
    __slate_anon_5: __SlateRecord_struct_rusage_h_59,
    __slate_anon_6: __SlateRecord_struct_rusage_h_66,
    __slate_anon_7: __SlateRecord_struct_rusage_h_72,
    __slate_anon_8: __SlateRecord_struct_rusage_h_78,
    __slate_anon_9: __SlateRecord_struct_rusage_h_85,
    __slate_anon_10: __SlateRecord_struct_rusage_h_91,
    __slate_anon_11: __SlateRecord_struct_rusage_h_97,
    __slate_anon_12: __SlateRecord_struct_rusage_h_103,
    __slate_anon_13: __SlateRecord_struct_rusage_h_109,
    __slate_anon_14: __SlateRecord_struct_rusage_h_117,
    __slate_anon_15: __SlateRecord_struct_rusage_h_124,
}

fn timeval_secs(tv: &slate_support::timeval) -> i64 {
    unsafe { (*(tv as *const slate_support::timeval)).tv_sec }
        .wrapping_mul(1000)
        .wrapping_add((unsafe { (*(tv as *const slate_support::timeval)).tv_usec }) / 1000)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_process_info(pinfo: *mut mi_process_info_s) {
    let mut rusage: rusage = unsafe { core::mem::zeroed() };
    unsafe {
        getrusage(
            slate_support::RUSAGE_SELF as i32,
            core::ptr::addr_of_mut!(rusage),
        )
    };
    unsafe {
        (*pinfo).utime = timeval_secs(
            &(*(core::ptr::addr_of_mut!(rusage.ru_utime) as *const slate_support::timeval)),
        );
    }
    unsafe {
        (*pinfo).stime = timeval_secs(
            &(*(core::ptr::addr_of_mut!(rusage.ru_stime) as *const slate_support::timeval)),
        );
    }
    unsafe {
        (*pinfo).page_faults = rusage.__slate_anon_7.ru_majflt as u64;
    }
    unsafe {
        (*pinfo).peak_rss = unsafe { rusage.__slate_anon_2.ru_maxrss }.wrapping_mul(1024) as u64;
    }
    // Linux/BSD report in KiB
    // use defaults for commit
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_out_stderr(msg: *const i8) {
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputs(msg, slate_support::stderr()) };
    unsafe { fflush(core::ptr::null_mut()) };
}

fn mi_get_environ() -> *mut *mut i8 {
    unsafe { environ }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_getenv(name: *const i8, result: *mut i8, result_size: u64) -> bool {
    if name.is_null() {
        return false;
    }
    let len: u64 = _mi_strlen(name);
    if len == 0 {
        return false;
    }
    let env: *mut *mut i8 = mi_get_environ();
    if env.is_null() {
        return false;
    }
    // compare up to 10000 entries
    let mut i: i32 = 0;
    while i < 10000 && !unsafe { *env.wrapping_offset(i as isize) }.is_null() {
        let s: *const i8 = (unsafe { *env.wrapping_offset(i as isize) }) as *const i8;
        if _mi_strnicmp(name, s, len) == 0
            && ((unsafe { *s.wrapping_offset(len as isize) }) as i32) == 61
        {
            // case insensitive
            // found it
            _mi_strlcpy(
                result,
                s.wrapping_offset(len as isize).wrapping_offset(1 as isize),
                result_size,
            );
            return true;
        }
        i = i.wrapping_add(1);
    }
    false
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_random_buf(buf: *mut (), buf_len: u64) -> bool {
    // Modern Linux provides `getrandom` but different distributions either use `sys/random.h` or `linux/random.h`
    //
    // and for the latter the actual `getrandom` call is not always defined.
    //
    // (see <https://stackoverflow.com/questions/45237324/why-doesnt-getrandom-compile>)
    //
    // We therefore use a syscall directly and fall back dynamically to /dev/urandom when needed.
    // = 0
    if mi_atomic_load_acquire!(unsafe { core::ptr::addr_of_mut!(no_getrandom) }) == 0 {
        let ret: i64 = unsafe { syscall(318i64 as i64, buf, buf_len, 1i32) as i64 };
        if ret >= 0 {
            return buf_len == (ret as u64);
        }
        if (unsafe { *slate_support::errno_location() }) != (slate_support::ENOSYS as i32) {
            return false;
        }
        mi_atomic_store_release!(unsafe { core::ptr::addr_of_mut!(no_getrandom) }, 1u64); // don't call again, and fall back to /dev/urandom
    }
    let mut flags: i32 = slate_support::O_RDONLY as i32;
    flags |= 524288;
    let fd: i32 = mi_prim_open(c"/dev/urandom".as_ptr(), flags);
    if fd < 0 {
        return false;
    }
    let mut count: u64 = 0;
    while count < buf_len {
        let ret: i64 = mi_prim_read(
            fd,
            (buf as *mut i8).wrapping_offset(count as isize) as *mut (),
            buf_len.wrapping_sub(count),
        );
        if ret <= 0 {
            if (unsafe { *slate_support::errno_location() }) != (slate_support::EAGAIN as i32)
                && (unsafe { *slate_support::errno_location() }) != (slate_support::EINTR as i32)
            {
                break;
            }
        } else {
            count = count.wrapping_add(ret as u64);
        }
    }
    mi_prim_close(fd);
    count == buf_len
}

static mut no_getrandom: u64 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
static mut _mi_heap_default_key: u32 = 1i32.wrapping_neg() as u32;

#[unsafe(link_section = ".text.slate_distinct.obmalloc.mi_pthread_done")]
extern "C-unwind" fn mi_pthread_done(value: *mut ()) {
    if !value.is_null() {
        _mi_thread_done(value as *mut mi_heap_s);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_thread_init_auto_done() {
    unsafe {
        pthread_key_create(
            core::ptr::addr_of_mut!(_mi_heap_default_key) as *mut u32,
            unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                    Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> ()>,
                >(Some(mi_pthread_done))
            },
        ) as i32
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_thread_done_auto_done() {
    // nothing to do
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _mi_prim_thread_associate_default_heap(heap: *mut mi_heap_s) {
    if (unsafe { _mi_heap_default_key }) != (1i32.wrapping_neg() as u32) {
        // can happen during recursive invocation on freeBSD
        unsafe {
            pthread_setspecific(
                _mi_heap_default_key as u32,
                (heap as *const ()) as *const core::ffi::c_void,
            ) as i32
        };
    }
}

// _PyInterpreterState_HasFeature
// _PyDebugAllocatorStats() definition
// _Py_FatalErrorFormat()
// _PyInterpreterState_GET
// OBJECT_STAT_INC_COND()
// malloc()
// for stats
// low-level allocator implementations
// the default raw allocator (wraps malloc)
// Defined in tracemalloc.c
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_RawMalloc")]
extern "C-unwind" fn _PyMem_RawMalloc(_unused_ctx: *mut (), mut size: u64) -> *mut () {
    // PyMem_RawMalloc(0) means malloc(1). Some systems would return NULL
    // for malloc(0), which would be treated as an error. Some platforms would
    // return a pointer with no memory behind it, which would break pymalloc.
    // To solve these problems, allocate an extra byte.
    if size == 0 {
        size = 1;
    }
    unsafe { malloc(size as usize) as *mut () }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_RawCalloc")]
extern "C-unwind" fn _PyMem_RawCalloc(
    _unused_ctx: *mut (),
    mut nelem: u64,
    mut elsize: u64,
) -> *mut () {
    // PyMem_RawCalloc(0, 0) means calloc(1, 1). Some systems would return NULL
    // for calloc(0, 0), which would be treated as an error. Some platforms
    // would return a pointer with no memory behind it, which would break
    // pymalloc.  To solve these problems, allocate an extra byte.
    if nelem == 0 || elsize == 0 {
        nelem = 1;
        elsize = 1;
    }
    unsafe { calloc(nelem as usize, elsize as usize) as *mut () }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_RawRealloc")]
extern "C-unwind" fn _PyMem_RawRealloc(
    _unused_ctx: *mut (),
    ptr: *mut (),
    mut size: u64,
) -> *mut () {
    if size == 0 {
        size = 1;
    }
    unsafe { realloc(ptr as *mut core::ffi::c_void, size as usize) as *mut () }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_RawFree")]
extern "C-unwind" fn _PyMem_RawFree(_unused_ctx: *mut (), ptr: *mut ()) {
    unsafe { free(ptr as *mut core::ffi::c_void) };
}

/// Forward declarations of functions used in our mimalloc modifications
fn _PyMem_mi_page_clear_qsbr(page: *mut mi_page_s) {}

/// Check if an empty, newly reclaimed page is safe to free now.
fn _PyMem_mi_page_is_safe_to_free(page: *mut mi_page_s) -> bool {
    true
}

fn _PyMem_mi_page_maybe_free(page: *mut mi_page_s, pq: *mut mi_page_queue_s, force: bool) -> bool {
    _mi_page_free(page, pq, force);
    true
}

fn _PyMem_mi_page_reclaimed(page: *mut mi_page_s) {}

fn _PyMem_mi_heap_collect_qsbr(heap: *mut mi_heap_s) {}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_MiMalloc")]
extern "C-unwind" fn _PyMem_MiMalloc(ctx: *mut (), size: u64) -> *mut () {
    mi_malloc(size)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_MiCalloc")]
extern "C-unwind" fn _PyMem_MiCalloc(ctx: *mut (), nelem: u64, elsize: u64) -> *mut () {
    mi_calloc(nelem, elsize)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_MiRealloc")]
extern "C-unwind" fn _PyMem_MiRealloc(ctx: *mut (), ptr: *mut (), size: u64) -> *mut () {
    mi_realloc(ptr, size)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_MiFree")]
extern "C-unwind" fn _PyMem_MiFree(ctx: *mut (), ptr: *mut ()) {
    mi_free(ptr);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_MiMalloc")]
extern "C-unwind" fn _PyObject_MiMalloc(ctx: *mut (), nbytes: u64) -> *mut () {
    mi_malloc(nbytes)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_MiCalloc")]
extern "C-unwind" fn _PyObject_MiCalloc(ctx: *mut (), nelem: u64, elsize: u64) -> *mut () {
    mi_calloc(nelem, elsize)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_MiRealloc")]
extern "C-unwind" fn _PyObject_MiRealloc(ctx: *mut (), ptr: *mut (), nbytes: u64) -> *mut () {
    mi_realloc(ptr, nbytes)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_MiFree")]
extern "C-unwind" fn _PyObject_MiFree(ctx: *mut (), ptr: *mut ()) {
    mi_free(ptr);
}

// the pymalloc allocator
// The actual implementation is further down.
// the default debug allocators
// The actual implementation is further down.
// default raw allocator (not swappable)
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_DefaultRawMalloc(size: u64) -> *mut () {
    _PyMem_RawMalloc(core::ptr::null_mut::<()>(), size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_DefaultRawCalloc(nelem: u64, elsize: u64) -> *mut () {
    _PyMem_RawCalloc(core::ptr::null_mut::<()>(), nelem, elsize)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_DefaultRawRealloc(ptr: *mut (), size: u64) -> *mut () {
    _PyMem_RawRealloc(core::ptr::null_mut::<()>(), ptr, size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_DefaultRawFree(ptr: *mut ()) {
    _PyMem_RawFree(core::ptr::null_mut::<()>(), ptr);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_DefaultRawWcsdup(str: *const i32) -> *mut i32 {
    let len: u64 = unsafe { wcslen(str as *const i32) as u64 };
    if len > (9223372036854775807 / (core::mem::size_of::<i32>() as u64)).wrapping_sub(1) {
        return core::ptr::null_mut::<i32>();
    }
    let size: u64 = len
        .wrapping_add(1)
        .wrapping_mul(core::mem::size_of::<i32>() as u64);
    let str2: *mut i32 = _PyMem_DefaultRawMalloc(size) as *mut i32;
    if str2.is_null() {
        return core::ptr::null_mut::<i32>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((str as *const ()) as *const core::ffi::c_void) as *const u8,
            ((str2 as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (size as usize) as usize,
        )
    };
    str2
}

// the low-level virtual memory allocator
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_ArenaAlloc")]
extern "C-unwind" fn _PyMem_ArenaAlloc(_unused_ctx: *mut (), size: u64) -> *mut () {
    let ptr: *mut () = unsafe {
        mmap(
            core::ptr::null_mut::<()>(),
            size,
            (slate_support::PROT_READ as i32) | (slate_support::PROT_WRITE as i32),
            (slate_support::MAP_PRIVATE as i32) | (slate_support::MAP_ANONYMOUS as i32),
            1i32.wrapping_neg(),
            0i64,
        )
    };
    if ptr == (1i32.wrapping_neg() as *mut ()) {
        return core::ptr::null_mut::<()>();
    }
    ptr
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_ArenaFree")]
extern "C-unwind" fn _PyMem_ArenaFree(_unused_ctx: *mut (), ptr: *mut (), size: u64) {
    // Unlike free(), munmap() does not special-case NULL to noop.
    if ptr.is_null() {
        return;
    }
    unsafe { munmap(ptr as *mut core::ffi::c_void, size as usize) as i32 };
}

// end low-level allocator implementations
// managing the allocators
fn set_default_allocator_unlocked(
    domain: u32,
    debug: i32,
    old_alloc: *mut __SlateRecord_pymem_h_48,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pymem_h_48 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pymem_h_48 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pymem_h_48 = unsafe { core::mem::zeroed() };
    if !old_alloc.is_null() {
        get_allocator_unlocked(domain, old_alloc);
    }
    let mut new_alloc: __SlateRecord_pymem_h_48 = unsafe { core::mem::zeroed() };
    match domain as u32 {
        0 => {
            new_alloc = unsafe {
                *{
                    __slate_compound_0 = __SlateRecord_pymem_h_48 {
                        ctx: core::ptr::null_mut::<()>(),
                        malloc: Some(_PyMem_RawMalloc),
                        calloc: Some(_PyMem_RawCalloc),
                        realloc: Some(_PyMem_RawRealloc),
                        free: Some(_PyMem_RawFree),
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        1 => {
            new_alloc = unsafe {
                *{
                    __slate_compound_2 = __SlateRecord_pymem_h_48 {
                        ctx: core::ptr::null_mut::<()>(),
                        malloc: Some(_PyObject_Malloc),
                        calloc: Some(_PyObject_Calloc),
                        realloc: Some(_PyObject_Realloc),
                        free: Some(_PyObject_Free),
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
        2 => {
            new_alloc = unsafe {
                *{
                    __slate_compound_3 = __SlateRecord_pymem_h_48 {
                        ctx: core::ptr::null_mut::<()>(),
                        malloc: Some(_PyObject_Malloc),
                        calloc: Some(_PyObject_Calloc),
                        realloc: Some(_PyObject_Realloc),
                        free: Some(_PyObject_Free),
                    };
                    core::ptr::addr_of_mut!(__slate_compound_3)
                }
            };
        }
        _ => {
            return 1i32.wrapping_neg(); // unknown domain
        }
    }
    set_allocator_unlocked(domain, core::ptr::addr_of_mut!(new_alloc));
    if debug != 0 {
        set_up_debug_hooks_domain_unlocked(domain);
    }
    0
}

static pydebug: i32 = 0;

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_GetAllocatorName(name: *const i8, allocator: *mut u32) -> i32 {
    if name.is_null() || ((unsafe { *name }) as i32) == 0 {
        // PYTHONMALLOC is empty or is not set or ignored (-E/-I command line
        // nameions): use default memory allocators
        unsafe {
            *allocator = 1;
        }
    } else {
        if (unsafe {
            strcmp(
                name as *const core::ffi::c_char,
                c"default".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            unsafe {
                *allocator = 1;
            }
        } else {
            if (unsafe {
                strcmp(
                    name as *const core::ffi::c_char,
                    c"debug".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                unsafe {
                    *allocator = 2;
                }
            } else {
                if (unsafe {
                    strcmp(
                        name as *const core::ffi::c_char,
                        c"pymalloc".as_ptr() as *const core::ffi::c_char,
                    ) as i32
                }) == 0
                {
                    unsafe {
                        *allocator = 5;
                    }
                } else {
                    if (unsafe {
                        strcmp(
                            name as *const core::ffi::c_char,
                            c"pymalloc_debug".as_ptr() as *const core::ffi::c_char,
                        ) as i32
                    }) == 0
                    {
                        unsafe {
                            *allocator = 6;
                        }
                    } else {
                        if (unsafe {
                            strcmp(
                                name as *const core::ffi::c_char,
                                c"mimalloc".as_ptr() as *const core::ffi::c_char,
                            ) as i32
                        }) == 0
                        {
                            unsafe {
                                *allocator = 7;
                            }
                        } else {
                            if (unsafe {
                                strcmp(
                                    name as *const core::ffi::c_char,
                                    c"mimalloc_debug".as_ptr() as *const core::ffi::c_char,
                                ) as i32
                            }) == 0
                            {
                                unsafe {
                                    *allocator = 8;
                                }
                            } else {
                                if (unsafe {
                                    strcmp(
                                        name as *const core::ffi::c_char,
                                        c"malloc".as_ptr() as *const core::ffi::c_char,
                                    ) as i32
                                }) == 0
                                {
                                    unsafe {
                                        *allocator = 3;
                                    }
                                } else {
                                    if (unsafe {
                                        strcmp(
                                            name as *const core::ffi::c_char,
                                            c"malloc_debug".as_ptr() as *const core::ffi::c_char,
                                        ) as i32
                                    }) == 0
                                    {
                                        unsafe {
                                            *allocator = 4;
                                        }
                                    } else {
                                        // unknown allocator
                                        return 1i32.wrapping_neg();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

fn set_up_allocators_unlocked(allocator: u32) -> i32 {
    '__slate_break_0: {
        match allocator as u32 {
            0 => {
                break '__slate_break_0; // do nothing
            }
            1 => {
                set_default_allocator_unlocked(
                    0u32,
                    unsafe { pydebug },
                    core::ptr::null_mut::<__SlateRecord_pymem_h_48>(),
                );
                set_default_allocator_unlocked(
                    1u32,
                    unsafe { pydebug },
                    core::ptr::null_mut::<__SlateRecord_pymem_h_48>(),
                );
                set_default_allocator_unlocked(
                    2u32,
                    unsafe { pydebug },
                    core::ptr::null_mut::<__SlateRecord_pymem_h_48>(),
                );
                unsafe {
                    _PyRuntime.allocators.is_debug_enabled = pydebug;
                }
            }
            2 => {
                set_default_allocator_unlocked(
                    0u32,
                    1i32,
                    core::ptr::null_mut::<__SlateRecord_pymem_h_48>(),
                );
                set_default_allocator_unlocked(
                    1u32,
                    1i32,
                    core::ptr::null_mut::<__SlateRecord_pymem_h_48>(),
                );
                set_default_allocator_unlocked(
                    2u32,
                    1i32,
                    core::ptr::null_mut::<__SlateRecord_pymem_h_48>(),
                );
                unsafe {
                    _PyRuntime.allocators.is_debug_enabled = 1;
                }
            }
            5 | 6 => {
                let mut malloc_alloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
                    ctx: core::ptr::null_mut::<()>(),
                    malloc: Some(_PyMem_RawMalloc),
                    calloc: Some(_PyMem_RawCalloc),
                    realloc: Some(_PyMem_RawRealloc),
                    free: Some(_PyMem_RawFree),
                };
                set_allocator_unlocked(0u32, core::ptr::addr_of_mut!(malloc_alloc));
                let mut pymalloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
                    ctx: core::ptr::null_mut::<()>(),
                    malloc: Some(_PyObject_Malloc),
                    calloc: Some(_PyObject_Calloc),
                    realloc: Some(_PyObject_Realloc),
                    free: Some(_PyObject_Free),
                };
                set_allocator_unlocked(1u32, core::ptr::addr_of_mut!(pymalloc));
                set_allocator_unlocked(2u32, core::ptr::addr_of_mut!(pymalloc));
                let is_debug: i32 = ((allocator as u32) == 6) as i32;
                unsafe {
                    _PyRuntime.allocators.is_debug_enabled = is_debug;
                }
                if is_debug != 0 {
                    set_up_debug_hooks_unlocked();
                }
            }
            7 | 8 => {
                let mut malloc_alloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
                    ctx: core::ptr::null_mut::<()>(),
                    malloc: Some(_PyMem_RawMalloc),
                    calloc: Some(_PyMem_RawCalloc),
                    realloc: Some(_PyMem_RawRealloc),
                    free: Some(_PyMem_RawFree),
                };
                set_allocator_unlocked(0u32, core::ptr::addr_of_mut!(malloc_alloc));
                let mut pymalloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
                    ctx: core::ptr::null_mut::<()>(),
                    malloc: Some(_PyMem_MiMalloc),
                    calloc: Some(_PyMem_MiCalloc),
                    realloc: Some(_PyMem_MiRealloc),
                    free: Some(_PyMem_MiFree),
                };
                set_allocator_unlocked(1u32, core::ptr::addr_of_mut!(pymalloc));
                let mut objmalloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
                    ctx: core::ptr::null_mut::<()>(),
                    malloc: Some(_PyObject_MiMalloc),
                    calloc: Some(_PyObject_MiCalloc),
                    realloc: Some(_PyObject_MiRealloc),
                    free: Some(_PyObject_MiFree),
                };
                set_allocator_unlocked(2u32, core::ptr::addr_of_mut!(objmalloc));
                let is_debug: i32 = ((allocator as u32) == 8) as i32;
                unsafe {
                    _PyRuntime.allocators.is_debug_enabled = is_debug;
                }
                if is_debug != 0 {
                    set_up_debug_hooks_unlocked();
                }
            }
            3 | 4 => {
                let mut malloc_alloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
                    ctx: core::ptr::null_mut::<()>(),
                    malloc: Some(_PyMem_RawMalloc),
                    calloc: Some(_PyMem_RawCalloc),
                    realloc: Some(_PyMem_RawRealloc),
                    free: Some(_PyMem_RawFree),
                };
                set_allocator_unlocked(0u32, core::ptr::addr_of_mut!(malloc_alloc));
                set_allocator_unlocked(1u32, core::ptr::addr_of_mut!(malloc_alloc));
                set_allocator_unlocked(2u32, core::ptr::addr_of_mut!(malloc_alloc));
                let is_debug: i32 = ((allocator as u32) == 4) as i32;
                unsafe {
                    _PyRuntime.allocators.is_debug_enabled = is_debug;
                }
                if is_debug != 0 {
                    set_up_debug_hooks_unlocked();
                }
            }
            _ => {
                return 1i32.wrapping_neg(); // unknown allocator
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_SetupAllocators(allocator: u32) -> i32 {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    let res: i32 = set_up_allocators_unlocked(allocator);
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    res
}

fn pymemallocator_eq(a: *mut __SlateRecord_pymem_h_48, b: &__SlateRecord_pymem_h_48) -> i32 {
    ((unsafe {
        memcmp(
            (a as *const ()) as *const core::ffi::c_void,
            ((b as *const __SlateRecord_pymem_h_48) as *const ()) as *const core::ffi::c_void,
            (core::mem::size_of::<__SlateRecord_pymem_h_48>() as u64) as usize,
        ) as i32
    }) == 0) as i32
}

fn get_current_allocator_name_unlocked() -> *const i8 {
    let mut malloc_alloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: core::ptr::null_mut::<()>(),
        malloc: Some(_PyMem_RawMalloc),
        calloc: Some(_PyMem_RawCalloc),
        realloc: Some(_PyMem_RawRealloc),
        free: Some(_PyMem_RawFree),
    };
    let mut pymalloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: core::ptr::null_mut::<()>(),
        malloc: Some(_PyObject_Malloc),
        calloc: Some(_PyObject_Calloc),
        realloc: Some(_PyObject_Realloc),
        free: Some(_PyObject_Free),
    };
    let mut mimalloc: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: core::ptr::null_mut::<()>(),
        malloc: Some(_PyMem_MiMalloc),
        calloc: Some(_PyMem_MiCalloc),
        realloc: Some(_PyMem_MiRealloc),
        free: Some(_PyMem_MiFree),
    };
    let mut mimalloc_obj: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: core::ptr::null_mut::<()>(),
        malloc: Some(_PyObject_MiMalloc),
        calloc: Some(_PyObject_MiCalloc),
        realloc: Some(_PyObject_MiRealloc),
        free: Some(_PyObject_MiFree),
    };
    let __v0: bool = pymemallocator_eq(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.raw) },
        unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
    ) != 0
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.mem) },
            unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
        ) != 0;
    let __v1: bool = __v0
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.obj) },
            unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
        ) != 0;
    if __v1 {
        return c"malloc".as_ptr();
    }
    let __v2: bool = pymemallocator_eq(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.raw) },
        unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
    ) != 0
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.mem) },
            unsafe { &(*core::ptr::addr_of_mut!(pymalloc)) },
        ) != 0;
    let __v3: bool = __v2
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.obj) },
            unsafe { &(*core::ptr::addr_of_mut!(pymalloc)) },
        ) != 0;
    if __v3 {
        return c"pymalloc".as_ptr();
    }
    let __v4: bool = pymemallocator_eq(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.raw) },
        unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
    ) != 0
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.mem) },
            unsafe { &(*core::ptr::addr_of_mut!(mimalloc)) },
        ) != 0;
    let __v5: bool = __v4
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.obj) },
            unsafe { &(*core::ptr::addr_of_mut!(mimalloc_obj)) },
        ) != 0;
    if __v5 {
        return c"mimalloc".as_ptr();
    }
    let mut dbg_raw: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: (unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.raw) }) as *mut (),
        malloc: Some(_PyMem_DebugRawMalloc),
        calloc: Some(_PyMem_DebugRawCalloc),
        realloc: Some(_PyMem_DebugRawRealloc),
        free: Some(_PyMem_DebugRawFree),
    };
    let mut dbg_mem: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: (unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.mem) }) as *mut (),
        malloc: Some(_PyMem_DebugMalloc),
        calloc: Some(_PyMem_DebugCalloc),
        realloc: Some(_PyMem_DebugRealloc),
        free: Some(_PyMem_DebugFree),
    };
    let mut dbg_obj: __SlateRecord_pymem_h_48 = __SlateRecord_pymem_h_48 {
        ctx: (unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.obj) }) as *mut (),
        malloc: Some(_PyMem_DebugMalloc),
        calloc: Some(_PyMem_DebugCalloc),
        realloc: Some(_PyMem_DebugRealloc),
        free: Some(_PyMem_DebugFree),
    };
    let __v6: bool = pymemallocator_eq(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.raw) },
        unsafe { &(*core::ptr::addr_of_mut!(dbg_raw)) },
    ) != 0
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.mem) },
            unsafe { &(*core::ptr::addr_of_mut!(dbg_mem)) },
        ) != 0;
    let __v7: bool = __v6
        && pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.standard.obj) },
            unsafe { &(*core::ptr::addr_of_mut!(dbg_obj)) },
        ) != 0;
    if __v7 {
        // Debug hooks installed
        let __v8: bool = pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.raw.alloc) },
            unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
        ) != 0
            && pymemallocator_eq(
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.mem.alloc) },
                unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
            ) != 0;
        let __v9: bool = __v8
            && pymemallocator_eq(
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.obj.alloc) },
                unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
            ) != 0;
        if __v9 {
            return c"malloc_debug".as_ptr();
        }
        let __v10: bool = pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.raw.alloc) },
            unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
        ) != 0
            && pymemallocator_eq(
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.mem.alloc) },
                unsafe { &(*core::ptr::addr_of_mut!(pymalloc)) },
            ) != 0;
        let __v11: bool = __v10
            && pymemallocator_eq(
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.obj.alloc) },
                unsafe { &(*core::ptr::addr_of_mut!(pymalloc)) },
            ) != 0;
        if __v11 {
            return c"pymalloc_debug".as_ptr();
        }
        let __v12: bool = pymemallocator_eq(
            unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.raw.alloc) },
            unsafe { &(*core::ptr::addr_of_mut!(malloc_alloc)) },
        ) != 0
            && pymemallocator_eq(
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.mem.alloc) },
                unsafe { &(*core::ptr::addr_of_mut!(mimalloc)) },
            ) != 0;
        let __v13: bool = __v12
            && pymemallocator_eq(
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.obj.alloc) },
                unsafe { &(*core::ptr::addr_of_mut!(mimalloc_obj)) },
            ) != 0;
        if __v13 {
            return c"mimalloc_debug".as_ptr();
        }
    }
    core::ptr::null::<i8>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_GetCurrentAllocatorName() -> *const i8 {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    let name: *const i8 = get_current_allocator_name_unlocked();
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    name
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_DebugEnabled() -> i32 {
    unsafe { _PyRuntime.allocators.is_debug_enabled }
}

fn _PyMem_PymallocEnabled() -> i32 {
    if _PyMem_DebugEnabled() != 0 {
        return ((unsafe { _PyRuntime.allocators.debug.obj.alloc.malloc }) == Some(_PyObject_Malloc))
            as i32;
    } else {
        return ((unsafe { _PyRuntime.allocators.standard.obj.malloc }) == Some(_PyObject_Malloc))
            as i32;
    }
}

fn _PyMem_MimallocEnabled() -> i32 {
    if _PyMem_DebugEnabled() != 0 {
        return ((unsafe { _PyRuntime.allocators.debug.obj.alloc.malloc })
            == Some(_PyObject_MiMalloc)) as i32;
    } else {
        return ((unsafe { _PyRuntime.allocators.standard.obj.malloc }) == Some(_PyObject_MiMalloc))
            as i32;
    }
}

fn set_up_debug_hooks_domain_unlocked(domain: u32) {
    let mut alloc: __SlateRecord_pymem_h_48 = unsafe { core::mem::zeroed() };
    if (domain as u32) == 0 {
        if (unsafe { _PyRuntime.allocators.standard.raw.malloc }) == Some(_PyMem_DebugRawMalloc) {
            return;
        }
        get_allocator_unlocked(domain, unsafe {
            core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.raw.alloc)
        });
        alloc.ctx =
            (unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.raw) }) as *mut ();
        alloc.malloc = Some(_PyMem_DebugRawMalloc);
        alloc.calloc = Some(_PyMem_DebugRawCalloc);
        alloc.realloc = Some(_PyMem_DebugRawRealloc);
        alloc.free = Some(_PyMem_DebugRawFree);
        set_allocator_unlocked(domain, core::ptr::addr_of_mut!(alloc));
    } else {
        if (domain as u32) == 1 {
            if (unsafe { _PyRuntime.allocators.standard.mem.malloc }) == Some(_PyMem_DebugMalloc) {
                return;
            }
            get_allocator_unlocked(domain, unsafe {
                core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.mem.alloc)
            });
            alloc.ctx =
                (unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.mem) }) as *mut ();
            alloc.malloc = Some(_PyMem_DebugMalloc);
            alloc.calloc = Some(_PyMem_DebugCalloc);
            alloc.realloc = Some(_PyMem_DebugRealloc);
            alloc.free = Some(_PyMem_DebugFree);
            set_allocator_unlocked(domain, core::ptr::addr_of_mut!(alloc));
        } else {
            if (domain as u32) == 2 {
                if (unsafe { _PyRuntime.allocators.standard.obj.malloc })
                    == Some(_PyMem_DebugMalloc)
                {
                    return;
                }
                get_allocator_unlocked(domain, unsafe {
                    core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.obj.alloc)
                });
                alloc.ctx = (unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.debug.obj) })
                    as *mut ();
                alloc.malloc = Some(_PyMem_DebugMalloc);
                alloc.calloc = Some(_PyMem_DebugCalloc);
                alloc.realloc = Some(_PyMem_DebugRealloc);
                alloc.free = Some(_PyMem_DebugFree);
                set_allocator_unlocked(domain, core::ptr::addr_of_mut!(alloc));
            }
        }
    }
}

fn set_up_debug_hooks_unlocked() {
    set_up_debug_hooks_domain_unlocked(0u32);
    set_up_debug_hooks_domain_unlocked(1u32);
    set_up_debug_hooks_domain_unlocked(2u32);
    unsafe {
        _PyRuntime.allocators.is_debug_enabled = 1;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_SetupDebugHooks() {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    set_up_debug_hooks_unlocked();
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
}

fn get_allocator_unlocked(domain: u32, allocator: *mut __SlateRecord_pymem_h_48) {
    match domain as u32 {
        0 => unsafe {
            *allocator = _PyRuntime.allocators.standard.raw;
        },
        1 => unsafe {
            *allocator = _PyRuntime.allocators.standard.mem;
        },
        2 => unsafe {
            *allocator = _PyRuntime.allocators.standard.obj;
        },
        _ => {
            unsafe {
                (*allocator).ctx = core::ptr::null_mut::<()>();
            }
            // unknown domain: set all attributes to NULL
            unsafe {
                (*allocator).malloc = None;
            }
            unsafe {
                (*allocator).calloc = None;
            }
            unsafe {
                (*allocator).realloc = None;
            }
            unsafe {
                (*allocator).free = None;
            }
        }
    }
}

fn set_allocator_unlocked(domain: u32, allocator: *mut __SlateRecord_pymem_h_48) {
    '__slate_break_0: {
        match domain as u32 {
            0 => unsafe {
                _PyRuntime.allocators.standard.raw = *allocator;
            },
            1 => unsafe {
                _PyRuntime.allocators.standard.mem = *allocator;
            },
            2 => {
                unsafe {
                    _PyRuntime.allocators.standard.obj = *allocator;
                }
                break '__slate_break_0;
                // ignore unknown domain
            }
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_GetAllocator(domain: u32, allocator: *mut __SlateRecord_pymem_h_48) {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    get_allocator_unlocked(domain, allocator);
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_SetAllocator(domain: u32, allocator: *mut __SlateRecord_pymem_h_48) {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    set_allocator_unlocked(domain, allocator);
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_GetArenaAllocator(allocator: *mut __SlateRecord_objimpl_h_68) {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    unsafe {
        *allocator = _PyRuntime.allocators.obj_arena;
    }
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_SetArenaAllocator(allocator: *mut __SlateRecord_objimpl_h_68) {
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
    unsafe {
        _PyRuntime.allocators.obj_arena = *allocator;
    }
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!(_PyRuntime.allocators.mutex) });
}

// Note that there is a possible, but very unlikely, race in any place
// below where we call one of the allocator functions.  We access two
// fields in each case:  "malloc", etc. and "ctx".
//
// It is unlikely that the allocator will be changed while one of those
// calls is happening, much less in that very narrow window.
// Furthermore, the likelihood of a race is drastically reduced by the
// fact that the allocator may not be changed after runtime init
// (except with a wrapper).
//
// With the above in mind, we currently don't worry about locking
// around these uses of the runtime-global allocators state.
// the "arena" allocator
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_VirtualAlloc(size: u64) -> *mut () {
    unsafe {
        unsafe { _PyRuntime.allocators.obj_arena.alloc }.unwrap()(
            _PyRuntime.allocators.obj_arena.ctx,
            size,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_VirtualFree(obj: *mut (), size: u64) {
    unsafe {
        unsafe { _PyRuntime.allocators.obj_arena.free }.unwrap()(
            _PyRuntime.allocators.obj_arena.ctx,
            obj,
            size,
        )
    };
}

// the "raw" allocator
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc.PyMem_RawMalloc")]
extern "C-unwind" fn PyMem_RawMalloc(size: u64) -> *mut () {
    // Limit ourselves to PY_SSIZE_T_MAX bytes to prevent security holes.
    // Most python internals blindly use a signed Py_ssize_t to track
    // things without checking for overflows or negatives.
    // As size_t is unsigned, checking for size < 0 is not required.
    if size > 9223372036854775807 {
        return core::ptr::null_mut::<()>();
    }
    unsafe {
        unsafe { _PyRuntime.allocators.standard.raw.malloc }.unwrap()(
            _PyRuntime.allocators.standard.raw.ctx,
            size,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_RawCalloc(nelem: u64, elsize: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if elsize != 0 && nelem > 9223372036854775807 / elsize {
        return core::ptr::null_mut::<()>();
    }
    unsafe {
        unsafe { _PyRuntime.allocators.standard.raw.calloc }.unwrap()(
            _PyRuntime.allocators.standard.raw.ctx,
            nelem,
            elsize,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_RawRealloc(ptr: *mut (), new_size: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if new_size > 9223372036854775807 {
        return core::ptr::null_mut::<()>();
    }
    unsafe {
        unsafe { _PyRuntime.allocators.standard.raw.realloc }.unwrap()(
            _PyRuntime.allocators.standard.raw.ctx,
            ptr,
            new_size,
        )
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc.PyMem_RawFree")]
extern "C-unwind" fn PyMem_RawFree(ptr: *mut ()) {
    unsafe {
        unsafe { _PyRuntime.allocators.standard.raw.free }.unwrap()(
            _PyRuntime.allocators.standard.raw.ctx,
            ptr,
        )
    };
}

// the "mem" allocator
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc.PyMem_Malloc")]
extern "C-unwind" fn PyMem_Malloc(size: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if size > 9223372036854775807 {
        return core::ptr::null_mut::<()>();
    }
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC!();
    unsafe {
        unsafe { _PyRuntime.allocators.standard.mem.malloc }.unwrap()(
            _PyRuntime.allocators.standard.mem.ctx,
            size,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_Calloc(nelem: u64, elsize: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if elsize != 0 && nelem > 9223372036854775807 / elsize {
        return core::ptr::null_mut::<()>();
    }
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC!();
    unsafe {
        unsafe { _PyRuntime.allocators.standard.mem.calloc }.unwrap()(
            _PyRuntime.allocators.standard.mem.ctx,
            nelem,
            elsize,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMem_Realloc(ptr: *mut (), new_size: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if new_size > 9223372036854775807 {
        return core::ptr::null_mut::<()>();
    }
    unsafe {
        unsafe { _PyRuntime.allocators.standard.mem.realloc }.unwrap()(
            _PyRuntime.allocators.standard.mem.ctx,
            ptr,
            new_size,
        )
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc.PyMem_Free")]
extern "C-unwind" fn PyMem_Free(ptr: *mut ()) {
    OBJECT_STAT_INC!();
    unsafe {
        unsafe { _PyRuntime.allocators.standard.mem.free }.unwrap()(
            _PyRuntime.allocators.standard.mem.ctx,
            ptr,
        )
    };
}

// pymem utility functions
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_RawWcsdup(str: *const i32) -> *mut i32 {
    let len: u64 = unsafe { wcslen(str as *const i32) as u64 };
    if len > (9223372036854775807 / (core::mem::size_of::<i32>() as u64)).wrapping_sub(1) {
        return core::ptr::null_mut::<i32>();
    }
    let size: u64 = len
        .wrapping_add(1)
        .wrapping_mul(core::mem::size_of::<i32>() as u64);
    let str2: *mut i32 = PyMem_RawMalloc(size) as *mut i32;
    if str2.is_null() {
        return core::ptr::null_mut::<i32>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((str as *const ()) as *const core::ffi::c_void) as *const u8,
            ((str2 as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (size as usize) as usize,
        )
    };
    str2
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_RawStrdup(str: *const i8) -> *mut i8 {
    let size: u64 = unsafe { strlen(str as *const core::ffi::c_char) as u64 }.wrapping_add(1);
    let copy: *mut i8 = PyMem_RawMalloc(size) as *mut i8;
    if copy.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((str as *const ()) as *const core::ffi::c_void) as *const u8,
            ((copy as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (size as usize) as usize,
        )
    };
    copy
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_Strdup(str: *const i8) -> *mut i8 {
    let size: u64 = unsafe { strlen(str as *const core::ffi::c_char) as u64 }.wrapping_add(1);
    let copy: *mut i8 = PyMem_Malloc(size) as *mut i8;
    if copy.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((str as *const ()) as *const core::ffi::c_void) as *const u8,
            ((copy as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (size as usize) as usize,
        )
    };
    copy
}

// Delayed freeing support for Py_GIL_DISABLED
// So that sizeof(struct _mem_work_chunk) is 4096 bytes on 64-bit platforms.
/// A pointer to be freed once the QSBR read sequence reaches qsbr_goal.
#[repr(C)]
#[derive(Clone, Copy)]
struct _mem_work_item {
    /// lowest bit tagged 1 for objects freed with PyObject_Free
    ptr: u64,
    qsbr_goal: u64,
}

/// A fixed-size buffer of pointers to be freed
#[repr(C)]
#[derive(Clone, Copy)]
struct _mem_work_chunk {
    /// Linked list node of chunks in queue
    node: llist_node,
    /// index of next item to read
    rd_idx: i64,
    /// index of next item to write
    wr_idx: i64,
    array: [_mem_work_item; 254],
}

fn work_item_should_decref(ptr: u64) -> i32 {
    (ptr & 1) as i32
}

fn free_work_item(
    ptr: u64,
    cb: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ())>,
    state: *mut (),
) {
    if work_item_should_decref(ptr) != 0 {
        let obj: *mut _object = ptr.wrapping_sub(1) as *mut _object;
        Py_DECREF!(obj);
    } else {
        PyMem_Free(ptr as *mut ());
    }
}

fn free_delayed(ptr: u64, size: u64) {
    free_work_item(ptr, None, core::ptr::null_mut::<()>());
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_FreeDelayed(ptr: *mut (), size: u64) {
    if !ptr.is_null() {
        free_delayed(ptr as u64, size);
    }
}

fn work_queue_first(head: *mut llist_node) -> *mut _mem_work_chunk {
    llist_data!(unsafe { (*head).next }, _mem_work_chunk)
}

fn process_queue(
    head: *mut llist_node,
    tstate: *mut _PyThreadStateImpl,
    keep_empty: bool,
    cb: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ())>,
    state: *mut (),
) {
    while llist_empty(head) == 0 {
        let buf: *mut _mem_work_chunk = work_queue_first(head);
        if (unsafe { (*buf).rd_idx }) < unsafe { (*buf).wr_idx } {
            let item: *mut _mem_work_item =
                unsafe { (*buf).array.as_mut_ptr() as *mut _mem_work_item }
                    .wrapping_offset((unsafe { (*buf).rd_idx }) as isize);
            if !unsafe { _Py_qsbr_poll((*tstate).qsbr, (*item).qsbr_goal) } {
                return;
            }
            let __v1: i64 = unsafe { (*buf).rd_idx };
            unsafe {
                (*buf).rd_idx = __v1.wrapping_add(1);
            }
            // NB: free_work_item may re-enter or execute arbitrary code
            free_work_item(unsafe { (*item).ptr }, cb, state);
        } else {
            if keep_empty && (unsafe { (*buf).node.next }) == head {
                // Keep the last buffer in the queue to reduce re-allocations
                unsafe {
                    (*buf).wr_idx = 0;
                }
                unsafe {
                    (*buf).rd_idx = 0;
                }
                return;
            }
            llist_remove(unsafe { core::ptr::addr_of_mut!((*buf).node) });
            PyMem_Free(buf as *mut ());
        }
    }
}

fn process_interp_queue(
    queue: *mut _Py_mem_interp_free_queue,
    tstate: *mut _PyThreadStateImpl,
    cb: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ())>,
    state: *mut (),
) {
    process_queue(
        unsafe { core::ptr::addr_of_mut!((*queue).head) },
        tstate,
        false,
        cb,
        state,
    );
    let more_work: i32 =
        (llist_empty(unsafe { core::ptr::addr_of_mut!((*queue).head) }) == 0) as i32;
    _Py_atomic_store_int_relaxed(
        unsafe { core::ptr::addr_of_mut!((*queue).has_work) },
        more_work,
    );
}

fn maybe_process_interp_queue(
    queue: *mut _Py_mem_interp_free_queue,
    tstate: *mut _PyThreadStateImpl,
    cb: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ())>,
    state: *mut (),
) {
    if _Py_atomic_load_int_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*queue).has_work) }) as *const i32,
    ) == 0
    {
        return;
    }
    // Try to acquire the lock, but don't block if it's already held.
    if ((unsafe { _PyMutex_LockTimed(core::ptr::addr_of_mut!((*queue).mutex), 0i64, 0u32) }) as u32)
        == 1
    {
        process_interp_queue(queue, tstate, cb, state);
        _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*queue).mutex) });
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_ProcessDelayed(tstate: *mut _ts) {
    let interp: *mut _is = unsafe { (*tstate).interp };
    let tstate_impl: *mut _PyThreadStateImpl = tstate as *mut _PyThreadStateImpl;
    unsafe {
        (*(*tstate_impl).qsbr).should_process = false;
    }
    // Process thread-local work
    process_queue(
        unsafe { core::ptr::addr_of_mut!((*tstate_impl).mem_free_queue) },
        tstate_impl,
        true,
        None,
        core::ptr::null_mut::<()>(),
    );
    // Process shared interpreter work
    maybe_process_interp_queue(
        unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue) },
        tstate_impl,
        None,
        core::ptr::null_mut::<()>(),
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_ProcessDelayedNoDealloc(
    tstate: *mut _ts,
    cb: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ())>,
    state: *mut (),
) {
    let interp: *mut _is = unsafe { (*tstate).interp };
    let tstate_impl: *mut _PyThreadStateImpl = tstate as *mut _PyThreadStateImpl;
    // Process thread-local work
    process_queue(
        unsafe { core::ptr::addr_of_mut!((*tstate_impl).mem_free_queue) },
        tstate_impl,
        true,
        cb,
        state,
    );
    // Process shared interpreter work
    maybe_process_interp_queue(
        unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue) },
        tstate_impl,
        cb,
        state,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_AbandonDelayed(tstate: *mut _ts) {
    let interp: *mut _is = unsafe { (*tstate).interp };
    let queue: *mut llist_node =
        unsafe { core::ptr::addr_of_mut!((*(tstate as *mut _PyThreadStateImpl)).mem_free_queue) };
    if llist_empty(queue) != 0 {
        return;
    }
    // Check if the queue contains one empty buffer
    let buf: *mut _mem_work_chunk = work_queue_first(queue);
    if (unsafe { (*buf).rd_idx }) == unsafe { (*buf).wr_idx } {
        llist_remove(unsafe { core::ptr::addr_of_mut!((*buf).node) });
        PyMem_Free(buf as *mut ());
        return;
    }
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue.mutex) });
    // Merge the thread's work queue into the interpreter's work queue.
    llist_concat(
        unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue.head) },
        queue,
    );
    // Process the merged queue now (see gh-130794).
    let this_tstate: *mut _PyThreadStateImpl = _PyThreadState_GET() as *mut _PyThreadStateImpl;
    process_interp_queue(
        unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue) },
        this_tstate,
        None,
        core::ptr::null_mut::<()>(),
    );
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue.mutex) });
    // the thread's queue is now empty
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_FiniDelayed(interp: *mut _is) {
    let head: *mut llist_node = unsafe { core::ptr::addr_of_mut!((*interp).mem_free_queue.head) };
    while llist_empty(head) == 0 {
        let buf: *mut _mem_work_chunk = work_queue_first(head);
        if (unsafe { (*buf).rd_idx }) < unsafe { (*buf).wr_idx } {
            // Free the remaining items immediately. There should be no other
            //
            // threads accessing the memory at this point during shutdown.
            let item: *mut _mem_work_item =
                unsafe { (*buf).array.as_mut_ptr() as *mut _mem_work_item }
                    .wrapping_offset((unsafe { (*buf).rd_idx }) as isize);
            let __v1: i64 = unsafe { (*buf).rd_idx };
            unsafe {
                (*buf).rd_idx = __v1.wrapping_add(1);
            }
            // NB: free_work_item may re-enter or execute arbitrary code
            free_work_item(unsafe { (*item).ptr }, None, core::ptr::null_mut::<()>());
        } else {
            llist_remove(unsafe { core::ptr::addr_of_mut!((*buf).node) });
            PyMem_Free(buf as *mut ());
        }
    }
}

// the "object" allocator
#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_Malloc(size: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if size > 9223372036854775807 {
        return core::ptr::null_mut::<()>();
    }
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC!();
    unsafe {
        unsafe { _PyRuntime.allocators.standard.obj.malloc }.unwrap()(
            _PyRuntime.allocators.standard.obj.ctx,
            size,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_Calloc(nelem: u64, elsize: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if elsize != 0 && nelem > 9223372036854775807 / elsize {
        return core::ptr::null_mut::<()>();
    }
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC_COND!();
    OBJECT_STAT_INC!();
    unsafe {
        unsafe { _PyRuntime.allocators.standard.obj.calloc }.unwrap()(
            _PyRuntime.allocators.standard.obj.ctx,
            nelem,
            elsize,
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_Realloc(ptr: *mut (), new_size: u64) -> *mut () {
    // see PyMem_RawMalloc()
    if new_size > 9223372036854775807 {
        return core::ptr::null_mut::<()>();
    }
    unsafe {
        unsafe { _PyRuntime.allocators.standard.obj.realloc }.unwrap()(
            _PyRuntime.allocators.standard.obj.ctx,
            ptr,
            new_size,
        )
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc.PyObject_Free")]
extern "C-unwind" fn PyObject_Free(ptr: *mut ()) {
    OBJECT_STAT_INC!();
    unsafe {
        unsafe { _PyRuntime.allocators.standard.obj.free }.unwrap()(
            _PyRuntime.allocators.standard.obj.ctx,
            ptr,
        )
    };
}

// If we're using GCC, use __builtin_expect() to reduce overhead of
// the valgrind checks
/// obmalloc state for main interpreter and shared by all interpreters without
/// their own obmalloc state.  By not explicitly initializing this structure, it
/// will be allocated in the BSS which is a small performance win.  The radix
/// tree arrays are fairly large but are sparsely used.
static mut obmalloc_state_main: _obmalloc_state = unsafe { core::mem::zeroed() };

static mut obmalloc_state_initialized: bool = unsafe { core::mem::zeroed() };

fn has_own_state(interp: *mut _is) -> i32 {
    let __v0: bool = _Py_IsMainInterpreter(interp) != 0
        || (unsafe { (*interp).feature_flags }) & 32u64 == 0
        || (unsafe { _Py_IsMainInterpreterFinalizing(interp) }) != 0;
    __v0 as i32
}

fn get_state() -> *mut _obmalloc_state {
    let interp: *mut _is = _PyInterpreterState_GET();
    // otherwise not initialized or freed
    unsafe { (*interp).obmalloc }
}

// These macros all rely on a local "state" variable.
#[unsafe(link_section = ".text.slate_distinct.obmalloc.count_blocks")]
extern "C-unwind" fn count_blocks(
    heap: *const mi_heap_s,
    area: *const mi_heap_area_s,
    block: *mut (),
    block_size: u64,
    allocated_blocks: *mut (),
) -> bool {
    let __v1: u64 = unsafe { *(allocated_blocks as *mut u64) };
    unsafe {
        *(allocated_blocks as *mut u64) = __v1.wrapping_add((*area).used);
    }
    true
}

fn get_mimalloc_allocated_blocks(interp: *mut _is) -> i64 {
    let mut allocated_blocks: u64 = 0;
    // TODO(sgross): this only counts the current thread's blocks.
    let heap: *mut mi_heap_s = mi_heap_get_default();
    mi_heap_visit_blocks(
        heap as *const mi_heap_s,
        false,
        Some(count_blocks),
        core::ptr::addr_of_mut!(allocated_blocks) as *mut (),
    );
    allocated_blocks as i64
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyInterpreterState_GetAllocatedBlocks(interp: *mut _is) -> i64 {
    if _PyMem_MimallocEnabled() != 0 {
        return get_mimalloc_allocated_blocks(interp);
    }
    if has_own_state(interp) == 0 {
        unsafe {
            _Py_FatalErrorFunc(
                c"_PyInterpreterState_GetAllocatedBlocks".as_ptr(),
                c"the interpreter doesn't have its own allocator".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    }
    let state: *mut _obmalloc_state = unsafe { (*interp).obmalloc };
    if state.is_null() {
        return 0;
    }
    let mut n: i64 = unsafe { (*state).mgmt.raw_allocated_blocks };
    // add up allocated blocks for used pools
    let mut i: u32 = 0;
    while i < unsafe { (*state).mgmt.maxarenas } {
        // Skip arenas which are not allocated.
        if (unsafe { (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).address }) != 0
        {
            let mut base: u64 = _Py_ALIGN_UP!(
                unsafe { (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).address },
                16384i32
            ) as u64;
            // visit every pool in the arena
            while base
                < ((unsafe {
                    (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).pool_address
                }) as u64)
            {
                n = n.wrapping_add((unsafe { (*(base as *mut pool_header)).r#ref.count }) as i64);
                base = base.wrapping_add(16384u64);
            }
        }
        i = i.wrapping_add(1);
    }
    n
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyInterpreterState_FinalizeAllocatedBlocks(interp: *mut _is) {
    if _PyMem_MimallocEnabled() != 0 {
        let leaked: i64 = _PyInterpreterState_GetAllocatedBlocks(interp);
        let __v0: *mut pyruntimestate = unsafe { (*interp).runtime };
        let __v1: i64 = unsafe { (*__v0).obmalloc.interpreter_leaks };
        unsafe {
            (*__v0).obmalloc.interpreter_leaks = __v1.wrapping_add(leaked);
        }
        return;
    }
    if has_own_state(interp) != 0 && !unsafe { (*interp).obmalloc }.is_null() {
        let leaked: i64 = _PyInterpreterState_GetAllocatedBlocks(interp);
        let __v3: *mut pyruntimestate = unsafe { (*interp).runtime };
        let __v4: i64 = unsafe { (*__v3).obmalloc.interpreter_leaks };
        unsafe {
            (*__v3).obmalloc.interpreter_leaks = __v4.wrapping_add(leaked);
        }
        if _PyMem_obmalloc_state_on_heap(interp) && leaked == 0 {
            // free the obmalloc arenas and radix tree nodes.  If leaked > 0
            //
            // then some of the memory allocated by obmalloc has not been
            //
            // freed.  It might be safe to free the arenas in that case but
            //
            // it's possible that extension modules are still using that
            //
            // memory.  So, it is safer to not free and to leak.  Perhaps there
            //
            // should be warning when this happens.  It should be possible to
            //
            // use a tool like "-fsanitize=address" to track down these leaks.
            free_obmalloc_arenas(interp);
        }
    }
}

/// We preserve the number of blocks leaked during runtime finalization,
/// so they can be reported if the runtime is initialized again.
///
/// XXX We don't lose any information by dropping this,
///
/// so we should consider doing so.
static mut last_final_leaks: i64 = 0;

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_FinalizeAllocatedBlocks(runtime: *mut pyruntimestate) {
    unsafe {
        last_final_leaks = get_num_global_allocated_blocks(runtime);
    }
    unsafe {
        (*runtime).obmalloc.interpreter_leaks = 0;
    }
}

fn get_num_global_allocated_blocks(runtime: *mut pyruntimestate) -> i64 {
    let mut total: i64 = 0;
    if !_PyRuntimeState_GetFinalizing(runtime).is_null() {
        let interp: *mut _is = unsafe { _PyInterpreterState_Main() };
        if interp.is_null() {
            // We are at the very end of runtime finalization.
            // We can't rely on finalizing->interp since that thread
            // state is probably already freed, so we don't worry
            // about it.
        } else {
            // It is probably the last interpreter but not necessarily.
            let __v0: i64 = _PyInterpreterState_GetAllocatedBlocks(interp);
            total = total.wrapping_add(__v0);
        }
    } else {
        unsafe { _PyEval_StopTheWorldAll(core::ptr::addr_of_mut!(_PyRuntime)) };
        HEAD_LOCK!(runtime);
        let mut interp: *mut _is = unsafe { PyInterpreterState_Head() };
        while !interp.is_null() {
            if has_own_state(interp) != 0 {
                let __v3: i64 = _PyInterpreterState_GetAllocatedBlocks(interp);
                total = total.wrapping_add(__v3);
            }
            interp = unsafe { PyInterpreterState_Next(interp) };
        }
        HEAD_UNLOCK!(runtime);
        unsafe { _PyEval_StartTheWorldAll(core::ptr::addr_of_mut!(_PyRuntime)) };
    }
    total = total.wrapping_add(unsafe { (*runtime).obmalloc.interpreter_leaks });
    total = total.wrapping_add(unsafe { last_final_leaks });
    total
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_GetGlobalAllocatedBlocks() -> i64 {
    get_num_global_allocated_blocks(unsafe { core::ptr::addr_of_mut!(_PyRuntime) })
}

// radix tree for tracking arena usage.
/// Return a pointer to a bottom tree node, return NULL if it doesn't exist or
/// it cannot be created
#[inline(always)]
fn arena_map_get(state: *mut _obmalloc_state, p: *mut u8, create: i32) -> *mut arena_map_bot {
    // sanity check that IGNORE_BITS is correct
    let i1: i32 = MAP_TOP_INDEX!(p) as i32;
    if unsafe {
        *((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
            .wrapping_offset(i1 as isize)
    }
    .is_null()
    {
        if create == 0 {
            return core::ptr::null_mut::<arena_map_bot>();
        }
        let n: *mut arena_map_mid =
            PyMem_RawCalloc(1u64, core::mem::size_of::<arena_map_mid>() as u64)
                as *mut arena_map_mid;
        if n.is_null() {
            return core::ptr::null_mut::<arena_map_bot>();
        }
        unsafe {
            *((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
                .wrapping_offset(i1 as isize) = n;
        }
        let __v1: i32 = unsafe { (*state).usage.arena_map_mid_count };
        unsafe {
            (*state).usage.arena_map_mid_count = __v1.wrapping_add(1);
        }
    }
    let i2: i32 = MAP_MID_INDEX!(p) as i32;
    if unsafe {
        *((*(*((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
            .wrapping_offset(i1 as isize)))
        .ptrs
        .as_mut_ptr() as *mut *mut arena_map_bot)
            .wrapping_offset(i2 as isize)
    }
    .is_null()
    {
        if create == 0 {
            return core::ptr::null_mut::<arena_map_bot>();
        }
        let n: *mut arena_map_bot =
            PyMem_RawCalloc(1u64, core::mem::size_of::<arena_map_bot>() as u64)
                as *mut arena_map_bot;
        if n.is_null() {
            return core::ptr::null_mut::<arena_map_bot>();
        }
        unsafe {
            *((*(*((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
                .wrapping_offset(i1 as isize)))
            .ptrs
            .as_mut_ptr() as *mut *mut arena_map_bot)
                .wrapping_offset(i2 as isize) = n;
        }
        let __v4: i32 = unsafe { (*state).usage.arena_map_bot_count };
        unsafe {
            (*state).usage.arena_map_bot_count = __v4.wrapping_add(1);
        }
    }
    unsafe {
        *((*(*((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
            .wrapping_offset(i1 as isize)))
        .ptrs
        .as_mut_ptr() as *mut *mut arena_map_bot)
            .wrapping_offset(i2 as isize)
    }
}

// The radix tree only tracks arenas.  So, for 16 MiB arenas, we throw
// away 24 bits of the address.  That reduces the space requirement of
// the tree compared to similar radix tree page-map schemes.  In
// exchange for slashing the space requirement, it needs more
// computation to check an address.
//
// Tracking coverage is done by "ideal" arena address.  It is easier to
// explain in decimal so let's say that the arena size is 100 bytes.
// Then, ideal addresses are 100, 200, 300, etc.  For checking if a
// pointer address is inside an actual arena, we have to check two ideal
// arena addresses.  E.g. if pointer is 357, we need to check 200 and
// 300.  In the rare case that an arena is aligned in the ideal way
// (e.g. base address of arena is 200) then we only have to check one
// ideal address.
//
// The tree nodes for 200 and 300 both store the address of arena.
// There are two cases: the arena starts at a lower ideal arena and
// extends to this one, or the arena starts in this arena and extends to
// the next ideal arena.  The tail_lo and tail_hi members correspond to
// these two cases.
/// mark or unmark addresses covered by arena
fn arena_map_mark_used(state: *mut _obmalloc_state, arena_base: u64, is_used: i32) -> i32 {
    // sanity check that IGNORE_BITS is correct
    let n_hi: *mut arena_map_bot = arena_map_get(state, arena_base as *mut u8, is_used);
    if n_hi.is_null() {
        // otherwise node should already exist
        return 0; // failed to allocate space for node
    }
    let i3: i32 = MAP_BOT_INDEX!(arena_base as *mut u8) as i32;
    let tail: i32 = (arena_base & 1048575u64) as i32;
    if tail == 0 {
        // is ideal arena address
        unsafe {
            (*((*n_hi).arenas.as_mut_ptr() as *mut __SlateRecord_pycore_obmalloc_h_622)
                .wrapping_offset(i3 as isize))
            .tail_hi = if is_used != 0 { 1i32.wrapping_neg() } else { 0 };
        }
    } else {
        // arena_base address is not ideal (aligned to arena size) and
        // so it potentially covers two MAP_BOT nodes.  Get the MAP_BOT node
        // for the next arena.  Note that it might be in different MAP_TOP
        // and MAP_MID nodes as well so we need to call arena_map_get()
        // again (do the full tree traversal).
        unsafe {
            (*((*n_hi).arenas.as_mut_ptr() as *mut __SlateRecord_pycore_obmalloc_h_622)
                .wrapping_offset(i3 as isize))
            .tail_hi = if is_used != 0 { tail } else { 0 };
        }
        let arena_base_next: u64 = arena_base.wrapping_add(1048576u64);
        // If arena_base is a legit arena address, so is arena_base_next - 1
        // (last address in arena).  If arena_base_next overflows then it
        // must overflow to 0.  However, that would mean arena_base was
        // "ideal" and we should not be in this case.
        let n_lo: *mut arena_map_bot = arena_map_get(state, arena_base_next as *mut u8, is_used);
        if n_lo.is_null() {
            // otherwise should already exist
            unsafe {
                (*((*n_hi).arenas.as_mut_ptr() as *mut __SlateRecord_pycore_obmalloc_h_622)
                    .wrapping_offset(i3 as isize))
                .tail_hi = 0;
            }
            return 0; // failed to allocate space for node
        }
        let i3_next: i32 = MAP_BOT_INDEX!(arena_base_next) as i32;
        unsafe {
            (*((*n_lo).arenas.as_mut_ptr() as *mut __SlateRecord_pycore_obmalloc_h_622)
                .wrapping_offset(i3_next as isize))
            .tail_lo = if is_used != 0 { tail } else { 0 };
        }
    }
    1
}

/// Return true if 'p' is a pointer inside an obmalloc arena.
/// _PyObject_Free() calls this so it needs to be very fast.
fn arena_map_is_used(state: *mut _obmalloc_state, p: *mut u8) -> i32 {
    let n: *mut arena_map_bot = arena_map_get(state, p, 0i32);
    if n.is_null() {
        return 0;
    }
    let i3: i32 = MAP_BOT_INDEX!(p) as i32;
    // ARENA_BITS must be < 32 so that the tail is a non-negative int32_t.
    let hi: i32 = unsafe {
        (*((*n).arenas.as_mut_ptr() as *mut __SlateRecord_pycore_obmalloc_h_622)
            .wrapping_offset(i3 as isize))
        .tail_hi
    };
    let lo: i32 = unsafe {
        (*((*n).arenas.as_mut_ptr() as *mut __SlateRecord_pycore_obmalloc_h_622)
            .wrapping_offset(i3 as isize))
        .tail_lo
    };
    let tail: i32 = (AS_UINT!(p) & 1048575u64) as i32;
    (tail < lo || tail >= hi && hi != 0) as i32
}

// end of radix tree logic
/// Allocate a new arena.  If we run out of memory, return NULL.  Else
/// allocate a new arena, and return the address of an arena_object
/// describing the new arena.  It's expected that the caller will set
/// `usable_arenas` to the return value.
fn new_arena(state: *mut _obmalloc_state) -> *mut arena_object {
    let mut arenaobj: *mut arena_object = unsafe { core::mem::zeroed() };
    let mut excess: u32 = 0; // number of bytes above pool alignment
    let mut address: *mut () = unsafe { core::mem::zeroed() };
    let mut debug_stats: i32 = unsafe { _PyRuntime.obmalloc.dump_debug_stats };
    if debug_stats == 1i32.wrapping_neg() {
        let opt: *const i8 = (unsafe { Py_GETENV(c"PYTHONMALLOCSTATS".as_ptr()) }) as *const i8;
        debug_stats = (!opt.is_null() && ((unsafe { *opt }) as i32) != 0) as i32;
        unsafe {
            _PyRuntime.obmalloc.dump_debug_stats = debug_stats;
        }
    }
    if debug_stats != 0 {
        _PyObject_DebugMallocStats(slate_support::stderr());
    }
    if unsafe { (*state).mgmt.unused_arena_objects }.is_null() {
        let mut numarenas: u32 = 0;
        let mut nbytes: u64 = 0;
        // Double the number of arena objects on each allocation.
        // Note that it's possible for `numarenas` to overflow.
        numarenas = if (unsafe { (*state).mgmt.maxarenas }) != 0 {
            (unsafe { (*state).mgmt.maxarenas }) << 1i32
        } else {
            16
        };
        if numarenas <= unsafe { (*state).mgmt.maxarenas } {
            return core::ptr::null_mut::<arena_object>();
        }
        // overflow
        nbytes = (numarenas as u64).wrapping_mul(core::mem::size_of::<arena_object>() as u64);
        arenaobj = PyMem_RawRealloc((unsafe { (*state).mgmt.arenas }) as *mut (), nbytes)
            as *mut arena_object;
        if arenaobj.is_null() {
            return core::ptr::null_mut::<arena_object>();
        }
        unsafe {
            (*state).mgmt.arenas = arenaobj;
        }
        // We might need to fix pointers that were copied.  However,
        // new_arena only gets called when all the pages in the
        // previous arenas are full.  Thus, there are *no* pointers
        // into the old array. Thus, we don't have to worry about
        // invalid pointers.  Just to be sure, some asserts:
        // Put the new arenas on the unused_arena_objects list.
        for i in unsafe { (*state).mgmt.maxarenas }..numarenas {
            unsafe {
                (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).address = 0;
            }
            // mark as unassociated
            unsafe {
                (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).nextarena =
                    if i < numarenas.wrapping_sub(1) {
                        unsafe { (*state).mgmt.arenas }.wrapping_offset(i.wrapping_add(1) as isize)
                    } else {
                        core::ptr::null_mut::<arena_object>()
                    };
            }
        }
        // Update globals.
        unsafe {
            (*state).mgmt.unused_arena_objects =
                unsafe { (*state).mgmt.arenas }.wrapping_offset((*state).mgmt.maxarenas as isize);
        }
        unsafe {
            (*state).mgmt.maxarenas = numarenas;
        }
    }
    // Take the next available arena object off the head of the list.
    arenaobj = unsafe { (*state).mgmt.unused_arena_objects };
    unsafe {
        (*state).mgmt.unused_arena_objects = (*arenaobj).nextarena;
    }
    address = unsafe {
        unsafe { _PyRuntime.allocators.obj_arena.alloc }.unwrap()(
            _PyRuntime.allocators.obj_arena.ctx,
            1048576u64,
        )
    };
    if !address.is_null() {
        if arena_map_mark_used(state, address as u64, 1i32) == 0 {
            // marking arena in radix tree failed, abort
            unsafe {
                unsafe { _PyRuntime.allocators.obj_arena.free }.unwrap()(
                    _PyRuntime.allocators.obj_arena.ctx,
                    address,
                    1048576u64,
                )
            };
            address = core::ptr::null_mut::<()>();
        }
    }
    if address.is_null() {
        // The allocation failed: return NULL after putting the
        // arenaobj back.
        unsafe {
            (*arenaobj).nextarena = (*state).mgmt.unused_arena_objects;
        }
        unsafe {
            (*state).mgmt.unused_arena_objects = arenaobj;
        }
        return core::ptr::null_mut::<arena_object>();
    }
    unsafe {
        (*arenaobj).address = address as u64;
    }
    let __v4: u64 = unsafe { (*state).mgmt.narenas_currently_allocated };
    unsafe {
        (*state).mgmt.narenas_currently_allocated = __v4.wrapping_add(1);
    }
    let __v7: u64 = unsafe { (*state).mgmt.ntimes_arena_allocated };
    unsafe {
        (*state).mgmt.ntimes_arena_allocated = __v7.wrapping_add(1);
    }
    if (unsafe { (*state).mgmt.narenas_currently_allocated })
        > unsafe { (*state).mgmt.narenas_highwater }
    {
        unsafe {
            (*state).mgmt.narenas_highwater = (*state).mgmt.narenas_currently_allocated;
        }
    }
    unsafe {
        (*arenaobj).freepools = core::ptr::null_mut::<pool_header>();
    }
    // pool_address <- first pool-aligned address in the arena
    // nfreepools <- number of whole pools that fit after alignment
    unsafe {
        (*arenaobj).pool_address = (*arenaobj).address as *mut u8;
    }
    unsafe {
        (*arenaobj).nfreepools = 64u32;
    }
    excess = ((unsafe { (*arenaobj).address }) & 16383u64) as u32;
    if excess != 0 {
        let __v10: u32 = unsafe { (*arenaobj).nfreepools };
        unsafe {
            (*arenaobj).nfreepools = __v10.wrapping_sub(1);
        }
        let __v13: *mut u8 = unsafe { (*arenaobj).pool_address };
        unsafe {
            (*arenaobj).pool_address =
                __v13.wrapping_offset(16384u32.wrapping_sub(excess) as isize);
        }
    }
    unsafe {
        (*arenaobj).ntotalpools = (*arenaobj).nfreepools;
    }
    arenaobj
}

/// Return true if and only if P is an address that was allocated by
/// pymalloc.  When the radix tree is used, 'poolp' is unused.
fn address_in_range(
    state: *mut _obmalloc_state,
    p: *mut (),
    _unused_pool: *mut pool_header,
) -> bool {
    arena_map_is_used(state, p as *mut u8) != 0
}

/// Called when freelist is exhausted.  Extend the freelist if there is
///
/// space for a block.  Otherwise, remove this pool from usedpools.
fn pymalloc_pool_extend(mut pool: *mut pool_header, size: u32) {
    if UNLIKELY!((unsafe { (*pool).nextoffset }) <= unsafe { (*pool).maxnextoffset }) != 0 {
        // There is room for another block.
        unsafe {
            (*pool).freeblock = (pool as *mut u8).wrapping_offset((*pool).nextoffset as isize);
        }
        let __v1: u32 = unsafe { (*pool).nextoffset };
        let __v2: u32 = __v1.wrapping_add(INDEX2SIZE!(size));
        unsafe {
            (*pool).nextoffset = __v2;
        }
        unsafe {
            *((*pool).freeblock as *mut *mut u8) = core::ptr::null_mut::<u8>();
        }
        return;
    }
    // Pool is full, unlink from used pools.
    let next: *mut pool_header = unsafe { (*pool).nextpool };
    pool = unsafe { (*pool).prevpool };
    unsafe {
        (*next).prevpool = pool;
    }
    unsafe {
        (*pool).nextpool = next;
    }
}

/// called when pymalloc_alloc can not allocate a block from usedpool.
/// This function takes new pool and allocate a block from it.
fn allocate_from_new_pool(state: *mut _obmalloc_state, mut size: u32) -> *mut () {
    // There isn't a pool of the right size class immediately
    // available:  use a free pool.
    if UNLIKELY!(unsafe { (*state).mgmt.usable_arenas }.is_null()) != 0 {
        // No arena has a free pool:  allocate a new arena.
        unsafe {
            (*state).mgmt.usable_arenas = new_arena(state);
        }
        if unsafe { (*state).mgmt.usable_arenas }.is_null() {
            return core::ptr::null_mut::<()>();
        }
        unsafe {
            (*(*state).mgmt.usable_arenas).prevarena = core::ptr::null_mut::<arena_object>();
        }
        unsafe {
            (*(*state).mgmt.usable_arenas).nextarena = core::ptr::null_mut::<arena_object>();
        }
        unsafe {
            *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
                .wrapping_offset((*(*state).mgmt.usable_arenas).nfreepools as isize) =
                (*state).mgmt.usable_arenas;
        }
    }
    // This arena already had the smallest nfreepools value, so decreasing
    // nfreepools doesn't change that, and we don't need to rearrange the
    // usable_arenas list.  However, if the arena becomes wholly allocated,
    // we need to remove its arena_object from usable_arenas.
    if (unsafe {
        *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
            .wrapping_offset((*(*state).mgmt.usable_arenas).nfreepools as isize)
    }) == unsafe { (*state).mgmt.usable_arenas }
    {
        // It's the last of this size, so there won't be any.
        unsafe {
            *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
                .wrapping_offset((*(*state).mgmt.usable_arenas).nfreepools as isize) =
                core::ptr::null_mut::<arena_object>();
        }
    }
    // If any free pools will remain, it will be the new smallest.
    if (unsafe { (*(*state).mgmt.usable_arenas).nfreepools }) > 1 {
        unsafe {
            *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object).wrapping_offset(
                unsafe { (*(*state).mgmt.usable_arenas).nfreepools }.wrapping_sub(1) as isize,
            ) = (*state).mgmt.usable_arenas;
        }
    }
    // Try to get a cached free pool.
    let mut pool: *mut pool_header = unsafe { (*(*state).mgmt.usable_arenas).freepools };
    if LIKELY!(!pool.is_null()) != 0 {
        // Unlink from cached pools.
        unsafe {
            (*(*state).mgmt.usable_arenas).freepools = (*pool).nextpool;
        }
        let __v0: *mut arena_object = unsafe { (*state).mgmt.usable_arenas };
        let __v1: u32 = unsafe { (*__v0).nfreepools };
        unsafe {
            (*__v0).nfreepools = __v1.wrapping_sub(1);
        }
        if UNLIKELY!((unsafe { (*(*state).mgmt.usable_arenas).nfreepools }) == 0) != 0 {
            // Wholly allocated:  remove.
            unsafe {
                (*state).mgmt.usable_arenas = (*(*state).mgmt.usable_arenas).nextarena;
            }
            if !unsafe { (*state).mgmt.usable_arenas }.is_null() {
                unsafe {
                    (*(*state).mgmt.usable_arenas).prevarena =
                        core::ptr::null_mut::<arena_object>();
                }
            }
        }
    // nfreepools > 0:  it must be that freepools
    // isn't NULL, or that we haven't yet carved
    // off all the arena's pools for the first
    // time.
    } else {
        // Carve off a new pool.
        pool = (unsafe { (*(*state).mgmt.usable_arenas).pool_address }) as *mut pool_header;
        unsafe {
            (*pool).arenaindex = (unsafe { (*state).mgmt.usable_arenas }
                .offset_from((*state).mgmt.arenas as *mut arena_object)
                as i64) as u32;
        }
        unsafe {
            (*pool).szidx = 65535;
        }
        let __v3: *mut arena_object = unsafe { (*state).mgmt.usable_arenas };
        let __v4: *mut u8 = unsafe { (*__v3).pool_address };
        unsafe {
            (*__v3).pool_address = __v4.wrapping_offset(16384i32 as isize);
        }
        let __v6: *mut arena_object = unsafe { (*state).mgmt.usable_arenas };
        let __v7: u32 = unsafe { (*__v6).nfreepools };
        unsafe {
            (*__v6).nfreepools = __v7.wrapping_sub(1);
        }
        if (unsafe { (*(*state).mgmt.usable_arenas).nfreepools }) == 0 {
            // Unlink the arena:  it is completely allocated.
            unsafe {
                (*state).mgmt.usable_arenas = (*(*state).mgmt.usable_arenas).nextarena;
            }
            if !unsafe { (*state).mgmt.usable_arenas }.is_null() {
                unsafe {
                    (*(*state).mgmt.usable_arenas).prevarena =
                        core::ptr::null_mut::<arena_object>();
                }
            }
        }
    }
    // Frontlink to used pools.
    let mut bp: *mut u8 = unsafe { core::mem::zeroed() };
    let next: *mut pool_header = unsafe {
        *((*state).pools.used.as_mut_ptr() as *mut *mut pool_header)
            .wrapping_offset(size.wrapping_add(size) as isize)
    }; // == prev
    unsafe {
        (*pool).nextpool = next;
    }
    unsafe {
        (*pool).prevpool = next;
    }
    unsafe {
        (*next).nextpool = pool;
    }
    unsafe {
        (*next).prevpool = pool;
    }
    unsafe {
        (*pool).r#ref.count = 1;
    }
    if (unsafe { (*pool).szidx }) == size {
        // Luckily, this pool last contained blocks
        // of the same size class, so its header
        // and free list are already initialized.
        bp = unsafe { (*pool).freeblock };
        unsafe {
            (*pool).freeblock = *(bp as *mut *mut u8);
        }
        return bp as *mut ();
    }
    // Initialize the pool header, set up the free list to
    // contain just the second block, and return the first
    // block.
    unsafe {
        (*pool).szidx = size;
    }
    size = INDEX2SIZE!(size);
    bp = (pool as *mut u8)
        .wrapping_offset(
            _Py_SIZE_ROUND_UP!(core::mem::size_of::<pool_header>() as u64, 16i32) as isize,
        );
    unsafe {
        (*pool).nextoffset = _Py_SIZE_ROUND_UP!(core::mem::size_of::<pool_header>() as u64, 16i32)
            .wrapping_add((size << 1i32) as u64) as u32;
    }
    unsafe {
        (*pool).maxnextoffset = 16384u32.wrapping_sub(size);
    }
    unsafe {
        (*pool).freeblock = bp.wrapping_offset(size as isize);
    }
    unsafe {
        *((*pool).freeblock as *mut *mut u8) = core::ptr::null_mut::<u8>();
    }
    bp as *mut ()
}

/// pymalloc allocator
///
/// Return a pointer to newly allocated memory if pymalloc allocated memory.
///
/// Return NULL if pymalloc failed to allocate the memory block: on bigger
/// requests, on error in the code below (as a last chance to serve the request)
/// or when the max memory limit has been reached.
fn pymalloc_alloc(state: *mut _obmalloc_state, _unused_ctx: *mut (), nbytes: u64) -> *mut () {
    if UNLIKELY!(nbytes == 0) != 0 {
        return core::ptr::null_mut::<()>();
    }
    if UNLIKELY!(nbytes > 512) != 0 {
        return core::ptr::null_mut::<()>();
    }
    let size: u32 = (nbytes.wrapping_sub(1) as u32) >> 4i32;
    let pool: *mut pool_header = unsafe {
        *((*state).pools.used.as_mut_ptr() as *mut *mut pool_header)
            .wrapping_offset(size.wrapping_add(size) as isize)
    };
    let mut bp: *mut u8 = unsafe { core::mem::zeroed() };
    if LIKELY!(pool != unsafe { (*pool).nextpool }) != 0 {
        // There is a used pool for this size class.
        // Pick up the head block of its free list.
        let __v1: u32 = unsafe { (*pool).r#ref.count };
        unsafe {
            (*pool).r#ref.count = __v1.wrapping_add(1);
        }
        bp = unsafe { (*pool).freeblock };
        let __v3: *mut u8 = unsafe { *(bp as *mut *mut u8) };
        unsafe {
            (*pool).freeblock = __v3;
        }
        if __v3.is_null() {
            // Reached the end of the free list, try to extend it.
            pymalloc_pool_extend(pool, size);
        }
    } else {
        // There isn't a pool of the right size class immediately
        // available:  use a free pool.
        bp = allocate_from_new_pool(state, size) as *mut u8;
    }
    bp as *mut ()
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_Malloc")]
extern "C-unwind" fn _PyObject_Malloc(ctx: *mut (), nbytes: u64) -> *mut () {
    let state: *mut _obmalloc_state = get_state();
    let mut ptr: *mut () = pymalloc_alloc(state, ctx, nbytes);
    if LIKELY!(!ptr.is_null()) != 0 {
        return ptr;
    }
    ptr = PyMem_RawMalloc(nbytes);
    if !ptr.is_null() {
        let __v1: i64 = unsafe { (*state).mgmt.raw_allocated_blocks };
        unsafe {
            (*state).mgmt.raw_allocated_blocks = __v1.wrapping_add(1);
        }
    }
    ptr
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_Calloc")]
extern "C-unwind" fn _PyObject_Calloc(ctx: *mut (), nelem: u64, elsize: u64) -> *mut () {
    let nbytes: u64 = nelem.wrapping_mul(elsize);
    let state: *mut _obmalloc_state = get_state();
    let mut ptr: *mut () = pymalloc_alloc(state, ctx, nbytes);
    if LIKELY!(!ptr.is_null()) != 0 {
        unsafe {
            core::ptr::write_bytes(
                (ptr as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                (nbytes as usize) as usize,
            )
        };
        return ptr;
    }
    ptr = PyMem_RawCalloc(nelem, elsize);
    if !ptr.is_null() {
        let __v1: i64 = unsafe { (*state).mgmt.raw_allocated_blocks };
        unsafe {
            (*state).mgmt.raw_allocated_blocks = __v1.wrapping_add(1);
        }
    }
    ptr
}

fn insert_to_usedpool(state: *mut _obmalloc_state, pool: *mut pool_header) {
    // else the pool is empty
    let size: u32 = unsafe { (*pool).szidx };
    let next: *mut pool_header = unsafe {
        *((*state).pools.used.as_mut_ptr() as *mut *mut pool_header)
            .wrapping_offset(size.wrapping_add(size) as isize)
    };
    let prev: *mut pool_header = unsafe { (*next).prevpool };
    // insert pool before next:   prev <-> pool <-> next
    unsafe {
        (*pool).nextpool = next;
    }
    unsafe {
        (*pool).prevpool = prev;
    }
    unsafe {
        (*next).prevpool = pool;
    }
    unsafe {
        (*prev).nextpool = pool;
    }
}

fn insert_to_freepool(state: *mut _obmalloc_state, pool: *mut pool_header) {
    let next: *mut pool_header = unsafe { (*pool).nextpool };
    let prev: *mut pool_header = unsafe { (*pool).prevpool };
    unsafe {
        (*next).prevpool = prev;
    }
    unsafe {
        (*prev).nextpool = next;
    }
    // Link the pool to freepools.  This is a singly-linked
    // list, and pool->prevpool isn't used there.
    let ao: *mut arena_object =
        unsafe { (*state).mgmt.arenas }.wrapping_offset((unsafe { (*pool).arenaindex }) as isize);
    unsafe {
        (*pool).nextpool = (*ao).freepools;
    }
    unsafe {
        (*ao).freepools = pool;
    }
    let mut nf: u32 = unsafe { (*ao).nfreepools };
    // If this is the rightmost arena with this number of free pools,
    // nfp2lasta[nf] needs to change.  Caution:  if nf is 0, there
    // are no arenas in usable_arenas with that value.
    let lastnf: *mut arena_object = unsafe {
        *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
            .wrapping_offset(nf as isize)
    };
    if lastnf == ao {
        // it is the rightmost
        let p: *mut arena_object = unsafe { (*ao).prevarena };
        unsafe {
            *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
                .wrapping_offset(nf as isize) = if !p.is_null() && (*p).nfreepools == nf {
                p
            } else {
                core::ptr::null_mut::<arena_object>()
            };
        }
    }
    let __v1: u32 = nf.wrapping_add(1);
    nf = __v1;
    unsafe {
        (*ao).nfreepools = __v1;
    }
    // All the rest is arena management.  We just freed
    // a pool, and there are 4 cases for arena mgmt:
    // 1. If all the pools are free, return the arena to
    //    the system free().  Except if this is the last
    //    arena in the list, keep it to avoid thrashing:
    //    keeping one wholly free arena in the list avoids
    //    pathological cases where a simple loop would
    //    otherwise provoke needing to allocate and free an
    //    arena on every iteration.  See bpo-37257.
    // 2. If this is the only free pool in the arena,
    //    add the arena back to the `usable_arenas` list.
    // 3. If the "next" arena has a smaller count of free
    //    pools, we have to "slide this arena right" to
    //    restore that usable_arenas is sorted in order of
    //    nfreepools.
    // 4. Else there's nothing more to do.
    if nf == unsafe { (*ao).ntotalpools } && !unsafe { (*ao).nextarena }.is_null() {
        // Case 1.  First unlink ao from usable_arenas.
        // Fix the pointer in the prevarena, or the
        // usable_arenas pointer.
        if unsafe { (*ao).prevarena }.is_null() {
            unsafe {
                (*state).mgmt.usable_arenas = (*ao).nextarena;
            }
        } else {
            unsafe {
                (*(*ao).prevarena).nextarena = (*ao).nextarena;
            }
        }
        // Fix the pointer in the nextarena.
        if !unsafe { (*ao).nextarena }.is_null() {
            unsafe {
                (*(*ao).nextarena).prevarena = (*ao).prevarena;
            }
        }
        // Record that this arena_object slot is
        // available to be reused.
        unsafe {
            (*ao).nextarena = (*state).mgmt.unused_arena_objects;
        }
        unsafe {
            (*state).mgmt.unused_arena_objects = ao;
        }
        // mark arena region as not under control of obmalloc
        arena_map_mark_used(state, unsafe { (*ao).address }, 0i32);
        // Free the entire arena.
        unsafe {
            unsafe { _PyRuntime.allocators.obj_arena.free }.unwrap()(
                _PyRuntime.allocators.obj_arena.ctx,
                (*ao).address as *mut (),
                1048576u64,
            )
        };
        unsafe {
            (*ao).address = 0;
        }
        // mark unassociated
        let __v3: u64 = unsafe { (*state).mgmt.narenas_currently_allocated };
        unsafe {
            (*state).mgmt.narenas_currently_allocated = __v3.wrapping_sub(1);
        }
        return;
    }
    if nf == 1 {
        // Case 2.  Put ao at the head of
        // usable_arenas.  Note that because
        // ao->nfreepools was 0 before, ao isn't
        // currently on the usable_arenas list.
        unsafe {
            (*ao).nextarena = (*state).mgmt.usable_arenas;
        }
        unsafe {
            (*ao).prevarena = core::ptr::null_mut::<arena_object>();
        }
        if !unsafe { (*state).mgmt.usable_arenas }.is_null() {
            unsafe {
                (*(*state).mgmt.usable_arenas).prevarena = ao;
            }
        }
        unsafe {
            (*state).mgmt.usable_arenas = ao;
        }
        if unsafe {
            *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
                .wrapping_offset(1 as isize)
        }
        .is_null()
        {
            unsafe {
                *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
                    .wrapping_offset(1 as isize) = ao;
            }
        }
        return;
    }
    // If this arena is now out of order, we need to keep
    // the list sorted.  The list is kept sorted so that
    // the "most full" arenas are used first, which allows
    // the nearly empty arenas to be completely freed.  In
    // a few un-scientific tests, it seems like this
    // approach allowed a lot more memory to be freed.
    //
    // If this is the only arena with nf, record that.
    if unsafe {
        *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
            .wrapping_offset(nf as isize)
    }
    .is_null()
    {
        unsafe {
            *((*state).mgmt.nfp2lasta.as_mut_ptr() as *mut *mut arena_object)
                .wrapping_offset(nf as isize) = ao;
        }
    }
    // else the rightmost with nf doesn't change
    // If this was the rightmost of the old size, it remains in place.
    if ao == lastnf {
        // Case 4.  Nothing to do.
        return;
    }
    // If ao were the only arena in the list, the last block would have
    // gotten us out.
    // Case 3:  We have to move the arena towards the end of the list,
    // because it has more free pools than the arena to its right.  It needs
    // to move to follow lastnf.
    // First unlink ao from usable_arenas.
    if !unsafe { (*ao).prevarena }.is_null() {
        // ao isn't at the head of the list
        unsafe {
            (*(*ao).prevarena).nextarena = (*ao).nextarena;
        }
    } else {
        // ao is at the head of the list
        unsafe {
            (*state).mgmt.usable_arenas = (*ao).nextarena;
        }
    }
    unsafe {
        (*(*ao).nextarena).prevarena = (*ao).prevarena;
    }
    // And insert after lastnf.
    unsafe {
        (*ao).prevarena = lastnf;
    }
    unsafe {
        (*ao).nextarena = (*lastnf).nextarena;
    }
    if !unsafe { (*ao).nextarena }.is_null() {
        unsafe {
            (*(*ao).nextarena).prevarena = ao;
        }
    }
    unsafe {
        (*lastnf).nextarena = ao;
    }
    // Verify that the swaps worked.
}

/// Free a memory block allocated by pymalloc_alloc().
/// Return 1 if it was freed.
/// Return 0 if the block was not allocated by pymalloc_alloc().
fn pymalloc_free(state: *mut _obmalloc_state, _unused_ctx: *mut (), p: *mut ()) -> i32 {
    let pool: *mut pool_header = POOL_ADDR!(p);
    if UNLIKELY!(!address_in_range(state, p, pool)) != 0 {
        return 0;
    }
    // We allocated this address.
    // Link p to the start of the pool's freeblock list.  Since
    // the pool had at least the p block outstanding, the pool
    // wasn't empty (so it's already in a usedpools[] list, or
    // was full and is in no list -- it's not in the freeblocks
    // list in any case).
    // else it was empty
    let lastfree: *mut u8 = unsafe { (*pool).freeblock };
    unsafe {
        *(p as *mut *mut u8) = lastfree;
    }
    unsafe {
        (*pool).freeblock = p as *mut u8;
    }
    let __v1: u32 = unsafe { (*pool).r#ref.count };
    unsafe {
        (*pool).r#ref.count = __v1.wrapping_sub(1);
    }
    if UNLIKELY!(lastfree.is_null()) != 0 {
        // Pool was full, so doesn't currently live in any list:
        // link it to the front of the appropriate usedpools[] list.
        // This mimics LRU pool usage for new allocations and
        // targets optimal filling when several pools contain
        // blocks of the same size class.
        insert_to_usedpool(state, pool);
        return 1;
    }
    // freeblock wasn't NULL, so the pool wasn't full,
    // and the pool is in a usedpools[] list.
    if LIKELY!((unsafe { (*pool).r#ref.count }) != 0) != 0 {
        // pool isn't empty:  leave it in usedpools
        return 1;
    }
    // Pool is now empty:  unlink from usedpools, and
    // link to the front of freepools.  This ensures that
    // previously freed pools will be allocated later
    // (being not referenced, they are perhaps paged out).
    insert_to_freepool(state, pool);
    1
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_Free")]
extern "C-unwind" fn _PyObject_Free(ctx: *mut (), p: *mut ()) {
    // PyObject_Free(NULL) has no effect
    if p.is_null() {
        return;
    }
    let state: *mut _obmalloc_state = get_state();
    if UNLIKELY!(pymalloc_free(state, ctx, p) == 0) != 0 {
        // pymalloc didn't allocate this address
        PyMem_RawFree(p);
        let __v1: i64 = unsafe { (*state).mgmt.raw_allocated_blocks };
        unsafe {
            (*state).mgmt.raw_allocated_blocks = __v1.wrapping_sub(1);
        }
    }
}

/// pymalloc realloc.
///
/// If nbytes==0, then as the Python docs promise, we do not treat this like
/// free(p), and return a non-NULL result.
///
/// Return 1 if pymalloc reallocated memory and wrote the new pointer into
/// newptr_p.
///
/// Return 0 if pymalloc didn't allocated p.
fn pymalloc_realloc(
    state: *mut _obmalloc_state,
    ctx: *mut (),
    newptr_p: &mut *mut (),
    p: *mut (),
    nbytes: u64,
) -> i32 {
    let mut size: u64 = 0;
    let pool: *mut pool_header = POOL_ADDR!(p);
    if !address_in_range(state, p, pool) {
        // pymalloc is not managing this block.
        //
        // If nbytes <= SMALL_REQUEST_THRESHOLD, it's tempting to try to take
        // over this block.  However, if we do, we need to copy the valid data
        // from the C-managed block to one of our blocks, and there's no
        // portable way to know how much of the memory space starting at p is
        // valid.
        //
        // As bug 1185883 pointed out the hard way, it's possible that the
        // C-managed block is "at the end" of allocated VM space, so that a
        // memory fault can occur if we try to copy nbytes bytes starting at p.
        // Instead we punt: let C continue to manage this block.
        return 0;
    }
    // pymalloc is in charge of this block
    size = INDEX2SIZE!(unsafe { (*pool).szidx }) as u64;
    if nbytes <= size {
        // The block is staying the same or shrinking.
        //
        // If it's shrinking, there's a tradeoff: it costs cycles to copy the
        // block to a smaller size class, but it wastes memory not to copy it.
        //
        // The compromise here is to copy on shrink only if at least 25% of
        // size can be shaved off.
        if 4u64.wrapping_mul(nbytes) > 3u64.wrapping_mul(size) {
            // It's the same, or shrinking and new/old > 3/4.
            unsafe {
                *(newptr_p as *mut *mut ()) = p;
            }
            return 1;
        }
        size = nbytes;
    }
    let bp: *mut () = _PyObject_Malloc(ctx, nbytes);
    if !bp.is_null() {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((p as *const ()) as *const core::ffi::c_void) as *const u8,
                (bp as *mut core::ffi::c_void) as *mut u8,
                (size as usize) as usize,
            )
        };
        _PyObject_Free(ctx, p);
    }
    unsafe {
        *(newptr_p as *mut *mut ()) = bp;
    }
    1
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyObject_Realloc")]
extern "C-unwind" fn _PyObject_Realloc(ctx: *mut (), ptr: *mut (), nbytes: u64) -> *mut () {
    let mut ptr2: *mut () = unsafe { core::mem::zeroed() };
    if ptr.is_null() {
        return _PyObject_Malloc(ctx, nbytes);
    }
    let state: *mut _obmalloc_state = get_state();
    if pymalloc_realloc(
        state,
        ctx,
        unsafe { &mut (*core::ptr::addr_of_mut!(ptr2)) },
        ptr,
        nbytes,
    ) != 0
    {
        return ptr2;
    }
    PyMem_RawRealloc(ptr, nbytes)
}

// A x-platform debugging allocator.  This doesn't manage memory directly,
// it wraps a real allocator, adding extra debugging info to the memory blocks.
// Uncomment this define to add the "serialno" field
//
// #define PYMEM_DEBUG_SERIALNO
/// Read sizeof(size_t) bytes at p as a big-endian size_t.
fn read_size_t(p: *const ()) -> u64 {
    let mut q: *const u8 = p as *const u8;
    let mut result: u64 = 0;
    let __v0: *const u8 = q;
    q = __v0.wrapping_offset(1 as isize);
    result = (unsafe { *__v0 }) as u64;
    let mut i: i32 = 8;
    loop {
        let __v3: i32 = i.wrapping_sub(1);
        i = __v3;
        if __v3 <= 0 {
            break;
        }
        result = result << 8i32 | (((unsafe { *q }) as i32) as u64);
        let __v5: *const u8 = q;
        q = __v5.wrapping_offset(1 as isize);
    }
    result
}

/// Write n as a big-endian size_t, MSB at address p, LSB at
/// p + sizeof(size_t) - 1.
fn write_size_t(p: *mut (), mut n: u64) {
    let mut q: *mut u8 = (p as *mut u8)
        .wrapping_offset(8 as isize)
        .wrapping_offset(-(1 as isize));
    let mut i: i32 = 8;
    loop {
        let __v3: i32 = i.wrapping_sub(1);
        i = __v3;
        if __v3 < 0 {
            break;
        }
        unsafe {
            *q = (n & 255) as u8;
        }
        n >>= 8i32;
        let __v5: *mut u8 = q;
        q = __v5.wrapping_offset(-(1 as isize));
    }
}

fn fill_mem_debug(
    api: *mut __SlateRecord_pycore_runtime_structs_h_19,
    data: *mut (),
    c: i32,
    nbytes: u64,
    is_alloc: bool,
) {
    unsafe {
        core::ptr::write_bytes(
            (data as *mut core::ffi::c_void) as *mut u8,
            (c as i32) as u8,
            (nbytes as usize) as usize,
        )
    };
}

// Let S = sizeof(size_t).  The debug malloc asks for 4 * S extra bytes and
//    fills them with useful stuff, here calling the underlying malloc's result p:
//
// p[0: S]
//     Number of bytes originally asked for.  This is a size_t, big-endian (easier
//     to read in a memory dump).
// p[S]
//     API ID.  See PEP 445.  This is a character, but seems undocumented.
// p[S+1: 2*S]
//     Copies of PYMEM_FORBIDDENBYTE.  Used to catch under- writes and reads.
// p[2*S: 2*S+n]
//     The requested memory, filled with copies of PYMEM_CLEANBYTE.
//     Used to catch reference to uninitialized memory.
//     &p[2*S] is returned.  Note that this is 8-byte aligned if pymalloc
//     handled the request itself.
// p[2*S+n: 2*S+n+S]
//     Copies of PYMEM_FORBIDDENBYTE.  Used to catch over- writes and reads.
// p[2*S+n+S: 2*S+n+2*S]
//     A serial number, incremented by 1 on each call to _PyMem_DebugMalloc
//     and _PyMem_DebugRealloc.
//     This is a big-endian size_t.
//     If "bad memory" is detected later, the serial number gives an
//     excellent way to set a breakpoint on the next run, to capture the
//     instant at which this block was passed out.
//
// If PYMEM_DEBUG_SERIALNO is not defined (default), the debug malloc only asks
// for 3 * S extra bytes, and omits the last serialno field.
fn _PyMem_DebugRawAlloc(use_calloc: i32, ctx: *mut (), nbytes: u64) -> *mut () {
    let api: *mut __SlateRecord_pycore_runtime_structs_h_19 =
        ctx as *mut __SlateRecord_pycore_runtime_structs_h_19;
    let mut p: *mut u8 = unsafe { core::mem::zeroed() }; // base address of malloc'ed pad block
    let mut total: u64 = 0; // nbytes + PYMEM_DEBUG_EXTRA_BYTES
    if nbytes > 9223372036854775783u64 {
        // integer overflow: can't represent total as a Py_ssize_t
        return core::ptr::null_mut::<()>();
    }
    total = nbytes.wrapping_add(24u64);
    // Layout: [SSSS IFFF CCCC...CCCC FFFF NNNN]
    //          ^--- p    ^--- data   ^--- tail
    // S: nbytes stored as size_t
    // I: API identifier (1 byte)
    // F: Forbidden bytes (size_t - 1 bytes before, size_t bytes after)
    // C: Clean bytes used later to store actual data
    // N: Serial number stored as size_t
    //
    // If PYMEM_DEBUG_SERIALNO is not defined (default), the last NNNN field
    // is omitted.
    if use_calloc != 0 {
        p = (unsafe { unsafe { (*api).alloc.calloc }.unwrap()((*api).alloc.ctx, 1u64, total) })
            as *mut u8;
    } else {
        p = (unsafe { unsafe { (*api).alloc.malloc }.unwrap()((*api).alloc.ctx, total) })
            as *mut u8;
    }
    if p.is_null() {
        return core::ptr::null_mut::<()>();
    }
    // p + 2*SST == pointer to data bytes
    let data: *mut u8 = p.wrapping_offset(16i32 as isize);
    // at p, write size (SST bytes), id (1 byte), pad (SST-1 bytes)
    write_size_t(p as *mut (), nbytes);
    unsafe {
        *p.wrapping_offset(8 as isize) = (*api).api_id as u8;
    }
    unsafe {
        core::ptr::write_bytes(
            ((p.wrapping_offset(8 as isize).wrapping_offset(1 as isize) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            (253i32 as i32) as u8,
            (7u64 as usize) as usize,
        )
    };
    if nbytes > 0 && use_calloc == 0 {
        fill_mem_debug(api, data as *mut (), 205i32, nbytes, true);
    }
    // at tail, write pad (SST bytes) and serialno (SST bytes)
    // data + nbytes == pointer to tail pad bytes
    let tail: *mut u8 = data.wrapping_offset(nbytes as isize);
    unsafe {
        core::ptr::write_bytes(
            ((tail as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (253i32 as i32) as u8,
            (8u64 as usize) as usize,
        )
    };
    data as *mut ()
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugRawMalloc")]
extern "C-unwind" fn _PyMem_DebugRawMalloc(ctx: *mut (), nbytes: u64) -> *mut () {
    _PyMem_DebugRawAlloc(0i32, ctx, nbytes)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugRawCalloc")]
extern "C-unwind" fn _PyMem_DebugRawCalloc(ctx: *mut (), nelem: u64, elsize: u64) -> *mut () {
    let mut nbytes: u64 = 0;
    nbytes = nelem.wrapping_mul(elsize);
    _PyMem_DebugRawAlloc(1i32, ctx, nbytes)
}

/// The debug free first checks the 2*SST bytes on each end for sanity (in
/// particular, that the FORBIDDENBYTEs with the api ID are still intact).
/// Then fills the original bytes with PYMEM_DEADBYTE.
/// Then calls the underlying free.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugRawFree")]
extern "C-unwind" fn _PyMem_DebugRawFree(ctx: *mut (), p: *mut ()) {
    // PyMem_Free(NULL) has no effect
    if p.is_null() {
        return;
    }
    let api: *mut __SlateRecord_pycore_runtime_structs_h_19 =
        ctx as *mut __SlateRecord_pycore_runtime_structs_h_19;
    let q: *mut u8 = (p as *mut u8).wrapping_offset(-(16i32 as isize)); // address returned from malloc
    let mut nbytes: u64 = 0;
    _PyMem_DebugCheckAddress(
        c"_PyMem_DebugRawFree".as_ptr(),
        unsafe { (*api).api_id },
        p as *const (),
    );
    nbytes = read_size_t(q as *const ());
    nbytes = nbytes.wrapping_add(8u64);
    unsafe {
        core::ptr::write_bytes(
            ((q as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (221i32 as i32) as u8,
            (16u64 as usize) as usize,
        )
    };
    fill_mem_debug(api, p, 221i32, nbytes, false);
    unsafe { unsafe { (*api).alloc.free }.unwrap()((*api).alloc.ctx, q as *mut ()) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugRawRealloc")]
extern "C-unwind" fn _PyMem_DebugRawRealloc(ctx: *mut (), p: *mut (), mut nbytes: u64) -> *mut () {
    if p.is_null() {
        return _PyMem_DebugRawAlloc(0i32, ctx, nbytes);
    }
    let mut head: *mut u8 = unsafe { core::mem::zeroed() }; // base address of malloc'ed pad block
    let mut data: *mut u8 = unsafe { core::mem::zeroed() }; // pointer to data bytes
    let mut tail: *mut u8 = unsafe { core::mem::zeroed() }; // data + nbytes == pointer to tail pad bytes
    let mut total: u64 = 0; // 2 * SST + nbytes + 2 * SST
    let mut original_nbytes: u64 = 0;
    _PyMem_DebugCheckAddress(
        c"_PyMem_DebugRawRealloc".as_ptr(),
        unsafe { (*(ctx as *mut __SlateRecord_pycore_runtime_structs_h_19)).api_id },
        p as *const (),
    );
    data = p as *mut u8;
    head = data.wrapping_offset(-(16i32 as isize));
    original_nbytes = read_size_t(head as *const ());
    if nbytes > 9223372036854775783u64 {
        // integer overflow: can't represent total as a Py_ssize_t
        return core::ptr::null_mut::<()>();
    }
    total = nbytes.wrapping_add(24u64);
    tail = data.wrapping_offset(original_nbytes as isize);
    // Mark the header, the trailer, ERASED_SIZE bytes at the begin and
    // ERASED_SIZE bytes at the end as dead and save the copy of erased bytes.
    let mut save: __SlateAlign16<[u8; 128]> = __SlateAlign16([0u8; 128]); // A copy of erased bytes.
    if original_nbytes <= (core::mem::size_of::<[u8; 128]>() as u64) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((data as *const ()) as *const core::ffi::c_void) as *const u8,
                ((save.0.as_mut_ptr() as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (original_nbytes as usize) as usize,
            )
        };
        unsafe {
            core::ptr::write_bytes(
                ((data.wrapping_offset(-(16i32 as isize)) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                (221i32 as i32) as u8,
                (original_nbytes.wrapping_add(24u64) as usize) as usize,
            )
        };
    } else {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((data as *const ()) as *const core::ffi::c_void) as *const u8,
                ((save.0.as_mut_ptr() as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (64u64 as usize) as usize,
            )
        };
        unsafe {
            core::ptr::write_bytes(
                ((head as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (221i32 as i32) as u8,
                (80u64 as usize) as usize,
            )
        };
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((tail.wrapping_offset(-(64 as isize)) as *const ()) as *const core::ffi::c_void)
                    as *const u8,
                (((save.0.as_mut_ptr() as *mut u8).wrapping_offset(64 as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                (64u64 as usize) as usize,
            )
        };
        unsafe {
            core::ptr::write_bytes(
                ((tail.wrapping_offset(-(64 as isize)) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                (221i32 as i32) as u8,
                (72u64 as usize) as usize,
            )
        };
    }
    // Resize and add decorations.
    let r: *mut u8 = (unsafe {
        unsafe {
            (*(ctx as *mut __SlateRecord_pycore_runtime_structs_h_19))
                .alloc
                .realloc
        }
        .unwrap()(
            (*(ctx as *mut __SlateRecord_pycore_runtime_structs_h_19))
                .alloc
                .ctx,
            head as *mut (),
            total,
        )
    }) as *mut u8;
    if r.is_null() {
        // if realloc() failed: rewrite header and footer which have
        // just been erased
        nbytes = original_nbytes;
    } else {
        head = r;
    }
    data = head.wrapping_offset(16i32 as isize);
    write_size_t(head as *mut (), nbytes);
    unsafe {
        *head.wrapping_offset(8 as isize) =
            (*(ctx as *mut __SlateRecord_pycore_runtime_structs_h_19)).api_id as u8;
    }
    unsafe {
        core::ptr::write_bytes(
            ((head.wrapping_offset(8 as isize).wrapping_offset(1 as isize) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            (253i32 as i32) as u8,
            (7u64 as usize) as usize,
        )
    };
    tail = data.wrapping_offset(nbytes as isize);
    unsafe {
        core::ptr::write_bytes(
            ((tail as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (253i32 as i32) as u8,
            (8u64 as usize) as usize,
        )
    };
    // Restore saved bytes.
    if original_nbytes <= (core::mem::size_of::<[u8; 128]>() as u64) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((save.0.as_mut_ptr() as *const ()) as *const core::ffi::c_void) as *const u8,
                ((data as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (Py_MIN!(nbytes, original_nbytes) as usize) as usize,
            )
        };
    } else {
        let i: u64 = original_nbytes.wrapping_sub(64);
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((save.0.as_mut_ptr() as *const ()) as *const core::ffi::c_void) as *const u8,
                ((data as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (Py_MIN__4!(nbytes, 64i32) as usize) as usize,
            )
        };
        if nbytes > i {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((save.0.as_mut_ptr() as *mut u8).wrapping_offset(64 as isize) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((data.wrapping_offset(i as isize) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    (Py_MIN__4!(nbytes.wrapping_sub(i), 64i32) as usize) as usize,
                )
            };
        }
    }
    if r.is_null() {
        return core::ptr::null_mut::<()>();
    }
    if nbytes > original_nbytes {
        // growing: mark new extra memory clean
        unsafe {
            core::ptr::write_bytes(
                ((data.wrapping_offset(original_nbytes as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                (205i32 as i32) as u8,
                (nbytes.wrapping_sub(original_nbytes) as usize) as usize,
            )
        };
    }
    data as *mut ()
}

fn _PyMem_DebugCheckGIL(func: *const i8) {
    let tstate: *mut _ts = _PyThreadState_GET();
    if tstate.is_null() {
        unsafe {
            _Py_FatalErrorFunc(
                func,
                c"Python memory allocator called without holding the GIL".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugMalloc")]
extern "C-unwind" fn _PyMem_DebugMalloc(ctx: *mut (), nbytes: u64) -> *mut () {
    _PyMem_DebugCheckGIL(c"_PyMem_DebugMalloc".as_ptr());
    _PyMem_DebugRawMalloc(ctx, nbytes)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugCalloc")]
extern "C-unwind" fn _PyMem_DebugCalloc(ctx: *mut (), nelem: u64, elsize: u64) -> *mut () {
    _PyMem_DebugCheckGIL(c"_PyMem_DebugCalloc".as_ptr());
    _PyMem_DebugRawCalloc(ctx, nelem, elsize)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugFree")]
extern "C-unwind" fn _PyMem_DebugFree(ctx: *mut (), ptr: *mut ()) {
    _PyMem_DebugCheckGIL(c"_PyMem_DebugFree".as_ptr());
    _PyMem_DebugRawFree(ctx, ptr);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.obmalloc._PyMem_DebugRealloc")]
extern "C-unwind" fn _PyMem_DebugRealloc(ctx: *mut (), ptr: *mut (), nbytes: u64) -> *mut () {
    _PyMem_DebugCheckGIL(c"_PyMem_DebugRealloc".as_ptr());
    _PyMem_DebugRawRealloc(ctx, ptr, nbytes)
}

/// Check the forbidden bytes on both ends of the memory allocated for p.
/// If anything is wrong, print info to stderr via _PyObject_DebugDumpAddress,
/// and call Py_FatalError to kill the program.
/// The API id, is also checked.
fn _PyMem_DebugCheckAddress(func: *const i8, api: i8, p: *const ()) {
    let q: *const u8 = p as *const u8;
    let mut nbytes: u64 = 0;
    let mut i: i32 = 0;
    let mut id: i8 = 0;
    // Check the API id
    id = (unsafe { *q.wrapping_offset(8i32.wrapping_neg() as isize) }) as i8;
    if (id as i32) != (api as i32) {
        _PyObject_DebugDumpAddress(p);
        unsafe {
            _Py_FatalErrorFormat(
                func,
                c"bad ID: Allocated using API '%c', verified using API '%c'".as_ptr(),
                id as i32,
                api as i32,
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    }
    // Check the stuff at the start of p first:  if there's underwrite
    // corruption, the number-of-bytes field may be nuts, and checking
    // the tail could lead to a segfault then.
    i = 7i32;
    while i >= 1 {
        if ((unsafe { *q.wrapping_offset(-(i as isize)) }) as i32) != 253 {
            _PyObject_DebugDumpAddress(p);
            unsafe { _Py_FatalErrorFunc(func, c"bad leading pad byte".as_ptr()) };
            unsafe { core::hint::unreachable_unchecked() };
        }
        i = i.wrapping_sub(1);
    }
    nbytes = read_size_t(q.wrapping_offset(-(16i32 as isize)) as *const ());
    let tail: *const u8 = q.wrapping_offset(nbytes as isize);
    i = 0;
    while i < 8 {
        if ((unsafe { *tail.wrapping_offset(i as isize) }) as i32) != 253 {
            _PyObject_DebugDumpAddress(p);
            unsafe { _Py_FatalErrorFunc(func, c"bad trailing pad byte".as_ptr()) };
            unsafe { core::hint::unreachable_unchecked() };
        }
        i = i.wrapping_add(1);
    }
}

/// Display info to stderr about the memory block at p.
fn _PyObject_DebugDumpAddress(p: *const ()) {
    let mut q: *const u8 = p as *const u8;
    let mut nbytes: u64 = 0;
    let mut i: i32 = 1;
    let mut ok: i32 = 1;
    let mut id: i8 = 0;
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            slate_support::stderr(),
            c"Debug memory block at address p=%p:".as_ptr(),
            p,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    if p.is_null() {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fprintf(slate_support::stderr(), c"\n".as_ptr()) };
        unsafe { fflush(core::ptr::null_mut()) };
        return;
    }
    id = (unsafe { *q.wrapping_offset(8i32.wrapping_neg() as isize) }) as i8;
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fprintf(slate_support::stderr(), c" API '%c'\n".as_ptr(), id as i32) };
    unsafe { fflush(core::ptr::null_mut()) };
    nbytes = read_size_t(q.wrapping_offset(-(16i32 as isize)) as *const ());
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            slate_support::stderr(),
            c"    %zu bytes originally requested\n".as_ptr(),
            nbytes,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    // In case this is nuts, check the leading pad bytes first.
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            slate_support::stderr(),
            c"    The %d pad bytes at p-%d are ".as_ptr(),
            7i32,
            7i32,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    while i <= 7i32 {
        if ((unsafe { *q.wrapping_offset(-(i as isize)) }) as i32) != 253 {
            ok = 0;
            break;
        }
        i = i.wrapping_add(1);
    }
    if ok != 0 {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe {
            slate_support::fputs(
                c"FORBIDDENBYTE, as expected.\n".as_ptr(),
                slate_support::stderr(),
            )
        };
        unsafe { fflush(core::ptr::null_mut()) };
    } else {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe {
            slate_support::fprintf(
                slate_support::stderr(),
                c"not all FORBIDDENBYTE (0x%02x):\n".as_ptr(),
                253i32,
            )
        };
        unsafe { fflush(core::ptr::null_mut()) };
        i = 7i32;
        while i >= 1 {
            let byte: u8 = unsafe { *q.wrapping_offset(-(i as isize)) };
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c"        at p-%d: 0x%02x".as_ptr(),
                    i,
                    byte as i32,
                )
            };
            unsafe { fflush(core::ptr::null_mut()) };
            if (byte as i32) != 253 {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe { slate_support::fputs(c" *** OUCH".as_ptr(), slate_support::stderr()) };
                unsafe { fflush(core::ptr::null_mut()) };
            }
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fputc(10i32, slate_support::stderr()) };
            unsafe { fflush(core::ptr::null_mut()) };
            i = i.wrapping_sub(1);
        }
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe {
            slate_support::fputs(c"    Because memory is corrupted at the start, the count of bytes requested\n       may be bogus, and checking the trailing pad bytes may segfault.\n".as_ptr(), slate_support::stderr())
        };
        unsafe { fflush(core::ptr::null_mut()) };
    }
    let tail: *const u8 = q.wrapping_offset(nbytes as isize);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            slate_support::stderr(),
            c"    The %d pad bytes at tail=%p are ".as_ptr(),
            8i32,
            tail as *mut (),
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    ok = 1;
    i = 0;
    while i < 8 {
        if ((unsafe { *tail.wrapping_offset(i as isize) }) as i32) != 253 {
            ok = 0;
            break;
        }
        i = i.wrapping_add(1);
    }
    if ok != 0 {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe {
            slate_support::fputs(
                c"FORBIDDENBYTE, as expected.\n".as_ptr(),
                slate_support::stderr(),
            )
        };
        unsafe { fflush(core::ptr::null_mut()) };
    } else {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe {
            slate_support::fprintf(
                slate_support::stderr(),
                c"not all FORBIDDENBYTE (0x%02x):\n".as_ptr(),
                253i32,
            )
        };
        unsafe { fflush(core::ptr::null_mut()) };
        i = 0;
        while i < 8 {
            let byte: u8 = unsafe { *tail.wrapping_offset(i as isize) };
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c"        at tail+%d: 0x%02x".as_ptr(),
                    i,
                    byte as i32,
                )
            };
            unsafe { fflush(core::ptr::null_mut()) };
            if (byte as i32) != 253 {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe { slate_support::fputs(c" *** OUCH".as_ptr(), slate_support::stderr()) };
                unsafe { fflush(core::ptr::null_mut()) };
            }
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fputc(10i32, slate_support::stderr()) };
            unsafe { fflush(core::ptr::null_mut()) };
            i = i.wrapping_add(1);
        }
    }
    if nbytes > 0 {
        i = 0;
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fputs(c"    Data at p:".as_ptr(), slate_support::stderr()) };
        unsafe { fflush(core::ptr::null_mut()) };
        // print up to 8 bytes at the start
        while q < tail && i < 8 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(slate_support::stderr(), c" %02x".as_ptr(), *q as i32)
            };
            unsafe { fflush(core::ptr::null_mut()) };
            i = i.wrapping_add(1);
            let __v14: *const u8 = q;
            q = __v14.wrapping_offset(1 as isize);
        }
        // and up to 8 at the end
        if q < tail {
            if ((unsafe { tail.offset_from(q as *const u8) }) as i64) > 8 {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe { slate_support::fputs(c" ...".as_ptr(), slate_support::stderr()) };
                unsafe { fflush(core::ptr::null_mut()) };
                q = tail.wrapping_offset(-(8 as isize));
            }
            while q < tail {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe {
                    slate_support::fprintf(slate_support::stderr(), c" %02x".as_ptr(), *q as i32)
                };
                unsafe { fflush(core::ptr::null_mut()) };
                let __v17: *const u8 = q;
                q = __v17.wrapping_offset(1 as isize);
            }
        }
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fputc(10i32, slate_support::stderr()) };
        unsafe { fflush(core::ptr::null_mut()) };
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputc(10i32, slate_support::stderr()) };
    unsafe { fflush(core::ptr::null_mut()) };
    unsafe { fflush(slate_support::stderr()) };
    unsafe { _PyMem_DumpTraceback(fileno(slate_support::stderr()), p) };
}

fn printone(out: *mut slate_support::FILE, msg: *const i8, mut value: u64) -> u64 {
    let mut i: i32 = 0;
    let mut k: i32 = 3;
    let mut buf: __SlateAlign16<[i8; 100]> = __SlateAlign16([0i8; 100]);
    let origvalue: u64 = value;
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputs(msg, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    i = (unsafe { strlen(msg as *const core::ffi::c_char) as u64 }) as i32;
    while i < 35 {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fputc(32i32, out) };
        unsafe { fflush(core::ptr::null_mut()) };
        i = i.wrapping_add(1);
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputc(61i32, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    // Write the value with commas.
    i = 22;
    let __v3: i32 = i;
    i = __v3.wrapping_sub(1);
    buf.0[__v3 as usize] = 0;
    let __v5: i32 = i;
    i = __v5.wrapping_sub(1);
    buf.0[__v5 as usize] = 10;
    loop {
        let nextvalue: u64 = value / 10;
        let digit: u32 = value.wrapping_sub(nextvalue.wrapping_mul(10)) as u32;
        value = nextvalue;
        let __v7: i32 = i;
        i = __v7.wrapping_sub(1);
        buf.0[__v7 as usize] = digit.wrapping_add(48) as i8;
        k = k.wrapping_sub(1);
        if k == 0 && value != 0 && i >= 0 {
            k = 3;
            let __v11: i32 = i;
            i = __v11.wrapping_sub(1);
            buf.0[__v11 as usize] = 44;
        }
        if !(value != 0 && i >= 0) {
            break;
        }
    }
    while i >= 0 {
        let __v14: i32 = i;
        i = __v14.wrapping_sub(1);
        buf.0[__v14 as usize] = 32;
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputs(buf.0.as_mut_ptr() as *const i8, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    origvalue
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDebugAllocatorStats(
    out: *mut slate_support::FILE,
    block_name: *const i8,
    num_blocks: i32,
    sizeof_block: u64,
) {
    let mut buf1: __SlateAlign16<[i8; 128]> = __SlateAlign16([0i8; 128]);
    let mut buf2: __SlateAlign16<[i8; 128]> = __SlateAlign16([0i8; 128]);
    unsafe {
        PyOS_snprintf(
            buf1.0.as_mut_ptr() as *mut i8,
            core::mem::size_of::<[i8; 128]>() as u64,
            c"%d %ss * %zd bytes each".as_ptr(),
            num_blocks,
            block_name,
            sizeof_block,
        )
    };
    unsafe {
        PyOS_snprintf(
            buf2.0.as_mut_ptr() as *mut i8,
            core::mem::size_of::<[i8; 128]>() as u64,
            c"%48s ".as_ptr(),
            buf1.0.as_mut_ptr() as *mut i8,
        )
    };
    printone(
        out,
        buf2.0.as_mut_ptr() as *const i8,
        (num_blocks as u64).wrapping_mul(sizeof_block),
    );
}

/// Return true if the obmalloc state structure is heap allocated,
///
/// by PyMem_RawCalloc().  For the main interpreter, this structure
///
/// allocated in the BSS.  Allocating that way gives some memory savings
///
/// and a small performance win (at least on a demand paged OS).  On
///
/// 64-bit platforms, the obmalloc structure is 256 kB. Most of that
///
/// memory is for the arena_map_top array.  Since normally only one entry
///
/// of that array is used, only one page of resident memory is actually
///
/// used, rather than the full 256 kB.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_obmalloc_state_on_heap(interp: *mut _is) -> bool {
    !unsafe { (*interp).obmalloc }.is_null()
        && (unsafe { (*interp).obmalloc })
            != unsafe { core::ptr::addr_of_mut!(obmalloc_state_main) }
}

fn init_obmalloc_pools(interp: *mut _is) {
    // initialize the obmalloc->pools structure.  This must be done
    //
    // before the obmalloc alloc/free functions can be called.
    let mut temp: __SlateAlign16<[*mut pool_header; 64]> = __SlateAlign16([
        PTA!((*unsafe { (*interp).obmalloc }).pools, 0i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 0i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 1i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 1i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 2i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 2i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 3i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 3i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 4i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 4i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 5i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 5i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 6i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 6i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 7i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 7i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 8i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 8i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 9i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 9i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 10i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 10i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 11i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 11i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 12i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 12i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 13i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 13i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 14i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 14i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 15i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 15i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 16i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 16i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 17i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 17i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 18i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 18i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 19i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 19i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 20i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 20i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 21i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 21i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 22i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 22i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 23i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 23i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 24i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 24i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 25i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 25i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 26i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 26i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 27i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 27i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 28i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 28i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 29i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 29i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 30i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 30i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 31i32),
        PTA!((*unsafe { (*interp).obmalloc }).pools, 31i32),
    ]);
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((temp.0.as_mut_ptr() as *const ()) as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!((*(*interp).obmalloc).pools.used) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<[*mut pool_header; 64]>() as u64) as usize) as usize,
        )
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMem_init_obmalloc(interp: *mut _is) -> i32 {
    // Initialize obmalloc, but only for subinterpreters,
    // since the main interpreter is initialized statically.
    let __v0: bool = _Py_IsMainInterpreter(interp) != 0
        || (unsafe { _PyInterpreterState_HasFeature(interp, 32u64) }) != 0;
    if __v0 {
        unsafe {
            (*interp).obmalloc = core::ptr::addr_of_mut!(obmalloc_state_main);
        }
        if !unsafe { obmalloc_state_initialized } {
            init_obmalloc_pools(interp);
            unsafe {
                obmalloc_state_initialized = true;
            }
        }
    } else {
        unsafe {
            (*interp).obmalloc =
                PyMem_RawCalloc(1u64, core::mem::size_of::<_obmalloc_state>() as u64)
                    as *mut _obmalloc_state;
        }
        if unsafe { (*interp).obmalloc }.is_null() {
            return 1i32.wrapping_neg();
        }
        init_obmalloc_pools(interp);
    }
    return 0; // success
}

fn free_obmalloc_arenas(interp: *mut _is) {
    let state: *mut _obmalloc_state = unsafe { (*interp).obmalloc };
    let mut i: u32 = 0;
    while i < unsafe { (*state).mgmt.maxarenas } {
        // free each obmalloc memory arena
        let ao: *mut arena_object = unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize);
        unsafe {
            unsafe { _PyRuntime.allocators.obj_arena.free }.unwrap()(
                _PyRuntime.allocators.obj_arena.ctx,
                (*ao).address as *mut (),
                1048576u64,
            )
        };
        i = i.wrapping_add(1);
    }
    // free the array containing pointers to all arenas
    PyMem_RawFree((unsafe { (*state).mgmt.arenas }) as *mut ());
    // Free the middle and bottom nodes of the radix tree.  These are allocated
    //
    // by arena_map_mark_used() but not freed when arenas are freed.
    for i1 in 0..32768i32 {
        let mid: *mut arena_map_mid = unsafe {
            *((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
                .wrapping_offset(i1 as isize)
        };
        if mid.is_null() {
            continue;
        }
        for i2 in 0..32768i32 {
            let bot: *mut arena_map_bot = unsafe {
                *((*(*((*state).usage.arena_map_root.ptrs.as_mut_ptr() as *mut *mut arena_map_mid)
                    .wrapping_offset(i1 as isize)))
                .ptrs
                .as_mut_ptr() as *mut *mut arena_map_bot)
                    .wrapping_offset(i2 as isize)
            };
            if bot.is_null() {
                continue;
            }
            PyMem_RawFree(bot as *mut ());
        }
        PyMem_RawFree(mid as *mut ());
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _alloc_stats {
    allocated_blocks: u64,
    allocated_bytes: u64,
    allocated_with_overhead: u64,
    bytes_reserved: u64,
    bytes_committed: u64,
}

#[unsafe(link_section = ".text.slate_distinct.obmalloc._collect_alloc_stats")]
extern "C-unwind" fn _collect_alloc_stats(
    heap: *const mi_heap_s,
    area: *const mi_heap_area_s,
    block: *mut (),
    block_size: u64,
    arg: *mut (),
) -> bool {
    let __v1: u64 = unsafe { (*(arg as *mut _alloc_stats)).allocated_blocks };
    unsafe {
        (*(arg as *mut _alloc_stats)).allocated_blocks = __v1.wrapping_add((*area).used);
    }
    let __v4: u64 = unsafe { (*(arg as *mut _alloc_stats)).allocated_bytes };
    unsafe {
        (*(arg as *mut _alloc_stats)).allocated_bytes =
            __v4.wrapping_add(unsafe { (*area).used }.wrapping_mul((*area).block_size));
    }
    let __v7: u64 = unsafe { (*(arg as *mut _alloc_stats)).allocated_with_overhead };
    unsafe {
        (*(arg as *mut _alloc_stats)).allocated_with_overhead =
            __v7.wrapping_add(unsafe { (*area).used }.wrapping_mul((*area).full_block_size));
    }
    let __v10: u64 = unsafe { (*(arg as *mut _alloc_stats)).bytes_reserved };
    unsafe {
        (*(arg as *mut _alloc_stats)).bytes_reserved = __v10.wrapping_add((*area).reserved);
    }
    let __v13: u64 = unsafe { (*(arg as *mut _alloc_stats)).bytes_committed };
    unsafe {
        (*(arg as *mut _alloc_stats)).bytes_committed = __v13.wrapping_add((*area).committed);
    }
    true
}

fn py_mimalloc_print_stats(out: *mut slate_support::FILE) {
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"Small block threshold = %zu, in %u size classes.\n".as_ptr(),
            16384u64,
            73u32,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fprintf(out, c"Medium block threshold = %zu\n".as_ptr(), 131072u64) };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fprintf(out, c"Large object max size = %zu\n".as_ptr(), 16777216u64) };
    unsafe { fflush(core::ptr::null_mut()) };
    let heap: *mut mi_heap_s = mi_heap_get_default();
    let mut stats: _alloc_stats = unsafe { core::mem::zeroed() };
    unsafe {
        core::ptr::write_bytes(
            ((core::ptr::addr_of_mut!(stats) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<_alloc_stats>() as u64) as usize) as usize,
        )
    };
    mi_heap_visit_blocks(
        heap as *const mi_heap_s,
        false,
        Some(_collect_alloc_stats),
        core::ptr::addr_of_mut!(stats) as *mut (),
    );
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"    Allocated Blocks: %zd\n".as_ptr(),
            stats.allocated_blocks,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"    Allocated Bytes: %zd\n".as_ptr(),
            stats.allocated_bytes,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"    Allocated Bytes w/ Overhead: %zd\n".as_ptr(),
            stats.allocated_with_overhead,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"    Bytes Reserved: %zd\n".as_ptr(),
            stats.bytes_reserved,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"    Bytes Committed: %zd\n".as_ptr(),
            stats.bytes_committed,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
}

fn pymalloc_print_stats(out: *mut slate_support::FILE) {
    let state: *mut _obmalloc_state = get_state();
    let mut i: u32 = 0;
    let numclasses: u32 = 32u32;
    // # of pools, allocated blocks, and free blocks per class index
    let mut numpools: __SlateAlign16<[u64; 32]> = __SlateAlign16([0u64; 32]);
    let mut numblocks: __SlateAlign16<[u64; 32]> = __SlateAlign16([0u64; 32]);
    let mut numfreeblocks: __SlateAlign16<[u64; 32]> = __SlateAlign16([0u64; 32]);
    // total # of allocated bytes in used and full pools
    let mut allocated_bytes: u64 = 0;
    // total # of available bytes in used pools
    let mut available_bytes: u64 = 0;
    // # of free pools + pools not yet carved out of current arena
    let mut numfreepools: u32 = 0;
    // # of bytes for arena alignment padding
    let mut arena_alignment: u64 = 0;
    // # of bytes in used and full pools used for pool_headers
    let mut pool_header_bytes: u64 = 0;
    // # of bytes in used and full pools wasted due to quantization,
    // i.e. the necessarily leftover space at the ends of used and
    // full pools.
    let mut quantization: u64 = 0;
    // # of arenas actually allocated.
    let mut narenas: u64 = 0;
    // running total -- should equal narenas * ARENA_SIZE
    let mut total: u64 = 0;
    let mut buf: __SlateAlign16<[i8; 128]> = __SlateAlign16([0i8; 128]);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            out,
            c"Small block threshold = %d, in %u size classes.\n".as_ptr(),
            512i32,
            numclasses,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    while i < numclasses {
        numfreeblocks.0[i as usize] = 0;
        numblocks.0[i as usize] = 0;
        numpools.0[i as usize] = 0;
        i = i.wrapping_add(1);
    }
    // Because full pools aren't linked to from anything, it's easiest
    // to march over all the arenas.  If we're lucky, most of the memory
    // will be living in full pools -- would be a shame to miss them.
    i = 0;
    while i < unsafe { (*state).mgmt.maxarenas } {
        let mut base: u64 =
            unsafe { (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).address };
        // Skip arenas which are not allocated.
        if (unsafe { (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).address }) != 0
        {
            narenas = narenas.wrapping_add(1);
            numfreepools = numfreepools.wrapping_add(unsafe {
                (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).nfreepools
            });
            // round up to pool alignment
            if base & 16383u64 != 0 {
                arena_alignment = arena_alignment.wrapping_add(16384u64);
                let __v11: u64 = base & 18446744073709535232u64;
                base = __v11;
                base = base.wrapping_add(16384u64);
            }
            // visit every pool in the arena
            while base
                < ((unsafe {
                    (*unsafe { (*state).mgmt.arenas }.wrapping_offset(i as isize)).pool_address
                }) as u64)
            {
                let sz: u32 = unsafe { (*(base as *mut pool_header)).szidx };
                let mut freeblocks: u32 = 0;
                if (unsafe { (*(base as *mut pool_header)).r#ref.count }) == 0 {
                    // currently unused
                } else {
                    let __v15: *mut u64 =
                        (numpools.0.as_mut_ptr() as *mut u64).wrapping_offset(sz as isize);
                    let __v16: u64 = unsafe { *__v15 };
                    unsafe {
                        *__v15 = __v16.wrapping_add(1);
                    }
                    let __v18: *mut u64 =
                        (numblocks.0.as_mut_ptr() as *mut u64).wrapping_offset(sz as isize);
                    let __v19: u64 = unsafe { *__v18 };
                    unsafe {
                        *__v18 =
                            __v19.wrapping_add((*(base as *mut pool_header)).r#ref.count as u64);
                    }
                    freeblocks = NUMBLOCKS!(sz)
                        .wrapping_sub(unsafe { (*(base as *mut pool_header)).r#ref.count });
                    let __v21: *mut u64 =
                        (numfreeblocks.0.as_mut_ptr() as *mut u64).wrapping_offset(sz as isize);
                    let __v22: u64 = unsafe { *__v21 };
                    unsafe {
                        *__v21 = __v22.wrapping_add(freeblocks as u64);
                    }
                }
                base = base.wrapping_add(16384u64);
            }
        }
        i = i.wrapping_add(1);
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputc(10i32, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fputs(c"class   size   num pools   blocks in use  avail blocks\n-----   ----   ---------   -------------  ------------\n".as_ptr(), out)
    };
    unsafe { fflush(core::ptr::null_mut()) };
    i = 0;
    while i < numclasses {
        let p: u64 = numpools.0[i as usize];
        let b: u64 = numblocks.0[i as usize];
        let f: u64 = numfreeblocks.0[i as usize];
        let size: u32 = INDEX2SIZE!(i);
        if p != 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    out,
                    c"%5u %6u %11zu %15zu %13zu\n".as_ptr(),
                    i,
                    size,
                    p,
                    b,
                    f,
                )
            };
            unsafe { fflush(core::ptr::null_mut()) };
            allocated_bytes = allocated_bytes.wrapping_add(b.wrapping_mul(size as u64));
            available_bytes = available_bytes.wrapping_add(f.wrapping_mul(size as u64));
            let __v33: u64 = pool_header_bytes;
            pool_header_bytes = __v33.wrapping_add(p.wrapping_mul(_Py_SIZE_ROUND_UP!(
                core::mem::size_of::<pool_header>() as u64,
                16i32
            )));
            let __v35: u64 = quantization;
            quantization = __v35.wrapping_add(p.wrapping_mul(
                16384u64.wrapping_sub(_Py_SIZE_ROUND_UP!(
                    core::mem::size_of::<pool_header>() as u64,
                    16i32
                )) % (size as u64),
            ));
        }
        i = i.wrapping_add(1);
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputc(10i32, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    printone(out, c"# arenas allocated total".as_ptr(), unsafe {
        (*state).mgmt.ntimes_arena_allocated
    });
    printone(
        out,
        c"# arenas reclaimed".as_ptr(),
        unsafe { (*state).mgmt.ntimes_arena_allocated }.wrapping_sub(narenas),
    );
    printone(out, c"# arenas highwater mark".as_ptr(), unsafe {
        (*state).mgmt.narenas_highwater
    });
    printone(out, c"# arenas allocated current".as_ptr(), narenas);
    unsafe {
        PyOS_snprintf(
            buf.0.as_mut_ptr() as *mut i8,
            core::mem::size_of::<[i8; 128]>() as u64,
            c"%zu arenas * %d bytes/arena".as_ptr(),
            narenas,
            1048576i32,
        )
    };
    printone(
        out,
        buf.0.as_mut_ptr() as *const i8,
        narenas.wrapping_mul(1048576u64),
    );
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputc(10i32, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    // Account for what all of those arena bytes are being used for.
    total = printone(
        out,
        c"# bytes in allocated blocks".as_ptr(),
        allocated_bytes,
    );
    let __v39: u64 = printone(
        out,
        c"# bytes in available blocks".as_ptr(),
        available_bytes,
    );
    total = total.wrapping_add(__v39);
    unsafe {
        PyOS_snprintf(
            buf.0.as_mut_ptr() as *mut i8,
            core::mem::size_of::<[i8; 128]>() as u64,
            c"%u unused pools * %d bytes".as_ptr(),
            numfreepools,
            16384i32,
        )
    };
    let __v42: u64 = printone(
        out,
        buf.0.as_mut_ptr() as *const i8,
        (numfreepools as u64).wrapping_mul(16384u64),
    );
    total = total.wrapping_add(__v42);
    let __v45: u64 = printone(
        out,
        c"# bytes lost to pool headers".as_ptr(),
        pool_header_bytes,
    );
    total = total.wrapping_add(__v45);
    let __v48: u64 = printone(out, c"# bytes lost to quantization".as_ptr(), quantization);
    total = total.wrapping_add(__v48);
    let __v51: u64 = printone(
        out,
        c"# bytes lost to arena alignment".as_ptr(),
        arena_alignment,
    );
    total = total.wrapping_add(__v51);
    printone(out, c"Total".as_ptr(), total);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputs(c"\narena map counts\n".as_ptr(), out) };
    unsafe { fflush(core::ptr::null_mut()) };
    printone(
        out,
        c"# arena map mid nodes".as_ptr(),
        (unsafe { (*state).usage.arena_map_mid_count }) as u64,
    );
    printone(
        out,
        c"# arena map bot nodes".as_ptr(),
        (unsafe { (*state).usage.arena_map_bot_count }) as u64,
    );
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fputc(10i32, out) };
    unsafe { fflush(core::ptr::null_mut()) };
    total = printone(
        out,
        c"# bytes lost to arena map root".as_ptr(),
        core::mem::size_of::<arena_map_top>() as u64,
    );
    let __v54: u64 = printone(
        out,
        c"# bytes lost to arena map mid".as_ptr(),
        (core::mem::size_of::<arena_map_mid>() as u64)
            .wrapping_mul((unsafe { (*state).usage.arena_map_mid_count }) as u64),
    );
    total = total.wrapping_add(__v54);
    let __v57: u64 = printone(
        out,
        c"# bytes lost to arena map bot".as_ptr(),
        (core::mem::size_of::<arena_map_bot>() as u64)
            .wrapping_mul((unsafe { (*state).usage.arena_map_bot_count }) as u64),
    );
    total = total.wrapping_add(__v57);
    printone(out, c"Total".as_ptr(), total);
}

/// Print summary info to "out" about the state of pymalloc's structures.
/// In Py_DEBUG mode, also perform some expensive internal consistency
/// checks.
///
/// Return 0 if the memory debug hooks are not installed or no statistics was
/// written into out, return 1 otherwise.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_DebugMallocStats(out: *mut slate_support::FILE) -> i32 {
    if _PyMem_MimallocEnabled() != 0 {
        py_mimalloc_print_stats(out);
        return 1;
    } else {
        if _PyMem_PymallocEnabled() != 0 {
            pymalloc_print_stats(out);
            return 1;
        } else {
            return 0;
        }
    }
}
