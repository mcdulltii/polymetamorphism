#! /usr/bin/python3

import time
import string
import itertools
from pwn import *

BINARY = './shellcode'
context.binary = ELF(BINARY)

key = 'TISC{th1s_1s_n0t_th3_ac7u4l_fl4g_lM40}'

"""
void __fastcall __noreturn dump(__int64 a1)
{
  /* snip */

  qmemcpy((void *)(v1 - 112), "Time to get Morbed, ", 20);
  qmemcpy((void *)(v1 - 160), "TISC{th1s_1s_n0t_th3_ac7u4l_fl4g_lM40}", 38);
  *(_DWORD *)(v1 - 16) = 38;
  *(_DWORD *)(v1 - 20) = 45;
  *(_QWORD *)(v1 - 208) = 0x7E56A68CA5787463LL;
  *(_QWORD *)(v1 - 200) = 0x422E4090F47BFFD3LL;
  *(_QWORD *)(v1 - 192) = 0xB1F5265BD497A25LL;
  *(_QWORD *)(v1 - 184) = 0xE12AA70A6C3D420LL;
  *(_QWORD *)(v1 - 176) = 0x1905C7AB726BB76ALL;
  *(_QWORD *)(v1 - 168) = 0x108A4CA19BAD9325LL;
  *(_DWORD *)(v1 - 24) = 48;

  /* snip */

  sys_read(2u, (char *)(v1 - 272), 50uLL);

  /* snip */

  *(v1 - 288) = (*(char *)(v1 + *(v1 - 20) - 2 - 272) << 8) | *(char *)(v1 - 272 + 15);
  *(v1 - 284) = (*(char *)(v1 + *(v1 - 20) - 4 - 272) << 8) | *(char *)(v1 - 272 + 13);
  *(v1 - 280) = (*(char *)(v1 + *(v1 - 20) - 5 - 272) << 8) | *(char *)(v1 + *(v1 - 20) - 2 - 272);
  *(v1 - 276) = (*(char *)(v1 + *(v1 - 20) - 6 - 272) << 8) | *(char *)(v1 + *(v1 - 20) - 1 - 272);

  /* snip */
}

NOTE:
  input is read to (char *)(v1 - 272).
  input is checked that it starts with (void *)(v1 - 160).

  Since (v1 - 20) is 45,
    *(v1 - 288) = (*(char *)(input + 45 - 2) << 8) | *(char *)(input + 15);
    *(v1 - 284) = (*(char *)(input + 45 - 4) << 8) | *(char *)(input + 13);
    *(v1 - 280) = (*(char *)(input + 45 - 5) << 8) | *(char *)(input + 45 - 2);
    *(v1 - 276) = (*(char *)(input + 45 - 6) << 8) | *(char *)(input + 45 - 1);
  And *(v1 - 288) is referenced later in a function call.

  Unique input chars used are:
  input[13] // Known
  input[15] // Known
  input[39]
  input[40]
  input[41]
  input[43]
  input[44]

  Thus 5 characters to bruteforce.
"""

def main():
    printable = string.printable
    inpt = list(key + (45 - len(key)) * " ")

    pty = process.PTY
    for each in itertools.permutations(printable, 5):
        c = process(stdin=pty, stdout=pty)

        log.info(b'Prompt: ' + c.recv())

        inpt[39] = each[0]
        inpt[40] = each[1]
        inpt[41] = each[2]
        inpt[43] = each[3]
        inpt[44] = each[4]

        c.sendline(''.join(inpt))

        output = c.recv()

        c.close()

        log.info(f"Tried {each}")
        if b'TISC' in output:
            log.info(f"{output=}")
            break

if __name__ == '__main__':
    start = time.time()
    main()
    end = time.time()
    log.info(f"Finished in {end - start}s")

