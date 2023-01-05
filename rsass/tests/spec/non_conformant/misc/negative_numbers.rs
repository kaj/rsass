//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/negative_numbers.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("negative_numbers")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zero: 0;\
             \na {\
             \n  zero: -$zero;\
             \n  zero: $zero * -1;\
             \n}\
             \n$near: 0.000000000001;\
             \na {\
             \n  near: -$near;\
             \n  near: $near * -1;\
             \n}\n"),
        "a {\
         \n  zero: 0;\
         \n  zero: 0;\
         \n}\
         \na {\
         \n  near: 0;\
         \n  near: 0;\
         \n}\n"
    );
}
