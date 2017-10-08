//! Tests from `spec/parser/interpolate`

use rsass::{OutputStyle, compile_scss};

mod t01_literal {
    use super::check;

    #[test]
    fn t06_escape_interpolation() {
        check(b"$input: literal;\n\
                .result {\n  output: \"[\\#{literal}]\";\n  \
                output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
                output: \"['\\#{literal}']\";\n}\n",
              ".result {\n  output: \"[\\#{literal}]\";\n  \
               output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
               output: \"['\\#{literal}']\";\n}\n")
    }
}

mod t04_space_list_quoted {
    use super::check;

    #[test]
    fn t01_inline() {
        check(b".result {\n  output: \"alpha\" 'beta';\n  \
                output: #{\"alpha\" 'beta'};\n  \
                output: \"[#{\"alpha\" 'beta'}]\";\n  \
                output: \"#{\"alpha\" 'beta'}\";\n  \
                output: '#{\"alpha\" 'beta'}';\n  \
                output: \"['#{\"alpha\" 'beta'}']\";\n}\n",
              ".result {\n  output: \"alpha\" 'beta';\n  \
               output: alpha beta;\n  output: \"[alpha beta]\";\n  \
               output: \"alpha beta\";\n  output: \"alpha beta\";\n  \
               output: \"['alpha beta']\";\n}\n")
    }
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
