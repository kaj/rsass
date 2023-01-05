//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1188.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1188")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "$columns: 4;\
             \n$context: 120px;\
             \n$name-multiplicator: 2;\
             \nfoo {\
             \n  *width: expression((this.parentNode.clientWidth/#{$context}*#{($columns / $name-multiplicator)} - parseInt(this.currentStyle[\'paddingLeft\']) - parseInt(this.currentStyle[\'paddingRight\'])) + \'px\');\
             \n}"
        ),
        "foo {\
         \n  *width: expression((this.parentNode.clientWidth/120px*2 - parseInt(this.currentStyle[\"paddingLeft\"]) - parseInt(this.currentStyle[\"paddingRight\"])) + \"px\");\
         \n}\n"
    );
}
