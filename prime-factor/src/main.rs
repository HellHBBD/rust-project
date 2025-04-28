use myio::read_line_with_prompt;
use prime_factor::{decompose, print};

fn main() {
    let line = read_line_with_prompt("number: ");
    let n: i32 = line.parse().unwrap();
    print(decompose(n));
    println!("");
}
