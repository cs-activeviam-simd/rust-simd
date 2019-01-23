#![feature(test)]

const VECTOR_SIZE: usize = 256 << 0;

extern crate rand;
extern crate test;

pub fn add_reg(data: &[i32], datb: &[i32], res: &mut [i32]) {
    for i in 0..VECTOR_SIZE {
        res[i] = data[i] + datb[i]
    }
}

pub fn add_simd(data: &[i32], datb: &[i32], res: &mut [i32]) {
    #[cfg(target_arch = "x86_64")]
    {
        // Nothing happens when no SIMD
        #[cfg(target_feature = "avx2")]
        unsafe {
            for i in 0..VECTOR_SIZE / 8 {
                add_simd_8(&data[i * 8..], &datb[i * 8..], &mut res[i * 8..]);
            }
        };
    }
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

extern { fn add_simd_c(a: *const i32, b: *const i32, c: *mut i32, size: i32); }

pub fn add_simd_ffi(data: &[i32], datb: &[i32], res: &mut [i32]) {
    unsafe {
        add_simd_c(data.as_ptr(), datb.as_ptr(), res.as_mut_ptr(), VECTOR_SIZE as i32);
    }
}


pub fn mul_reg(data: &[i32], datb: &[i32], res: &mut [i32]) {
    for i in 0..VECTOR_SIZE {
        res[i] = data[i].wrapping_mul(datb[i]); // Does not overflow
    }
}

pub fn mul_simd(data: &[i32], datb: &[i32], res: &mut [i32]) {
    #[cfg(target_arch = "x86_64")]
    {
        // Nothing happens when no SIMD
        #[cfg(target_feature = "avx2")]
        unsafe {
            for i in 0..VECTOR_SIZE / 8 {
                mul_simd_8(&data[i * 8..], &datb[i * 8..], &mut res[i * 8..]);
            }
        };
    }
}

// Adds two vectors of 8 i32 through SIMD, loads it in dst
// It gets the first 8 i32s of data and datb, does not care about further i32s
#[target_feature(enable = "avx2")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
unsafe fn mul_simd_8(data: &[i32], datb: &[i32], dst: &mut [i32]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // Get the first 8 i32 in a SIMD type
    let veca = _mm256_loadu_si256(data.as_ptr() as *const _);
    let vecb = _mm256_loadu_si256(datb.as_ptr() as *const _);
    // Store the addition result in dst
    _mm256_storeu_si256(dst.as_ptr() as *mut _, _mm256_mullo_epi32(veca, vecb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn rand_vec() -> Vec<i32> {
        let mut res = Vec::with_capacity(VECTOR_SIZE);
        for _ in 0..VECTOR_SIZE {
            res.push(rand::random::<i32>() / 2);
        }
        res
    }

    fn empty_vec() -> Vec<i32> {
        let mut res = Vec::with_capacity(VECTOR_SIZE);
        unsafe {
            res.set_len(VECTOR_SIZE);
        }
        res
    }

    #[test]
    fn test_add_simd() {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        let mut res_simd = empty_vec();
        let mut res_cimd = empty_vec();
        add_reg(&data, &datb, &mut res_reg);
        add_simd(&data, &datb, &mut res_simd);
        add_simd_ffi(&data, &datb, &mut res_cimd);
        for i in 0..VECTOR_SIZE {
            assert_eq!(res_reg[i], res_simd[i]);
            assert_eq!(res_cimd[i], res_simd[i]);
        }
    }

    #[bench]
    fn bench_add_reg(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        b.iter(|| add_reg(&data, &datb, &mut res_reg));
    }

    #[bench]
    fn bench_add_simd(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| add_simd(&data, &datb, &mut res_simd));
    }

    #[bench]
    fn bench_add_cimd(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| add_simd_ffi(&data, &datb, &mut res_simd));
    }

    #[test]
    fn test_mul_simd() {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        let mut res_simd = empty_vec();
        mul_reg(&data, &datb, &mut res_reg);
        mul_simd(&data, &datb, &mut res_simd);
        for i in 0..VECTOR_SIZE {
            assert_eq!(res_reg[i], res_simd[i]);
        }
    }

    #[bench]
    fn bench_mul_reg(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        b.iter(|| mul_reg(&data, &datb, &mut res_reg));
    }

    #[bench]
    fn bench_mul_simd(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| mul_simd(&data, &datb, &mut res_simd));
    }
}
