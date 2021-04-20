#! /bin/bash
set -o errexit -o pipefail -o nounset

if command -v pretty-exec >/dev/null; then
  run() {
    echo
    pretty-exec -- "$@"
  }
else
  run() {
    echo
    echo "$@"
    command "$@"
  }
fi

test() {
  run cargo clippy "$@" -- -D warnings
  run cargo test "$@"
}

test
test --no-default-features
test --all-features
