use std::{fmt, io::stdin};

struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    fn new() -> Self {
        Self(vec![])
    }
    fn push_row(&mut self, row: Vec<i32>) {
        self.0.push(row)
    }
    fn swap_rows(&mut self, x: usize, y: usize) {
        self.0.swap(x, y)
    }

    fn swap_cols(&mut self, x: usize, y: usize) {
        for row in self.0.iter_mut() {
            // Ensure both column indices are within bounds
            if x < row.len() && y < row.len() {
                row.swap(x, y);
            } else {
                panic!("failed to swap columns: column indices out of bounds.");
            }
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for (i, &value) in row.iter().enumerate() {
                write!(f, "{}{}", value, if i == row.len() - 1 { "" } else { " " })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [n, x, y, ..] = binding.as_slice() else {
        panic!("failed to parse input: {buf}");
    };
    let n: usize = n.trim().parse().unwrap();
    let x: usize = x.trim().parse::<usize>().unwrap() - 1;
    let y: usize = y.trim().parse::<usize>().unwrap() - 1;

    let mut mat = Matrix::new();
    for _ in 0..n {
        let mut row = String::new();
        stdin().read_line(&mut row).unwrap();
        mat.push_row(
            row.split_whitespace()
                .map(|n| n.trim().parse().unwrap())
                .collect(),
        );
    }
    mat.swap_rows(x, y);

    mat.swap_cols(x, y);
    println!("{mat}")
}
