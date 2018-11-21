#![feature(test)]

const VECTOR_SIZE: usize = 4096;

extern crate test;
extern crate rand;

pub fn add_reg(data: [i32; VECTOR_SIZE], datb: [i32; VECTOR_SIZE]) -> [i32; VECTOR_SIZE] {
    let mut datc = [0;VECTOR_SIZE];
    for i in 0..VECTOR_SIZE {
        datc[i] = data[i] + datb[i]
    }
    return datc
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn rand_vec() -> [i32; VECTOR_SIZE] {
        let mut res = [0; VECTOR_SIZE];
        for i in 0..VECTOR_SIZE {
            res[i] = rand::random::<i32>();
        }
        res
    }

    #[bench]
    fn bench_add_reg(b: &mut Bencher) {
        let data = rand_vec();
        let datb = rand_vec();
        b.iter(|| add_reg(data, datb));
    }
}
