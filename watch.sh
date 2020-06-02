#!/bin/sh

PRIVATES=true

if [ "$PRIVATES" = true ] ; then
    DOC_CMD="cargo doc -q --document-private-items"
else
    DOC_CMD="cargo doc -q"
fi

# Watches for changes in ./src/
# Executes cargo run & doc
cargo watch -qc -w src -x "run -q" -s "$DOC_CMD"
