use std::io::stdin;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn count_primes(vec: Vec<i32>) -> usize {
    vec.iter().cloned().filter(|&x| is_prime(x)).count()
}

fn is_palindrome(n: i32) -> bool {
    let digits: Vec<_> = n.to_string().chars().collect();
    let reversed: String = digits.iter().rev().collect();
    digits.iter().collect::<String>() == reversed
}

fn count_palindromes(vec: Vec<i32>) -> usize {
    vec.iter().cloned().filter(|&x| is_palindrome(x)).count()
}

fn count_divisors(n: i32) -> usize {
    (1..=n).filter(|&i| n % i == 0).count()
}

fn max_divisors_number(vec: Vec<i32>) -> Option<i32> {
    let mut max_divisors_count = 0;
    let mut result = None;

    for num in vec {
        let divisors_count = count_divisors(num);
        if divisors_count > max_divisors_count {
            max_divisors_count = divisors_count;
            result = Some(num);
        } else if divisors_count == max_divisors_count {
            result = result.map(|r| r.max(num));
        }
    }

    result
}

fn main() {
    {
        // Useless input. But for C/C++ developers who work with stupid-arrays :D
        let mut size = String::new();
        stdin().read_line(&mut size).unwrap();
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let vec: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if let Some(max_value) = vec.iter().max() {
        println!("The maximum number : {}", max_value);
    }

    if let Some(min_value) = vec.iter().min() {
        println!("The minimum number : {}", min_value);
    }

    println!(
        "The number of prime numbers : {}",
        count_primes(vec.clone())
    );

    println!(
        "The number of palindrome numbers : {}",
        count_palindromes(vec.clone())
    );

    if let Some(max_divisors_num) = max_divisors_number(vec) {
        println!(
            "The number that has the maximum number of divisors : {}",
            max_divisors_num
        );
    }
}
