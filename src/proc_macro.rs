#[cfg(test)]
mod tests {
  use llm_proc_macro::function_to_string;

  #[allow(dead_code)]
  const OUTPUT: &str = "";

  #[function_to_string]
  fn some_function_for_ai_llm(_whathever_param: &str) {
    /// This is an awesome function
    /// Well shall give it to an AI to guess and output
    /// IN a structured manner
    println!("{}", OUTPUT)
  }

  #[test]
  fn test_llm_proc_macros() {
    let x: &str = some_function_for_ai_llm("some input");

    dbg!(x);
  }
}