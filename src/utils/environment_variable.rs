use std::fmt;
use termion::color;
use crate::input;
use structopt::StructOpt;

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

  pub fn balance_lengths(&mut self, name_len: usize, declared_len: usize) {
    // align all equal signs
    for _ in 0..(name_len - self.name.len()) {
      self.name.push(' ');
    }

    if input::Cli::from_args().show_declared_in {
      for _ in 0..(declared_len - self.declared_in.len()) {
        self.declared_in.push(' ');
      }
    }
  }
}

impl fmt::Display for EnvironmentVariable {
  /// expects fixed-length fields
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let access_color: color::Rgb = match self.scope {
      Scope::System => color::Rgb(124, 171, 230),
      Scope::User => color::Rgb(220, 222, 89),
      Scope::Terminal => color::Rgb(134, 38, 237),
      Scope::Process => color::Rgb(162, 232, 21)
    };
    let show_declared_in: bool = input::Cli::from_args().show_declared_in;
    if show_declared_in {
      let declared_in_color = color::Rgb(116, 184, 164);
      write!(f, "{}{} {}{} {} = {}",
        color::Fg(declared_in_color),
        self.declared_in,
        color::Fg(access_color),
        self.name,
        color::Fg(color::LightWhite),
        self.value
      )
    } else {
      write!(f, "{}{} {} = {}",
        color::Fg(access_color),
        self.name,
        color::Fg(color::LightWhite),
        self.value
      )
    }
  }
}
