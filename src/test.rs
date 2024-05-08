use std::io;

fn main() {
    let mut test = String::new();
    io::stdin().read_line(&mut test).expect("wrong");
    print!("{}", test.chars().all(char::is_alphanumeric));
}