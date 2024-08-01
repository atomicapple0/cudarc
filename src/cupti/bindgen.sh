#!/bin/bash
set -exu

bindgen \
  --allowlist-var="^CUDA_VERSION.*" \
  --allowlist-type="^cupti.*" \
  --allowlist-var="^cupti.*" \
  --allowlist-function="^cupti.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --use-core \
  wrapper.h -- -I/usr/local/cuda/include \
  > sys.rs
