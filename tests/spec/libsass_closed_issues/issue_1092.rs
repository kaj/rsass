//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1092.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$bar: \"\";\
            \n$baz: \" \";\
            \na { a: foo #{\"\"}; }\
            \nb { b: foo #{\" \"}; }\
            \nc { c: foo #{$bar}; }\
            \nd { d: foo #{$baz}; }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  a: foo;\
        \n}\
        \nb {\
        \n  b: foo  ;\
        \n}\
        \nc {\
        \n  c: foo;\
        \n}\
        \nd {\
        \n  d: foo  ;\
        \n}\
        \n"
    );
}
