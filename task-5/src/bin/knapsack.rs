use std::io::stdin;

fn knapsack(items: &[(u64, u64)], n: usize, w: u64) -> u64 {
    if n == 0 || w == 0 {
        return 0;
    }

    let (weight, value) = items[n - 1];

    if weight > w {
        return knapsack(items, n - 1, w);
    } else {
        let include_item = value + knapsack(items, n - 1, w - weight);
        let exclude_item = knapsack(items, n - 1, w);
        return include_item.max(exclude_item);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let w: u64 = iter.next().unwrap().parse().unwrap();

    let mut items = Vec::new();

    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let weight: u64 = iter.next().unwrap().parse().unwrap();
        let value: u64 = iter.next().unwrap().parse().unwrap();
        items.push((weight, value));
    }

    let result = knapsack(&items, n, w);
    println!("{}", result);
}
