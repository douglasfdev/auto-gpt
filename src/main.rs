mod my_func;
mod other_funcs;

use crate::my_func::add_five;
use crate::other_funcs::minus_funcs::subtract_ten;

fn main() {
  let x: u32 = 50;
  let y: u32 = 20;
  let add_five_result: u32 = add_five(x);
  let subtract_ten: u32 = subtract_ten(y);

  println!("Resultado de add_five: {} | subtract: {}", add_five_result, subtract_ten);
}