use wassily::prelude::*;

fn main() {
    let mut texture = Canvas::new(1080, 1080);
    checkered(&mut texture);
    let mut canvas = Canvas::new(texture.height(), texture.height());
    canvas.fill(*BLACK);
    let r = canvas.height() as f32;
    let mut sphere = SphereScene::basic(Point3::new(0.0, 0.0, 1.3 * r), &texture);
    sphere.rotation_y = PI / 16.0;
    sphere.specular = Some(100.0);
    let w = canvas.width() as f32;
    let ambient = Light::ambient(0.0);
    let direct = Light::point(0.9, -w, 2.5 * w, -2.5 * w);
    let directional = Light::directional(0.1, -0.5, -1.0, -1.0);
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