use noise::*;
use std::collections::HashMap;
use wassily::prelude::*;
use wassily::raqote::Canvas;

const WIDTH: u32 = 8191;
const HEIGHT: u32 = 8191;

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

struct Lsystem<T>
where
    T: noise::NoiseFn<3>,
{
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
    ns: Noise<T, 3>,
    width: u32,
    height: u32,
    depth: u32,
}

impl<T> Lsystem<T>
where
    T: noise::NoiseFn<3>,
{
    fn new(
        angle: f32,
        length: f32,
        axiom: Axiom,
        rules: Rules,
        cmds: Cmds,
        ns: Noise<T, 3>,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Self {
        Self {
            points: vec![TaggedPoint::new(point2(0.0, 0.0))],
            direction: 0.0,
            angle,
            color: WHITE,
            length,
            thickness: 2.0,
            stack: vec![],
            axiom,
            rules,
            cmds,
            ns,
            width,
            height,
            depth,
        }
    }

    fn jitter(&mut self) -> Point {
        let dx = self.length * self.direction.cos();
        let dy = self.length * self.direction.sin();
        let n = self.points.len() - 1;
        let x = self.points[n].point.x + dx;
        let y = self.points[n].point.y + dy;
        let x1 = x + self.ns.noise(x, y, 0.0);
        let y1 = y + self.ns.noise(x, y, 0.1);
        point2(x1, y1)
    }

    fn forward(&mut self) {
        let p = self.jitter();
        self.points.push(TaggedPoint::new(p));
    }

    fn go(&mut self) {
        let p = self.jitter();
        self.points.push(TaggedPoint::with_pen(p, false));
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
            Sym::F => self.forward(),
            Sym::G => self.go(),
            Sym::Plus => self.right(),
            Sym::Minus => self.left(),
            Sym::Push => self.push(),
            Sym::Pop => self.pop(),
            Sym::Null => {}
        }
    }

    fn bounding_box(&self) -> (Point, Point) {
        let mut low_x = f32::MAX;
        let mut high_x = f32::MIN;
        let mut low_y = f32::MAX;
        let mut high_y = f32::MIN;

        for q in &self.points {
            let p = q.point;
            if p.x < low_x {
                low_x = p.x
            }
            if p.x > high_x {
                high_x = p.x
            }
            if p.y < low_y {
                low_y = p.y
            }
            if p.y > high_y {
                high_y = p.y
            }
        }
        (point2(low_x, low_y), point2(high_x, high_y))
    }

    fn run(&mut self, canvas: &mut Canvas) {
        let production = iter_rules(self.axiom.clone(), &self.rules, self.depth);
        let production = to_sym(production, &self.cmds);
        for k in production {
            self.interp(k);
        }

        let (low, high) = self.bounding_box();
        let mx = 0.5 * (high.x + low.x);
        let my = 0.5 * (high.y + low.y);
        let x = self.width as f32 / 2.0 - mx;
        let y = (self.height as f32) / 2.0 - my;
        let transform = Transform::identity().post_translate(vec2(x, y));

        let path = ShapeBuilder::new()
            .tagged_points(&self.points)
            .no_fill()
            // .fill_color(RGBA::white())
            .stroke_weight(self.thickness)
            .stroke_color(self.color)
            // .line_join(LineJoin::Bevel)
            .line_cap(LineCap::Square)
            .fill_rule(FillRule::EvenOdd)
            .transform(transform)
            .build();

        path.draw(canvas);
    }
}

