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
//! Our own locale-independent ctype.h-like macros
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
#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

/// 0x0 '\x00'
/// 0x1 '\x01'
/// 0x2 '\x02'
/// 0x3 '\x03'
/// 0x4 '\x04'
/// 0x5 '\x05'
/// 0x6 '\x06'
/// 0x7 '\x07'
/// 0x8 '\x08'
/// 0x9 '\t'
/// 0xa '\n'
/// 0xb '\v'
/// 0xc '\f'
/// 0xd '\r'
/// 0xe '\x0e'
/// 0xf '\x0f'
/// 0x10 '\x10'
/// 0x11 '\x11'
/// 0x12 '\x12'
/// 0x13 '\x13'
/// 0x14 '\x14'
/// 0x15 '\x15'
/// 0x16 '\x16'
/// 0x17 '\x17'
/// 0x18 '\x18'
/// 0x19 '\x19'
/// 0x1a '\x1a'
/// 0x1b '\x1b'
/// 0x1c '\x1c'
/// 0x1d '\x1d'
/// 0x1e '\x1e'
/// 0x1f '\x1f'
/// 0x20 ' '
/// 0x21 '!'
/// 0x22 '"'
/// 0x23 '#'
/// 0x24 '$'
/// 0x25 '%'
/// 0x26 '&'
/// 0x27 "'"
/// 0x28 '('
/// 0x29 ')'
/// 0x2a '*'
/// 0x2b '+'
/// 0x2c ','
/// 0x2d '-'
/// 0x2e '.'
/// 0x2f '/'
/// 0x30 '0'
/// 0x31 '1'
/// 0x32 '2'
/// 0x33 '3'
/// 0x34 '4'
/// 0x35 '5'
/// 0x36 '6'
/// 0x37 '7'
/// 0x38 '8'
/// 0x39 '9'
/// 0x3a ':'
/// 0x3b ';'
/// 0x3c '<'
/// 0x3d '='
/// 0x3e '>'
/// 0x3f '?'
/// 0x40 '@'
/// 0x41 'A'
/// 0x42 'B'
/// 0x43 'C'
/// 0x44 'D'
/// 0x45 'E'
/// 0x46 'F'
/// 0x47 'G'
/// 0x48 'H'
/// 0x49 'I'
/// 0x4a 'J'
/// 0x4b 'K'
/// 0x4c 'L'
/// 0x4d 'M'
/// 0x4e 'N'
/// 0x4f 'O'
/// 0x50 'P'
/// 0x51 'Q'
/// 0x52 'R'
/// 0x53 'S'
/// 0x54 'T'
/// 0x55 'U'
/// 0x56 'V'
/// 0x57 'W'
/// 0x58 'X'
/// 0x59 'Y'
/// 0x5a 'Z'
/// 0x5b '['
/// 0x5c '\\'
/// 0x5d ']'
/// 0x5e '^'
/// 0x5f '_'
/// 0x60 '`'
/// 0x61 'a'
/// 0x62 'b'
/// 0x63 'c'
/// 0x64 'd'
/// 0x65 'e'
/// 0x66 'f'
/// 0x67 'g'
/// 0x68 'h'
/// 0x69 'i'
/// 0x6a 'j'
/// 0x6b 'k'
/// 0x6c 'l'
/// 0x6d 'm'
/// 0x6e 'n'
/// 0x6f 'o'
/// 0x70 'p'
/// 0x71 'q'
/// 0x72 'r'
/// 0x73 's'
/// 0x74 't'
/// 0x75 'u'
/// 0x76 'v'
/// 0x77 'w'
/// 0x78 'x'
/// 0x79 'y'
/// 0x7a 'z'
/// 0x7b '{'
/// 0x7c '|'
/// 0x7d '}'
/// 0x7e '~'
/// 0x7f '\x7f'
#[unsafe(no_mangle)]
static _Py_ctype_table: __SlateAlign16<[u32; 256]> = __SlateAlign16([
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 8u32, 8u32, 8u32, 8u32, 8u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    8u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    20u32, 20u32, 20u32, 20u32, 20u32, 20u32, 20u32, 20u32, 20u32, 20u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 18u32, 18u32, 18u32, 18u32, 18u32, 18u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32,
    2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 17u32, 17u32, 17u32, 17u32, 17u32, 17u32, 1u32, 1u32, 1u32, 1u32, 1u32,
    1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    0u32, 0u32, 0u32, 0u32,
]);

