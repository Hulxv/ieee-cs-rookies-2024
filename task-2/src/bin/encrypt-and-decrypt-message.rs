use std::io::stdin;

const KEY: &str = "PgEfTYaWGHjDAmxQqFLRpCJBownyUKZXkbvzIdshurMilNSVOtec#@_!=.+-*/";
const ORIGINAL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

trait Encrypt {
    fn encrypt(&self) -> Self;
}

trait Decrypt {
    fn decrypt(&self) -> Self;
}

impl Encrypt for char {
    fn encrypt(&self) -> Self {
        let char_location_at_original = ORIGINAL.find(*self).unwrap();
        return KEY.chars().nth(char_location_at_original).unwrap();
    }
}

impl Decrypt for char {
    fn decrypt(&self) -> Self {
        let char_location_at_key = KEY.find(*self).unwrap();
        return ORIGINAL.chars().nth(char_location_at_key).unwrap();
    }
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        return self.chars().map(|ch| ch.encrypt()).collect();
    }
}

impl Decrypt for String {
    fn decrypt(&self) -> Self {
        return self.chars().map(|ch| ch.decrypt()).collect();
    }
}

fn main() {
    let mut mode = String::new();
    stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().parse::<i32>().unwrap();

    let mut text = String::new();
    stdin().read_line(&mut text).unwrap();
    let text = text.trim().to_owned();

    println!(
        "{}",
        match mode {
            1 => text.encrypt(),
            _ => text.decrypt(),
        }
    )
}
