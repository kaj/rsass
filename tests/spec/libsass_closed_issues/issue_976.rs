//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_976.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".debug {\
            \n  @debug-this {\
            \n    foo: bar;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@debug-this {\
        \n  .debug {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}
