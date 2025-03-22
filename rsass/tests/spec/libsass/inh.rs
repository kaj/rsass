//! Tests auto-converted from "sass-spec/spec/libsass/inh.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inh")
}

#[test]
#[ignore] // unexepected error
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
