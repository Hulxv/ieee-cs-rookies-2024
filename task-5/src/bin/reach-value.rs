use std::io::stdin;

fn can_reach_n(current: u64, target: u64) -> bool {
    if current == target {
        return true;
    }

    if current > target {
        return false;
    }

    can_reach_n(current * 10, target) || can_reach_n(current * 20, target)
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let n: u64 = input.trim().parse().unwrap();

        if can_reach_n(1, n) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
