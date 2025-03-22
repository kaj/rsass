//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1102.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1102")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "foo {\
             \n  display:expression(\"inline\",\
             \n    (this.innerHTML += (this.innerHTML.indexOf(\",\") == -1 ? \", \" : \"\")),\
             \n    this.runtimeStyle.display = \"inline\");\
             \n}\n"
        ),
        "foo {\
         \n  display: expression(\"inline\", (this.innerHTML += (this.innerHTML.indexOf(\",\") == -1 ? \", \" : \"\")), this.runtimeStyle.display = \"inline\");\
         \n}\n"
    );
}
