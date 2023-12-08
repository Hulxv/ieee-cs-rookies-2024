use std::io::stdin;

fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    if format!("{n}")
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>().unwrap()
        == n
        && n % 2 != 0
    {
        println!("YES");
    } else {
        println!("NO")
    }
}
