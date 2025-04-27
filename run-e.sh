#!/bin/bash

RUST_BACKTRACE=1 ./target/debug/tic $@ ; echo "Exited with code $?"
