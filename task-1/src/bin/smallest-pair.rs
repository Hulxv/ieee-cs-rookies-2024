use std::{io::stdin, mem::swap, vec};

type I32Vecs = Vec<Vec<i32>>;

fn main() {
    let mut tests = String::new();

    stdin().read_line(&mut tests).unwrap();

    let tests: usize = tests.trim().parse().unwrap();

    let mut min_solutions: Vec<i32> = vec![];
    
    for _ in 0..tests {
        {
            // Useless input. But for C/C++ developers who work with stupid-arrays :D
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
        }

        min_solutions.push(
            get_solutions(get_numbers_from_input_inline())
                .into_iter()
                .min()
                .unwrap(),
        )
    }

    for m in min_solutions {
        println!("{m}")
    }
}

fn get_solutions(vec: Vec<i32>) -> Vec<i32> {
    let mut solutions: Vec<i32> = vec![];
    for ii in 1..vec.len() + 1 {
        for jj in ii + 1..vec.len() + 1 {
            let sol = vec[ii - 1] + vec[jj - 1] + (jj as i32 - ii as i32);
            solutions.push(sol)
        }
    }

    solutions
}

fn get_numbers_from_input_inline() -> Vec<i32> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}
