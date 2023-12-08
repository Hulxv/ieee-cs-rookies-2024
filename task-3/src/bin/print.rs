use std::io::stdin;

fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    print_nums(n.trim().parse().unwrap());
}

fn print_nums(n: i32) {
    for i in 1..n+1 {
        print!("{i}{}", if i == n { "" } else { " " })
    }
}
