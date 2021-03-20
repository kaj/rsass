//! Tests auto-converted from "sass-spec/spec/css/escape.hrx"

mod error {
    mod syntax {

        // Ignoring "too_high", error tests are not supported yet.
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
