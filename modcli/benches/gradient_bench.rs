use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modcli::output::{gradient, BLUE, GREEN, ORANGE, RED, YELLOW};

fn bench_gradient(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog";

    c.bench_function("gradient_two_color", |b| {
        b.iter(|| {
            let s = gradient::two_color(black_box(text), RED, ORANGE);
            black_box(s);
        })
    });

    c.bench_function("gradient_three_color", |b| {
        b.iter(|| {
            let s = gradient::three_color(black_box(text), BLUE, GREEN, YELLOW);
            black_box(s);
        })
    });
}

criterion_group!(benches, bench_gradient);
criterion_main!(benches);
