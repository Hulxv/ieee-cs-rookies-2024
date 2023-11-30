use std::{fmt::Display, io::stdin};

type I32Vecs = Vec<Vec<i32>>;

fn main() {
    let mut tests = String::new();
    stdin().read_line(&mut tests).unwrap();
    let tests = tests.trim().parse::<i32>().unwrap();

    let mut arrays: I32Vecs = Vec::new();

    for _ in 0..tests {
        let mut len = String::new();
        stdin().read_line(&mut len).unwrap();
        arrays.push(get_numbers_from_input_inline());
    }

    for arr in arrays {
        println!("{}", count_non_decreasing_subarrays(arr));
    }
}

fn get_numbers_from_input_inline() -> Vec<i32> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn count_non_decreasing_subarrays(arr: Vec<i32>) -> i32 {
    let mut counter = 0;

    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i + j >= arr.len() {
                break;
            }
            let sub = (&arr[j..(i + j + 1)]).to_vec();
            if is_non_decreasing_array(sub.clone()) {
                counter += 1
            }
        }
    }

    counter
}

fn is_non_decreasing_array(arr: Vec<i32>) -> bool {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    arr == sorted_arr
}
