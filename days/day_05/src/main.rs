use std::{
  collections::HashSet,
  path::{Path, PathBuf},
};

#[derive(Hash, Eq, PartialEq, Debug)]
struct PageOrderingRule {
  before: u32,
  after:  u32,
}

struct Input {
  rules:     HashSet<PageOrderingRule>,
  page_sets: Vec<Vec<u32>>,
}

impl Input {
  fn from_file(input_file_path: &Path) -> Self {
    let content = std::fs::read_to_string(input_file_path)
      .expect("failed to read input file");

    Self::from_string(content)
  }

  fn from_string(content: String) -> Self {
    let mut rules = HashSet::new();
    let mut page_sets = Vec::new();

    for line in content.lines() {
      if line.contains('|') {
        let rule_parts = line.split('|').collect::<Vec<_>>();
        rules.insert(PageOrderingRule {
          before: rule_parts[0]
            .trim()
            .parse()
            .expect("failed to parse \"before\" number"),
          after:  rule_parts[1]
            .trim()
            .parse()
            .expect("failed to parse \"after\" number"),
        });
      } else {
        if line.is_empty() {
          continue;
        }

        page_sets.push(
          line
            .split(',')
            .map(|part| part.parse().expect("failed to parse number"))
            .collect(),
        );
      }
    }

    Self { rules, page_sets }
  }

  fn complies_with_rules(&self, page_set: &[u32]) -> bool {
    for PageOrderingRule { before, after } in self.rules.iter() {
      if !page_set.contains(before) || !page_set.contains(after) {
        continue;
      }

      let before_index =
        page_set.iter().position(|&page| page == *before).unwrap();
      let after_index =
        page_set.iter().position(|&page| page == *after).unwrap();

      if before_index > after_index {
        return false;
      }
    }

    true
  }

  fn part_1(&self) -> u32 {
    let complies_with_rules = |page_set: &[u32]| -> bool {
      for PageOrderingRule { before, after } in self.rules.iter() {
        if !page_set.contains(before) || !page_set.contains(after) {
          continue;
        }

        let before_index =
          page_set.iter().position(|&page| page == *before).unwrap();
        let after_index =
          page_set.iter().position(|&page| page == *after).unwrap();

        if before_index > after_index {
          return false;
        }
      }

      true
    };

    self
      .page_sets
      .iter()
      .filter(|page_set| complies_with_rules(page_set))
      // get the middle number of each page set
      .map(|page_set| page_set[page_set.len() / 2])
      .sum()
  }

  fn part_2(&self) -> u32 {
    let non_compliant_page_sets: Vec<_> = self
      .page_sets
      .iter()
      .filter(|page_set| !self.complies_with_rules(page_set))
      .collect();

    // correct the non-compliant page sets
    // for each page set, iterate through the rules and swap the numbers if the
    // rule is violated

    let corrected_page_sets = non_compliant_page_sets
      .iter()
      .map(|page_set| {
        let mut page_set = Vec::clone(page_set);
        let mut changed = true;
        while changed {
          changed = false;
          for PageOrderingRule { before, after } in self.rules.iter() {
            if !page_set.contains(before) || !page_set.contains(after) {
              continue;
            }

            let before_index =
              page_set.iter().position(|&page| page == *before).unwrap();
            let after_index =
              page_set.iter().position(|&page| page == *after).unwrap();

            if before_index > after_index {
              page_set.swap(before_index, after_index);
              changed = true;
            }
          }
        }

        page_set
      })
      .collect::<Vec<_>>();

    // get the middle number of each page set
    corrected_page_sets
      .iter()
      .map(|page_set| page_set[page_set.len() / 2])
      .sum()
  }
}

fn main() {
  let now = std::time::Instant::now();
  let input = Input::from_file(PathBuf::from("inputs/day_05.txt").as_path());
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
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

  #[test]
  fn part_1_example() {
    let input = Input::from_string(MINI_INPUT.to_string());
    assert_eq!(input.part_1(), 143);
  }

  #[test]
  fn part_2_example() {
    let input = Input::from_string(MINI_INPUT.to_string());
    assert_eq!(input.part_2(), 123);
  }
}
