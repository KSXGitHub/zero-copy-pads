#! /bin/bash
set -o errexit -o pipefail -o nounset

if command -v pretty-exec >/dev/null; then
  run() {
    pretty-exec -- "$@"
    echo
  }
else
  run() {
    echo "$@"
    command "$@"
    echo
  }
fi

test() {
  run cargo clippy "$@" -- -D warnings
  run cargo test "$@"
}

test
test --no-default-features
test --all-features
run cargo fmt -- --check
