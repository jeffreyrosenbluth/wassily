// The conversion routines in this file is based on code by Björn Ottosson.
// Copyright(c) 2021 Björn Ottosson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files(the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and /or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::kolor::{Colorful, ConvertColor};
use tiny_skia::Color;

/// all values in [0, 1]
pub struct Oklab {
    l: f32,
    a: f32,
    b: f32,
}

impl ConvertColor for Oklab {
    fn to_color(&self) -> Color {
        let (r, g, b) = oklab_to_linear_srgb(self.l, self.a, self.b);
        Color::from_rgba(
            srgb_transfer_function(r),
            srgb_transfer_function(g),
            srgb_transfer_function(b),
            1.0,
        )
        .unwrap()
    }
    fn from_color(color: &Color) -> Self {
        let (mut r, mut g, mut b, _) = color.as_f32s();
        r = srgb_transfer_function_inv(r);
        g = srgb_transfer_function_inv(g);
        b = srgb_transfer_function_inv(b);
        let o = linear_srgb_to_oklab(r, g, b);
        Oklab {
            l: o.0,
            a: o.1,
            b: o.2,
        }
    }
}

/// all values in [0, 1]
pub struct Okhsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl ConvertColor for Okhsl {
    fn to_color(&self) -> Color {
        let (r, g, b) = okhsl_to_srgb(self.h, self.s, self.l);
        Color::from_rgba(r, g, b, 1.0).unwrap()
    }

    fn from_color(color: &Color) -> Self {
        let (r, g, b, _) = color.as_f32s();
        let o = srgb_to_okhsl(r, g, b);
        Okhsl {
            h: o.0,
            s: o.1,
            l: o.2,
        }
    }
}

pub struct Okhsv {
    h: f32,
    s: f32,
    v: f32,
}

impl ConvertColor for Okhsv {
    fn to_color(&self) -> Color {
        let (r, g, b) = okhsv_to_srgb(self.h, self.s, self.v);
        Color::from_rgba(r, g, b, 1.0).unwrap()
    }

    fn from_color(color: &Color) -> Self {
        let (r, g, b, _) = color.as_f32s();
        let o = srgb_to_okhsv(r, g, b);
        Okhsv {
            h: o.0,
            s: o.1,
            v: o.2,
        }
    }
}

pub fn srgb_transfer_function(a: f32) -> f32 {
    let a = a.clamp(0.0, 1.0);
    if 0.0031308 >= a {
        12.92 * a
    } else {
        1.055 * a.powf(0.4166666666666667) - 0.055
    }
}

pub fn srgb_transfer_function_inv(a: f32) -> f32 {
    let a = a.clamp(0.0, 1.0);
    if a >= 0.04045 {
        ((a + 0.055) / (1.0 + 0.055)).powf(2.4)
    } else {
        a / 12.92
    }
}

fn compute_max_saturation(a: f32, b: f32) -> f32 {
    let (k0, k1, k2, k3, k4, wl, wm, ws) = if -1.88170328 * a - 0.80936493 * b > 1.0 {
        (
            1.19086277,
            1.76576728,
            0.59662641,
            0.75515197,
            0.56771245,
            4.0767416621,
            -3.3077115913,
            0.2309699292,
        )
    } else if 1.81444104 * a - 1.19445276 * b > 1.0 {
        (
            0.73956515,
            -0.45954404,
            0.08285427,
            0.12541070,
            0.14503204,
            -1.2684380046,
            2.6097574011,
            -0.3413193965,
        )
    } else {
        (
            1.35733652,
            -0.00915799,
            -1.15130210,
            -0.50559606,
            0.00692167,
            -0.0041960863,
            -0.7034186147,
            1.7076147010,
        )
    };

    let s = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    let k_l = 0.3963377774 * a + 0.2158037573 * b;
    let k_m = -0.1055613458 * a - 0.0638541728 * b;
    let k_s = -0.0894841775 * a - 1.2914855480 * b;

    let l_ = 1.0 + s * k_l;
    let m_ = 1.0 + s * k_m;
    let s_ = 1.0 + s * k_s;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let l_ds = 3.0 * k_l * l_ * l_;
    let m_ds = 3.0 * k_m * m_ * m_;
    let s_ds = 3.0 * k_s * s_ * s_;

    let l_ds2 = 6.0 * k_l * k_l * l_;
    let m_ds2 = 6.0 * k_m * k_m * m_;
    let s_ds2 = 6.0 * k_s * k_s * s_;

    let f = wl * l + wm * m + ws * s;
    let f1 = wl * l_ds + wm * m_ds + ws * s_ds;
    let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

    s - f * f1 / (f1 * f1 - 0.5 * f * f2)
}

