//! Tests auto-converted from "sass-spec/spec/css/escape.hrx"

mod error {
    mod syntax {
        #[test]
        #[ignore] // missing error
        fn too_high() {
            assert_eq!(
                crate::rsass(
                    "// Regression test for sass/dart-sass#1030.\
             \na {b: \\110000}\
             \n"
                )
                .unwrap_err(),
                "Error: Invalid Unicode code point.\
         \n  ,\
         \n2 | a {b: \\110000}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "// Although zero is not a valid code point per spec, we pass it through because\
            \n// it can be used for browser hacks.\
            \na {b: \\0}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \\0 ;\
        \n}\
        \n"
    );
}
