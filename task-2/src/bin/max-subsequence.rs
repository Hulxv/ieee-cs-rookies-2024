use std::io::stdin;

type Strings = Vec<String>;

fn main() {
    {
        // Useless input. But for C/C++ developers who work with stupid-arrays :D
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
    }
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    println!(
        "{:?}",
        get_subsequences(&s)
            .iter()
            .filter(|w| !w.is_empty())
            .filter(|w| !is_there_2_adjacent_chars(w))
            .map(|w| w.len())
            .max()
            .unwrap()
    );
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

fn is_there_2_adjacent_chars(s: &str) -> bool {
    if s.len() == 1 {
        return false;
    }
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();

    let mut idx = 0;
    while idx < s.len() - 1 {
        let next = chars.next().unwrap();
        if next == prev {
            return true;
        }
        prev = next;
        idx += 1;
    }
    false
}
