use std::path::{Path, PathBuf};

use regex::Regex;

#[derive(Debug, Clone)]
struct Input {
  data: String,
}

impl Input {
  fn from_file(input_file_path: &Path) -> Self {
    let data = std::fs::read_to_string(input_file_path)
      .expect("failed to read input file");
    Self { data }
  }

  fn part_1(&self) -> u64 {
    let re =
      Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("failed to build regex");

    let output = re
      .captures_iter(&self.data)
      .map(|cap| {
        let first =
          cap[1].parse::<u64>().expect("failed to parse first number");
        let second = cap[2]
          .parse::<u64>()
          .expect("failed to parse second number");

        first * second
      })
      .sum::<u64>();

    output
  }

  fn part_2(&self) -> u64 {
    // parse mul instructions, and keep their position in the text
    let mul_re =
      Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("failed to build regex");
    let mul_instructions: Vec<_> = mul_re
      .captures_iter(&self.data)
      .map(|cap| {
        let first = cap
          .get(1)
          .unwrap()
          .as_str()
          .parse::<u64>()
          .expect("failed to parse first number");
        let second = cap
          .get(2)
          .unwrap()
          .as_str()
          .parse::<u64>()
          .expect("failed to parse second number");

        (cap.get(0).unwrap().start(), Instruction::Mul(first, second))
      })
      .collect();

    // parse do instructions, and keep their position in the text
    let do_re = Regex::new(r"do\(\)").expect("failed to build regex");
    let do_instructions: Vec<_> = do_re
      .find_iter(&self.data)
      .map(|m| (m.start(), Instruction::Do))
      .collect();

    // parse dont instructions, and keep their position in the text
    let dont_re = Regex::new(r"don't\(\)").expect("failed to build regex");
    let dont_instructions: Vec<_> = dont_re
      .find_iter(&self.data)
      .map(|m| (m.start(), Instruction::Dont))
      .collect();

    // merge all instructions, sort them by their position in the text
    let mut instructions = mul_instructions;
    instructions.extend(do_instructions);
    instructions.extend(dont_instructions);
    instructions.sort_by_key(|(s, _)| *s);

    // apply instructions in order. `mul` is disabled after `dont`, and enabled
    // after `do`
    let mut mul_enabled = true;
    let mut output = 0;
    for (_, instruction) in instructions {
      match instruction {
        Instruction::Mul(first, second) => {
          if mul_enabled {
            output += first * second;
          }
        }
        Instruction::Do => {
          mul_enabled = true;
        }
        Instruction::Dont => {
          mul_enabled = false;
        }
      }
    }

    output
  }
}

enum Instruction {
  Mul(u64, u64),
  Do,
  Dont,
}

fn main() {
  let input = Input::from_file(PathBuf::from("inputs/day_03.txt").as_path());

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
