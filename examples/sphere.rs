use wassily::prelude::*;

fn main() {
    let mut texture = Canvas::new(1080, 1080);
    checkered(&mut texture);
    let mut canvas = Canvas::new(texture.height(), texture.height());
    canvas.fill((*BLACK).lerp(&RED, 0.0125));
    let r = canvas.height() as f32 / 2.0;
    let mut sphere = SphereScene::new(Point3::new(0.0, 0.0, 2.75 * r), r, &texture);
    sphere.rotation_z = -PI / 5.0;
    sphere.rotation_y = PI / 8.0;
    sphere.specular = Some(75.0);
    let w = canvas.width() as f32;
    let ambient = Light::new(LightSource::Ambient, 0.0, Point3::new(0.0, 0.0, 0.0));
    let direct = Light::new(
        LightSource::Point,
        0.5,
        Point3::new(-w, 2.5 * w, -2.5 * w),
    );
    let directional = Light::new(LightSource::Directional, 0.5, Point3::new(-0.5, -1.0, -1.0));
    sphere.lights = vec![ambient, direct, directional];
    sphere.on_sphere(&mut canvas);
    canvas.save_png("./sphere.png");
}

fn checkered(texture: &mut Canvas) {
    let width = texture.width();
    let mut palette = Palette::steal("./fruit.png", 90);
    palette.rotate_hue(180.0);
    let num_colors = palette.len() as f32;
    let n = (0.5 + num_colors.sqrt()) as usize;
    let swatch_width = width as f32 / n as f32;
    for i in 0..n {
        for j in 0..n {
            if palette.colors.len() <= (i * n + j) {
                break;
            }
            let c = palette[(i * n + j)];
            let paint = paint_solid(c);
            let x = j as f32 * swatch_width;
            let y = i as f32 * swatch_width;
            texture.fill_rect(x, y, swatch_width, swatch_width, &paint);
        }
    }
}