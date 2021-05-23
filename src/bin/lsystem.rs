use noise::*;
use std::collections::HashMap;
use wassily::prelude::*;
use wassily::skia::Canvas;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Sym {
    F,
    G,
    Plus,
    Minus,
    Push,
    Pop,
}

type Rules = HashMap<char, Vec<char>>;
type Cmds = HashMap<char, Sym>;

fn apply_rule(k: char, rules: &Rules) -> Vec<char> {
    match rules.get(&k) {
        Some(rule) => rule.to_vec(),
        None => vec![k],
    }
}

fn apply_rules(cs: Vec<char>, rules: &Rules) -> Vec<char> {
    cs.into_iter().flat_map(|k| apply_rule(k, rules)).collect()
}

fn iter_rules(cs: Vec<char>, rules: &Rules, n: u32) -> Vec<char> {
    let mut result = cs;
    for _ in 0..n {
        result = apply_rules(result, rules)
    }
    result
}

fn char_to_sym(c: char, cmds: &Cmds) -> Sym {
    if let Some(s) = cmds.get(&c) {
        *s
    } else {
        panic!("Character missing from commands")
    }
}

fn to_sym(cs: Vec<char>, cmds: &Cmds) -> Vec<Sym> {
    cs.into_iter().map(|c| char_to_sym(c, cmds)).collect()
}

struct Turtle {
    pb: PathBuilder,
    direction: f32,
    angle: f32,
    color: RGBA,
    length: f32,
    thickness: f32,
    pen: bool,
    stack: Vec<Sym>,
}

impl Turtle {
    fn new() -> Self {
        let mut pb = PathBuilder::new();
        pb.move_to(300.0, 900.0);
        let direction = 0.0;
        let angle = 2.0 * PI / 3.0;
        let color = RGBA::black();
        let length = 10.0;
        let thickness = 2.0;
        let pen = true;
        let stack = vec![];
        Self {
            pb,
            direction,
            angle,
            color,
            length,
            thickness,
            pen,
            stack,
        }
    }

    fn forward(&mut self, distance: f32) {
        let dx = distance * self.direction.cos();
        let dy = distance * self.direction.sin();
        if self.pen {
            self.pb.line_by(dx, dy);
        } else {
            self.pb.move_by(dx, dy);
        }
    }

    fn right(&mut self) {
        self.direction += self.angle;
    }

    fn left(&mut self) {
        self.direction -= self.angle;
    }

    fn interp(&mut self, sym: Sym) {
        match sym {
            Sym::F => {
                self.pen = true;
                self.forward(self.length)
            }
            Sym::G => {
                self.pen = false;
                self.forward(self.length)
            }
            Sym::Plus => self.right(),
            Sym::Minus => self.left(),
            Sym::Push => {}
            Sym::Pop => {}
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(1200, 1200);
    let mut rules = HashMap::new();
    let n = 6;
    rules.insert(
        'F',
        vec![
            'F',
            '-',
            'G',
            '+',
            'F',
            '+',
            'G',
            '-',
            'F',
        ],
    );
    rules.insert('G', vec!['G', 'G']);
    let axiom = vec!['F', '-', 'G', '-', 'G'];
    let mut cmds = HashMap::new();
    cmds.insert('F', Sym::F);
    cmds.insert('-', Sym::Minus);
    cmds.insert('+', Sym::Plus);
    cmds.insert('G', Sym::F);
    let production = iter_rules(axiom, &rules, n);
    let production = to_sym(production, &cmds);
    let mut turtle = Turtle::new();
    for k in production {
        turtle.interp(k);
    }
    let path = turtle.pb.finish();
    let texture = Texture::SolidColor(turtle.color);
    let mut stroke = Stroke::default();
    stroke.width = turtle.thickness;
    canvas.fill(RGBA::white());
    canvas.stroke_path(&path, &texture, &stroke);
    canvas.save("koch.png")
}
