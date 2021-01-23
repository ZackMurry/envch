pub mod utils;
pub mod input;
use std::cmp::max;
use input::List;
use structopt::StructOpt;

fn list_env_vars(options: List) {
    let vars = utils::list_env::get_all_environment_variables(options);
    
    if vars.is_some() {
        let unwrapped = vars.unwrap();
        if unwrapped.len() == 0 {
            return;
        }

        let mut name_len = 0;
        for var in &unwrapped {
            name_len= max(name_len, var.get_name().len());
        }

        if options.show_declared_in {
            let mut declared_len = 0;
            for var in &unwrapped {
                declared_len = max(declared_len, var.get_declared_in().len());
            }
            for mut var in unwrapped {
                var.balance_lengths_with_declared(name_len, declared_len);
                var.print(options);
            }
        } else {
            for mut var in unwrapped {
                var.balance_lengths(name_len);
                var.print(options);
            }
        }

        // todo optional column names
    } else {
        println!("Failed to execute. There are likely more logs above.");
    }

}

fn main() {
    if let Some(subcommand) = input::Cli::from_args().command {
        match subcommand {
            input::Command::List(cfg) => list_env_vars(cfg)
        }
    } else {
        println!("Please use a subcommand. You can view subcommands by using the `--help` flag.");
    }
}
