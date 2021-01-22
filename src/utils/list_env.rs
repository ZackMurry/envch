use std::{env, fs};
use structopt::StructOpt;
use crate::utils::environment_variable::{EnvironmentVariable, Scope};
use crate::input;

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

fn parse_bash(file_name: String, content: String) -> Vec<EnvironmentVariable> {
  let lines = content.lines();
  
  let mut env_variables: Vec<EnvironmentVariable> = Vec::new();
  for line in lines {
    let trimmed = line.trim();
    if trimmed.starts_with("export") {

      // skipping the "export " part
      let assignment: String = trimmed.chars().skip(7).collect();
      let name_option = assignment.split("=").next();
      if name_option.is_none() {
        if input::Cli::from_args().debug {
          println!("Couldn't find a name for an environment variable. Continuing...");
        }
        continue;
      }
      let name = name_option.unwrap();
      if name == "PATH" {
        continue;
      }
      let value_result = env::var(name);
      if value_result.is_err() {
        if input::Cli::from_args().debug {
          println!("{} declared in {} but not found", name, file_name);
        }
        continue;
      }
      let value = value_result.unwrap();
      let env_variable = EnvironmentVariable::new(name.to_string(), value, Scope::User, file_name.clone());
      env_variables.push(env_variable);
    }
  }
  env_variables
} 

pub fn get_user_environment_variables() -> Option<Vec<EnvironmentVariable>> {
  let user_profile_path = "/etc/profile.d";
  let err_msg = "Error reading /etc/profile.d";
  let files = fs::read_dir(user_profile_path).expect(err_msg);

  let mut env_variables: Vec<EnvironmentVariable> = Vec::new();
  for file in files {
    let cur = file.unwrap();
    if cur.file_type().expect(err_msg).is_dir() {
      if input::Cli::from_args().debug {
        println!("Encountered a folder in /etc/profile.d. Skipping...");
      }
      continue;
    }
    let content = fs::read_to_string(cur.path()).ok()?;
    let file_name = cur.file_name().to_str().expect(err_msg).to_string();
    let mut file_path = user_profile_path.clone().to_string();
    file_path.push('/');
    file_path.push_str(file_name.as_str());
    let parsed_vars = parse_bash(file_path, content);
    for env_var in parsed_vars {
      env_variables.push(env_var);
    }
  }
  Some(env_variables)
}

pub fn get_all_environment_variables() -> Option<Vec<EnvironmentVariable>> {
  let mut system_vars = get_system_environment_variables();
  
  if input::Cli::from_args().show_path {
    let path_name = "PATH";
    let path_result = env::var(path_name);
    if path_result.is_err() {
      println!("Error getting PATH: not found.")
    } else {
      let path_var = EnvironmentVariable::new(path_name.to_string(), path_result.unwrap(), Scope::System, "".to_string());
      system_vars.push(path_var);
    }
  }

  let user_vars_option = get_user_environment_variables();
  if user_vars_option.is_none() {
    return None
  }
  let user_vars = user_vars_option.unwrap();
  for user_var in user_vars {
    system_vars.push(user_var);
  }

  Some(system_vars)
}
