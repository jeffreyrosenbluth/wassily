use wassily::prelude::*;

fn main() {
    let mut texture = Pixmap::new(1080, 1080).unwrap();
    checkered(&mut texture);
    let mut drawing = Drawing::new(texture.height(), texture.height(), 1.0);
    drawing.clear(*BLACK);

    let r = texture.height() as f32;
    let mut sphere = SphereScene::basic(Point3::new(0.0, 0.0, 1.3 * r), &texture);
    sphere.rotation_y = PI / 16.0;

    let ambient = Light::ambient(0.0);
    let point = Light::point(0.9, -r, 2.5 * r, -2.5 * r);
    let directional = Light::directional(0.1, -0.5, -1.0, -1.0);
    sphere.lights = vec![ambient, point, directional];
    sphere.specular = Some(100.0);

    sphere.on_sphere(&mut drawing);
    drawing.render();
    drawing.save_png("./sphere0.png");
}

fn checkered(texture: &mut Pixmap) {
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
            let mut paint = Paint::default();
            paint.set_color(c);
            let x = j as f32 * swatch_width;
            let y = i as f32 * swatch_width;
            let rect = Rect::from_xywh(x, y, swatch_width, swatch_width).unwrap();
            texture.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }
}
