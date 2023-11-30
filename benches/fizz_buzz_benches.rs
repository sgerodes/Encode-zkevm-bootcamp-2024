use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fizz_buzz::*;

static TEST_INPUT_UPPER_BOUND: u64 = 6301;


fn test_fizzbuzz_perf<F>(c: &mut Criterion, test_name: &str, fizzbuzz_func: F) where F: Fn(i32) -> Option<&'static str>,
{
    c.bench_function(test_name, |b| {
        b.iter(|| {
            (1..=TEST_INPUT_UPPER_BOUND)
                .for_each(|i| {
                    fizzbuzz_func(black_box(i as i32));
                })
        })
    });
}

fn criterion_benchmark_fizzbuzz_match_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizz_buzz_match", fizz_buzz_match);
}

fn criterion_benchmark_fizz_buzz_match_2_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizz_buzz_match_2", fizz_buzz_match_2);
}

fn criterion_benchmark_fizz_buzz_match_3_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizz_buzz_match_3", fizz_buzz_match_3);
}

fn criterion_benchmark_fizz_buzz_match_4_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizz_buzz_match_4", fizz_buzz_match_4);
}

fn criterion_benchmark_fizz_buzz_match_5_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizz_buzz_match_5", fizz_buzz_match_5);
}

fn criterion_benchmark_fizzbuzz_static_lookup_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizzbuzz_static_lookup_table", fizzbuzz_static_lookup_table);
}

fn criterion_benchmark_fizzbuzz_lookup_perf(c: &mut Criterion) {
    test_fizzbuzz_perf(c, "fizz_buzz_lookup", fizz_buzz_lookup);
}

criterion_group!(benches,
    criterion_benchmark_fizzbuzz_lookup_perf,
    criterion_benchmark_fizzbuzz_static_lookup_perf,
    criterion_benchmark_fizzbuzz_match_perf,
    criterion_benchmark_fizz_buzz_match_2_perf,
    criterion_benchmark_fizz_buzz_match_3_perf,
    criterion_benchmark_fizz_buzz_match_4_perf,
    criterion_benchmark_fizz_buzz_match_5_perf);
criterion_main!(benches);
