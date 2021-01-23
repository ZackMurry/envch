use termion::color;
use crate::input;

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
  scope: Scope,
  declared_in: String
}

impl EnvironmentVariable {
  pub fn new(name: String, value: String, scope: Scope, declared_in: String) -> EnvironmentVariable {
    EnvironmentVariable {
      name,
      value,
      scope,
      declared_in
    }
  }

  pub fn get_name(&self) -> &str {
    self.name.as_str()
  }

  pub fn get_value(&self) -> &str {
    self.value.as_str()
  }

  pub fn get_declared_in(&self) -> &str {
    self.declared_in.as_str()
  }

  pub fn balance_lengths_with_declared(&mut self, name_len: usize, declared_len: usize) {
    // align all equal signs
    for _ in 0..(name_len - self.name.len()) {
      self.name.push(' ');
    }
    for _ in 0..(declared_len - self.declared_in.len()) {
      self.declared_in.push(' ');
    }
  }

  pub fn balance_lengths(&mut self, name_len: usize) {
    // align all equal signs
    for _ in 0..(name_len - self.name.len()) {
      self.name.push(' ');
    }
  }

  pub fn print(&mut self, options: input::List) {
    let access_color: color::Rgb = match self.scope {
      Scope::System => color::Rgb(124, 171, 230),
      Scope::User => color::Rgb(220, 222, 89),
      Scope::Terminal => color::Rgb(163, 113, 217),
      Scope::Process => color::Rgb(162, 232, 21)
    };
    if options.show_declared_in {
      let declared_in_color = color::Rgb(116, 184, 164);
      println!("{}{} {}{} {}= {}{}",
        color::Fg(declared_in_color),
        self.declared_in,
        color::Fg(access_color),
        self.name,
        color::Fg(color::LightBlack),
        color::Fg(color::LightWhite),
        self.value
      )
    } else {
      println!("{}{} {} = {}{}",
        color::Fg(access_color),
        self.name,
        color::Fg(color::LightBlack),
        color::Fg(color::LightWhite),
        self.value
      )
    }
  }
}
