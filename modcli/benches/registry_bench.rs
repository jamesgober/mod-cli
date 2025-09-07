use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modcli::command::Command;
use modcli::loader::CommandRegistry;

struct Nop;
impl Command for Nop {
    fn name(&self) -> &str { "nop" }
    fn validate(&self, _args: &[String]) -> Result<(), String> { Ok(()) }
    fn execute(&self, _args: &[String]) { /* no-op */ }
}

struct NopAlias;
impl Command for NopAlias {
    fn name(&self) -> &str { "primary" }
    fn aliases(&self) -> &[&str] { &["alias", "a"] }
    fn validate(&self, _args: &[String]) -> Result<(), String> { Ok(()) }
    fn execute(&self, _args: &[String]) { /* no-op */ }
}

fn bench_registry(c: &mut Criterion) {
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Nop));

    let mut alias_reg = CommandRegistry::new();
    alias_reg.register(Box::new(NopAlias));

    let mut pref_reg = CommandRegistry::new();
    pref_reg.set_prefix("x");
    pref_reg.register(Box::new(Nop));

    c.bench_function("registry_name", |b| {
        b.iter(|| {
            reg.execute(black_box("nop"), black_box(&[]));
        })
    });

    c.bench_function("registry_alias", |b| {
        b.iter(|| {
            alias_reg.execute(black_box("alias"), black_box(&[]));
        })
    });

    c.bench_function("registry_prefixed", |b| {
        b.iter(|| {
            pref_reg.execute(black_box("x:nop"), black_box(&[]));
        })
    });
}

criterion_group!(benches, bench_registry);
criterion_main!(benches);
