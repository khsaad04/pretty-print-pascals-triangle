use std::io::{self, stdin, stdout, Write};

fn main() -> io::Result<()> {
    let mut prompt = String::new();

    print!("Enter the number of rows: ");
    let _ = stdout().flush();
    stdin().read_line(&mut prompt)?;

    let n = prompt.trim().parse::<u128>().unwrap();
    pascal_triangle(n);
    Ok(())
}

fn pascal_triangle(n: u128) {
    let space = (2_u128.pow(n as u32)).to_string().len();
    for i in 1..n + 1 {
        print!("{}1", " ".repeat(space * (n - i) as usize));
        let mut res = 1;
        for j in 1..i {
            res *= i - j;
            res /= j;
            print!("{}{}", " ".repeat(space * 2 - res.to_string().len()), res);
        }
        println!();
    }
}
