use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "envch", about = "An intuitive program that allows users to set, modify, list, and remove environment variables")]
pub struct Cli {
  /// Subcommands
  #[structopt(subcommand)]
  pub command: Option<Command>,

}

#[derive(Debug, StructOpt)]
pub enum Command {
  List(List)
}

#[derive(Debug, StructOpt, Clone, Copy)]
pub struct List {
  /// Print the file in which each variable is declared
  #[structopt(short, long)]
  pub show_declared_in: bool,

  /// Show PATH in output
  #[structopt(short = "p", long)]
  pub show_path: bool,

  #[structopt(short, long)]
  pub debug: bool,

  #[structopt(short = "c", long)]
  pub show_columns: bool

}
