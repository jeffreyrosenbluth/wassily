use noise::*;
use std::collections::HashMap;
use wassily::prelude::*;
use wassily::skia::Canvas;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Sym {
    F,
    Plus,
    Minus,
    Push,
    Pop,
}

type Rules = HashMap<char, Vec<char>>;
type Axiom = Vec<char>;
type Cmds = HashMap<char, Sym>;

fn std_cmds() -> Cmds {
    let mut cmds = HashMap::new();
    cmds.insert('F', Sym::F);
    cmds.insert('-', Sym::Minus);
    cmds.insert('+', Sym::Plus);
    cmds
}

fn add_rule(k: char, v: &str, rules: &mut Rules) {
    let v = v.chars().collect();
    rules.insert(k, v);
}

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

struct Lsystem {
    points: Vec<Point>,
    direction: f32,
    angle: f32,
    color: RGBA,
    length: f32,
    thickness: f32,
    stack: Vec<Sym>,
    axiom: Axiom,
    rules: Rules,
    cmds: Cmds,
}

impl Lsystem {
    fn new(start: Point, angle: f32, length: f32, axiom: Axiom, rules: Rules, cmds: Cmds) -> Self {
        Self {
            points: vec![start],
            direction: 0.0,
            angle,
            color: RGBA::white(),
            length,
            thickness: 2.0,
            stack: vec![],
            axiom,
            rules,
            cmds,
        }
    }

    fn forward(&mut self, distance: f32) {
        let dx = distance * self.direction.cos();
        let dy = distance * self.direction.sin();
        let n = self.points.len() - 1;
        self.points
            .push(point2(self.points[n].x + dx, self.points[n].y + dy));
    }

    fn right(&mut self) {
        self.direction -= self.angle;
    }

    fn left(&mut self) {
        self.direction += self.angle;
    }

    fn interp(&mut self, sym: Sym) {
        match sym {
            Sym::F => self.forward(self.length),
            Sym::Plus => self.right(),
            Sym::Minus => self.left(),
            Sym::Push => {}
            Sym::Pop => {}
        }
    }

    fn run<T>(&mut self, canvas: &mut Canvas, iters: u32, ns: Noise<[f64; 3], T>)
    where
        T: noise::NoiseFn<[f64; 3]>,
    {
        let production = iter_rules(self.axiom.clone(), &self.rules, iters);
        let production = to_sym(production, &self.cmds);
        for k in production {
            self.interp(k);
        }
        let points: Vec<Point> = self
            .points
            .iter()
            .map(|p| point2(p.x + ns.noise(p.x, p.y, 0.0), p.y + ns.noise(p.x, p.y, 0.1)))
            .collect();


        let path = ShapeBuilder::new()
            .points(&points)
            .no_fill()
            .fill_color(RGBA::new(1.0, 0.0, 0.0, 0.5))
            .stroke_weight(self.thickness)
            .stroke_color(self.color)
            .line_join(LineJoin::Bevel)
            .fill_rule(FillRule::EvenOdd)
            // .quad()
            .build();

        path.draw(canvas);
    }
}

fn main() {
    let mut canvas = Canvas::new(8191, 8191);
    canvas.fill(RGBA::black());
    let mut ns = Noise::<[f64; 3], _>::new(8191.0, 8191.0, OpenSimplex::new());
    ns.set_noise_factor(300.);
    ns.set_noise_scale(10.0);

    // Dragon
    ns.set_noise_factor(300.);
    ns.set_noise_scale(10.0);
    let mut rules = HashMap::new();
    let axiom = vec!['F'];
    add_rule('F', "F+G", &mut rules);
    add_rule('G', "F-G", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('G', Sym::F);
    let mut dragon = Lsystem::new(
        point2(6000., 2800.),
        PI / 2.0,
        60.0,
        axiom.clone(),
        rules,
        cmds,
    );
    dragon.thickness = 8.0;

    // Koch Lake
    ns.set_noise_factor(300.);
    ns.set_noise_scale(30.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "F+F+F+F".chars().collect();
    add_rule('F', "F+f-FF+F+FF+Ff+FF-f+FF-F-FF-Ff-FFF", &mut rules);
    add_rule('f', "ffffff", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('f', Sym::F);
    let mut lake = Lsystem::new(
        point2(1950., 6290.0),
        PI / 2.0,
        20.0,
        axiom.clone(),
        rules,
        cmds,
    );
    lake.thickness = 8.0;

    // Koch 3
    ns.set_noise_factor(150.);
    ns.set_noise_scale(10.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "F-F-F".chars().collect();
    add_rule('F', "FF-F+F-F-FF", &mut rules);
    let cmds = std_cmds();
    let mut koch3 = Lsystem::new(
        point2(2700., 2700.0),
        PI / 2.0,
        115.0,
        axiom.clone(),
        rules,
        cmds,
    );
    koch3.thickness = 20.0;
    koch3.color = RGBA::new(1.0, 1.0, 1.0, 0.5);

    koch3.run(&mut canvas, 4, ns);
    canvas.save("lsys.png")
}
