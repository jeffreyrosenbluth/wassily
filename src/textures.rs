use crate::canvas::*;
use crate::kolor::*;
use crate::noises::*;
use crate::prelude::pt;
use crate::shape::*;
use crate::stipple::uniform;
use noise::core::worley::distance_functions::euclidean_squared;
use noise::core::worley::ReturnType;
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
    canvas.pixmap.fill(color1);
    while l < height as f32 {
        Shape::new()
            .line(pt(0.0, l), pt(width, l))
            .stroke_color(color2)
            .stroke_weight(4.0)
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

pub fn foam(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    color3: Color,
    scale: f32,
    seed: u32,
) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let nf = Worley::default()
        .set_distance_function(euclidean_squared)
        .set_return_type(ReturnType::Distance)
        .set_seed(seed);
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let a = noise2d_01(&nf, &opts, i as f32, j as f32);
            let mut c = color1.lerp(&color2, a);
            if a > 0.48 && a < 0.52 {
                c = color3
            }
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

pub fn marble(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    scale: f32,
    seed: u32,
) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let nf = Fbm::<Perlin>::default().set_seed(seed);
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let a = noise2d(&nf, &opts, i as f32, j as f32);
            let b =
                0.5 * (((j as f32 + a * 1000.0) * std::f32::consts::PI * 2.0 / 200.0).sin() + 1.0);
            let c = color1.lerp(&color2, b);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

// From noise-rs. https://github.com/Razaekel/noise-rs/blob/develop/examples/texturewood.rs
pub fn wood(width: u32, height: u32, color1: Color, color2: Color, scale: f32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let base_wood = Cylinders::new().set_frequency(16.0);

    // Basic Multifractal noise to use for the wood grain.
    let wood_grain_noise = BasicMulti::<Perlin>::new(0)
        .set_frequency(48.0)
        .set_persistence(0.5)
        .set_lacunarity(2.20703125)
        .set_octaves(3);

    // Stretch the perlin noise in the same direction as the center of the log. Should
    // produce a nice wood-grain texture.
    let scaled_base_wood_grain = ScalePoint::new(wood_grain_noise).set_z_scale(0.25);

    // Scale the wood-grain values so that they can be added to the base wood texture.
    let wood_grain = ScaleBias::new(scaled_base_wood_grain)
        .set_scale(0.25)
        .set_bias(0.125);

    // Add the wood grain texture to the base wood texture.
    let combined_wood = Add::new(base_wood, wood_grain);

    // Slightly perturb the wood to create a more realistic texture.
    let perturbed_wood = Turbulence::<_, Perlin>::new(combined_wood)
        .set_seed(1)
        .set_frequency(4.0)
        .set_power(1.0 / 256.0)
        .set_roughness(4);

    let nf = Turbulence::<_, Perlin>::new(perturbed_wood)
        .set_seed(2)
        .set_frequency(2.0)
        .set_power(1.0 / 64.0)
        .set_roughness(4);
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let b = noise2d_01(
                &nf,
                &opts,
                i as f32 - width as f32 / 2.0,
                j as f32 - height as f32 / 2.0,
            );
            let c = color1.lerp(&color2, b);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

// From noise-rs. https://github.com/Razaekel/noise-rs/blob/develop/examples/texturegranite.rs
pub fn granite(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    scale: f32,
    seed: u32,
) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    // Primary granite texture. This generates the "roughness" of the texture
    // when lit by a light source.
    let primary_granite = Billow::<Perlin>::new(0)
        .set_frequency(8.0)
        .set_persistence(0.625)
        .set_lacunarity(2.18359375)
        .set_octaves(6)
        .set_seed(seed);

    // Use Worley polygons to produce the small grains for the granite texture.
    let base_grains = Worley::new(1)
        .set_frequency(16.0)
        .set_return_type(ReturnType::Distance);

    // Scale the small grain values so that they can be added to the base
    // granite texture. Worley polygons normally generate pits, so apply a
    // negative scaling factor to produce bumps instead.
    let scaled_grains = ScaleBias::new(base_grains).set_scale(-0.5).set_bias(0.0);

    // Combine the primary granite texture with the small grain texture.
    let combined_granite = Add::new(primary_granite, scaled_grains);

    // Finally, perturb the granite texture to add realism.
    let nf = Turbulence::<_, Perlin>::new(combined_granite)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 8.0)
        .set_roughness(6);
    let opts = NoiseOpts::with_wh(width, height).scales(scale);
    for i in 0..width {
        for j in 0..height {
            let b = noise2d_01(&nf, &opts, i as f32, j as f32);
            let c = color1.lerp(&color2, b);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas
}

pub fn pattern<'a>(
    texture_canvas: &'a Canvas,
    pattern_canvas: &'a mut Canvas,
    bbox: Rect,
) -> Paint<'a> {
    let x = bbox.x();
    let y = bbox.y();
    pattern_canvas.pixmap.draw_pixmap(
        x as i32,
        y as i32,
        texture_canvas.pixmap.as_ref(),
        &PixmapPaint::default(),
        Transform::identity(),
        None,
    );
    let pattern = Pattern::new(
        pattern_canvas.pixmap.as_ref(),
        SpreadMode::Pad,
        FilterQuality::Bicubic,
        1.0,
        Transform::identity(),
    );
    paint_shader(pattern)
}

pub struct PatternPaint<'a> {
    pub texture_canvas: &'a Canvas,
    pub pattern_canvas: Canvas,
    pub bbox: Rect,
}

impl<'a> PatternPaint<'a> {
    pub fn new(texture_canvas: &'a Canvas, bbox: Rect, width: u32, height: u32) -> Self {
        let pattern_canvas = Canvas::new(width, height);
        Self {
            texture_canvas,
            pattern_canvas,
            bbox,
        }
    }

    pub fn paint(&'a mut self) -> Paint<'a> {
        let x = self.bbox.x();
        let y = self.bbox.y();
        self.pattern_canvas.pixmap.draw_pixmap(
            x as i32,
            y as i32,
            self.texture_canvas.pixmap.as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
        let pattern = Pattern::new(
            self.pattern_canvas.pixmap.as_ref(),
            SpreadMode::Pad,
            FilterQuality::Bicubic,
            1.0,
            Transform::identity(),
        );
        paint_shader(pattern)
    }
}
