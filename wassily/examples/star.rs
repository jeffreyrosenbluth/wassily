use wassily::prelude::*;

fn main() {
    let mut canvas = Canvas::new(500, 500);
    canvas.fill(*CORNFLOWERBLUE);
    let pos = center(500, 500);
    let star = Shape::new()
        .star(pos, 100.0, 175.0, 8)
        .fill_color(*GREENYELLOW)
        .stroke_color(*MIDNIGHTBLUE)
        .stroke_weight(3.0);
    star.draw(&mut canvas);
    canvas.save_png("./outputs/star.png");
}
