#!/bin/bash
STYLO_BASE=../../../
DYLD_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/ $STYLO_BASE/rust-bindgen/target/debug/bindgen $STYLO_BASE/gecko/obj-x86_64-apple-darwin`uname -r`/dist/include/mozilla/servo/ExternalAPI.h -o bindings.rs

