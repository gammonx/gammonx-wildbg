#!/bin/sh
if [ "$BUILD_CONFIGURATION" = "release" ]; then
  cargo build --release
else
  cargo check
  cargo build
fi