
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
