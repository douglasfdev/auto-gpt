use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_hashmaps() {
    let person_one: &str = "Alice";
    let person_two: &str = "Bob";

    let mut results_rm: HashMap<&str, u32> = HashMap::new();
    results_rm.insert(person_one, 55);
    results_rm.insert(person_two, 51);

    let test_result: Option<&u32> = results_rm.get(person_one);
    dbg!(test_result.unwrap());

    if results_rm.contains_key("Alice") {
      dbg!("Alice is in the map");
    }
  }

  #[test]
  fn test_hashsets() {
    let mut names_hs: HashSet<&str> = HashSet::new();
    names_hs.insert("Alice");
    names_hs.insert("Bob");
    names_hs.insert("Jane");

    if names_hs.contains("Bob") {
      dbg!("Bob is in the set");
    }
  }
}