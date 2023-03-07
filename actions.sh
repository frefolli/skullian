#!/bin/bash

LANG=$1
cargo run -- examples/samples/main.$LANG -c examples/samples/$LANG.tsg
