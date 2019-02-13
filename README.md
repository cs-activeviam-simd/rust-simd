# rust-simd
Benchamrking SIMD operations in Rust
The SIMD operation currently only works on avx2-compatible processors.

## Usage

You need to use Nightly Rust:
`rustup override set nightly`

Run the tests:
`cargo test`

Run the benchmark:
`ARRAY_LENGTH=<> cargo bench`
ARRAY_LENGTH defaults to 256. Add `-- --nocapture` to get the test/bench outputs
