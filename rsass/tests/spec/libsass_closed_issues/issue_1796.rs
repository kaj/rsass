//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1796.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1796")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n.parent {\
             \n    .brother, .sister, .cousin {\
             \n        color: green;\
             \n        sel: &;\n\
             \n        $new-sel: ();\
             \n        @each $s in & {\
             \n            $last: list.nth($s, -1);\
             \n            $new-sel: list.append($new-sel, $s #{\'+\'} $last, comma);\
             \n            x: $new-sel;\
             \n        }\
             \n        @at-root #{$new-sel} {\
             \n            debug: foo;\
             \n        }\
             \n    }\
             \n}"
        ),
        ".parent .brother, .parent .sister, .parent .cousin {\
         \n  color: green;\
         \n  sel: .parent .brother, .parent .sister, .parent .cousin;\
         \n  x: .parent .brother + .brother;\
         \n  x: .parent .brother + .brother, .parent .sister + .sister;\
         \n  x: .parent .brother + .brother, .parent .sister + .sister, .parent .cousin + .cousin;\
         \n}\
         \n.parent .brother + .brother, .parent .sister + .sister, .parent .cousin + .cousin {\
         \n  debug: foo;\
         \n}\n"
    );
}
