//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1574.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  bar: baz;\
             \n}\n\
             \ninput[type=\"text\"],\
             \ninput[type=\"search\"],\
             \ninput[type=\"url\"],\
             \ninput[type=\"email\"],\
             \ninput[type=\"password\"],\
             \ninput[type=\"number\"],\
             \ninput[type=\"tel\"],\
             \ninput[type=\"date\"],\
             \ninput[type=\"range\"],\
             \ntextarea {\
             \n  @extend .foo;\
             \n}\n"),
        ".foo, input[type=text],\
         \ninput[type=search],\
         \ninput[type=url],\
         \ninput[type=email],\
         \ninput[type=password],\
         \ninput[type=number],\
         \ninput[type=tel],\
         \ninput[type=date],\
         \ninput[type=range],\
         \ntextarea {\
         \n  bar: baz;\
         \n}\n"
    );
}
