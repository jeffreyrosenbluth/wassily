//! HTML Color Names
//! ## Example
//! ```rust
//! Shape::new().fill_color(*CORNFLOWERBLUE);
//!
use crate::kolor::rgb8;
use once_cell::sync::Lazy;
use tiny_skia::Color;

pub static ALICEBLUE: Lazy<Color> = Lazy::new(|| rgb8(240, 248, 255));
pub static ANTIQUEWHITE: Lazy<Color> = Lazy::new(|| rgb8(250, 235, 215));
pub static AQUA: Lazy<Color> = Lazy::new(|| rgb8(0, 255, 255));
pub static AQUAMARINE: Lazy<Color> = Lazy::new(|| rgb8(127, 255, 212));
pub static AZURE: Lazy<Color> = Lazy::new(|| rgb8(240, 255, 255));
pub static BEIGE: Lazy<Color> = Lazy::new(|| rgb8(245, 245, 220));
pub static BISQUE: Lazy<Color> = Lazy::new(|| rgb8(255, 228, 196));
pub static BLACK: Lazy<Color> = Lazy::new(|| rgb8(0, 0, 0));
pub static BLANCHEDALMOND: Lazy<Color> = Lazy::new(|| rgb8(255, 235, 205));
pub static BLUE: Lazy<Color> = Lazy::new(|| rgb8(0, 0, 255));
pub static BLUEVIOLET: Lazy<Color> = Lazy::new(|| rgb8(138, 43, 226));
pub static BROWN: Lazy<Color> = Lazy::new(|| rgb8(165, 42, 42));
pub static BURLYWOOD: Lazy<Color> = Lazy::new(|| rgb8(222, 184, 135));
pub static CADETBLUE: Lazy<Color> = Lazy::new(|| rgb8(95, 158, 160));
pub static CHARTREUSE: Lazy<Color> = Lazy::new(|| rgb8(127, 255, 0));
pub static CHOCOLATE: Lazy<Color> = Lazy::new(|| rgb8(210, 105, 30));
pub static CORAL: Lazy<Color> = Lazy::new(|| rgb8(255, 127, 80));
pub static CORNFLOWERBLUE: Lazy<Color> = Lazy::new(|| rgb8(100, 149, 237));
pub static CORNSILK: Lazy<Color> = Lazy::new(|| rgb8(255, 248, 220));
pub static CRIMSON: Lazy<Color> = Lazy::new(|| rgb8(220, 20, 60));
pub static CYAN: Lazy<Color> = Lazy::new(|| rgb8(0, 255, 255));
pub static DARKBLUE: Lazy<Color> = Lazy::new(|| rgb8(0, 0, 139));
pub static DARKCYAN: Lazy<Color> = Lazy::new(|| rgb8(0, 139, 139));
pub static DARKGOLDENROD: Lazy<Color> = Lazy::new(|| rgb8(184, 134, 11));
pub static DARKGRAY: Lazy<Color> = Lazy::new(|| rgb8(169, 169, 169));
pub static DARKGREEN: Lazy<Color> = Lazy::new(|| rgb8(0, 100, 0));
pub static DARKGREY: Lazy<Color> = Lazy::new(|| rgb8(169, 169, 169));
pub static DARKKHAKI: Lazy<Color> = Lazy::new(|| rgb8(189, 183, 107));
pub static DARKMAGENTA: Lazy<Color> = Lazy::new(|| rgb8(139, 0, 139));
pub static DARKOLIVEGREEN: Lazy<Color> = Lazy::new(|| rgb8(85, 107, 47));
pub static DARKORANGE: Lazy<Color> = Lazy::new(|| rgb8(255, 140, 0));
pub static DARKORCHID: Lazy<Color> = Lazy::new(|| rgb8(153, 50, 204));
pub static DARKRED: Lazy<Color> = Lazy::new(|| rgb8(139, 0, 0));
pub static DARKSALMON: Lazy<Color> = Lazy::new(|| rgb8(233, 150, 122));
pub static DARKSEAGREEN: Lazy<Color> = Lazy::new(|| rgb8(143, 188, 143));
pub static DARKSLATEBLUE: Lazy<Color> = Lazy::new(|| rgb8(72, 61, 139));
pub static DARKSLATEGRAY: Lazy<Color> = Lazy::new(|| rgb8(47, 79, 79));
pub static DARKSLATEGREY: Lazy<Color> = Lazy::new(|| rgb8(47, 79, 79));
pub static DARKTURQUOISE: Lazy<Color> = Lazy::new(|| rgb8(0, 206, 209));
pub static DARKVIOLET: Lazy<Color> = Lazy::new(|| rgb8(148, 0, 211));
pub static DEEPPINK: Lazy<Color> = Lazy::new(|| rgb8(255, 20, 147));
pub static DEEPSKYBLUE: Lazy<Color> = Lazy::new(|| rgb8(0, 191, 255));
pub static DIMGRAY: Lazy<Color> = Lazy::new(|| rgb8(105, 105, 105));
pub static DIMGREY: Lazy<Color> = Lazy::new(|| rgb8(105, 105, 105));
pub static DODGERBLUE: Lazy<Color> = Lazy::new(|| rgb8(30, 144, 255));
pub static FIREBRICK: Lazy<Color> = Lazy::new(|| rgb8(178, 34, 34));
pub static FLORALWHITE: Lazy<Color> = Lazy::new(|| rgb8(255, 250, 240));
pub static FORESTGREEN: Lazy<Color> = Lazy::new(|| rgb8(34, 139, 34));
pub static FUCHSIA: Lazy<Color> = Lazy::new(|| rgb8(255, 0, 255));
pub static GAINSBORO: Lazy<Color> = Lazy::new(|| rgb8(220, 220, 220));
pub static GHOSTWHITE: Lazy<Color> = Lazy::new(|| rgb8(248, 248, 255));
pub static GOLD: Lazy<Color> = Lazy::new(|| rgb8(255, 215, 0));
pub static GOLDENROD: Lazy<Color> = Lazy::new(|| rgb8(218, 165, 32));
pub static GRAY: Lazy<Color> = Lazy::new(|| rgb8(128, 128, 128));
pub static GREEN: Lazy<Color> = Lazy::new(|| rgb8(0, 128, 0));
pub static GREENYELLOW: Lazy<Color> = Lazy::new(|| rgb8(173, 255, 47));
pub static GREY: Lazy<Color> = Lazy::new(|| rgb8(128, 128, 128));
pub static HONEYDEW: Lazy<Color> = Lazy::new(|| rgb8(240, 255, 240));
pub static HOTPINK: Lazy<Color> = Lazy::new(|| rgb8(255, 105, 180));
pub static INDIANRED: Lazy<Color> = Lazy::new(|| rgb8(205, 92, 92));
pub static INDIGO: Lazy<Color> = Lazy::new(|| rgb8(75, 0, 130));
pub static IVORY: Lazy<Color> = Lazy::new(|| rgb8(255, 255, 240));
pub static KHAKI: Lazy<Color> = Lazy::new(|| rgb8(240, 230, 140));
pub static LAVENDER: Lazy<Color> = Lazy::new(|| rgb8(230, 230, 250));
pub static LAVENDERBLUSH: Lazy<Color> = Lazy::new(|| rgb8(255, 240, 245));
pub static LAWNGREEN: Lazy<Color> = Lazy::new(|| rgb8(124, 252, 0));
pub static LEMONCHIFFON: Lazy<Color> = Lazy::new(|| rgb8(255, 250, 205));
pub static LIGHTBLUE: Lazy<Color> = Lazy::new(|| rgb8(173, 216, 230));
pub static LIGHTCORAL: Lazy<Color> = Lazy::new(|| rgb8(240, 128, 128));
pub static LIGHTCYAN: Lazy<Color> = Lazy::new(|| rgb8(224, 255, 255));
pub static LIGHTGOLDENRODYELLOW: Lazy<Color> = Lazy::new(|| rgb8(250, 250, 210));
pub static LIGHTGRAY: Lazy<Color> = Lazy::new(|| rgb8(211, 211, 211));
pub static LIGHTGREEN: Lazy<Color> = Lazy::new(|| rgb8(144, 238, 144));
pub static LIGHTGREY: Lazy<Color> = Lazy::new(|| rgb8(211, 211, 211));
pub static LIGHTPINK: Lazy<Color> = Lazy::new(|| rgb8(255, 182, 193));
pub static LIGHTSALMON: Lazy<Color> = Lazy::new(|| rgb8(255, 160, 122));
pub static LIGHTSEAGREEN: Lazy<Color> = Lazy::new(|| rgb8(32, 178, 170));
pub static LIGHTSKYBLUE: Lazy<Color> = Lazy::new(|| rgb8(135, 206, 250));
pub static LIGHTSLATEGRAY: Lazy<Color> = Lazy::new(|| rgb8(119, 136, 153));
pub static LIGHTSLATEGREY: Lazy<Color> = Lazy::new(|| rgb8(119, 136, 153));
pub static LIGHTSTEELBLUE: Lazy<Color> = Lazy::new(|| rgb8(176, 196, 222));
pub static LIGHTYELLOW: Lazy<Color> = Lazy::new(|| rgb8(255, 255, 224));
pub static LIME: Lazy<Color> = Lazy::new(|| rgb8(0, 255, 0));
pub static LIMEGREEN: Lazy<Color> = Lazy::new(|| rgb8(50, 205, 50));
pub static LINEN: Lazy<Color> = Lazy::new(|| rgb8(250, 240, 230));
pub static MAGENTA: Lazy<Color> = Lazy::new(|| rgb8(255, 0, 255));
pub static MAROON: Lazy<Color> = Lazy::new(|| rgb8(128, 0, 0));
pub static MEDIUMAQUAMARINE: Lazy<Color> = Lazy::new(|| rgb8(102, 205, 170));
pub static MEDIUMBLUE: Lazy<Color> = Lazy::new(|| rgb8(0, 0, 205));
pub static MEDIUMORCHID: Lazy<Color> = Lazy::new(|| rgb8(186, 85, 211));
pub static MEDIUMPURPLE: Lazy<Color> = Lazy::new(|| rgb8(147, 112, 219));
pub static MEDIUMSEAGREEN: Lazy<Color> = Lazy::new(|| rgb8(60, 179, 113));
pub static MEDIUMSLATEBLUE: Lazy<Color> = Lazy::new(|| rgb8(123, 104, 238));
pub static MEDIUMSPRINGGREEN: Lazy<Color> = Lazy::new(|| rgb8(0, 250, 154));
pub static MEDIUMTURQUOISE: Lazy<Color> = Lazy::new(|| rgb8(72, 209, 204));
pub static MEDIUMVIOLETRED: Lazy<Color> = Lazy::new(|| rgb8(199, 21, 133));
pub static MIDNIGHTBLUE: Lazy<Color> = Lazy::new(|| rgb8(25, 25, 112));
pub static MINTCREAM: Lazy<Color> = Lazy::new(|| rgb8(245, 255, 250));
pub static MISTYROSE: Lazy<Color> = Lazy::new(|| rgb8(255, 228, 225));
pub static MOCCASIN: Lazy<Color> = Lazy::new(|| rgb8(255, 228, 181));
pub static NAVAJOWHITE: Lazy<Color> = Lazy::new(|| rgb8(255, 222, 173));
pub static NAVY: Lazy<Color> = Lazy::new(|| rgb8(0, 0, 128));
pub static OLDLACE: Lazy<Color> = Lazy::new(|| rgb8(253, 245, 230));
pub static OLIVE: Lazy<Color> = Lazy::new(|| rgb8(128, 128, 0));
pub static OLIVEDRAB: Lazy<Color> = Lazy::new(|| rgb8(107, 142, 35));
pub static ORANGE: Lazy<Color> = Lazy::new(|| rgb8(255, 165, 0));
pub static ORANGERED: Lazy<Color> = Lazy::new(|| rgb8(255, 69, 0));
pub static ORCHID: Lazy<Color> = Lazy::new(|| rgb8(218, 112, 214));
pub static PALEGOLDENROD: Lazy<Color> = Lazy::new(|| rgb8(238, 232, 170));
pub static PALEGREEN: Lazy<Color> = Lazy::new(|| rgb8(152, 251, 152));
pub static PALETURQUOISE: Lazy<Color> = Lazy::new(|| rgb8(175, 238, 238));
pub static PALEVIOLETRED: Lazy<Color> = Lazy::new(|| rgb8(219, 112, 147));
pub static PAPAYAWHIP: Lazy<Color> = Lazy::new(|| rgb8(255, 239, 213));
pub static PEACHPUFF: Lazy<Color> = Lazy::new(|| rgb8(255, 218, 185));
pub static PERU: Lazy<Color> = Lazy::new(|| rgb8(205, 133, 63));
pub static PINK: Lazy<Color> = Lazy::new(|| rgb8(255, 192, 203));
pub static PLUM: Lazy<Color> = Lazy::new(|| rgb8(221, 160, 221));
pub static POWDERBLUE: Lazy<Color> = Lazy::new(|| rgb8(176, 224, 230));
pub static PURPLE: Lazy<Color> = Lazy::new(|| rgb8(128, 0, 128));
pub static REBECCAPURPLE: Lazy<Color> = Lazy::new(|| rgb8(102, 51, 153));
pub static RED: Lazy<Color> = Lazy::new(|| rgb8(255, 0, 0));
pub static ROSYBROWN: Lazy<Color> = Lazy::new(|| rgb8(188, 143, 143));
pub static ROYALBLUE: Lazy<Color> = Lazy::new(|| rgb8(65, 105, 225));
pub static SADDLEBROWN: Lazy<Color> = Lazy::new(|| rgb8(139, 69, 19));
pub static SALMON: Lazy<Color> = Lazy::new(|| rgb8(250, 128, 114));
pub static SANDYBROWN: Lazy<Color> = Lazy::new(|| rgb8(244, 164, 96));
pub static SEAGREEN: Lazy<Color> = Lazy::new(|| rgb8(46, 139, 87));
pub static SEASHELL: Lazy<Color> = Lazy::new(|| rgb8(255, 245, 238));
pub static SIENNA: Lazy<Color> = Lazy::new(|| rgb8(160, 82, 45));
pub static SILVER: Lazy<Color> = Lazy::new(|| rgb8(192, 192, 192));
pub static SKYBLUE: Lazy<Color> = Lazy::new(|| rgb8(135, 206, 235));
pub static SLATEBLUE: Lazy<Color> = Lazy::new(|| rgb8(106, 90, 205));
pub static SLATEGRAY: Lazy<Color> = Lazy::new(|| rgb8(112, 128, 144));
pub static SLATEGREY: Lazy<Color> = Lazy::new(|| rgb8(112, 128, 144));
pub static SNOW: Lazy<Color> = Lazy::new(|| rgb8(255, 250, 250));
pub static SPRINGGREEN: Lazy<Color> = Lazy::new(|| rgb8(0, 255, 127));
pub static STEELBLUE: Lazy<Color> = Lazy::new(|| rgb8(70, 130, 180));
pub static TAN: Lazy<Color> = Lazy::new(|| rgb8(210, 180, 140));
pub static TEAL: Lazy<Color> = Lazy::new(|| rgb8(0, 128, 128));
pub static THISTLE: Lazy<Color> = Lazy::new(|| rgb8(216, 191, 216));
pub static TOMATO: Lazy<Color> = Lazy::new(|| rgb8(255, 99, 71));
pub static TURQUOISE: Lazy<Color> = Lazy::new(|| rgb8(64, 224, 208));
pub static VIOLET: Lazy<Color> = Lazy::new(|| rgb8(238, 130, 238));
pub static WHEAT: Lazy<Color> = Lazy::new(|| rgb8(245, 222, 179));
pub static WHITE: Lazy<Color> = Lazy::new(|| rgb8(255, 255, 255));
pub static WHITESMOKE: Lazy<Color> = Lazy::new(|| rgb8(245, 245, 245));
pub static YELLOW: Lazy<Color> = Lazy::new(|| rgb8(255, 255, 0));
pub static YELLOWGREEN: Lazy<Color> = Lazy::new(|| rgb8(154, 205, 50));

