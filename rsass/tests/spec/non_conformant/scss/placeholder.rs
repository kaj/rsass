//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/placeholder.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%x {\
             \n  color: red;\
             \n}\n\
             \nfoo {\
             \n  width: 10px;\
             \n  @extend %x;\
             \n}\n\
             \nhux {\
             \n  height: 12px;\
             \n  @extend %x;\
             \n}"),
        "hux, foo {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  width: 10px;\
         \n}\
         \nhux {\
         \n  height: 12px;\
         \n}\n"
    );
}
