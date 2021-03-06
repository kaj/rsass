//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/rgb/rgba/b.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  c-1: rgba(0,0,-1,1);\
             \n  c0: rgba(0,0,0,1);\
             \n  c1: rgba(0,0,1,1);\
             \n  c2: rgba(0,0,2,1);\
             \n  c3: rgba(0,0,3,1);\
             \n  c4: rgba(0,0,4,1);\
             \n  c5: rgba(0,0,5,1);\
             \n  c6: rgba(0,0,6,1);\
             \n  c7: rgba(0,0,7,1);\
             \n  c8: rgba(0,0,8,1);\
             \n  c9: rgba(0,0,9,1);\
             \n  c10: rgba(0,0,10,1);\
             \n  c11: rgba(0,0,11,1);\
             \n  c12: rgba(0,0,12,1);\
             \n  c13: rgba(0,0,13,1);\
             \n  c14: rgba(0,0,14,1);\
             \n  c15: rgba(0,0,15,1);\
             \n  c16: rgba(0,0,16,1);\
             \n  c17: rgba(0,0,17,1);\
             \n  c18: rgba(0,0,18,1);\
             \n  c19: rgba(0,0,19,1);\
             \n  c20: rgba(0,0,20,1);\
             \n  c21: rgba(0,0,21,1);\
             \n  c22: rgba(0,0,22,1);\
             \n  c23: rgba(0,0,23,1);\
             \n  c24: rgba(0,0,24,1);\
             \n  c25: rgba(0,0,25,1);\
             \n  c26: rgba(0,0,26,1);\
             \n  c27: rgba(0,0,27,1);\
             \n  c28: rgba(0,0,28,1);\
             \n  c29: rgba(0,0,29,1);\
             \n  c30: rgba(0,0,30,1);\
             \n  c31: rgba(0,0,31,1);\
             \n  c32: rgba(0,0,32,1);\
             \n  c33: rgba(0,0,33,1);\
             \n  c34: rgba(0,0,34,1);\
             \n  c35: rgba(0,0,35,1);\
             \n  c36: rgba(0,0,36,1);\
             \n  c37: rgba(0,0,37,1);\
             \n  c38: rgba(0,0,38,1);\
             \n  c39: rgba(0,0,39,1);\
             \n  c40: rgba(0,0,40,1);\
             \n  c41: rgba(0,0,41,1);\
             \n  c42: rgba(0,0,42,1);\
             \n  c43: rgba(0,0,43,1);\
             \n  c44: rgba(0,0,44,1);\
             \n  c45: rgba(0,0,45,1);\
             \n  c46: rgba(0,0,46,1);\
             \n  c47: rgba(0,0,47,1);\
             \n  c48: rgba(0,0,48,1);\
             \n  c49: rgba(0,0,49,1);\
             \n  c50: rgba(0,0,50,1);\
             \n  c51: rgba(0,0,51,1);\
             \n  c52: rgba(0,0,52,1);\
             \n  c53: rgba(0,0,53,1);\
             \n  c54: rgba(0,0,54,1);\
             \n  c55: rgba(0,0,55,1);\
             \n  c56: rgba(0,0,56,1);\
             \n  c57: rgba(0,0,57,1);\
             \n  c58: rgba(0,0,58,1);\
             \n  c59: rgba(0,0,59,1);\
             \n  c60: rgba(0,0,60,1);\
             \n  c61: rgba(0,0,61,1);\
             \n  c62: rgba(0,0,62,1);\
             \n  c63: rgba(0,0,63,1);\
             \n  c64: rgba(0,0,64,1);\
             \n  c65: rgba(0,0,65,1);\
             \n  c66: rgba(0,0,66,1);\
             \n  c67: rgba(0,0,67,1);\
             \n  c68: rgba(0,0,68,1);\
             \n  c69: rgba(0,0,69,1);\
             \n  c70: rgba(0,0,70,1);\
             \n  c71: rgba(0,0,71,1);\
             \n  c72: rgba(0,0,72,1);\
             \n  c73: rgba(0,0,73,1);\
             \n  c74: rgba(0,0,74,1);\
             \n  c75: rgba(0,0,75,1);\
             \n  c76: rgba(0,0,76,1);\
             \n  c77: rgba(0,0,77,1);\
             \n  c78: rgba(0,0,78,1);\
             \n  c79: rgba(0,0,79,1);\
             \n  c80: rgba(0,0,80,1);\
             \n  c81: rgba(0,0,81,1);\
             \n  c82: rgba(0,0,82,1);\
             \n  c83: rgba(0,0,83,1);\
             \n  c84: rgba(0,0,84,1);\
             \n  c85: rgba(0,0,85,1);\
             \n  c86: rgba(0,0,86,1);\
             \n  c87: rgba(0,0,87,1);\
             \n  c88: rgba(0,0,88,1);\
             \n  c89: rgba(0,0,89,1);\
             \n  c90: rgba(0,0,90,1);\
             \n  c91: rgba(0,0,91,1);\
             \n  c92: rgba(0,0,92,1);\
             \n  c93: rgba(0,0,93,1);\
             \n  c94: rgba(0,0,94,1);\
             \n  c95: rgba(0,0,95,1);\
             \n  c96: rgba(0,0,96,1);\
             \n  c97: rgba(0,0,97,1);\
             \n  c98: rgba(0,0,98,1);\
             \n  c99: rgba(0,0,99,1);\
             \n  c100: rgba(0,0,100,1);\
             \n  c101: rgba(0,0,101,1);\
             \n  c102: rgba(0,0,102,1);\
             \n  c103: rgba(0,0,103,1);\
             \n  c104: rgba(0,0,104,1);\
             \n  c105: rgba(0,0,105,1);\
             \n  c106: rgba(0,0,106,1);\
             \n  c107: rgba(0,0,107,1);\
             \n  c108: rgba(0,0,108,1);\
             \n  c109: rgba(0,0,109,1);\
             \n  c110: rgba(0,0,110,1);\
             \n  c111: rgba(0,0,111,1);\
             \n  c112: rgba(0,0,112,1);\
             \n  c113: rgba(0,0,113,1);\
             \n  c114: rgba(0,0,114,1);\
             \n  c115: rgba(0,0,115,1);\
             \n  c116: rgba(0,0,116,1);\
             \n  c117: rgba(0,0,117,1);\
             \n  c118: rgba(0,0,118,1);\
             \n  c119: rgba(0,0,119,1);\
             \n  c120: rgba(0,0,120,1);\
             \n  c121: rgba(0,0,121,1);\
             \n  c122: rgba(0,0,122,1);\
             \n  c123: rgba(0,0,123,1);\
             \n  c124: rgba(0,0,124,1);\
             \n  c125: rgba(0,0,125,1);\
             \n  c126: rgba(0,0,126,1);\
             \n  c127: rgba(0,0,127,1);\
             \n  c128: rgba(0,0,128,1);\
             \n  c129: rgba(0,0,129,1);\
             \n  c130: rgba(0,0,130,1);\
             \n  c131: rgba(0,0,131,1);\
             \n  c132: rgba(0,0,132,1);\
             \n  c133: rgba(0,0,133,1);\
             \n  c134: rgba(0,0,134,1);\
             \n  c135: rgba(0,0,135,1);\
             \n  c136: rgba(0,0,136,1);\
             \n  c137: rgba(0,0,137,1);\
             \n  c138: rgba(0,0,138,1);\
             \n  c139: rgba(0,0,139,1);\
             \n  c140: rgba(0,0,140,1);\
             \n  c141: rgba(0,0,141,1);\
             \n  c142: rgba(0,0,142,1);\
             \n  c143: rgba(0,0,143,1);\
             \n  c144: rgba(0,0,144,1);\
             \n  c145: rgba(0,0,145,1);\
             \n  c146: rgba(0,0,146,1);\
             \n  c147: rgba(0,0,147,1);\
             \n  c148: rgba(0,0,148,1);\
             \n  c149: rgba(0,0,149,1);\
             \n  c150: rgba(0,0,150,1);\
             \n  c151: rgba(0,0,151,1);\
             \n  c152: rgba(0,0,152,1);\
             \n  c153: rgba(0,0,153,1);\
             \n  c154: rgba(0,0,154,1);\
             \n  c155: rgba(0,0,155,1);\
             \n  c156: rgba(0,0,156,1);\
             \n  c157: rgba(0,0,157,1);\
             \n  c158: rgba(0,0,158,1);\
             \n  c159: rgba(0,0,159,1);\
             \n  c160: rgba(0,0,160,1);\
             \n  c161: rgba(0,0,161,1);\
             \n  c162: rgba(0,0,162,1);\
             \n  c163: rgba(0,0,163,1);\
             \n  c164: rgba(0,0,164,1);\
             \n  c165: rgba(0,0,165,1);\
             \n  c166: rgba(0,0,166,1);\
             \n  c167: rgba(0,0,167,1);\
             \n  c168: rgba(0,0,168,1);\
             \n  c169: rgba(0,0,169,1);\
             \n  c170: rgba(0,0,170,1);\
             \n  c171: rgba(0,0,171,1);\
             \n  c172: rgba(0,0,172,1);\
             \n  c173: rgba(0,0,173,1);\
             \n  c174: rgba(0,0,174,1);\
             \n  c175: rgba(0,0,175,1);\
             \n  c176: rgba(0,0,176,1);\
             \n  c177: rgba(0,0,177,1);\
             \n  c178: rgba(0,0,178,1);\
             \n  c179: rgba(0,0,179,1);\
             \n  c180: rgba(0,0,180,1);\
             \n  c181: rgba(0,0,181,1);\
             \n  c182: rgba(0,0,182,1);\
             \n  c183: rgba(0,0,183,1);\
             \n  c184: rgba(0,0,184,1);\
             \n  c185: rgba(0,0,185,1);\
             \n  c186: rgba(0,0,186,1);\
             \n  c187: rgba(0,0,187,1);\
             \n  c188: rgba(0,0,188,1);\
             \n  c189: rgba(0,0,189,1);\
             \n  c190: rgba(0,0,190,1);\
             \n  c191: rgba(0,0,191,1);\
             \n  c192: rgba(0,0,192,1);\
             \n  c193: rgba(0,0,193,1);\
             \n  c194: rgba(0,0,194,1);\
             \n  c195: rgba(0,0,195,1);\
             \n  c196: rgba(0,0,196,1);\
             \n  c197: rgba(0,0,197,1);\
             \n  c198: rgba(0,0,198,1);\
             \n  c199: rgba(0,0,199,1);\
             \n  c200: rgba(0,0,200,1);\
             \n  c201: rgba(0,0,201,1);\
             \n  c202: rgba(0,0,202,1);\
             \n  c203: rgba(0,0,203,1);\
             \n  c204: rgba(0,0,204,1);\
             \n  c205: rgba(0,0,205,1);\
             \n  c206: rgba(0,0,206,1);\
             \n  c207: rgba(0,0,207,1);\
             \n  c208: rgba(0,0,208,1);\
             \n  c209: rgba(0,0,209,1);\
             \n  c210: rgba(0,0,210,1);\
             \n  c211: rgba(0,0,211,1);\
             \n  c212: rgba(0,0,212,1);\
             \n  c213: rgba(0,0,213,1);\
             \n  c214: rgba(0,0,214,1);\
             \n  c215: rgba(0,0,215,1);\
             \n  c216: rgba(0,0,216,1);\
             \n  c217: rgba(0,0,217,1);\
             \n  c218: rgba(0,0,218,1);\
             \n  c219: rgba(0,0,219,1);\
             \n  c220: rgba(0,0,220,1);\
             \n  c221: rgba(0,0,221,1);\
             \n  c222: rgba(0,0,222,1);\
             \n  c223: rgba(0,0,223,1);\
             \n  c224: rgba(0,0,224,1);\
             \n  c225: rgba(0,0,225,1);\
             \n  c226: rgba(0,0,226,1);\
             \n  c227: rgba(0,0,227,1);\
             \n  c228: rgba(0,0,228,1);\
             \n  c229: rgba(0,0,229,1);\
             \n  c230: rgba(0,0,230,1);\
             \n  c231: rgba(0,0,231,1);\
             \n  c232: rgba(0,0,232,1);\
             \n  c233: rgba(0,0,233,1);\
             \n  c234: rgba(0,0,234,1);\
             \n  c235: rgba(0,0,235,1);\
             \n  c236: rgba(0,0,236,1);\
             \n  c237: rgba(0,0,237,1);\
             \n  c238: rgba(0,0,238,1);\
             \n  c239: rgba(0,0,239,1);\
             \n  c240: rgba(0,0,240,1);\
             \n  c241: rgba(0,0,241,1);\
             \n  c242: rgba(0,0,242,1);\
             \n  c243: rgba(0,0,243,1);\
             \n  c244: rgba(0,0,244,1);\
             \n  c245: rgba(0,0,245,1);\
             \n  c246: rgba(0,0,246,1);\
             \n  c247: rgba(0,0,247,1);\
             \n  c248: rgba(0,0,248,1);\
             \n  c249: rgba(0,0,249,1);\
             \n  c250: rgba(0,0,250,1);\
             \n  c251: rgba(0,0,251,1);\
             \n  c252: rgba(0,0,252,1);\
             \n  c253: rgba(0,0,253,1);\
             \n  c254: rgba(0,0,254,1);\
             \n  c255: rgba(0,0,255,1);\
             \n  c256: rgba(0,0,256,1);\
             \n}\n\
             \nfoo {\
             \n  c-1: rgba(0,0,-1%,1);\
             \n  c0: rgba(0,0,0%,1);\
             \n  c1: rgba(0,0,1%,1);\
             \n  c2: rgba(0,0,2%,1);\
             \n  c3: rgba(0,0,3%,1);\
             \n  c4: rgba(0,0,4%,1);\
             \n  c5: rgba(0,0,5%,1);\
             \n  c6: rgba(0,0,6%,1);\
             \n  c7: rgba(0,0,7%,1);\
             \n  c8: rgba(0,0,8%,1);\
             \n  c9: rgba(0,0,9%,1);\
             \n  c10: rgba(0,0,10%,1);\
             \n  c11: rgba(0,0,11%,1);\
             \n  c12: rgba(0,0,12%,1);\
             \n  c13: rgba(0,0,13%,1);\
             \n  c14: rgba(0,0,14%,1);\
             \n  c15: rgba(0,0,15%,1);\
             \n  c16: rgba(0,0,16%,1);\
             \n  c17: rgba(0,0,17%,1);\
             \n  c18: rgba(0,0,18%,1);\
             \n  c19: rgba(0,0,19%,1);\
             \n  c20: rgba(0,0,20%,1);\
             \n  c21: rgba(0,0,21%,1);\
             \n  c22: rgba(0,0,22%,1);\
             \n  c23: rgba(0,0,23%,1);\
             \n  c24: rgba(0,0,24%,1);\
             \n  c25: rgba(0,0,25%,1);\
             \n  c26: rgba(0,0,26%,1);\
             \n  c27: rgba(0,0,27%,1);\
             \n  c28: rgba(0,0,28%,1);\
             \n  c29: rgba(0,0,29%,1);\
             \n  c30: rgba(0,0,30%,1);\
             \n  c31: rgba(0,0,31%,1);\
             \n  c32: rgba(0,0,32%,1);\
             \n  c33: rgba(0,0,33%,1);\
             \n  c34: rgba(0,0,34%,1);\
             \n  c35: rgba(0,0,35%,1);\
             \n  c36: rgba(0,0,36%,1);\
             \n  c37: rgba(0,0,37%,1);\
             \n  c38: rgba(0,0,38%,1);\
             \n  c39: rgba(0,0,39%,1);\
             \n  c40: rgba(0,0,40%,1);\
             \n  c41: rgba(0,0,41%,1);\
             \n  c42: rgba(0,0,42%,1);\
             \n  c43: rgba(0,0,43%,1);\
             \n  c44: rgba(0,0,44%,1);\
             \n  c45: rgba(0,0,45%,1);\
             \n  c46: rgba(0,0,46%,1);\
             \n  c47: rgba(0,0,47%,1);\
             \n  c48: rgba(0,0,48%,1);\
             \n  c49: rgba(0,0,49%,1);\
             \n  c50: rgba(0,0,50%,1);\
             \n  c51: rgba(0,0,51%,1);\
             \n  c52: rgba(0,0,52%,1);\
             \n  c53: rgba(0,0,53%,1);\
             \n  c54: rgba(0,0,54%,1);\
             \n  c55: rgba(0,0,55%,1);\
             \n  c56: rgba(0,0,56%,1);\
             \n  c57: rgba(0,0,57%,1);\
             \n  c58: rgba(0,0,58%,1);\
             \n  c59: rgba(0,0,59%,1);\
             \n  c60: rgba(0,0,60%,1);\
             \n  c61: rgba(0,0,61%,1);\
             \n  c62: rgba(0,0,62%,1);\
             \n  c63: rgba(0,0,63%,1);\
             \n  c64: rgba(0,0,64%,1);\
             \n  c65: rgba(0,0,65%,1);\
             \n  c66: rgba(0,0,66%,1);\
             \n  c67: rgba(0,0,67%,1);\
             \n  c68: rgba(0,0,68%,1);\
             \n  c69: rgba(0,0,69%,1);\
             \n  c70: rgba(0,0,70%,1);\
             \n  c71: rgba(0,0,71%,1);\
             \n  c72: rgba(0,0,72%,1);\
             \n  c73: rgba(0,0,73%,1);\
             \n  c74: rgba(0,0,74%,1);\
             \n  c75: rgba(0,0,75%,1);\
             \n  c76: rgba(0,0,76%,1);\
             \n  c77: rgba(0,0,77%,1);\
             \n  c78: rgba(0,0,78%,1);\
             \n  c79: rgba(0,0,79%,1);\
             \n  c80: rgba(0,0,80%,1);\
             \n  c81: rgba(0,0,81%,1);\
             \n  c82: rgba(0,0,82%,1);\
             \n  c83: rgba(0,0,83%,1);\
             \n  c84: rgba(0,0,84%,1);\
             \n  c85: rgba(0,0,85%,1);\
             \n  c86: rgba(0,0,86%,1);\
             \n  c87: rgba(0,0,87%,1);\
             \n  c88: rgba(0,0,88%,1);\
             \n  c89: rgba(0,0,89%,1);\
             \n  c90: rgba(0,0,90%,1);\
             \n  c91: rgba(0,0,91%,1);\
             \n  c92: rgba(0,0,92%,1);\
             \n  c93: rgba(0,0,93%,1);\
             \n  c94: rgba(0,0,94%,1);\
             \n  c95: rgba(0,0,95%,1);\
             \n  c96: rgba(0,0,96%,1);\
             \n  c97: rgba(0,0,97%,1);\
             \n  c98: rgba(0,0,98%,1);\
             \n  c99: rgba(0,0,99%,1);\
             \n  c100: rgba(0,0,100%,1);\
             \n  c101: rgba(0,0,101%,1);\
             \n}\n"),
        "foo {\
         \n  c-1: black;\
         \n  c0: black;\
         \n  c1: #000001;\
         \n  c2: #000002;\
         \n  c3: #000003;\
         \n  c4: #000004;\
         \n  c5: #000005;\
         \n  c6: #000006;\
         \n  c7: #000007;\
         \n  c8: #000008;\
         \n  c9: #000009;\
         \n  c10: #00000a;\
         \n  c11: #00000b;\
         \n  c12: #00000c;\
         \n  c13: #00000d;\
         \n  c14: #00000e;\
         \n  c15: #00000f;\
         \n  c16: #000010;\
         \n  c17: #000011;\
         \n  c18: #000012;\
         \n  c19: #000013;\
         \n  c20: #000014;\
         \n  c21: #000015;\
         \n  c22: #000016;\
         \n  c23: #000017;\
         \n  c24: #000018;\
         \n  c25: #000019;\
         \n  c26: #00001a;\
         \n  c27: #00001b;\
         \n  c28: #00001c;\
         \n  c29: #00001d;\
         \n  c30: #00001e;\
         \n  c31: #00001f;\
         \n  c32: #000020;\
         \n  c33: #000021;\
         \n  c34: #000022;\
         \n  c35: #000023;\
         \n  c36: #000024;\
         \n  c37: #000025;\
         \n  c38: #000026;\
         \n  c39: #000027;\
         \n  c40: #000028;\
         \n  c41: #000029;\
         \n  c42: #00002a;\
         \n  c43: #00002b;\
         \n  c44: #00002c;\
         \n  c45: #00002d;\
         \n  c46: #00002e;\
         \n  c47: #00002f;\
         \n  c48: #000030;\
         \n  c49: #000031;\
         \n  c50: #000032;\
         \n  c51: #000033;\
         \n  c52: #000034;\
         \n  c53: #000035;\
         \n  c54: #000036;\
         \n  c55: #000037;\
         \n  c56: #000038;\
         \n  c57: #000039;\
         \n  c58: #00003a;\
         \n  c59: #00003b;\
         \n  c60: #00003c;\
         \n  c61: #00003d;\
         \n  c62: #00003e;\
         \n  c63: #00003f;\
         \n  c64: #000040;\
         \n  c65: #000041;\
         \n  c66: #000042;\
         \n  c67: #000043;\
         \n  c68: #000044;\
         \n  c69: #000045;\
         \n  c70: #000046;\
         \n  c71: #000047;\
         \n  c72: #000048;\
         \n  c73: #000049;\
         \n  c74: #00004a;\
         \n  c75: #00004b;\
         \n  c76: #00004c;\
         \n  c77: #00004d;\
         \n  c78: #00004e;\
         \n  c79: #00004f;\
         \n  c80: #000050;\
         \n  c81: #000051;\
         \n  c82: #000052;\
         \n  c83: #000053;\
         \n  c84: #000054;\
         \n  c85: #000055;\
         \n  c86: #000056;\
         \n  c87: #000057;\
         \n  c88: #000058;\
         \n  c89: #000059;\
         \n  c90: #00005a;\
         \n  c91: #00005b;\
         \n  c92: #00005c;\
         \n  c93: #00005d;\
         \n  c94: #00005e;\
         \n  c95: #00005f;\
         \n  c96: #000060;\
         \n  c97: #000061;\
         \n  c98: #000062;\
         \n  c99: #000063;\
         \n  c100: #000064;\
         \n  c101: #000065;\
         \n  c102: #000066;\
         \n  c103: #000067;\
         \n  c104: #000068;\
         \n  c105: #000069;\
         \n  c106: #00006a;\
         \n  c107: #00006b;\
         \n  c108: #00006c;\
         \n  c109: #00006d;\
         \n  c110: #00006e;\
         \n  c111: #00006f;\
         \n  c112: #000070;\
         \n  c113: #000071;\
         \n  c114: #000072;\
         \n  c115: #000073;\
         \n  c116: #000074;\
         \n  c117: #000075;\
         \n  c118: #000076;\
         \n  c119: #000077;\
         \n  c120: #000078;\
         \n  c121: #000079;\
         \n  c122: #00007a;\
         \n  c123: #00007b;\
         \n  c124: #00007c;\
         \n  c125: #00007d;\
         \n  c126: #00007e;\
         \n  c127: #00007f;\
         \n  c128: navy;\
         \n  c129: #000081;\
         \n  c130: #000082;\
         \n  c131: #000083;\
         \n  c132: #000084;\
         \n  c133: #000085;\
         \n  c134: #000086;\
         \n  c135: #000087;\
         \n  c136: #000088;\
         \n  c137: #000089;\
         \n  c138: #00008a;\
         \n  c139: darkblue;\
         \n  c140: #00008c;\
         \n  c141: #00008d;\
         \n  c142: #00008e;\
         \n  c143: #00008f;\
         \n  c144: #000090;\
         \n  c145: #000091;\
         \n  c146: #000092;\
         \n  c147: #000093;\
         \n  c148: #000094;\
         \n  c149: #000095;\
         \n  c150: #000096;\
         \n  c151: #000097;\
         \n  c152: #000098;\
         \n  c153: #000099;\
         \n  c154: #00009a;\
         \n  c155: #00009b;\
         \n  c156: #00009c;\
         \n  c157: #00009d;\
         \n  c158: #00009e;\
         \n  c159: #00009f;\
         \n  c160: #0000a0;\
         \n  c161: #0000a1;\
         \n  c162: #0000a2;\
         \n  c163: #0000a3;\
         \n  c164: #0000a4;\
         \n  c165: #0000a5;\
         \n  c166: #0000a6;\
         \n  c167: #0000a7;\
         \n  c168: #0000a8;\
         \n  c169: #0000a9;\
         \n  c170: #0000aa;\
         \n  c171: #0000ab;\
         \n  c172: #0000ac;\
         \n  c173: #0000ad;\
         \n  c174: #0000ae;\
         \n  c175: #0000af;\
         \n  c176: #0000b0;\
         \n  c177: #0000b1;\
         \n  c178: #0000b2;\
         \n  c179: #0000b3;\
         \n  c180: #0000b4;\
         \n  c181: #0000b5;\
         \n  c182: #0000b6;\
         \n  c183: #0000b7;\
         \n  c184: #0000b8;\
         \n  c185: #0000b9;\
         \n  c186: #0000ba;\
         \n  c187: #0000bb;\
         \n  c188: #0000bc;\
         \n  c189: #0000bd;\
         \n  c190: #0000be;\
         \n  c191: #0000bf;\
         \n  c192: #0000c0;\
         \n  c193: #0000c1;\
         \n  c194: #0000c2;\
         \n  c195: #0000c3;\
         \n  c196: #0000c4;\
         \n  c197: #0000c5;\
         \n  c198: #0000c6;\
         \n  c199: #0000c7;\
         \n  c200: #0000c8;\
         \n  c201: #0000c9;\
         \n  c202: #0000ca;\
         \n  c203: #0000cb;\
         \n  c204: #0000cc;\
         \n  c205: mediumblue;\
         \n  c206: #0000ce;\
         \n  c207: #0000cf;\
         \n  c208: #0000d0;\
         \n  c209: #0000d1;\
         \n  c210: #0000d2;\
         \n  c211: #0000d3;\
         \n  c212: #0000d4;\
         \n  c213: #0000d5;\
         \n  c214: #0000d6;\
         \n  c215: #0000d7;\
         \n  c216: #0000d8;\
         \n  c217: #0000d9;\
         \n  c218: #0000da;\
         \n  c219: #0000db;\
         \n  c220: #0000dc;\
         \n  c221: #0000dd;\
         \n  c222: #0000de;\
         \n  c223: #0000df;\
         \n  c224: #0000e0;\
         \n  c225: #0000e1;\
         \n  c226: #0000e2;\
         \n  c227: #0000e3;\
         \n  c228: #0000e4;\
         \n  c229: #0000e5;\
         \n  c230: #0000e6;\
         \n  c231: #0000e7;\
         \n  c232: #0000e8;\
         \n  c233: #0000e9;\
         \n  c234: #0000ea;\
         \n  c235: #0000eb;\
         \n  c236: #0000ec;\
         \n  c237: #0000ed;\
         \n  c238: #0000ee;\
         \n  c239: #0000ef;\
         \n  c240: #0000f0;\
         \n  c241: #0000f1;\
         \n  c242: #0000f2;\
         \n  c243: #0000f3;\
         \n  c244: #0000f4;\
         \n  c245: #0000f5;\
         \n  c246: #0000f6;\
         \n  c247: #0000f7;\
         \n  c248: #0000f8;\
         \n  c249: #0000f9;\
         \n  c250: #0000fa;\
         \n  c251: #0000fb;\
         \n  c252: #0000fc;\
         \n  c253: #0000fd;\
         \n  c254: #0000fe;\
         \n  c255: blue;\
         \n  c256: blue;\
         \n}\
         \nfoo {\
         \n  c-1: black;\
         \n  c0: black;\
         \n  c1: #000003;\
         \n  c2: #000005;\
         \n  c3: #000008;\
         \n  c4: #00000a;\
         \n  c5: #00000d;\
         \n  c6: #00000f;\
         \n  c7: #000012;\
         \n  c8: #000014;\
         \n  c9: #000017;\
         \n  c10: #00001a;\
         \n  c11: #00001c;\
         \n  c12: #00001f;\
         \n  c13: #000021;\
         \n  c14: #000024;\
         \n  c15: #000026;\
         \n  c16: #000029;\
         \n  c17: #00002b;\
         \n  c18: #00002e;\
         \n  c19: #000030;\
         \n  c20: #000033;\
         \n  c21: #000036;\
         \n  c22: #000038;\
         \n  c23: #00003b;\
         \n  c24: #00003d;\
         \n  c25: #000040;\
         \n  c26: #000042;\
         \n  c27: #000045;\
         \n  c28: #000047;\
         \n  c29: #00004a;\
         \n  c30: #00004d;\
         \n  c31: #00004f;\
         \n  c32: #000052;\
         \n  c33: #000054;\
         \n  c34: #000057;\
         \n  c35: #000059;\
         \n  c36: #00005c;\
         \n  c37: #00005e;\
         \n  c38: #000061;\
         \n  c39: #000063;\
         \n  c40: #000066;\
         \n  c41: #000069;\
         \n  c42: #00006b;\
         \n  c43: #00006e;\
         \n  c44: #000070;\
         \n  c45: #000073;\
         \n  c46: #000075;\
         \n  c47: #000078;\
         \n  c48: #00007a;\
         \n  c49: #00007d;\
         \n  c50: navy;\
         \n  c51: #000082;\
         \n  c52: #000085;\
         \n  c53: #000087;\
         \n  c54: #00008a;\
         \n  c55: #00008c;\
         \n  c56: #00008f;\
         \n  c57: #000091;\
         \n  c58: #000094;\
         \n  c59: #000096;\
         \n  c60: #000099;\
         \n  c61: #00009c;\
         \n  c62: #00009e;\
         \n  c63: #0000a1;\
         \n  c64: #0000a3;\
         \n  c65: #0000a6;\
         \n  c66: #0000a8;\
         \n  c67: #0000ab;\
         \n  c68: #0000ad;\
         \n  c69: #0000b0;\
         \n  c70: #0000b3;\
         \n  c71: #0000b5;\
         \n  c72: #0000b8;\
         \n  c73: #0000ba;\
         \n  c74: #0000bd;\
         \n  c75: #0000bf;\
         \n  c76: #0000c2;\
         \n  c77: #0000c4;\
         \n  c78: #0000c7;\
         \n  c79: #0000c9;\
         \n  c80: #0000cc;\
         \n  c81: #0000cf;\
         \n  c82: #0000d1;\
         \n  c83: #0000d4;\
         \n  c84: #0000d6;\
         \n  c85: #0000d9;\
         \n  c86: #0000db;\
         \n  c87: #0000de;\
         \n  c88: #0000e0;\
         \n  c89: #0000e3;\
         \n  c90: #0000e6;\
         \n  c91: #0000e8;\
         \n  c92: #0000eb;\
         \n  c93: #0000ed;\
         \n  c94: #0000f0;\
         \n  c95: #0000f2;\
         \n  c96: #0000f5;\
         \n  c97: #0000f7;\
         \n  c98: #0000fa;\
         \n  c99: #0000fc;\
         \n  c100: blue;\
         \n  c101: blue;\
         \n}\n"
    );
}
