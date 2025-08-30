# php2ir

**PHP 8.x → LLVM-IR → native ELF/EXE/Mach-O**
*First true AOT pipeline that skips C as an intermediate step.*

---

## Why

Traditional “compile PHP” efforts transpile to C first. **php2ir** compiles **directly to LLVM IR**, producing fast native binaries for Linux, macOS, and Windows without a C intermediary. This unlocks LTO, PGO, and modern LLVM optimizations while keeping the pipeline simple and deterministic.

---

## Features

* **Direct AOT**: PHP 8.x source → LLVM IR → native binary (ELF/EXE/Mach-O)
* **Zero C shim**: no C transpile stage, no platform-specific codegen glue
* **SSA at the source level**: aggressive constant folding, DCE, inlining
* **Mixed runtime modes**: static (no VM) or minimal runtime shims
* **Interop**: FFI for native calls (dlopen/dlsym on \*nix, Win32 on Windows)
* **Portable toolchain**: clang/llc/lld or `llvm`-monolithic toolchains
* **Deterministic builds**: reproducible, hermetic Docker images
* **Unit + IR tests**: golden IR snapshots to catch regressions

---

## Status

> Early alpha. The compiler supports a pragmatic PHP subset + a growing standard library. See **Supported PHP** below.

---

## Quick Start

```bash
# 1) Prereqs (LLVM 17+ recommended)
# macOS (brew):
brew install llvm@17
# Linux (apt):
sudo apt-get install -y llvm lld clang

# 2) Clone
git clone https://github.com/makalin/php2ir.git
cd php2ir

# 3) Build compiler
make build         # or: cargo build --release (if using Rust toolchain)

# 4) Compile PHP → native
./target/release/php2ir examples/hello.php -o hello
./hello
```

Docker:

```bash
docker build -t php2ir .
docker run --rm -v $PWD:/w -w /w php2ir ./php2ir examples/hello.php -o hello
```

---

## Hello, World

```php
<?php
function greet(string $name): string {
  return "Hello, $name!";
}

echo greet("World"), PHP_EOL;
```

Compile:

```bash
php2ir hello.php -o hello
./hello
```

---

## Supported PHP (alpha)

* **Scalars**: int, float, bool, string
* **Arrays**: packed arrays, associative arrays (lowered to hashmap vectors)
* **Control flow**: if/else, while/for/foreach, switch, match
* **Functions**: user functions, default params, return types, recursion
* **Closures**: by-value capture (by-ref WIP)
* **OOP**: classes, properties, methods, `new`, visibility (runtime-enforced), `static`
* **Attributes**: parsed, exposed to IR metadata (custom passes possible)
* **Exceptions**: `try/catch/finally` (zero-cost where available)
* **I/O**: `echo`, basic filesystem APIs via runtime shims
* **FFI**: call native functions (see Interop)

*Not yet*: fibers, generators, dynamic properties (deprecated), traits (partial), enums (parsing ok, codegen WIP), references (&) semantics (partial), magic methods (partial), JIT (not applicable), full `ext/*` set.

---

## Architecture

```
PHP source
   └─▶ Parser (php-src AST or nikic/php-parser)
         └─▶ Normalizer (types, attributes, constant folding)
               └─▶ High-level SSA (phi insertion, dominance, CPC)
                     └─▶ Lowering to LLVM IR (typed ops, GC barriers)
                           └─▶ LLVM passes (O2/LTO/PGO)
                                 └─▶ lld → native binary
```

Key components:

* **Front-end**: AST import + semantic analysis (type hints + flow inference)
* **IR Builder**: high-level SSA → LLVM IR (call graph, inliner, DCE)
* **Runtime**: small `libphp2ir` for arrays/strings/hashmaps/exceptions/IO
* **GC**: configurable (ARC-like refcount default; optional Boehm/MC WIP)
* **Linker**: `lld` by default; `link.exe` supported on Windows

---

## Install

### From source

```bash
make build
# binaries in ./target/release
```

### Toolchain requirements

* LLVM 16+ (17+ recommended), `clang`, `llc`, `lld`
* CMake (for runtime lib), Ninja (optional)
* PHP 8.x headers if building with php-src AST mode (optional)
* Rust 1.78+ or C++20 (depending on selected backend in `Makefile.config`)

---

## CLI

