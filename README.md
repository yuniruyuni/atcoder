# atcoder for yuniruyuni

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

## limitation

- Currently, it supports only `abc` contest
