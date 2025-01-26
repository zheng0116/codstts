use codstts::core::ProjectAnalyzer;
use criterion::{criterion_group, criterion_main, Criterion};

fn analyze_benchmark(c: &mut Criterion) {
    let mut analyzer = ProjectAnalyzer::new();
    c.bench_function("analyze small project", |b| {
        b.iter(|| analyzer.analyze_project("."))
    });
}

criterion_group!(benches, analyze_benchmark);
criterion_main!(benches);
