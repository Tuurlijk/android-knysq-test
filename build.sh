#!/usr/bin/env bash

./build.cargo.ndk.sh

./gradlew build
./gradlew installDebug