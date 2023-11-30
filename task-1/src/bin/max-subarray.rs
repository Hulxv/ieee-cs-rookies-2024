use std::{fmt::Display, io::stdin};

type i32Vecs = Vec<Vec<i32>>;

fn print_inline<T: Display>(vec: Vec<T>) {
    for (i, e) in vec.iter().enumerate() {
        print!("{e}{}", if i + 1 >= vec.len() { "\n" } else { " " });
    }
}

fn main() {
    let mut tests = String::new();
    stdin().read_line(&mut tests).unwrap();
    let tests = tests.trim().parse::<i32>().unwrap();

    let mut arrays: i32Vecs = Vec::new();

    for _ in 0..tests {
        let mut len = String::new();
        stdin().read_line(&mut len).unwrap();
        arrays.push(get_numbers_from_input_inline());
    }

    for arr in arrays {
        print_inline(
            get_subarrays(arr)
                .iter()
                .map(|sub| sub.iter().max().unwrap())
                .collect(),
        );
    }
}

fn get_numbers_from_input_inline() -> Vec<i32> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn get_subarrays(arr: Vec<i32>) -> i32Vecs {
    let mut buf: i32Vecs = Vec::new();

    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i + j >= arr.len() {
                break;
            }
            buf.push((&arr[j..(i + j + 1)]).to_vec());
        }
    }

    buf
}
