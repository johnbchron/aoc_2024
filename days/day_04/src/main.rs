use std::path::{Path, PathBuf};

use rayon::prelude::*;

const DIRECTIONS: [(i32, i32); 8] = [
  (0, 1),
  (1, 0),
  (1, 1),
  (-1, 1),
  (1, -1),
  (-1, -1),
  (0, -1),
  (-1, 0),
];

#[derive(Debug)]
struct Input {
  width:   usize,
  height:  usize,
  content: Vec<Vec<char>>,
}

impl Input {
  fn from_file(input_file_path: &Path) -> Self {
    let content = std::fs::read_to_string(input_file_path)
      .expect("failed to read input file");

    Self::from_string(content)
  }

  fn from_string(content: String) -> Self {
    let content: Vec<_> = content
      .lines()
      .filter(|line| !line.is_empty())
      .map(|line| line.chars().collect())
      .collect();

    let width = content
      .iter()
      .map(Vec::len)
      .max()
      .expect("failed to find max line width");
    let height = content.len();

    Self {
      width,
      height,
      content,
    }
  }

  fn char_at(&self, x: usize, y: usize) -> char {
    self
      .content
      .get(y)
      .and_then(|line| line.get(x))
      .copied()
      .unwrap_or_else(|| {
        panic!(
          "failed to get character at x = {}, y = {} (width = {}, height = {})",
          x, y, self.width, self.height
        )
      })
  }

  fn part_1(&self) -> u32 {
    const MATCH_STRING: &str = "XMAS";

    (0..self.height)
      .flat_map(|y| (0..self.width).map(move |x| (x, y)))
      .filter(|(x, y)| {
        self.char_at(*x, *y) == MATCH_STRING.chars().next().unwrap()
      })
      .map(|(x, y)| {
        DIRECTIONS
          .iter()
          .filter(|(dx, dy)| {
            (1..MATCH_STRING.len()).all(|i| {
              let x = (x as i32) + (i as i32) * dx;
              let y = (y as i32) + (i as i32) * dy;

              if x < 0
                || x >= self.width as i32
                || y < 0
                || y >= self.height as i32
              {
                return false;
              }

              self.char_at(x as usize, y as usize)
                == MATCH_STRING.chars().nth(i).unwrap()
            })
          })
          .count() as u32
      })
      .sum()
  }

  fn part_2(&self) -> u32 {
    (0..self.height)
      .flat_map(|y| (0..self.width).map(move |x| (x, y)))
      .filter(|(x, y)| self.char_at(*x, *y) == 'A')
      .filter(|(x, y)| {
        let x: i32 = *x as i32;
        let y: i32 = *y as i32;

        if (x - 1) < 0
          || (x + 1) >= self.width as i32
          || (y - 1) < 0
          || (y + 1) >= self.height as i32
        {
          return false;
        }

        let upper_left = self.char_at((x - 1) as usize, (y - 1) as usize);
        let upper_right = self.char_at((x + 1) as usize, (y - 1) as usize);
        let lower_left = self.char_at((x - 1) as usize, (y + 1) as usize);
        let lower_right = self.char_at((x + 1) as usize, (y + 1) as usize);

        // diagonals must match "MAS", forward or backward
        match (upper_left, lower_right) {
          ('M', 'S') | ('S', 'M') => (),
          _ => return false,
        }
        match (upper_right, lower_left) {
          ('M', 'S') | ('S', 'M') => (),
          _ => return false,
        }

        true
      })
      .count() as u32
  }
}

fn main() {
  let now = std::time::Instant::now();
  let input = Input::from_file(PathBuf::from("inputs/day_04.txt").as_path());
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

  #[test]
  fn test_part_1() {
    let input = Input::from_string(MINI_INPUT.to_string());
    assert_eq!(input.part_1(), 18);
  }

  #[test]
  fn test_part_2() {
    let input = Input::from_string(MINI_INPUT.to_string());
    assert_eq!(input.part_2(), 9);
  }
}
