//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/saturate.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  c0: saturate(hsl(100, 0%, 50%), 0%);\
            \n  c1: saturate(hsl(100, 0%, 50%), 1%);\
            \n  c2: saturate(hsl(100, 0%, 50%), 2%);\
            \n  c3: saturate(hsl(100, 0%, 50%), 3%);\
            \n  c4: saturate(hsl(100, 0%, 50%), 4%);\
            \n  c5: saturate(hsl(100, 0%, 50%), 5%);\
            \n  c6: saturate(hsl(100, 0%, 50%), 6%);\
            \n  c7: saturate(hsl(100, 0%, 50%), 7%);\
            \n  c8: saturate(hsl(100, 0%, 50%), 8%);\
            \n  c9: saturate(hsl(100, 0%, 50%), 9%);\
            \n  c10: saturate(hsl(100, 0%, 50%), 10%);\
            \n  c11: saturate(hsl(100, 0%, 50%), 11%);\
            \n  c12: saturate(hsl(100, 0%, 50%), 12%);\
            \n  c13: saturate(hsl(100, 0%, 50%), 13%);\
            \n  c14: saturate(hsl(100, 0%, 50%), 14%);\
            \n  c15: saturate(hsl(100, 0%, 50%), 15%);\
            \n  c16: saturate(hsl(100, 0%, 50%), 16%);\
            \n  c17: saturate(hsl(100, 0%, 50%), 17%);\
            \n  c18: saturate(hsl(100, 0%, 50%), 18%);\
            \n  c19: saturate(hsl(100, 0%, 50%), 19%);\
            \n  c20: saturate(hsl(100, 0%, 50%), 20%);\
            \n  c21: saturate(hsl(100, 0%, 50%), 21%);\
            \n  c22: saturate(hsl(100, 0%, 50%), 22%);\
            \n  c23: saturate(hsl(100, 0%, 50%), 23%);\
            \n  c24: saturate(hsl(100, 0%, 50%), 24%);\
            \n  c25: saturate(hsl(100, 0%, 50%), 25%);\
            \n  c26: saturate(hsl(100, 0%, 50%), 26%);\
            \n  c27: saturate(hsl(100, 0%, 50%), 27%);\
            \n  c28: saturate(hsl(100, 0%, 50%), 28%);\
            \n  c29: saturate(hsl(100, 0%, 50%), 29%);\
            \n  c30: saturate(hsl(100, 0%, 50%), 30%);\
            \n  c31: saturate(hsl(100, 0%, 50%), 31%);\
            \n  c32: saturate(hsl(100, 0%, 50%), 32%);\
            \n  c33: saturate(hsl(100, 0%, 50%), 33%);\
            \n  c34: saturate(hsl(100, 0%, 50%), 34%);\
            \n  c35: saturate(hsl(100, 0%, 50%), 35%);\
            \n  c36: saturate(hsl(100, 0%, 50%), 36%);\
            \n  c37: saturate(hsl(100, 0%, 50%), 37%);\
            \n  c38: saturate(hsl(100, 0%, 50%), 38%);\
            \n  c39: saturate(hsl(100, 0%, 50%), 39%);\
            \n  // c40: saturate(hsl(100, 0%, 50%), 40%);\
            \n  c41: saturate(hsl(100, 0%, 50%), 41%);\
            \n  c42: saturate(hsl(100, 0%, 50%), 42%);\
            \n  c43: saturate(hsl(100, 0%, 50%), 43%);\
            \n  c44: saturate(hsl(100, 0%, 50%), 44%);\
            \n  c45: saturate(hsl(100, 0%, 50%), 45%);\
            \n  c46: saturate(hsl(100, 0%, 50%), 46%);\
            \n  c47: saturate(hsl(100, 0%, 50%), 47%);\
            \n  c48: saturate(hsl(100, 0%, 50%), 48%);\
            \n  c49: saturate(hsl(100, 0%, 50%), 49%);\
            \n  c50: saturate(hsl(100, 0%, 50%), 50%);\
            \n  c51: saturate(hsl(100, 0%, 50%), 51%);\
            \n  c52: saturate(hsl(100, 0%, 50%), 52%);\
            \n  c53: saturate(hsl(100, 0%, 50%), 53%);\
            \n  c54: saturate(hsl(100, 0%, 50%), 54%);\
            \n  c55: saturate(hsl(100, 0%, 50%), 55%);\
            \n  c56: saturate(hsl(100, 0%, 50%), 56%);\
            \n  c57: saturate(hsl(100, 0%, 50%), 57%);\
            \n  c58: saturate(hsl(100, 0%, 50%), 58%);\
            \n  c59: saturate(hsl(100, 0%, 50%), 59%);\
            \n  c60: saturate(hsl(100, 0%, 50%), 60%);\
            \n  c61: saturate(hsl(100, 0%, 50%), 61%);\
            \n  c62: saturate(hsl(100, 0%, 50%), 62%);\
            \n  c63: saturate(hsl(100, 0%, 50%), 63%);\
            \n  c64: saturate(hsl(100, 0%, 50%), 64%);\
            \n  c65: saturate(hsl(100, 0%, 50%), 65%);\
            \n  c66: saturate(hsl(100, 0%, 50%), 66%);\
            \n  c67: saturate(hsl(100, 0%, 50%), 67%);\
            \n  c68: saturate(hsl(100, 0%, 50%), 68%);\
            \n  c69: saturate(hsl(100, 0%, 50%), 69%);\
            \n  c70: saturate(hsl(100, 0%, 50%), 70%);\
            \n  c71: saturate(hsl(100, 0%, 50%), 71%);\
            \n  c72: saturate(hsl(100, 0%, 50%), 72%);\
            \n  c73: saturate(hsl(100, 0%, 50%), 73%);\
            \n  c74: saturate(hsl(100, 0%, 50%), 74%);\
            \n  c75: saturate(hsl(100, 0%, 50%), 75%);\
            \n  c76: saturate(hsl(100, 0%, 50%), 76%);\
            \n  c77: saturate(hsl(100, 0%, 50%), 77%);\
            \n  c78: saturate(hsl(100, 0%, 50%), 78%);\
            \n  c79: saturate(hsl(100, 0%, 50%), 79%);\
            \n  // c80: saturate(hsl(100, 0%, 50%), 80%);\
            \n  c81: saturate(hsl(100, 0%, 50%), 81%);\
            \n  c82: saturate(hsl(100, 0%, 50%), 82%);\
            \n  c83: saturate(hsl(100, 0%, 50%), 83%);\
            \n  c84: saturate(hsl(100, 0%, 50%), 84%);\
            \n  c85: saturate(hsl(100, 0%, 50%), 85%);\
            \n  c86: saturate(hsl(100, 0%, 50%), 86%);\
            \n  c87: saturate(hsl(100, 0%, 50%), 87%);\
            \n  c88: saturate(hsl(100, 0%, 50%), 88%);\
            \n  c89: saturate(hsl(100, 0%, 50%), 89%);\
            \n  c90: saturate(hsl(100, 0%, 50%), 90%);\
            \n  c91: saturate(hsl(100, 0%, 50%), 91%);\
            \n  c92: saturate(hsl(100, 0%, 50%), 92%);\
            \n  c93: saturate(hsl(100, 0%, 50%), 93%);\
            \n  c94: saturate(hsl(100, 0%, 50%), 94%);\
            \n  c95: saturate(hsl(100, 0%, 50%), 95%);\
            \n  c96: saturate(hsl(100, 0%, 50%), 96%);\
            \n  c97: saturate(hsl(100, 0%, 50%), 97%);\
            \n  c98: saturate(hsl(100, 0%, 50%), 98%);\
            \n  c99: saturate(hsl(100, 0%, 50%), 99%);\
            \n  c100: saturate(hsl(100, 0%, 50%), 100%);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  c0: gray;\
        \n  c1: #7f817e;\
        \n  c2: #7f827d;\
        \n  c3: #7e837c;\
        \n  c4: #7e857a;\
        \n  c5: #7d8679;\
        \n  c6: #7d8778;\
        \n  c7: #7d8877;\
        \n  c8: #7c8a75;\
        \n  c9: #7c8b74;\
        \n  c10: #7b8c73;\
        \n  c11: #7b8e71;\
        \n  c12: #7a8f70;\
        \n  c13: #7a906f;\
        \n  c14: #7a916e;\
        \n  c15: #79936c;\
        \n  c16: #79946b;\
        \n  c17: #78956a;\
        \n  c18: #789669;\
        \n  c19: #779867;\
        \n  c20: #779966;\
        \n  c21: #779a65;\
        \n  c22: #769c63;\
        \n  c23: #769d62;\
        \n  c24: #759e61;\
        \n  c25: #759f60;\
        \n  c26: #74a15e;\
        \n  c27: #74a25d;\
        \n  c28: #74a35c;\
        \n  c29: #73a45b;\
        \n  c30: #73a659;\
        \n  c31: #72a758;\
        \n  c32: #72a857;\
        \n  c33: #71aa55;\
        \n  c34: #71ab54;\
        \n  c35: #71ac53;\
        \n  c36: #70ad52;\
        \n  c37: #70af50;\
        \n  c38: #6fb04f;\
        \n  c39: #6fb14e;\
        \n  c41: #6eb44b;\
        \n  c42: #6eb54a;\
        \n  c43: #6db649;\
        \n  c44: #6db847;\
        \n  c45: #6cb946;\
        \n  c46: #6cba45;\
        \n  c47: #6cbb44;\
        \n  c48: #6bbd42;\
        \n  c49: #6bbe41;\
        \n  c50: #6abf40;\
        \n  c51: #6ac13e;\
        \n  c52: #69c23d;\
        \n  c53: #69c33c;\
        \n  c54: #69c43b;\
        \n  c55: #68c639;\
        \n  c56: #68c738;\
        \n  c57: #67c837;\
        \n  c58: #67c936;\
        \n  c59: #66cb34;\
        \n  c60: #66cc33;\
        \n  c61: #66cd32;\
        \n  c62: #65cf30;\
        \n  c63: #65d02f;\
        \n  c64: #64d12e;\
        \n  c65: #64d22d;\
        \n  c66: #63d42b;\
        \n  c67: #63d52a;\
        \n  c68: #63d629;\
        \n  c69: #62d728;\
        \n  c70: #62d926;\
        \n  c71: #61da25;\
        \n  c72: #61db24;\
        \n  c73: #60dd22;\
        \n  c74: #60de21;\
        \n  c75: #60df20;\
        \n  c76: #5fe01f;\
        \n  c77: #5fe21d;\
        \n  c78: #5ee31c;\
        \n  c79: #5ee41b;\
        \n  c81: #5de718;\
        \n  c82: #5de817;\
        \n  c83: #5ce916;\
        \n  c84: #5ceb14;\
        \n  c85: #5bec13;\
        \n  c86: #5bed12;\
        \n  c87: #5bee11;\
        \n  c88: #5af00f;\
        \n  c89: #5af10e;\
        \n  c90: #59f20d;\
        \n  c91: #59f40b;\
        \n  c92: #58f50a;\
        \n  c93: #58f609;\
        \n  c94: #58f708;\
        \n  c95: #57f906;\
        \n  c96: #57fa05;\
        \n  c97: #56fb04;\
        \n  c98: #56fc03;\
        \n  c99: #55fe01;\
        \n  c100: #55ff00;\
        \n}\
        \n"
    );
}
