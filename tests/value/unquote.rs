/// From `sass-spec/spec/libsass/unquote`
use super::check_eval;

#[test]
fn a() {
    check_eval("unquote(\"foo\")", "foo")
}
#[test]
fn b() {
    check_eval(
        "unquote(\"I'm a \\\"fashion\\\" \\\"expert\\\".\")",
        "I'm a \"fashion\" \"expert\".",
    )
}
#[test]
fn c() {
    check_eval("unquote(\\\"wha)", "\\\"wha")
}
#[test]
fn d() {
    check_eval(
        "unquote(\"column1\\tcolumn2\")",
        "column1tcolumn2",
    )
}
#[test]
fn e() {
    check_eval("unquote(23+1)", "24")
}
