// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// Create a grid of values based on a function of it's coordinates. Used for
// example for flow fields.
pub struct Grid<T> {
    pub width: f32,
    pub height: f32,
    pub spacing: f32,
    pub grid: Vec<T>,
    pub pts: Vec<Point>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(width: f32, height: f32, spacing: f32, gen: impl Fn(f32, f32) -> T) -> Self {
        let rows = (height / spacing) as usize;
        let cols = (width / spacing) as usize;
        let mut grid = vec![];
        let mut pts = vec![];
        for i in 0..rows {
            let y = i as f32 * spacing;
            for j in 0..cols {
                let x = j as f32 * spacing;
                grid.push(gen(x, y));
                pts.push(pt2(x, y));
            }
        }
        Self {
            width,
            height,
            spacing,
            grid,
            pts,
        }
    }

    pub fn rows(&self) -> usize {
        (self.height / self.spacing) as usize
    }

    pub fn cols(&self) -> usize {
        (self.width / self.spacing) as usize
    }

    pub fn get(&self, x: f32, y: f32) -> T {
        let n = self.rows();
        let m = self.cols();
        let xn = x;
        let yn = y;

        let mut col = if xn < 0.0 {
            0
        } else {
            (x / self.spacing) as usize
        };
        let mut row = if yn < 0.0 {
            0
        } else {
            (y / self.spacing) as usize
        };

        while col >= m {
            col -= 1;
        }
        while row >= n {
            row -= 1;
        }

        self.grid[row * m + col]
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }

    pub fn x_bounds(&self) -> (f32, f32) {
        (0.0, self.width)
    }

    pub fn y_bounds(&self) -> (f32, f32) {
        (0.0, self.height)
    }
}

pub struct GridIter<'a, T>
where
    T: Copy,
{
    grid: &'a Grid<T>,
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Copy,
{
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        let n = (self.grid.width / self.grid.spacing) as usize;
        if self.i * n + self.j >= self.grid.grid.len() {
            return None;
        };
        let x = self.j as f32 * self.grid.spacing;
        let y = self.i as f32 * self.grid.spacing;
        let result = (pt2(x, y), self.grid.grid[self.i * n + self.j]);

        if self.j >= n - 1 {
            self.j = 0;
            self.i += 1;
        } else {
            self.j += 1;
        };

        Some(result)
    }
}

pub fn gen_points(
    f: impl Fn(f32) -> f32,
    g: impl Fn(f32) -> f32,
    delta: f32,
    max: f32,
) -> Vec<Point> {
    let mut points = vec![];
    let mut t = 0.0;
    while t <= max {
        let x = f(t);
        let y = g(t);
        points.push(pt2(x, y));
        t += delta;
    }
    points
}