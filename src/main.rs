use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    let a = BigInt::from_str("16777216").unwrap(); // 2^23
    let b = BigInt::from_str("2199023255552").unwrap(); // 2^40

    //  сложения
    let sum = &a + &b;
    println!("{} + {} = {}", a, b, sum);

    // вычитания
    let diff = &a - &b;
    println!("{} - {} = {}", a, b, diff);

    // умножения
    let multiply = &a * &b;
    println!("{} * {} = {}", a, b, multiply);

    // деления, знаю точно что a != 0
    let quotient = &b / &a;
    println!("{} / {} = {}", a, b, quotient);
}
