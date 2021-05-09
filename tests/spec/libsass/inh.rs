//! Tests auto-converted from "sass-spec/spec/libsass/inh.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo.a {\
             \n  width: 10px;\
             \n}\n\
             \nbar {\
             \n  color: red;\
             \n  @extend foo;\
             \n}"),
        "foo.a, bar.a {\
         \n  width: 10px;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\n"
    );
}