pub fn alice_blue() -> Color {
    rgb8(240, 248, 255)
}
pub fn antique_white() -> Color {
    rgb8(250, 235, 215)
}
pub fn aqua() -> Color {
    rgb8(0, 255, 255)
}
pub fn aquamarine() -> Color {
    rgb8(127, 255, 212)
}
pub fn azure() -> Color {
    rgb8(240, 255, 255)
}
pub fn beige() -> Color {
    rgb8(245, 245, 220)
}
pub fn bisque() -> Color {
    rgb8(255, 228, 196)
}
pub fn black() -> Color {
    rgb8(0, 0, 0)
}
pub fn blanched_almond() -> Color {
    rgb8(255, 235, 205)
}
pub fn blue() -> Color {
    rgb8(0, 0, 255)
}
pub fn blue_violet() -> Color {
    rgb8(138, 43, 226)
}
pub fn brown() -> Color {
    rgb8(165, 42, 42)
}
pub fn burlywood() -> Color {
    rgb8(222, 184, 135)
}
pub fn cadet_blue() -> Color {
    rgb8(95, 158, 160)
}
pub fn chartreuse() -> Color {
    rgb8(127, 255, 0)
}
pub fn chocolate() -> Color {
    rgb8(210, 105, 30)
}
pub fn coral() -> Color {
    rgb8(255, 127, 80)
}
pub fn cornflower_blue() -> Color {
    rgb8(100, 149, 237)
}
pub fn cornsilk() -> Color {
    rgb8(255, 248, 220)
}
pub fn crimson() -> Color {
    rgb8(220, 20, 60)
}
pub fn cyan() -> Color {
    rgb8(0, 255, 255)
}
pub fn dark_blue() -> Color {
    rgb8(0, 0, 139)
}
pub fn dark_cyan() -> Color {
    rgb8(0, 139, 139)
}
pub fn dark_goldenrod() -> Color {
    rgb8(184, 134, 11)
}
pub fn dark_gray() -> Color {
    rgb8(169, 169, 169)
}
pub fn dark_green() -> Color {
    rgb8(0, 100, 0)
}
pub fn dark_grey() -> Color {
    rgb8(169, 169, 169)
}
pub fn dark_khaki() -> Color {
    rgb8(189, 183, 107)
}
pub fn dark_magenta() -> Color {
    rgb8(139, 0, 139)
}
pub fn dark_olive_green() -> Color {
    rgb8(85, 107, 47)
}
pub fn dark_orange() -> Color {
    rgb8(255, 140, 0)
}
pub fn dark_orchid() -> Color {
    rgb8(153, 50, 204)
}
pub fn dark_red() -> Color {
    rgb8(139, 0, 0)
}
pub fn dark_salmon() -> Color {
    rgb8(233, 150, 122)
}
pub fn dark_sea_green() -> Color {
    rgb8(143, 188, 143)
}
pub fn dark_slate_blue() -> Color {
    rgb8(72, 61, 139)
}
pub fn dark_slate_gray() -> Color {
    rgb8(47, 79, 79)
}
pub fn dark_slate_grey() -> Color {
    rgb8(47, 79, 79)
}
pub fn dark_turquoise() -> Color {
    rgb8(0, 206, 209)
}
pub fn dark_violet() -> Color {
    rgb8(148, 0, 211)
}
pub fn deep_pink() -> Color {
    rgb8(255, 20, 147)
}
pub fn deep_sky_blue() -> Color {
    rgb8(0, 191, 255)
}
pub fn dim_gray() -> Color {
    rgb8(105, 105, 105)
}
pub fn dim_grey() -> Color {
    rgb8(105, 105, 105)
}
pub fn dodger_blue() -> Color {
    rgb8(30, 144, 255)
}
pub fn firebrick() -> Color {
    rgb8(178, 34, 34)
}
pub fn floral_white() -> Color {
    rgb8(255, 250, 240)
}
pub fn forest_green() -> Color {
    rgb8(34, 139, 34)
}
pub fn fuchsia() -> Color {
    rgb8(255, 0, 255)
}
pub fn gainsboro() -> Color {
    rgb8(220, 220, 220)
}
pub fn ghost_white() -> Color {
    rgb8(248, 248, 255)
}
pub fn gold() -> Color {
    rgb8(255, 215, 0)
}
pub fn goldenrod() -> Color {
    rgb8(218, 165, 32)
}
pub fn gray() -> Color {
    rgb8(128, 128, 128)
}
pub fn green() -> Color {
    rgb8(0, 128, 0)
}
pub fn green_yellow() -> Color {
    rgb8(173, 255, 47)
}
pub fn grey() -> Color {
    rgb8(128, 128, 128)
}
pub fn honeydew() -> Color {
    rgb8(240, 255, 240)
}
pub fn hot_pink() -> Color {
    rgb8(255, 105, 180)
}
pub fn indian_red() -> Color {
    rgb8(205, 92, 92)
}
pub fn indigo() -> Color {
    rgb8(75, 0, 130)
}
pub fn ivory() -> Color {
    rgb8(255, 255, 240)
}
pub fn khaki() -> Color {
    rgb8(240, 230, 140)
}
pub fn lavender() -> Color {
    rgb8(230, 230, 250)
}
pub fn lavender_blush() -> Color {
    rgb8(255, 240, 245)
}
pub fn lawn_green() -> Color {
    rgb8(124, 252, 0)
}
pub fn lemon_chiffon() -> Color {
    rgb8(255, 250, 205)
}
pub fn light_blue() -> Color {
    rgb8(173, 216, 230)
}
pub fn light_coral() -> Color {
    rgb8(240, 128, 128)
}
pub fn light_cyan() -> Color {
    rgb8(224, 255, 255)
}
pub fn light_goldenrod_yellow() -> Color {
    rgb8(250, 250, 210)
}
pub fn light_gray() -> Color {
    rgb8(211, 211, 211)
}
pub fn light_green() -> Color {
    rgb8(144, 238, 144)
}
pub fn light_grey() -> Color {
    rgb8(211, 211, 211)
}
pub fn light_pink() -> Color {
    rgb8(255, 182, 193)
}
pub fn light_salmon() -> Color {
    rgb8(255, 160, 122)
}
pub fn light_sea_green() -> Color {
    rgb8(32, 178, 170)
}
pub fn light_sky_blue() -> Color {
    rgb8(135, 206, 250)
}
pub fn light_slate_gray() -> Color {
    rgb8(119, 136, 153)
}
pub fn light_slate_grey() -> Color {
    rgb8(119, 136, 153)
}
pub fn light_steel_blue() -> Color {
    rgb8(176, 196, 222)
}
pub fn light_yellow() -> Color {
    rgb8(255, 255, 224)
}
pub fn lime() -> Color {
    rgb8(0, 255, 0)
}
pub fn lime_green() -> Color {
    rgb8(50, 205, 50)
}
pub fn linen() -> Color {
    rgb8(250, 240, 230)
}
pub fn magenta() -> Color {
    rgb8(255, 0, 255)
}
pub fn maroon() -> Color {
    rgb8(128, 0, 0)
}
pub fn medium_aquamarine() -> Color {
    rgb8(102, 205, 170)
}
pub fn medium_blue() -> Color {
    rgb8(0, 0, 205)
}
pub fn medium_orchid() -> Color {
    rgb8(186, 85, 211)
}
pub fn medium_purple() -> Color {
    rgb8(147, 112, 219)
}
pub fn medium_sea_green() -> Color {
    rgb8(60, 179, 113)
}
pub fn medium_slate_blue() -> Color {
    rgb8(123, 104, 238)
}
pub fn medium_spring_green() -> Color {
    rgb8(0, 250, 154)
}
pub fn medium_turquoise() -> Color {
    rgb8(72, 209, 204)
}
pub fn medium_violet_red() -> Color {
    rgb8(199, 21, 133)
}
pub fn midnight_blue() -> Color {
    rgb8(25, 25, 112)
}
pub fn mint_cream() -> Color {
    rgb8(245, 255, 250)
}
pub fn misty_rose() -> Color {
    rgb8(255, 228, 225)
}
pub fn moccasin() -> Color {
    rgb8(255, 228, 181)
}
pub fn navajo_white() -> Color {
    rgb8(255, 222, 173)
}
pub fn navy() -> Color {
    rgb8(0, 0, 128)
}
pub fn old_lace() -> Color {
    rgb8(253, 245, 230)
}
pub fn olive() -> Color {
    rgb8(128, 128, 0)
}
pub fn olive_drab() -> Color {
    rgb8(107, 142, 35)
}
pub fn orange() -> Color {
    rgb8(255, 165, 0)
}
pub fn orange_red() -> Color {
    rgb8(255, 69, 0)
}
pub fn orchid() -> Color {
    rgb8(218, 112, 214)
}
pub fn pale_goldenrod() -> Color {
    rgb8(238, 232, 170)
}
pub fn pale_green() -> Color {
    rgb8(152, 251, 152)
}
pub fn pale_turquoise() -> Color {
    rgb8(175, 238, 238)
}
pub fn pale_violet_red() -> Color {
    rgb8(219, 112, 147)
}
pub fn papaya_whip() -> Color {
    rgb8(255, 239, 213)
}
pub fn peach_puff() -> Color {
    rgb8(255, 218, 185)
}
pub fn peru() -> Color {
    rgb8(205, 133, 63)
}
pub fn pink() -> Color {
    rgb8(255, 192, 203)
}
pub fn plum() -> Color {
    rgb8(221, 160, 221)
}
pub fn powder_blue() -> Color {
    rgb8(176, 224, 230)
}
pub fn purple() -> Color {
    rgb8(128, 0, 128)
}
pub fn rebecca_purple() -> Color {
    rgb8(102, 51, 153)
}
pub fn red() -> Color {
    rgb8(255, 0, 0)
}
pub fn rosy_brown() -> Color {
    rgb8(188, 143, 143)
}
pub fn royal_blue() -> Color {
    rgb8(65, 105, 225)
}
pub fn saddle_brown() -> Color {
    rgb8(139, 69, 19)
}
pub fn salmon() -> Color {
    rgb8(250, 128, 114)
}
pub fn sandy_brown() -> Color {
    rgb8(244, 164, 96)
}
pub fn sea_green() -> Color {
    rgb8(46, 139, 87)
}
pub fn seashell() -> Color {
    rgb8(255, 245, 238)
}
pub fn sienna() -> Color {
    rgb8(160, 82, 45)
}
pub fn silver() -> Color {
    rgb8(192, 192, 192)
}
pub fn sky_blue() -> Color {
    rgb8(135, 206, 235)
}
pub fn slate_blue() -> Color {
    rgb8(106, 90, 205)
}
pub fn slate_gray() -> Color {
    rgb8(112, 128, 144)
}
pub fn slate_grey() -> Color {
    rgb8(112, 128, 144)
}
pub fn snow() -> Color {
    rgb8(255, 250, 250)
}
pub fn spring_green() -> Color {
    rgb8(0, 255, 127)
}
pub fn steel_blue() -> Color {
    rgb8(70, 130, 180)
}
pub fn tan() -> Color {
    rgb8(210, 180, 140)
}
pub fn teal() -> Color {
    rgb8(0, 128, 128)
}
pub fn thistle() -> Color {
    rgb8(216, 191, 216)
}
pub fn tomato() -> Color {
    rgb8(255, 99, 71)
}
pub fn turquoise() -> Color {
    rgb8(64, 224, 208)
}
pub fn violet() -> Color {
    rgb8(238, 130, 238)
}
pub fn wheat() -> Color {
    rgb8(245, 222, 179)
}
pub fn white() -> Color {
    rgb8(255, 255, 255)
}
pub fn white_smoke() -> Color {
    rgb8(245, 245, 245)
}
pub fn yellow() -> Color {
    rgb8(255, 255, 0)
}
pub fn yellow_green() -> Color {
    rgb8(154, 205, 50)
}
