//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1672.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$breakpoint: \'tablet\';\
            \n\
            \n.-#{$breakpoint} {\
            \n  color: #FFF;\
            \n}"
        )
        .unwrap(),
        ".-tablet {\
        \n  color: #FFF;\
        \n}\
        \n"
    );
}
