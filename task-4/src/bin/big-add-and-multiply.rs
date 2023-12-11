use std::{fmt, io::stdin};

struct BigInt {
    value: Vec<u64>,
}

impl BigInt {
    fn new(value: String) -> Self {
        Self {
            value: value
                .chars()
                .rev()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>(),
        }
    }

    fn add(&self, value: u64) -> Self {
        let mut result = self.value.clone();
        let mut carry = value;

        for i in 0..result.len() {
            result[i] += carry;
            carry = result[i] / 10;
            result[i] %= 10;
        }

        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }

        Self { value: result }
    }

    fn multiply(&self, value: u64) -> Self {
        let mut result = Vec::new();
        let mut carry = 0;

        for &digit in self.value.iter() {
            let product = digit * value + carry;
            result.push(product % 10);
            carry = product / 10;
        }

        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }

        BigInt { value: result }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.value
                .iter()
                .rev()
                .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
                .collect::<String>()
        )
    }
}

fn main() {
    let mut num = String::new();
    stdin().read_line(&mut num).unwrap();
    let num = num.trim().to_string();
    
    println!("{}", BigInt::new(num.clone()).add(9999));
    println!("{}", BigInt::new(num).multiply(9999));
}
