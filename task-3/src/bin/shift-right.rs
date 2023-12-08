use std::{fmt, io::stdin};

trait ShiftRight {
    fn shift_right(&self, x: i32) -> Self;
}

impl ShiftRight for Vec<i32> {
    fn shift_right(&self, x: i32) -> Self {
        let x = x % self.len() as i32;

        let mut result = self.clone();

        for i in 0..self.len() {
            result[i] = self[(self.len() - x as usize + i) % self.len()].clone();
        }
        result
    }
}

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [n, x, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };
    let n: usize = n.trim().parse().unwrap();
    let x: i32 = x.trim().parse().unwrap();

    buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    for (i, &value) in str_to_vec(buf).shift_right(x).iter().enumerate() {
        print!("{}{}", value, if i == n - 1 { "\n" } else { " " });
    }
}

fn str_to_vec(str: String) -> Vec<i32> {
    str.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}
