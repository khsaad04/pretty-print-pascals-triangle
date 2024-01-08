use std::io::{stdin, stdout, Write};

fn main() {
    let mut prompt = String::new();
    print!("Enter the number of rows: ");
    let _ = stdout().flush();
    stdin().read_line(&mut prompt).unwrap();
    let n = prompt.trim().parse::<usize>().unwrap();

    for i in 1..n + 1 {
        pascal_triangle(i);
        println!();
    }
}

fn pascal_triangle(n: usize) {
    let mut space = (2_usize.pow(n as u32) / n * 2 / 3).to_string().len();
    if n == 6 {
        space = 2;
    }
    for i in 1..n + 1 {
        print!("{}1", " ".repeat(space * (n - i)));
        let mut res = 1;
        for j in 1..i {
            res *= i - j;
            res /= j;
            print!("{}{}", " ".repeat(space * 2 - res.to_string().len()), res);
        }
        println!();
    }
}
