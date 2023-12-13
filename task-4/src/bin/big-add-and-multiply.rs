use std::{fmt, io::stdin, ops};

#[derive(Clone)]
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
}

impl ops::Add<u64> for BigInt {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        let mut result = self.value.clone();
        let mut carry = rhs;

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
}

impl ops::Mul<u64> for BigInt {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        let mut result = Vec::new();
        let mut carry = 0;

        for &digit in self.value.iter() {
            let product = digit * rhs + carry;
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
    let num = BigInt::new(num.trim().to_string());

    println!("{}", num.clone() + 9999);
    println!("{}", num * 9999);
}
