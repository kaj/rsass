//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/41_slashy_urls.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  blah: url(//some/absolute/path);\
             \n  blee: url(/*looks-like-a*/comment);\
             \n}"),
        "div {\
         \n  blah: url(//some/absolute/path);\
         \n  blee: url(/*looks-like-a*/comment);\
         \n}\n"
    );
}
