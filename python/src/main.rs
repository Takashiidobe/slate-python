#![feature(linkage)]
#![feature(link_llvm_intrinsics)]
#![feature(thread_local)]
#![feature(used_with_arg)]
#![no_main]

//! Minimal main program -- everything is loaded from the library
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
#[path = "Python_ast.rs"]
mod __slate_unit_Python_ast;
#[path = "Python_tokenize.rs"]
mod __slate_unit_Python_tokenize;
#[path = "_abc.rs"]
mod __slate_unit__abc;
#[path = "_codecsmodule.rs"]
mod __slate_unit__codecsmodule;
#[path = "_collectionsmodule.rs"]
mod __slate_unit__collectionsmodule;
#[path = "_contextvars.rs"]
mod __slate_unit__contextvars;
#[path = "_datetimemodule.rs"]
mod __slate_unit__datetimemodule;
#[path = "_functoolsmodule.rs"]
mod __slate_unit__functoolsmodule;
#[path = "_iomodule.rs"]
mod __slate_unit__iomodule;
#[path = "_localemodule.rs"]
mod __slate_unit__localemodule;
#[path = "_opcode.rs"]
mod __slate_unit__opcode;
#[path = "_operator.rs"]
mod __slate_unit__operator;
#[path = "_stat.rs"]
mod __slate_unit__stat;
#[path = "_suggestions.rs"]
mod __slate_unit__suggestions;
#[path = "_sysconfig.rs"]
mod __slate_unit__sysconfig;
#[path = "_threadmodule.rs"]
mod __slate_unit__threadmodule;
#[path = "_tracemalloc.rs"]
mod __slate_unit__tracemalloc;
#[path = "_typesmodule.rs"]
mod __slate_unit__typesmodule;
#[path = "_typingmodule.rs"]
mod __slate_unit__typingmodule;
#[path = "_warnings.rs"]
mod __slate_unit__warnings;
#[path = "_weakref.rs"]
mod __slate_unit__weakref;
#[path = "abstract.rs"]
mod __slate_unit_abstract;
#[path = "action_helpers.rs"]
mod __slate_unit_action_helpers;
#[path = "asdl.rs"]
mod __slate_unit_asdl;
#[path = "assemble.rs"]
mod __slate_unit_assemble;
#[path = "ast.rs"]
mod __slate_unit_ast;
#[path = "ast_preprocess.rs"]
mod __slate_unit_ast_preprocess;
#[path = "ast_unparse.rs"]
mod __slate_unit_ast_unparse;
#[path = "atexitmodule.rs"]
mod __slate_unit_atexitmodule;
#[path = "bltinmodule.rs"]
mod __slate_unit_bltinmodule;
#[path = "boolobject.rs"]
mod __slate_unit_boolobject;
#[path = "bootstrap_hash.rs"]
mod __slate_unit_bootstrap_hash;
#[path = "brc.rs"]
mod __slate_unit_brc;
#[path = "buffer.rs"]
mod __slate_unit_buffer;
#[path = "bufferedio.rs"]
mod __slate_unit_bufferedio;
#[path = "bytearrayobject.rs"]
mod __slate_unit_bytearrayobject;
#[path = "bytes_methods.rs"]
mod __slate_unit_bytes_methods;
#[path = "bytesio.rs"]
mod __slate_unit_bytesio;
#[path = "bytesobject.rs"]
mod __slate_unit_bytesobject;
#[path = "call.rs"]
mod __slate_unit_call;
#[path = "capsule.rs"]
mod __slate_unit_capsule;
#[path = "cellobject.rs"]
mod __slate_unit_cellobject;
#[path = "ceval.rs"]
mod __slate_unit_ceval;
#[path = "ceval_gil.rs"]
mod __slate_unit_ceval_gil;
#[path = "classobject.rs"]
mod __slate_unit_classobject;
#[path = "codecs.rs"]
mod __slate_unit_codecs;
#[path = "codegen.rs"]
mod __slate_unit_codegen;
#[path = "codeobject.rs"]
mod __slate_unit_codeobject;
#[path = "compile.rs"]
mod __slate_unit_compile;
#[path = "complexobject.rs"]
mod __slate_unit_complexobject;
#[path = "config.rs"]
mod __slate_unit_config;
#[path = "context.rs"]
mod __slate_unit_context;
#[path = "critical_section.rs"]
mod __slate_unit_critical_section;
#[path = "crossinterp.rs"]
mod __slate_unit_crossinterp;
#[path = "descrobject.rs"]
mod __slate_unit_descrobject;
#[path = "dictobject.rs"]
mod __slate_unit_dictobject;
#[path = "dtoa.rs"]
mod __slate_unit_dtoa;
#[path = "dynamic_annotations.rs"]
mod __slate_unit_dynamic_annotations;
#[path = "dynload_shlib.rs"]
mod __slate_unit_dynload_shlib;
#[path = "enumobject.rs"]
mod __slate_unit_enumobject;
#[path = "errnomodule.rs"]
mod __slate_unit_errnomodule;
#[path = "errors.rs"]
mod __slate_unit_errors;
#[path = "exceptions.rs"]
mod __slate_unit_exceptions;
#[path = "faulthandler.rs"]
mod __slate_unit_faulthandler;
#[path = "file_tokenizer.rs"]
mod __slate_unit_file_tokenizer;
#[path = "fileio.rs"]
mod __slate_unit_fileio;
#[path = "fileobject.rs"]
mod __slate_unit_fileobject;
#[path = "fileutils.rs"]
mod __slate_unit_fileutils;
#[path = "floatobject.rs"]
mod __slate_unit_floatobject;
#[path = "flowgraph.rs"]
mod __slate_unit_flowgraph;
#[path = "formatter_unicode.rs"]
mod __slate_unit_formatter_unicode;
#[path = "frame.rs"]
mod __slate_unit_frame;
#[path = "frameobject.rs"]
mod __slate_unit_frameobject;
#[path = "frozen.rs"]
mod __slate_unit_frozen;
#[path = "frozenmain.rs"]
mod __slate_unit_frozenmain;
#[path = "funcobject.rs"]
mod __slate_unit_funcobject;
#[path = "future.rs"]
mod __slate_unit_future;
#[path = "gc.rs"]
mod __slate_unit_gc;
#[path = "gc_free_threading.rs"]
mod __slate_unit_gc_free_threading;
#[path = "gc_gil.rs"]
mod __slate_unit_gc_gil;
#[path = "gcmodule.rs"]
mod __slate_unit_gcmodule;
#[path = "genericaliasobject.rs"]
mod __slate_unit_genericaliasobject;
#[path = "genobject.rs"]
mod __slate_unit_genobject;
#[path = "getargs.rs"]
mod __slate_unit_getargs;
#[path = "getbuildinfo.rs"]
mod __slate_unit_getbuildinfo;
#[path = "getcompiler.rs"]
mod __slate_unit_getcompiler;
#[path = "getcopyright.rs"]
mod __slate_unit_getcopyright;
#[path = "getopt.rs"]
mod __slate_unit_getopt;
#[path = "getpath.rs"]
mod __slate_unit_getpath;
#[path = "getplatform.rs"]
mod __slate_unit_getplatform;
#[path = "getversion.rs"]
mod __slate_unit_getversion;
#[path = "hamt.rs"]
mod __slate_unit_hamt;
#[path = "hashtable.rs"]
mod __slate_unit_hashtable;
#[path = "helpers.rs"]
mod __slate_unit_helpers;
#[path = "import.rs"]
mod __slate_unit_import;
#[path = "importdl.rs"]
mod __slate_unit_importdl;
#[path = "index_pool.rs"]
mod __slate_unit_index_pool;
#[path = "initconfig.rs"]
mod __slate_unit_initconfig;
#[path = "instruction_sequence.rs"]
mod __slate_unit_instruction_sequence;
#[path = "instrumentation.rs"]
mod __slate_unit_instrumentation;
#[path = "interpconfig.rs"]
mod __slate_unit_interpconfig;
#[path = "interpolationobject.rs"]
mod __slate_unit_interpolationobject;
#[path = "intrinsics.rs"]
mod __slate_unit_intrinsics;
#[path = "iobase.rs"]
mod __slate_unit_iobase;
#[path = "iterobject.rs"]
mod __slate_unit_iterobject;
#[path = "itertoolsmodule.rs"]
mod __slate_unit_itertoolsmodule;
#[path = "jit.rs"]
mod __slate_unit_jit;
#[path = "legacy_tracing.rs"]
mod __slate_unit_legacy_tracing;
#[path = "lexer.rs"]
mod __slate_unit_lexer;
#[path = "listobject.rs"]
mod __slate_unit_listobject;
#[path = "lock.rs"]
mod __slate_unit_lock;
#[path = "longobject.rs"]
mod __slate_unit_longobject;
#[path = "__slate_unit_main.rs"]
mod __slate_unit_main;
#[path = "marshal.rs"]
mod __slate_unit_marshal;
#[path = "memoryobject.rs"]
mod __slate_unit_memoryobject;
#[path = "methodobject.rs"]
mod __slate_unit_methodobject;
#[path = "modsupport.rs"]
mod __slate_unit_modsupport;
#[path = "moduleobject.rs"]
mod __slate_unit_moduleobject;
#[path = "myreadline.rs"]
mod __slate_unit_myreadline;
#[path = "mysnprintf.rs"]
mod __slate_unit_mysnprintf;
#[path = "mystrtoul.rs"]
mod __slate_unit_mystrtoul;
#[path = "namespaceobject.rs"]
mod __slate_unit_namespaceobject;
#[path = "object.rs"]
mod __slate_unit_object;
#[path = "object_stack.rs"]
mod __slate_unit_object_stack;
#[path = "obmalloc.rs"]
mod __slate_unit_obmalloc;
#[path = "odictobject.rs"]
mod __slate_unit_odictobject;
#[path = "optimizer.rs"]
mod __slate_unit_optimizer;
#[path = "optimizer_analysis.rs"]
mod __slate_unit_optimizer_analysis;
#[path = "optimizer_symbols.rs"]
mod __slate_unit_optimizer_symbols;
#[path = "parking_lot.rs"]
mod __slate_unit_parking_lot;
#[path = "parser.rs"]
mod __slate_unit_parser;
#[path = "pathconfig.rs"]
mod __slate_unit_pathconfig;
#[path = "peg_api.rs"]
mod __slate_unit_peg_api;
#[path = "pegen.rs"]
mod __slate_unit_pegen;
#[path = "pegen_errors.rs"]
mod __slate_unit_pegen_errors;
#[path = "perf_jit_trampoline.rs"]
mod __slate_unit_perf_jit_trampoline;
#[path = "perf_trampoline.rs"]
mod __slate_unit_perf_trampoline;
#[path = "picklebufobject.rs"]
mod __slate_unit_picklebufobject;
#[path = "posixmodule.rs"]
mod __slate_unit_posixmodule;
#[path = "preconfig.rs"]
mod __slate_unit_preconfig;
#[path = "pwdmodule.rs"]
mod __slate_unit_pwdmodule;
#[path = "pyarena.rs"]
mod __slate_unit_pyarena;
#[path = "pyctype.rs"]
mod __slate_unit_pyctype;
#[path = "pyfpe.rs"]
mod __slate_unit_pyfpe;
#[path = "pyhash.rs"]
mod __slate_unit_pyhash;
#[path = "pylifecycle.rs"]
mod __slate_unit_pylifecycle;
#[path = "pymath.rs"]
mod __slate_unit_pymath;
#[path = "pystate.rs"]
mod __slate_unit_pystate;
#[path = "pystrcmp.rs"]
mod __slate_unit_pystrcmp;
#[path = "pystrhex.rs"]
mod __slate_unit_pystrhex;
#[path = "pystrtod.rs"]
mod __slate_unit_pystrtod;
#[path = "pythonrun.rs"]
mod __slate_unit_pythonrun;
#[path = "pytime.rs"]
mod __slate_unit_pytime;
#[path = "qsbr.rs"]
mod __slate_unit_qsbr;
#[path = "rangeobject.rs"]
mod __slate_unit_rangeobject;
#[path = "readline_tokenizer.rs"]
mod __slate_unit_readline_tokenizer;
#[path = "remote_debugging.rs"]
mod __slate_unit_remote_debugging;
#[path = "setobject.rs"]
mod __slate_unit_setobject;
#[path = "signalmodule.rs"]
mod __slate_unit_signalmodule;
#[path = "sliceobject.rs"]
mod __slate_unit_sliceobject;
#[path = "specialize.rs"]
mod __slate_unit_specialize;
#[path = "sre.rs"]
mod __slate_unit_sre;
#[path = "stackrefs.rs"]
mod __slate_unit_stackrefs;
#[path = "state.rs"]
mod __slate_unit_state;
#[path = "string_parser.rs"]
mod __slate_unit_string_parser;
#[path = "string_tokenizer.rs"]
mod __slate_unit_string_tokenizer;
#[path = "stringio.rs"]
mod __slate_unit_stringio;
#[path = "structmember.rs"]
mod __slate_unit_structmember;
#[path = "structseq.rs"]
mod __slate_unit_structseq;
#[path = "suggestions.rs"]
mod __slate_unit_suggestions;
#[path = "symtable.rs"]
mod __slate_unit_symtable;
#[path = "symtablemodule.rs"]
mod __slate_unit_symtablemodule;
#[path = "sysmodule.rs"]
mod __slate_unit_sysmodule;
#[path = "templateobject.rs"]
mod __slate_unit_templateobject;
#[path = "textio.rs"]
mod __slate_unit_textio;
#[path = "thread.rs"]
mod __slate_unit_thread;
#[path = "timemodule.rs"]
mod __slate_unit_timemodule;
#[path = "token.rs"]
mod __slate_unit_token;
#[path = "traceback.rs"]
mod __slate_unit_traceback;
#[path = "tracemalloc.rs"]
mod __slate_unit_tracemalloc;
#[path = "tupleobject.rs"]
mod __slate_unit_tupleobject;
#[path = "typeobject.rs"]
mod __slate_unit_typeobject;
#[path = "typevarobject.rs"]
mod __slate_unit_typevarobject;
#[path = "unicodectype.rs"]
mod __slate_unit_unicodectype;
#[path = "unicodeobject.rs"]
mod __slate_unit_unicodeobject;
#[path = "unionobject.rs"]
mod __slate_unit_unionobject;
#[path = "uniqueid.rs"]
mod __slate_unit_uniqueid;
#[path = "utf8_tokenizer.rs"]
mod __slate_unit_utf8_tokenizer;
#[path = "weakrefobject.rs"]
mod __slate_unit_weakrefobject;

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
    fn Py_BytesMain(arg0: i32, arg1: *mut *mut i8) -> i32;
}

fn __slate_main(argc: i32, argv: *mut *mut i8) -> i32 {
    unsafe { Py_BytesMain(argc, argv) }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn main(
    argc: i32,
    argv: *mut *mut core::ffi::c_char,
    envp: *mut *mut core::ffi::c_char,
) -> i32 {
    unsafe { __slate_main(argc as _, argv as _) }
}
