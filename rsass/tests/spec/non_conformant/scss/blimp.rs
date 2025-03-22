//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/blimp.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("blimp")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("blimp { color: green }\n"),
        "blimp {\
         \n  color: green;\
         \n}\n"
    );
}
