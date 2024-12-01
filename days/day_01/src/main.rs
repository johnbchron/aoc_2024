use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
};

#[derive(Debug, Clone)]
struct Input {
  first_list:  Vec<u32>,
  second_list: Vec<u32>,
}

impl Input {
  fn from_file(input_file_path: &PathBuf) -> Self {
    let input_file_string = std::fs::read_to_string(input_file_path)
      .expect("failed to read input file");

    let (first_list, second_list): (Vec<_>, Vec<_>) = input_file_string
      .lines()
      .map(|line| {
        let mut parts = line.split_whitespace();
        let first = parts
          .next()
          .expect("missing first number")
          .parse::<u32>()
          .expect("failed to parse first number");
        let second = parts
          .next()
          .expect("missing second number")
          .parse::<u32>()
          .expect("failed to parse second number");
        (first, second)
      })
      .unzip();

    Self {
      first_list,
      second_list,
    }
  }

  fn part_1(mut self) -> u32 {
    self.first_list.sort();
    self.second_list.sort();

    let output = (0..self.first_list.len())
      .map(|i| {
        let first = self.first_list[i] as i32;
        let second = self.second_list[i] as i32;

        (second - first).abs()
      })
      .sum::<i32>();
    output as u32
  }

  fn part_2(self) -> u32 {
    let first_histogram =
      self
        .first_list
        .iter()
        .fold(HashMap::new(), |mut acc, &value| {
          *acc.entry(value).or_insert(0) += 1;
          acc
        });
    let second_histogram =
      self
        .second_list
        .iter()
        .fold(HashMap::new(), |mut acc, &value| {
          *acc.entry(value).or_insert(0) += 1;
          acc
        });

    let first_keyset = first_histogram.keys().collect::<HashSet<_>>();
    let second_keyset = second_histogram.keys().collect::<HashSet<_>>();

    let intersection = first_keyset.intersection(&second_keyset);

    let output = intersection
      .map(|key| {
        let first_count = first_histogram.get(key).unwrap();
        let second_count = second_histogram.get(key).unwrap();

        **key * (first_count * second_count)
      })
      .sum::<u32>();

    output
  }
}

fn main() {
  let input = Input::from_file(&["inputs", "day_01"].iter().collect());

  println!("part_1: {}", input.clone().part_1());
  println!("part_2: {}", input.clone().part_2());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_2() {
    let input = Input {
      first_list:  vec![3, 4, 2, 1, 3, 3],
      second_list: vec![4, 3, 5, 3, 9, 3],
    };

    assert_eq!(input.part_2(), 31);
  }
}
