#!/usr/bin/env bash

set -e

for DIR in *; do
  if [ -d "${DIR}" ]; then
    ( cd "${DIR}"
      if [ -r "Cargo.toml" ]; then
        echo "#### Testing \"${DIR}\" ..."
        cargo test --verbose
        echo
      fi
    )
  fi
done
