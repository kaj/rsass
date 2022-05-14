//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/hyphen-interpolated.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hyphen-interpolated")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  foo: -hux-#{2+3};\
             \n  bar: hux-#{2+3};\
             \n}"),
        "div {\
         \n  foo: -hux-5;\
         \n  bar: hux-5;\
         \n}\n"
    );
}
