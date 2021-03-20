//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1393.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  back#{ground}: {\
            \n    imag#{e}: url(foo.png);\
            \n    pos#{it}ion: 50%;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  background-image: url(foo.png);\
        \n  background-position: 50%;\
        \n}\
        \n"
    );
}
