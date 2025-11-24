use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use markdown_ai_cite_remove::{remove_citations, CitationRemover};

fn bench_simple_inline_citations(c: &mut Criterion) {
    let input = "Recent research[1][2] shows promise[3] in this field[4].";

    let mut group = c.benchmark_group("simple_citations");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("inline_numeric", |b| {
        b.iter(|| remove_citations(black_box(input)))
    });

    group.finish();
}

fn bench_complex_document(c: &mut Criterion) {
    let input = r#"# Research Paper

This is a comprehensive study[1][2] that examines multiple factors[3].

## Introduction

Recent findings[4] suggest that the methodology[5][6] is sound.

## Methods

We used standard protocols[7] with modifications[8][9][10].

## Results

The data shows[11][12] significant improvements[13].

## References

[1]: https://example.com/paper1
[2]: https://example.com/paper2
[3]: https://example.com/paper3
[4]: https://example.com/paper4
[5]: https://example.com/paper5
[6]: https://example.com/paper6
[7]: https://example.com/paper7
[8]: https://example.com/paper8
[9]: https://example.com/paper9
[10]: https://example.com/paper10
[11]: https://example.com/paper11
[12]: https://example.com/paper12
[13]: https://example.com/paper13
"#;

    let mut group = c.benchmark_group("complex_document");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("full_document", |b| {
        b.iter(|| remove_citations(black_box(input)))
    });

    group.finish();
}

fn bench_real_world_chatgpt(c: &mut Criterion) {
    let input = include_str!("../tests/fixtures/chatgpt.md");

    let mut group = c.benchmark_group("real_world");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("chatgpt_format", |b| {
        b.iter(|| remove_citations(black_box(input)))
    });

    group.finish();
}

fn bench_real_world_perplexity(c: &mut Criterion) {
    let input = include_str!("../tests/fixtures/perplexity.md");

    let mut group = c.benchmark_group("real_world");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("perplexity_format", |b| {
        b.iter(|| remove_citations(black_box(input)))
    });

    group.finish();
}

fn bench_batch_processing(c: &mut Criterion) {
    let urls = vec![
        "Text[1] here.",
        "More[2] content[3].",
        "Research[4][5] shows.",
        "Studies[source:1] indicate.",
        "Final[6] text[7][8].",
    ];

    let total_bytes: usize = urls.iter().map(|s| s.len()).sum();

    let mut group = c.benchmark_group("batch_processing");
    group.throughput(Throughput::Bytes(total_bytes as u64));

    group.bench_function("5_documents", |b| {
        b.iter(|| {
            let remover = CitationRemover::new();
            for url in &urls {
                black_box(remover.remove(url));
            }
        })
    });

    group.finish();
}

fn bench_no_citations(c: &mut Criterion) {
    let input = "This is a long document with no citations at all. It contains multiple paragraphs and sentences, but no citation markers whatsoever.";

    let mut group = c.benchmark_group("edge_cases");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("no_citations", |b| {
        b.iter(|| remove_citations(black_box(input)))
    });

    group.finish();
}

fn bench_only_citations(c: &mut Criterion) {
    let input = "[1][2][3][4][5][6][7][8][9][10]";

    let mut group = c.benchmark_group("edge_cases");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("only_citations", |b| {
        b.iter(|| remove_citations(black_box(input)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_inline_citations,
    bench_complex_document,
    bench_real_world_chatgpt,
    bench_real_world_perplexity,
    bench_batch_processing,
    bench_no_citations,
    bench_only_citations
);
criterion_main!(benches);
