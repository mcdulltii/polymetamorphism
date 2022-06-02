BITS 64
global  _start

section .text

_start:     mov     rdx, len
            mov     rsi, msg
            call    _print
            mov     rax, 10                 ; mprotect
            mov     rdi, 0x200000           ; multiple of pagesize
            mov     rsi, 4096               ; pagesize
            mov     rdx, 0x1
            or      rdx, 0x2
            syscall

            mov     rdx, 50                 ; length
            mov     rsi, input              ; input buffer
            mov     rdi, 2
            mov     rax, 0                  ; sys_read
            syscall

            test    rax, rax                ; check if stdin is read
            jle     EOF
            
            mov     rsi, key                ; key
            mov     rdi, input              ; input
            mov     rcx, klen-19            ; len(key)
            cld
            repe    cmpsb                   ; strncmp(key, input, len(key))
            jrcxz   decode                  ; if rcx == 0

            mov     rdx, wlen               ; length
            mov     rsi, wrong              ; incorrect buffer
            call    _print

EOF:        mov     rax, 60                 ; _exit()
            syscall

decode:     xor     rbx, rbx                ; index
cycle:      mov     al, byte [buf + rbx]    ; flag buffer index
            ;xor     al, 0x39                ; decode with xor 0x39
            mov     cl, byte [input + klen - 1]
            xor     al, cl
            ;sub     al, 0x64                ; decode with sub 0x64
            mov     dl, byte [input + klen - 2]
            sub     al, dl
            mov     [flag + rbx], rax       ; copy to flag string buffer
            inc     rbx
            cmp     rbx, blen               ; end of flag buffer
            jne     cycle

            mov     rdx, clen
            mov     rsi, correct
            call    _print
            mov     rdx, flen               ; length
            mov     rsi, flag               ; flag string buffer
            call    _print
            jmp     EOF

_print:     mov     rdi, 1
            mov     rax, 1
            syscall
            ret

section .data

msg:        db "Guess the flag?", 0xa
len:        equ     $ - msg
wrong:      db "Incorrect", 0xa
wlen:       equ     $ - wrong
correct:    db "Flag: "
clen:       equ     $ - correct
key:        db "Why_c4nt_1_d3c0mp1l3_th1s", 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0    ; "Why_c4nt_1_d3c0mp1l3_th1s__________________d9"
klen:       equ     $ - key
buf:        db 0x9c, 0xa0, 0x88, 0xfa, 0xac, 0xa0, 0xeb, 0xb2, 0xe1, 0xfa, 0xe1, 0xf5, 0xa1, 0xe1, 0xfa, 0xae, 0xa1, 0xee, 0xe4, 0xfa, 0xa1, 0xf3, 0xe1, 0xae, 0xef, 0xfa, 0xa1, 0xe9, 0xe9, 0xfa, 0xa1, 0xa5, 0xa0, 0xa2, 0xa3, 0xac, 0xa1, 0xa4, 0xad, 0xaf, 0x57
blen:       equ     $ - buf

section .bss

input       resb 200
inlen       equ     $ - input
flag        resb 100                ; "A5M_15n't_th4t_34sy_4ft3r_4ll_4857614902", 0xa
flen        equ     $ - flag

