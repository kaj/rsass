//! Tests auto-converted from "sass-spec/spec/libsass/interpolated-urls.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$base_url: \"/static_loc/\";\
            \ndiv {\
            \n  background-image: \"url(\"#{$base_url}\"img/beta.png)\";\
            \n}\
            \n\
            \nspan {\
            \n  background-image: url(#{$base_url}img/beta.png);\
            \n}\
            \n\
            \nfudge {\
            \n  walnuts: blix\"fludge\"#{hey now}123;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-image: \"url(\" /static_loc/ \"img/beta.png)\";\
        \n}\
        \nspan {\
        \n  background-image: url(/static_loc/img/beta.png);\
        \n}\
        \nfudge {\
        \n  walnuts: blix \"fludge\" hey now123;\
        \n}\
        \n"
    );
}
