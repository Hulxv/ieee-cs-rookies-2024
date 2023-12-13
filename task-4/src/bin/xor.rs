use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [a, b, q, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();
    let q: i32 = q.trim().parse().unwrap();

    println!("{}", func(a, b, q))
}

fn func(a: i32, b: i32, q: i32) -> i32 {
    match q {
        1 => a,
        2 => b,
        _ => func(a, b, q - 1) ^ func(a, b, q - 2),
    }
}
