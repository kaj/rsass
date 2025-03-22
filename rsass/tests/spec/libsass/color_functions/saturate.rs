//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/saturate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("saturate")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  c0: color.adjust(hsl(100, 0%, 50%), $saturation: 0%);\
             \n  c1: color.adjust(hsl(100, 0%, 50%), $saturation: 1%);\
             \n  c2: color.adjust(hsl(100, 0%, 50%), $saturation: 2%);\
             \n  c3: color.adjust(hsl(100, 0%, 50%), $saturation: 3%);\
             \n  c4: color.adjust(hsl(100, 0%, 50%), $saturation: 4%);\
             \n  c5: color.adjust(hsl(100, 0%, 50%), $saturation: 5%);\
             \n  c6: color.adjust(hsl(100, 0%, 50%), $saturation: 6%);\
             \n  c7: color.adjust(hsl(100, 0%, 50%), $saturation: 7%);\
             \n  c8: color.adjust(hsl(100, 0%, 50%), $saturation: 8%);\
             \n  c9: color.adjust(hsl(100, 0%, 50%), $saturation: 9%);\
             \n  c10: color.adjust(hsl(100, 0%, 50%), $saturation: 10%);\
             \n  c11: color.adjust(hsl(100, 0%, 50%), $saturation: 11%);\
             \n  c12: color.adjust(hsl(100, 0%, 50%), $saturation: 12%);\
             \n  c13: color.adjust(hsl(100, 0%, 50%), $saturation: 13%);\
             \n  c14: color.adjust(hsl(100, 0%, 50%), $saturation: 14%);\
             \n  c15: color.adjust(hsl(100, 0%, 50%), $saturation: 15%);\
             \n  c16: color.adjust(hsl(100, 0%, 50%), $saturation: 16%);\
             \n  c17: color.adjust(hsl(100, 0%, 50%), $saturation: 17%);\
             \n  c18: color.adjust(hsl(100, 0%, 50%), $saturation: 18%);\
             \n  c19: color.adjust(hsl(100, 0%, 50%), $saturation: 19%);\
             \n  c20: color.adjust(hsl(100, 0%, 50%), $saturation: 20%);\
             \n  c21: color.adjust(hsl(100, 0%, 50%), $saturation: 21%);\
             \n  c22: color.adjust(hsl(100, 0%, 50%), $saturation: 22%);\
             \n  c23: color.adjust(hsl(100, 0%, 50%), $saturation: 23%);\
             \n  c24: color.adjust(hsl(100, 0%, 50%), $saturation: 24%);\
             \n  c25: color.adjust(hsl(100, 0%, 50%), $saturation: 25%);\
             \n  c26: color.adjust(hsl(100, 0%, 50%), $saturation: 26%);\
             \n  c27: color.adjust(hsl(100, 0%, 50%), $saturation: 27%);\
             \n  c28: color.adjust(hsl(100, 0%, 50%), $saturation: 28%);\
             \n  c29: color.adjust(hsl(100, 0%, 50%), $saturation: 29%);\
             \n  c30: color.adjust(hsl(100, 0%, 50%), $saturation: 30%);\
             \n  c31: color.adjust(hsl(100, 0%, 50%), $saturation: 31%);\
             \n  c32: color.adjust(hsl(100, 0%, 50%), $saturation: 32%);\
             \n  c33: color.adjust(hsl(100, 0%, 50%), $saturation: 33%);\
             \n  c34: color.adjust(hsl(100, 0%, 50%), $saturation: 34%);\
             \n  c35: color.adjust(hsl(100, 0%, 50%), $saturation: 35%);\
             \n  c36: color.adjust(hsl(100, 0%, 50%), $saturation: 36%);\
             \n  c37: color.adjust(hsl(100, 0%, 50%), $saturation: 37%);\
             \n  c38: color.adjust(hsl(100, 0%, 50%), $saturation: 38%);\
             \n  c39: color.adjust(hsl(100, 0%, 50%), $saturation: 39%);\
             \n  // c40: saturate(hsl(100, 0%, 50%), 40%);\
             \n  c41: color.adjust(hsl(100, 0%, 50%), $saturation: 41%);\
             \n  c42: color.adjust(hsl(100, 0%, 50%), $saturation: 42%);\
             \n  c43: color.adjust(hsl(100, 0%, 50%), $saturation: 43%);\
             \n  c44: color.adjust(hsl(100, 0%, 50%), $saturation: 44%);\
             \n  c45: color.adjust(hsl(100, 0%, 50%), $saturation: 45%);\
             \n  c46: color.adjust(hsl(100, 0%, 50%), $saturation: 46%);\
             \n  c47: color.adjust(hsl(100, 0%, 50%), $saturation: 47%);\
             \n  c48: color.adjust(hsl(100, 0%, 50%), $saturation: 48%);\
             \n  c49: color.adjust(hsl(100, 0%, 50%), $saturation: 49%);\
             \n  c50: color.adjust(hsl(100, 0%, 50%), $saturation: 50%);\
             \n  c51: color.adjust(hsl(100, 0%, 50%), $saturation: 51%);\
             \n  c52: color.adjust(hsl(100, 0%, 50%), $saturation: 52%);\
             \n  c53: color.adjust(hsl(100, 0%, 50%), $saturation: 53%);\
             \n  c54: color.adjust(hsl(100, 0%, 50%), $saturation: 54%);\
             \n  c55: color.adjust(hsl(100, 0%, 50%), $saturation: 55%);\
             \n  c56: color.adjust(hsl(100, 0%, 50%), $saturation: 56%);\
             \n  c57: color.adjust(hsl(100, 0%, 50%), $saturation: 57%);\
             \n  c58: color.adjust(hsl(100, 0%, 50%), $saturation: 58%);\
             \n  c59: color.adjust(hsl(100, 0%, 50%), $saturation: 59%);\
             \n  c60: color.adjust(hsl(100, 0%, 50%), $saturation: 60%);\
             \n  c61: color.adjust(hsl(100, 0%, 50%), $saturation: 61%);\
             \n  c62: color.adjust(hsl(100, 0%, 50%), $saturation: 62%);\
             \n  c63: color.adjust(hsl(100, 0%, 50%), $saturation: 63%);\
             \n  c64: color.adjust(hsl(100, 0%, 50%), $saturation: 64%);\
             \n  c65: color.adjust(hsl(100, 0%, 50%), $saturation: 65%);\
             \n  c66: color.adjust(hsl(100, 0%, 50%), $saturation: 66%);\
             \n  c67: color.adjust(hsl(100, 0%, 50%), $saturation: 67%);\
             \n  c68: color.adjust(hsl(100, 0%, 50%), $saturation: 68%);\
             \n  c69: color.adjust(hsl(100, 0%, 50%), $saturation: 69%);\
             \n  c70: color.adjust(hsl(100, 0%, 50%), $saturation: 70%);\
             \n  c71: color.adjust(hsl(100, 0%, 50%), $saturation: 71%);\
             \n  c72: color.adjust(hsl(100, 0%, 50%), $saturation: 72%);\
             \n  c73: color.adjust(hsl(100, 0%, 50%), $saturation: 73%);\
             \n  c74: color.adjust(hsl(100, 0%, 50%), $saturation: 74%);\
             \n  c75: color.adjust(hsl(100, 0%, 50%), $saturation: 75%);\
             \n  c76: color.adjust(hsl(100, 0%, 50%), $saturation: 76%);\
             \n  c77: color.adjust(hsl(100, 0%, 50%), $saturation: 77%);\
             \n  c78: color.adjust(hsl(100, 0%, 50%), $saturation: 78%);\
             \n  c79: color.adjust(hsl(100, 0%, 50%), $saturation: 79%);\
             \n  // c80: saturate(hsl(100, 0%, 50%), 80%);\
             \n  c81: color.adjust(hsl(100, 0%, 50%), $saturation: 81%);\
             \n  c82: color.adjust(hsl(100, 0%, 50%), $saturation: 82%);\
             \n  c83: color.adjust(hsl(100, 0%, 50%), $saturation: 83%);\
             \n  c84: color.adjust(hsl(100, 0%, 50%), $saturation: 84%);\
             \n  c85: color.adjust(hsl(100, 0%, 50%), $saturation: 85%);\
             \n  c86: color.adjust(hsl(100, 0%, 50%), $saturation: 86%);\
             \n  c87: color.adjust(hsl(100, 0%, 50%), $saturation: 87%);\
             \n  c88: color.adjust(hsl(100, 0%, 50%), $saturation: 88%);\
             \n  c89: color.adjust(hsl(100, 0%, 50%), $saturation: 89%);\
             \n  c90: color.adjust(hsl(100, 0%, 50%), $saturation: 90%);\
             \n  c91: color.adjust(hsl(100, 0%, 50%), $saturation: 91%);\
             \n  c92: color.adjust(hsl(100, 0%, 50%), $saturation: 92%);\
             \n  c93: color.adjust(hsl(100, 0%, 50%), $saturation: 93%);\
             \n  c94: color.adjust(hsl(100, 0%, 50%), $saturation: 94%);\
             \n  c95: color.adjust(hsl(100, 0%, 50%), $saturation: 95%);\
             \n  c96: color.adjust(hsl(100, 0%, 50%), $saturation: 96%);\
             \n  c97: color.adjust(hsl(100, 0%, 50%), $saturation: 97%);\
             \n  c98: color.adjust(hsl(100, 0%, 50%), $saturation: 98%);\
             \n  c99: color.adjust(hsl(100, 0%, 50%), $saturation: 99%);\
             \n  c100: color.adjust(hsl(100, 0%, 50%), $saturation: 100%);\
             \n}\n"),
        "foo {\
         \n  c0: hsl(100, 0%, 50%);\
         \n  c1: hsl(100, 1%, 50%);\
         \n  c2: hsl(100, 2%, 50%);\
         \n  c3: hsl(100, 3%, 50%);\
         \n  c4: hsl(100, 4%, 50%);\
         \n  c5: hsl(100, 5%, 50%);\
         \n  c6: hsl(100, 6%, 50%);\
         \n  c7: hsl(100, 7%, 50%);\
         \n  c8: hsl(100, 8%, 50%);\
         \n  c9: hsl(100, 9%, 50%);\
         \n  c10: hsl(100, 10%, 50%);\
         \n  c11: hsl(100, 11%, 50%);\
         \n  c12: hsl(100, 12%, 50%);\
         \n  c13: hsl(100, 13%, 50%);\
         \n  c14: hsl(100, 14%, 50%);\
         \n  c15: hsl(100, 15%, 50%);\
         \n  c16: hsl(100, 16%, 50%);\
         \n  c17: hsl(100, 17%, 50%);\
         \n  c18: hsl(100, 18%, 50%);\
         \n  c19: hsl(100, 19%, 50%);\
         \n  c20: hsl(100, 20%, 50%);\
         \n  c21: hsl(100, 21%, 50%);\
         \n  c22: hsl(100, 22%, 50%);\
         \n  c23: hsl(100, 23%, 50%);\
         \n  c24: hsl(100, 24%, 50%);\
         \n  c25: hsl(100, 25%, 50%);\
         \n  c26: hsl(100, 26%, 50%);\
         \n  c27: hsl(100, 27%, 50%);\
         \n  c28: hsl(100, 28%, 50%);\
         \n  c29: hsl(100, 29%, 50%);\
         \n  c30: hsl(100, 30%, 50%);\
         \n  c31: hsl(100, 31%, 50%);\
         \n  c32: hsl(100, 32%, 50%);\
         \n  c33: hsl(100, 33%, 50%);\
         \n  c34: hsl(100, 34%, 50%);\
         \n  c35: hsl(100, 35%, 50%);\
         \n  c36: hsl(100, 36%, 50%);\
         \n  c37: hsl(100, 37%, 50%);\
         \n  c38: hsl(100, 38%, 50%);\
         \n  c39: hsl(100, 39%, 50%);\
         \n  c41: hsl(100, 41%, 50%);\
         \n  c42: hsl(100, 42%, 50%);\
         \n  c43: hsl(100, 43%, 50%);\
         \n  c44: hsl(100, 44%, 50%);\
         \n  c45: hsl(100, 45%, 50%);\
         \n  c46: hsl(100, 46%, 50%);\
         \n  c47: hsl(100, 47%, 50%);\
         \n  c48: hsl(100, 48%, 50%);\
         \n  c49: hsl(100, 49%, 50%);\
         \n  c50: hsl(100, 50%, 50%);\
         \n  c51: hsl(100, 51%, 50%);\
         \n  c52: hsl(100, 52%, 50%);\
         \n  c53: hsl(100, 53%, 50%);\
         \n  c54: hsl(100, 54%, 50%);\
         \n  c55: hsl(100, 55%, 50%);\
         \n  c56: hsl(100, 56%, 50%);\
         \n  c57: hsl(100, 57%, 50%);\
         \n  c58: hsl(100, 58%, 50%);\
         \n  c59: hsl(100, 59%, 50%);\
         \n  c60: hsl(100, 60%, 50%);\
         \n  c61: hsl(100, 61%, 50%);\
         \n  c62: hsl(100, 62%, 50%);\
         \n  c63: hsl(100, 63%, 50%);\
         \n  c64: hsl(100, 64%, 50%);\
         \n  c65: hsl(100, 65%, 50%);\
         \n  c66: hsl(100, 66%, 50%);\
         \n  c67: hsl(100, 67%, 50%);\
         \n  c68: hsl(100, 68%, 50%);\
         \n  c69: hsl(100, 69%, 50%);\
         \n  c70: hsl(100, 70%, 50%);\
         \n  c71: hsl(100, 71%, 50%);\
         \n  c72: hsl(100, 72%, 50%);\
         \n  c73: hsl(100, 73%, 50%);\
         \n  c74: hsl(100, 74%, 50%);\
         \n  c75: hsl(100, 75%, 50%);\
         \n  c76: hsl(100, 76%, 50%);\
         \n  c77: hsl(100, 77%, 50%);\
         \n  c78: hsl(100, 78%, 50%);\
         \n  c79: hsl(100, 79%, 50%);\
         \n  c81: hsl(100, 81%, 50%);\
         \n  c82: hsl(100, 82%, 50%);\
         \n  c83: hsl(100, 83%, 50%);\
         \n  c84: hsl(100, 84%, 50%);\
         \n  c85: hsl(100, 85%, 50%);\
         \n  c86: hsl(100, 86%, 50%);\
         \n  c87: hsl(100, 87%, 50%);\
         \n  c88: hsl(100, 88%, 50%);\
         \n  c89: hsl(100, 89%, 50%);\
         \n  c90: hsl(100, 90%, 50%);\
         \n  c91: hsl(100, 91%, 50%);\
         \n  c92: hsl(100, 92%, 50%);\
         \n  c93: hsl(100, 93%, 50%);\
         \n  c94: hsl(100, 94%, 50%);\
         \n  c95: hsl(100, 95%, 50%);\
         \n  c96: hsl(100, 96%, 50%);\
         \n  c97: hsl(100, 97%, 50%);\
         \n  c98: hsl(100, 98%, 50%);\
         \n  c99: hsl(100, 99%, 50%);\
         \n  c100: hsl(100, 100%, 50%);\
         \n}\n"
    );
}