#[unsafe(no_mangle)]
static _Py_ctype_tolower: __SlateAlign16<[u8; 256]> = __SlateAlign16([
    0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
    17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8, 30u8, 31u8, 32u8,
    33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8, 44u8, 45u8, 46u8, 47u8, 48u8,
    49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8, 58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8,
    97u8, 98u8, 99u8, 100u8, 101u8, 102u8, 103u8, 104u8, 105u8, 106u8, 107u8, 108u8, 109u8, 110u8,
    111u8, 112u8, 113u8, 114u8, 115u8, 116u8, 117u8, 118u8, 119u8, 120u8, 121u8, 122u8, 91u8, 92u8,
    93u8, 94u8, 95u8, 96u8, 97u8, 98u8, 99u8, 100u8, 101u8, 102u8, 103u8, 104u8, 105u8, 106u8,
    107u8, 108u8, 109u8, 110u8, 111u8, 112u8, 113u8, 114u8, 115u8, 116u8, 117u8, 118u8, 119u8,
    120u8, 121u8, 122u8, 123u8, 124u8, 125u8, 126u8, 127u8, 128u8, 129u8, 130u8, 131u8, 132u8,
    133u8, 134u8, 135u8, 136u8, 137u8, 138u8, 139u8, 140u8, 141u8, 142u8, 143u8, 144u8, 145u8,
    146u8, 147u8, 148u8, 149u8, 150u8, 151u8, 152u8, 153u8, 154u8, 155u8, 156u8, 157u8, 158u8,
    159u8, 160u8, 161u8, 162u8, 163u8, 164u8, 165u8, 166u8, 167u8, 168u8, 169u8, 170u8, 171u8,
    172u8, 173u8, 174u8, 175u8, 176u8, 177u8, 178u8, 179u8, 180u8, 181u8, 182u8, 183u8, 184u8,
    185u8, 186u8, 187u8, 188u8, 189u8, 190u8, 191u8, 192u8, 193u8, 194u8, 195u8, 196u8, 197u8,
    198u8, 199u8, 200u8, 201u8, 202u8, 203u8, 204u8, 205u8, 206u8, 207u8, 208u8, 209u8, 210u8,
    211u8, 212u8, 213u8, 214u8, 215u8, 216u8, 217u8, 218u8, 219u8, 220u8, 221u8, 222u8, 223u8,
    224u8, 225u8, 226u8, 227u8, 228u8, 229u8, 230u8, 231u8, 232u8, 233u8, 234u8, 235u8, 236u8,
    237u8, 238u8, 239u8, 240u8, 241u8, 242u8, 243u8, 244u8, 245u8, 246u8, 247u8, 248u8, 249u8,
    250u8, 251u8, 252u8, 253u8, 254u8, 255u8,
]);

#[unsafe(no_mangle)]
static _Py_ctype_toupper: __SlateAlign16<[u8; 256]> = __SlateAlign16([
    0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
    17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8, 30u8, 31u8, 32u8,
    33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8, 44u8, 45u8, 46u8, 47u8, 48u8,
    49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8, 58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8,
    65u8, 66u8, 67u8, 68u8, 69u8, 70u8, 71u8, 72u8, 73u8, 74u8, 75u8, 76u8, 77u8, 78u8, 79u8, 80u8,
    81u8, 82u8, 83u8, 84u8, 85u8, 86u8, 87u8, 88u8, 89u8, 90u8, 91u8, 92u8, 93u8, 94u8, 95u8, 96u8,
    65u8, 66u8, 67u8, 68u8, 69u8, 70u8, 71u8, 72u8, 73u8, 74u8, 75u8, 76u8, 77u8, 78u8, 79u8, 80u8,
    81u8, 82u8, 83u8, 84u8, 85u8, 86u8, 87u8, 88u8, 89u8, 90u8, 123u8, 124u8, 125u8, 126u8, 127u8,
    128u8, 129u8, 130u8, 131u8, 132u8, 133u8, 134u8, 135u8, 136u8, 137u8, 138u8, 139u8, 140u8,
    141u8, 142u8, 143u8, 144u8, 145u8, 146u8, 147u8, 148u8, 149u8, 150u8, 151u8, 152u8, 153u8,
    154u8, 155u8, 156u8, 157u8, 158u8, 159u8, 160u8, 161u8, 162u8, 163u8, 164u8, 165u8, 166u8,
    167u8, 168u8, 169u8, 170u8, 171u8, 172u8, 173u8, 174u8, 175u8, 176u8, 177u8, 178u8, 179u8,
    180u8, 181u8, 182u8, 183u8, 184u8, 185u8, 186u8, 187u8, 188u8, 189u8, 190u8, 191u8, 192u8,
    193u8, 194u8, 195u8, 196u8, 197u8, 198u8, 199u8, 200u8, 201u8, 202u8, 203u8, 204u8, 205u8,
    206u8, 207u8, 208u8, 209u8, 210u8, 211u8, 212u8, 213u8, 214u8, 215u8, 216u8, 217u8, 218u8,
    219u8, 220u8, 221u8, 222u8, 223u8, 224u8, 225u8, 226u8, 227u8, 228u8, 229u8, 230u8, 231u8,
    232u8, 233u8, 234u8, 235u8, 236u8, 237u8, 238u8, 239u8, 240u8, 241u8, 242u8, 243u8, 244u8,
    245u8, 246u8, 247u8, 248u8, 249u8, 250u8, 251u8, 252u8, 253u8, 254u8, 255u8,
]);
