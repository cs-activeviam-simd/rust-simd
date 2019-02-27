#![feature(test)]
#![allow(non_snake_case)]
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref ARRAY_LENGTH: usize = std::env::vars()
        .find_map(|(key, val)| if key == "ARRAY_LENGTH" {
            val.parse::<usize>().ok()
        } else {
            None
        })
        .unwrap_or(256);
}

extern crate rand;
extern crate test;

//  ------- Addition --------

pub fn addRegular(data: &[i32], datb: &[i32], res: &mut [i32]) {
    for i in 0..*ARRAY_LENGTH {
        res[i] = data[i] + datb[i]
    }
}

#[cfg(target_feature = "avx2")]
pub fn addSIMD256Rust(data: &[i32], datb: &[i32], res: &mut [i32]) {
    // Nothing happens when no SIMD
    unsafe {
        for i in 0..*ARRAY_LENGTH / 8 {
            addSIMD256Rust_8(&data[i * 8..], &datb[i * 8..], &mut res[i * 8..]);
        }
    }
}

// Adds two vectors of 8 i32 through SIMD, loads it in dst
// It gets the first 8 i32s of data and datb, does not care about further i32s
#[target_feature(enable = "avx2")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
unsafe fn addSIMD256Rust_8(data: &[i32], datb: &[i32], dst: &mut [i32]) {
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

#[cfg(target_feature = "avx2")]
extern "C" {
    fn addSIMD256_C(a: *const i32, b: *const i32, c: *mut i32, size: i32);
}

#[cfg(target_feature = "avx512f")]
extern "C" {
    fn addSIMD512_C(a: *const i32, b: *const i32, c: *mut i32, size: i32);
}

#[cfg(target_feature = "avx2")]
pub fn addSIMD256(data: &[i32], datb: &[i32], res: &mut [i32]) {
    unsafe {
        addSIMD256_C(
            data.as_ptr(),
            datb.as_ptr(),
            res.as_mut_ptr(),
            *ARRAY_LENGTH as i32,
        )
    }
}


#[cfg(target_feature = "avx512f")]
pub fn addSIMD512(data: &[i32], datb: &[i32], res: &mut [i32]) {
    unsafe {
        addSIMD512_C(
            data.as_ptr(),
            datb.as_ptr(),
            res.as_mut_ptr(),
            *ARRAY_LENGTH as i32,
        )
    }
}



//  ------- Multiplication --------

pub fn mulRegular(data: &[i32], datb: &[i32], res: &mut [i32]) {
    for i in 0..*ARRAY_LENGTH {
        res[i] = data[i].wrapping_mul(datb[i]); // Does not overflow
    }
}

#[cfg(target_feature = "avx2")]
pub fn mulSIMD256Rust(data: &[i32], datb: &[i32], res: &mut [i32]) {
    // Nothing happens when no SIMD
    unsafe {
        for i in 0..*ARRAY_LENGTH / 8 {
            mulSIMD256Rust_8(&data[i * 8..], &datb[i * 8..], &mut res[i * 8..]);
        }
    }
}

// Adds two vectors of 8 i32 through SIMD, loads it in dst
// It gets the first 8 i32s of data and datb, does not care about further i32s
#[target_feature(enable = "avx2")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
unsafe fn mulSIMD256Rust_8(data: &[i32], datb: &[i32], dst: &mut [i32]) {
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


#[cfg(target_feature = "avx512f")]
extern "C" {
    fn mulSIMD512_C(a: *const i32, b: *const i32, c: *mut i32, size: i32);
}

#[cfg(target_feature = "avx512f")]
pub fn mulSIMD512(data: &[i32], datb: &[i32], res: &mut [i32]) {
    unsafe {
        mulSIMD512_C(
            data.as_ptr(),
            datb.as_ptr(),
            res.as_mut_ptr(),
            *ARRAY_LENGTH as i32,
        );
        return;
    }
}




//  ------- FilterSum --------

pub fn filterSumRegular(x : i32, data: &[i32], datb: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..*ARRAY_LENGTH {
        if data[i] == x {
            sum += datb[i];
        }
    }
    sum
}


#[cfg(target_feature = "avx2")]
extern "C" {
    fn filterSumSIMD256_C(x: i32, data: *const i32, datb: *const i32, size: i32) -> i32;
}

#[cfg(target_feature = "avx512f")]
extern "C" {
    fn filterSumSIMD512_C(x: i32, data: *const i32, datb: *const i32, size: i32) -> i32;
}

#[cfg(target_feature = "avx2")]
pub fn filterSumSIMD256(x : i32, data: &[i32], datb: &[i32]) -> i32 {
    unsafe {
        filterSumSIMD256_C(
            x,
            data.as_ptr(),
            datb.as_ptr(),
            *ARRAY_LENGTH as i32,
        )
    }
}

#[cfg(target_feature = "avx512f")]
pub fn filterSumSIMD512(x : i32, data: &[i32], datb: &[i32]) -> i32 {
    unsafe {
        filterSumSIMD512_C(
            x,
            data.as_ptr(),
            datb.as_ptr(),
            *ARRAY_LENGTH as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn rand_vec() -> Vec<i32> {
        let mut res = Vec::with_capacity(*ARRAY_LENGTH);
        for _ in 0..*ARRAY_LENGTH {
            res.push(rand::random::<i32>() / 2);
        }
        res
    }

    fn rand_small_vec() -> Vec<i32> {
        let mut res = Vec::with_capacity(*ARRAY_LENGTH);
        for _ in 0..*ARRAY_LENGTH {
            res.push(rand::random::<i32>() % 32);
        }
        res
    }

    fn empty_vec() -> Vec<i32> {
        let mut res = Vec::with_capacity(*ARRAY_LENGTH);
        unsafe {
            res.set_len(*ARRAY_LENGTH);
        }
        res
    }

    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_add() {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        let mut res_simd = empty_vec();
        let mut res_simd256 = empty_vec();
        #[cfg(target_feature = "avx512f")]
        let mut res_simd512 = empty_vec();

        addRegular(&data, &datb, &mut res_reg);
        addSIMD256Rust(&data, &datb, &mut res_simd);
        addSIMD256(&data, &datb, &mut res_simd256);
        #[cfg(target_feature = "avx512f")]
        addSIMD512(&data, &datb, &mut res_simd512);

        for i in 0..*ARRAY_LENGTH {
            assert_eq!(res_reg[i], res_simd[i]);
            assert_eq!(res_simd256[i], res_reg[i]);
            #[cfg(target_feature = "avx512f")]
            assert_eq!(res_simd512[i], res_reg[i]);
        }
    }

    #[bench]
    fn bench_addRegular(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        b.iter(|| addRegular(&data, &datb, &mut res_reg));
    }

    #[bench]
    #[cfg(target_feature = "avx2")]
    fn bench_addSIMD256Rust(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| addSIMD256Rust(&data, &datb, &mut res_simd));
    }

    #[bench]
    #[cfg(target_feature = "avx2")]
    fn bench_addSIMD256(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| addSIMD256(&data, &datb, &mut res_simd));
    }

    #[bench]
    #[cfg(target_feature = "avx512f")]
    fn bench_addSIMD512(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| addSIMD512(&data, &datb, &mut res_simd));
    }

    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_mul() {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        let mut res_simd = empty_vec();
        #[cfg(target_feature = "avx512f")]
        let mut res_simd512 = empty_vec();

        mulRegular(&data, &datb, &mut res_reg);
        mulSIMD256Rust(&data, &datb, &mut res_simd);
        #[cfg(target_feature = "avx512f")]
        mulSIMD512(&data, &datb, &mut res_simd512);

        for i in 0..*ARRAY_LENGTH {
            assert_eq!(res_reg[i], res_simd[i]);
            #[cfg(target_feature = "avx512f")]
            assert_eq!(res_reg[i], res_simd512[i]);
        }
    }

    #[bench]
    fn bench_mulRegular(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_reg = empty_vec();
        b.iter(|| mulRegular(&data, &datb, &mut res_reg));
    }

    #[bench]
    #[cfg(target_feature = "avx2")]
    fn bench_mulSIMD256Rust(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| mulSIMD256Rust(&data, &datb, &mut res_simd));
    }

    #[bench]
    #[cfg(target_feature = "avx512f")]
    fn bench_mulSIMD512(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        let mut res_simd = empty_vec();
        b.iter(|| mulSIMD512(&data, &datb, &mut res_simd));
    }


    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_filterSum() {
        let x = rand::random::<i32>() % 32;
        let data = rand_small_vec();
        let datb = rand_small_vec();

        let res_reg = filterSumRegular(x, &data, &datb);
        let res_simd256 = filterSumSIMD256(x, &data, &datb);
        #[cfg(target_feature = "avx512f")]
        let res_simd512 = filterSumSIMD512(x, &data, &datb);

        assert_eq!(res_reg, res_simd256);
        #[cfg(target_feature = "avx512f")]
        assert_eq!(res_reg, res_simd512);
    }

    #[bench]
    fn bench_filterSumRegular(b: &mut Bencher) {
        let x = rand::random::<i32>() % 32;
        let data = rand_small_vec();
        let datb = rand_small_vec();
        b.iter(|| filterSumRegular(x, &data, &datb));
    }

    #[bench]
    #[cfg(target_feature = "avx2")]
    fn bench_filterSumSIMD256(b: &mut Bencher) {
        let x = rand::random::<i32>() % 32;
        let data = rand_small_vec();
        let datb = rand_small_vec();
        b.iter(|| filterSumSIMD256(x, &data, &datb));
    }

    #[bench]
    #[cfg(target_feature = "avx512f")]
    fn bench_filterSumSIMD512(b: &mut Bencher) {
        let x = rand::random::<i32>() % 32;
        let data = rand_small_vec();
        let datb = rand_small_vec();
        b.iter(|| filterSumSIMD512(x, &data, &datb));
    }

}
