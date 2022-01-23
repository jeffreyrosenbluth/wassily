use crate::matrix::*;
use crate::prelude::pt;
use crate::quiet::{noise2d, NoiseOpts};
use noise::NoiseFn;
use std::collections::VecDeque;
use std::f32::consts::PI;
use tiny_skia::Point;

pub type Cell = (u32, u32);
pub type Curve = Vec<Vertex>;
pub type Grid = Matrix<Vec<Vertex>>;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub cell: Cell,
}

impl Vertex {
    pub fn new(x: f32, y: f32, theta: f32, cell_size: f32) -> Self {
        let mut v = Vertex {
            x,
            y,
            theta,
            cell: (0, 0),
        };
        let cell = v.cell_of(cell_size);
        v.cell = cell;
        v
    }

    pub fn to_point(&self) -> Point {
        pt(self.x, self.y)
    }

    pub fn dist2(&self, other: &Vertex) -> f32 {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }

    pub fn distance(&self, width: u32, height: u32, grid: &Grid) -> f32 {
        let mut min_dist = std::f32::MAX;
        let i = self.cell.0;
        let j = self.cell.1;
        let mut d: f32;
        let ns = neighbors(width, height, i, j);
        for c in ns {
            if !grid.valid(c.0 as usize, c.1 as usize) {
                break;
            }
            for v in &grid[c.0 as usize][c.1 as usize] {
                d = self.dist2(v);
                if d < min_dist {
                    min_dist = d;
                }
            }
        }
        min_dist.sqrt()
    }

    pub fn cell_of(&self, cell_size: f32) -> Cell {
        let x0 = self.x / cell_size;
        let y0 = self.y / cell_size;
        (x0 as u32, y0 as u32)
    }
}

pub struct FlowField {
    pub grid: Grid,
    pub noise_function: Box<dyn NoiseFn<f64, 2>>,
    pub noise_opts: NoiseOpts,
    pub sepration: f32,
    pub step_size: f32,
    pub width: u32,
    pub height: u32,
    pub max_length: u32,
}

impl FlowField {
    pub fn new(
        grid: Grid,
        noise_function: Box<dyn NoiseFn<f64, 2>>,
        noise_opts: NoiseOpts,
        sepration: f32,
        step_size: f32,
        width: u32,
        height: u32,
        max_length: u32,
    ) -> Self {
        Self {
            grid,
            noise_function,
            noise_opts,
            sepration,
            step_size,
            width,
            height,
            max_length,
        }
    }

    pub fn curve(&mut self, x: f32, y: f32) -> Vec<Point> {
        let mut pts: VecDeque<Vertex> = VecDeque::new();
        let mut theta = noise2d(&self.noise_function, &self.noise_opts, x, y) * PI;
        let v = Vertex::new(x, y, theta, self.sepration);
        if v.distance(self.width, self.height, &self.grid) < self.sepration {
            return vec![];
        }
        pts.push_back(v);
        let mut p: Vertex;
        let mut x1: f32;
        let mut y1: f32;
        let mut v1: Vertex;
        for _ in 0..self.max_length {
            p = *pts.back().unwrap();
            x1 = p.x + self.step_size * p.theta.cos();
            y1 = p.y + self.step_size * p.theta.sin();
            theta = noise2d(&self.noise_function, &self.noise_opts, x1, y1) * PI;
            v1 = Vertex::new(x1, y1, theta, self.sepration);
            if v1.distance(self.width, self.height, &self.grid) < self.sepration
                || v1.x > self.width as f32 - 1.0
                || v1.y > self.height as f32 - 1.0
                || v1.x < 0.0
                || v1.y < 0.0
            {
                break;
            } else {
                pts.push_back(v1);
            }
        }
        for _ in 0..self.max_length {
            p = *pts.front().unwrap();
            x1 = p.x - self.step_size * p.theta.cos();
            y1 = p.y - self.step_size * p.theta.sin();
            theta = noise2d(&self.noise_function, &self.noise_opts, x1, y1) * PI;
            v1 = Vertex::new(x1, y1, theta, self.sepration);
            if v1.distance(self.width, self.height, &self.grid) < self.sepration
                || v1.x > self.width as f32 - 1.0
                || v1.y > self.height as f32 - 1.0
                || v1.x < 0.0
                || v1.y < 0.0
            {
                break;
            } else {
                pts.push_front(v1);
            }
        }
        for v in pts.iter() {
            self.grid[v.cell.0 as usize][v.cell.1 as usize].push(*v);
        }
        pts.into_iter().map(|v| v.to_point()).collect()
    }
}

pub fn neighbors(width: u32, height: u32, i: u32, j: u32) -> Vec<Cell> {
    let i = i as i32;
    let j = j as i32;
    let mut x;
    let mut y;
    let mut cells = vec![];
    for di in -1..=1 {
        x = i + di;
        if !(0..=width as i32).contains(&x) {
            continue;
        }
        for dj in -1..=1 {
            y = j + dj;
            if (0..=height as i32).contains(&y) {
                cells.push((x as u32, y as u32));
            }
        }
    }
    cells
}
