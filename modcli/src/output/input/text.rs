use std::io::{stdin, stdout, Write};

pub fn prompt_text(message: &str) -> String {
    prompt_text_with_validation(message, |_| Ok(()))
}

pub fn prompt_text_with_validation<F>(message: &str, validator: F) -> String
where
    F: Fn(&str) -> Result<(), &str>,
{
    let mut input = String::new();
    loop {
        print!("{}: ", message);
        stdout().flush().unwrap();
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        match validator(trimmed) {
            Ok(_) => return trimmed.to_string(),
            Err(err) => {
                println!("Invalid input: {}", err);
            }
        }
    }
}
