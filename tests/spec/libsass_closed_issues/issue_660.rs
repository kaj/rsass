//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_660.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: true;\
            \n\
            \ndiv {\
            \n  blah: $foo;\
            \n}\
            \n\
            \ndiv {\
            \n  blah: not $foo;\
            \n}\
            \n\
            \ndiv {\
            \n  blah: not ($foo);\
            \n}\
            \n\
            \ndiv {\
            \n  blah: not (true);\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "div {\
        \n  blah: true;\
        \n}\
        \ndiv {\
        \n  blah: false;\
        \n}\
        \ndiv {\
        \n  blah: false;\
        \n}\
        \ndiv {\
        \n  blah: false;\
        \n}\
        \n"
    );
}
