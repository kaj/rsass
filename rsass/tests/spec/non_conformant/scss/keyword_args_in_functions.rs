//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/keyword_args_in_functions.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("keyword_args_in_functions")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".keyed { color: rgba($color: #a7c, $alpha: 0.4) }\n"),
        ".keyed {\
         \n  color: rgba(170, 119, 204, 0.4);\
         \n}\n"
    );
}
