#!/usr/bin/env bash

cargo ndk \
    -t x86 \
    -t arm64-v8a \
    -o app/src/main/jniLibs/ build
