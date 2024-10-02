use engine::Engine;
use scanner::Scanner;

mod engine;
mod scanner;
mod types;

fn main() {
    let input = "2+24/3";
    let mut sc = Scanner::new(input);
    let tokens = sc.lex();

    let en = Engine::new(tokens);
    let res = en.eval().unwrap();
    println!("Result: {res}");
}
