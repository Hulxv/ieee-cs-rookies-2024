use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    println!(
        "{}",
        match get_subsequences(&s).contains(&"hello".to_string()) {
            true => "YES",
            false => "NO",
        }
    )
}

fn get_subsequences(s: &str) -> Vec<String> {
    let mut subsequences = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    for i in 0..(1 << n) {
        let mut subsequence = String::new();
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                subsequence.push(chars[j]);
            }
        }
        subsequences.push(subsequence);
    }

    subsequences
}
