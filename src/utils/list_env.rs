use std::{env, fs};
use crate::utils::environment_variable::{EnvironmentVariable, Scope};

pub fn get_system_environment_variables() -> Vec<EnvironmentVariable> {
  let system_environment_path = "/etc/environment";
  let parse_error_msg = "Error parsing /etc/environment";

  let declared_in = system_environment_path.to_string();
  let contents = fs::read_to_string(system_environment_path).expect("Failed to read /etc/environment");
  let mut split_lines = contents.split("\n");
  let mut env_variables: Vec<EnvironmentVariable> = Vec::new();
  let mut current_line = split_lines.next();

  while current_line.is_some() && !current_line.unwrap().is_empty() {
    let mut parts = current_line.unwrap().split('=');
    let var_name = parts.next().expect(parse_error_msg).to_string();
    if var_name == "PATH" {
      current_line = split_lines.next();
      continue;
    }
    let mut var_value = parts.next().expect(parse_error_msg).to_string();
    if var_value.starts_with('"') && var_value.ends_with('"') {
      var_value.pop();
      var_value = var_value.chars().skip(1).collect();
    }
    
    let env_variable = EnvironmentVariable::new(var_name, var_value, Scope::System, declared_in.clone());
    env_variables.push(env_variable);
    current_line = split_lines.next();
  }

  env_variables
}

fn parse_bash(file_name: String, content: String, debug: bool, get_value_from_env: bool) -> Vec<EnvironmentVariable> {
  let lines = content.lines();
  
  let mut env_variables: Vec<EnvironmentVariable> = Vec::new();
  for line in lines {
    let trimmed = line.trim();
    if trimmed.starts_with("export ") {

      // skipping the "export " part
      let assignment: String = trimmed.chars().skip(7).collect();
      let mut parts = assignment.split('=');
      let name_option = parts.next();
      if name_option.is_none() {
        if debug {
          println!("Couldn't find a name for an environment variable. Continuing...");
        }
        continue;
      }
      let name = name_option.unwrap();
      if name == "PATH" || name.starts_with('#') || name.contains('\t') {
        continue;
      }
      let mut value = String::new();
      if get_value_from_env {
        let value_result = env::var(name);
        if value_result.is_err() {
          if debug {
            println!("{} declared in {} but not found in system", name, file_name);
          }
          let value_opt = parts.next();
          if value_opt.is_none() {
            if debug {
              println!("No equals sign found in {} for name {}", file_name, name);
            }
            continue;
          }
          value = value_opt.unwrap().to_string();
          if value.contains("${") {
            if debug {
              println!("Cannot extract value from variable declared with a variable and not found in system");
            }
            continue;
          }
        } else {
          value = value_result.unwrap();
        }
      }
      if value.starts_with('"') && value.ends_with('"') {
        value = value.chars().skip(1).take(value.len() - 2).collect();
      }
      let env_variable = EnvironmentVariable::new(name.to_string(), value, Scope::User, file_name.clone());
      env_variables.push(env_variable);
    }
  }
  env_variables
} 

pub fn get_user_environment_variables(debug: bool, get_value_from_env: bool) -> Option<Vec<EnvironmentVariable>> {
  let user_profile_path = "/etc/profile.d";
  let err_msg = "Error reading /etc/profile.d";
  let files = fs::read_dir(user_profile_path).expect(err_msg);

  let mut env_variables: Vec<EnvironmentVariable> = Vec::new();
  for file in files {
    let cur = file.unwrap();
    if cur.file_type().expect(err_msg).is_dir() {
      if debug {
        println!("Encountered a folder in /etc/profile.d. Skipping...");
      }
      continue;
    }
    let content = fs::read_to_string(cur.path()).ok()?;
    let file_name = cur.file_name().to_str().expect(err_msg).to_string();
    let mut file_path = user_profile_path.clone().to_string();
    file_path.push('/');
    file_path.push_str(file_name.as_str());
    let parsed_vars = parse_bash(file_path, content, debug, get_value_from_env);
    for env_var in parsed_vars {
      env_variables.push(env_var);
    }
  }
  Some(env_variables)
}

