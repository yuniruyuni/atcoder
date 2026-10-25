# atcoder for yuniruyuni

## what is it?

yuniruyuni's atcoder write-ups.
this repository have some utilities to go at atcoder problems.

## security and compiler baseline (2026-10-25)

The historical Rust 1.42 / 2020 contest dependency set has been migrated to
Rust 1.96.1 for local maintenance. This is a tested toolchain baseline, not a
claim that it matches any current contest judge or the minimum compiler each
library supports. These manifests and the new Cargo.lock no longer support
Rust 1.42. Verify the judge's allowed Rust/crate versions before submitting.
No contest submission is part of this migration.

All 90 solution manifests retain their source code and sample tests. The
matched dependencies move to nalgebra 0.27.1, ordered-float 1.1.1, regex 1.12.4
and smallvec 1.15.1. Updating regex removes thread_local. The unused im-rc
dependency is removed: its sized-chunks dependency has an additional 2026
panic-safety advisory with **no fixed version**, so raising sized-chunks to
the old 0.6.3 fix would not suffice. No solution imports these affected APIs.
The full local audit also found RUSTSEC-2026-0097 in rand 0.7; rand moves to
0.8.6 with compatible rand_core/chacha/hc/pcg/distr and getrandom versions.
Random APIs/sequences and nalgebra types can differ from the old contest set;
the current solutions do not call them.

`gen` applies `scripts/secure-template.sh` to newly generated manifests after
loading the historical template, so the old vulnerable pins are not restored.
The migration does not execute `gen`, `submit`, judge login or network downloads
of contest tasks. The template transformation is tested using local files.

RustSec database commit `21d909e4c8bd4e14c7aea58bf980c881a0053ab1` was downloaded
and matched locally; no private dependency inventory was uploaded. The final
lockfile has no vulnerability or unsoundness matches. The remaining paste
warning concerns maintenance. Nalgebra 0.27.1 also emits a future-compiler
compatibility warning; the chosen Rust 1.96.1 build succeeds.

Validation on COCOA (x86_64-pc-windows-msvc, two build jobs):
`cargo build --workspace --all-targets --locked`,
`cargo clippy --workspace --all-targets --locked`,
`cargo test --workspace --locked`, `cargo doc --workspace --no-deps --locked`.
All projects are binaries, so there are no library doctests. Historical Clippy
style warnings are reported rather than suppressed. No live deployment or
release publication is configured.

## setup

```
cargo install cargo-generate
pip3 install online-judge-tools
oj login https://atcoder.jp/login
```

## usage

1. run following command

```
./gen <context-id> <task>
```

ex: `./gen abc123 a`

2. write codes and test.

`gen` command opens your `nvim` with codes and tests.

3. submit with ./submit.sh

```
./submit.sh <context-id> <task>
```

ex: `./submit abc123 a`

## test

you can run `cargo test -p <sub-project dir>` in root dir.
For fast build, it share the crates cache for all sub-project dirs.

ex: `cargo test -p abc152-a`

also you can run all sub-project's testing with `cargo test`.
it aims to regression testing after drastic (maybe affect to all sub-project) operation.
