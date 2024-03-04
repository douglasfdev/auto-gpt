pub fn add_five(num: u32) -> u32 {
  return num + 5;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_five_test() {
    assert_eq!(add_five(5), 10);
  }
}