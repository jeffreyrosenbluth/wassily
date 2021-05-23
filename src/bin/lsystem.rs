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
    Null,
}

type Rules = HashMap<char, Vec<char>>;
type Axiom = Vec<char>;
type Cmds = HashMap<char, Sym>;

fn std_cmds() -> Cmds {
    let mut cmds = HashMap::new();
    cmds.insert('F', Sym::F);
    cmds.insert('G', Sym::G);
    cmds.insert('-', Sym::Minus);
    cmds.insert('+', Sym::Plus);
    cmds.insert('[', Sym::Push);
    cmds.insert(']', Sym::Pop);
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

struct State {
    position: Point,
    direction: f32,
}

impl State {
    fn new(position: Point, direction: f32) -> Self {
        Self {
            position,
            direction,
        }
    }
}

struct Lsystem {
    points: Vec<TaggedPoint>,
    direction: f32,
    angle: f32,
    color: RGBA,
    length: f32,
    thickness: f32,
    stack: Vec<State>,
    axiom: Axiom,
    rules: Rules,
    cmds: Cmds,
}

impl Lsystem {
    fn new(start: Point, angle: f32, length: f32, axiom: Axiom, rules: Rules, cmds: Cmds) -> Self {
        Self {
            points: vec![TaggedPoint::new(start)],
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
        self.points.push(TaggedPoint::new(point2(
            self.points[n].point.x + dx,
            self.points[n].point.y + dy,
        )));
    }

    fn go(&mut self, distance: f32) {
        let dx = distance * self.direction.cos();
        let dy = distance * self.direction.sin();
        let n = self.points.len() - 1;
        self.points.push(TaggedPoint::with_pen(
            point2(self.points[n].point.x + dx, self.points[n].point.y + dy),
            false,
        ));
    }

    fn right(&mut self) {
        self.direction -= self.angle;
    }

    fn left(&mut self) {
        self.direction += self.angle;
    }

    fn push(&mut self) {
        let n = self.points.len() - 1;
        let state = State::new(self.points[n].point, self.direction);
        self.stack.push(state);
    }

    fn pop(&mut self) {
        let state = self.stack.pop().expect("Nothing to pop");
        self.direction = state.direction;
        self.points
            .push(TaggedPoint::with_pen(state.position, false));
    }

    fn interp(&mut self, sym: Sym) {
        match sym {
            Sym::F => self.forward(self.length),
            Sym::G => self.go(self.length),
            Sym::Plus => self.right(),
            Sym::Minus => self.left(),
            Sym::Push => self.push(),
            Sym::Pop => self.pop(),
            Sym::Null => {},
        }
    }

    fn run<T>(&mut self, canvas: &mut Canvas, iters: u32, ns: Noise<[f64; 3], T>)
    where
        T: noise::NoiseFn<[f64; 3]>,
    {
        fn jitter(p: TaggedPoint, dx: f32, dy: f32) -> TaggedPoint {
            if p.pen {
                TaggedPoint::with_pen(point2(p.point.x + dx, p.point.y + dy), true)
            } else {
                p
            }
        }
        let production = iter_rules(self.axiom.clone(), &self.rules, iters);
        let production = to_sym(production, &self.cmds);
        for k in production {
            self.interp(k);
        }
        let points: Vec<TaggedPoint> = self
            .points
            .iter()
            .map(|p| {
                jitter(
                    *p,
                    ns.noise(p.point.x, p.point.y, 0.0),
                    ns.noise(p.point.x, p.point.y, 0.1),
                )
            })
            .collect();

        let path = ShapeBuilder::new()
            .tagged_points(&points)
            .no_fill()
            // .fill_color(RGBA::new(1.0, 0.0, 0.0, 0.5))
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

    // Fern
    ns.set_noise_factor(0.0);
    ns.set_noise_scale(10.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "X".chars().collect();
    add_rule('F', "FF", &mut rules);
    add_rule('X', "F+[[X]-X]-F[-FX]+X", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('X', Sym::Null);
    let mut fern = Lsystem::new(
        point2(4000.0, 8100.0),
        50.0 * PI / 360.0,
        50.0,
        axiom.clone(),
        rules,
        cmds,
    );
    fern.thickness = 20.0;
    fern.direction = -PI / 2.0;
    fern.color = RGBA::new(0.4, 0.8, 0.3, 1.0);

    fern.run(&mut canvas, 6, ns);
    canvas.save("fern.png")
}
