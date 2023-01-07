//! Tests auto-converted from "sass-spec/spec/values/maps/length.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("length")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: (aaa: 100, bbb: 200, ccc: 300);\n\
             \na {\
             \n  b: length($map);\
             \n}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
