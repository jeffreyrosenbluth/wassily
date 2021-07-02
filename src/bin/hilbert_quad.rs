use wassily::prelude::*;
use wassily::svg::Canvas;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;
const ORDER: u32 = 6;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(RGBA::new(0.0, 0.0, 0.0, 1.0));

    let width = WIDTH as f32;
    let n = 2u32.pow(ORDER);

    let total = n * n;
    let mut path = vec![];

    for i in 0..total {
        let j = i as usize;
        path.push(hilbert(i, ORDER));
        let m = width / n as f32;
        path[j] = point2(m * path[j].x, m * path[j].y);
        path[j] = point2(path[j].x + m / 2.0, path[j].y + m / 2.0);
    }
    path = path.into_iter().collect();

    let color = WHITE;

    let shape = ShapeBuilder::new()
        .no_stroke()
        .fill_color(color)
        .quad()
        .points(&path)
        .build();
    shape.draw(&mut canvas);
    canvas.save("hilbert.svg");
}

fn hilbert(k: u32, order: u32) -> Point {
    let points = vec![
        point2(0.0, 0.0),
        point2(0.0, 1.0),
        point2(1.0, 1.0),
        point2(1.0, 0.0),
    ];
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
