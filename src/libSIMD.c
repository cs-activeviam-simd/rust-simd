// src/hello.c

#include <stdio.h>
#include <immintrin.h>

#ifdef __AVX512F__

void addSIMD512_C(int * data, int * datb, int * dst, int n) {
    for (int i = 0 ; i < n ; i  = i+16) {
        // Get the first 16 i32 in a SIMD type
        __m512i veca = _mm512_loadu_si512((__m512i *)(data+i));
        __m512i vecb = _mm512_loadu_si512((__m512i *)(datb+i));
        // Store the addition result in dst
        _mm512_storeu_si512((__m512i *)(dst+i), _mm512_add_epi32(veca, vecb));
    }
}

void mulSIMD512_C(int * data, int * datb, int * dst, int n) {
    for (int i = 0 ; i < n ; i  = i+16) {
        __m512i veca = _mm512_loadu_si512((__m512i *)(data+i));
        __m512i vecb = _mm512_loadu_si512((__m512i *)(datb+i));
        // Store the addition result in dst
        _mm512_storeu_si512((__m512i *)(dst+i), _mm512_mul_epi32(veca, vecb));
    }
}


#endif


#ifdef __AVX2__

void addSIMD256_C(int * data, int * datb, int * dst, int n) {
    for (int i = 0 ; i < n ; i  = i+8) {
        // Get the first 8 i32 in a SIMD type
        __m256i veca = _mm256_loadu_si256((__m256i *) (data+i));
        __m256i vecb = _mm256_loadu_si256((__m256i *) (datb+i));

        // Store the addition result in dst
        _mm256_storeu_si256((__m256i *)(dst+i), _mm256_add_epi32(veca, vecb));
    }
}
#endif
