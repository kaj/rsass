//! Tests auto-converted from "sass-spec/spec/libsass/image-url.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  blah: image-url(\"hello.png\", false);\
            \n  blah: image-url(\"hello.png\", true);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: image-url(\"hello.png\", false);\
        \n  blah: image-url(\"hello.png\", true);\
        \n}\
        \n"
    );
}
