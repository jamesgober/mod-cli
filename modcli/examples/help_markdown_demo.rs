use modcli::command::Command;
use modcli::output::messages;
use modcli::ModCli;

struct Ops;
impl Command for Ops {
    fn name(&self) -> &str {
        "ops"
    }
    fn help(&self) -> Option<&str> {
        Some("# Operations\n- `ops deploy`  Deploy the app\n- `ops status`  Show system status")
    }
    fn execute(&self, _args: &[String]) {}
}

struct Deploy;
impl Command for Deploy {
    fn name(&self) -> &str {
        "ops:deploy"
    }
    fn help(&self) -> Option<&str> {
        Some("# Deploy\nDeploy the app to production.\n\n**Usage**\n- `ops deploy --env prod`\n\n*Notes*: Requires credentials.")
    }
    fn execute(&self, _args: &[String]) {
        println!("deploying...");
    }
}

struct Status;
impl Command for Status {
    fn name(&self) -> &str {
        "ops:status"
    }
    fn help(&self) -> Option<&str> {
        Some("# Status\nShow current status.\n\n- **OK** when healthy\n- *Warn* when degraded")
    }
    fn execute(&self, _args: &[String]) {
        println!("all green");
    }
}

fn main() {
    // Customize help header/footer via messages catalog
    messages::set_message(
        "help.header",
        "# Help\n- Use `help <ns>` to see namespaced commands",
    );
    messages::set_message(
        "help.footer",
        "\n*Tip*: Try `help ops` for namespaced help\n",
    );

    let mut cli = ModCli::new();
    cli.registry.register(Box::new(Ops));
    cli.registry.register(Box::new(Deploy));
    cli.registry.register(Box::new(Status));

    // Demo: print root help, then namespaced help
    cli.run(vec!["help".into()]);
    println!("\n---\n");
    cli.run(vec!["help".into(), "ops".into()]);
}
