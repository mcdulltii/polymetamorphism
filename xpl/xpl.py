#! /usr/bin/python3

import time
import string
import itertools
from pwn import *
from Crypto.Util.number import long_to_bytes

BINARY = './shellcode'
context.binary = ELF(BINARY)
context.log_level = 'error'

"""
*(_QWORD *)(v1 - 160) = 0x1E475B546C7C667BLL;
*(_QWORD *)(v1 - 152) = 0x5B1F41705C1E705CLL;
*(_QWORD *)(v1 - 144) = 0x184C4E701C475B70LL;
*(_QWORD *)(v1 - 136) = 0x481B434970431B5ALL;
*(_DWORD *)(v1 - 128) = 459424624;
*(_WORD *)(v1 - 124) = 21023;

Input is xor-ed with 47 and compared with this QWORD array.
"""

key = b''.join([long_to_bytes(i)[::-1] for i in [0x1E475B546C7C667B, 0x5B1F41705C1E705C, 0x184C4E701C475B70, 0x481B434970431B5A, 459424624, 21023]])
key = bytearray(i ^ 47 for i in key)
print(f"{key=}")

"""
Input character checks:
if ( (*(char *)(v1 + *(int *)(v1 - 4) - 288) <= 47 || *(char *)(v1 + *(int *)(v1 - 4) - 288) > 57)
    && (*(char *)(v1 + *(int *)(v1 - 4) - 288) <= 64 || *(char *)(v1 + *(int *)(v1 - 4) - 288) > 125) )

Input characters referenced:
*(v1 - 304) = (*(char *)(v1 + *(v1 - 28) - 3 - 288) << 8) | (char)(*(_BYTE *)(v1 - 273) ^ 0x2F);
*(v1 - 300) = 0x64 << 8 | (char)(*(_BYTE *)(v1 - 275) ^ 0x2F);
*(v1 - 296) = 0x4A << 8 | *(char *)(v1 + *(v1 - 28) - 3 - 288);
*(v1 - 292) = (*(char *)(v1 + *(v1 - 28) - 9 - 288) << 8) | 0x32;

Since (v1 - 28) is 49, and input is sys_read into (char *)(v1 - 288),
*(v1 - 304) = (*(char *)(input + 49 - 3) << 8) | (char)(*(_BYTE *)(input + 15) ^ 0x2F);
*(v1 - 300) = 0x64 << 8 | (char)(*(_BYTE *)(input + 13) ^ 0x2F);
*(v1 - 296) = 0x4A << 8 | *(char *)(input + 49 - 3);
*(v1 - 292) = (*(char *)(input + 49 - 9) << 8) | 0x32;

Thus, input chars referenced are:
input[13] // Known
input[15] // Known
input[40] // To bruteforce
input[46] // To bruteforce
"""

def main():
    printable = [i for i in string.printable if (ord(i) >= 0x30 and ord(i) <= 0x39) or (ord(i) >= 0x41 and ord(i) <= 0x7D)]
    inpt = bytearray(key + (49 - len(key)) * b"A")

    pty = process.PTY
    log.progress("Bruteforcing input")
    for each in itertools.permutations(printable, 2):
        c = process(stdin=pty, stdout=pty)

        log.info(b'Prompt: ' + c.recv())

        inpt[40] = ord(each[0])
        inpt[46] = ord(each[1])

        c.sendline(inpt)

        log.info(f"Tried {inpt}")

        try:
            output = c.recv()
            if b'TISC' in output:
                print(f"{output=}")
                break
        except:
            log.warn("Failed recv")

        c.close()

if __name__ == '__main__':
    start = time.time()
    main()
    end = time.time()
    print(f"Finished in {end - start}s")

