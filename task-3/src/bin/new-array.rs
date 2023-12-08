use std::io::stdin;
fn main() {
    {
        // Useless input. But for C/C++ developers who work with stupid-arrays :D
        let mut size = String::new();
        stdin().read_line(&mut size).unwrap();
    }

    let mut a = String::new();
    let mut b = String::new();

    stdin().read_line(&mut a).unwrap();
    stdin().read_line(&mut b).unwrap();

    let mut a = str_to_vec(a);
    let mut b = str_to_vec(b);

    b.append(&mut a);
    print_vec(b);
}

fn str_to_vec(str: String) -> Vec<i32> {
    str.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn print_vec(vec: Vec<i32>) {
    for (i, n) in vec.iter().enumerate() {
        print!("{n}{}", if i == vec.len() - 1 { "\n" } else { " " })
    }
}
