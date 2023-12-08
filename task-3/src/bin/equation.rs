use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [x, n, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };
    let x: i32 = x.trim().parse().unwrap();
    let n: u32 = n.trim().parse().unwrap();

    println!("{}", equation(x, n))
}

fn equation(x: i32, n: u32) -> i32 {
    let mut res = 0;
    for i in 2..n + 1 {
        if i % 2 == 0 {
            res += x.pow(i);
        }
    }
    res
}
