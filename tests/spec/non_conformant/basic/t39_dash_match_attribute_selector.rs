//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/39_dash_match_attribute_selector.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div[class|=\"blah\"] {\
            \n  color: blue;\
            \n}"
        )
        .unwrap(),
        "div[class|=blah] {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
