//! Tests auto-converted from "sass-spec/spec/libsass/image-url.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  blah: image-url(\"hello.png\", false);\
             \n  blah: image-url(\"hello.png\", true);\
             \n}"),
        "div {\
         \n  blah: image-url(\"hello.png\", false);\
         \n  blah: image-url(\"hello.png\", true);\
         \n}\n"
    );
}
