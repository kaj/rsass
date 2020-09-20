//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/extend-tests/001_test_basic.hrx"
#[test]
#[ignore] // unexepected error
fn t001_test_basic() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/002_test_basic.hrx"
#[test]
#[ignore] // unexepected error
fn t002_test_basic() {
    assert_eq!(
        rsass(
            ".bar {@extend .foo}\
            \n.foo {a: b}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/003_test_basic.hrx"
#[test]
#[ignore] // unexepected error
fn t003_test_basic() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {c: d; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/004_test_basic.hrx"
#[test]
#[ignore] // wrong result
fn t004_test_basic() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo; c: d}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/005_test_multiple_targets.hrx"
#[test]
#[ignore] // unexepected error
fn t005_test_multiple_targets() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo}\
            \n.blip .foo {c: d}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.blip .foo, .blip .bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/006_test_multiple_extendees.hrx"
#[test]
#[ignore] // unexepected error
fn t006_test_multiple_extendees() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {c: d}\
            \n.baz {@extend .foo; @extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .baz {\
        \n  a: b;\
        \n}\
        \n.bar, .baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/007_test_multiple_extends_with_single_extender_and_single_target.hrx"
#[test]
#[ignore] // unexepected error
fn t007_test_multiple_extends_with_single_extender_and_single_target() {
    assert_eq!(
        rsass(
            ".foo .bar {a: b}\
            \n.baz {@extend .foo; @extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .baz, .baz .bar, .baz .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/008_test_multiple_extends_with_single_extender_and_single_target.hrx"
#[test]
#[ignore] // unexepected error
fn t008_test_multiple_extends_with_single_extender_and_single_target() {
    assert_eq!(
        rsass(
            ".foo.bar {a: b}\
            \n.baz {@extend .foo; @extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/009_test_multiple_extends_with_multiple_extenders_and_single_target.hrx"
#[test]
#[ignore] // unexepected error
fn t009_test_multiple_extends_with_multiple_extenders_and_single_target() {
    assert_eq!(
        rsass(
            ".foo .bar {a: b}\
            \n.baz {@extend .foo}\
            \n.bang {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .bang, .baz .bar, .baz .bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/010_test_multiple_extends_with_multiple_extenders_and_single_target.hrx"
#[test]
#[ignore] // unexepected error
fn t010_test_multiple_extends_with_multiple_extenders_and_single_target() {
    assert_eq!(
        rsass(
            ".foo.bar {a: b}\
            \n.baz {@extend .foo}\
            \n.bang {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .foo.bang, .bar.baz, .baz.bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/011_test_chained_extends.hrx"
#[test]
#[ignore] // unexepected error
fn t011_test_chained_extends() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {@extend .foo}\
            \n.baz {@extend .bar}\
            \n.bip {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .bar, .bip, .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/012_test_dynamic_extendee.hrx"
#[test]
#[ignore] // unexepected error
fn t012_test_dynamic_extendee() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {@extend #{\".foo\"}}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/013_test_dynamic_extendee.hrx"
#[test]
#[ignore] // unexepected error
fn t013_test_dynamic_extendee() {
    assert_eq!(
        rsass(
            "[baz^=\"blip12px\"] {a: b}\
            \n.bar {@extend [baz^=\"blip#{12px}\"]}\
            \n"
        )
        .unwrap(),
        "[baz^=blip12px], .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/014_test_nested_target.hrx"
#[test]
#[ignore] // unexepected error
fn t014_test_nested_target() {
    assert_eq!(
        rsass(
            ".foo .bar {a: b}\
            \n.baz {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/015_test_target_with_child.hrx"
#[test]
#[ignore] // unexepected error
fn t015_test_target_with_child() {
    assert_eq!(
        rsass(
            ".foo .bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .baz .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/016_test_class_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t016_test_class_unification() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \n.baz {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a .bar.baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/017_test_class_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t017_test_class_unification() {
    assert_eq!(
        rsass(
            "%-a .foo.baz {a: b}\
            \n.baz {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/018_test_id_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t018_test_id_unification() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \n#baz {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a .bar#baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/019_test_id_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t019_test_id_unification() {
    assert_eq!(
        rsass(
            "%-a .foo#baz {a: b}\
            \n#baz {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a #baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/020_test_universal_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t020_test_universal_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo, -a * {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/021_test_universal_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t021_test_universal_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo, -a *|* {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/022_test_universal_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t022_test_universal_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/023_test_universal_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t023_test_universal_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/024_test_universal_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t024_test_universal_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a ns|*.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/025_test_universal_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t025_test_universal_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a * {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/026_test_universal_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t026_test_universal_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a * {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/027_test_universal_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t027_test_universal_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *|*.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|*.foo, -a * {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/028_test_universal_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t028_test_universal_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *|*.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|* {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/029_test_universal_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t029_test_universal_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *.foo {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/030_test_universal_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t030_test_universal_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *|*.foo {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|*.foo, -a ns|* {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/031_test_universal_unification_with_namespaced_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t031_test_universal_unification_with_namespaced_universal_target() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/032_test_universal_unification_with_namespaced_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t032_test_universal_unification_with_namespaced_universal_target() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|* {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/033_test_universal_unification_with_namespaced_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t033_test_universal_unification_with_namespaced_universal_target() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|* {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/034_test_universal_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t034_test_universal_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a a.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/035_test_universal_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t035_test_universal_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a a.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/036_test_universal_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t036_test_universal_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a *|a.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a.foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/037_test_universal_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t037_test_universal_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a *|a.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/038_test_universal_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t038_test_universal_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a a.foo {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/039_test_universal_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t039_test_universal_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a *|a.foo {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a.foo, -a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/040_test_universal_unification_with_namespaced_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t040_test_universal_unification_with_namespaced_element_target() {
    assert_eq!(
        rsass(
            "%-a ns|a.foo {a: b}\
            \n* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/041_test_universal_unification_with_namespaced_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t041_test_universal_unification_with_namespaced_element_target() {
    assert_eq!(
        rsass(
            "%-a ns|a.foo {a: b}\
            \n*|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/042_test_universal_unification_with_namespaced_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t042_test_universal_unification_with_namespaced_element_target() {
    assert_eq!(
        rsass(
            "%-a ns|a.foo {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/043_test_element_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t043_test_element_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/044_test_element_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t044_test_element_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a a.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/045_test_element_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t045_test_element_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a *|a.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/046_test_element_unification_with_simple_target.hrx"
#[test]
#[ignore] // unexepected error
fn t046_test_element_unification_with_simple_target() {
    assert_eq!(
        rsass(
            "%-a .foo.bar {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a ns|a.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/047_test_element_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t047_test_element_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *.foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/048_test_element_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t048_test_element_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *.foo {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *.foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/049_test_element_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t049_test_element_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *|*.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|*.foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/050_test_element_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t050_test_element_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *|*.foo {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|*.foo, -a *|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/051_test_element_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t051_test_element_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/052_test_element_unification_with_namespaceless_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t052_test_element_unification_with_namespaceless_universal_target() {
    assert_eq!(
        rsass(
            "%-a *|*.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|*.foo, -a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/053_test_element_unification_with_namespaced_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t053_test_element_unification_with_namespaced_universal_target() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/054_test_element_unification_with_namespaced_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t054_test_element_unification_with_namespaced_universal_target() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo, -a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/055_test_element_unification_with_namespaced_universal_target.hrx"
#[test]
#[ignore] // unexepected error
fn t055_test_element_unification_with_namespaced_universal_target() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo, -a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/056_test_element_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t056_test_element_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a a.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/057_test_element_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t057_test_element_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a a.foo {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/058_test_element_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t058_test_element_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a *|a.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a.foo, -a a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/059_test_element_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t059_test_element_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a *|a.foo {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/060_test_element_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t060_test_element_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a a.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/061_test_element_unification_with_namespaceless_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t061_test_element_unification_with_namespaceless_element_target() {
    assert_eq!(
        rsass(
            "%-a *|a.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a *|a.foo, -a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/062_test_element_unification_with_namespaced_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t062_test_element_unification_with_namespaced_element_target() {
    assert_eq!(
        rsass(
            "%-a ns|a.foo {a: b}\
            \na {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/063_test_element_unification_with_namespaced_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t063_test_element_unification_with_namespaced_element_target() {
    assert_eq!(
        rsass(
            "%-a ns|a.foo {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/064_test_element_unification_with_namespaced_element_target.hrx"
#[test]
#[ignore] // unexepected error
fn t064_test_element_unification_with_namespaced_element_target() {
    assert_eq!(
        rsass(
            "%-a ns|a.foo {a: b}\
            \nns|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/065_test_attribute_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t065_test_attribute_unification() {
    assert_eq!(
        rsass(
            "%-a [foo=bar].baz {a: b}\
            \n[foo=baz] {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a [foo=bar].baz, -a [foo=bar][foo=baz] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/066_test_attribute_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t066_test_attribute_unification() {
    assert_eq!(
        rsass(
            "%-a [foo=bar].baz {a: b}\
            \n[foo^=bar] {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a [foo=bar].baz, -a [foo=bar][foo^=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/067_test_attribute_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t067_test_attribute_unification() {
    assert_eq!(
        rsass(
            "%-a [foo=bar].baz {a: b}\
            \n[foot=bar] {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a [foo=bar].baz, -a [foo=bar][foot=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/068_test_attribute_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t068_test_attribute_unification() {
    assert_eq!(
        rsass(
            "%-a [foo=bar].baz {a: b}\
            \n[ns|foo=bar] {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a [foo=bar].baz, -a [foo=bar][ns|foo=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/069_test_attribute_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t069_test_attribute_unification() {
    assert_eq!(
        rsass(
            "%-a %-a [foo=bar].bar {a: b}\
            \n[foo=bar] {@extend .bar} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a -a [foo=bar] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/070_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t070_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a :foo.baz {a: b}\
            \n:foo(2n+1) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :foo.baz, -a :foo:foo(2n+1) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/071_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t071_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a :foo.baz {a: b}\
            \n::foo {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :foo.baz, -a :foo::foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/072_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t072_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a ::foo.baz {a: b}\
            \n::foo {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ::foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/073_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t073_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a ::foo(2n+1).baz {a: b}\
            \n::foo(2n+1) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ::foo(2n+1) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/074_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t074_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a :foo.baz {a: b}\
            \n:bar {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :foo.baz, -a :foo:bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/075_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t075_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a .baz:foo {a: b}\
            \n:after {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .baz:foo, -a :foo:after {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/076_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t076_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a .baz:after {a: b}\
            \n:foo {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .baz:after, -a :foo:after {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/077_test_pseudo_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t077_test_pseudo_unification() {
    assert_eq!(
        rsass(
            "%-a :foo.baz {a: b}\
            \n:foo {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/078_test_pseudoelement_remains_at_end_of_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t078_test_pseudoelement_remains_at_end_of_selector() {
    assert_eq!(
        rsass(
            ".foo::bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo::bar, .baz::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/079_test_pseudoelement_remains_at_end_of_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t079_test_pseudoelement_remains_at_end_of_selector() {
    assert_eq!(
        rsass(
            "a.foo::bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        "a.foo::bar, a.baz::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/080_test_pseudoclass_remains_at_end_of_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t080_test_pseudoclass_remains_at_end_of_selector() {
    assert_eq!(
        rsass(
            ".foo:bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo:bar, .baz:bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/081_test_pseudoclass_remains_at_end_of_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t081_test_pseudoclass_remains_at_end_of_selector() {
    assert_eq!(
        rsass(
            "a.foo:bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        "a.foo:bar, a.baz:bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/082_test_not_remains_at_end_of_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t082_test_not_remains_at_end_of_selector() {
    assert_eq!(
        rsass(
            ".foo:not(.bar) {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo:not(.bar), .baz:not(.bar) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/083_test_pseudoelement_goes_lefter_than_pseudoclass.hrx"
#[test]
#[ignore] // unexepected error
fn t083_test_pseudoelement_goes_lefter_than_pseudoclass() {
    assert_eq!(
        rsass(
            ".foo::bar {a: b}\
            \n.baz:bang {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo::bar, .baz:bang::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/084_test_pseudoelement_goes_lefter_than_pseudoclass.hrx"
#[test]
#[ignore] // unexepected error
fn t084_test_pseudoelement_goes_lefter_than_pseudoclass() {
    assert_eq!(
        rsass(
            ".foo:bar {a: b}\
            \n.baz::bang {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo:bar, .baz:bar::bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/085_test_pseudoelement_goes_lefter_than_not.hrx"
#[test]
#[ignore] // unexepected error
fn t085_test_pseudoelement_goes_lefter_than_not() {
    assert_eq!(
        rsass(
            ".foo::bar {a: b}\
            \n.baz:not(.bang) {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo::bar, .baz:not(.bang)::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/086.1_test_pseudoelement_goes_lefter_than_not.hrx"
#[test]
#[ignore] // wrong result
fn t086_1_test_pseudoelement_goes_lefter_than_not() {
    assert_eq!(
        rsass(
            "%a {\
            \n  x:y;\
            \n}\
            \nb:after:not(:first-child) {\
            \n  @extend %a;\
            \n}\
            \nc:s {\
            \n  @extend %a;  \
            \n}\
            \nd::e {\
            \n  @extend c;\
            \n}"
        )
        .unwrap(),
        "c:s, d:s::e, b:after:not(:first-child) {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/086_test_pseudoelement_goes_lefter_than_not.hrx"
#[test]
#[ignore] // unexepected error
fn t086_test_pseudoelement_goes_lefter_than_not() {
    assert_eq!(
        rsass(
            ".foo:not(.bang) {a: b}\
            \n.baz::bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo:not(.bang), .baz:not(.bang)::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/087_test_negation_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t087_test_negation_unification() {
    assert_eq!(
        rsass(
            "%-a :not(.foo).baz {a: b}\
            \n:not(.bar) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :not(.foo).baz, -a :not(.foo):not(.bar) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/088_test_negation_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t088_test_negation_unification() {
    assert_eq!(
        rsass(
            "%-a :not(.foo).baz {a: b}\
            \n:not(.foo) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :not(.foo) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/089_test_negation_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t089_test_negation_unification() {
    assert_eq!(
        rsass(
            "%-a :not([a=b]).baz {a: b}\
            \n:not([a = b]) {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a :not([a=b]) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/090_test_comma_extendee.hrx"
#[test]
#[ignore] // unexepected error
fn t090_test_comma_extendee() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {c: d}\
            \n.baz {@extend .foo, .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .baz {\
        \n  a: b;\
        \n}\
        \n.bar, .baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/091_test_redundant_selector_elimination.hrx"
#[test]
#[ignore] // unexepected error
fn t091_test_redundant_selector_elimination() {
    assert_eq!(
        rsass(
            ".foo.bar {a: b}\
            \n.x {@extend .foo, .bar}\
            \n.y {@extend .foo, .bar}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .y, .x {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/094_test_long_extendee_runs_unification.hrx"

// Ignoring "t094_test_long_extendee_runs_unification", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/extend-tests/095_test_long_extender.hrx"
#[test]
#[ignore] // unexepected error
fn t095_test_long_extender() {
    assert_eq!(
        rsass(
            ".foo.bar {a: b}\
            \n.baz.bang {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .bar.baz.bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/096_test_long_extender_runs_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t096_test_long_extender_runs_unification() {
    assert_eq!(
        rsass(
            "ns|*.foo.bar {a: b}\
            \na.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        "ns|*.foo.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/097_test_nested_extender.hrx"
#[test]
#[ignore] // unexepected error
fn t097_test_nested_extender() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \nfoo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/098_test_nested_extender_runs_unification.hrx"
#[test]
#[ignore] // unexepected error
fn t098_test_nested_extender_runs_unification() {
    assert_eq!(
        rsass(
            ".foo.bar {a: b}\
            \nfoo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo.bar, foo bar.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/099_test_nested_extender_alternates_parents.hrx"
#[test]
#[ignore] // unexepected error
fn t099_test_nested_extender_alternates_parents() {
    assert_eq!(
        rsass(
            ".baz .bip .foo {a: b}\
            \nfoo .grank bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .bip .foo, .baz .bip foo .grank bar, foo .grank .baz .bip bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/100_test_nested_extender_unifies_identical_parents.hrx"
#[test]
#[ignore] // unexepected error
fn t100_test_nested_extender_unifies_identical_parents() {
    assert_eq!(
        rsass(
            ".baz .bip .foo {a: b}\
            \n.baz .bip bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .bip .foo, .baz .bip bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/101_test_nested_extender_unifies_common_substring.hrx"
#[test]
#[ignore] // unexepected error
fn t101_test_nested_extender_unifies_common_substring() {
    assert_eq!(
        rsass(
            ".baz .bip .bap .bink .foo {a: b}\
            \n.brat .bip .bap bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .bip .bap .bink .foo, .baz .brat .bip .bap .bink bar, .brat .baz .bip .bap .bink bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/102_test_nested_extender_unifies_common_subseq.hrx"
#[test]
#[ignore] // unexepected error
fn t102_test_nested_extender_unifies_common_subseq() {
    assert_eq!(
        rsass(
            ".a .x .b .y .foo {a: b}\
            \n.a .n .b .m bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".a .x .b .y .foo, .a .x .n .b .y .m bar, .a .n .x .b .y .m bar, .a .x .n .b .m .y bar, .a .n .x .b .m .y bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/103_test_nested_extender_chooses_first_subseq.hrx"
#[test]
#[ignore] // unexepected error
fn t103_test_nested_extender_chooses_first_subseq() {
    assert_eq!(
        rsass(
            ".a .b .c .d .foo {a: b}\
            \n.c .d .a .b .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".a .b .c .d .foo, .a .b .c .d .a .b .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

mod t104_test_nested_extender_counts_extended_subselectors;

// From "sass-spec/spec/non_conformant/extend-tests/105_test_nested_extender_counts_extended_superselectors.hrx"
#[test]
#[ignore] // unexepected error
fn t105_test_nested_extender_counts_extended_superselectors() {
    assert_eq!(
        rsass(
            ".a .bip .foo {a: b}\
            \n.b .bip.bop .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".a .bip .foo, .a .b .bip.bop .bar, .b .a .bip.bop .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/106_test_nested_extender_with_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t106_test_nested_extender_with_child_selector() {
    assert_eq!(
        rsass(
            ".baz .foo {a: b}\
            \nfoo > bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .foo, .baz foo > bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/107_test_nested_extender_finds_common_selectors_around_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t107_test_nested_extender_finds_common_selectors_around_child_selector() {
    assert_eq!(
        rsass(
            "a > b c .c1 {a: b}\
            \na c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a > b c .c1, a > b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/108_test_nested_extender_finds_common_selectors_around_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t108_test_nested_extender_finds_common_selectors_around_child_selector() {
    assert_eq!(
        rsass(
            "a > b c .c1 {a: b}\
            \nb c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a > b c .c1, a > b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/109_test_nested_extender_finds_common_selectors_around_adjacent_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t109_test_nested_extender_finds_common_selectors_around_adjacent_sibling()
{
    assert_eq!(
        rsass(
            "a + b c .c1 {a: b}\
            \na c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a + b c .c1, a + b a c .c2, a a + b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/110_test_nested_extender_finds_common_selectors_around_adjacent_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t110_test_nested_extender_finds_common_selectors_around_adjacent_sibling()
{
    assert_eq!(
        rsass(
            "a + b c .c1 {a: b}\
            \na b .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a + b c .c1, a a + b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/111_test_nested_extender_finds_common_selectors_around_adjacent_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t111_test_nested_extender_finds_common_selectors_around_adjacent_sibling()
{
    assert_eq!(
        rsass(
            "a + b c .c1 {a: b}\
            \nb c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a + b c .c1, a + b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/112_test_nested_extender_finds_common_selectors_around_sibling_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t112_test_nested_extender_finds_common_selectors_around_sibling_selector()
{
    assert_eq!(
        rsass(
            "a ~ b c .c1 {a: b}\
            \na c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a ~ b c .c1, a ~ b a c .c2, a a ~ b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/113_test_nested_extender_finds_common_selectors_around_sibling_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t113_test_nested_extender_finds_common_selectors_around_sibling_selector()
{
    assert_eq!(
        rsass(
            "a ~ b c .c1 {a: b}\
            \na b .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a ~ b c .c1, a a ~ b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/114_test_nested_extender_finds_common_selectors_around_sibling_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t114_test_nested_extender_finds_common_selectors_around_sibling_selector()
{
    assert_eq!(
        rsass(
            "a ~ b c .c1 {a: b}\
            \nb c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a ~ b c .c1, a ~ b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/118_test_nested_extender_with_early_child_selectors_doesnt_subseq_them.hrx"
#[test]
#[ignore] // unexepected error
fn t118_test_nested_extender_with_early_child_selectors_doesnt_subseq_them() {
    assert_eq!(
        rsass(
            ".bip > .bap .foo {a: b}\
            \n.grip > .bap .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".bip > .bap .foo, .bip > .bap .grip > .bap .bar, .grip > .bap .bip > .bap .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/119_test_nested_extender_with_early_child_selectors_doesnt_subseq_them.hrx"
#[test]
#[ignore] // unexepected error
fn t119_test_nested_extender_with_early_child_selectors_doesnt_subseq_them() {
    assert_eq!(
        rsass(
            ".bap > .bip .foo {a: b}\
            \n.bap > .grip .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".bap > .bip .foo, .bap > .bip .bap > .grip .bar, .bap > .grip .bap > .bip .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/120_test_nested_extender_with_child_selector_unifies.hrx"
#[test]
#[ignore] // unexepected error
fn t120_test_nested_extender_with_child_selector_unifies() {
    assert_eq!(
        rsass(
            ".baz.foo {a: b}\
            \nfoo > bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz.foo, foo > bar.baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/121_test_nested_extender_with_child_selector_unifies.hrx"
#[test]
#[ignore] // unexepected error
fn t121_test_nested_extender_with_child_selector_unifies() {
    assert_eq!(
        rsass(
            ".baz > {\
            \n.foo {a: b}\
            \n.bar {@extend .foo}\
            \n}\
            \n"
        )
        .unwrap(),
        ".baz > .foo, .baz > .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/122_test_nested_extender_with_child_selector_unifies.hrx"
#[test]
#[ignore] // unexepected error
fn t122_test_nested_extender_with_child_selector_unifies() {
    assert_eq!(
        rsass(
            ".foo {\
            \n.bar {a: b}\
            \n> .baz {@extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/123_test_nested_extender_with_early_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t123_test_nested_extender_with_early_child_selector() {
    assert_eq!(
        rsass(
            ".foo {\
            \n.bar {a: b}\
            \n.bip > .baz {@extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .bip > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/124_test_nested_extender_with_early_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t124_test_nested_extender_with_early_child_selector() {
    assert_eq!(
        rsass(
            ".foo {\
            \n.bip .bar {a: b}\
            \n> .baz {@extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bip .bar, .foo .bip .foo > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/125_test_nested_extender_with_early_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t125_test_nested_extender_with_early_child_selector() {
    assert_eq!(
        rsass(
            ".foo > .bar {a: b}\
            \n.bip + .baz {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo > .bar, .foo > .bip + .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/126_test_nested_extender_with_early_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t126_test_nested_extender_with_early_child_selector() {
    assert_eq!(
        rsass(
            ".foo + .bar {a: b}\
            \n.bip > .baz {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo + .bar, .bip > .foo + .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/127_test_nested_extender_with_early_child_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t127_test_nested_extender_with_early_child_selector() {
    assert_eq!(
        rsass(
            ".foo > .bar {a: b}\
            \n.bip > .baz {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo > .bar, .bip.foo > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/128_test_nested_extender_with_sibling_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t128_test_nested_extender_with_sibling_selector() {
    assert_eq!(
        rsass(
            ".baz .foo {a: b}\
            \nfoo + bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .foo, .baz foo + bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/129_test_nested_extender_with_hacky_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t129_test_nested_extender_with_hacky_selector() {
    assert_eq!(
        rsass(
            ".baz .foo {a: b}\
            \nfoo + > > + bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .foo, .baz foo + > > + bar, foo .baz + > > + bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/130_test_nested_extender_with_hacky_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t130_test_nested_extender_with_hacky_selector() {
    assert_eq!(
        rsass(
            ".baz .foo {a: b}\
            \n> > bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .foo, > > .baz bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/131_test_nested_extender_merges_with_same_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t131_test_nested_extender_merges_with_same_selector() {
    assert_eq!(
        rsass(
            ".foo {\
            \n.bar {a: b}\
            \n.baz {@extend .bar} }\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/132_test_nested_extender_with_child_selector_merges_with_same_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t132_test_nested_extender_with_child_selector_merges_with_same_selector() {
    assert_eq!(
        rsass(
            ".foo > .bar .baz {a: b}\
            \n.foo > .bar .bang {@extend .baz}\
            \n"
        )
        .unwrap(),
        ".foo > .bar .baz, .foo > .bar .bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/133_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t133_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a > + x {a: b}\
            \n.b y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > + x, .a .b > + y, .b .a > + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/134_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t134_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a x {a: b}\
            \n.b > + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a x, .a .b > + y, .b .a > + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/135_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t135_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a > + x {a: b}\
            \n.b > + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > + x, .a .b > + y, .b .a > + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/136_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t136_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a ~ > + x {a: b}\
            \n.b > + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ > + x, .a .b ~ > + y, .b .a ~ > + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/137_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t137_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a + > x {a: b}\
            \n.b > + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + > x {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/138_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t138_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a + > x {a: b}\
            \n.b > + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + > x {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/139_test_combinator_unification_for_hacky_combinators.hrx"
#[test]
#[ignore] // unexepected error
fn t139_test_combinator_unification_for_hacky_combinators() {
    assert_eq!(
        rsass(
            ".a ~ > + .b > x {a: b}\
            \n.c > + .d > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ > + .b > x, .a .c ~ > + .d.b > y, .c .a ~ > + .d.b > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/140_test_combinator_unification_double_tilde.hrx"
#[test]
#[ignore] // unexepected error
fn t140_test_combinator_unification_double_tilde() {
    assert_eq!(
        rsass(
            ".a.b ~ x {a: b}\
            \n.a ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b ~ x, .a.b ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/141_test_combinator_unification_double_tilde.hrx"
#[test]
#[ignore] // unexepected error
fn t141_test_combinator_unification_double_tilde() {
    assert_eq!(
        rsass(
            ".a ~ x {a: b}\
            \n.a.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ x, .a.b ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/142_test_combinator_unification_double_tilde.hrx"
#[test]
#[ignore] // unexepected error
fn t142_test_combinator_unification_double_tilde() {
    assert_eq!(
        rsass(
            ".a ~ x {a: b}\
            \n.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ x, .a ~ .b ~ y, .b ~ .a ~ y, .b.a ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/143_test_combinator_unification_double_tilde.hrx"
#[test]
#[ignore] // unexepected error
fn t143_test_combinator_unification_double_tilde() {
    assert_eq!(
        rsass(
            "a.a ~ x {a: b}\
            \nb.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a ~ x, a.a ~ b.b ~ y, b.b ~ a.a ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/144_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t144_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            ".a.b + x {a: b}\
            \n.a ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b + x, .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/145_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t145_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.a.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .a.b ~ .a + y, .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/146_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t146_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .b ~ .a + y, .b.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/147_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t147_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            "a.a + x {a: b}\
            \nb.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a + x, b.b ~ a.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/148_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t148_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            ".a.b ~ x {a: b}\
            \n.a + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b ~ x, .a.b ~ .a + y, .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/149_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t149_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            ".a ~ x {a: b}\
            \n.a.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ x, .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/150_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t150_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            ".a ~ x {a: b}\
            \n.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ x, .a ~ .b + y, .b.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/151_test_combinator_unification_tilde_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t151_test_combinator_unification_tilde_plus() {
    assert_eq!(
        rsass(
            "a.a ~ x {a: b}\
            \nb.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a ~ x, a.a ~ b.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/152_test_combinator_unification_angle_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t152_test_combinator_unification_angle_sibling() {
    assert_eq!(
        rsass(
            ".a > x {a: b}\
            \n.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .a > .b ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/153_test_combinator_unification_angle_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t153_test_combinator_unification_angle_sibling() {
    assert_eq!(
        rsass(
            ".a > x {a: b}\
            \n.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .a > .b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/154_test_combinator_unification_angle_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t154_test_combinator_unification_angle_sibling() {
    assert_eq!(
        rsass(
            ".a ~ x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ x, .b > .a ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/155_test_combinator_unification_angle_sibling.hrx"
#[test]
#[ignore] // unexepected error
fn t155_test_combinator_unification_angle_sibling() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .b > .a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/156_test_combinator_unification_double_angle.hrx"
#[test]
#[ignore] // unexepected error
fn t156_test_combinator_unification_double_angle() {
    assert_eq!(
        rsass(
            ".a.b > x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b > x, .b.a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/157_test_combinator_unification_double_angle.hrx"
#[test]
#[ignore] // unexepected error
fn t157_test_combinator_unification_double_angle() {
    assert_eq!(
        rsass(
            ".a > x {a: b}\
            \n.a.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .a.b > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/158_test_combinator_unification_double_angle.hrx"
#[test]
#[ignore] // unexepected error
fn t158_test_combinator_unification_double_angle() {
    assert_eq!(
        rsass(
            ".a > x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .b.a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/159_test_combinator_unification_double_angle.hrx"
#[test]
#[ignore] // unexepected error
fn t159_test_combinator_unification_double_angle() {
    assert_eq!(
        rsass(
            "a.a > x {a: b}\
            \nb.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a > x {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/160_test_combinator_unification_double_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t160_test_combinator_unification_double_plus() {
    assert_eq!(
        rsass(
            ".a.b + x {a: b}\
            \n.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b + x, .b.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/161_test_combinator_unification_double_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t161_test_combinator_unification_double_plus() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.a.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/162_test_combinator_unification_double_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t162_test_combinator_unification_double_plus() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .b.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/163_test_combinator_unification_double_plus.hrx"
#[test]
#[ignore] // unexepected error
fn t163_test_combinator_unification_double_plus() {
    assert_eq!(
        rsass(
            "a.a + x {a: b}\
            \nb.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a + x {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/164_test_combinator_unification_angle_space.hrx"
#[test]
#[ignore] // unexepected error
fn t164_test_combinator_unification_angle_space() {
    assert_eq!(
        rsass(
            ".a.b > x {a: b}\
            \n.a y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b > x, .a.b > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/165_test_combinator_unification_angle_space.hrx"
#[test]
#[ignore] // unexepected error
fn t165_test_combinator_unification_angle_space() {
    assert_eq!(
        rsass(
            ".a > x {a: b}\
            \n.a.b y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .a.b .a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/166_test_combinator_unification_angle_space.hrx"
#[test]
#[ignore] // unexepected error
fn t166_test_combinator_unification_angle_space() {
    assert_eq!(
        rsass(
            ".a > x {a: b}\
            \n.b y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .b .a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/167_test_combinator_unification_angle_space.hrx"
#[test]
#[ignore] // unexepected error
fn t167_test_combinator_unification_angle_space() {
    assert_eq!(
        rsass(
            ".a.b x {a: b}\
            \n.a > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b x, .a.b .a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/168_test_combinator_unification_angle_space.hrx"
#[test]
#[ignore] // unexepected error
fn t168_test_combinator_unification_angle_space() {
    assert_eq!(
        rsass(
            ".a x {a: b}\
            \n.a.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a x, .a.b > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/169_test_combinator_unification_angle_space.hrx"
#[test]
#[ignore] // unexepected error
fn t169_test_combinator_unification_angle_space() {
    assert_eq!(
        rsass(
            ".a x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a x, .a .b > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/170_test_combinator_unification_plus_space.hrx"
#[test]
#[ignore] // unexepected error
fn t170_test_combinator_unification_plus_space() {
    assert_eq!(
        rsass(
            ".a.b + x {a: b}\
            \n.a y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b + x, .a .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/171_test_combinator_unification_plus_space.hrx"
#[test]
#[ignore] // unexepected error
fn t171_test_combinator_unification_plus_space() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.a.b y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .a.b .a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/172_test_combinator_unification_plus_space.hrx"
#[test]
#[ignore] // unexepected error
fn t172_test_combinator_unification_plus_space() {
    assert_eq!(
        rsass(
            ".a + x {a: b}\
            \n.b y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a + x, .b .a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/173_test_combinator_unification_plus_space.hrx"
#[test]
#[ignore] // unexepected error
fn t173_test_combinator_unification_plus_space() {
    assert_eq!(
        rsass(
            ".a.b x {a: b}\
            \n.a + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b x, .a.b .a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/174_test_combinator_unification_plus_space.hrx"
#[test]
#[ignore] // unexepected error
fn t174_test_combinator_unification_plus_space() {
    assert_eq!(
        rsass(
            ".a x {a: b}\
            \n.a.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a x, .a .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/175_test_combinator_unification_plus_space.hrx"
#[test]
#[ignore] // unexepected error
fn t175_test_combinator_unification_plus_space() {
    assert_eq!(
        rsass(
            ".a x {a: b}\
            \n.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a x, .a .b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/176_test_combinator_unification_nested.hrx"
#[test]
#[ignore] // unexepected error
fn t176_test_combinator_unification_nested() {
    assert_eq!(
        rsass(
            ".a > .b + x {a: b}\
            \n.c > .d + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > .b + x, .c.a > .d.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/177_test_combinator_unification_nested.hrx"
#[test]
#[ignore] // unexepected error
fn t177_test_combinator_unification_nested() {
    assert_eq!(
        rsass(
            ".a > .b + x {a: b}\
            \n.c > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > .b + x, .c.a > .b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/178_test_combinator_unification_with_newlines.hrx"
#[test]
#[ignore] // unexepected error
fn t178_test_combinator_unification_with_newlines() {
    assert_eq!(
        rsass(
            ".a >\
            \n.b\
            \n+ x {a: b}\
            \n.c\
            \n> .d +\
            \ny {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > .b + x, .c.a > .d.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/179_test_extend_self_loop.hrx"
#[test]
#[ignore] // unexepected error
fn t179_test_extend_self_loop() {
    assert_eq!(
        rsass(
            ".foo {a: b; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/180_test_basic_extend_loop.hrx"
#[test]
#[ignore] // unexepected error
fn t180_test_basic_extend_loop() {
    assert_eq!(
        rsass(
            ".foo {a: b; @extend .bar}\
            \n.bar {c: d; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.bar, .foo {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/181_test_three_level_extend_loop.hrx"
#[test]
#[ignore] // unexepected error
fn t181_test_three_level_extend_loop() {
    assert_eq!(
        rsass(
            ".foo {a: b; @extend .bar}\
            \n.bar {c: d; @extend .baz}\
            \n.baz {e: f; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, .baz, .bar {\
        \n  a: b;\
        \n}\
        \n.bar, .foo, .baz {\
        \n  c: d;\
        \n}\
        \n.baz, .bar, .foo {\
        \n  e: f;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/182_test_nested_extend_loop.hrx"
#[test]
#[ignore] // unexepected error
fn t182_test_nested_extend_loop() {
    assert_eq!(
        rsass(
            ".bar {\
            \na: b;\
            \n.foo {c: d; @extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".bar, .bar .foo {\
        \n  a: b;\
        \n}\
        \n.bar .foo {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/183_test_multiple_extender_merges_with_superset_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t183_test_multiple_extender_merges_with_superset_selector() {
    assert_eq!(
        rsass(
            ".foo {@extend .bar; @extend .baz}\
            \na.bar.baz {a: b}\
            \n"
        )
        .unwrap(),
        "a.bar.baz, a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/184_test_control_flow_if.hrx"
#[test]
#[ignore] // wrong result
fn t184_test_control_flow_if() {
    assert_eq!(
        rsass(
            ".true  { color: green; }\
            \n.false { color: red;   }\
            \n.also-true {\
            \n@if true { @extend .true;  }\
            \n@else    { @extend .false; }\
            \n}\
            \n.also-false {\
            \n@if false { @extend .true;  }\
            \n@else     { @extend .false; }\
            \n}\
            \n"
        )
        .unwrap(),
        ".true, .also-true {\
        \n  color: green;\
        \n}\
        \n.false, .also-false {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/185_test_control_flow_for.hrx"
#[test]
#[ignore] // wrong result
fn t185_test_control_flow_for() {
    assert_eq!(
        rsass(
            ".base-0  { color: green; }\
            \n.base-1  { display: block; }\
            \n.base-2  { border: 1px solid blue; }\
            \n.added {\
            \n@for $i from 0 to 3 {\
            \n  @extend .base-#{$i};\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        ".base-0, .added {\
        \n  color: green;\
        \n}\
        \n.base-1, .added {\
        \n  display: block;\
        \n}\
        \n.base-2, .added {\
        \n  border: 1px solid blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/186_test_control_flow_while.hrx"
#[test]
#[ignore] // wrong result
fn t186_test_control_flow_while() {
    assert_eq!(
        rsass(
            ".base-0  { color: green; }\
            \n.base-1  { display: block; }\
            \n.base-2  { border: 1px solid blue; }\
            \n.added {\
            \n$i : 0;\
            \n@while $i < 3 {\
            \n  @extend .base-#{$i};\
            \n  $i : $i + 1;\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        ".base-0, .added {\
        \n  color: green;\
        \n}\
        \n.base-1, .added {\
        \n  display: block;\
        \n}\
        \n.base-2, .added {\
        \n  border: 1px solid blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/187_test_basic_placeholder_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t187_test_basic_placeholder_selector() {
    assert_eq!(
        rsass(
            "%foo {a: b}\
            \n.bar {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/188_test_unused_placeholder_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t188_test_unused_placeholder_selector() {
    assert_eq!(
        rsass(
            "%foo {color: blue}\
            \n%bar {color: red}\
            \n.baz {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".baz {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/189_test_placeholder_descendant_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t189_test_placeholder_descendant_selector() {
    assert_eq!(
        rsass(
            "#context %foo a {a: b}\
            \n.bar {@extend %foo}\
            \n"
        )
        .unwrap(),
        "#context .bar a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/190_test_semi_placeholder_selector.hrx"
#[test]
#[ignore] // wrong result
fn t190_test_semi_placeholder_selector() {
    assert_eq!(
        rsass(
            "#context %foo, .bar .baz {color: blue}\
            \n\
            \n.bat {\
            \n  @extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "#context .bat, .bar .baz {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/191_test_placeholder_selector_with_multiple_extenders.hrx"
#[test]
#[ignore] // unexepected error
fn t191_test_placeholder_selector_with_multiple_extenders() {
    assert_eq!(
        rsass(
            "%foo {color: blue}\
            \n.bar {@extend %foo}\
            \n.baz {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".baz, .bar {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/192_test_placeholder_interpolation.hrx"
#[test]
#[ignore] // unexepected error
fn t192_test_placeholder_interpolation() {
    assert_eq!(
        rsass(
            "$foo: foo;\
            \n\
            \n%#{$foo} {color: blue}\
            \n.bar {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".bar {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/193_test_media_in_placeholder_selector.hrx"
#[test]
#[ignore] // wrong result
fn t193_test_media_in_placeholder_selector() {
    assert_eq!(
        rsass(
            "%foo {bar {@media screen {a {b: c}}}}\
            \n.baz {c: d}\
            \n"
        )
        .unwrap(),
        ".baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/194_test_extend_within_media.hrx"
#[test]
#[ignore] // unexepected error
fn t194_test_extend_within_media() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n.foo {a: b}\
            \n.bar {@extend .foo}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/195_test_extend_within_unknown_directive.hrx"
#[test]
#[ignore] // unexepected error
fn t195_test_extend_within_unknown_directive() {
    assert_eq!(
        rsass(
            "@flooblehoof {\
            \n.foo {a: b}\
            \n.bar {@extend .foo}\
            \n}\
            \n"
        )
        .unwrap(),
        "@flooblehoof {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/196_test_extend_within_nested_directives.hrx"
#[test]
#[ignore] // unexepected error
fn t196_test_extend_within_nested_directives() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n@flooblehoof {\
            \n  .foo {a: b}\
            \n  .bar {@extend .foo}\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  @flooblehoof {\
        \n    .foo, .bar {\
        \n      a: b;\
        \n    }\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/197_test_extend_within_disparate_media.hrx"
#[test]
#[ignore] // unexepected error
fn t197_test_extend_within_disparate_media() {
    assert_eq!(
        rsass(
            "@media screen {.foo {a: b}}\
            \n@media screen {.bar {@extend .foo}}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/198_test_extend_within_disparate_unknown_directive.hrx"
#[test]
#[ignore] // unexepected error
fn t198_test_extend_within_disparate_unknown_directive() {
    assert_eq!(
        rsass(
            "@flooblehoof {.foo {a: b}}\
            \n@flooblehoof {.bar {@extend .foo}}\
            \n"
        )
        .unwrap(),
        "@flooblehoof {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n@flooblehoof {}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/199_test_extend_within_disparate_nested_directives.hrx"
#[test]
#[ignore] // unexepected error
fn t199_test_extend_within_disparate_nested_directives() {
    assert_eq!(
        rsass(
            "@media screen {@flooblehoof {.foo {a: b}}}\
            \n@media screen {@flooblehoof {.bar {@extend .foo}}}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  @flooblehoof {\
        \n    .foo, .bar {\
        \n      a: b;\
        \n    }\
        \n  }\
        \n}\
        \n@media screen {\
        \n  @flooblehoof {}\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/206_test_extend_succeeds_when_one_extension_fails_but_others_dont.hrx"
#[test]
#[ignore] // unexepected error
fn t206_test_extend_succeeds_when_one_extension_fails_but_others_dont() {
    assert_eq!(
        rsass(
            "a.bar {a: b}\
            \n.bar {c: d}\
            \nb.foo {@extend .bar}\
            \n"
        )
        .unwrap(),
        "a.bar {\
        \n  a: b;\
        \n}\
        \n.bar, b.foo {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/207_test_optional_extend_succeeds_when_extendee_doesnt_exist.hrx"
#[test]
#[ignore] // unexepected error
fn t207_test_optional_extend_succeeds_when_extendee_doesnt_exist() {
    assert_eq!(
        rsass(
            ".foo {@extend .bar !optional}\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/208_test_optional_extend_succeeds_when_extension_fails.hrx"
#[test]
#[ignore] // unexepected error
fn t208_test_optional_extend_succeeds_when_extension_fails() {
    assert_eq!(
        rsass(
            "a.bar {a: b}\
            \nb.foo {@extend .bar !optional}\
            \n"
        )
        .unwrap(),
        "a.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/209_test_pseudo_element_superselector.hrx"
#[test]
#[ignore] // unexepected error
fn t209_test_pseudo_element_superselector() {
    assert_eq!(
        rsass(
            "%x#bar {a: b} // Add an id to make the results have high specificity\
            \n%y, %y::fblthp {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar::fblthp {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/210_test_pseudo_element_superselector.hrx"
#[test]
#[ignore] // unexepected error
fn t210_test_pseudo_element_superselector() {
    assert_eq!(
        rsass(
            "%x#bar {a: b}\
            \n%y, %y:fblthp {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/211_test_pseudo_element_superselector.hrx"
#[test]
#[ignore] // unexepected error
fn t211_test_pseudo_element_superselector() {
    assert_eq!(
        rsass(
            "%x#bar {a: b}\
            \n%y, %y:first-line {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar:first-line {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/212_test_pseudo_element_superselector.hrx"
#[test]
#[ignore] // unexepected error
fn t212_test_pseudo_element_superselector() {
    assert_eq!(
        rsass(
            "%x#bar {a: b}\
            \n%y, %y:first-letter {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar:first-letter {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/213_test_pseudo_element_superselector.hrx"
#[test]
#[ignore] // unexepected error
fn t213_test_pseudo_element_superselector() {
    assert_eq!(
        rsass(
            "%x#bar {a: b}\
            \n%y, %y:before {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar:before {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/214_test_pseudo_element_superselector.hrx"
#[test]
#[ignore] // unexepected error
fn t214_test_pseudo_element_superselector() {
    assert_eq!(
        rsass(
            "%x#bar {a: b}\
            \n%y, %y:after {@extend %x}\
            \na {@extend %y}\
            \n"
        )
        .unwrap(),
        "a#bar, a#bar:after {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/215_test_multiple_source_redundancy_elimination.hrx"
#[test]
#[ignore] // unexepected error
fn t215_test_multiple_source_redundancy_elimination() {
    assert_eq!(
        rsass(
            "%default-color {color: red}\
            \n%alt-color {color: green}\
            \n\
            \n%default-style {\
            \n@extend %default-color;\
            \n&:hover {@extend %alt-color}\
            \n&:active {@extend %default-color}\
            \n}\
            \n\
            \n.test-case {@extend %default-style}\
            \n"
        )
        .unwrap(),
        ".test-case:active, .test-case {\
        \n  color: red;\
        \n}\
        \n.test-case:hover {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/216_test_nested_sibling_extend.hrx"
#[test]
#[ignore] // unexepected error
fn t216_test_nested_sibling_extend() {
    assert_eq!(
        rsass(
            ".foo {@extend .bar}\
            \n\
            \n.parent {\
            \n.bar {\
            \n  width: 2000px;\
            \n}\
            \n.foo {\
            \n  @extend .bar\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent .bar, .parent .foo {\
        \n  width: 2000px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/217_test_parent_and_sibling_extend.hrx"
#[test]
#[ignore] // unexepected error
fn t217_test_parent_and_sibling_extend() {
    assert_eq!(
        rsass(
            "%foo %bar%baz {c: d}\
            \n\
            \n.parent1 {\
            \n@extend %foo;\
            \n.child1 {@extend %bar}\
            \n}\
            \n\
            \n.parent2 {\
            \n@extend %foo;\
            \n.child2 {@extend %baz}\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent1 .parent2 .child1.child2, .parent2 .parent1 .child1.child2 {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/218_test_nested_extend_specificity.hrx"
#[test]
#[ignore] // unexepected error
fn t218_test_nested_extend_specificity() {
    assert_eq!(
        rsass(
            "%foo {a: b}\
            \n\
            \na {\
            \n:b {@extend %foo}\
            \n:b:c {@extend %foo}\
            \n}\
            \n"
        )
        .unwrap(),
        "a :b:c, a :b {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/219_test_nested_double_extend_optimization.hrx"
#[test]
#[ignore] // wrong result
fn t219_test_nested_double_extend_optimization() {
    assert_eq!(
        rsass(
            "%foo %bar {\
            \na: b;\
            \n}\
            \n\
            \n.parent1 {\
            \n@extend %foo;\
            \n\
            \n.child {\
            \n  @extend %bar;\
            \n}\
            \n}\
            \n\
            \n.parent2 {\
            \n@extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent1 .child {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/220_test_extend_in_double_nested_media_query.hrx"
#[test]
#[ignore] // unexepected error
fn t220_test_extend_in_double_nested_media_query() {
    assert_eq!(
        rsass(
            "@media all {\
            \n@media (orientation: landscape) {\
            \n  %foo {color: blue}\
            \n  .bar {@extend %foo}\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (orientation: landscape) {\
        \n  .bar {\
        \n    color: blue;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/221_test_partially_failed_extend.hrx"
#[test]
#[ignore] // wrong result
fn t221_test_partially_failed_extend() {
    assert_eq!(
        rsass(
            "test { @extend .rc; }\
            \n.rc {color: white;}\
            \n.prices span.pill span.rc {color: red;}\
            \n"
        )
        .unwrap(),
        ".rc, test {\
        \n  color: white;\
        \n}\
        \n.prices span.pill span.rc {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/222_test_newline_near_combinator.hrx"
#[test]
#[ignore] // unexepected error
fn t222_test_newline_near_combinator() {
    assert_eq!(
        rsass(
            ".a +\
            \n.b x {a: b}\
            \n.c y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a +\
        \n.b x, .a +\
        \n.b .c y, .c .a +\
        \n.b y {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/223_test_duplicated_selector_with_newlines.hrx"
#[test]
#[ignore] // unexepected error
fn t223_test_duplicated_selector_with_newlines() {
    assert_eq!(
        rsass(
            ".example-1-1,\
            \n.example-1-2,\
            \n.example-1-3 {\
            \na: b;\
            \n}\
            \n\
            \n.my-page-1 .my-module-1-1 {@extend .example-1-2}\
            \n"
        )
        .unwrap(),
        ".example-1-1,\
        \n.example-1-2,\
        \n.my-page-1 .my-module-1-1,\
        \n.example-1-3 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/224_test_nested_selector_with_child_selector_hack_extendee.hrx"
#[test]
#[ignore] // unexepected error
fn t224_test_nested_selector_with_child_selector_hack_extendee() {
    assert_eq!(
        rsass(
            "> .foo {a: b}\
            \nfoo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        "> .foo, > foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/225_test_nested_selector_with_child_selector_hack_extender.hrx"
#[test]
#[ignore] // unexepected error
fn t225_test_nested_selector_with_child_selector_hack_extender() {
    assert_eq!(
        rsass(
            ".foo .bar {a: b}\
            \n> foo bar {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, > .foo foo bar, > foo .foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/226_test_nested_selector_with_child_selector_hack_extender_and_extendee.hrx"
#[test]
#[ignore] // unexepected error
fn t226_test_nested_selector_with_child_selector_hack_extender_and_extendee()
{
    assert_eq!(
        rsass(
            "> .foo {a: b}\
            \n> foo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        "> .foo, > foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/227_test_nested_with_child_hack_extender_and_sibling_extendee.hrx"
#[test]
#[ignore] // unexepected error
fn t227_test_nested_with_child_hack_extender_and_sibling_extendee() {
    assert_eq!(
        rsass(
            "~ .foo {a: b}\
            \n> foo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        "~ .foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/228_test_nested_with_child_selector_hack_extender_extendee_newline.hrx"
#[test]
#[ignore] // unexepected error
fn t228_test_nested_with_child_selector_hack_extender_extendee_newline() {
    assert_eq!(
        rsass(
            "> .foo {a: b}\
            \nflip,\
            \n> foo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        "> .foo, > flip,\
        \n> foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/229_test_extended_parent_and_child_redundancy_elimination.hrx"
#[test]
#[ignore] // unexepected error
fn t229_test_extended_parent_and_child_redundancy_elimination() {
    assert_eq!(
        rsass(
            "a {\
            \nb {a: b}\
            \nc {@extend b}\
            \n}\
            \nd {@extend a}\
            \n"
        )
        .unwrap(),
        "a b, d b, a c, d c {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/230_test_extend_redundancy_elimination_when_it_would_reduce_specificity.hrx"
#[test]
#[ignore] // unexepected error
fn t230_test_extend_redundancy_elimination_when_it_would_reduce_specificity()
{
    assert_eq!(
        rsass(
            "a {a: b}\
            \na.foo {@extend a}\
            \n"
        )
        .unwrap(),
        "a, a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/231_test_extend_redundancy_elimination_when_it_would_preserve_specificity.hrx"
#[test]
#[ignore] // unexepected error
fn t231_test_extend_redundancy_elimination_when_it_would_preserve_specificity(
) {
    assert_eq!(
        rsass(
            ".bar a {a: b}\
            \na.foo {@extend a}\
            \n"
        )
        .unwrap(),
        ".bar a {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/232_test_extend_redundancy_elimination_never_eliminates_base_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t232_test_extend_redundancy_elimination_never_eliminates_base_selector() {
    assert_eq!(
        rsass(
            "a.foo {a: b}\
            \n.foo {@extend a}\
            \n"
        )
        .unwrap(),
        "a.foo, .foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/233_test_extend_cross_branch_redundancy_elimination.hrx"
#[test]
#[ignore] // unexepected error
fn t233_test_extend_cross_branch_redundancy_elimination() {
    assert_eq!(
        rsass(
            "%x .c %y {a: b}\
            \n.a, .b {@extend %x}\
            \n.a .d {@extend %y}\
            \n"
        )
        .unwrap(),
        ".a .c .d, .b .c .a .d {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/234_test_extend_cross_branch_redundancy_elimination.hrx"
#[test]
#[ignore] // unexepected error
fn t234_test_extend_cross_branch_redundancy_elimination() {
    assert_eq!(
        rsass(
            ".e %z {a: b}\
            \n%x .c %y {@extend %z}\
            \n.a, .b {@extend %x}\
            \n.a .d {@extend %y}\
            \n"
        )
        .unwrap(),
        ".e .a .c .d, .e .b .c .a .d, .a .e .b .c .d, .a .c .e .d, .b .c .e .a .d {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/235_extend_with_universal_selector.hrx"
#[test]
#[ignore] // unexepected error
fn t235_extend_with_universal_selector() {
    assert_eq!(
        rsass(
            "%-a *.foo1 {a: b}\
            \na {@extend .foo1}\
            \n-a {@extend %-a}\
            \n\
            \n%-b *|*.foo2 {b: b}\
            \nb {@extend .foo2}\
            \n-b {@extend %-b}\
            \n"
        )
        .unwrap(),
        "-a *.foo1, -a a {\
        \n  a: b;\
        \n}\
        \n-b *|*.foo2, -b b {\
        \n  b: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/236_extend_with_universal_selector_empty_namespace.hrx"
#[test]
#[ignore] // unexepected error
fn t236_extend_with_universal_selector_empty_namespace() {
    assert_eq!(
        rsass(
            "%-a |*.foo {a: b}\
            \na {@extend .foo}\
            \n-a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a |*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/237_extend_with_universal_selector_different_namespace.hrx"
#[test]
#[ignore] // unexepected error
fn t237_extend_with_universal_selector_different_namespace() {
    assert_eq!(
        rsass(
            "%-a ns|*.foo {a: b}\
            \na {@extend .foo}\
            \n-a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/238_unify_root_pseudoelement.hrx"
#[test]
#[ignore] // unexepected error
fn t238_unify_root_pseudoelement() {
    assert_eq!(
        rsass(
            "// We assume that by default classes don\'t apply to the :root unless marked explicitly.\
            \n:root .foo-1 { test: 1; }\
            \n.bar-1 .baz-1 { @extend .foo-1; }\
            \n\
            \n// We know the two classes must be the same :root element so we can combine them.\
            \n.foo-2:root .bar-2 { test: 2; }\
            \n.baz-2:root .bang-2 { @extend .bar-2; }\
            \n\
            \n// This extend should not apply because the :root elements are different.\
            \nhtml:root .bar-3 { test: 3; }\
            \nxml:root .bang-3 { @extend .bar-3}\
            \n\
            \n// We assume that direct descendant of the :root is not the same element as a descendant.\
            \n.foo-4:root > .bar-4 .x-4 { test: 4; }\
            \n.baz-4:root .bang-4 .y-4 {@extend .x-4}\
            \n"
        )
        .unwrap(),
        ":root .foo-1, :root .bar-1 .baz-1 {\
        \n  test: 1;\
        \n}\
        \n.foo-2:root .bar-2, .baz-2.foo-2:root .bang-2 {\
        \n  test: 2;\
        \n}\
        \nhtml:root .bar-3 {\
        \n  test: 3;\
        \n}\
        \n.foo-4:root > .bar-4 .x-4, .baz-4.foo-4:root > .bar-4 .bang-4 .y-4 {\
        \n  test: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/compound-unification-in-not.hrx"
#[test]
#[ignore] // unexepected error
fn compound_unification_in_not() {
    assert_eq!(
        rsass(
            "// Make sure compound selectors are unified when two :not()s are extended.\
            \n// :not() is special here because it\'s the only selector that\'s extended by\
            \n// adding to the compound selector, rather than creating a new selector list.\
            \n.a {@extend .c}\
            \n.b {@extend .d}\
            \n:not(.c):not(.d) {x: y}\
            \n"
        )
        .unwrap(),
        ":not(.c):not(.a):not(.d):not(.b) {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/does_not_move_page_block_in_media.hrx"
#[test]
#[ignore] // wrong result
fn does_not_move_page_block_in_media() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n  a { x:y; }\
            \n  @page {}\
            \n}"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    x: y;\
        \n  }\
        \n  @page {}\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/escaped_selector.hrx"
#[test]
#[ignore] // unexepected error
fn escaped_selector() {
    assert_eq!(
        rsass(
            "// Escapes in selectors\' identifiers should be normalized before `@extend` is\
            \n// applied.\
            \n.foo {escape: none}\
            \n\\.foo {escape: slash dot}\
            \n\\2E foo {escape: hex}\
            \n\
            \n.bar {@extend \\02e foo}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  escape: none;\
        \n}\
        \n\\.foo, .bar {\
        \n  escape: slash dot;\
        \n}\
        \n\\.foo, .bar {\
        \n  escape: hex;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/extend-extender.hrx"
#[test]
#[ignore] // unexepected error
fn extend_extender() {
    assert_eq!(
        rsass(
            "// For implementations like Dart Sass that process extensions as they occur,\
            \n// extending rules that contain their own extends needs special handling.\
            \n.b {@extend .a}\
            \n.c {@extend .b}\
            \n.a {x: y}\
            \n"
        )
        .unwrap(),
        ".a, .b, .c {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/extend-loop.hrx"
#[test]
#[ignore] // unexepected error
fn extend_loop() {
    assert_eq!(
        rsass(
            "// Make sure extend loops are handled correctly. Test in all different orderings\
            \n// so we can be sure this works for implementations like Dart Sass where extend\
            \n// order matters.\
            \n\
            \n@media order1 {\
            \n  .x1.y1.a1 {x: y; @extend .b1}\
            \n  .z1.b1 {x: y; @extend .c1}\
            \n  .c1 {x: y; @extend .a1}\
            \n}\
            \n\
            \n@media order2 {\
            \n  .x2.y2.a2 {x: y; @extend .b2}\
            \n  .c2 {x: y; @extend .a2}\
            \n  .z2.b2 {x: y; @extend .c2}\
            \n}\
            \n\
            \n@media order3 {\
            \n  .z3.b3 {x: y; @extend .c3}\
            \n  .x3.y3.a3 {x: y; @extend .b3}\
            \n  .c3 {x: y; @extend .a3}\
            \n}\
            \n\
            \n@media order4 {\
            \n  .z4.b4 {x: y; @extend .c4}\
            \n  .c4 {x: y; @extend .a4}\
            \n  .x4.y4.a4 {x: y; @extend .b4}\
            \n}\
            \n\
            \n@media order5 {\
            \n  .c5 {x: y; @extend .a5}\
            \n  .z5.b5 {x: y; @extend .c5}\
            \n  .x5.y5.a5 {x: y; @extend .b5}\
            \n}\
            \n\
            \n@media order6 {\
            \n  .c6 {x: y; @extend .a6}\
            \n  .x6.y6.a6 {x: y; @extend .b6}\
            \n  .z6.b6 {x: y; @extend .c6}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media order1 {\
        \n  .x1.y1.a1, .x1.y1.c1, .x1.y1.z1.b1 {\
        \n    x: y;\
        \n  }\
        \n  .z1.b1, .z1.x1.y1.a1, .z1.x1.y1.c1, .z1.x1.y1.b1 {\
        \n    x: y;\
        \n  }\
        \n  .c1, .z1.b1, .z1.x1.y1.a1, .z1.x1.y1.c1, .z1.x1.y1.b1 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order2 {\
        \n  .x2.y2.a2, .x2.y2.c2, .x2.y2.z2.b2 {\
        \n    x: y;\
        \n  }\
        \n  .c2, .z2.b2, .z2.x2.y2.a2, .z2.x2.y2.c2, .z2.x2.y2.b2 {\
        \n    x: y;\
        \n  }\
        \n  .z2.b2, .z2.x2.y2.a2, .z2.x2.y2.c2, .z2.x2.y2.b2 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order3 {\
        \n  .z3.b3, .z3.x3.y3.a3, .z3.x3.y3.c3, .z3.x3.y3.b3 {\
        \n    x: y;\
        \n  }\
        \n  .x3.y3.a3, .x3.y3.c3, .x3.y3.z3.b3 {\
        \n    x: y;\
        \n  }\
        \n  .c3, .z3.b3, .z3.x3.y3.a3, .z3.x3.y3.c3, .z3.x3.y3.b3 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order4 {\
        \n  .z4.b4, .z4.x4.y4.a4, .z4.x4.y4.c4, .z4.x4.y4.b4 {\
        \n    x: y;\
        \n  }\
        \n  .c4, .z4.b4, .z4.x4.y4.a4, .z4.x4.y4.c4, .z4.x4.y4.b4 {\
        \n    x: y;\
        \n  }\
        \n  .x4.y4.a4, .x4.y4.c4, .x4.y4.z4.b4 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order5 {\
        \n  .c5, .z5.b5, .z5.x5.y5.a5, .z5.x5.y5.c5, .z5.x5.y5.b5 {\
        \n    x: y;\
        \n  }\
        \n  .z5.b5, .z5.x5.y5.a5, .z5.x5.y5.c5, .z5.x5.y5.b5 {\
        \n    x: y;\
        \n  }\
        \n  .x5.y5.a5, .x5.y5.c5, .x5.y5.z5.b5 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order6 {\
        \n  .c6, .z6.b6, .z6.x6.y6.a6, .z6.x6.y6.c6, .z6.x6.y6.b6 {\
        \n    x: y;\
        \n  }\
        \n  .x6.y6.a6, .x6.y6.c6, .x6.y6.z6.b6 {\
        \n    x: y;\
        \n  }\
        \n  .z6.b6, .z6.x6.y6.a6, .z6.x6.y6.c6, .z6.x6.y6.b6 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/extend-result-of-extend.hrx"
#[test]
#[ignore] // unexepected error
fn extend_result_of_extend() {
    assert_eq!(
        rsass(
            "// The result of :not(.c) being extended should itself be extendable.\
            \n.a {@extend :not(.b)}\
            \n.b {@extend .c}\
            \n:not(.c) {x: y}\
            \n"
        )
        .unwrap(),
        ":not(.c):not(.b), .a:not(.c) {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/extend-self.hrx"
#[test]
#[ignore] // unexepected error
fn extend_self() {
    assert_eq!(
        rsass(
            "// This shouldn\'t change the selector.\
            \n.c, .a .b .c, .a .c .b {x: y; @extend .c}\
            \n"
        )
        .unwrap(),
        ".c, .a .b .c, .a .c .b {\
        \n  x: y;\
        \n}\
        \n"
    );
}

mod fake_pseudo_element_order;

// From "sass-spec/spec/non_conformant/extend-tests/issue_146.hrx"
#[test]
#[ignore] // wrong result
fn issue_146() {
    assert_eq!(
        rsass(
            "%btn-style-default {\
            \n  background: green;\
            \n  &:hover{\
            \n    background: black;\
            \n  }\
            \n}\
            \n\
            \nbutton {\
            \n  @extend %btn-style-default;\
            \n}"
        )
        .unwrap(),
        "button {\
        \n  background: green;\
        \n}\
        \nbutton:hover {\
        \n  background: black;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/nested-compound-unification.hrx"
#[test]
#[ignore] // unexepected error
fn nested_compound_unification() {
    assert_eq!(
        rsass(
            "// Make sure compound unification properly handles weaving together parent\
            \n// selectors.\
            \n.a .b {@extend .e}\
            \n.c .d {@extend .f}\
            \n.e.f {x: y}\
            \n"
        )
        .unwrap(),
        ".e.f, .a .f.b, .c .e.d, .a .c .b.d, .c .a .b.d {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/not-into-not-not.hrx"
#[test]
#[ignore] // unexepected error
fn not_into_not_not() {
    assert_eq!(
        rsass(
            "// Regression test for dart-sass#191.\
            \n:not(:not(.x)) {a: b}\
            \n:not(.y) {@extend .x}\
            \n"
        )
        .unwrap(),
        ":not(:not(.x)) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/selector_list.hrx"
#[test]
#[ignore] // unexepected error
fn selector_list() {
    assert_eq!(
        rsass(
            ".foo {a: b}\
            \n.bar {x: y}\
            \n\
            \n// Extending a selector list is equivalent to writing two @extends.\
            \n.baz {@extend .foo, .bar}\
            \n\
            \n// The selector list should be parsed after interpolation is resolved.\
            \n.bang {@extend .foo #{\",\"} .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .bang, .baz {\
        \n  a: b;\
        \n}\
        \n.bar, .bang, .baz {\
        \n  x: y;\
        \n}\
        \n"
    );
}

mod subject_operator;
