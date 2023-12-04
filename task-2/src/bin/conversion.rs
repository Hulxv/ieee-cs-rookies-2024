use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf = buf
        .chars()
        .map(|ch| match ch {
            ',' => return " ".to_owned(),

            e if e.is_alphabetic() => match e.is_lowercase() {
                true => return e.to_uppercase().to_string(),
                false => return e.to_lowercase().to_string(),
            },

            e => e.to_string(),
        })
        .collect();

    println!("{buf}");
}
