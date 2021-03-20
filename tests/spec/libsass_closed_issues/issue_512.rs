//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_512.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$list: a b c;\
            \n.css {\
            \n  debug: index($list, a);\
            \n\
            \n  @if type-of(index($list, 2)) == \"null\" {\
            \n    debug: foo;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".css {\
        \n  debug: 1;\
        \n  debug: foo;\
        \n}\
        \n"
    );
}
