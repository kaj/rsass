//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_cdo_and_cdc_ignored_at_toplevel.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: baz; }\
            \n\
            \nbar {\
            \n  bar: baz; }\
            \n\
            \nbaz {\
            \n  bar: baz; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \nbar {\
        \n  bar: baz;\
        \n}\
        \nbaz {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
