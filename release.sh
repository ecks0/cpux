#!/bin/bash

set -e

[ $# -ne 1 ] && {
  echo "Usage: $(basename $0) <VERSION>"
  exit 1
}

rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
tar -C target/x86_64-unknown-linux-musl/release -c cpux|gzip -9 >cpux-$1-x86_64-linux-musl.tar.gz

