//! Tests auto-converted from "sass-spec/spec/libsass/at-root/140_test_at_root_in_unknown_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@fblthp {\
            \n  .foo {\
            \n    @at-root .bar {a: b}\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@fblthp {\
        \n  .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
