//! Tests auto-converted from "sass-spec/spec/non_conformant/variables.hrx"

mod global {
    mod first_declaration {
        #[test]
        fn nested() {
            assert_eq!(
                crate::rsass(
                    "x {$var: value !global}\
            \na {b: $var}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: value;\
        \n}\
        \n"
            );
        }
        #[test]
        fn top_level() {
            assert_eq!(
                crate::rsass(
                    "$var: value !global;\
            \na {b: $var}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: value;\
        \n}\
        \n"
            );
        }
    }
}
