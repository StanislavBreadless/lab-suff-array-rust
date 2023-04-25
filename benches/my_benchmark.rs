use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use suffix_array_build::build_suffix_array;

fn generate_random_string(n: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let charset_len = charset.len();
    let mut rng = rand::thread_rng();

    (0..n)
        .map(|_| charset.chars().nth(rng.gen_range(0..charset_len)).unwrap())
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut len = 1000;

    while len <= 1000000 {
        let string = generate_random_string(len);
        c.bench_function(&format!("sum_vec_{}", len), |b| b.iter(|| build_suffix_array(string.clone())));
        len *= 10;
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
