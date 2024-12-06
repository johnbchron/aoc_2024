mod grid;

use std::path::{Path, PathBuf};

use rayon::prelude::*;

use self::grid::Grid;

#[derive(Clone)]
struct Input {
  grid:         Grid,
  starting_pos: (usize, usize),
  starting_dir: Direction,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn turn_right(&self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }

  fn vector(&self) -> (isize, isize) {
    match self {
      Direction::Up => (0, -1),
      Direction::Right => (1, 0),
      Direction::Down => (0, 1),
      Direction::Left => (-1, 0),
    }
  }

  fn as_usize(&self) -> usize {
    match self {
      Direction::Up => 0,
      Direction::Right => 1,
      Direction::Down => 2,
      Direction::Left => 3,
    }
  }
}

impl Input {
  fn from_file(input_file_path: &Path) -> Self {
    let content = std::fs::read_to_string(input_file_path)
      .expect("failed to read input file");

    Self::from_string(content)
  }

  fn from_string(content: String) -> Self {
    let mut grid = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut starting_pos = (0, 0);
    let mut starting_dir = Direction::Up;

    for (y, line) in content
      .lines()
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .enumerate()
    {
      height += 1;
      width = line.len();

      for (x, c) in line.chars().enumerate() {
        match c {
          '.' => grid.push(false),
          '#' => grid.push(true),
          '^' => {
            grid.push(false);
            starting_pos = (x, y);
            starting_dir = Direction::Up;
          }
          '>' => {
            grid.push(false);
            starting_pos = (x, y);
            starting_dir = Direction::Right;
          }
          '<' => {
            grid.push(false);
            starting_pos = (x, y);
            starting_dir = Direction::Left;
          }
          'v' => {
            grid.push(false);
            starting_pos = (x, y);
            starting_dir = Direction::Down;
          }
          c => panic!("unexpected character in input file: {c:?}"),
        }
      }
    }

    Self {
      grid: Grid::from_vec(width, height, grid),
      starting_pos,
      starting_dir,
    }
  }

  fn part_1(&self) -> usize {
    // walk through the grid. if we hit a wall, turn right. if we go out of
    // bounds, stop.

    let mut visited_grid = Grid::new(self.grid.width(), self.grid.height());

    let mut pos = self.starting_pos;
    let mut dir = self.starting_dir;

    loop {
      visited_grid.set(pos.0, pos.1, true);

      let (dx, dy) = dir.vector();
      let (x, y) = (pos.0 as isize + dx, pos.1 as isize + dy);

      if x < 0
        || x >= self.grid.width() as isize
        || y < 0
        || y >= self.grid.height() as isize
      {
        break;
      }

      if self.grid.get(x as usize, y as usize) {
        dir = dir.turn_right();
        continue;
      }

      pos = (x as usize, y as usize);
    }

    visited_grid.iter().filter(|&v| v).count()
  }

  fn part_2(&self) -> usize {
    let cell_count = self.grid.width() * self.grid.height();

    let mutate_input = |i: usize| -> Option<Self> {
      let mut input = self.clone();

      let x = i % self.grid.width();
      let y = i / self.grid.width();

      // if the cell is already filled or is the starting position, return None
      if input.grid.get(x, y) || (x, y) == input.starting_pos {
        return None;
      }

      // otherwise fill the cell
      input.grid.set(x, y, true);

      Some(input)
    };
    let check_sim_for_loops = |input: &Input| -> bool {
      let mut visited_grid_directions =
        vec![[false; 4]; input.grid.width() * input.grid.height()];

      let mut pos = input.starting_pos;
      let mut dir = input.starting_dir;

      loop {
        if visited_grid_directions[pos.1 * input.grid.width() + pos.0]
          [dir.as_usize()]
        {
          return true;
        }
        visited_grid_directions[pos.1 * input.grid.width() + pos.0]
          [dir.as_usize()] = true;

        let (dx, dy) = dir.vector();
        let (x, y) = (pos.0 as isize + dx, pos.1 as isize + dy);

        if x < 0
          || x >= input.grid.width() as isize
          || y < 0
          || y >= input.grid.height() as isize
        {
          break;
        }

        if input.grid.get(x as usize, y as usize) {
          dir = dir.turn_right();
          continue;
        }

        pos = (x as usize, y as usize);
      }

      false
    };

    (0..cell_count)
      .into_par_iter()
      .filter_map(mutate_input)
      .filter(check_sim_for_loops)
      .count()
  }
}

fn main() {
  let now = std::time::Instant::now();
  let input = Input::from_file(PathBuf::from("inputs/day_06.txt").as_path());
  println!(
    "Parsed input in {:.3}ms",
    now.elapsed().as_secs_f32() * 1000.0
  );

  let now = std::time::Instant::now();
  let part_1 = input.part_1();
  println!(
    "Part 1: {part_1}, in {:.3}ms",
    now.elapsed().as_secs_f32() * 1000.0
  );
  let now = std::time::Instant::now();
  let part_2 = input.part_2();
  println!(
    "Part 2: {part_2}, in {:.3}ms",
    now.elapsed().as_secs_f32() * 1000.0
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  const MINI_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

  #[test]
  fn part_1_example() {
    let input = Input::from_string(MINI_INPUT.to_string());
    assert_eq!(input.part_1(), 41);
  }

  #[test]
  fn part_2_example() {
    let input = Input::from_string(MINI_INPUT.to_string());
    assert_eq!(input.part_2(), 6);
  }
}
