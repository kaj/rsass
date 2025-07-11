//! Tests auto-converted from "sass-spec/spec/css/custom_properties/empty.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty")
}

#[test]
fn interpolation() {
    assert_eq!(
        runner().ok(".empty {\
             \n  --property:#{\"\"};\
             \n}\n"),
        ".empty {\
         \n  --property:;\
         \n}\n"
    );
}
#[test]
fn literal() {
    assert_eq!(
        runner().ok(".empty {\
             \n  --property:;\
             \n}\n"),
        ".empty {\
         \n  --property:;\
         \n}\n"
    );
}
