use noise::*;
use wassily::prelude::*;
use wassily::skia::Canvas;

fn main() {
    let mut canvas = Canvas::load_png("coin.png");
    let width = 2612.0;
    let height = 2612.0;
    let scale = 15.0;
    let rings = 19;
    let center = point2(width * 0.5, height * 0.5);

    let ns = Noise::<Fbm<OpenSimplex>, 3>::new(width, height, Fbm::new(0))
        .set_seed(1)
        .set_noise_scales(scale, scale, 1.0)
        .set_noise_factor(4.0);
    let mut rando = Rand::new(0);
    let mut radius = 6.0;
    let ring_color = RGBA::rgb8(249, 73, 35);

    for i in 0..rings {
        let ms = ns
            .clone()
            .set_noise_scales(scale - i as f32 / 2.0, scale - i as f32 / 2.0, 1.0);
        let cps = circle_points(center, radius, &ms, 30 + 4 * i);
        let mut w = rando.rand_range(6.0, 10.0);
        if i == rings - 1 {
            w = 30.0
        }
        ShapeBuilder::new()
            .points(&cps)
            .fill_color(RGBA::new(0.0, 0.0, 0.0, 0.0))
            .stroke_color(ring_color)
            .stroke_weight(w)
            .build()
            .draw(&mut canvas);
        let m = rando.rand_range(50.0, 90.0);
        radius += m - 1.7 * i as f32;
    }

    let crack_ps = crack_points(center, 120.0, &ns, rings, TAU / 16.0, 0.07);
    ShapeBuilder::new()
        .points(&crack_ps)
        .no_stroke()
        .fill_color(ring_color)
        .build()
        .draw(&mut canvas);

    let crack_ps = crack_points(center, 80.0, &ns, rings, PI + 0.1, 0.12);
    ShapeBuilder::new()
        .points(&crack_ps)
        .no_stroke()
        .fill_color(ring_color)
        .build()
        .draw(&mut canvas);

    let crack_ps = crack_points(center, 200.0, &ns, rings, PI + 1.0, 0.05);
    ShapeBuilder::new()
        .points(&crack_ps)
        .no_stroke()
        .fill_color(ring_color)
        .build()
        .draw(&mut canvas);

    ShapeBuilder::new()
        .circle(center, 1400.0)
        .stroke_weight(880.0)
        .stroke_color(RGBA::with_8(0, 0, 0, 220))
        .no_fill()
        .build()
        .draw(&mut canvas);

    canvas.save("elmflip.png");
}

fn circle_points<T>(center: Point, radius: f32, ns: &Noise<T, 3>, n: u32) -> Vec<Point>
where
    T: NoiseFn<3>,
{
    let mut ps = vec![];
    let delta = TAU / n as f32;
    let mut theta = 0.0;
    while theta < TAU {
        let mut x = center.x + radius * theta.cos();
        let mut y = center.y + radius * theta.sin();
        let mut dr = ns.noise(x, y, radius);
        dr = map_range(dr, -2.0, 2.0, 20.0, 60.0);
        x = center.x + (radius + dr) * theta.cos();
        y = center.y + (radius + dr) * theta.sin();
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
    T: NoiseFn<3>,
{
    let delta = radius / n as f32;
    let mut r = radius;
    let mut out = vec![];
    let mut back = vec![];
    while r < 1500.0 {
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
