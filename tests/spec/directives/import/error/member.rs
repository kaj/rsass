//! Tests auto-converted from "sass-spec/spec/directives/import/error/member.hrx"

mod inaccessible {
    mod nested {
        #[test]
        #[ignore] // unexepected error
        fn function() {
            assert_eq!(
                crate::rsass(
                    "a {@import \"other\"}\
            \n\
            \nb {c: d()}\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: d();\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
                crate::rsass(
                    "a {@import \"other\"}\
             \n\
             \nb {@include c}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | b {@include c}\
         \n  |    ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:4  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            assert_eq!(
                crate::rsass(
                    "a {@import \"other\"}\
             \n\
             \nb {c: $d}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | b {c: $d}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
