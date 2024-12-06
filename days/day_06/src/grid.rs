#[derive(Clone)]
pub struct Grid {
  width:  usize,
  height: usize,
  grid:   Vec<bool>,
}

impl Grid {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      grid: vec![false; width * height],
    }
  }
  pub fn from_vec(width: usize, height: usize, grid: Vec<bool>) -> Self {
    Self {
      width,
      height,
      grid,
    }
  }
  pub fn get(&self, x: usize, y: usize) -> bool {
    self.grid[y * self.width + x]
  }
  pub fn set(&mut self, x: usize, y: usize, value: bool) {
    self.grid[y * self.width + x] = value;
  }
  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }
  pub fn iter(&self) -> GridIter {
    GridIter {
      grid: self,
      x:    0,
      y:    0,
    }
  }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Grid {
  fn to_string(&self) -> String {
    let mut out = String::new();

    for y in 0..self.height() {
      for x in 0..self.width() {
        let c = if self.get(x, y) { '#' } else { '.' };
        out.push(c);
      }
      out.push('\n');
    }
    out
  }
}

pub struct GridIter<'a> {
  grid: &'a Grid,
  x:    usize,
  y:    usize,
}

impl Iterator for GridIter<'_> {
  type Item = bool;

  fn next(&mut self) -> Option<Self::Item> {
    if self.y >= self.grid.height() {
      return None;
    }

    let value = self.grid.get(self.x, self.y);

    self.x += 1;
    if self.x >= self.grid.width() {
      self.x = 0;
      self.y += 1;
    }

    Some(value)
  }
}
