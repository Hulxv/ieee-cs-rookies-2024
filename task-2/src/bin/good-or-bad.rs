use std::{io::stdin, vec};

fn main() {
    let mut tests = String::new();
    stdin().read_line(&mut tests).unwrap();
    let tests = tests.trim().parse::<i32>().unwrap();

    let mut inputs: Vec<String> = vec![];

    for _ in 0..tests {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        inputs.push(buf.trim().to_string());
    }

    for input in inputs {
        let substrings = get_substrings(input, 3);
        println!(
            "{}",
            if substrings.contains(&"010".to_string()) || substrings.contains(&"101".to_string()) {
                "Good"
            } else {
                "Bad"
            }
        )
    }
}

fn get_substrings(s: String, len: usize) -> Vec<String> {
    let mut container: Vec<String> = vec![];

    let mut idx = 0;
    while idx < s.len() - len + 1 {
        container.push(s[idx..idx + len].to_string());
        idx += 1;
    }

    container
}
