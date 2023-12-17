use std::io::stdin;

fn print_digits_recursive(n: u32, divisor: u32) {
    if divisor > 0 {
        let digit = (n / divisor) % 10;
        print!("{} ", digit);
        print_digits_recursive(n, divisor / 10);
    }
}

fn print_digits(n: u32) {
    let mut divisor = 1;
    while n / divisor >= 10 {
        divisor *= 10;
    }
    print_digits_recursive(n, divisor);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t: u32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let n: u32 = input.trim().parse().unwrap();

        print_digits(n);
        println!();
    }
}
