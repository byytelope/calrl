use scanner::Scanner;

mod errors;
mod scanner;

fn main() {
    let input = "2+24/3";
    let mut sc = Scanner::new(input);

    let tokens = sc.lex();
    println!("{tokens:#?}");
}
