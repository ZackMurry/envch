use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "envch", about = "An intuitive program that allows users to set, modify, list, and remove environment variables")]
pub struct Cli {

  /// Show debug log
  #[structopt(short, long)]
  pub debug: bool,

  /// Print the file in which each variable is declared
  #[structopt(short, long)]
  pub show_declared_in: bool,
  
  /// Show PATH in the output
  #[structopt(short = "p", long)]
  pub show_path: bool

}
