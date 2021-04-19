use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;
const ORDER: u32 = 6;

fn main() {
    let mut canvas = Pixmap::new(WIDTH, HEIGHT).unwrap();
    // let mut pixmap = Pixmap::load_png("soup2.png").expect("Can't load png");
    canvas.fill(Color::from_rgba(0.0, 0.0, 0.0, 1.0).unwrap());

    let width = WIDTH as f32;
    let n = 2u32.pow(ORDER);

    let total = n * n;
    let mut path = vec![];

    for i in 0..total {
        let j = i as usize;
        path.push(hilbert(i, ORDER));
        let m = width / n as f32;
        path[j] = pt2(m * path[j].x, m * path[j].y);
        path[j] += pt2(m / 2.0, m / 2.0);
    }
    path = path.into_iter().collect();

    let color = Color::WHITE;

    let shape = ShapeBuilder::new()
        // .stroke_color(color)
        // .stroke_weight(1.5)
        .no_stroke()
        .fill_color(color)
        .quad()
        .points(&path)
        .build();
    shape.draw(&mut canvas);
    canvas.save_png("hilbert.png").unwrap();
}

fn hilbert(k: u32, order: u32) -> Point {
    let points = vec![pt2(0.0, 0.0), pt2(0.0, 1.0), pt2(1.0, 1.0), pt2(1.0, 0.0)];
    let idx = k as usize & 3;
    let mut v = points[idx];
    let mut i = k;

    for j in 1..order {
        i >>= 2;
        let index = i & 3;
        let n = 2u32.pow(j) as f32;
        match index {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
            }
            1 => {
                v.y += n;
            }
            2 => {
                v.x += n;
                v.y += n;
            }
            3 => {
                let temp = n - 1.0 - v.x;
                v.x = n - 1.0 - v.y;
                v.y = temp;
                v.x += n;
            }
            _ => {}
        }
    }
    v
}
