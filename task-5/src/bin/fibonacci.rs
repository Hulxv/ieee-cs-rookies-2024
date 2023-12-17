use std::io::stdin;

fn fibonacci(n: u64) -> u64 {
    match n {
        1 => 0,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: u64 = buf.trim().parse().unwrap();

    println!("{}", fibonacci(n));
}
