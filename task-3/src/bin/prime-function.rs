use std::io::stdin;

fn main() {
    let mut tests = String::new();
    stdin().read_line(&mut tests).unwrap();
    let tests = tests.trim().parse::<i32>().unwrap();

    let mut answers: Vec<String> = vec![];
    for _ in 0..tests {
        let mut num = String::new();
        stdin().read_line(&mut num).unwrap();
        answers.push(
            match is_prime(num.trim().parse().unwrap()) {
                true => "YES",
                _ => "NO",
            }
            .to_string(),
        )
    }
    for a in answers {
        println!("{a}")
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
