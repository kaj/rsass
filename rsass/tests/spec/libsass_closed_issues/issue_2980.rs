//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2980.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2980")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n$config: (\
             \n        phone: (\
             \n                break-point-width:0px,\
             \n                break-point-name: xs\
             \n        ),\
             \n        tablet: (\
             \n                break-point-width:600px,\
             \n                break-point-name: sm\
             \n        ),\
             \n        laptop: (\
             \n                break-point-width:900px,\
             \n                break-point-name: md\
             \n        ),\
             \n        desktop: (\
             \n                break-point-width:1200px,\
             \n                break-point-name:lg\
             \n        ),\
             \n);\n\
             \n@each $key, $map in $config {\
             \n  $break-point-width: map.get($map, break-point-width);\
             \n  $break-point-name: map.get($map, break-point-name);\
             \n  $infix: if($break-point-width == 0px, null, -$break-point-name);\
             \n      .foo#{$infix} {\
             \n        content: \'#{$break-point-name}\';\
             \n      }\
             \n}\n"
        ),
        ".foo {\
         \n  content: \"xs\";\
         \n}\
         \n.foo-sm {\
         \n  content: \"sm\";\
         \n}\
         \n.foo-md {\
         \n  content: \"md\";\
         \n}\
         \n.foo-lg {\
         \n  content: \"lg\";\
         \n}\n"
    );
}
