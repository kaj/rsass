//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-warn/property.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\r\
             \n  b: {\r\
             \n    @warn \"warn\";\r\
             \n    foo: bar;\r\
             \n  }\r\
             \n}"),
        "a {\
         \n  b-foo: bar;\
         \n}\n"
    );
}
