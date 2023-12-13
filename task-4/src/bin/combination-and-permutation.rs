use std::io::stdin;

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        (2..=n).product()
    }
}

fn ncr(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn npr(n: u64, r: u64) -> u64 {
    factorial(n) / factorial(n - r)
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [a, b, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };
    let a: u64 = a.trim().parse().unwrap();
    let b: u64 = b.trim().parse().unwrap();

    println!("{} {}", ncr(a, b), npr(a, b));
}
