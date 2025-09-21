//! Tests auto-converted from "sass-spec/spec/non_conformant/colors/change-color.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("change-color")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \np {\
             \n  color: color.change(#102030, $blue: 5);\
             \n  color: color.change(#102030, $alpha: .325);\
             \n  color: color.change(#102030, $red: 120, $blue: 5);\
             \n  color: color.change(hsl(25, 100%, 80%), $lightness: 40%, $alpha: 0.8);\
             \n}"
        ),
        "p {\
         \n  color: #102005;\
         \n  color: rgba(16, 32, 48, 0.325);\
         \n  color: #782005;\
         \n  color: hsla(25, 100%, 40%, 0.8);\
         \n}\n"
    );
}
