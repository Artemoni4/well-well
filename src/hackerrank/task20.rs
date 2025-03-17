use std::io;

fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);
    std::cmp::min(from_front, from_back)
}

fn main() {
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).expect("Failed to read line");
    let n: i32 = n_input.trim().parse().expect("Invalid input");

    let mut p_input = String::new();
    io::stdin().read_line(&mut p_input).expect("Failed to read line");
    let p: i32 = p_input.trim().parse().expect("Invalid input");

    let result = page_count(n, p);
    println!("{}", result);
}