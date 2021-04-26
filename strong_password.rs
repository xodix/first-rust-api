use std::collections::HashMap;

pub fn strong_pass(passwd: String) -> i32 {
  let mut n_steps = 0;
  // create a hash map of mistakes
  let mut map: HashMap<&str, bool> = HashMap::new();
  map.insert("!lowercase", false);
  map.insert("!uppercase", false);
  map.insert("!digit", false);
  // find mistakes in a string
  let mut last_char = ' ';
  let mut n_repeated = 0;
  for c in (&passwd).chars() {
    // check for char repetition
    if c == last_char {
      n_repeated += 1;
    } else {
      last_char = c;
    }
    if n_repeated == 3 {
      n_repeated = 0;
      n_steps += 1;
    }
    // checks for complexity mistakes
    if c.is_uppercase() {
      map.insert("uppercase", true);
    } else if c.is_lowercase() {
      map.insert("lowercase", true);
    } else if c.is_numeric() {
      map.insert("digit", true);
    }
  }
  if n_steps > 0 && !*map.get("lowercase").unwrap() {
    n_steps -= 1;
  } else if n_steps == 0 && !*map.get("lowercase").unwrap() {
    n_steps += 1;
  }
  if n_steps > 0 && !*map.get("uppercase").unwrap() {
    n_steps -= 1;
  } else if n_steps == 0 && !*map.get("uppercase").unwrap() {
    n_steps += 1;
  }
  if n_steps > 0 && !*map.get("digit").unwrap() {
    n_steps -= 1;
  } else if n_steps == 0 && !*map.get("digit").unwrap() {
    n_steps += 1;
  }
  let length = (&passwd).len();
  if length > 20 {
    n_steps += length - 20;
  } else if length < 6 {
    n_steps += 6 - length;
  }
  return n_steps as i32;
}
