# Polymorphism and Metamorphism in Rust
[![](https://img.shields.io/badge/Category-Binary%20Exploitation-E5A505?style=flat-square)]() [![](https://img.shields.io/badge/Language-Rust%20%2f%20C%20%2f%20ASM-E5A505?style=flat-square)]() [![](https://img.shields.io/badge/Version-0.3.0-E5A505?style=flat-square&color=green)]()

## Details

- Rust polymorphic loader
- Retrieves ELF sections by blake2 hash
- Runtime ChaCha20 stream cipher decryption on shellcode buffer
- Shellcode reads user input
- Execution continues if user input matches hardcoded string
- Some characters of user input used for XTEA block cipher decryption on flag buffer
- Flag decrypted buffer printed

## Project Layout

```html
.
|-- ABOUT.md
|-- asm_src
|   |-- embedded.c
|   |-- encdec_algo
|   |   |-- xtea
|   |   `-- xtea.c
|   |-- Makefile
|   |-- res
|   |   |-- asmwrapper -> ../../utils/ASMWrapper/asmwrapper
|   |   |-- decipher.bin
|   |   |-- embedded.nasm
|   |   `-- obfuscate.py
|   `-- shellcode.c
|-- Cargo.lock
|-- Cargo.toml
|-- dist
|   `-- morbius
|-- flag.txt
|-- LICENSE
|-- Makefile
|-- README.md
|-- src
|   |-- main.rs
|   |-- metamorphic.rs
|   |-- polymorphic.rs
|   `-- quotes.rs
|-- tree.txt
|-- utils
|   `-- ASMWrapper
|       `-- asmwrapper
|           |-- chunk.py
|           |-- graph.py
|           |-- utils.py
|           `-- wrapper.py
`-- xpl
    |-- dump
    |-- Makefile
    |-- morbius
    |-- morbius_extract
    |-- morbius_rewrite
    |-- res
    |   |-- nop_keystream.png
    |   |-- objdump.png
    |   |-- sections_bottom.png
    |   |-- sections_top.png
    |   `-- xpl.png
    |-- shellcode.c
    |-- shellcode.i64
    `-- xpl.py

17 directories, 60 files
```

## LICENSE

This project is licensed under the [GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/).

## References

- https://github.com/mmore21/dolos
- https://github.com/PoCInnovation/Whitecomet-Research
