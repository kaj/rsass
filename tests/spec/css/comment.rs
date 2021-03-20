//! Tests auto-converted from "sass-spec/spec/css/comment.hrx"

mod converts_newlines {
    mod sass {}
    mod scss {
        #[test]
        fn cr() {
            assert_eq!(
                crate::rsass(
                    "/* foo\r * bar */\
            \n"
                )
                .unwrap(),
                "/* foo\
        \n * bar */\
        \n"
            );
        }
        #[test]
        fn ff() {
            assert_eq!(
                crate::rsass(
                    "/* foo\u{c} * bar */\
            \n"
                )
                .unwrap(),
                "/* foo\
        \n * bar */\
        \n"
            );
        }
    }
}
mod error {
    mod loud {
        mod multi_line {}
        mod unterminated {

            // Ignoring "scss", error tests are not supported yet.
        }
    }
}
mod inline {
    mod loud {
        #[test]
        fn scss() {
            assert_eq!(
                crate::rsass(
                    "a {\
            \n  b: c /* d */ e;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c e;\
        \n}\
        \n"
            );
        }
    }
    mod silent {
        #[test]
        fn scss() {
            assert_eq!(
                crate::rsass(
                    "a {\
            \n  b: c // d\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: c;\
        \n}\
        \n"
            );
        }
    }
}
#[test]
fn multiple() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  /* Foo Bar */\
            \n  /* Baz Bang */ }\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  /* Foo Bar */\
        \n  /* Baz Bang */\
        \n}\
        \n"
    );
}
#[test]
fn multiple_stars() {
    assert_eq!(
        crate::rsass(
            "a /***/ b {x: y}\
            \na /****/ b {x: y}\
            \na /* **/ b {x: y}\
            \na /** */ b {x: y}\
            \n"
        )
        .unwrap(),
        "a b {\
        \n  x: y;\
        \n}\
        \na b {\
        \n  x: y;\
        \n}\
        \na b {\
        \n  x: y;\
        \n}\
        \na b {\
        \n  x: y;\
        \n}\
        \n"
    );
}
#[test]
fn weird_indentation() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n    /* Foo\
            \n Bar\
            \nBaz */\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  /* Foo\
        \n   Bar\
        \n  Baz */\
        \n  a: b;\
        \n}\
        \n"
    );
}
