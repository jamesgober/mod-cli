use modcli::{banner_file, ModCli};

fn main() {
    // Register a banner from a file (read now; printed once on first run)
    banner_file!("modcli/examples/ascii/banner.txt");

    let mut cli = ModCli::new();
    cli.run(vec!["help".into()]);
}
