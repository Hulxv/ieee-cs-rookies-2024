use std::io::stdin;

trait IsSubSequence {
    fn is_subsequence_of(&self, arr: Vec<i32>) -> bool;
}

impl IsSubSequence for Vec<i32> {
    fn is_subsequence_of(&self, arr: Vec<i32>) -> bool {
        if self.len() > arr.len() {
            return false;
        }

        arr.into_iter()
            .filter(|e| self.contains(e))
            .collect::<Vec<i32>>()
            == *self
    }
}

fn main() {
    {
        // Useless input. But for C/C++ developers who work with stupid-arrays
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
    }

    let a = get_numbers_from_input_inline();
    let b = get_numbers_from_input_inline();

    println!(
        "{}",
        match b.is_subsequence_of(a) {
            true => "YES",
            false => "NO",
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
