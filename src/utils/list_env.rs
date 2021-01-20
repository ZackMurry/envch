use std::fs;
use crate::utils::environment_variable::{EnvironmentVariable, Scope};

pub fn get_system_environment_variables() -> Vec<EnvironmentVariable> {
  let system_environment_path = "/etc/environment";
  let contents = fs::read_to_string(system_environment_path).expect("Failed to read /etc/environment");
  println!("{}", contents);
  let mut split_lines = contents.split("\n");
  let mut env_variables: Vec<EnvironmentVariable> = Vec::new();
  let mut current_line = split_lines.next();
  while current_line.is_some() && !current_line.unwrap().is_empty() {
    let mut parts = current_line.unwrap().split('=');
    let var_name = parts.next().expect("Error parsing /etc/environment").to_string();
    let mut var_value = parts.next().expect("Error parsing /etc/environment").to_string();
    if var_value.starts_with('"') && var_value.ends_with('"') {
      var_value.pop();
      var_value = var_value.chars().skip(1).collect();
    }
    
    let env_variable = EnvironmentVariable::new(var_name, var_value, Scope::System);
    env_variables.push(env_variable);
    current_line = split_lines.next();
  }
  env_variables
}
