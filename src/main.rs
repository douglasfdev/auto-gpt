mod my_funcs;
mod other_funcs;
mod mod_async;
mod collections;

use crate::my_funcs::add_five;
use crate::other_funcs::minus_funcs::subtract_ten;

#[tokio::main]
async fn main() {
  let x: u32 = 50;
  let y: u32 = 25;
  let add_five_result: u32 = add_five(x);
  let subtract_ten: u32 = subtract_ten(y);

  println!("Resultado de add_five: {} | subtract: {}", add_five_result, subtract_ten);
}