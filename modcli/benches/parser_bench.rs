use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modcli::parser::parse_line;

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parse_simple", |b| {
        b.iter(|| {
            let _ = parse_line(black_box("hello world there"));
        })
    });

    c.bench_function("parse_quoted", |b| {
        b.iter(|| {
            let _ = parse_line(black_box("say \"hello world\" 'and universe'"));
        })
    });

    c.bench_function("parse_escaped", |b| {
        b.iter(|| {
            let _ = parse_line(black_box("run path\\ with\\ spaces \"quote\""));
        })
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
