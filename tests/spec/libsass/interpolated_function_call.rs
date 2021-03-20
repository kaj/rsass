//! Tests auto-converted from "sass-spec/spec/libsass/interpolated-function-call.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$f: foo;\
            \n\
            \ndiv {\
            \n  color: #{$f}(a, 1+2, c);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  color: foo(a, 3, c);\
        \n}\
        \n"
    );
}
