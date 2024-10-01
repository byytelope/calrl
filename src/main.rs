use scanner::Scanner;

mod scanner;

fn main() {
    let input = "2+24/3=";
    let mut sc = Scanner::new(input);

    sc.eval();
}
