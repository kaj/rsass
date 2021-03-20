//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/41_slashy_urls.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  blah: url(//some/absolute/path);\
            \n  blee: url(/*looks-like-a*/comment);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: url(//some/absolute/path);\
        \n  blee: url(/*looks-like-a*/comment);\
        \n}\
        \n"
    );
}
