use noise::*;
use wassily::prelude::*;
use wassily::raqote::Canvas;

const WIDTH: u32 = 19200;
const HEIGHT: u32 = 12000;
const OFFSET: f32 = 0.015;
const D: f32 = 0.6;
const GRID: f32 = 10.0;
const SCALE: f32 = 1.0;
const FACTOR: f32 = 1.5;

pub fn pmin<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn tint(color: RGBA, k: f32) -> RGBA {
    let rm = pmin(1.0, k * color.r);
    let gm = pmin(1.0, k * color.g);
    let bm = pmin(1.0, k * color.b);
    RGBA::new(rm, gm, bm, color.a)
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let source = BasicMulti::default()
        .set_frequency(4.0)
        .set_persistence(0.75);
    let turb = Turbulence::new(source).set_power(2.0);

    let mut ks = Noise::<_, 2>::new(WIDTH as f32, WIDTH as f32, turb);
    ks.set_noise_scales(SCALE, SCALE);
    ks.set_noise_factor(FACTOR);

    // 0, 48, 73
    let lr = 87;
    let lg = 16;
    let lb = 137;

    // 247, 127, 0
    let rr = 234;
    let rg = 105;
    let rb = 139;

    let color_left = RGBA::with_8(lr, lg, lb, 255);
    let color_left = tint(color_left, D);
    let color_right = RGBA::with_8(rr, rg, rb, 255);

    Canvas::fill_rect(
        &mut canvas,
        0.0,
        0.0,
        WIDTH as f32 / 2.0,
        HEIGHT as f32,
        &Texture::new(TextureKind::SolidColor(color_left))
    );
    Canvas::fill_rect(
        &mut canvas,
        WIDTH as f32 / 2.0,
        0.0,
        WIDTH as f32 / 2.0,
        HEIGHT as f32,
        &Texture::new(TextureKind::SolidColor(color_right))
    );

    for x in (0..WIDTH).step_by(GRID as usize) {
        for y in (0..HEIGHT).step_by(GRID as usize) {
            let mut n = ks.noise(x as f32, y as f32);
            n = n.abs().clamp(0.0, 1.0);
            let mut c = RGBA::new(1.0, 1.0, 1.0, 1.0);
            c.a = n;
                let square = ShapeBuilder::new()
                    .rect_xywh(point2(x as f32, y as f32), point2(GRID, GRID))
                    .fill_color(c)
                    .no_stroke()
                    .build();
                square.draw(&mut canvas);
        }
    }

    Canvas::fill_rect(
        &mut canvas,
        WIDTH as f32 * (0.50 - OFFSET),
        0.0,
        2.0 * WIDTH as f32 * OFFSET,
        HEIGHT as f32,
        &Texture::new(TextureKind::SolidColor(color_left))
    );

    canvas.save("half.png")
}
