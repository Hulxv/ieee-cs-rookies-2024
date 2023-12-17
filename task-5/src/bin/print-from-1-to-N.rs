use std::io::stdin;

fn print_numbers_recursive(current: u32, n: u32) {
    if current <= n {
        println!("{}", current);
        print_numbers_recursive(current + 1, n);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    print_numbers_recursive(1, n);
}
