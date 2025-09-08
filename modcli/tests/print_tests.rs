use modcli::output::print;

#[test]
fn print_scroll_no_delay_completes() {
    let lines = ["a", "b", "c"];
    print::scroll(&lines, 0);
}

#[test]
fn print_scroll_with_small_delay_completes() {
    let lines = ["x", "y"];
    print::scroll(&lines, 1);
}

#[test]
#[cfg_attr(miri, ignore)]
fn print_file_missing_path_does_not_panic() {
    // This should log an error via print::error but must not panic
    print::file("/this/path/does/not/exist___", 0);
}
