use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modcli::output::build;

fn bench_output_builder(c: &mut Criterion) {
    c.bench_function("output_builder_simple", |b| {
        b.iter(|| {
            let s = build()
                .part("Hello").bold().space()
                .part("world").get();
            black_box(s);
        })
    });

    c.bench_function("output_builder_multi_styles", |b| {
        b.iter(|| {
            let s = build()
                .part("label:").bold().space()
                .part("value").italic().underline().space()
                .part("!\n").get();
            black_box(s);
        })
    });
}

criterion_group!(benches, bench_output_builder);
criterion_main!(benches);
