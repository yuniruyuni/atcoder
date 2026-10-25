#!/bin/sh
# Run after cargo-generate: the historical template still contains 2020 pins.
set -eu
manifest=${1:?usage: secure-template.sh path/to/Cargo.toml}
sed -i \
    -e 's/nalgebra = "=0.20.0"/nalgebra = "=0.27.1"/' \
    -e 's/ordered-float = "=1.0.2"/ordered-float = "=1.1.1"/' \
    -e 's/regex = "=1.3.6"/regex = "=1.12.4"/' \
    -e 's/smallvec = "=1.2.0"/smallvec = "=1.15.1"/' \
    -e 's/version = "=0.7.3"/version = "=0.8.6"/' \
    -e 's/getrandom = "=0.1.14"/getrandom = "=0.2.16"/' \
    -e 's/rand_chacha = "=0.2.2"/rand_chacha = "=0.3.1"/' \
    -e 's/rand_core = "=0.5.1"/rand_core = "=0.6.4"/' \
    -e 's/rand_hc = "=0.2.0"/rand_hc = "=0.3.2"/' \
    -e 's/rand_pcg = "=0.2.1"/rand_pcg = "=0.3.1"/' \
    -e 's/rand_distr = "=0.2.2"/rand_distr = "=0.4.3"/' \
    -e '/^im-rc = "=14.3.0"$/d' \
    "$manifest"
