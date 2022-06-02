# Polymorphism and Metamorphism in Rust
[![](https://img.shields.io/badge/Category-Binary%20Exploitation-E5A505?style=flat-square)]() [![](https://img.shields.io/badge/Language-Rust%20%2f%20C%20%2f%20ASM-E5A505?style=flat-square)]() [![](https://img.shields.io/badge/Version-0.3.0-E5A505?style=flat-square&color=green)]()

## Details

- Rust polymorphic loader
- Runtime ChaCha20 stream cipher decryption on shellcode buffer
- Shellcode reads user input
- Execution continues if user input matches hardcoded string
- Some characters of user input used for XTEA block cipher decryption on flag buffer
- Flag decrypted buffer printed

## LICENSE

This project is licensed under the [GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/).

## References

- https://github.com/mmore21/dolos
- https://github.com/PoCInnovation/Whitecomet-Research