fn get_zsh_environment_variables(debug: bool) -> Option<Vec<EnvironmentVariable>> {
  let zshenv_path = &shellexpand::tilde("~/.zshenv").to_string();
  let content_result = fs::read_to_string(zshenv_path);
  if content_result.is_err() {
    if debug {
      println!(".zshenv not found. User is not using zsh");
    }
    // don't throw an error if path not found -- user just doesn't use zsh
    return Some(Vec::new());
  }
  let content = content_result.unwrap();
  let mut env_vars = Vec::new();
  for line in content.lines() {
    let mut parts = line.split('=');
    let mut name = parts.next().expect("Error reading .zshenv").to_string();
    let val_opt = parts.next();
    if val_opt.is_none() {
      continue;
    }
    let mut value = val_opt.unwrap().to_string();
    if name.starts_with('#') {
      continue;
    }
    if name.starts_with("export ") {
      name = name.chars().skip(7).collect();
    }
    if value.starts_with('"') && value.ends_with('"') {
      value = value.chars().skip(1).take(value.len() - 2).collect();
    }
    let env_var = EnvironmentVariable::new(name, value, Scope::Terminal, zshenv_path.to_owned());
    env_vars.push(env_var);
  }
  Some(env_vars)
}

fn get_bash_environment_variables(debug: bool) -> Option<Vec<EnvironmentVariable>> {
  let bashrc_path = &shellexpand::tilde("~/.bashrc").to_string();
  let content_result = fs::read_to_string(bashrc_path);
  if content_result.is_err() {
    if debug {
      println!(".bashrc not found. Apparently the user has never installed bash.");
    }
    return Some(Vec::new());
  }
  
  let content = content_result.unwrap();

  let mut env_vars = Vec::new();
  for line in content.lines() {
    let mut parts = line.split('=');
    let mut name = parts.next().expect("Error reading .bashrc").to_string();
    let val_opt = parts.next();
    if val_opt.is_none() {
      continue;
    }
    let value = val_opt.unwrap().to_string();
    if name.starts_with("export ") {
      name = name.chars().skip(7).collect();
    }
    
    // not including vars with \t in their name because it likely means that they're in an if statement
    // (i am not trying to make a bash parser smh)
    if name.contains(' ') || name.starts_with('#') || name.contains('\t') {
      continue;
    }
    let env_var = EnvironmentVariable::new(name, value, Scope::Terminal, bashrc_path.to_owned());
    env_vars.push(env_var);
  }
  Some(env_vars)
}

pub fn get_terminal_environment_variables(debug: bool) -> Option<Vec<EnvironmentVariable>> {
  let terminal_vars_opt = get_zsh_environment_variables(debug);
  if terminal_vars_opt.is_none() {
    return None
  }
  let mut terminal_vars = terminal_vars_opt.unwrap();

  let bash_vars_opt = get_bash_environment_variables(debug);
  if bash_vars_opt.is_none() {
    return None
  }
  for var in bash_vars_opt.unwrap() {
    terminal_vars.push(var);
  }

  Some(terminal_vars)
}

pub fn get_all_environment_variables(debug: bool, get_value_from_env: bool, show_path: bool) -> Option<Vec<EnvironmentVariable>> {
  // the other vars are just added to system_vars
  let mut system_vars = get_system_environment_variables();
  
  if show_path {
    let path_name = "PATH";
    let path_result = env::var(path_name);
    if path_result.is_err() {
      println!("Error getting PATH: not found.")
    } else {
      let path_var = EnvironmentVariable::new(path_name.to_string(), path_result.unwrap(), Scope::System, "".to_string());
      system_vars.push(path_var);
    }
  }

  let user_vars_option = get_user_environment_variables(debug, get_value_from_env);
  if user_vars_option.is_none() {
    if debug {
      println!("get_user_environment_variables() failed.");
    }
    return None
  }
  let user_vars = user_vars_option.unwrap();
  for user_var in user_vars {
    system_vars.push(user_var);
  }

  let terminal_vars_option = get_terminal_environment_variables(debug);
  if terminal_vars_option.is_none() {
    if debug {
      println!("get_terminal_environment_variables() failed.")
    }
    return None
  }
  let terminal_vars = terminal_vars_option.unwrap();
  for terminal_var in terminal_vars {
    system_vars.push(terminal_var);
  }

  Some(system_vars)
}
