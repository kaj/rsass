//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2467.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2467")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: [footer-right] / 120px;\
             \n  b: [footer-right]/ 120px;\
             \n  c: [footer-right] /120px;\
             \n  d: [footer-right]/120px;\
             \n  e: [footer-right] / 120px 1fr;\
             \n  f: [footer-right]/ 120px 1fr;\
             \n  g: [footer-right] /120px 1fr;\
             \n  h: [footer-right]/120px 1fr;\
             \n}\n"),
        "foo {\
         \n  a: [footer-right]/120px;\
         \n  b: [footer-right]/120px;\
         \n  c: [footer-right]/120px;\
         \n  d: [footer-right]/120px;\
         \n  e: [footer-right]/120px 1fr;\
         \n  f: [footer-right]/120px 1fr;\
         \n  g: [footer-right]/120px 1fr;\
         \n  h: [footer-right]/120px 1fr;\
         \n}\n"
    );
}
