use noise::Perlin;
use std::collections::HashSet;
use wassily::prelude::*;
use wassily::skia::Canvas;
use rand::seq::IteratorRandom;

const WIDTH: u32 = 8000;
const HEIGHT: u32 = 6400;
const SIZE: f32 = 200.0;
const SCALE: f32 = 1.0;

pub struct SandBox {
    xy: Point,
    wh: Point,
    bg_color: RGBA,
    color1: RGBA,
    color2: RGBA,
}

impl SandBox {
    pub fn new(xy: Point, wh: Point) -> Self {
        Self {
            xy,
            wh,
            bg_color: WHITE,
            color1: BLACK,
            color2: BLACK,
        }
    }

    pub fn set_bg(mut self, color: RGBA) -> Self {
        self.bg_color = color;
        self
    }

    pub fn set_color1(mut self, color: RGBA) -> Self {
        self.color1 = color;
        self
    }

    pub fn set_color2(mut self, color: RGBA) -> Self {
        self.color2 = color;
        self
    }

    pub fn draw<T: Sketch>(&mut self, canvas: &mut T) {
        let ns = Noise::<_, 2>::new(self.wh.x, self.wh.y, Perlin::default())
            .scales(SCALE);
        canvas.fill_rect(
            self.xy.x,
            self.xy.y,
            self.wh.x,
            self.wh.y,
            &Texture::solid_color(self.bg_color),
        );
        for i in 0..self.wh.x as u32 {
            let from = point2(self.xy.x + i as f32, self.xy.y);
            let to = point2(self.xy.x + i as f32, self.xy.y + self.wh.y);
            let alpha = map_range(ns.get(from.x, from.y), -1.0, 1.0, 0.0, 1.0);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color1.set_opacity(alpha))
                .build()
                .draw(canvas);
        }
        for i in 0..self.wh.y as u32 {
            let from = point2(self.xy.x, self.xy.y + i as f32);
            let to = point2(self.xy.x + self.wh.x, self.xy.y + i as f32);
            let alpha = map_range(ns.get(from.x, from.y), -1.0, 1.0, 0.0, 1.0);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color2.set_opacity(alpha))
                .build()
                .draw(canvas);
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Domino {
    x: i32,
    y: i32,
    orientation: Orientation,
}

impl Domino {
    fn new(x: i32, y: i32, orientation: Orientation) -> Self {
        Self { x, y, orientation }
    }
}

fn squares(dominos: &HashSet<Domino>) -> HashSet<(Domino, Domino)> {
    let mut sq = HashSet::new();
    for d1 in dominos.iter() {
        for d2 in dominos.iter() {
            if d1.orientation == Orientation::Horizontal {
                if (d2.orientation == Orientation::Horizontal) && d1.x == d2.x {
                    if d2.y == d1.y - 1 {
                        sq.insert((d2.clone(), d1.clone()));
                    } else if d2.y == d1.y + 1 {
                        sq.insert((d1.clone(), d2.clone()));
                    }
                }
            }
            if d1.orientation == Orientation::Vertical {
                if d2.orientation == Orientation::Vertical && d1.y == d2.y{
                    if d2.x == d1.x - 1 {
                        sq.insert((d2.clone(), d1.clone()));
                    } else if d2.x == d1.x + 1 {
                        sq.insert((d1.clone(), d2.clone()));
                    }
                }
            }
        }
    }
    sq
}

fn flip(dominos: &mut HashSet<Domino>, square: &(Domino, Domino)) {
    dominos.remove(&square.0);
    dominos.remove(&square.1);
    let o = &square.0.orientation;
    match o {
        Orientation::Horizontal => {
            dominos.insert(Domino::new(square.0.x, square.0.y, Orientation::Vertical));
            dominos.insert(Domino::new(
                square.0.x + 1,
                square.0.y,
                Orientation::Vertical,
            ));
        }
        Orientation::Vertical => {
            dominos.insert(Domino::new(square.0.x, square.0.y, Orientation::Horizontal));
            dominos.insert(Domino::new(
                square.0.x,
                square.0.y + 1,
                Orientation::Horizontal,
            ));
        }
    }
}

fn mk_dominos(x: u32, y: u32) -> HashSet<Domino> {
    let mut dominos = HashSet::new();
    for r in 0..x {
        for c in 0..y {
            dominos.insert(Domino::new(2 * r as i32, c as i32, Orientation::Horizontal));
        }
    }
    dominos
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut palette = Palette::with_img("fruit.png", Some(2000));
    palette.sort_by_chroma();
    let mut palette1 = palette.clone();
    palette1.rotate_hue(0.0);
    let mut palette2 = palette.clone();
    palette2.rotate_hue(180.0);
    canvas.fill(RGBA::rgb8(150, 150, 150));
    canvas.fill(WHITE);
    let mut rng = rand::thread_rng();
    let mut dominos = mk_dominos(WIDTH / (2 * SIZE as u32), HEIGHT / SIZE as u32);
    for _ in 0..10_000 {
        let sq = squares(&dominos).into_iter().choose(&mut rng).unwrap();
        flip(&mut dominos, &sq);
    }
    for d in dominos {
        let p = point2(d.x as f32 * SIZE, d.y as f32 * SIZE);
        let mut sb = match d.orientation {
            Orientation::Horizontal => SandBox::new(p, point2(2.0 * SIZE, SIZE)),
            Orientation::Vertical => SandBox::new(p, point2(SIZE, 2.0 * SIZE)),
        };
        sb = sb.set_color1(palette1.rand_color());
        sb = sb.set_color2(palette2.rand_color());
        sb.draw(&mut canvas);
    }
    canvas.save("sandbox.png");
}