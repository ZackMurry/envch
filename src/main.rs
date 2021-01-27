pub mod utils;
pub mod input;
use std::{cmp::max, fs, fs::File};
use termion::style::{Underline, NoUnderline};
use input::{List, Remove, Set};
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
    
    let mut content_res = fs::read_to_string(envch_sh_path);
    if content_res.is_err() {
        let create_res = File::create(envch_sh_path);
        if create_res.is_err() {
            println!("Error accessing {} -- try using `sudo`", envch_sh_path);
            return
        }
        content_res = fs::read_to_string(envch_sh_path);
    }
    let content = content_res.unwrap();
    let mut new_content = String::new();

    // already_declared is just for if, for some reason, list_env doesn't catch this variable
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
            if cur_val.is_some() {
                println!("Previous value: {}", cur_val.unwrap());
            } else {
                new_content.push_str(line);
                new_content.push('\n');
                continue;
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
    let write_res = fs::write(envch_sh_path, new_content);
    if write_res.is_err() {
        println!("Error writing to {} -- try using `sudo`", envch_sh_path);
    } else {
        println!("Successfully set environment variable in {}", envch_sh_path);
        println!("Log out of your computer for changes to take effect");
    }
}

fn set_env_var_system(options: Set) {
    let environment_path = "/etc/environment";

    let content_res = fs::read_to_string(environment_path);
    if content_res.is_err() {
        println!("Error reading {} -- make sure you are using `sudo`", environment_path);
        return
    }
    let content = content_res.unwrap();
    let mut new_content = String::new();
    
    let mut already_declared = false; // just in case list_env doesn't catch this variable
    for line in content.lines() {
        let mut parts = line.split('=');
        let name_opt = parts.next();
        if name_opt.is_none() {
            if options.debug {
                println!("Warning -- name not found in variable declaration in /etc/environment. Line: {}", line);
            }
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }
        if name_opt.unwrap() == options.name {
            let cur_val = parts.next();
            if cur_val.is_some() {
                println!("Previous value: {}", cur_val.unwrap());
            } else {
                new_content.push_str(line);
                new_content.push('\n');
                continue;
            }
            already_declared = true;
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
        new_content.push_str(&options.name);
        new_content.push_str("=\"");
        new_content.push_str(&options.value);
        new_content.push_str("\"");
    }
    let write_res = fs::write(environment_path, new_content);
    if write_res.is_err() {
        println!("Error writing to {} -- try using `sudo`", environment_path);
    } else {
        println!("Successfully set environment variable in {}", environment_path);
        println!("Restart your computer for changes to take effect");
    }
}

fn set_env_var_terminal(options: Set) {
    // getting user's default shell -- not necessarily the current shell
    let cur_shell_path_res = std::env::var("SHELL");
    if cur_shell_path_res.is_err() {
        println!("Error finding current shell");
        return;
    }
    let cur_shell_path = cur_shell_path_res.unwrap();
    let cur_shell_path_end_opt = cur_shell_path.split("/").last();
    if cur_shell_path_end_opt.is_none() {
        println!("Error finding current shell");
        return;
    }
    let cur_shell_opt = match cur_shell_path_end_opt.unwrap() {
        "zsh" => Some("zsh"),
        "bash" => Some("bash"),
        _ => None
    };
    if cur_shell_opt.is_none() {
        println!("Error: unsupported shell.");
        println!("Terminal environment variables are set in the shell's configuration file. Please create an issue on github to get your shell supported.");
        return;
    }
    let cur_shell = cur_shell_opt.unwrap();
    println!("Configuring for {}", cur_shell);

    let configuration_path: String;
    if cur_shell == "zsh" {
        configuration_path = shellexpand::tilde("~/.zshenv").to_string();
    } else {
        configuration_path = shellexpand::tilde("~/.bashrc").to_string();
    }
    let content_res = fs::read_to_string(&configuration_path);
    if content_res.is_err() {
        println!("Error accessing {}", configuration_path);
        return;
    }
    let mut content = content_res.unwrap();
    content.push_str("\nexport ");
    content.push_str(&options.name);
    content.push('=');
    content.push_str(&options.value);
    let res = fs::write(&configuration_path, content);
    if res.is_ok() {
        println!("Successfully updated {}", configuration_path);
        println!("Restart your terminal for changes to take effect");
    } else {
        println!("Error updating {}", configuration_path);
    }
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
        let content_opt = fs::read_to_string(existing_var.get_declared_in());
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

        let write_res = fs::write(existing_var.get_declared_in(), new_content);
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
    } else if options.scope == Scope::Terminal {
        set_env_var_terminal(options);
    }
}

