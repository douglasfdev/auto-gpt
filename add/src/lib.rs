/**
 * Adds left and right
 *
 * # Arguments
 *
 * * `left` - a usize to add. The left hand side of the expression
 * * `right` - a usize to add. The right hand side of the expression
 *
 * # Example
 * ```
 * # use add_one::add; // Assuming the crate's name is `add_one`
 * let l: usize = 20;
 * let r: usize = 5;
 * assert_eq!(add(l, r), 25);
 * ```
 */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
