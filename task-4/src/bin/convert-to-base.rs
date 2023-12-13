use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let t: i32 = buf.trim().parse().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [n, x, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };

    let n: u64 = n.trim().parse().unwrap();
    let x: u64 = x.trim().parse().unwrap();
    match t {
        1 => println!("{}", base_to_decimal(n, x)),
        2 => println!("{}", decimal_to_base(n, x)),
        _ => (),
    }
}

fn base_to_decimal(n: u64, x: u64) -> u64 {
    u64::from_str_radix(&n.to_string(), x as u32).unwrap()
}

fn decimal_to_base(mut n: u64, x: u64) -> String {
    let mut result = String::new();

    while n > 0 {
        let digit = n % x;
        let digit_char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'A' + (digit - 10) as u8) as char
        };
        result.insert(0, digit_char);
        n /= x;
    }

    result
}
