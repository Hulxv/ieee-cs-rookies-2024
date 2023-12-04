use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    println!(
        "{}",
        input
            .split_whitespace()
            .filter(|word| word
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .len()
                >= 1)
            .collect::<Vec<&str>>()
            .len()
    )
}
