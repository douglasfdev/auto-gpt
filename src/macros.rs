// Macro Captures
/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* smtm
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name, etc
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>, String, Vec<i32>, etc
*/

/* path
    matches to a rust path
    std::collections::HashMap, std::io::Error, etc
*/

// REPETITION SPECIFIER

// * - match zero or more repetitions
// + - match one or more repetitions
// ? - match zero or one repetitions
// {n}

#[cfg(test)]
mod tests {
  #[allow(unused_macros)]
  macro_rules! mad_skills {
      // ($x: expr) => {
      //   format!("You sent a expression: {}", $x)
      // };
      ($x: ty) => {
        match stringify!($x) {
          "i32" => "You sent an i32".to_string(),
          "String" => "You sent a String".to_string(),
          "Vec<String>" => "You sent a Vec<String>".to_string(),
          "Option<T>" => "You sent an Option<T>".to_string(),
          "String" => "You sent a String".to_string(),
          "Vec<i32>" => "You sent a Vec<i32>".to_string(),
          _ => "You sent something else".to_string(),
        }
      }
  }

  #[test]
  fn tests_declarative_macro() {
    let some_var: String = mad_skills!(i32);
    dbg!(some_var);
  }
}