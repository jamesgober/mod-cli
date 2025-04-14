use rpassword::read_password;

pub fn prompt_password(message: &str) -> String {
    prompt_password_with_validation(message, |_| Ok(()))
}

pub fn prompt_password_with_validation<F>(message: &str, validator: F) -> String
where
    F: Fn(&str) -> Result<(), &str>,
{
    loop {
        print!("{}: ", message);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let password = read_password().unwrap();

        match validator(password.trim()) {
            Ok(_) => return password.trim().to_string(),
            Err(err) => {
                println!("Invalid password: {}", err);
            }
        }
    }
}