fn oklab_to_linear_srgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    (
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    )
}

fn linear_srgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    let l_ = l.powf(1.0 / 3.0);
    let m_ = m.powf(1.0 / 3.0);
    let s_ = s.powf(1.0 / 3.0);

    (
        0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
    )
}

// returns (L, C)
fn find_cusp(a: f32, b: f32) -> (f32, f32) {
    let s_cusp = compute_max_saturation(a, b);

    let (r, g, b) = oklab_to_linear_srgb(1.0, s_cusp * a, s_cusp * b);
    let l_cusp = (1.0 / r.max(g).max(b)).powf(1.0 / 3.0);
    let c_cusp = l_cusp * s_cusp;

    (l_cusp, c_cusp)
}

fn find_gamut_intersection(
    a: f32,
    b: f32,
    l1: f32,
    c1: f32,
    l0: f32,
    cusp_l: f32,
    cusp_c: f32,
) -> f32 {
    if ((l1 - l0) * cusp_c - (cusp_l - l0) * c1) <= 0.0 {
        cusp_c * l0 / (c1 * cusp_l + cusp_c * (l0 - l1))
    } else {
        let t = cusp_c * (l0 - 1.0) / (c1 * (cusp_l - 1.0) + cusp_c * (l0 - l1));
        {
            let dl = l1 - l0;
            let dc = c1;

            let k_l = 0.3963377774 * a + 0.2158037573 * b;
            let k_m = -0.1055613458 * a - 0.0638541728 * b;
            let k_s = -0.0894841775 * a - 1.2914855480 * b;

            let l_dt = dl + dc * k_l;
            let m_dt = dl + dc * k_m;
            let s_dt = dl + dc * k_s;

            {
                let l = l0 * (1.0 - t) + t * l1;
                let c = t * c1;

                let l_ = l + c * k_l;
                let m_ = l + c * k_m;
                let s_ = l + c * k_s;

                let l = l_ * l_ * l_;
                let m = m_ * m_ * m_;
                let s = s_ * s_ * s_;

                let ldt = 3.0 * l_dt * l_ * l_;
                let mdt = 3.0 * m_dt * m_ * m_;
                let sdt = 3.0 * s_dt * s_ * s_;

                let ldt2 = 6.0 * l_dt * l_dt * l_;
                let mdt2 = 6.0 * m_dt * m_dt * m_;
                let sdt2 = 6.0 * s_dt * s_dt * s_;

                let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s - 1.0;
                let r1 = 4.0767416621 * ldt - 3.3077115913 * mdt + 0.2309699292 * sdt;
                let r2 = 4.0767416621 * ldt2 - 3.3077115913 * mdt2 + 0.2309699292 * sdt2;

                let u_r = r1 / (r1 * r1 - 0.5 * r * r2);
                let t_r = -r * u_r;

                let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s - 1.0;
                let g1 = -1.2684380046 * ldt + 2.6097574011 * mdt - 0.3413193965 * sdt;
                let g2 = -1.2684380046 * ldt2 + 2.6097574011 * mdt2 - 0.3413193965 * sdt2;

                let u_g = g1 / (g1 * g1 - 0.5 * g * g2);
                let t_g = -g * u_g;

                let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s - 1.0;
                let b1 = -0.0041960863 * ldt - 0.7034186147 * mdt + 1.7076147010 * sdt;
                let b2 = -0.0041960863 * ldt2 - 0.7034186147 * mdt2 + 1.7076147010 * sdt2;

                let u_b = b1 / (b1 * b1 - 0.5 * b * b2);
                let t_b = -b * u_b;

                let t_r = if u_r >= 0.0 { t_r } else { std::f32::INFINITY };
                let t_g = if u_g >= 0.0 { t_g } else { std::f32::INFINITY };
                let t_b = if u_b >= 0.0 { t_b } else { std::f32::INFINITY };
                t + t_r.min(t_g.min(t_b))
            }
        }
    }
}

