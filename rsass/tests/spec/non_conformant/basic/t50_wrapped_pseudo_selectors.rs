//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/50_wrapped_pseudo_selectors.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("50_wrapped_pseudo_selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "div {\
             \n  :-moz-any(ol p.blah, ul, menu, dir) :-moz-any(ol span + h1, ul, menu, dir) ul {\
             \n    list-style-type: square;\
             \n  }\
             \n  :-moz-any(ol span + h1, ul, menu, dir) ul {\
             \n    list-style-type: square;\
             \n  }\
             \n  :foo(p div) {\
             \n    hi: hi;\
             \n  }\
             \n  :foo(ol) {\
             \n    hi: hi;\
             \n  }\
             \n}\n"
        ),
        "div :-moz-any(ol p.blah, ul, menu, dir) :-moz-any(ol span + h1, ul, menu, dir) ul {\
         \n  list-style-type: square;\
         \n}\
         \ndiv :-moz-any(ol span + h1, ul, menu, dir) ul {\
         \n  list-style-type: square;\
         \n}\
         \ndiv :foo(p div) {\
         \n  hi: hi;\
         \n}\
         \ndiv :foo(ol) {\
         \n  hi: hi;\
         \n}\n"
    );
}
