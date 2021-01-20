use std::fmt;

#[derive(Debug)]
pub enum Scope {
  System,
  User,
  Terminal,
  Process
}

#[derive(Debug)]
pub struct EnvironmentVariable {
  name: String,
  value: String,
  scope: Scope
}

impl EnvironmentVariable {
  pub fn new(name: String, value: String, scope: Scope) -> EnvironmentVariable {
    EnvironmentVariable {
      name,
      value,
      scope
    }
  }
}

impl fmt::Display for EnvironmentVariable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} = {}", self.name, self.value)
  }
}
