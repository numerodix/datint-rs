#!/bin/bash


cargo rustc --release -- -C target-feature=+popcnt


# to examine the generated code:
#
# $ objdump -d target/release/libdatint.rlib | less
#
# ...look for compute_parity_bit_opt...  popcnt instruction emitted:
#
#   b:   66 f3 0f b8 c0          popcnt %ax,%ax