// Dragon
#[allow(dead_code)]
fn dragon() -> Lsystem<OpenSimplex> {
    let ns = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_factor(0.)
        .set_noise_scales(10.0, 10.0, 10.0);
    let mut rules = HashMap::new();
    let axiom = vec!['F'];
    add_rule('F', "F+G", &mut rules);
    add_rule('G', "F-G", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('G', Sym::F);
    let mut dragon = Lsystem::new(
        PI / 2.0,
        60.0,
        axiom.clone(),
        rules,
        cmds,
        ns,
        WIDTH,
        HEIGHT,
        13,
    );
    dragon.thickness = 8.0;
    dragon
}

// Koch Lake
#[allow(dead_code)]
fn lake() -> Lsystem<OpenSimplex> {
    let ns = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_factor(0.)
        .set_noise_scales(10.0, 10.0, 10.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "F+F+F+F".chars().collect();
    add_rule('F', "F+f-FF+F+FF+Ff+FF-f+FF-F-FF-Ff-FFF", &mut rules);
    add_rule('f', "ffffff", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('f', Sym::F);
    let mut lake = Lsystem::new(
        PI / 2.0,
        20.0,
        axiom.clone(),
        rules,
        cmds,
        ns,
        WIDTH,
        HEIGHT,
        3,
    );
    lake.thickness = 8.0;
    lake
}

// Koch 3
#[allow(dead_code)]
fn koch3() -> Lsystem<OpenSimplex> {
    let ns = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_factor(0.)
        .set_noise_scales(10.0, 10.0, 10.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "F-F-F".chars().collect();
    add_rule('F', "FF-F+F-F-FF", &mut rules);
    let cmds = std_cmds();
    let mut koch3 = Lsystem::new(
        PI / 2.0,
        115.0,
        axiom.clone(),
        rules,
        cmds,
        ns,
        WIDTH,
        HEIGHT,
        3,
    );
    koch3.thickness = 20.0;
    koch3.color = RGBA::new(1.0, 1.0, 1.0, 0.5);
    koch3
}

// Fern
#[allow(dead_code)]
fn fern() -> Lsystem<OpenSimplex> {
    let ns = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_factor(0.0)
        .set_noise_scales(20.0, 20.0, 20.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "X".chars().collect();
    add_rule('F', "FF", &mut rules);
    add_rule('X', "F+[[X]-X]-F[-FX]+X", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('X', Sym::Null);
    let mut fern = Lsystem::new(
        50.0 * PI / 360.0,
        50.0,
        axiom.clone(),
        rules,
        cmds,
        ns,
        WIDTH,
        HEIGHT,
        5,
    );
    fern.thickness = 20.0;
    fern.direction = -PI / 2.0;
    fern.color = RGBA::new(0.4, 0.8, 0.3, 1.0);
    fern
}

// Sierpinski
#[allow(dead_code)]
fn sier() -> Lsystem<OpenSimplex> {
    let ns = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_factor(0.0)
        .set_noise_scales(20.0, 20.0, 20.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "F+XF+F+XF".chars().collect();
    add_rule('X', "XF-F+F-XF+F+XF-F+F-X", &mut rules);
    let mut cmds = std_cmds();
    cmds.insert('X', Sym::Null);
    let mut sier = Lsystem::new(
        PI / 2.0,
        55.0,
        axiom.clone(),
        rules,
        cmds,
        ns,
        WIDTH,
        HEIGHT,
        5,
    );
    sier.thickness = 20.0;
    sier.direction = PI / 2.0;
    sier
}

// Carpet
#[allow(dead_code)]
fn carpet() -> Lsystem<OpenSimplex> {
    let ns = Noise::<_, 3>::new(WIDTH as f32, HEIGHT as f32, OpenSimplex::default())
        .set_noise_factor(0.0)
        .set_noise_scales(20.0, 20.0, 20.0);
    let mut rules = HashMap::new();
    let axiom: Vec<char> = "F-F-F-F".chars().collect();
    add_rule('F', "F[F]-F+F[--F]+F-F", &mut rules);
    let cmds = std_cmds();
    let mut carpet = Lsystem::new(
        PI / 2.0,
        75.0,
        axiom.clone(),
        rules,
        cmds,
        ns,
        WIDTH,
        HEIGHT,
        4,
    );
    carpet.color = RGBA::with_8(191, 36, 93, 100);
    carpet.thickness = 15.0;
    carpet.direction = PI / 2.0;
    carpet
}
fn main() {
    let mut canvas = Canvas::new(WIDTH, WIDTH);
    canvas.fill(RGBA::with_8(242, 187, 197, 255));
    // canvas.fill(RGBA::with_8(242, 232, 233, 255));

    let mut lsys = carpet();
    lsys.run(&mut canvas);
    canvas.save("carpet.png")
}
