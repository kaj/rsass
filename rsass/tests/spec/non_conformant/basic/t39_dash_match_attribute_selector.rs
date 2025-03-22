//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/39_dash_match_attribute_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("39_dash_match_attribute_selector")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div[class|=\"blah\"] {\
             \n  color: blue;\
             \n}"),
        "div[class|=blah] {\
         \n  color: blue;\
         \n}\n"
    );
}
