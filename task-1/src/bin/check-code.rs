use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let &[lhs_size, rhs_size] = buf
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .as_slice()
    else {
        panic!("expected 2 elements");
    };

    let mut code = String::new();
    stdin().read_line(&mut code).unwrap();
    let &[lhs, rhs] = code.split("-").collect::<Vec<&str>>().as_slice() else {
        println!("No");
        return;
    };

    if lhs.trim().len() != lhs_size || rhs.trim().len() != rhs_size {
        println!("No");
    } else {
        println!("Yes");
    }
}
