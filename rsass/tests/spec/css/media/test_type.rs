//! Tests auto-converted from "sass-spec/spec/css/media/type.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
fn not() {
    assert_eq!(
        runner().ok("@media not a {x {y: z}}\n"),
        "@media not a {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
