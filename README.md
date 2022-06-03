# Morbius

Rust polymorphic shellcode loader with metamorphic junk byte insertion

# Description (public)

```shell
```

# Setup Guide

1. `dist/morbius` contains the initial unencrypted ELF build. Run `make release` at least once to self-encrypt the ELF build.

2. Ensure the byte contents of `objdump -s -j .lbss ./dist/morbius` is `01`.

2. Provide `morbius` in `dist/` to players

# Solution

`objcopy morbius_extract /dev/null --dump-section .hash=dump`

# Flag

`TISC{P0lyM0rph15m_r3m1nd5_m3_0f_M0rb1us_7359430}`

