use modcli::output::hook;
use modcli::ModCli;

fn main() {
    let mut cli = ModCli::new();

    // Pre/post hooks
    cli.registry.set_pre_hook(|cmd, args| {
        hook::info(&format!("[pre] cmd='{cmd}' args={args:?}"));
    });
    cli.registry.set_post_hook(|cmd, args, res| {
        hook::info(&format!(
            "[post] cmd='{cmd}' args={args:?} ok={}",
            res.is_ok()
        ));
    });

    // Error formatter
    cli.registry
        .set_error_formatter(|err| format!("Whoops! {err}"));

    // Drive some examples: a good command and an unknown one
    cli.run(vec!["help".into()]);
    println!("\n-- unknown --\n");
    cli.run(vec!["does-not-exist".into()]);
}
