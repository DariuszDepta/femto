#!/usr/bin/env bash

set -e

###############################################################################
# Dependencies:
#
# $ sudo dnf install lcov
# $ rustup component add llvm-tools-preview
# $ cargo install grcov
#
###############################################################################

WORKING_DIRECTORY=$(pwd)
CARGO_NAME="cw-multi-test"
CARGO_VERSION="$1"

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=unwind"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=unwind"

# run all tests
cargo +nightly test

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage

# generate coverage info
grcov . --llvm -s . -t lcov --ignore-not-existing \
     --excl-line='\s+\)$|\s+\}$|\s+\);$|\s+\}\)$|\s+\)\?\;$|\s+\};$|\s+\},$|\s+\)\?\)$' \
     --ignore "*tests*" \
     --ignore "*ahash*" \
     --ignore "*anyhow*" \
     --ignore "*base64*" \
     --ignore "*bech32*" \
     --ignore "*bech32*" \
     --ignore="*block*" \
     --ignore="*bnum*" \
     --ignore="*byteorder*" \
     --ignore="*bytes*" \
     --ignore="*const-oid*" \
     --ignore="*cosmwasm-std*" \
     --ignore="*cpufeatures*" \
     --ignore="*crypto-bigint*" \
     --ignore="*curve25519-dalek*" \
     --ignore="*cw-storage-plus*" \
     --ignore="*cw-utils*" \
     --ignore="*der*" \
     --ignore="*digest*" \
     --ignore="*dyn-clone*" \
     --ignore="*ecdsa*" \
     --ignore="*ed25519-zebra*" \
     --ignore="*elliptic-curve*" \
     --ignore="*ff*" \
     --ignore="*generic-array*" \
     --ignore="*getrandom*" \
     --ignore="*hashbrown*" \
     --ignore="*hex*" \
     --ignore="*itertools*" \
     --ignore="*itoa*" \
     --ignore="*k256*" \
     --ignore="*once_cell*" \
     --ignore="*opaque-debug*" \
     --ignore="*prost*" \
     --ignore="*rand_core*" \
     --ignore="*ryu*" \
     --ignore="*schemars*" \
     --ignore="*sec1*" \
     --ignore="*semver*" \
     --ignore="*serde*" \
     --ignore="*sha*" \
     --ignore="*spki*" \
     --ignore="*subtle*" \
     --ignore="*thiserror*" \
     --ignore="*typenum*" \
     --ignore="*zeroize*" \
     -o ./target/lcov/lcov.info

# generate coverage report in HTML format
genhtml -t "$CARGO_NAME $CARGO_VERSION" -q --ignore-errors unmapped,unmapped -o ./target/coverage ./target/lcov/lcov.info

# display final message
echo ""
echo "Open coverage report:"
echo "  HTML file://$WORKING_DIRECTORY/target/coverage/index.html"
echo ""

