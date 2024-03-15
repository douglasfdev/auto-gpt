trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
enum Character {
  Warrior,
  Archer,
  Wizard,
}

impl Attacker for Character {
  fn choose_style(&self) -> String {
      match self {
          Character::Warrior => "wing chun".to_string(),
          Character::Archer => "kung fu".to_string(),
          Character::Wizard => "thai chi".to_string(),
      }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_traits() {
    let my_character = Character::Archer;
    let chosen_style = my_character.choose_style();

    dbg!(chosen_style);
  }
}