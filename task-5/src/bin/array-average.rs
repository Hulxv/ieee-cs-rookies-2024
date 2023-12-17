use std::io::stdin;

fn average_recursive(arr: &[i64], n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let sum = arr[n - 1] as f64 + average_recursive(arr, n - 1);
    sum
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sum = average_recursive(&arr, n) as f64;
    let average = sum / n as f64;

    println!("{:.6}", average);
}
