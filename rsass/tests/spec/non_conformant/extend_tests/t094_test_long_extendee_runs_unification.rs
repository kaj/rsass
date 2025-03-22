//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/094_test_long_extendee_runs_unification.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("094_test_long_extendee_runs_unification")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "ns|*.foo.bar {a: b}\
             \na.baz {@extend .foo.bar}\n"
        ),
        "Error: compound selectors may no longer be extended.\
         \nConsider `@extend .foo, .bar` instead.\
         \nSee https://sass-lang.com/d/extend-compound for details.\n\
         \n  ,\
         \n2 | a.baz {@extend .foo.bar}\
         \n  |                ^^^^^^^^\
         \n  \'\
         \n  input.scss 2:16  root stylesheet",
    );
}
