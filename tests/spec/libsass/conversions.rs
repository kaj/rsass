//! Tests auto-converted from "sass-spec/spec/libsass/conversions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("conversions")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  width: 3cm * 2in * 2in / 1cm / 1cm;\
             \n  width: 3cm * 2in / 1cm;\
             \n  width: 4cm * (12in / 3in);\
             \n}"),
        "div {\
         \n  width: 30.48in;\
         \n  width: 6in;\
         \n  width: 16cm;\
         \n}\n"
    );
}