// Returns (S, T)
fn to_st(cusp_l: f32, cusp_c: f32) -> (f32, f32) {
    (cusp_c / cusp_l, cusp_c / (1.0 - cusp_l))
}

// Returns (S, T)
fn get_st_mid(a_: f32, b_: f32) -> (f32, f32) {
    let s = 0.11516993
        + 1.0
            / (7.44778970
                + 4.15901240 * b_
                + a_ * (-2.19557347
                    + 1.75198401 * b_
                    + a_ * (-2.13704948 - 10.02301043 * b_
                        + a_ * (-4.24894561 + 5.38770819 * b_ + 4.69891013 * a_))));

    let t = 0.11239642
        + 1.0
            / (1.61320320 - 0.68124379 * b_
                + a_ * (0.40370612
                    + 0.90148123 * b_
                    + a_ * (-0.27087943
                        + 0.61223990 * b_
                        + a_ * (0.00299215 - 0.45399568 * b_ - 0.14661872 * a_))));

    (s, t)
}

fn get_cs(l: f32, a_: f32, b_: f32) -> (f32, f32, f32) {
    let (cusp_l, cusp_c) = find_cusp(a_, b_);

    let c_max = find_gamut_intersection(a_, b_, l, 1.0, l, cusp_l, cusp_c);
    let (st_max_s, st_max_t) = to_st(cusp_l, cusp_c);

    let k = c_max / (l * st_max_s).min((1.0 - l) * st_max_t);

    let c_mid = {
        let (st_mid_s, st_mid_t) = get_st_mid(a_, b_);

        let c_a = l * st_mid_s;
        let c_b = (1.0 - l) * st_mid_t;
        0.9 * k
            * (1.0 / (1.0 / (c_a * c_a * c_a * c_a) + 1.0 / (c_b * c_b * c_b * c_b)))
                .sqrt()
                .sqrt()
    };

    let c_0 = {
        let c_a = l * 0.4;
        let c_b = (1.0 - l) * 0.8;

        (1.0 / (1.0 / (c_a * c_a) + 1.0 / (c_b * c_b))).sqrt()
    };

    (c_0, c_mid, c_max)
}

fn toe(x: f32) -> f32 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    0.5 * (k_3 * x - k_1 + ((k_3 * x - k_1) * (k_3 * x - k_1) + 4.0 * k_2 * k_3 * x)).sqrt()
}

fn toe_inv(x: f32) -> f32 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    (x * x + k_1 * x) / (k_3 * (x + k_2))
}

pub fn okhsl_to_srgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if l >= 1.0 {
        return (1.0, 1.0, 1.0);
    }
    if l <= 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let a_ = (2.0 * std::f32::consts::PI * h).cos();
    let b_ = (2.0 * std::f32::consts::PI * h).sin();

    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    let l = (l * l + k_1 * l) / (k_3 * (l + k_2));

    let (c_0, c_mid, c_max) = get_cs(l, a_, b_);

    let mid = 0.8;
    let mid_inv = 1.25;

    let c = {
        if s < mid {
            let t = mid_inv * s;
            let k_1 = mid * c_0;
            let k_2 = 1.0 - k_1 / c_mid;
            t * k_1 / (1.0 - k_2 * t)
        } else {
            let t = (s - mid) / (1.0 - mid);
            let k_1 = (1.0 - mid) * c_mid * c_mid * mid_inv * mid_inv / c_0;
            let k_2 = 1.0 - k_1 / (c_max - c_mid);
            c_mid + t * k_1 / (1.0 - k_2 * t)
        }
    };

    let (r, g, b) = oklab_to_linear_srgb(l, c * a_, c * b_);
    (
        srgb_transfer_function(r),
        srgb_transfer_function(g),
        srgb_transfer_function(b),
    )
}

