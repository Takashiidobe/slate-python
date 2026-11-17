//! The implementation of the hash table (_Py_hashtable_t) is based on the
//! cfuhash project:
//! http://sourceforge.net/projects/libcfu/
//!
//! Copyright of cfuhash:
//! Creation date: 2005-06-24 21:22:40
//! Authors: Don
//! Change log:
//!
//! Copyright (c) 2005 Don Owens
//! All rights reserved.
//!
//! This code is released under the BSD license:
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions
//! are met:
//!
//!   * Redistributions of source code must retain the above copyright
//!     notice, this list of conditions and the following disclaimer.
//!
//!   * Redistributions in binary form must reproduce the above
//!     copyright notice, this list of conditions and the following
//!     disclaimer in the documentation and/or other materials provided
//!     with the distribution.
//!
//!   * Neither the name of the author nor the names of its
//!     contributors may be used to endorse or promote products derived
//!     from this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
//! "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
//! LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
//! FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
//! COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
//! INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
//! (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
//! HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
//! STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
//! ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
//! OF THE POSSIBILITY OF SUCH DAMAGE.
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
macro_rules! BUCKETS_HEAD {
    ($SLIST:expr) => {
        _Py_SLIST_HEAD!(unsafe { core::ptr::addr_of_mut!($SLIST) })
            as *mut __SlateRecord_pycore_hashtable_h_35
    };
}

macro_rules! ENTRY_NEXT {
    ($ENTRY:expr) => {
        _Py_SLIST_ITEM_NEXT!($ENTRY) as *mut __SlateRecord_pycore_hashtable_h_35
    };
}

macro_rules! TABLE_HEAD {
    ($HT:expr, $BUCKET:expr) => {
        _Py_SLIST_HEAD!(unsafe { (*$HT).buckets }.wrapping_offset(($BUCKET) as isize))
            as *mut __SlateRecord_pycore_hashtable_h_35
    };
}

/// Prevent using an expression as a l-value.
///
/// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
macro_rules! _Py_RVALUE {
    ($EXPR:expr) => {
        $EXPR
    };
}

macro_rules! _Py_SLIST_HEAD {
    ($SLIST:expr) => {
        _Py_RVALUE!(unsafe { (*$SLIST).head })
    };
}

macro_rules! _Py_SLIST_ITEM_NEXT {
    ($ITEM:expr) => {
        _Py_RVALUE!(unsafe { (*(($ITEM) as *mut _Py_slist_item_s)).next })
    };
}

