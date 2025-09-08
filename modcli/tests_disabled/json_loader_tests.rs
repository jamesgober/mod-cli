#[cfg(feature = "json-loader")]
mod json_loader_integration {
    use modcli::loader::CommandRegistry;
    use modcli::loader::sources::JsonFileSource;

    #[test]
    fn loads_commands_from_json_and_resolves_aliases() {
        let mut reg = CommandRegistry::new();
        let source = JsonFileSource::new("modcli/examples/commands.json");
        reg.load_from(Box::new(source));

        // Primary command from JSON
        assert!(reg.get("json-hello").is_some(), "json-hello should be loaded from JSON");

        // Alias should resolve via registry alias map
        // We test by executing the alias; it should route to the command (no panic/unknown)
        // Since the DynamicJsonCommand prints to stdout, we simply ensure no panic here.
        reg.execute("jh", &[]);
    }
}
