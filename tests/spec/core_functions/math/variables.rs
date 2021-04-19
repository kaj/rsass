//! Tests auto-converted from "sass-spec/spec/core_functions/math/variables.hrx"

#[test]
fn e() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.$e}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2.7182818285;\
        \n}\
        \n"
    );
}
mod error {
    mod assignment {
        #[test]
        #[ignore] // wrong error
        fn e() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \nmath.$e: 0;\
             \n"
                )
                .unwrap_err(),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$e: 0;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn pi() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \nmath.$pi: 0;\
             \n"
                )
                .unwrap_err(),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$pi: 0;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn pi() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.$pi}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3.1415926536;\
        \n}\
        \n"
    );
}
