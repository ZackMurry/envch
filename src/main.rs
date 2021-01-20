pub mod utils;

fn main() {
    let system_env_variables =  utils::list_env::get_system_environment_variables();
    println!("{:?}", system_env_variables);
}
