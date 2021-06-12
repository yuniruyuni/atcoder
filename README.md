# atcoder for yuniruyuni

## what is it?

yuniruyuni's atcoder write-ups.
this repository have some utilities to go at atcoder problems.

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
