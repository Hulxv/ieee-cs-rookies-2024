use std::io::stdin;

fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut vec: Vec<i32>;

    let mut buf_input = String::new();
    stdin().read_line(&mut buf_input).unwrap();
    vec = buf_input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let max = vec.iter().max().unwrap();
    let min = vec.iter().min().unwrap();

    let max_pos = vec.iter().position(|ele| ele == max).unwrap();
    let min_pos = vec.iter().position(|ele| ele == min).unwrap();

    vec.swap(min_pos, max_pos);

    for (i, n) in vec.iter().enumerate() {
        print!("{n}{}", if i != vec.len() - 1 { " " } else { "" });
    }
}
