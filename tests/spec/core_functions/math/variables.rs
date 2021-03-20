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

        // Ignoring "e", error tests are not supported yet.

        // Ignoring "pi", error tests are not supported yet.
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
