use std::io;

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn get_permutation(n: usize, mut k: usize) -> String {
    let mut numbers: Vec<usize> = (1..=n).collect();
    k -= 1;
    let mut result = String::new();

    for i in (1..=n).rev() {
        let fact = factorial(i - 1);
        let index = k / fact;

        result.push_str(&numbers[index].to_string());
        numbers.remove(index);
        k %= fact;
    }

    result
}

fn main() {
    println!("Введите n и k по ограничению 1 <= n <= 9 и 1 <= k <= n!: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let parts: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Введите числа"))
        .collect();

    if parts.len() != 2 {
        println!("Введите ровно два числа n и k.");
        return;
    }

    let n = parts[0];
    let k = parts[1];

    if (n <= 0 || n > 9) || (k <= 0 || k > factorial(n)){ 
        return;
    }
    
    let permutation = get_permutation(n, k);
    println!("{}", permutation);
}
