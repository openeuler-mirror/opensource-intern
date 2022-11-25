#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "SM3.h"

unsigned int SM3::T(int j)
{
    if (j < 16)
        return 0x79cc4519;
    return 0x7a879d8a;
}

unsigned int SM3::FF(unsigned int x, unsigned int y, unsigned int z, int j)
{
    if (j < 16)
        return x ^ y ^ z;
    return (x & y) | (x & z) | (y & z);
}

unsigned int SM3::GG(unsigned int x, unsigned int y, unsigned int z, int j)
{
    if (j < 16)
        return x ^ y ^ z;
    return (x & y) | (~x & z);
}

unsigned int SM3::P0(unsigned int X)
{
    return X ^ ROTL(X, 9) ^ ROTL(X, 17);
}

unsigned int SM3::P1(unsigned int X)
{
    return X ^ ROTL(X, 15) ^ ROTL(X, 23);
}

unsigned int SM3::ROTL(unsigned int x, int n)
{
    return x << n | x >> (32 - n);
}

void SM3::getbytes(unsigned char *bytes, unsigned long long l)
{
    for (int i = 0; i < 8; i++)
        bytes[i] = l >> (56 - i * 8);
}

unsigned char *SM3::padding(unsigned char *input, unsigned long long *len)
{
    int padlen, length = *len;
    int tmp = length % 64;
    if (tmp == 56)
        padlen = 64;
    else if (tmp < 56)
        padlen = 56 - tmp;
    else
        padlen = 120 - tmp;
    unsigned char *output = (unsigned char *)malloc(length + padlen + 8);
    memcpy(output, input, length);
    output[length] = 128;
    for (int i = 0; i < padlen - 1; i++)
        output[length + i + 1] = 0;
    unsigned char bytes[8];
    getbytes(bytes, length * 8);
    length += padlen;
    memcpy(output + length, bytes, 8);
    *len = length + 8;
    return output;
}

void SM3::expend(unsigned char *m, unsigned int *w)
{
    int i;
    for (i = 0; i < 16; i++)
        w[i] = m[i * 4] << 24 | m[i * 4 + 1] << 16 | m[i * 4 + 2] << 8 | m[i * 4 + 3];
    for (i = 16; i < 68; i++)
        w[i] = P1(w[i - 16] ^ w[i - 9] ^ ROTL(w[i - 3], 15)) ^ ROTL(w[i - 13], 7) ^ w[i - 6];
}

void SM3::doFinal(unsigned char *msg, unsigned char *res)
{
    unsigned int IV[8] = {0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e};
    unsigned long long len = strlen((char *)msg);
    unsigned char *nmsg = padding(msg, &len);
    for (int i = 0; i < len / 64; i++)
    {
        unsigned char tmp[64];
        unsigned int w[68] = {0}, nw[64] = {0};
        memcpy(tmp, nmsg + i * 64, 64);
        expend(tmp, w);
        for (int j = 0; j < 64; j++)
            nw[j] = w[j] ^ w[j + 4];
        unsigned int A = IV[0], B = IV[1], C = IV[2], D = IV[3], E = IV[4], F = IV[5], G = IV[6], H = IV[7];
        for (int j = 0; j < 64; j++)
        {
            unsigned int SS1 = ROTL(ROTL(A, 12) + E + ROTL(T(j), j), 7);
            unsigned int SS2 = SS1 ^ ROTL(A, 12);
            unsigned int TT1 = FF(A, B, C, j) + D + SS2 + nw[j];
            unsigned int TT2 = GG(E, F, G, j) + H + SS1 + w[j];
            D = C;
            C = ROTL(B, 9);
            B = A;
            A = TT1;
            H = G;
            G = ROTL(F, 19);
            F = E;
            E = P0(TT2);
        }
        IV[0] = IV[0] ^ A;
        IV[1] = IV[1] ^ B;
        IV[2] = IV[2] ^ C;
        IV[3] = IV[3] ^ D;
        IV[4] = IV[4] ^ E;
        IV[5] = IV[5] ^ F;
        IV[6] = IV[6] ^ G;
        IV[7] = IV[7] ^ H;
    }
    for (int i = 0; i < 8; i++)
    {
        res[4 * i] = (IV[i] >> 24) & 0xff;
        res[4 * i + 1] = (IV[i] >> 16) & 0xff;
        res[4 * i + 2] = (IV[i] >> 8) & 0xff;
        res[4 * i + 3] = IV[i] & 0xff;
    }
    free(nmsg);
}

void test_func()
{
    char msg[100] = {"adminadmin"};
    SM3 sm3;
    unsigned char res[32] = {0};
    sm3.doFinal((unsigned char *)msg, res);
    printf("text: %s\nhash res:\n", msg);
    for (int i = 0; i < 32; i++)
    {
        printf("%02x", res[i]);
        if ((i + 1) % 4 == 0)
            printf(" ");
    }
    printf("\n");
}
