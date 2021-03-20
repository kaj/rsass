//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/238_unify_root_pseudoelement.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// We assume that by default classes don\'t apply to the :root unless marked explicitly.\
            \n:root .foo-1 { test: 1; }\
            \n.bar-1 .baz-1 { @extend .foo-1; }\
            \n\
            \n// We know the two classes must be the same :root element so we can combine them.\
            \n.foo-2:root .bar-2 { test: 2; }\
            \n.baz-2:root .bang-2 { @extend .bar-2; }\
            \n\
            \n// This extend should not apply because the :root elements are different.\
            \nhtml:root .bar-3 { test: 3; }\
            \nxml:root .bang-3 { @extend .bar-3}\
            \n\
            \n// We assume that direct descendant of the :root is not the same element as a descendant.\
            \n.foo-4:root > .bar-4 .x-4 { test: 4; }\
            \n.baz-4:root .bang-4 .y-4 {@extend .x-4}\
            \n"
        )
        .unwrap(),
        ":root .foo-1, :root .bar-1 .baz-1 {\
        \n  test: 1;\
        \n}\
        \n.foo-2:root .bar-2, .baz-2.foo-2:root .bang-2 {\
        \n  test: 2;\
        \n}\
        \nhtml:root .bar-3 {\
        \n  test: 3;\
        \n}\
        \n.foo-4:root > .bar-4 .x-4, .baz-4.foo-4:root > .bar-4 .bang-4 .y-4 {\
        \n  test: 4;\
        \n}\
        \n"
    );
}
