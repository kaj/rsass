//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/039_test_almost_ambiguous_nested_rules_and_declarations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("039_test_almost_ambiguous_nested_rules_and_declarations")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "foo {\
             \n  bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses {a: b};\
             \n  bar:baz bang bop biddle woo look at all these elems {a: b};\
             \n  bar:baz bang bop biddle woo look at all these elems; }\n"
        ),
        "foo {\
         \n  bar: baz bang bop biddle woo look at all these elems;\
         \n}\
         \nfoo bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses {\
         \n  a: b;\
         \n}\
         \nfoo bar:baz bang bop biddle woo look at all these elems {\
         \n  a: b;\
         \n}\n"
    );
}
