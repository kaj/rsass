//! Tests auto-converted from "sass-spec/spec/libsass/placeholder-mediaquery.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder-mediaquery")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo {\
             \n\t@media screen and (min-width: 300px) {\
             \n\t\tmax-width: 80%;\
             \n\t}\
             \n}\n\
             \nbar {\
             \n\t@extend %foo;\
             \n}\n"),
        "@media screen and (min-width: 300px) {\
         \n  bar {\
         \n    max-width: 80%;\
         \n  }\
         \n}\n"
    );
}
