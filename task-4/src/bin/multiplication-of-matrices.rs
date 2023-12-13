use std::{
    fmt,
    io::{self, BufRead},
};

struct Matrix {
    rows: usize,
    cols: usize,
    val: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(rows: usize, cols: usize, val: Vec<Vec<i32>>) -> Self {
        Matrix { rows, cols, val }
    }

    fn read_from_input<I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let (rows, cols) = get_dimensions_from_user_input(lines);
        let val = (0..rows)
            .map(|_| {
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();

        Matrix::new(rows, cols, val)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.val {
            write!(
                f,
                "{}\n",
                row.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }
        Ok(())
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let result_val = (0..self.rows)
            .map(|i| {
                (0..rhs.cols)
                    .map(|j| (0..self.cols).map(|k| self.val[i][k] * rhs.val[k][j]).sum())
                    .collect()
            })
            .collect();

        Matrix::new(self.rows, rhs.cols, result_val)
    }
}

fn get_dimensions_from_user_input<I>(lines: &mut I) -> (usize, usize)
where
    I: Iterator<Item = String>,
{
    let line = lines.next().unwrap();
    let mut iter = line.split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();
    (rows, cols)
}

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(|x| x.expect("Read error"));

    let a = Matrix::read_from_input(&mut lines);
    let b = Matrix::read_from_input(&mut lines);

    println!("{}", a * b);
}
