use wassily::prelude::*;

fn main() {
    let img = image::open("./noise.png").unwrap();
    let img = img.into_rgba8();
    let texture: Canvas = img.into();
    let mut canvas = Canvas::new(texture.height(), texture.height());
    canvas.fill(*BLACK);
    let r = canvas.height() as f32 / 2.0;
    let mut sphere = SphereScene::new(Point3::new(0.0, 0.0, 2.5 * r), r, &texture);
    sphere.rotation_z = -PI / 7.0;
    sphere.rotation_y = PI / 7.0;
    sphere.specular = Some(100.0);
    let w = canvas.width() as f32;
    let ambient = Light::new(LightSource::Ambient, 0.0, Point3::new(0.0, 0.0, 0.0));
    let direct = Light::new(
        LightSource::Point,
        0.8,
        Point3::new(-w, 2.5 * w, -2.5 * w),
    );
    let directional = Light::new(LightSource::Directional, 0.1, Point3::new(2.0, 4.0, -1.0));
    sphere.lights = vec![ambient, direct, directional];
    sphere.on_sphere(&mut canvas);
    canvas.save_png("./sphere.png");
}