use std::io::stdin;

fn main() {
    {
        // Useless input. But for C/C++ developers who work with stupid-arrays :D
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
    }

    let vec = get_numbers_from_input_inline();

    println!("{}", max_operations(vec))
}

fn get_numbers_from_input_inline() -> Vec<i32> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn is_all_even(vec: Vec<i32>) -> bool {
    vec.clone()
        .into_iter()
        .filter(|e| e % 2 == 0)
        .collect::<Vec<i32>>()
        .len()
        == vec.len()
}

fn max_operations(vec: Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut vec = vec;
    while is_all_even(vec.clone()) {
        vec = vec.iter().map(|e| e / 2).collect();
        counter += 1;
    }
    counter
}