pub fn srgb_to_okhsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let lab = linear_srgb_to_oklab(
        srgb_transfer_function_inv(r),
        srgb_transfer_function_inv(g),
        srgb_transfer_function_inv(b),
    );

    let c = (lab.1 * lab.1 + lab.2 * lab.2).sqrt();
    let a_ = lab.1 / c;
    let b_ = lab.2 / c;

    let l = lab.0;
    let h = 0.5 + 0.5 * (-lab.2).atan2(-lab.1) / std::f32::consts::PI;

    let cs = get_cs(l, a_, b_);
    let c_0 = cs.0;
    let c_mid = cs.1;
    let c_max = cs.2;

    // Inverse of the interpolation in okhsl_to_srgb:

    let mid = 0.8;
    let mid_inv = 1.25;

    let s: f32;
    if c < c_mid {
        let k_1 = mid * c_0;
        let k_2 = 1.0 - k_1 / c_mid;

        let t = c / (k_1 + k_2 * c);
        s = t * mid;
    } else {
        let k_0 = c_mid;
        let k_1 = (1.0 - mid) * c_mid * c_mid * mid_inv * mid_inv / c_0;
        let k_2 = 1.0 - (k_1) / (c_max - c_mid);

        let t = (c - k_0) / (k_1 + k_2 * (c - k_0));
        s = mid + (1.0 - mid) * t;
    }

    let l = toe(l);
    (h, s, l)
}

pub fn okhsv_to_srgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let a_ = (2.0 * std::f32::consts::PI * h).cos();
    let b_ = (2.0 * std::f32::consts::PI * h).sin();

    let (cusp_l, cusp_c) = find_cusp(a_, b_);
    let (s_max, t_max) = to_st(cusp_l, cusp_c);
    let s_0 = 0.5;
    let k = 1.0 - s_0 / s_max;

    let l_v = 1.0 - s * s_0 / (s_0 + t_max - t_max * k * s);
    let c_v = s * t_max * s_0 / (s_0 + t_max - t_max * k * s);

    let l = v * l_v;
    let c = v * c_v;

    let l_vt = toe_inv(l_v);
    let c_vt = c_v * l_vt / l_v;

    let l_new = toe_inv(l);
    let c = c * l_new / l;
    let l = l_new;

    let (r_scale, g_scale, b_scale) = oklab_to_linear_srgb(l_vt, a_ * c_vt, b_ * c_vt);
    let scale_l = (1.0 / r_scale.max(g_scale).max(b_scale).max(0.0)).powf(1.0 / 3.0);

    let l = l * scale_l;
    let c = c * scale_l;

    let (r, g, b) = oklab_to_linear_srgb(l, c * a_, c * b_);
    (
        srgb_transfer_function(r),
        srgb_transfer_function(g),
        srgb_transfer_function(b),
    )
}

pub fn srgb_to_okhsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let lab = linear_srgb_to_oklab(
        srgb_transfer_function_inv(r),
        srgb_transfer_function_inv(g),
        srgb_transfer_function_inv(b),
    );

    let c = (lab.1 * lab.1 + lab.2 * lab.2).sqrt();
    let a_ = lab.1 / c;
    let b_ = lab.2 / c;

    let mut l = lab.0;
    let h = 0.5 + 0.5 * (-lab.2).atan2(-lab.1) / std::f32::consts::PI;

    let cusp = find_cusp(a_, b_);
    let st_max = to_st(cusp.0, cusp.1);
    let s_max = st_max.0;
    let t_max = st_max.1;
    let s_0 = 0.5;
    let k = 1.0 - s_0 / s_max;

    // first we find L_v, C_v, L_vt and C_vt

    let t = t_max / (c + l * t_max);
    let l_v = t * l;
    let c_v = t * c;

    let l_vt = toe_inv(l_v);
    let c_vt = c_v * l_vt / l_v;

    // we can then use these to invert the step that compensates for the toe and the curved top part of the triangle:
    let rgb_scale = oklab_to_linear_srgb(l_vt, a_ * c_vt, b_ * c_vt);
    let scale_l = (1.0 / (rgb_scale.0.max(rgb_scale.1).max(rgb_scale.2.max(0.0)))).powf(1.0 / 3.0);
    l = l / scale_l;
    l = toe(l);

    // we can now compute v and s:
    let v = l / l_v;
    let s = (s_0 + t_max) * c_v / ((t_max * s_0) + t_max * k * c_v);
    (h, s, v)
}
