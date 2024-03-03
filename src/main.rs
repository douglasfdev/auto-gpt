fn add_five(num: u32) -> u32 {
  return num + 5;
}

fn main() {
  let mut x: u32 = 50;
  let add_five_result: u32 = add_five(x);

  println!("Resultado de add_five: {}", add_five_result);
}