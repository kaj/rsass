//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/compound-unification-in-not.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("compound-unification-in-not")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "// Make sure compound selectors are unified when two :not()s are extended.\
             \n// :not() is special here because it\'s the only selector that\'s extended by\
             \n// adding to the compound selector, rather than creating a new selector list.\
             \n.a {@extend .c}\
             \n.b {@extend .d}\
             \n:not(.c):not(.d) {x: y}\n"
        ),
        ":not(.c):not(.a):not(.d):not(.b) {\
         \n  x: y;\
         \n}\n"
    );
}
