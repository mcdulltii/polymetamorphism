#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define BLOCK_SIZE 8 //XTEA uses 64-bit blocks, 64 bits is 8 bytes

// key is Ut dn JU M2
unsigned int key[4] = { 0x5574, 0x646e, 0x4a55, 0x4d32 };

void encipher(unsigned int num_rounds, uint32_t v[2], uint32_t const key[4]) 
{
    unsigned int i;
    uint32_t v0 = v[0], v1 = v[1], sum = 0, delta = 0x9E3779B9;
    for (i = 0; i < num_rounds; i++) 
    {
        v0 += (((v1 << 4) ^ (v1 >> 5)) + v1) ^ (sum + key[sum & 3]);
        sum += delta;
        v1 += (((v0 << 4) ^ (v0 >> 5)) + v0) ^ (sum + key[(sum >> 11) & 3]);
    }
    v[0] = v0; v[1] = v1;
}

void decipher(unsigned int num_rounds, uint32_t v[2], uint32_t const key[4])
{
    unsigned int i;
    uint32_t v0 = v[0], v1 = v[1], delta = 0x9E3779B9, sum = delta * num_rounds;
    for (i = 0; i < num_rounds; i++)
    {
        v1 -= (((v0 << 4) ^ (v0 >> 5)) + v0) ^ (sum + key[(sum >> 11) & 3]);
        sum -= delta;
        v0 -= (((v1 << 4) ^ (v1 >> 5)) + v1) ^ (sum + key[sum & 3]);
    }
    v[0] = v0; v[1] = v1;
}

void printHex(char *str) {

    int i;

    printf("Length = %d, Flag = ", strlen(str)); 
    for(i=0; i<strlen(str); i++) {
        printf("\\x%02x", (unsigned char)(int)str[i]);
    }
    printf("\n");
}

int main(int argc, char *argv[])
{
    unsigned char flag[] = "TISC{P0lyM0rph15m_r3m1nd5_m3_0f_M0rb1us_7359430}";

    char str[512];
    strcpy(str, flag);

    printf("[Original]\n");
    printHex(str);
    
    for (int i=0; i < (int)strlen(str) / BLOCK_SIZE; i++)
        encipher(32, (uint32_t*)(str + i * BLOCK_SIZE), key);

    printf("[Encrypted]\n");
    printHex(str);
    printf("\n");

    printf("[Encrypted]\n");
    printHex(str);
    
    for (int i=0; i < (int)strlen(str) / BLOCK_SIZE; i++)
        decipher(32, (uint32_t*)(str + i * BLOCK_SIZE), key);

    printf("[Source]\n");
    printHex(str);
    printf("\n");

    return 0;
}

