# slate-python

Rust translation of the CPython interpreter by
[slate](https://github.com/Takashiidobe/slate), a C to Rust translator.

CPython 3.14.8
([`8e6e75d9102e`](https://github.com/python/cpython/commit/8e6e75d9102e39bed2a2b279203a396741180f12)),
x86_64 Linux, configured with `CC=clang`:

- The 190 C translation units of the `python` binary translate into one crate,
  `python/`.
- With the native build's standard library and extension modules,
  `python -m test` passes the same 457 test files (48,355 tests) as the native
  interpreter, in 95s against 90s native. `test_gdb` and perf's DWARF unwinding
  test are excluded: they inspect the C build's debug info.

## Layout

| Path                        | What it is                                                         |
| --------------------------- | ------------------------------------------------------------------ |
| `python/`                   | The translated crate: one module per C file                        |
| `python/.cargo/config.toml` | Link flags, target dir, and `run.sh` as the runner for `cargo run` |
| `run.sh`                    | Builds the crate if needed, points it at `cpython/`, and runs it   |
| `cpython/`                  | CPython checkout and native build (you create this, see below)     |

## Requirements

- clang 23 (the Rust was translated against the struct layouts and `pyconfig.h`
  that this clang and `configure` produced)
- [rustup](https://rustup.rs): `rust-toolchain.toml` pins the nightly
- CPython's build dependencies (`make`, and the dev packages for the optional
  modules you want: `libffi`, `openssl`, `zlib`, `bzip2`, `xz`, `sqlite`,
  `readline`, `ncurses`, `gdbm`, `uuid`)

## Get the standard library

The translation replaces the interpreter, not the standard library. Python code
lives in `cpython/Lib`, and C extension modules (`_ssl`, `_sqlite3`, `math`,
...) come from a native build of the same commit, which must use the same
`configure` arguments so the extension ABI matches:

```sh
git clone https://github.com/Takashiidobe/slate-python
cd slate-python
git init cpython
git -C cpython fetch --depth 1 https://github.com/python/cpython 8e6e75d9102e39bed2a2b279203a396741180f12
git -C cpython checkout FETCH_HEAD
mkdir cpython/build-clang
cd cpython/build-clang
../configure CC=clang
make -j"$(nproc)"
cd ../..
```

`run.sh` looks for CPython in `./cpython`; set `SLATE_CPYTHON` to use another
checkout. The build also produces `Python/asm_trampoline.o`, which the crate
links in.

## Run

```sh
./run.sh
```

This builds the translated interpreter on the first run and drops into its REPL.
Arguments go to the interpreter:

```sh
./run.sh -c 'import sys; print(sys.version)'
./run.sh -m test -j0 -x test_gdb --ignore '*TestPerfProfilerWithDwarf*'
```

`cargo run --release` from `python/` does the same.

## Regenerate

From a slate checkout, with CPython in `~/c-corpus/cpython`:

```sh
python3 tools/corpus/e2e.py cpython
```

This translates, builds, runs `python -m test` against both interpreters, and
writes the crate and its logs here.
