# Morbed, Morphed, Morbed

Rust polymorphic shellcode loader with metamorphic junk byte insertion

# Description (public)

![gif](https://preview.redd.it/jt5qrg8r3c391.gif?format=mp4&s=efa415cfc9a6336e9d07f2a58cff073ce1a669b5)

```shell
It's morbin time!
```

# Setup Guide

1. `dist/morbius` contains the initial unencrypted ELF build. Run `make release` at least once to self-encrypt the ELF build.

2. Ensure the byte contents of `objdump -s -j .lbss ./dist/morbius` is `01`.

2. Provide `morbius` in `dist/` to players

# Solution

1. ELF Binary deletes itself, then decrypts and re-encrypts itself on runtime, and write into its initial path.
We can trace where the binary re-encrypts itself, and make the binary decrypt and write itself by NOP-ing the encrypt function call.

`objcopy morbius_extract /dev/null --dump-section .hash=dump`

# Flag

`TISC{P0lyM0rph15m_r3m1nd5_m3_0f_M0rb1us_7359430}`

