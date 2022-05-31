#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/syscall.h>

void shellcode(void) {
    const char start_print[16] = "Guess the flag?\n";
    const int start_print_len = 16;
    const char wrong_print[10] = "Incorrect\n";
    const int wrong_print_len = 10;
    const char correct_print[6] = "Flag: ";
    const int correct_print_len = 6;
    const char key[45] = { 'W','h','y','_','c','4','n','t','_','1','_','d','3','c','0','m','p','1','l','3','_','t','h','1','s','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0','\0' };
    const int key_len = 45;
    const char buf[41] = { 0x9c, 0xa0, 0x88, 0xfa, 0xac, 0xa0, 0xeb, 0xb2, 0xe1, 0xfa, 0xe1, 0xf5, 0xa1, 0xe1, 0xfa, 0xae, 0xa1, 0xee, 0xe4, 0xfa, 0xa1, 0xf3, 0xe1, 0xae, 0xef, 0xfa, 0xa1, 0xe9, 0xe9, 0xfa, 0xa1, 0xa5, 0xa0, 0xa2, 0xa3, 0xac, 0xa1,  0xa4, 0xad, 0xaf, 0x57 };
    const int buf_len = 41;
    char input[200];
    const int input_len = 200;
    char flag[100];
    const int flag_len = 100;

    // Start Print
    asm (
            "lea %0, %%rsi\n"\
            "call _print"
            : : "m" (start_print), "d" (start_print_len)
        );

    // mprotect syscall
    asm (
            "mov $10, %%rax\n"\
            "mov $0x200000, %%rdi\n"\
            "mov $4096, %%rsi\n"\
            "mov $0x1, %%rdx\n"\
            "or $0x2, %%rdx\n"\
            "syscall"
            : :
        );

    // Read input
    asm (
            "mov $50, %%rdx\n"\
            "lea %0, %%rsi\n"\
            "mov $2, %%rdi\n"\
            "xor %%rax, %%rax\n"\
            "syscall"
            : : "m" (input)
        );

    // Check input is read
    asm (
            "test %%rax, %%rax\n"\
            "jle EOF"
            : :
        );

    // strncmp key with input
    asm (
            "lea %0, %%rsi\n"\
            "lea %1, %%rdi\n"\
            "cld\n"\
            "repe cmpsb\n"\
            "jrcxz decode\n"
            : : "m" (key), "m" (input), "c" (key_len - 19)
        );

    // strncmp result fails
    asm (
            "lea %0, %%rsi\n"\
            "call _print"
            : : "m" (wrong_print), "d" (wrong_print_len)
        );

    // Exit function
    asm (
            "EOF:\n"\
            "    mov $0, %%rdi\n"\
            "    mov $60, %%rax\n"\
            "    syscall"
            : :
        );

    // Decoding loop
    asm (
            "decode:"
            : :
        );
    for (int i=0; i < buf_len; i++) {
        flag[i] = (buf[i] ^ input[key_len-1]) - input[key_len-2];
    }

    // strncmp result succeeds
    asm (
            "lea %0, %%rsi\n"\
            "call _print"
            : : "m" (correct_print), "d" (correct_print_len)
        );

    // Print flag
    asm (
            "lea %0, %%rsi\n"\
            "call _print\n"\
            "jmp EOF"
            : : "m" (flag), "d" (flag_len)
        );
}

// Print Function
__asm__ (
        "_print:\n"\
        "   mov $1, %rdi\n"\
        "   mov $1, %rax\n"\
        "   syscall\n"\
        "   ret"
    );

void END_SHELLCODE(void) {}

int main(void) {
    FILE* output = fopen("output.bin", "w");
    fwrite(shellcode, (uintptr_t)END_SHELLCODE - (uintptr_t)shellcode, 1, output);
    fclose(output);

    return 0;
}
