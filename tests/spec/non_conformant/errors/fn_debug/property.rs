//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-debug/property.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\r\
             \n  b: {\r\
             \n    @debug \"debug\";\r\
             \n    foo: bar;\r\
             \n  }\r\
             \n}"),
        "a {\
         \n  b-foo: bar;\
         \n}\n"
    );
}
