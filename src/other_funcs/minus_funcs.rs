pub fn subtract_ten(num: u32) -> u32 {
  return num - 10;
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn subtract_ten_test() {
    assert_eq!(subtract_ten(25), 15);
  }
}