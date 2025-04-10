/// Basic output formatting tools.
pub fn print_success(msg: &str) {
    println!("✅ {}", msg);
}

pub fn print_error(msg: &str) {
    eprintln!("❌ {}", msg);
}

pub fn print_info(msg: &str) {
    println!("ℹ️  {}", msg);
}