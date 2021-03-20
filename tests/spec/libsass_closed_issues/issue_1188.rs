//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1188.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$columns: 4;\
            \n$context: 120px;\
            \n$name-multiplicator: 2;\
            \nfoo {\
            \n  *width: expression((this.parentNode.clientWidth/#{$context}*#{($columns / $name-multiplicator)} - parseInt(this.currentStyle[\'paddingLeft\']) - parseInt(this.currentStyle[\'paddingRight\'])) + \'px\');\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  *width: expression((this.parentNode.clientWidth/120px*2 - parseInt(this.currentStyle[\"paddingLeft\"]) - parseInt(this.currentStyle[\"paddingRight\"])) + \"px\");\
        \n}\
        \n"
    );
}
