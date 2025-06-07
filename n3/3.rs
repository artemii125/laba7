use std::io;

const DIGITS: &str = "123456789";
const N: usize = 8;

fn search(pos: usize, expr: String, current_value: i64, last_value: i64, m: i64) {
    if pos == N {
        let total = current_value + last_value;
        if total == m {
            println!("{}", expr);
        }
        return;
    }

    let digit = (DIGITS.as_bytes()[pos + 1] - b'0') as i64;

    search(pos + 1, format!("{}{}", expr, (digit as u8 + b'0') as char), current_value, last_value * 10 + if last_value >= 0 { digit } else { -digit }, m);
    search(pos + 1, format!("{}+{}", expr, (digit as u8 + b'0') as char), current_value + last_value, digit, m);
    search(pos + 1, format!("{}-{}", expr, (digit as u8 + b'0') as char), current_value + last_value, -digit, m);
}

fn main() {
    println!("Введите число m: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let m: i64 = input.trim().parse().unwrap();

    let first_digit = (DIGITS.as_bytes()[0] - b'0') as i64;
    search(0, DIGITS[0..1].to_string(), 0, first_digit, m);
}