/// Removes environment variable from /etc/environment
fn remove_system_env_var(options: Remove) {
    let system_var_path = "/etc/environment";
    let content_res = fs::read_to_string(system_var_path);
    if content_res.is_err() {
        println!("Error reading {}", system_var_path);
        return;
    }
    let content = content_res.unwrap();
    let mut new_content = String::new();

    let mut updated = false;
    for line in content.lines() {
        let mut parts = line.split('=');
        let name_opt = parts.next();
        if name_opt.is_none() {
            if options.debug {
                println!("No name found for environment variable line {} in {}", line, system_var_path);
            }
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }
        let name = name_opt.unwrap();
        if name == options.name {
            updated = true;
        } else {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    if !updated {
        println!("Couldn't find {} in {} even though it was found during the initial scan", options.name, system_var_path);
        return;
    }
    let write_res = fs::write(system_var_path, new_content);
    if write_res.is_err() {
        println!("Error writing to {} -- try using `sudo`", system_var_path);
    } else {
        println!("Successfuly removed {} from {}", options.name, system_var_path);
        println!("Restart you computer for changes to take effect");
    }
}

fn remove_bash_env_var(options: Remove, declared_in: &str) {
    let content_res = fs::read_to_string(&declared_in);
    if content_res.is_err() {
        println!("Error reading {}", declared_in);
        return;
    }
    let content = content_res.unwrap();
    let mut new_content = String::new();
    let mut updated = false;
    for line in content.lines() {
        // not removing variables that are nested in if statements and stuff bc the user probably doesn't want to remove those
        if !line.starts_with("export ") && !line.starts_with(&options.name) {
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }
        let mut assignment = line.to_string();
        if assignment.starts_with("export ") {
            // skipping "export "
            assignment = assignment.chars().skip(7).collect();
        }
        let mut parts = assignment.split('=');
        let name_opt = parts.next();
        if name_opt.is_none() {
            if options.debug {
                println!("Variable in {} has no name. Line: {}", declared_in, line);
            }
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }
        let name = name_opt.unwrap();
        if name == options.name {
            updated = true;
        } else {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    if !updated {
        println!("Environment variable found in {} but later not found", declared_in);
        return;
    }
    let write_res = fs::write(declared_in, new_content);
    if write_res.is_err() {
        println!("Error writing to {} -- try using `sudo`", declared_in);
    } else {
        println!("Successfully removed {} from {}", options.name, declared_in);
        println!("Log out of your computer for changes to take effect");
    }
}

fn remove_env_var(options: Remove) {
    let env_vars_opt = utils::list_env::get_all_environment_variables(options.debug, false, false);
    if env_vars_opt.is_none() {
        println!("Error finding current environment variables");
        return;
    }
    let env_vars = env_vars_opt.unwrap();
    let mut env_var_opt = None;
    for var in env_vars {
        if var.get_name() == options.name {
            env_var_opt = Some(var);
            break;
        }
    } 
    if env_var_opt.is_none() {
        println!("Could not find an environment variable with the name {}", options.name);
        return;
    }
    let env_var = env_var_opt.unwrap();
    if env_var.get_scope() == Scope::System {
        remove_system_env_var(options);
    } else {
        remove_bash_env_var(options, env_var.get_declared_in());
    }
}

fn main() {
    if let Some(subcommand) = input::Cli::from_args().command {
        match subcommand {
            input::Command::List(cfg) => list_env_vars(cfg),
            input::Command::Set(cfg) => set_env_var(cfg),
            input::Command::Remove(cfg) => remove_env_var(cfg)
        }
    } else {
        println!("Please use a subcommand. You can view subcommands by using the `--help` flag.");
    }
}
