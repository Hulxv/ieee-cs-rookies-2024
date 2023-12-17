// This sequence called Collatz conjecture
// I build a simple code to generate this sequence using C++ for fun (not really something important) before about 2 years :D
// Check:  https://github.com/Hulxv/Collatz-Conjecture

use std::io::stdin;

fn sequence_length(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    if n % 2 == 0 {
        return 1 + sequence_length(n / 2);
    } else {
        return 1 + sequence_length(3 * n + 1);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let length = sequence_length(n);
    println!("{}", length);
}
