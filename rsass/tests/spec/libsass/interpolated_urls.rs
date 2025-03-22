//! Tests auto-converted from "sass-spec/spec/libsass/interpolated-urls.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolated-urls")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$base_url: \"/static_loc/\";\
             \ndiv {\
             \n  background-image: \"url(\"#{$base_url}\"img/beta.png)\";\
             \n}\n\
             \nspan {\
             \n  background-image: url(#{$base_url}img/beta.png);\
             \n}\n\
             \nfudge {\
             \n  walnuts: blix\"fludge\"#{hey now}123;\
             \n}"),
        "div {\
         \n  background-image: \"url(\" /static_loc/ \"img/beta.png)\";\
         \n}\
         \nspan {\
         \n  background-image: url(/static_loc/img/beta.png);\
         \n}\
         \nfudge {\
         \n  walnuts: blix \"fludge\" hey now123;\
         \n}\n"
    );
}
