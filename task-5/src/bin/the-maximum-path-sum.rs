use std::io::stdin;

type Matrix = Vec<Vec<i64>>;

fn max_sum(matrix: Matrix, buf: &mut i64, (x, y): (usize, usize), n: usize, m: usize) -> i64 {
    *buf += matrix[x][y];

    let mut down_max_sum = 0;
    let mut right_max_sum = 0;

    if y < n - 1 {
        max_sum(matrix.clone(), &mut down_max_sum, (x, y + 1), n, m);
    }

    if x < m - 1 {
        max_sum(matrix, &mut right_max_sum, (x + 1, y), n, m);
    }

    *buf += [down_max_sum, right_max_sum].iter().max().unwrap();

    return *buf;
}

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let binding = buf.split_whitespace().collect::<Vec<&str>>();
    let [n, m, ..] = binding.as_slice() else {
        panic!("Cannot parse input: {buf}");
    };
    let n: usize = n.parse().unwrap();
    let m: usize = m.parse().unwrap();

    let mut matrix = Vec::with_capacity(n);

    for _ in 0..n {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let row: Vec<i64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let mut res = 0;
    max_sum(matrix, &mut res, (0, 0), n, m);
    println!("{res}");
}
