use std::io;

fn cantor(line: &mut Vec<char>, left: usize, right: usize, n: usize) {
    if n == 0 {
        return;
    } else {
        let third = (right - left) / 3;
        for i in (left + third)..(left + 2 * third) {
            line[i] = ' ';
        }
        cantor(line, left, left + third, n - 1);
        cantor(line, left + 2 * third, right, n - 1);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let length = 3usize.pow(n as u32);

    for i in 0..n {
        let mut line = vec!['-'; length];
        cantor(&mut line, 0, length, i);
        let line_string: String = line.into_iter().collect();
        println!("{}", line_string);
    }
}