//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1132.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1132")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  @for $i from 0 through 360 {\
             \n    i#{$i}: color.hue(hsl($i, 10%, 20%));\
             \n  }\
             \n}\n"),
        "foo {\
         \n  i0: 0deg;\
         \n  i1: 1deg;\
         \n  i2: 2deg;\
         \n  i3: 3deg;\
         \n  i4: 4deg;\
         \n  i5: 5deg;\
         \n  i6: 6deg;\
         \n  i7: 7deg;\
         \n  i8: 8deg;\
         \n  i9: 9deg;\
         \n  i10: 10deg;\
         \n  i11: 11deg;\
         \n  i12: 12deg;\
         \n  i13: 13deg;\
         \n  i14: 14deg;\
         \n  i15: 15deg;\
         \n  i16: 16deg;\
         \n  i17: 17deg;\
         \n  i18: 18deg;\
         \n  i19: 19deg;\
         \n  i20: 20deg;\
         \n  i21: 21deg;\
         \n  i22: 22deg;\
         \n  i23: 23deg;\
         \n  i24: 24deg;\
         \n  i25: 25deg;\
         \n  i26: 26deg;\
         \n  i27: 27deg;\
         \n  i28: 28deg;\
         \n  i29: 29deg;\
         \n  i30: 30deg;\
         \n  i31: 31deg;\
         \n  i32: 32deg;\
         \n  i33: 33deg;\
         \n  i34: 34deg;\
         \n  i35: 35deg;\
         \n  i36: 36deg;\
         \n  i37: 37deg;\
         \n  i38: 38deg;\
         \n  i39: 39deg;\
         \n  i40: 40deg;\
         \n  i41: 41deg;\
         \n  i42: 42deg;\
         \n  i43: 43deg;\
         \n  i44: 44deg;\
         \n  i45: 45deg;\
         \n  i46: 46deg;\
         \n  i47: 47deg;\
         \n  i48: 48deg;\
         \n  i49: 49deg;\
         \n  i50: 50deg;\
         \n  i51: 51deg;\
         \n  i52: 52deg;\
         \n  i53: 53deg;\
         \n  i54: 54deg;\
         \n  i55: 55deg;\
         \n  i56: 56deg;\
         \n  i57: 57deg;\
         \n  i58: 58deg;\
         \n  i59: 59deg;\
         \n  i60: 60deg;\
         \n  i61: 61deg;\
         \n  i62: 62deg;\
         \n  i63: 63deg;\
         \n  i64: 64deg;\
         \n  i65: 65deg;\
         \n  i66: 66deg;\
         \n  i67: 67deg;\
         \n  i68: 68deg;\
         \n  i69: 69deg;\
         \n  i70: 70deg;\
         \n  i71: 71deg;\
         \n  i72: 72deg;\
         \n  i73: 73deg;\
         \n  i74: 74deg;\
         \n  i75: 75deg;\
         \n  i76: 76deg;\
         \n  i77: 77deg;\
         \n  i78: 78deg;\
         \n  i79: 79deg;\
         \n  i80: 80deg;\
         \n  i81: 81deg;\
         \n  i82: 82deg;\
         \n  i83: 83deg;\
         \n  i84: 84deg;\
         \n  i85: 85deg;\
         \n  i86: 86deg;\
         \n  i87: 87deg;\
         \n  i88: 88deg;\
         \n  i89: 89deg;\
         \n  i90: 90deg;\
         \n  i91: 91deg;\
         \n  i92: 92deg;\
         \n  i93: 93deg;\
         \n  i94: 94deg;\
         \n  i95: 95deg;\
         \n  i96: 96deg;\
         \n  i97: 97deg;\
         \n  i98: 98deg;\
         \n  i99: 99deg;\
         \n  i100: 100deg;\
         \n  i101: 101deg;\
         \n  i102: 102deg;\
         \n  i103: 103deg;\
         \n  i104: 104deg;\
         \n  i105: 105deg;\
         \n  i106: 106deg;\
         \n  i107: 107deg;\
         \n  i108: 108deg;\
         \n  i109: 109deg;\
         \n  i110: 110deg;\
         \n  i111: 111deg;\
         \n  i112: 112deg;\
         \n  i113: 113deg;\
         \n  i114: 114deg;\
         \n  i115: 115deg;\
         \n  i116: 116deg;\
         \n  i117: 117deg;\
         \n  i118: 118deg;\
         \n  i119: 119deg;\
         \n  i120: 120deg;\
         \n  i121: 121deg;\
         \n  i122: 122deg;\
         \n  i123: 123deg;\
         \n  i124: 124deg;\
         \n  i125: 125deg;\
         \n  i126: 126deg;\
         \n  i127: 127deg;\
         \n  i128: 128deg;\
         \n  i129: 129deg;\
         \n  i130: 130deg;\
         \n  i131: 131deg;\
         \n  i132: 132deg;\
         \n  i133: 133deg;\
         \n  i134: 134deg;\
         \n  i135: 135deg;\
         \n  i136: 136deg;\
         \n  i137: 137deg;\
         \n  i138: 138deg;\
         \n  i139: 139deg;\
         \n  i140: 140deg;\
         \n  i141: 141deg;\
         \n  i142: 142deg;\
         \n  i143: 143deg;\
         \n  i144: 144deg;\
         \n  i145: 145deg;\
         \n  i146: 146deg;\
         \n  i147: 147deg;\
         \n  i148: 148deg;\
         \n  i149: 149deg;\
         \n  i150: 150deg;\
         \n  i151: 151deg;\
         \n  i152: 152deg;\
         \n  i153: 153deg;\
         \n  i154: 154deg;\
         \n  i155: 155deg;\
         \n  i156: 156deg;\
         \n  i157: 157deg;\
         \n  i158: 158deg;\
         \n  i159: 159deg;\
         \n  i160: 160deg;\
         \n  i161: 161deg;\
         \n  i162: 162deg;\
         \n  i163: 163deg;\
         \n  i164: 164deg;\
         \n  i165: 165deg;\
         \n  i166: 166deg;\
         \n  i167: 167deg;\
         \n  i168: 168deg;\
         \n  i169: 169deg;\
         \n  i170: 170deg;\
         \n  i171: 171deg;\
         \n  i172: 172deg;\
         \n  i173: 173deg;\
         \n  i174: 174deg;\
         \n  i175: 175deg;\
         \n  i176: 176deg;\
         \n  i177: 177deg;\
         \n  i178: 178deg;\
         \n  i179: 179deg;\
         \n  i180: 180deg;\
         \n  i181: 181deg;\
         \n  i182: 182deg;\
         \n  i183: 183deg;\
         \n  i184: 184deg;\
         \n  i185: 185deg;\
         \n  i186: 186deg;\
         \n  i187: 187deg;\
         \n  i188: 188deg;\
         \n  i189: 189deg;\
         \n  i190: 190deg;\
         \n  i191: 191deg;\
         \n  i192: 192deg;\
         \n  i193: 193deg;\
         \n  i194: 194deg;\
         \n  i195: 195deg;\
         \n  i196: 196deg;\
         \n  i197: 197deg;\
         \n  i198: 198deg;\
         \n  i199: 199deg;\
         \n  i200: 200deg;\
         \n  i201: 201deg;\
         \n  i202: 202deg;\
         \n  i203: 203deg;\
         \n  i204: 204deg;\
         \n  i205: 205deg;\
         \n  i206: 206deg;\
         \n  i207: 207deg;\
         \n  i208: 208deg;\
         \n  i209: 209deg;\
         \n  i210: 210deg;\
         \n  i211: 211deg;\
         \n  i212: 212deg;\
         \n  i213: 213deg;\
         \n  i214: 214deg;\
         \n  i215: 215deg;\
         \n  i216: 216deg;\
         \n  i217: 217deg;\
         \n  i218: 218deg;\
         \n  i219: 219deg;\
         \n  i220: 220deg;\
         \n  i221: 221deg;\
         \n  i222: 222deg;\
         \n  i223: 223deg;\
         \n  i224: 224deg;\
         \n  i225: 225deg;\
         \n  i226: 226deg;\
         \n  i227: 227deg;\
         \n  i228: 228deg;\
         \n  i229: 229deg;\
         \n  i230: 230deg;\
         \n  i231: 231deg;\
         \n  i232: 232deg;\
         \n  i233: 233deg;\
         \n  i234: 234deg;\
         \n  i235: 235deg;\
         \n  i236: 236deg;\
         \n  i237: 237deg;\
         \n  i238: 238deg;\
         \n  i239: 239deg;\
         \n  i240: 240deg;\
         \n  i241: 241deg;\
         \n  i242: 242deg;\
         \n  i243: 243deg;\
         \n  i244: 244deg;\
         \n  i245: 245deg;\
         \n  i246: 246deg;\
         \n  i247: 247deg;\
         \n  i248: 248deg;\
         \n  i249: 249deg;\
         \n  i250: 250deg;\
         \n  i251: 251deg;\
         \n  i252: 252deg;\
         \n  i253: 253deg;\
         \n  i254: 254deg;\
         \n  i255: 255deg;\
         \n  i256: 256deg;\
         \n  i257: 257deg;\
         \n  i258: 258deg;\
         \n  i259: 259deg;\
         \n  i260: 260deg;\
         \n  i261: 261deg;\
         \n  i262: 262deg;\
         \n  i263: 263deg;\
         \n  i264: 264deg;\
         \n  i265: 265deg;\
         \n  i266: 266deg;\
         \n  i267: 267deg;\
         \n  i268: 268deg;\
         \n  i269: 269deg;\
         \n  i270: 270deg;\
         \n  i271: 271deg;\
         \n  i272: 272deg;\
         \n  i273: 273deg;\
         \n  i274: 274deg;\
         \n  i275: 275deg;\
         \n  i276: 276deg;\
         \n  i277: 277deg;\
         \n  i278: 278deg;\
         \n  i279: 279deg;\
         \n  i280: 280deg;\
         \n  i281: 281deg;\
         \n  i282: 282deg;\
         \n  i283: 283deg;\
         \n  i284: 284deg;\
         \n  i285: 285deg;\
         \n  i286: 286deg;\
         \n  i287: 287deg;\
         \n  i288: 288deg;\
         \n  i289: 289deg;\
         \n  i290: 290deg;\
         \n  i291: 291deg;\
         \n  i292: 292deg;\
         \n  i293: 293deg;\
         \n  i294: 294deg;\
         \n  i295: 295deg;\
         \n  i296: 296deg;\
         \n  i297: 297deg;\
         \n  i298: 298deg;\
         \n  i299: 299deg;\
         \n  i300: 300deg;\
         \n  i301: 301deg;\
         \n  i302: 302deg;\
         \n  i303: 303deg;\
         \n  i304: 304deg;\
         \n  i305: 305deg;\
         \n  i306: 306deg;\
         \n  i307: 307deg;\
         \n  i308: 308deg;\
         \n  i309: 309deg;\
         \n  i310: 310deg;\
         \n  i311: 311deg;\
         \n  i312: 312deg;\
         \n  i313: 313deg;\
         \n  i314: 314deg;\
         \n  i315: 315deg;\
         \n  i316: 316deg;\
         \n  i317: 317deg;\
         \n  i318: 318deg;\
         \n  i319: 319deg;\
         \n  i320: 320deg;\
         \n  i321: 321deg;\
         \n  i322: 322deg;\
         \n  i323: 323deg;\
         \n  i324: 324deg;\
         \n  i325: 325deg;\
         \n  i326: 326deg;\
         \n  i327: 327deg;\
         \n  i328: 328deg;\
         \n  i329: 329deg;\
         \n  i330: 330deg;\
         \n  i331: 331deg;\
         \n  i332: 332deg;\
         \n  i333: 333deg;\
         \n  i334: 334deg;\
         \n  i335: 335deg;\
         \n  i336: 336deg;\
         \n  i337: 337deg;\
         \n  i338: 338deg;\
         \n  i339: 339deg;\
         \n  i340: 340deg;\
         \n  i341: 341deg;\
         \n  i342: 342deg;\
         \n  i343: 343deg;\
         \n  i344: 344deg;\
         \n  i345: 345deg;\
         \n  i346: 346deg;\
         \n  i347: 347deg;\
         \n  i348: 348deg;\
         \n  i349: 349deg;\
         \n  i350: 350deg;\
         \n  i351: 351deg;\
         \n  i352: 352deg;\
         \n  i353: 353deg;\
         \n  i354: 354deg;\
         \n  i355: 355deg;\
         \n  i356: 356deg;\
         \n  i357: 357deg;\
         \n  i358: 358deg;\
         \n  i359: 359deg;\
         \n  i360: 0deg;\
         \n}\n"
    );
}
