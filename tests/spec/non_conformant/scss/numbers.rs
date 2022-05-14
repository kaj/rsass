//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/numbers.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("numbers")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  width: 10px;\
             \n  height: 20%;\
             \n  blah: 12;\
             \n  color: #abc;\
             \n}"),
        "div {\
         \n  width: 10px;\
         \n  height: 20%;\
         \n  blah: 12;\
         \n  color: #abc;\
         \n}\n"
    );
}
