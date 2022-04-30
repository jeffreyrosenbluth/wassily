use crate::canvas::*;
use crate::prelude::pt;
use crate::kolor::*;
use crate::noises::*;
use crate::rectangles::SandBox;
use crate::shape::*;
use crate::stipple::uniform;
use noise::*;
use tiny_skia::*;

pub fn stipple_texture(width: u32, height: u32, color: Color, spacing: f32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let n = canvas.w_f32() * canvas.h_f32() / spacing;
    let dots = uniform(width as f32, height as f32, n as u32, 0);
    for d in dots {
        canvas.dot(d.x, d.y, color);
    }
    canvas
}

pub fn horizontal_stripe(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    spacing: f32,
) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let mut l = 0.0;
    canvas.fill(color1);
    while l < height as f32 {
        ShapeBuilder::new()
            .line(pt(0.0, l), pt(width, l))
            .stroke_color(color2)
            .stroke_weight(4.0)
            .build()
            .draw(&mut canvas);
        l += spacing;
    }
    canvas
}

pub fn ridge(width: u32, height: u32, color1: Color, color2: Color, scale: f32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let nf = RidgedMulti::<Perlin>::default();
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let a = noise2d_01(&nf, &opts, i as f32, j as f32);
            let c = color1.lerp(&color2, a);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

pub fn marble(width: u32, height: u32, color1: Color, color2: Color, scale: f32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let nf = Fbm::<Perlin>::default();
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let a = noise2d(&nf, &opts, i as f32, j as f32);
            let b = 0.5 * (((j as f32 + a * 100.0) * std::f32::consts::PI * 2.0 / 200.0).sin() + 1.0);
            let c = color1.lerp(&color2, b);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

pub fn wood(width: u32, height: u32, color1: Color, color2: Color, scale: f32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let nf = Fbm::<Perlin>::default();
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let a = noise2d(&nf, &opts, i as f32, j as f32);
            let b = (10.0 * a).fract();
            let c = color1.lerp(&color2, b);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

pub fn sand(width: u32, height: u32, color1: Color, color2: Color, scale: f32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let mut sb = SandBox::new(pt(0u32, 0u32), pt(width, height), color1, color2, color2, scale);
    sb.draw(&mut canvas);
    canvas
}

pub fn pattern<'a>(
    texture_canvas: &'a Canvas,
    pattern_canvas: &'a mut Canvas,
    bbox: Rect,
) -> Paint<'a> {
    let x = bbox.x();
    let y = bbox.y();
    pattern_canvas.draw_pixmap(
        x as i32,
        y as i32,
        texture_canvas.as_ref(),
        &PixmapPaint::default(),
        Transform::identity(),
        None,
    );
    let pattern = Pattern::new(
        pattern_canvas.as_ref(),
        SpreadMode::Pad,
        FilterQuality::Bicubic,
        1.0,
        Transform::identity(),
    );
    paint_shader(pattern)
}