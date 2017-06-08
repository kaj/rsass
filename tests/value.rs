extern crate num_rational;
extern crate num_traits;
extern crate nom;
extern crate rsass;

mod value {
    mod addition;
    mod logic_eq;
    mod binary_and_unary;
    mod unquote;

    use rsass::compile_value;
    use std::str::from_utf8;

    pub fn check_eval(expression: &str, expected: &str) {
        let data = compile_value(expression.as_bytes()).unwrap();
        let actual = from_utf8(&data).unwrap();
        assert_eq!(actual, expected)
    }
}
