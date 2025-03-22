//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_192.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_192")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function test($from, $to) {\r\
             \n    @warn \'Starting loop\';\r\
             \n    @for $i from $from through $to {\r\
             \n      @warn \'Step #{$i}\' ;\r\
             \n    }\r\
             \n    @warn \'Finished loop\';\r\
             \n    @return 100%;\r\
             \n}\r\
             \nbody {\r\
             \n    width: test(0, 1);\r\
             \n    height: test(-1, 1);\r\
             \n}"),
        "body {\
         \n  width: 100%;\
         \n  height: 100%;\
         \n}\n"
    );
}
