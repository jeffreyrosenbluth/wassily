use tiny_skia::Color;

pub struct Kolor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const fn kolor(r: u8, g: u8, b: u8) -> Kolor {
    Kolor { r, g, b, a: 255 }
}

impl From<Kolor> for Color {
    fn from(k: Kolor) -> Self {
        Color::from_rgba8(k.r, k.g, k.b, k.a)
    }
}

pub const ALICEBLUE: Kolor = kolor(240, 248, 255);
pub const ANTIQUEWHITE: Kolor = kolor(250, 235, 215);
pub const AQUA: Kolor = kolor(0, 255, 255);
pub const AQUAMARINE: Kolor = kolor(127, 255, 212);
pub const AZURE: Kolor = kolor(240, 255, 255);
pub const BEIGE: Kolor = kolor(245, 245, 220);
pub const BISQUE: Kolor = kolor(255, 228, 196);
pub const BLACK: Kolor = kolor(0, 0, 0);
pub const BLANCHEDALMOND: Kolor = kolor(255, 235, 205);
pub const BLUE: Kolor = kolor(0, 0, 255);
pub const BLUEVIOLET: Kolor = kolor(138, 43, 226);
pub const BROWN: Kolor = kolor(165, 42, 42);
pub const BURLYWOOD: Kolor = kolor(222, 184, 135);
pub const CADETBLUE: Kolor = kolor(95, 158, 160);
pub const CHARTREUSE: Kolor = kolor(127, 255, 0);
pub const CHOCOLATE: Kolor = kolor(210, 105, 30);
pub const CORAL: Kolor = kolor(255, 127, 80);
pub const CORNFLOWERBLUE: Kolor = kolor(100, 149, 237);
pub const CORNSILK: Kolor = kolor(255, 248, 220);
pub const CRIMSON: Kolor = kolor(220, 20, 60);
pub const CYAN: Kolor = kolor(0, 255, 255);
pub const DARKBLUE: Kolor = kolor(0, 0, 139);
pub const DARKCYAN: Kolor = kolor(0, 139, 139);
pub const DARKGOLDENROD: Kolor = kolor(184, 134, 11);
pub const DARKGRAY: Kolor = kolor(169, 169, 169);
pub const DARKGREEN: Kolor = kolor(0, 100, 0);
pub const DARKGREY: Kolor = kolor(169, 169, 169);
pub const DARKKHAKI: Kolor = kolor(189, 183, 107);
pub const DARKMAGENTA: Kolor = kolor(139, 0, 139);
pub const DARKOLIVEGREEN: Kolor = kolor(85, 107, 47);
pub const DARKORANGE: Kolor = kolor(255, 140, 0);
pub const DARKORCHID: Kolor = kolor(153, 50, 204);
pub const DARKRED: Kolor = kolor(139, 0, 0);
pub const DARKSALMON: Kolor = kolor(233, 150, 122);
pub const DARKSEAGREEN: Kolor = kolor(143, 188, 143);
pub const DARKSLATEBLUE: Kolor = kolor(72, 61, 139);
pub const DARKSLATEGRAY: Kolor = kolor(47, 79, 79);
pub const DARKSLATEGREY: Kolor = kolor(47, 79, 79);
pub const DARKTURQUOISE: Kolor = kolor(0, 206, 209);
pub const DARKVIOLET: Kolor = kolor(148, 0, 211);
pub const DEEPPINK: Kolor = kolor(255, 20, 147);
pub const DEEPSKYBLUE: Kolor = kolor(0, 191, 255);
pub const DIMGRAY: Kolor = kolor(105, 105, 105);
pub const DIMGREY: Kolor = kolor(105, 105, 105);
pub const DODGERBLUE: Kolor = kolor(30, 144, 255);
pub const FIREBRICK: Kolor = kolor(178, 34, 34);
pub const FLORALWHITE: Kolor = kolor(255, 250, 240);
pub const FORESTGREEN: Kolor = kolor(34, 139, 34);
pub const FUCHSIA: Kolor = kolor(255, 0, 255);
pub const GAINSBORO: Kolor = kolor(220, 220, 220);
pub const GHOSTWHITE: Kolor = kolor(248, 248, 255);
pub const GOLD: Kolor = kolor(255, 215, 0);
pub const GOLDENROD: Kolor = kolor(218, 165, 32);
pub const GRAY: Kolor = kolor(128, 128, 128);
pub const GREEN: Kolor = kolor(0, 128, 0);
pub const GREENYELLOW: Kolor = kolor(173, 255, 47);
pub const GREY: Kolor = kolor(128, 128, 128);
pub const HONEYDEW: Kolor = kolor(240, 255, 240);
pub const HOTPINK: Kolor = kolor(255, 105, 180);
pub const INDIANRED: Kolor = kolor(205, 92, 92);
pub const INDIGO: Kolor = kolor(75, 0, 130);
pub const IVORY: Kolor = kolor(255, 255, 240);
pub const KHAKI: Kolor = kolor(240, 230, 140);
pub const LAVENDER: Kolor = kolor(230, 230, 250);
pub const LAVENDERBLUSH: Kolor = kolor(255, 240, 245);
pub const LAWNGREEN: Kolor = kolor(124, 252, 0);
pub const LEMONCHIFFON: Kolor = kolor(255, 250, 205);
pub const LIGHTBLUE: Kolor = kolor(173, 216, 230);
pub const LIGHTCORAL: Kolor = kolor(240, 128, 128);
pub const LIGHTCYAN: Kolor = kolor(224, 255, 255);
pub const LIGHTGOLDENRODYELLOW: Kolor = kolor(250, 250, 210);
pub const LIGHTGRAY: Kolor = kolor(211, 211, 211);
pub const LIGHTGREEN: Kolor = kolor(144, 238, 144);
pub const LIGHTGREY: Kolor = kolor(211, 211, 211);
pub const LIGHTPINK: Kolor = kolor(255, 182, 193);
pub const LIGHTSALMON: Kolor = kolor(255, 160, 122);
pub const LIGHTSEAGREEN: Kolor = kolor(32, 178, 170);
pub const LIGHTSKYBLUE: Kolor = kolor(135, 206, 250);
pub const LIGHTSLATEGRAY: Kolor = kolor(119, 136, 153);
pub const LIGHTSLATEGREY: Kolor = kolor(119, 136, 153);
pub const LIGHTSTEELBLUE: Kolor = kolor(176, 196, 222);
pub const LIGHTYELLOW: Kolor = kolor(255, 255, 224);
pub const LIME: Kolor = kolor(0, 255, 0);
pub const LIMEGREEN: Kolor = kolor(50, 205, 50);
pub const LINEN: Kolor = kolor(250, 240, 230);
pub const MAGENTA: Kolor = kolor(255, 0, 255);
pub const MAROON: Kolor = kolor(128, 0, 0);
pub const MEDIUMAQUAMARINE: Kolor = kolor(102, 205, 170);
pub const MEDIUMBLUE: Kolor = kolor(0, 0, 205);
pub const MEDIUMORCHID: Kolor = kolor(186, 85, 211);
pub const MEDIUMPURPLE: Kolor = kolor(147, 112, 219);
pub const MEDIUMSEAGREEN: Kolor = kolor(60, 179, 113);
pub const MEDIUMSLATEBLUE: Kolor = kolor(123, 104, 238);
pub const MEDIUMSPRINGGREEN: Kolor = kolor(0, 250, 154);
pub const MEDIUMTURQUOISE: Kolor = kolor(72, 209, 204);
pub const MEDIUMVIOLETRED: Kolor = kolor(199, 21, 133);
pub const MIDNIGHTBLUE: Kolor = kolor(25, 25, 112);
pub const MINTCREAM: Kolor = kolor(245, 255, 250);
pub const MISTYROSE: Kolor = kolor(255, 228, 225);
pub const MOCCASIN: Kolor = kolor(255, 228, 181);
pub const NAVAJOWHITE: Kolor = kolor(255, 222, 173);
pub const NAVY: Kolor = kolor(0, 0, 128);
pub const OLDLACE: Kolor = kolor(253, 245, 230);
pub const OLIVE: Kolor = kolor(128, 128, 0);
pub const OLIVEDRAB: Kolor = kolor(107, 142, 35);
pub const ORANGE: Kolor = kolor(255, 165, 0);
pub const ORANGERED: Kolor = kolor(255, 69, 0);
pub const ORCHID: Kolor = kolor(218, 112, 214);
pub const PALEGOLDENROD: Kolor = kolor(238, 232, 170);
pub const PALEGREEN: Kolor = kolor(152, 251, 152);
pub const PALETURQUOISE: Kolor = kolor(175, 238, 238);
pub const PALEVIOLETRED: Kolor = kolor(219, 112, 147);
pub const PAPAYAWHIP: Kolor = kolor(255, 239, 213);
pub const PEACHPUFF: Kolor = kolor(255, 218, 185);
pub const PERU: Kolor = kolor(205, 133, 63);
pub const PINK: Kolor = kolor(255, 192, 203);
pub const PLUM: Kolor = kolor(221, 160, 221);
pub const POWDERBLUE: Kolor = kolor(176, 224, 230);
pub const PURPLE: Kolor = kolor(128, 0, 128);
pub const REBECCAPURPLE: Kolor = kolor(102, 51, 153);
pub const RED: Kolor = kolor(255, 0, 0);
pub const ROSYBROWN: Kolor = kolor(188, 143, 143);
pub const ROYALBLUE: Kolor = kolor(65, 105, 225);
pub const SADDLEBROWN: Kolor = kolor(139, 69, 19);
pub const SALMON: Kolor = kolor(250, 128, 114);
pub const SANDYBROWN: Kolor = kolor(244, 164, 96);
pub const SEAGREEN: Kolor = kolor(46, 139, 87);
pub const SEASHELL: Kolor = kolor(255, 245, 238);
pub const SIENNA: Kolor = kolor(160, 82, 45);
pub const SILVER: Kolor = kolor(192, 192, 192);
pub const SKYBLUE: Kolor = kolor(135, 206, 235);
pub const SLATEBLUE: Kolor = kolor(106, 90, 205);
pub const SLATEGRAY: Kolor = kolor(112, 128, 144);
pub const SLATEGREY: Kolor = kolor(112, 128, 144);
pub const SNOW: Kolor = kolor(255, 250, 250);
pub const SPRINGGREEN: Kolor = kolor(0, 255, 127);
pub const STEELBLUE: Kolor = kolor(70, 130, 180);
pub const TAN: Kolor = kolor(210, 180, 140);
pub const TEAL: Kolor = kolor(0, 128, 128);
pub const THISTLE: Kolor = kolor(216, 191, 216);
pub const TOMATO: Kolor = kolor(255, 99, 71);
pub const TURQUOISE: Kolor = kolor(64, 224, 208);
pub const VIOLET: Kolor = kolor(238, 130, 238);
pub const WHEAT: Kolor = kolor(245, 222, 179);
pub const WHITE: Kolor = kolor(255, 255, 255);
pub const WHITESMOKE: Kolor = kolor(245, 245, 245);
pub const YELLOW: Kolor = kolor(255, 255, 0);
pub const YELLOWGREEN: Kolor = kolor(154, 205, 50);
