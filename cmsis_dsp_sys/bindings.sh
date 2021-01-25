#!/bin/bash

# Run this to generate bindings to CMSIS-DSP
# This is only needed when upgrading CMSIS-DSP.

bindgen c/cmsis_dsp_combined.h \
    --use-core --ctypes-prefix crate::ctypes --default-enum-style moduleconsts \
    --whitelist-function "^arm.*" \
    --whitelist-var "^arm.*" \
    --blacklist-type "^__u?int\\d+_t" \
    --output src/bindings.rs  \
    -- -IARM.CMSIS.5.6.0/CMSIS/DSP/Include -IARM.CMSIS.5.6.0/CMSIS/Include