```text
php2ir <input.php> [-o <out>] [--emit-llvm] [--emit-llvm-only]
                   [--lto <thin|full>] [--pgo-gen|--pgo-use=<profdata>]
                   [--opt <O0|O1|O2|O3|Oz>] [--target <triple>]
                   [--stdlib <path>] [--no-rt] [--sanitize <address|ubsan>]
```

Examples:

```bash
# Emit IR only:
php2ir foo.php --emit-llvm -o foo.ll

# Native with ThinLTO at O3:
php2ir app.php --lto thin --opt O3 -o app

# Cross-compile (static):
php2ir svc.php --target x86_64-unknown-linux-gnu --opt O2 -o svc
```

---

## Interop (FFI)

```php
<?php
#[ffi("libm.so.6", "double cos(double)")]
function cos_native(float $x): float {}

echo cos_native(0.0), PHP_EOL;
```

Compile and link will auto-discover `libm` via `-lm` (configurable).

---

## Runtime Library

* **Strings**: UTF-8, small-string optimization
* **Arrays/Hashmaps**: packed + dict with copy-on-write fast paths
* **Exceptions**: zero-cost tables (Itanium on \*nix, SEH on Windows)
* **IO**: `fopen/fread/fwrite`, argv/env, timers
* **Platform**: POSIX & Win32 shims, high-res time, random

Toggle features in `runtime/config.h` (GC, SSO threshold, hash policy).

---

## Optimization Passes

* Type specialization and devirtualization (monomorphic hot paths)
* Escape analysis + stack promotion
* Interprocedural constant propagation and inlining
* Bounds-check hoisting for arrays/strings
* Optional `-fexperimental-new-pass-manager` pipelines
* LTO (Thin/Full), PGO (`.profdata`)

---

## Building From Source

```bash
# Configure
cp Makefile.config.example Makefile.config
# edit LLVM_PREFIX, BUILD_BACKEND=(rust|cpp), DEFAULT_OPTS, etc.

# Build compiler + runtime
make clean && make -j

# Test
make test
```

---

## Testing

* **Unit tests**: `cargo test` or `ctest` (backend-dependent)
* **IR golden tests**: compare `*.ll` against snapshots
* **Runtime tests**: black-box run + stdout/exit-code assertions
* **Bench**: micro-bench harness (see `benches/`)

```bash
make test
make irtest
make bench
```

---

## Roadmap

* Full references semantics (`&$x`) and alias analysis
* Generators / `yield`, fibers
* Traits/enums full codegen
* OPcache profile import → PGO seed
* Advanced GC (immix/RC hybrid), arena allocators
* Windows MSVC + ARM64 macOS native releases
* Composer package AOT (compile entire dependency graph)

---

## Limitations

* Not a drop-in replacement for the PHP VM yet
* Dynamic features (eval, variable variables, include hooks) are restricted or require AOT-friendly patterns
* Extensions: curated subset via shims; full `ext/*` parity is long-term

---

## Examples

* `examples/hello.php` – basics
* `examples/http_micro.php` – tiny HTTP server
* `examples/ffi_math.php` – native interop
* `examples/oop.php` – classes & methods

Run all:

```bash
make examples
```

---

## FAQs

**Why not transpile to C first?**
Skipping C removes an entire compilation layer, avoids C UB pitfalls, shortens build times, and lets LLVM see richer semantics earlier.

**Will my existing PHP app work?**
CLI apps with minimal dynamic features are great candidates today. Web apps can work via AOT of entrypoints + a tiny SAPI shim (WIP).

**Performance vs PHP-FPM?**
CPU-bound workloads typically see substantial gains with O2/LTO. IO-bound workloads benefit less; measure with our bench harness.

---

## Contributing

1. Fork & branch: `feat/<name>`
2. Run tests: `make test`
3. Add IR snapshot tests when changing codegen
4. Open PR with a brief design note and benchmarks

Issues and discussions welcome!

---

## License

Copyright 2025 Mehmet T. AKALIN

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---

## Citation

If this project helps your research or product, consider citing:

```
@software{php2ir,
  title        = {php2ir: Direct PHP → LLVM IR AOT compiler},
  author       = {Mehmet T. AKALIN},
  year         = {2025},
  url          = {https://github.com/makalin/php2ir}
}
```

---

## Acknowledgments

* LLVM community and docs
* PHP internals & `nikic/php-parser` ecosystem (when used as front-end)
