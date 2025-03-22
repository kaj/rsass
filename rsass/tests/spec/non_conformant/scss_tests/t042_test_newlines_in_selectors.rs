//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/042_test_newlines_in_selectors.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("042_test_newlines_in_selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo\
             \nbar {\
             \n  baz\
             \n  bang {a: b}\n\
             \n  bip bop {c: d}}\n"),
        "foo bar baz bang {\
         \n  a: b;\
         \n}\
         \nfoo bar bip bop {\
         \n  c: d;\
         \n}\n"
    );
}
