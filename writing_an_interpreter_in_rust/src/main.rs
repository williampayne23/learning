mod token;
mod lexer;
mod repl;


fn main() {
    repl::repl();
}

mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
