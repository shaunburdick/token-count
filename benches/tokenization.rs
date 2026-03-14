use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use token_count::tokenizers::registry::ModelRegistry;

fn benchmark_small_input(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();
    let text = "Hello world! This is a small test input."; // ~100 bytes

    c.bench_function("tokenization/small_100bytes", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(text)))
    });
}

fn benchmark_medium_input(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();

    // Generate ~1KB of text
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(20); // ~900 bytes

    c.bench_function("tokenization/medium_1kb", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(&text)))
    });
}

fn benchmark_large_input(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();

    // Generate ~10KB of text
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(200); // ~9KB

    c.bench_function("tokenization/large_10kb", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(&text)))
    });
}

fn benchmark_gemini_tokenization(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gemini", false).unwrap();
    let text = "Hello world! This is a small test input."; // ~100 bytes

    c.bench_function("tokenization/gemini_small", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(text)))
    });
}

fn benchmark_models(c: &mut Criterion) {
    let mut group = c.benchmark_group("models");
    let text = "Hello world! This is a test.";

    let models = vec![
        "gpt-3.5-turbo",
        "gpt-4",
        "gpt-4-turbo",
        "gpt-4o",
        "gemini-2.5-pro",
        "gemini-2.5-flash",
        "gemini-3-pro-preview",
    ];

    for model in models {
        let registry = ModelRegistry::global();
        let tokenizer = registry.get_tokenizer(model, false).unwrap();

        group.bench_with_input(BenchmarkId::from_parameter(model), &text, |b, &text| {
            b.iter(|| tokenizer.count_tokens(black_box(text)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_small_input,
    benchmark_medium_input,
    benchmark_large_input,
    benchmark_gemini_tokenization,
    benchmark_models
);
criterion_main!(benches);
