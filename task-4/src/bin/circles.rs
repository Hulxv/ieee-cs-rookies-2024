use std::io;

fn main() {
    let (x1, y1, x2, y2) = read_coordinates();
    let (x3, y3, x4, y4) = read_coordinates();

    let center_a = ((x1 + x2) as f64 / 2.0, (y1 + y2) as f64 / 2.0);
    let center_b = ((x3 + x4) as f64 / 2.0, (y3 + y4) as f64 / 2.0);

    let radius_a = ((x2 - x1).pow(2) as f64 + (y2 - y1).pow(2) as f64).sqrt() / 2.0;
    let radius_b = ((x4 - x3).pow(2) as f64 + (y4 - y3).pow(2) as f64).sqrt() / 2.0;

    let distance = ((center_b.0 - center_a.0).powf(2.0) + (center_b.1 - center_a.1).powf(2.0)).sqrt();

    if distance <= radius_a + radius_b {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn read_coordinates() -> (i32, i32, i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut coordinates = input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap());
    (coordinates.next().unwrap(), coordinates.next().unwrap(), coordinates.next().unwrap(), coordinates.next().unwrap())
}
