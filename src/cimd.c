// src/hello.c

#include <stdio.h>
#include <immintrin.h>


void add_simd_c( int * data, long long int * datb, long long int * dst ) {
    // Get the first 8 i32 in a SIMD type
    __m256i veca = _mm256_loadu_si256(data);
    __m256i vecb = _mm256_loadu_si256(datb);

    // Store the addition result in dst
    _mm256_storeu_si256(dst, _mm256_add_epi32(veca, vecb));
}

void add_simd_not( int * data, int * datb, int * dst ) {
   for (int i =0; i<8;i++) {
       dst[i] = data[i] + datb[i];
   }
}
