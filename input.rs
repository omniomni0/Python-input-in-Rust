use std::io::{self, Write};

pub fn input(placeholder: &str) -> String {
    print!("{placeholder}");
    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("ERROR");

    return input_text;
}
