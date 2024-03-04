/**
 * Function: add_five
 * # Arguments (num: u32)
 * # Returns (u32)
 * # Example
 * ```
 * let x = 5;
 * let y = add_five(x);
 * ```
 */
pub fn add_five(num: u32) -> u32 {
  /*
   First Item
   Returns result
  */
  num + 5
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_five_test() {
    assert_eq!(add_five(5), 10);
  }
}