unsafe extern "C" {
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
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

use slate_support::memset;

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

// export _Py_hashtable_new()
// _Py_HashPointerRaw()
fn _Py_slist_init(list: *mut __SlateRecord_pycore_hashtable_h_19) {
    unsafe {
        (*list).head = core::ptr::null_mut::<_Py_slist_item_s>();
    }
}

fn _Py_slist_prepend(list: *mut __SlateRecord_pycore_hashtable_h_19, item: *mut _Py_slist_item_s) {
    unsafe {
        (*item).next = (*list).head;
    }
    unsafe {
        (*list).head = item;
    }
}

fn _Py_slist_remove(
    list: *mut __SlateRecord_pycore_hashtable_h_19,
    previous: *mut _Py_slist_item_s,
    item: *mut _Py_slist_item_s,
) {
    if !previous.is_null() {
        unsafe {
            (*previous).next = (*item).next;
        }
    } else {
        unsafe {
            (*list).head = (*item).next;
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.hashtable._Py_hashtable_hash_ptr")]
extern "C-unwind" fn _Py_hashtable_hash_ptr(key: *const ()) -> u64 {
    _Py_HashPointerRaw(key) as u64
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.hashtable._Py_hashtable_compare_direct")]
extern "C-unwind" fn _Py_hashtable_compare_direct(key1: *const (), key2: *const ()) -> i32 {
    (key1 == key2) as i32
}

/// makes sure the real size of the buckets array is a power of 2
fn round_size(s: u64) -> u64 {
    let mut i: u64 = 1;
    if s < 16 {
        return 16;
    }
    while i < s {
        i <<= 1i32;
    }
    i
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_size(ht: *const _Py_hashtable_t) -> u64 {
    let mut size: u64 = core::mem::size_of::<_Py_hashtable_t>() as u64;
    // buckets
    size = size.wrapping_add(
        unsafe { (*ht).nbuckets }
            .wrapping_mul(core::mem::size_of::<*mut __SlateRecord_pycore_hashtable_h_35>() as u64),
    );
    // entries
    size = size.wrapping_add(
        unsafe { (*ht).nentries }
            .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_hashtable_h_35>() as u64),
    );
    size
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_len(ht: *const _Py_hashtable_t) -> u64 {
    unsafe { (*ht).nentries }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.hashtable._Py_hashtable_get_entry_generic")]
extern "C-unwind" fn _Py_hashtable_get_entry_generic(
    ht: *mut _Py_hashtable_t,
    key: *const (),
) -> *mut __SlateRecord_pycore_hashtable_h_35 {
    let key_hash: u64 = unsafe { unsafe { (*ht).hash_func }.unwrap()(key) };
    let index: u64 = key_hash & unsafe { (*ht).nbuckets }.wrapping_sub(1);
    let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 = TABLE_HEAD!(ht, index);
    while true {
        if entry.is_null() {
            return core::ptr::null_mut::<__SlateRecord_pycore_hashtable_h_35>();
        }
        let __v0: bool = (unsafe { (*entry).key_hash }) == key_hash
            && (unsafe { unsafe { (*ht).compare_func }.unwrap()(key, (*entry).key as *const ()) })
                != 0;
        if __v0 {
            break;
        }
        entry = ENTRY_NEXT!(entry);
    }
    entry
}

/// Specialized for:
///
/// hash_func == _Py_hashtable_hash_ptr
///
/// compare_func == _Py_hashtable_compare_direct
#[unsafe(link_section = ".text.slate_distinct.hashtable._Py_hashtable_get_entry_ptr")]
extern "C-unwind" fn _Py_hashtable_get_entry_ptr(
    ht: *mut _Py_hashtable_t,
    key: *const (),
) -> *mut __SlateRecord_pycore_hashtable_h_35 {
    let key_hash: u64 = _Py_hashtable_hash_ptr(key);
    let index: u64 = key_hash & unsafe { (*ht).nbuckets }.wrapping_sub(1);
    let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 = TABLE_HEAD!(ht, index);
    while true {
        if entry.is_null() {
            return core::ptr::null_mut::<__SlateRecord_pycore_hashtable_h_35>();
        }
        // Compare directly keys (ignore entry->key_hash)
        if (unsafe { (*entry).key }) == (key as *mut ()) {
            break;
        }
        entry = ENTRY_NEXT!(entry);
    }
    entry
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_steal(ht: *mut _Py_hashtable_t, key: *const ()) -> *mut () {
    let key_hash: u64 = unsafe { unsafe { (*ht).hash_func }.unwrap()(key) };
    let index: u64 = key_hash & unsafe { (*ht).nbuckets }.wrapping_sub(1);
    let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 = TABLE_HEAD!(ht, index);
    let mut previous: *mut __SlateRecord_pycore_hashtable_h_35 =
        core::ptr::null_mut::<__SlateRecord_pycore_hashtable_h_35>();
    while true {
        if entry.is_null() {
            // not found
            return core::ptr::null_mut::<()>();
        }
        let __v0: bool = (unsafe { (*entry).key_hash }) == key_hash
            && (unsafe { unsafe { (*ht).compare_func }.unwrap()(key, (*entry).key as *const ()) })
                != 0;
        if __v0 {
            break;
        }
        previous = entry;
        entry = ENTRY_NEXT!(entry);
    }
    _Py_slist_remove(
        unsafe { (*ht).buckets }.wrapping_offset(index as isize),
        previous as *mut _Py_slist_item_s,
        entry as *mut _Py_slist_item_s,
    );
    let __v3: u64 = unsafe { (*ht).nentries };
    unsafe {
        (*ht).nentries = __v3.wrapping_sub(1);
    }
    let value: *mut () = unsafe { (*entry).value };
    unsafe { unsafe { (*ht).alloc.free }.unwrap()(entry as *mut ()) };
    if ((((unsafe { (*ht).nentries }) as f32) / ((unsafe { (*ht).nbuckets }) as f32)) as f64)
        < 0.1f64
    {
        // Ignore failure: error cannot be reported to the caller
        hashtable_rehash(ht);
    }
    value
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_set(
    ht: *mut _Py_hashtable_t,
    key: *const (),
    value: *mut (),
) -> i32 {
    let entry: *mut __SlateRecord_pycore_hashtable_h_35 = (unsafe {
        unsafe { (*ht).alloc.malloc }.unwrap()(core::mem::size_of::<
            __SlateRecord_pycore_hashtable_h_35,
        >() as u64)
    })
        as *mut __SlateRecord_pycore_hashtable_h_35;
    if entry.is_null() {
        // memory allocation failed
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*entry).key_hash = unsafe { (*ht).hash_func }.unwrap()(key);
    }
    unsafe {
        (*entry).key = key as *mut ();
    }
    unsafe {
        (*entry).value = value;
    }
    let __v1: u64 = unsafe { (*ht).nentries };
    unsafe {
        (*ht).nentries = __v1.wrapping_add(1);
    }
    if ((((unsafe { (*ht).nentries }) as f32) / ((unsafe { (*ht).nbuckets }) as f32)) as f64)
        > 0.5f64
    {
        if hashtable_rehash(ht) < 0 {
            let __v4: u64 = unsafe { (*ht).nentries };
            unsafe {
                (*ht).nentries = __v4.wrapping_sub(1);
            }
            unsafe { unsafe { (*ht).alloc.free }.unwrap()(entry as *mut ()) };
            return 1i32.wrapping_neg();
        }
    }
    let index: u64 = (unsafe { (*entry).key_hash }) & unsafe { (*ht).nbuckets }.wrapping_sub(1);
    _Py_slist_prepend(
        unsafe { (*ht).buckets }.wrapping_offset(index as isize),
        entry as *mut _Py_slist_item_s,
    );
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_get(ht: *mut _Py_hashtable_t, key: *const ()) -> *mut () {
    let entry: *mut __SlateRecord_pycore_hashtable_h_35 =
        unsafe { unsafe { (*ht).get_entry_func }.unwrap()(ht, key) };
    if !entry.is_null() {
        return unsafe { (*entry).value };
    } else {
        return core::ptr::null_mut::<()>();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_foreach(
    ht: *mut _Py_hashtable_t,
    func: Option<
        unsafe extern "C-unwind" fn(*mut _Py_hashtable_t, *const (), *const (), *mut ()) -> i32,
    >,
    user_data: *mut (),
) -> i32 {
    let mut hv: u64 = 0;
    while hv < unsafe { (*ht).nbuckets } {
        let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 = TABLE_HEAD!(ht, hv);
        while !entry.is_null() {
            let res: i32 = unsafe {
                func.unwrap()(
                    ht,
                    (*entry).key as *const (),
                    (*entry).value as *const (),
                    user_data,
                )
            };
            if res != 0 {
                return res;
            }
            entry = ENTRY_NEXT!(entry);
        }
        hv = hv.wrapping_add(1);
    }
    0
}

/// Forward declaration
fn hashtable_rehash(ht: *mut _Py_hashtable_t) -> i32 {
    let new_size: u64 =
        round_size((((unsafe { (*ht).nentries }) as f64) * 2.0f64 / (0.1f64 + 0.5f64)) as u64);
    if new_size == unsafe { (*ht).nbuckets } {
        return 0;
    }
    let buckets_size: u64 =
        new_size.wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_hashtable_h_19>() as u64);
    let new_buckets: *mut __SlateRecord_pycore_hashtable_h_19 =
        (unsafe { unsafe { (*ht).alloc.malloc }.unwrap()(buckets_size) })
            as *mut __SlateRecord_pycore_hashtable_h_19;
    if new_buckets.is_null() {
        // memory allocation failed
        return 1i32.wrapping_neg();
    }
    unsafe {
        core::ptr::write_bytes(
            ((new_buckets as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            (buckets_size as usize) as usize,
        )
    };
    let mut bucket: u64 = 0;
    while bucket < unsafe { (*ht).nbuckets } {
        let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 =
            BUCKETS_HEAD!(*unsafe { (*ht).buckets }.wrapping_offset(bucket as isize));
        while !entry.is_null() {
            let next: *mut __SlateRecord_pycore_hashtable_h_35 = ENTRY_NEXT!(entry);
            let entry_index: u64 = (unsafe { (*entry).key_hash }) & new_size.wrapping_sub(1);
            _Py_slist_prepend(
                new_buckets.wrapping_offset(entry_index as isize),
                entry as *mut _Py_slist_item_s,
            );
            entry = next;
        }
        bucket = bucket.wrapping_add(1);
    }
    unsafe { unsafe { (*ht).alloc.free }.unwrap()((*ht).buckets as *mut ()) };
    unsafe {
        (*ht).nbuckets = new_size;
    }
    unsafe {
        (*ht).buckets = new_buckets;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_new_full(
    hash_func: Option<unsafe extern "C-unwind" fn(*const ()) -> u64>,
    compare_func: Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
    key_destroy_func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    value_destroy_func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    allocator: *mut __SlateRecord_pycore_hashtable_h_56,
) -> *mut _Py_hashtable_t {
    let mut alloc: __SlateRecord_pycore_hashtable_h_56 = unsafe { core::mem::zeroed() };
    if allocator.is_null() {
        alloc.malloc = unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>>(
                PyMem_Malloc as *const (),
            )
        };
        alloc.free = unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                PyMem_Free as *const (),
            )
        };
    } else {
        alloc = unsafe { *allocator };
    }
    let ht: *mut _Py_hashtable_t =
        (unsafe { alloc.malloc.unwrap()(core::mem::size_of::<_Py_hashtable_t>() as u64) })
            as *mut _Py_hashtable_t;
    if ht.is_null() {
        return ht;
    }
    unsafe {
        (*ht).nbuckets = 16;
    }
    unsafe {
        (*ht).nentries = 0;
    }
    let buckets_size: u64 = unsafe { (*ht).nbuckets }
        .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_hashtable_h_19>() as u64);
    unsafe {
        (*ht).buckets =
            alloc.malloc.unwrap()(buckets_size) as *mut __SlateRecord_pycore_hashtable_h_19;
    }
    if unsafe { (*ht).buckets }.is_null() {
        unsafe { alloc.free.unwrap()(ht as *mut ()) };
        return core::ptr::null_mut::<_Py_hashtable_t>();
    }
    unsafe {
        core::ptr::write_bytes(
            (((*ht).buckets as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            (buckets_size as usize) as usize,
        )
    };
    unsafe {
        (*ht).get_entry_func = Some(_Py_hashtable_get_entry_generic);
    }
    unsafe {
        (*ht).hash_func = hash_func;
    }
    unsafe {
        (*ht).compare_func = compare_func;
    }
    unsafe {
        (*ht).key_destroy_func = key_destroy_func;
    }
    unsafe {
        (*ht).value_destroy_func = value_destroy_func;
    }
    unsafe {
        (*ht).alloc = alloc;
    }
    if (unsafe { (*ht).hash_func }) == Some(_Py_hashtable_hash_ptr)
        && (unsafe { (*ht).compare_func }) == Some(_Py_hashtable_compare_direct)
    {
        unsafe {
            (*ht).get_entry_func = Some(_Py_hashtable_get_entry_ptr);
        }
    }
    ht
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_new(
    hash_func: Option<unsafe extern "C-unwind" fn(*const ()) -> u64>,
    compare_func: Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
) -> *mut _Py_hashtable_t {
    _Py_hashtable_new_full(
        hash_func,
        compare_func,
        None,
        None,
        core::ptr::null_mut::<__SlateRecord_pycore_hashtable_h_56>(),
    )
}

fn _Py_hashtable_destroy_entry(
    ht: *mut _Py_hashtable_t,
    entry: *mut __SlateRecord_pycore_hashtable_h_35,
) {
    if (unsafe { (*ht).key_destroy_func }) != None {
        unsafe { unsafe { (*ht).key_destroy_func }.unwrap()((*entry).key) };
    }
    if (unsafe { (*ht).value_destroy_func }) != None {
        unsafe { unsafe { (*ht).value_destroy_func }.unwrap()((*entry).value) };
    }
    unsafe { unsafe { (*ht).alloc.free }.unwrap()(entry as *mut ()) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_clear(ht: *mut _Py_hashtable_t) {
    let mut i: u64 = 0;
    while i < unsafe { (*ht).nbuckets } {
        let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 = TABLE_HEAD!(ht, i);
        while !entry.is_null() {
            let next: *mut __SlateRecord_pycore_hashtable_h_35 = ENTRY_NEXT!(entry);
            _Py_hashtable_destroy_entry(ht, entry);
            entry = next;
        }
        _Py_slist_init(unsafe { (*ht).buckets }.wrapping_offset(i as isize));
        i = i.wrapping_add(1);
    }
    unsafe {
        (*ht).nentries = 0;
    }
    // Ignore failure: clear function is not expected to fail
    //
    // because of a memory allocation failure.
    hashtable_rehash(ht);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_hashtable_destroy(ht: *mut _Py_hashtable_t) {
    let mut i: u64 = 0;
    while i < unsafe { (*ht).nbuckets } {
        let mut entry: *mut __SlateRecord_pycore_hashtable_h_35 = TABLE_HEAD!(ht, i);
        while !entry.is_null() {
            let entry_next: *mut __SlateRecord_pycore_hashtable_h_35 = ENTRY_NEXT!(entry);
            _Py_hashtable_destroy_entry(ht, entry);
            entry = entry_next;
        }
        i = i.wrapping_add(1);
    }
    unsafe { unsafe { (*ht).alloc.free }.unwrap()((*ht).buckets as *mut ()) };
    unsafe { unsafe { (*ht).alloc.free }.unwrap()(ht as *mut ()) };
}
