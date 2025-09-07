use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modcli::output::table::{render_table, TableMode, TableStyle};

fn bench_table(c: &mut Criterion) {
    let headers = ["Col1", "Col2", "Col3", "Col4"]; 
    let rows: Vec<Vec<&str>> = (0..100)
        .map(|i| vec!["row", "with", "some", Box::leak(format!("data-{i}").into_boxed_str())])
        .collect();

    c.bench_function("table_flex_heavy", |b| {
        b.iter(|| {
            render_table(&headers, black_box(&rows), TableMode::Flex, TableStyle::Heavy);
        })
    });

    c.bench_function("table_fixed_rounded", |b| {
        b.iter(|| {
            render_table(&headers, black_box(&rows), TableMode::Fixed(16), TableStyle::Rounded);
        })
    });
}

criterion_group!(benches, bench_table);
criterion_main!(benches);
