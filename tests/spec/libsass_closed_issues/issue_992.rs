//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_992.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$color: \'red\';\
            \n\
            \n.-text-#{$color}- {\
            \n  color: $color;\
            \n}"
        )
        .unwrap(),
        ".-text-red- {\
        \n  color: \"red\";\
        \n}\
        \n"
    );
}
