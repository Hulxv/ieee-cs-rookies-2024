use std::io::stdin;

fn main() {
    let mut word1 = String::new();
    let mut word2 = String::new();

    stdin().read_line(&mut word1).unwrap();
    stdin().read_line(&mut word2).unwrap();
    let word1 = word1.trim();
    let word2 = word2.trim();

    println!("{} {}", word1.len(), word2.len());
    println!("{} {}", word1.trim(), word2.trim());
}
