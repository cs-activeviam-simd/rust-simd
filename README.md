# rust-simd
Benchamrking SIMD operations in Rust
The SIMD operation currently only works on avx2-compatible processors.

## Usage

Stable Rust is recommended.

Run the tests:
`cargo test`

Run the benchmark:
`ARRAY_LENGTH=<> cargo bench`
ARRAY_LENGTH defaults to 256. Add `-- --nocapture` to get the test/bench outputs

## Adding a benchmark

### Rust code

Create a regular function to compare to.
Add a bench for it in `benches/bench.rs` and add it to the correct `criterion_group!`:
```rust
fn bench_addRegular(c: &mut Criterion) {}
```

Create a test fonction to compare the SIMD results to the regular ones.
```rust
    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_benchname() {}
```

Add the SIMD code. If in C, import it with
```rust
#[cfg(target_feature = "avx2")]
extern "C" {
    fn benchSIMD256_C(a: *const i32, b: *const i32, c: *mut i32, size: i32);
}
```

and call it from Rust with the proper pointer casting.

Create a bench for it:
```rust
    #[bench]
    #[cfg(target_feature = "avx2")]
    fn bench_SIMD: &mut Bencher) {}
```

## Run the benchmarks

First test that your results are coherent
```
ARRAY_LENGTH=<> cargo test <testname>
```
Try a benchmark
```
ARRAY_LENGTH=<> cargo bench <benchname>
```

Use the runner to run on multiple sizes (multiplied by 2 at each iteration)
```
./run_criterion.py <minsize> <maxsize> [extra cargo args]
```
