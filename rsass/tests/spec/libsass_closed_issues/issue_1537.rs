//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1537.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1537")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$map: (\
             \n  a: 1, two, 3,\
             \n  foo: \'bar\'\
             \n);\n\
             \ntest {\
             \n  a: map-get($map, a);\
             \n  type: type-of(map-get($map, a));\
             \n  keys: map-keys($map);\
             \n  try: map-get($map, two);\
             \n}\n"
        ),
        "Error: expected \":\".\
         \n  ,\
         \n2 |   a: 1, two, 3,\
         \n  |            ^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
