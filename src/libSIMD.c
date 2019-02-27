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

int filterSumSIMD512_C(int x, int * data, int * datb, int n) {
    __m512i vecSum = _mm512_set1_epi32(0);
    __m512i vecX = _mm512_set1_epi32(x);
    __m512i vec0 = _mm512_set1_epi32(0);
    for (int i = 0 ; i < n ; i  = i+16) {
        __m512i veca = _mm512_loadu_si512((__m512i *)(data+i));
        __m512i vecb = _mm512_loadu_si512((__m512i *)(datb+i));
        // Mask the positions of elements equal to x
        __mmask16 mask = _mm512_cmpeq_epi32_mask(vecX, veca);
        // Add elements from datb where masks is set
        vecSum = _mm512_mask_add_epi32(vecSum, mask, vecSum, vecb);
    }
    // Reduce vecSum into a single int
    return _mm512_reduce_add_epi32(vecSum);
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

int filterSumSIMD256_C(int x, int * data, int * datb, int n) {
    __m256i vecSum = _mm256_set1_epi32(0);
    __m256i vecX = _mm256_set1_epi32(x);
    __m256i vec0 = _mm256_set1_epi32(0);
    for (int i = 0 ; i < n ; i  = i+16) {
        __m256i veca = _mm256_loadu_si256((__m256i *)(data+i));
        // Mask the positions of elements equal to x
        __m256i mask = _mm256_cmpeq_epi32(vecX, veca);
        // Load only the elements where the mask is set
        __m256i vecb = _mm256_maskload_epi32((datb+i), mask);
        // Add elements from datb
        vecSum = _mm256_add_epi32(vecSum, vecb);
    }
    // Reduce vecSum into a single int
    int* sum = (int*) calloc(8, sizeof(int));
    _mm256_storeu_si256((__m256i *)sum, vecSum);
    int res = 0;
    for (int i = 0 ; i < 8 ; i++) {
        res += sum[i];
    }
    free(sum);
    return res;
}
#endif
