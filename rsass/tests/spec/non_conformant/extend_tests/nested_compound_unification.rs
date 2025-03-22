//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/nested-compound-unification.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested-compound-unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "// Make sure compound unification properly handles weaving together parent\
             \n// selectors.\
             \n.a .b {@extend .e}\
             \n.c .d {@extend .f}\
             \n.e.f {x: y}\n"
        ),
        ".e.f, .a .f.b, .c .e.d, .a .c .b.d, .c .a .b.d {\
         \n  x: y;\
         \n}\n"
    );
}
