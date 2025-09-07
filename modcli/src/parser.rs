pub fn parse_args(args: &[String]) -> (String, Vec<String>) {
    if args.is_empty() {
        return ("".to_string(), vec![]);
    }
    let cmd = args[0].clone();
    let rest = args[1..].to_vec();
    (cmd, rest)
}
