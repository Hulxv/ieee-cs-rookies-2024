use std::io::stdin;

trait IsPermutationWith {
    fn is_permutation_of(&mut self, arr: &mut Vec<i32>) -> bool;
}

impl IsPermutationWith for Vec<i32> {
    fn is_permutation_of(&mut self, arr: &mut Vec<i32>) -> bool {
        self.sort();
        arr.sort();
        *arr == *self
    }
}

fn main() {
    {
        // Useless input. But for C/C++ developers who work with stupid-arrays :D
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
    }

    let mut a = get_numbers_from_input_inline();
    let mut b = get_numbers_from_input_inline();

    println!(
        "{}",
        match b.is_permutation_of(&mut a) {
            true => "yes",
            false => "no",
        }
    )
}

fn get_numbers_from_input_inline() -> Vec<i32> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}
