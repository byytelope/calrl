use engine::Engine;
use scanner::Scanner;

mod engine;
mod scanner;
mod types;

fn main() {
    let input = "";
    let mut sc = Scanner::new(input);
    let tokens = sc.lex();

    let en = Engine::new(tokens);
    let res = en.eval().unwrap();
    println!("RES: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_integers() {
        let input = "(16/4)/2-1*8+12";
        let mut sc = Scanner::new(input);
        let tokens = sc.lex();

        let en = Engine::new(tokens);
        let res = en.eval().unwrap();
        assert_eq!(res, 6.0);
    }

    #[test]
    fn verify_float() {
        let input = "(16.25/4.5)/2-1*8.75+12";
        let mut sc = Scanner::new(input);
        let tokens = sc.lex();

        let en = Engine::new(tokens);
        let res = en.eval().unwrap();
        assert_eq!(res, 5.0555);
    }
}
