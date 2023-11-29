use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fizz_buzz::*;

static TEST_INPUT_UPPER_BOUND: u64 = 76301;

fn test_fizzbuzz_perf<F>(fizzbuzz_func: F)
    where F: Fn(i32) -> Option<&'static str>,
{
    (1..=TEST_INPUT_UPPER_BOUND)
        .for_each(|i| {
            fizzbuzz_func(black_box(i as i32));
        })
}

fn criterion_benchmark_fizzbuzz_match_perf(c: &mut Criterion) {

    c.bench_function("fizzbuzz_match_perf", |b| {
        b.iter(|| {
            test_fizzbuzz_perf(fizzbuzz_match);
        })
    });
}

fn criterion_benchmark_fizz_buzz_match_2_perf(c: &mut Criterion) {

    c.bench_function("fizzbuzz_match_2_perf", |b| {
        b.iter(|| {
            test_fizzbuzz_perf(fizz_buzz_match_2);
        })
    });
}

fn criterion_benchmark_fizzbuzz_static_lookup_perf(c: &mut Criterion) {

    c.bench_function("fizzbuzz_static_lookup_perf", |b| {
        b.iter(|| {
            test_fizzbuzz_perf(fizzbuzz_static_lookup_table);
        })
    });
}


fn criterion_benchmark_fizzbuzz_lookup_perf(c: &mut Criterion) {

    c.bench_function("fizzbuzz_lookup_perf", |b| {
        b.iter(|| {
            test_fizzbuzz_perf(fizz_buzz_lookup);
        })
    });
}

criterion_group!(benches, criterion_benchmark_fizzbuzz_lookup_perf, criterion_benchmark_fizzbuzz_static_lookup_perf, criterion_benchmark_fizzbuzz_match_perf, criterion_benchmark_fizz_buzz_match_2_perf);
criterion_main!(benches);