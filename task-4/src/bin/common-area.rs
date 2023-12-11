use std::cmp;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let t: usize = read_line().trim().parse().unwrap();

    for case_number in 1..=t {
        let n: usize = read_line().trim().parse().unwrap();

        let mut max_lower_left = Point { x: i32::MIN, y: i32::MIN };
        let mut min_upper_right = Point { x: i32::MAX, y: i32::MAX };

        for _ in 0..n {
            let (x1, y1, x2, y2) = read_coordinates();

            max_lower_left.x = cmp::max(max_lower_left.x, x1);
            max_lower_left.y = cmp::max(max_lower_left.y, y1);

            min_upper_right.x = cmp::min(min_upper_right.x, x2);
            min_upper_right.y = cmp::min(min_upper_right.y, y2);
        }

        let area = calculate_area(max_lower_left, min_upper_right);
        println!("Case #{}: {}", case_number, area);
    }
}

fn read_coordinates() -> (i32, i32, i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut coordinates = input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap());
    (coordinates.next().unwrap(), coordinates.next().unwrap(), coordinates.next().unwrap(), coordinates.next().unwrap())
}

fn calculate_area(lower_left: Point, upper_right: Point) -> i64 {
    let width = cmp::max(0, upper_right.x - lower_left.x) as i64;
    let height = cmp::max(0, upper_right.y - lower_left.y) as i64;
    width * height
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}
