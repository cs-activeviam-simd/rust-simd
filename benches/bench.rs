#![allow(non_snake_case)]

use rust_simd::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SIZES: Vec<usize> = {
        let mut sizes = vec![*ARRAY_LENGTH_MIN];
        let mut i = 0;
        while sizes[i] < *ARRAY_LENGTH_MAX {
            sizes.push(sizes[i]*2);
            i+=1;
        }
        sizes
    };
}

#[macro_use]
extern crate criterion;
use criterion::Criterion;


fn bench_addRegular(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_reg = empty_vec();
    c.bench_function_over_inputs("addRegular", move |b, &&size| { b.iter(|| addRegular(&data[0..size], &datb[0..size], &mut res_reg[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx2")]
fn bench_addSIMD256Rust(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_simd = empty_vec();
    c.bench_function_over_inputs("addSIMD256Rust", move |b, &&size| {  b.iter(|| addSIMD256Rust(&data[0..size], &datb[0..size], &mut res_simd[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx2")]
fn bench_addSIMD256(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_simd = empty_vec();
    c.bench_function_over_inputs("addSIMD256", move |b, &&size| {  b.iter(|| addSIMD256(&data[0..size], &datb[0..size], &mut res_simd[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx512f")]
fn bench_addSIMD512(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_simd = empty_vec();
    c.bench_function_over_inputs("addSIMD512", move |b, &&size| {  b.iter(|| addSIMD512(&data[0..size], &datb[0..size], &mut res_simd[0..size])) }, &*SIZES);
}


fn bench_mulRegular(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_reg = empty_vec();
    c.bench_function_over_inputs("mulRegular", move |b, &&size| {  b.iter(|| mulRegular(&data[0..size], &datb[0..size], &mut res_reg[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx2")]
fn bench_mulSIMD256Rust(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_simd = empty_vec();
    c.bench_function_over_inputs("mulSIMD256Rust", move |b, &&size| {  b.iter(|| mulSIMD256Rust(&data[0..size], &datb[0..size], &mut res_simd[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx512f")]
fn bench_mulSIMD512(c: &mut Criterion) {
    let data = rand_vec();
    let datb = rand_vec();
    let mut res_simd = empty_vec();
    c.bench_function_over_inputs("mulSIMD512", move |b, &&size| {  b.iter(|| mulSIMD512(&data[0..size], &datb[0..size], &mut res_simd[0..size])) }, &*SIZES);
}

fn bench_filterSumRegular(c: &mut Criterion) {
    let x = rand::random::<i32>() % 32;
    let data = rand_small_vec();
    let datb = rand_small_vec();
    c.bench_function_over_inputs("filterSumRegular", move |b, &&size| {  b.iter(|| filterSumRegular(x, &data[0..size], &datb[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx2")]
fn bench_filterSumSIMD256(c: &mut Criterion) {
    let x = rand::random::<i32>() % 32;
    let data = rand_small_vec();
    let datb = rand_small_vec();
    c.bench_function_over_inputs("filterSumSIMD256", move |b, &&size| {  b.iter(|| filterSumSIMD256(x, &data[0..size], &datb[0..size])) }, &*SIZES);
}


#[cfg(target_feature = "avx512f")]
fn bench_filterSumSIMD512(c: &mut Criterion) {
    let x = rand::random::<i32>() % 32;
    let data = rand_small_vec();
    let datb = rand_small_vec();
    c.bench_function_over_inputs("filterSumSIMD512", move |b, &&size| {  b.iter(|| filterSumSIMD512(x, &data[0..size], &datb[0..size])) }, &*SIZES);
}

criterion_group!(regular, bench_addRegular, bench_mulRegular, bench_filterSumRegular);

#[cfg(target_feature = "avx2")]
criterion_group!(avx2, bench_addSIMD256, bench_addSIMD256Rust, bench_mulSIMD256Rust, bench_filterSumSIMD256);

#[cfg(target_feature = "avx512f")]
criterion_group!(avx512, bench_addSIMD512, bench_mulSIMD512, bench_filterSumSIMD512);

#[cfg(target_feature = "avx512f")]
criterion_main!(regular, avx2, avx512);

#[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
criterion_main!(regular, avx2);

#[cfg(all(not(target_feature = "avx2"), not(target_feature = "avx512f")))]
criterion_main!(regular);
