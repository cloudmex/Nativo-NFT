#!/bin/sh

prefix=/Users/danielmora/Documents/Cloudmex/Nativo-NFT-Mkt/ntv-token/target/debug/build/jemalloc-sys-02f696cf025ab134/out
exec_prefix=/Users/danielmora/Documents/Cloudmex/Nativo-NFT-Mkt/ntv-token/target/debug/build/jemalloc-sys-02f696cf025ab134/out
libdir=${exec_prefix}/lib

DYLD_INSERT_LIBRARIES=${libdir}/libjemalloc.2.dylib
export DYLD_INSERT_LIBRARIES
exec "$@"
