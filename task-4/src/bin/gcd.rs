use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [a, b, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };

    let a: u64 = a.trim().parse().unwrap();
    let b: u64 = b.trim().parse().unwrap();

    println!("{} {}", gcd(a, b), lcm(a, b));
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
