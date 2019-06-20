use std::collections::HashMap;
use std::env::args;
use std::process;

const BAD_INPUT_ERROR: &str = "space sparated numbers please";

fn main() {
    let numbers = to_vec();
    if numbers.is_empty() {
        die(BAD_INPUT_ERROR);
    }
    println!("mean {}", mean(&numbers));
    println!("median {}", median(&numbers));
    println!("mode {}", mode(&numbers));
}

fn to_vec() -> Vec<i32> {
    let mut numbers = Vec::new();
    for arg in args().skip(1) {
        match arg.parse() {
            Ok(n) => numbers.push(n),
            Err(_) => die(BAD_INPUT_ERROR),
        }
    }
    numbers
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();
    sum as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<i32>) -> f32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    let is_even_len = numbers.len() % 2 == 0;
    let middle = numbers.len() / 2;
    if is_even_len {
        let (left, right) = (sorted[middle - 1], sorted[middle]);
        (left + right) as f32 / 2.0
    } else {
        sorted[middle] as f32
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for n in numbers {
        let count = counts.entry(*n).or_insert(0);
        *count += 1;
    }
    let max = counts.iter().max_by(|(_, a), (_, b)| a.cmp(b));
    *max.unwrap_or((&i32::max_value(), &-1)).0
}

fn die(msg: &str) -> ! {
    println!("{}", msg);
    process::exit(1)
}