use wassily::prelude::*;

fn main() {
    let mut texture = Canvas::new(1080, 1080);
    checkered(&mut texture);
    let mut canvas = Canvas::new(texture.height(), texture.height());
    canvas.fill(*BLACK);

    let r = canvas.height() as f32;
    let mut sphere = SphereScene::basic(Point3::new(0.0, 0.0, 1.3 * r), &texture);
    sphere.rotation_y = PI / 16.0;

    let ambient = Light::ambient(0.0);
    let point = Light::point(0.9, -r, 2.5 * r, -2.5 * r);
    let directional = Light::directional(0.1, -0.5, -1.0, -1.0);
    sphere.lights = vec![ambient, point, directional];
    sphere.specular = Some(100.0);

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
            ShapeBuilder::new()
                .rect_xywh(pt(x, y), pt(swatch_width, swatch_width))
                .fill_paint(&paint)
                .build()
                .draw(texture);
        }
    }
}
