use structopt::StructOpt;

use crate::utils::environment_variable::Scope;


#[derive(Debug, StructOpt)]
#[structopt(name = "envch", about = "An intuitive program that allows users to set, modify, list, and remove environment variables")]
pub struct Cli {
  /// Subcommands
  #[structopt(subcommand)]
  pub command: Option<Command>,

}

#[derive(Debug, StructOpt)]
pub enum Command {
  List(List),
  Set(Set),
  Remove(Remove)
}

#[derive(Debug, StructOpt, Clone, Copy)]
pub struct List {
  /// Print the file in which each variable is declared
  #[structopt(short, long)]
  pub show_declared_in: bool,

  /// Show PATH in output
  #[structopt(short = "p", long)]
  pub show_path: bool,

  /// Show debug log
  #[structopt(short, long)]
  pub debug: bool,

  /// Show column names in the output
  #[structopt(short = "c", long)]
  pub show_columns: bool

}

/// Updates an existing environment variable or creates a new one if not found
#[derive(Debug, StructOpt, Clone)]
pub struct Set {

  /// Name of environment variable to set
  pub name: String,

  /// Value to set the environment variable to
  pub value: String,

  /// Show debug log
  #[structopt(short, long)]
  pub debug: bool,

  /// The scope to set the environment variable to.
  /// Either "system", "user", or "terminal".
  /// This option only applies if an environment variable with this name doesn't already exists.
  /// If one already exists, it simply replaces the variable with the same scope.
  #[structopt(short, long, default_value = "user")]
  pub scope: Scope

}

#[derive(Debug, StructOpt, Clone)]
pub struct Remove {

  /// Name of environment variable to remove
  pub name: String,

  /// Show debug log
  #[structopt(short, long)]
  pub debug: bool

}
