//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1596.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@document url(http://www.w3.org/),\
            \n               url-prefix(http://www.w3.org/Style/),\
            \n               domain(mozilla.org),\
            \n               regexp(\"https:.*\");\
            \n"
        )
        .unwrap(),
        "@document url(http://www.w3.org/),\
        \n               url-prefix(http://www.w3.org/Style/),\
        \n               domain(mozilla.org),\
        \n               regexp(\"https:.*\");\
        \n"
    );
}
