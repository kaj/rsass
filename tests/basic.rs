extern crate rsass;
use rsass::compile_scss;

#[test]
fn t00_empty() {
    check(b"\n",
          b"")
}
#[test]
fn txx_empty_rule() {
    check(b"foo{}",
          b"")
}

#[test]
fn t01_simple_css() {
    check(b"a {\n  \
            color: blue;\n\
            }",
          b"a {\n  \
            color: blue;\n\
            }\n")
          
}

#[test]
fn t02_simple_nesting() {
    check(b"div {\n  \
            img {\n    \
            border: 0px;\n  \
            }\n\
            }",
          b"div img {\n  \
            border: 0px;\n\
            }\n")
}

fn check(input: &[u8], expected: &[u8]) {
    use std::str::from_utf8;
    let result = compile_scss(input);
    if let Ok(output) = result {
        if let (Ok(output), Ok(expected)) = (from_utf8(&output), from_utf8(expected)) {
            assert_eq!(output, expected)
        } else {
            assert_eq!(output, expected)
        }
    } else {
        assert_eq!(result, Ok(expected.into()))
    }
}
