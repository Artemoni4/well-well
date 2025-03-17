use std::collections::HashMap;
use std::io;

fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut sock_counts: HashMap<i32, i32> = HashMap::new();
    let mut pairs = 0;

    for &color in ar {
        let count = sock_counts.entry(color).or_insert(0);
        *count += 1;
    }

    for &count in sock_counts.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).expect("Failed to read line");
    let n: i32 = n_input.trim().parse().expect("Invalid input");

    let mut ar_input = String::new();
    io::stdin().read_line(&mut ar_input).expect("Failed to read line");
    let ar: Vec<i32> = ar_input
        .trim()
        .split(' ')
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let result = sock_merchant(n, &ar);
    println!("{}", result);
}
