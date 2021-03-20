//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1102.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  display:expression(\"inline\",\
            \n    (this.innerHTML += (this.innerHTML.indexOf(\",\") == -1 ? \", \" : \"\")),\
            \n    this.runtimeStyle.display = \"inline\");\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  display: expression(\"inline\", (this.innerHTML += (this.innerHTML.indexOf(\",\") == -1 ? \", \" : \"\")), this.runtimeStyle.display = \"inline\");\
        \n}\
        \n"
    );
}
