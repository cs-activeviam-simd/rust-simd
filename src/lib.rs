#![feature(test)]

const VECTOR_SIZE: usize = 4096;

extern crate rand;
extern crate test;

pub fn add_reg(data: [i32; VECTOR_SIZE], datb: [i32; VECTOR_SIZE]) -> [i32; VECTOR_SIZE] {
    let mut datc = [0; VECTOR_SIZE];
    for i in 0..VECTOR_SIZE {
        datc[i] = data[i] + datb[i]
    }
    datc
}

pub fn add_simd(data: [i32; VECTOR_SIZE], datb: [i32; VECTOR_SIZE]) -> [i32; VECTOR_SIZE] {
    let mut datc = [0; VECTOR_SIZE];
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe {
                for i in 0..VECTOR_SIZE / 8 {
                    add_simd_8(&data[i * 8..], &datb[i * 8..], &mut datc[i * 8..]);
                }
                datc
            };
        }
    }
    // Nothing happens when no SIMD
    datc
}

// Adds two vectors of 8 i32 through SIMD, loads it in dst
// It gets the first 8 i32s of data and datb, does not care about further i32s
#[target_feature(enable = "avx2")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
unsafe fn add_simd_8(data: &[i32], datb: &[i32], dst: &mut [i32]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // Get the first 8 i32 in a SIMD type
    let veca = _mm256_loadu_si256(data.as_ptr() as *const _);
    let vecb = _mm256_loadu_si256(datb.as_ptr() as *const _);

    // Store the addition result in dst
    _mm256_storeu_si256(dst.as_ptr() as *mut _, _mm256_add_epi32(veca, vecb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn rand_vec() -> [i32; VECTOR_SIZE] {
        let mut res = [0; VECTOR_SIZE];
        for i in 0..VECTOR_SIZE {
            res[i] = rand::random::<i32>() / 2;
        }
        res
    }

    #[test]
    fn test_add_simd() {
        let data = rand_vec();
        let datb = rand_vec();
        let res1 = add_reg(data, datb);
        let res2 = add_simd(data, datb);
        for i in 0..VECTOR_SIZE {
            assert_eq!(res1[i], res2[i]);
        }
    }

    #[bench]
    fn bench_add_reg(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        b.iter(|| add_reg(data, datb));
    }

    #[bench]
    fn bench_add_simd(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        b.iter(|| add_simd(data, datb));
    }
}
