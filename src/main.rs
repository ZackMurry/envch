pub mod utils;
pub mod input;
use std::{cmp::max};
use termion::style::{Underline, NoUnderline};
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
            if options.show_columns {
                let mut declared_in_column_spacing = "".to_string();
                let mut temp_decl_len = declared_len.clone() - 11; // "Declared in" is 11 chars long
                while temp_decl_len > 0 {
                    declared_in_column_spacing.push(' ');
                    temp_decl_len -= 1;
                }

                let mut name_column_spacing = "".to_string();
                let mut temp_name_len = name_len.clone() - 4; // "Name" is 4 chars long
                while temp_name_len > 0 {
                    name_column_spacing.push(' ');
                    temp_name_len -= 1;
                }
                println!("{}Declared in{}{} {}Name{}{}   {}Value{}", Underline, NoUnderline, declared_in_column_spacing, Underline, NoUnderline, name_column_spacing, Underline, NoUnderline);
            }

            for mut var in unwrapped {
                var.balance_lengths_with_declared(name_len, declared_len);
                var.print(options);
            }
        } else {
            if options.show_columns {
                let mut name_column_spacing = "".to_string();
                let mut temp_name_len = name_len.clone() - 4; // "Name" is 4 chars long
                while temp_name_len > 0 {
                    name_column_spacing.push(' ');
                    temp_name_len -= 1;
                }
                println!("{}Name{}{}   {}Value{}", Underline, NoUnderline, name_column_spacing, Underline, NoUnderline);
            }
            for mut var in unwrapped {
                var.balance_lengths(name_len);
                var.print(options);
            }
        }
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
