use noise::*;
use wassily::prelude::*;
use wassily::skia::Canvas;

fn main() {
    let mut canvas = Canvas::load_png("quarter.png");
    let width = 2612.0;
    let height = 2612.0;
    let scale = 4.0;
    let rings = 21;
    let center = point2(width * 0.5, height * 0.5);

    let ns = Noise::<Fbm, 3>::new(width, height, Fbm::new(0))
        .set_seed(1)
        .set_xy_scales(scale)
        .set_noise_factor(16.0);
    let mut rando = Rand::new(1);
    let radius0 = 25.0;
    let ring_color = RGBA::rgb8(249, 73, 35);
    let texture = Texture::solid_color(ring_color).mode(BlendMode::ColorBurn);
    let mut radius = radius0;
    let points = circle_points(radius0, &ns, 360);

    for _ in 0..rings {
        let w = 11.0;
        let m = rando.rand_range(60.0, 70.0);
        radius += m;
        let cps: Vec<Point> = points
            .iter()
            .map(|p| point2(radius * p.x / radius0, radius * p.y / radius0))
            .collect();
        ShapeBuilder::new()
            .points(&cps)
            .fill_color(RGBA::rgba(0.0, 0.0, 0.0, 0.0))
            .stroke_texture(&texture)
            .stroke_weight(w)
            .cartesian(width, height)
            .build()
            .draw(&mut canvas);
    }

    let crack_ps = crack_points(center, 120.0, &ns, rings, TAU / 16.0, 0.07);
    ShapeBuilder::new()
        .points(&crack_ps)
        .no_stroke()
        .fill_texture(&texture)
        .build()
        .draw(&mut canvas);

    let crack_ps = crack_points(center, 100.0, &ns, rings, PI + 0.1, 0.12);
    ShapeBuilder::new()
        .points(&crack_ps)
        .no_stroke()
        .fill_texture(&texture)
        .build()
        .draw(&mut canvas);

    let crack_ps = crack_points(center, 200.0, &ns, rings, PI + 1.0, 0.05);
    ShapeBuilder::new()
        .points(&crack_ps)
        .no_stroke()
        .fill_texture(&texture)
        .build()
        .draw(&mut canvas);

    let dots = stipple(width, height, 100_000);
    let pix_size = 64.0;
    let mut palette = Palette::with_img("sea.png", 1000);
    palette.rotate_hue(300.0);
    for d in dots.clone() {
        let d2 = (d.x - center.x).powi(2) + (d.y - center.y).powi(2);
        if d2 > 1280f32.powi(2) {
            let color = palette.rand_color();
            canvas.fill_rect(
                d.x - pix_size / 2.0,
                d.y - pix_size / 2.0,
                pix_size,
                pix_size,
                &Texture::solid_color(color.set_opacity(0.1)),
            );
        }
    }

    canvas.save("elmflip.png");
}

fn circle_points<T>(radius: f32, ns: &Noise<T, 3>, n: u32) -> Vec<Point>
where
    T: NoiseFn<f64, 3>,
{
    let mut ps = vec![];
    let delta = TAU / n as f32;
    let mut theta = 0.0;
    while theta < TAU {
        let mut x = radius * theta.cos();
        let mut y = radius * theta.sin();
        let mut dr = ns.noise(x, y, radius);
        dr = map_range(dr, -1.0, 1.0, 0.0, 5.0);
        x = (radius + dr) * theta.cos();
        y = (radius + dr) * theta.sin();
        ps.push(point2(x, y));
        theta += delta;
    }
    ps
}

fn crack_points<T>(
    center: Point,
    radius: f32,
    ns: &Noise<T, 3>,
    n: u32,
    theta: f32,
    dtheta: f32,
) -> Vec<Point>
where
    T: NoiseFn<f64, 3>,
{
    let delta = radius / n as f32;
    let mut r = radius;
    let mut out = vec![];
    let mut back = vec![];
    while r < 1320.0 {
        let x = center.x + r * theta.cos();
        let y = center.y + r * theta.sin();
        let dx = 5.0 * ns.noise(x, y, 0.0);
        let dy = 5.0 * ns.noise(x, y, 1.1);
        out.push(point2(x + dx, y + dy));
        let x = center.x + r * (theta + dtheta).cos();
        let y = center.y + r * (theta + dtheta).sin();
        let dx = 5.0 * ns.noise(x, y, 0.0);
        let dy = 5.0 * ns.noise(x, y, 1.1);
        back.push(point2(x + dx, y + dy));
        r += delta;
    }
    back.reverse();
    out.extend(&back);
    out
}
