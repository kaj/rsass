//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2349.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$path1: assets/images; // no errors thrown\r\
            \n$path2: /images;       // errors thrown\r\
            \n.test {\r\
            \n  background: url(#{$path1}/image.png);\r\
            \n  background: url(#{$path2}/image.png);\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  background: url(assets/images/image.png);\
        \n  background: url(/images/image.png);\
        \n}\
        \n"
    );
}
