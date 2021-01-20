pub mod utils;

fn main() {
    let vars = utils::list_env::get_all_environment_variables();
    println!("");
    if vars.is_some() {
        for var in vars.unwrap() {
            println!("{}", var);
        }
    }
}
