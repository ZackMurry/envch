pub mod utils;
pub mod input;
use std::{cmp::max, fs::File};
use termion::style::{Underline, NoUnderline};
use input::{List, Set};
use utils::environment_variable::{EnvironmentVariable, Scope};
use structopt::StructOpt;

fn list_env_vars(options: List) {
    let vars = utils::list_env::get_all_environment_variables(options.debug, true, options.show_path);
    
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

fn set_env_var_user(options: Set) {
    let envch_sh_path = "/etc/profile.d/envch.sh";
    
    let mut content_res = std::fs::read_to_string(envch_sh_path);
    if content_res.is_err() {
        let create_res = File::create(envch_sh_path);
        if create_res.is_err() {
            println!("Error accessing {} -- try using `sudo`", envch_sh_path);
            return
        }
        content_res = std::fs::read_to_string(envch_sh_path);
    }
    let content = content_res.unwrap();
    let mut new_content = String::new();
    let mut already_declared = false;
    for line in content.lines() {
        let assignment: String = line.to_string().chars().skip(7).collect(); // skip "export "
        let mut parts = assignment.split('=');
        let name = parts.next();
        if name.is_none() {
            continue;
        }
        if name.unwrap() == options.name {
            let cur_val = parts.next();
            if cur_val.is_none() {
                new_content.push_str(line);
            } else {
                println!("Previous value: {}", cur_val.unwrap());
            }
            already_declared = true;
            new_content.push_str("export ");
            new_content.push_str(&options.name);
            new_content.push_str("=\"");
            new_content.push_str(&options.value);
            new_content.push_str("\"");
        } else {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    if !already_declared {
        new_content.push_str("\nexport ");
        new_content.push_str(&options.name);
        new_content.push_str("=\"");
        new_content.push_str(&options.value);
        new_content.push_str("\"");
    }
    let write_res = std::fs::write(envch_sh_path, new_content);
    if write_res.is_err() {
        println!("Error writing to {} -- try using `sudo`", envch_sh_path);
    } else {
        println!("Successfully set environment variable in {}", envch_sh_path);
    }

}

fn set_env_var_system(options: Set) {
    
}

fn set_env_var(options: Set) {
    let current_vars_opt = utils::list_env::get_all_environment_variables(options.debug, false, false);
    if current_vars_opt.is_none() {
        println!("Error fetching current variables");
        return
    }
    let current_vars = current_vars_opt.unwrap();
    let mut existing_var_opt: Option<EnvironmentVariable> = None;
    for var in current_vars {
        if var.get_name() == options.name {
            existing_var_opt = Some(var);
            break;
        }
    }
    if existing_var_opt.is_some() {
        let existing_var = existing_var_opt.unwrap();
        let content_opt = std::fs::read_to_string(existing_var.get_declared_in());
        if content_opt.is_err() {
            println!("Error reading {} -- try using `sudo`", existing_var.get_declared_in());
        }
        let content = content_opt.unwrap();
        let mut new_content = String::new();
        for line in content.lines() {
            let mut assignment = line.to_string();
            let mut start_with_export = false;
            if assignment.starts_with("export ") {
                start_with_export = true;
                assignment = assignment.chars().skip(7).collect();
            }
            let mut parts = assignment.split("=");
            let name = parts.next().expect("Error reading file").to_string();
            if name != options.name {
                new_content.push_str(line);
                new_content.push('\n');
                continue;
            }
            let current_value = parts.next();
            if current_value.is_none() {
                continue;
            } else {
                println!("Replacing existing variable. Previous value: {}", current_value.unwrap());
            }
            let put_val_in_quotes = current_value.unwrap().starts_with('"') && current_value.unwrap().ends_with('"');
            if start_with_export {
                new_content.push_str("export ");
            }
            new_content.push_str(&name);
            new_content.push('=');
            if put_val_in_quotes {
                new_content.push('"');
            }
            new_content.push_str(&options.value);
            if put_val_in_quotes {
                new_content.push('"');
            }
            new_content.push('\n');
        }

        let write_res = std::fs::write(existing_var.get_declared_in(), new_content);
        if write_res.is_err() {
            println!("Error writing to {} -- try using `sudo`", existing_var.get_declared_in());
        } else {
            println!("Successfully wrote to {}", existing_var.get_declared_in());
            if existing_var.get_scope() == Scope::Terminal {
                println!("Restart your terminal for changes to take effect");
            } else if existing_var.get_scope() == Scope::User {
                println!("Log out of your computer for changes to take effect");
            } else {
                println!("Restart your computer for changes to take effect");
            }
        }
    } else if options.scope == Scope::System {
        set_env_var_system(options);
    } else if options.scope == Scope::User {
        set_env_var_user(options);
        println!("Log out of your computer for changes to take effect");
    }
}

fn main() {
    if let Some(subcommand) = input::Cli::from_args().command {
        match subcommand {
            input::Command::List(cfg) => list_env_vars(cfg),
            input::Command::Set(cfg) => set_env_var(cfg)
        }
    } else {
        println!("Please use a subcommand. You can view subcommands by using the `--help` flag.");
    }
}
