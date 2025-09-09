use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modcli::command::Command;
use modcli::loader::CommandRegistry;
use modcli::error::ModCliError;

struct Nop;
impl Command for Nop {
    fn name(&self) -> &str {
        "nop"
    }
    fn validate(&self, _args: &[String]) -> Result<(), ModCliError> {
        Ok(())
    }
    fn execute(&self, _args: &[String]) {}
}

struct NopAlias;
impl Command for NopAlias {
    fn name(&self) -> &str {
        "primary"
    }
    fn aliases(&self) -> &[&str] {
        &["alias", "a"]
    }
    fn validate(&self, _args: &[String]) -> Result<(), ModCliError> { Ok(()) }
    fn execute(&self, _args: &[String]) {}
}

fn bench_registry_try_execute(c: &mut Criterion) {
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Nop));

    let mut alias_reg = CommandRegistry::new();
    alias_reg.register(Box::new(NopAlias));

    let mut pref_reg = CommandRegistry::new();
    pref_reg.set_prefix("x");
    pref_reg.register(Box::new(Nop));

    c.bench_function("try_execute_name", |b| {
        b.iter(|| {
            let _ = reg.try_execute(black_box("nop"), black_box(&[]));
        })
    });

    c.bench_function("try_execute_alias", |b| {
        b.iter(|| {
            let _ = alias_reg.try_execute(black_box("alias"), black_box(&[]));
        })
    });

    c.bench_function("try_execute_prefixed", |b| {
        b.iter(|| {
            let _ = pref_reg.try_execute(black_box("x:nop"), black_box(&[]));
        })
    });
}

criterion_group!(benches, bench_registry_try_execute);
criterion_main!(benches);
