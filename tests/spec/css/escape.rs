//! Tests auto-converted from "sass-spec/spec/css/escape.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;
    mod syntax {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // missing error
        fn too_high() {
            assert_eq!(
                runner().err(
                    "// Regression test for sass/dart-sass#1030.\
             \na {b: \\110000}\n"
                ),
                "Error: Invalid Unicode code point.\
         \n  ,\
         \n2 | a {b: \\110000}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok(
            "// Although zero is not a valid code point per spec, we pass it through because\
             \n// it can be used for browser hacks.\
             \na {b: \\0}\n"
        ),
        "a {\
         \n  b: \\0 ;\
         \n}\n"
    );
}
