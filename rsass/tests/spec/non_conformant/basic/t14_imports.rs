//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/14_imports.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("14_imports")
        .mock_file("a.scss", "div {\n  span {\n    moo: goo;\n  }\n}\n\n$x: boo;")
        .mock_file("b.scss", "hoo {\n  mux: scooba-dee-doo;\n  flux: gooboo $x;\n  @import \"d.scss\";\n}")
        .mock_file("d.scss", "d {\n  inside: d now;\n}")
        .mock_file("sub/c.scss", "blux {\n  hey: another thing;\n  ho: will this work;\n}")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"a.scss\";\n\
             \nfoo {\
             \n  blah: blah;\
             \n  goo {\
             \n    blee: blee;\
             \n    @import \"../14_imports/b.scss\";\
             \n    hello: world;\
             \n  }\
             \n  @import \"sub/c.scss\";\
             \n}"),
        "div span {\
         \n  moo: goo;\
         \n}\
         \nfoo {\
         \n  blah: blah;\
         \n}\
         \nfoo goo {\
         \n  blee: blee;\
         \n}\
         \nfoo goo hoo {\
         \n  mux: scooba-dee-doo;\
         \n  flux: gooboo boo;\
         \n}\
         \nfoo goo hoo d {\
         \n  inside: d now;\
         \n}\
         \nfoo goo {\
         \n  hello: world;\
         \n}\
         \nfoo blux {\
         \n  hey: another thing;\
         \n  ho: will this work;\
         \n}\n"
    );
}
